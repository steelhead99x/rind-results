// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask gen-frames`: the EFR32MG24 OLED demo's three 128x64 1-bit
//! frames (`oled_demo/frames.h`) and their 4x previews, as the old
//! `tools/gen_frames.py` drew them with cairosvg and Pillow, in Rust: resvg
//! for the SVGs, and HarfRust, skrifa and a port of FreeType's monochrome
//! rasterizer for the text (no FreeType, cairo or Python).
//!
//! The text matches Pillow pixel for pixel; the two logos are a few pixels
//! off cairo's (see `svg.rs` and the tests).

mod raster;
mod resample;
mod svg;
#[cfg(test)]
mod tests;
mod text;

use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {
    /// The demo directory: SVGs are fetched into its assets/, and
    /// oled_demo/frames.h and preview*.png are written there
    #[arg(long)]
    dir: Option<PathBuf>,
    /// Directory holding DejaVuSans.ttf and DejaVuSans-Bold.ttf
    /// [default: tools/xtask/fonts, DejaVu 2.37]
    #[arg(long)]
    fonts: Option<PathBuf>,
}

const W: usize = 128;
const H: usize = 64;

/// An 8-bit greyscale image (Pillow mode "L").
#[derive(Clone)]
pub struct Gray {
    pub w: usize,
    pub h: usize,
    pub px: Vec<u8>,
}

impl Gray {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            px: vec![0; w * h],
        }
    }
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.px[y * self.w + x]
    }
    pub fn set(&mut self, x: usize, y: usize, v: u8) {
        self.px[y * self.w + x] = v;
    }

    /// `Image.getbbox()`: (left, top, right, bottom) of the non-zero pixels.
    fn bbox(&self) -> Option<(usize, usize, usize, usize)> {
        let mut b: Option<(usize, usize, usize, usize)> = None;
        for y in 0..self.h {
            for x in 0..self.w {
                if self.get(x, y) != 0 {
                    let e = b.get_or_insert((x, y, x + 1, y + 1));
                    e.0 = e.0.min(x);
                    e.1 = e.1.min(y);
                    e.2 = e.2.max(x + 1);
                    e.3 = e.3.max(y + 1);
                }
            }
        }
        b
    }

    fn crop(&self, (x0, y0, x1, y1): (usize, usize, usize, usize)) -> Self {
        let mut out = Self::new(x1 - x0, y1 - y0);
        for y in y0..y1 {
            for x in x0..x1 {
                out.set(x - x0, y - y0, self.get(x, y));
            }
        }
        out
    }

    /// `Image.paste(src, (x, y))`, clipped.
    fn paste(&mut self, src: &Self, x: i64, y: i64) {
        for sy in 0..src.h {
            for sx in 0..src.w {
                let (tx, ty) = (x + sx as i64, y + sy as i64);
                if tx >= 0 && ty >= 0 && (tx as usize) < self.w && (ty as usize) < self.h {
                    self.set(tx as usize, ty as usize, src.get(sx, sy));
                }
            }
        }
    }

    /// `img.point(lambda v: 255 if v >= thr else 0)`.
    fn threshold(&self, thr: u8) -> Self {
        Self {
            w: self.w,
            h: self.h,
            px: self
                .px
                .iter()
                .map(|&v| if v >= thr { 255 } else { 0 })
                .collect(),
        }
    }
}

/// `to_bits`: 255 where `lit(r, g, b, a)`, else 0.
fn to_bits(img: &svg::Rgba, lit: impl Fn(f64, f64, f64, u8) -> bool) -> Gray {
    Gray {
        w: img.w,
        h: img.h,
        px: img
            .px
            .iter()
            .map(|&[r, g, b, a]| {
                if lit(f64::from(r), f64::from(g), f64::from(b), a) {
                    255
                } else {
                    0
                }
            })
            .collect(),
    }
}

fn luma(r: f64, g: f64, b: f64) -> f64 {
    0.299 * r + 0.587 * g + 0.114 * b
}

/// `downsample`: BOX resize, then threshold.
fn downsample(mask: &Gray, w: usize, h: usize, thr: u8) -> Gray {
    resample::box_resize(mask, w, h).threshold(thr)
}

/// Python's `round()` (half to even) for a non-negative float.
fn round_half_even(v: f64) -> f64 {
    let r = v.round();
    if (v - v.trunc() - 0.5).abs() < f64::EPSILON && r % 2.0 != 0.0 {
        r - 1.0
    } else {
        r
    }
}

/// DejaVu Sans and DejaVu Sans Bold, as `gen_frames.py` loaded them.
pub struct Fonts {
    pub sans: Vec<u8>,
    pub bold: Vec<u8>,
}

impl Fonts {
    pub fn load(dir: &Path) -> Result<Self, String> {
        Ok(Self {
            sans: read(&dir.join("DejaVuSans.ttf"))?,
            bold: read(&dir.join("DejaVuSans-Bold.ttf"))?,
        })
    }
    fn sans(&self, size: u32) -> Result<text::Font<'_>, String> {
        text::Font::new(&self.sans, size)
    }
    fn bold(&self, size: u32) -> Result<text::Font<'_>, String> {
        text::Font::new(&self.bold, size)
    }
}

