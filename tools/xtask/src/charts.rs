// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask charts`: one SVG per host from schema-1 results: for each
//! report operation, rind's time over the fastest other Rust library's, on a
//! log scale (left of 1x: rind is faster). Hardware baselines are never
//! counted. `--png` also rasterizes each with rsvg-convert, as published.

use crate::py;
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {
    out: PathBuf,
    #[arg(required = true)]
    hosts: Vec<PathBuf>,
    /// also write `<name>.png` beside each SVG (rsvg-convert -w 900 -b white)
    #[arg(long)]
    png: bool,
}

const NOT_RIVALS: &[&str] = &["rind", "esp32s3-hw"];

fn ratios(host: &Value) -> Vec<(String, f64, String)> {
    let mut by: Vec<(String, Vec<(String, f64)>)> = vec![];
    for r in host["results"].as_array().into_iter().flatten() {
        let op = r["operation"].as_str().unwrap_or_default().to_string();
        let lib = r["library"].as_str().unwrap_or_default().to_string();
        let ns = r["ns"].as_f64().unwrap_or(f64::NAN);
        let i = by.iter().position(|(o, _)| *o == op).unwrap_or_else(|| {
            by.push((op, vec![]));
            by.len() - 1
        });
        match by[i].1.iter_mut().find(|(l, _)| *l == lib) {
            Some(e) => e.1 = ns,
            None => by[i].1.push((lib, ns)),
        }
    }
    let mut out = vec![];
    for (op, t) in by {
        let Some(rind) = t.iter().find(|(l, _)| l == "rind").map(|x| x.1) else {
            continue;
        };
        let rivals: Vec<&(String, f64)> = t
            .iter()
            .filter(|(l, _)| !NOT_RIVALS.contains(&l.as_str()) && !l.starts_with("rind-"))
            .collect();
        let Some(mut best) = rivals.first().copied() else {
            continue;
        };
        for r in &rivals[1..] {
            if r.1 < best.1 {
                best = r;
            }
        }
        out.push((
            op.replace("/{lib}", "").replace("{lib}/", ""),
            rind / best.1,
            best.0.clone(),
        ));
    }
    out
}

/// `html.escape(s)`.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// `f"{v:g}"` for the axis ticks (powers of two from 1/8 to 64).
fn g(v: f64) -> String {
    let s = py::repr(v);
    s.strip_suffix(".0").map(String::from).unwrap_or(s)
}

fn svg(host: &Value, rows: &[(String, f64, String)]) -> String {
    let (lo, hi) = (1.0 / 16.0, 64.0f64);
    let (w, left, right, bar, top) = (900i64, 300i64, 150i64, 16i64, 70i64);
    let h = top + rows.len() as i64 * (bar + 4) + 40;
    let span = (hi / lo).ln();
    let x =
        |v: f64| left as f64 + ((v.max(lo).min(hi) / lo).ln() / span) * (w - left - right) as f64;
    let noisy = host
        .get("load")
        .and_then(|l| l.get("noisy"))
        .is_some_and(|v| match v {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(n) => n.as_f64() != Some(0.0),
            Value::String(s) => !s.is_empty(),
            _ => true,
        });
    let mut s = vec![
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" font-family="sans-serif" font-size="12">"#
        ),
        format!(r##"<rect width="{w}" height="{h}" fill="#fff"/>"##),
        format!(
            r#"<text x="10" y="22" font-size="16" font-weight="bold">{}: rind's time over the fastest other Rust library</text>"#,
            escape(host["host"].as_str().unwrap_or_default())
        ),
        format!(
            r##"<text x="10" y="42" fill="#555">{}; log scale, left of 1x rind is faster{}</text>"##,
            escape(host["cpu"]["model"].as_str().unwrap_or_default()),
            if noisy { "; noisy run" } else { "" }
        ),
    ];
    for v in [
        1.0 / 8.0,
        1.0 / 4.0,
        1.0 / 2.0,
        1.0,
        2.0,
        4.0,
        8.0,
        16.0,
        32.0,
        64.0,
    ] {
        let xv = py::fixed(x(v), 1);
        let stroke = if v == 1.0 { "#000" } else { "#ddd" };
        s.push(format!(
            r#"<line x1="{xv}" y1="{}" x2="{xv}" y2="{}" stroke="{stroke}"/>"#,
            top - 8,
            h - 30
        ));
        s.push(format!(
            r##"<text x="{xv}" y="{}" text-anchor="middle" fill="#555">{}x</text>"##,
            h - 14,
            g(v)
        ));
    }
    for (i, (op, r, lib)) in rows.iter().enumerate() {
        let y = top + i as i64 * (bar + 4);
        let (mut a, mut b) = (x(1.0), x(*r));
        if b < a {
            std::mem::swap(&mut a, &mut b);
        }
        s.push(format!(
            r#"<text x="{}" y="{}" text-anchor="end">{}</text>"#,
            left - 6,
            y + 12,
            escape(op)
        ));
        s.push(format!(
            r#"<rect x="{}" y="{y}" width="{}" height="{bar}" fill="{}"/>"#,
            py::fixed(a, 1),
            py::fixed((b - a).max(1.0), 1),
            if *r <= 1.0 { "#2a7" } else { "#d54" }
        ));
        s.push(format!(
            r#"<text x="{}" y="{}">{}x {}</text>"#,
            w - right + 6,
            y + 12,
            py::fixed(*r, 2),
            escape(lib)
        ));
    }
    s.push("</svg>".into());
    s.join("\n")
}

pub fn run(args: Args) -> Result<(), String> {
    std::fs::create_dir_all(&args.out).map_err(|e| e.to_string())?;
    for path in &args.hosts {
        let host = crate::read_json(path)?;
        let name = format!(
            "{}-rind-vs-fastest",
            host["host"].as_str().unwrap_or("host")
        );
        let svg_path = args.out.join(format!("{name}.svg"));
        std::fs::write(&svg_path, svg(&host, &ratios(&host))).map_err(|e| e.to_string())?;
        if args.png {
            let png = args.out.join(format!("{name}.png"));
            let st = Command::new("rsvg-convert")
                .args(["-w", "900", "-b", "white", "-o"])
                .arg(&png)
                .arg(&svg_path)
                .status()
                .map_err(|e| format!("rsvg-convert: {e}"))?;
            if !st.success() {
                return Err(format!("rsvg-convert failed on {}", svg_path.display()));
            }
        }
    }
    Ok(())
}
