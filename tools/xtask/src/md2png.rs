// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask md2png`: screenshots a Markdown file (or the part from one
//! heading to the next heading of the same level) as GitHub-like HTML in
//! headless Chrome, cropped to the content. Needs google-chrome (or
//! `CHROME=...`).

use pulldown_cmark::{Options, Parser, html};
use regex::Regex;
use std::path::PathBuf;
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {
    src: PathBuf,
    out: PathBuf,
    #[arg(long)]
    section: Option<String>,
    #[arg(long, default_value_t = 1200)]
    width: u32,
}

fn section(text: &str, heading: &str) -> Result<String, String> {
    let re = Regex::new(&format!(r"(?m)^(#+) {}\s*$", regex::escape(heading))).unwrap();
    let m = re.captures(text).ok_or(format!("no heading {heading:?}"))?;
    let level = m[1].len();
    let whole = m.get(0).unwrap();
    let rest = &text[whole.start()..];
    let after = &rest[whole.len()..];
    let end = Regex::new(&format!(r"(?m)^#{{1,{level}}} ")).unwrap();
    Ok(match end.find(after) {
        Some(e) => rest[..whole.len() + e.start()].to_string(),
        None => rest.to_string(),
    })
}

pub fn run(a: Args) -> Result<(), String> {
    let mut text =
        std::fs::read_to_string(&a.src).map_err(|e| format!("{}: {e}", a.src.display()))?;
    if let Some(s) = &a.section {
        text = section(&text, s)?;
    }
    let mut body = String::new();
    html::push_html(&mut body, Parser::new_ext(&text, Options::ENABLE_TABLES));
    let css = format!(
        "body{{font:14px -apple-system,Segoe UI,Helvetica,Arial,sans-serif;color:#1f2328;margin:24px;max-width:{}px}}\n\
         table{{border-collapse:collapse;margin:12px 0}}th,td{{border:1px solid #d1d9e0;padding:4px 10px}}\n\
         tr:nth-child(2n){{background:#f6f8fa}}code{{background:#eff1f3;padding:1px 4px;border-radius:4px;font-size:90%}}\n\
         pre{{background:#f6f8fa;padding:12px;overflow:auto}}h1,h2{{border-bottom:1px solid #d1d9e0;padding-bottom:4px}}",
        a.width - 48
    );
    let dir = std::env::temp_dir().join(format!("md2png-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let page = dir.join("page.html");
    std::fs::write(
        &page,
        format!("<!doctype html><meta charset=utf-8><style>{css}</style>{body}"),
    )
    .map_err(|e| e.to_string())?;
    let chrome = std::env::var("CHROME").unwrap_or("google-chrome".into());
    // Shoot tall, then crop the blank space below the content.
    let count = |p: &str| body.matches(p).count();
    let n = count("<tr") + count("<li") + count("<p") + count("<h") + count("\n");
    let height = 16000.min(200 + 40 * n);
    let out = std::path::absolute(&a.out).map_err(|e| e.to_string())?;
    let st = Command::new(&chrome)
        .args([
            "--headless=new",
            "--disable-gpu",
            "--no-sandbox",
            "--hide-scrollbars",
        ])
        .arg(format!("--user-data-dir={}/p", dir.display()))
        .arg(format!("--window-size={},{height}", a.width))
        .arg(format!("--screenshot={}", out.display()))
        .arg(format!("file://{}", page.display()))
        .output()
        .map_err(|e| format!("{chrome}: {e}"))?;
    let _ = std::fs::remove_dir_all(&dir);
    if !st.status.success() {
        return Err(format!(
            "{chrome} failed: {}",
            String::from_utf8_lossy(&st.stderr)
        ));
    }
    crop(&out)
}

/// Crops the screenshot to 24 px below its last non-white row, as RGB.
fn crop(path: &std::path::Path) -> Result<(), String> {
    let dec = png::Decoder::new(std::fs::File::open(path).map_err(|e| e.to_string())?);
    let mut reader = dec.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    let (w, h) = (info.width as usize, info.height as usize);
    let ch = match info.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        other => return Err(format!("unexpected screenshot color type {other:?}")),
    };
    if info.bit_depth != png::BitDepth::Eight {
        return Err("unexpected screenshot bit depth".into());
    }
    let row = |y: usize| &buf[y * w * ch..(y + 1) * w * ch];
    let last = (0..h)
        .rev()
        .find(|&y| row(y).chunks(ch).any(|p| p[..3] != [255, 255, 255]));
    let Some(last) = last else { return Ok(()) };
    let keep = h.min(last + 1 + 24);
    let mut rgb = Vec::with_capacity(w * keep * 3);
    for y in 0..keep {
        for p in row(y).chunks(ch) {
            rgb.extend_from_slice(&p[..3]);
        }
    }
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, keep as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.set_compression(png::Compression::Best);
    let mut wr = enc.write_header().map_err(|e| e.to_string())?;
    wr.write_image_data(&rgb).map_err(|e| e.to_string())
}
