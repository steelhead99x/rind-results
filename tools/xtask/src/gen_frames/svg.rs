// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! SVG to straight-alpha RGBA with resvg, sized and placed the way
//! `cairosvg.svg2png(output_width=, output_height=)` does.
//!
//! cairo and tiny-skia anti-alias differently (cairo's scan converter takes
//! 15 sub-scanlines per pixel, tiny-skia 4), so the image is drawn at 4x and
//! box-averaged: 16 sub-scanlines per pixel, the closest match. Edge pixels
//! can still differ by a few levels, which after `gen_frames.py`'s
//! threshold, downsample and threshold again leaves a handful of logo
//! pixels different from the cairo frames (see the tests).

use resvg::{tiny_skia, usvg};

const SUPERSAMPLE: u32 = 4;

pub struct Rgba {
    pub w: usize,
    pub h: usize,
    /// Straight (not premultiplied) RGBA, as cairo's PNG writer stores it.
    pub px: Vec<[u8; 4]>,
}

/// Render `data` at `out_w` x `out_h`; without a height the aspect ratio
/// is kept.
pub fn render(data: &[u8], out_w: u32, out_h: Option<u32>) -> Result<Rgba, String> {
    let tree =
        usvg::Tree::from_data(data, &usvg::Options::default()).map_err(|e| format!("svg: {e}"))?;
    let size = tree.size();
    let (sw, sh) = (f64::from(size.width()), f64::from(size.height()));
    let fw = f64::from(out_w);
    let fh = match out_h {
        Some(h) => f64::from(h),
        None => sh * fw / sw,
    };
    // cairosvg rounds the PNG surface to whole pixels and keeps the exact
    // size for the viewport, scaled with preserveAspectRatio xMidYMid meet.
    let (pw, ph) = (fw.round() as usize, fh.round() as usize);
    let scale = (fw / sw).min(fh / sh);
    let tx = (fw / scale - sw) / 2.0;
    let ty = (fh / scale - sh) / 2.0;
    let ss = SUPERSAMPLE as usize;
    let mut pixmap =
        tiny_skia::Pixmap::new((pw * ss) as u32, (ph * ss) as u32).ok_or("svg: empty size")?;
    let k = scale * f64::from(SUPERSAMPLE);
    let transform =
        tiny_skia::Transform::from_scale(k as f32, k as f32).pre_translate(tx as f32, ty as f32);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let big = pixmap.pixels();
    let bw = pw * ss;
    let n = (ss * ss) as u32;
    let mut px = Vec::with_capacity(pw * ph);
    for y in 0..ph {
        for x in 0..pw {
            let mut acc = [0u32; 4];
            for dy in 0..ss {
                for dx in 0..ss {
                    let p = big[(y * ss + dy) * bw + x * ss + dx];
                    acc[0] += u32::from(p.red());
                    acc[1] += u32::from(p.green());
                    acc[2] += u32::from(p.blue());
                    acc[3] += u32::from(p.alpha());
                }
            }
            // Premultiplied average, then cairo's unpremultiply.
            let [r, g, b, a] = acc.map(|v| (v + n / 2) / n);
            let un = |c: u32| {
                (c * 255 + a / 2)
                    .checked_div(a)
                    .map_or(0, |v| v.min(255) as u8)
            };
            px.push([un(r), un(g), un(b), a as u8]);
        }
    }
    Ok(Rgba { w: pw, h: ph, px })
}
