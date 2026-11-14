// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Pillow's `Image.resize(size, Image.BOX)` for 8-bit greyscale images
//! (`libImaging/Resample.c`): separable box-filter coefficients in 22-bit
//! fixed point, a horizontal pass, then a vertical one.

use super::Gray;

const PRECISION_BITS: u32 = 32 - 8 - 2;

fn box_filter(x: f64) -> f64 {
    if x > -0.5 && x <= 0.5 { 1.0 } else { 0.0 }
}

/// `precompute_coeffs` plus `normalize_coeffs_8bpc`: per output pixel, the
/// first input index and its integer weights.
fn coeffs(in_size: usize, out_size: usize) -> Vec<(usize, Vec<i32>)> {
    let scale = in_size as f64 / out_size as f64;
    let filterscale = scale.max(1.0);
    let support = 0.5 * filterscale;
    let ss = 1.0 / filterscale;
    (0..out_size)
        .map(|xx| {
            let center = (xx as f64 + 0.5) * scale;
            let xmin = ((center - support + 0.5) as i64).max(0) as usize;
            let xmax = ((center + support + 0.5) as i64).min(in_size as i64) as usize;
            let mut k: Vec<f64> = (xmin..xmax)
                .map(|x| box_filter((x as f64 - center + 0.5) * ss))
                .collect();
            let ww: f64 = k.iter().sum();
            if ww != 0.0 {
                for w in &mut k {
                    *w /= ww;
                }
            }
            let k = k
                .iter()
                .map(|&w| {
                    let v = w * f64::from(1u32 << PRECISION_BITS);
                    if w < 0.0 {
                        (-0.5 + v) as i32
                    } else {
                        (0.5 + v) as i32
                    }
                })
                .collect();
            (xmin, k)
        })
        .collect()
}

fn clip8(v: i64) -> u8 {
    (v >> PRECISION_BITS).clamp(0, 255) as u8
}

/// Resize `src` to `w` x `h` with Pillow's BOX filter.
pub fn box_resize(src: &Gray, w: usize, h: usize) -> Gray {
    let mut cur = src.clone();
    if w != cur.w {
        let k = coeffs(cur.w, w);
        let mut out = Gray::new(w, cur.h);
        for y in 0..cur.h {
            for (x, (xmin, ws)) in k.iter().enumerate() {
                let mut ss: i64 = 1 << (PRECISION_BITS - 1);
                for (i, &wt) in ws.iter().enumerate() {
                    ss += i64::from(cur.get(xmin + i, y)) * i64::from(wt);
                }
                out.set(x, y, clip8(ss));
            }
        }
        cur = out;
    }
    if h != cur.h {
        let k = coeffs(cur.h, h);
        let mut out = Gray::new(cur.w, h);
        for (y, (ymin, ws)) in k.iter().enumerate() {
            for x in 0..cur.w {
                let mut ss: i64 = 1 << (PRECISION_BITS - 1);
                for (i, &wt) in ws.iter().enumerate() {
                    ss += i64::from(cur.get(x, ymin + i)) * i64::from(wt);
                }
                out.set(x, y, clip8(ss));
            }
        }
        cur = out;
    }
    cur
}