/// `center_text`: `text` centred between `x0` and `x1` by its advance width.
fn center_text(
    img: &mut Gray,
    y: f64,
    s: &str,
    font: &text::Font,
    x0: f64,
    x1: f64,
) -> Result<(), String> {
    let w = font.text_length(s)?;
    let x = x0 + ((x1 - x0 - w) / 2.0).floor();
    font.draw(img, x, y, s)
}

/// Frame 1: the moq.dev mark (favicon with its background dropped: the
/// white letters and the green arrow lit) and "moq.dev / Media over / QUIC".
pub fn frame_moq(favicon: &[u8], fonts: &Fonts) -> Result<Gray, String> {
    let icon = svg::render(favicon, 512, Some(512))?;
    let lit = to_bits(&icon, |r, g, b, a| a > 128 && luma(r, g, b) > 90.0);
    let mark = downsample(&lit, 60, 60, 110);
    let mut f = Gray::new(W, H);
    f.paste(&mark, 2, 2);
    center_text(&mut f, 8.0, "moq.dev", &fonts.bold(13)?, 64.0, 128.0)?;
    center_text(&mut f, 30.0, "Media over", &fonts.sans(10)?, 64.0, 128.0)?;
    center_text(&mut f, 42.0, "QUIC", &fonts.bold(10)?, 64.0, 128.0)?;
    Ok(f)
}

/// Frame 2: the rind wordmark (its cream highlight left dark as a gap) and
/// "pure-Rust crypto".
pub fn frame_rind(wordmark: &[u8], fonts: &Fonts) -> Result<Gray, String> {
    let wm = svg::render(wordmark, 1000, None)?;
    let mask = to_bits(&wm, |r, g, b, a| a > 128 && luma(r, g, b) < 225.0);
    let mask = mask.crop(mask.bbox().ok_or("wordmark: nothing lit")?);
    let h = 48;
    let w = round_half_even(mask.w as f64 * h as f64 / mask.h as f64) as usize;
    let small = downsample(&mask, w, h, 100);
    let mut f = Gray::new(W, H);
    f.paste(&small, (W as i64 - w as i64).div_euclid(2), 1);
    center_text(
        &mut f,
        52.0,
        "pure-Rust crypto",
        &fonts.sans(9)?,
        0.0,
        W as f64,
    )?;
    Ok(f)
}

/// Frame 3: the benchmark headline (rind b6651423, TLS 1.3 full handshake,
/// X25519).
pub fn frame_bench(fonts: &Fonts) -> Result<Gray, String> {
    let (sans, bold) = (fonts.sans(9)?, fonts.bold(9)?);
    let mut f = Gray::new(W, H);
    center_text(&mut f, 0.0, "TLS 1.3 handshake", &bold, 0.0, W as f64)?;
    center_text(&mut f, 10.0, "X25519, full", &sans, 0.0, W as f64)?;
    for x in 0..W {
        f.set(x, 21, 255);
    }
    let rows = [
        ("", "rind", "aws-lc"),
        ("Xeon", "228µs", "299µs"),
        ("M1 Pro", "284µs", "425µs"),
        ("Pi 5", "1.57ms", "1.33ms"),
    ];
    for (i, (a, b, c)) in rows.iter().enumerate() {
        let y = (23 + i * 10) as f64;
        let font = if i == 0 { &bold } else { &sans };
        font.draw(&mut f, 0.0, y, a)?;
        font.draw(&mut f, 48.0, y, b)?;
        font.draw(&mut f, 90.0, y, c)?;
    }
    Ok(f)
}

/// A frame in SSD1306 page order: 8 pages of 128 columns, LSB = top row.
pub fn pages(f: &Gray) -> Vec<u8> {
    let mut buf = Vec::with_capacity(W * H / 8);
    for page in 0..H / 8 {
        for x in 0..W {
            let mut v = 0u8;
            for bit in 0..8 {
                if f.get(x, page * 8 + bit) != 0 {
                    v |= 1 << bit;
                }
            }
            buf.push(v);
        }
    }
    buf
}

pub fn frames_h(frames: &[(&str, Gray)]) -> String {
    let mut lines = vec![
        "/* Generated by `cargo xtask gen-frames`. 128x64, SSD1306 page order (8 pages x 128 cols, LSB = top row), 1 = lit. */".to_string(),
        "#pragma once".to_string(),
        "#include <stdint.h>".to_string(),
        format!("#define DEMO_FRAME_COUNT {}", frames.len()),
    ];
    for (name, f) in frames {
        let buf = pages(f);
        lines.push(format!("static const uint8_t frame_{name}[1024] = {{"));
        for chunk in buf.chunks(16) {
            let row: Vec<String> = chunk.iter().map(|b| format!("0x{b:02x}")).collect();
            lines.push(format!("  {},", row.join(", ")));
        }
        lines.push("};".to_string());
    }
    let names: Vec<String> = frames.iter().map(|(n, _)| format!("frame_{n}")).collect();
    lines.push(format!(
        "static const uint8_t *const demo_frames[DEMO_FRAME_COUNT] = {{{}}};",
        names.join(", ")
    ));
    lines.join("\n") + "\n"
}

const S: usize = 4;
const PAD: usize = 16;

/// An RGB image: width, height, pixels.
pub struct Rgb {
    pub w: usize,
    pub h: usize,
    pub px: Vec<[u8; 3]>,
}

/// A frame at 4x, lit pixels (235, 245, 255) on black.
pub fn preview(f: &Gray) -> Rgb {
    let (w, h) = (W * S, H * S);
    let mut px = vec![[0u8; 3]; w * h];
    for y in 0..h {
        for x in 0..w {
            if f.get(x / S, y / S) != 0 {
                px[y * w + x] = [235, 245, 255];
            }
        }
    }
    Rgb { w, h, px }
}

/// All previews stacked on a (40, 40, 40) strip.
pub fn strip(previews: &[Rgb]) -> Rgb {
    let (w, h) = (W * S + 2 * PAD, previews.len() * (H * S + PAD) + PAD);
    let mut px = vec![[40u8; 3]; w * h];
    for (i, p) in previews.iter().enumerate() {
        let oy = PAD + i * (H * S + PAD);
        for y in 0..p.h {
            for x in 0..p.w {
                px[(oy + y) * w + PAD + x] = p.px[y * p.w + x];
            }
        }
    }
    Rgb { w, h, px }
}

pub fn encode_png(img: &Rgb) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, img.w as u32, img.h as u32);
        enc.set_color(png::ColorType::Rgb);
        enc.set_depth(png::BitDepth::Eight);
        let mut wr = enc.write_header().map_err(|e| format!("png: {e}"))?;
        let data: Vec<u8> = img.px.iter().flatten().copied().collect();
        wr.write_image_data(&data)
            .map_err(|e| format!("png: {e}"))?;
    }
    Ok(out)
}

/// Where `gen_frames.py` fetched its SVGs from (see SOURCES.md).
pub const SOURCES: [(&str, &str); 2] = [
    (
        "moqdev-layout_favicon.svg",
        "https://moq.dev/layout/favicon.svg",
    ),
    (
        "wordmark-light.svg",
        "https://raw.githubusercontent.com/moq-dev/rind/main/admin/wordmark-light.svg",
    ),
];

fn read(p: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))
}

fn write(p: &Path, data: &[u8]) -> Result<(), String> {
    std::fs::write(p, data).map_err(|e| format!("{}: {e}", p.display()))
}

/// Fetch whichever source SVGs are missing from `assets`, unmodified.
fn fetch_assets(assets: &Path) -> Result<(), String> {
    std::fs::create_dir_all(assets).map_err(|e| format!("{}: {e}", assets.display()))?;
    for (name, url) in SOURCES {
        let dest = assets.join(name);
        if dest.is_file() {
            continue;
        }
        let part = assets.join(format!("{name}.part"));
        let st = Command::new("curl")
            .args(["-fsSL", "--retry", "2", "-o"])
            .arg(&part)
            .arg(url)
            .status()
            .map_err(|e| format!("curl: {e}"))?;
        if !st.success() {
            let _ = std::fs::remove_file(&part);
            return Err(format!("curl {url}: {st}"));
        }
        std::fs::rename(&part, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
    }
    Ok(())
}

pub fn run(args: Args) -> Result<(), String> {
    let dir = args
        .dir
        .unwrap_or_else(|| crate::root().join("baremetal/mg24/oled-demo"));
    let fonts_dir = args
        .fonts
        .unwrap_or_else(|| crate::root().join("tools/xtask/fonts"));
    let assets = dir.join("assets");
    fetch_assets(&assets)?;
    let fonts = Fonts::load(&fonts_dir)?;
    let frames = vec![
        (
            "moq",
            frame_moq(&read(&assets.join(SOURCES[0].0))?, &fonts)?,
        ),
        (
            "rind",
            frame_rind(&read(&assets.join(SOURCES[1].0))?, &fonts)?,
        ),
        ("bench", frame_bench(&fonts)?),
    ];
    let header = dir.join("oled_demo");
    std::fs::create_dir_all(&header).map_err(|e| format!("{}: {e}", header.display()))?;
    write(&header.join("frames.h"), frames_h(&frames).as_bytes())?;
    let previews: Vec<Rgb> = frames.iter().map(|(_, f)| preview(f)).collect();
    for (i, ((name, _), p)) in frames.iter().zip(&previews).enumerate() {
        write(
            &dir.join(format!("preview-{}-{name}.png", i + 1)),
            &encode_png(p)?,
        )?;
    }
    write(&dir.join("preview.png"), &encode_png(&strip(&previews))?)?;
    let names: Vec<String> = frames.iter().map(|(n, _)| format!("'{n}'")).collect();
    println!("ok [{}]", names.join(", "));
    Ok(())
}
