// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask relink`: rind's time over the fastest other Rust library on
//! each report operation, across builds that differ only in layout (rind-hardware's
//! `--pad` relinks), with the median over layouts.
//!
//! Each `LABEL=RAW` holds `<bench>/run-*.jsonl`; a row's value in one layout
//! is the median over its runs of each run's median cycles. Only operations
//! present in every layout are listed; `--short` keeps those under 1 KB of
//! input or with no size, the rows layout moves most.

use crate::py::{self, Num};
use crate::tables::Tables;
use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long)]
    rind: PathBuf,
    /// LABEL=RAW_DIR, two or more
    #[arg(required = true, num_args = 2..)]
    layouts: Vec<String>,
    /// only rows under 1 KB of input or with no size
    #[arg(long)]
    short: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

const RIVALS: &[&str] = &["ring", "rustcrypto", "libcrux", "graviola", "aws-lc-rs"];

/// operation -> library -> median cycles, for one layout.
fn load(t: &Tables, raw: &PathBuf) -> Result<BTreeMap<String, BTreeMap<String, f64>>, String> {
    let mut per: BTreeMap<String, Vec<Num>> = BTreeMap::new();
    let mut benches: Vec<PathBuf> = std::fs::read_dir(raw)
        .map_err(|e| format!("{}: {e}", raw.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    benches.sort();
    for b in benches {
        let mut files: Vec<PathBuf> = std::fs::read_dir(&b)
            .map_err(|e| e.to_string())?
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .is_some_and(|n| n.to_string_lossy().ends_with(".jsonl"))
            })
            .collect();
        files.sort();
        for f in files {
            let text = std::fs::read_to_string(&f).map_err(|e| e.to_string())?;
            for line in text.lines() {
                let Ok(r) = serde_json::from_str::<Value>(line) else {
                    continue;
                };
                if r.get("type").and_then(|v| v.as_str()) == Some("result") {
                    let id = r["id"].as_str().unwrap_or_default().to_string();
                    per.entry(id)
                        .or_default()
                        .push(Num::of(&r["cycles"]["median"]).unwrap());
                }
            }
        }
    }
    let mut out: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
    for (id, v) in per {
        if let Some((op, row)) = t.operation_of(&id) {
            out.entry(op)
                .or_default()
                .insert(t.library(&row), py::median(&v).f());
        }
    }
    Ok(out)
}

fn short(op: &str, re: &Regex) -> bool {
    match re.captures(op) {
        None => true,
        Some(c) => {
            let n: f64 = c[1].parse().unwrap_or(0.0);
            n * if &c[2] == "KB" { 1024.0 } else { 1.0 } < 1024.0
        }
    }
}

pub fn run(a: Args) -> Result<(), String> {
    let t = Tables::load(&a.rind)?;
    let mut layouts = vec![];
    for l in &a.layouts {
        let (label, dir) = l
            .split_once('=')
            .ok_or(format!("{l}: want LABEL=RAW_DIR"))?;
        layouts.push((label.to_string(), load(&t, &PathBuf::from(dir))?));
    }
    let size = Regex::new(r"/(\d+)(KB|B)$").unwrap();
    let labels: Vec<&str> = layouts.iter().map(|(l, _)| l.as_str()).collect();
    let mut lines = vec![
        format!(
            "rind's time over the fastest other Rust library, per layout ({}), each the median over that \
             layout's runs; then the range over layouts, and the ratio of the medians over layouts.",
            labels.join(", ")
        ),
        String::new(),
        format!(
            "| Operation | {} | range | median-layout ratio | rind's cycles, spread over layouts |",
            labels.join(" | ")
        ),
        format!("|---|{}---|---:|---:|", "---:|".repeat(labels.len())),
    ];
    let mut moved = vec![];
    for op in t.operations() {
        if a.short && !short(op, &size) {
            continue;
        }
        let mut ratios = vec![];
        let mut rind = vec![];
        let mut fastest: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
        for (_, l) in &layouts {
            let Some(libs) = l.get(op) else { break };
            let Some(r) = libs.get("rind") else { break };
            let Some((lib, best)) = libs
                .iter()
                .filter(|(k, _)| RIVALS.contains(&k.as_str()))
                .fold(None::<(&String, f64)>, |b, (k, v)| match b {
                    Some((_, bv)) if bv <= *v => b,
                    _ => Some((k, *v)),
                })
            else {
                break;
            };
            ratios.push((r / best, lib.clone()));
            rind.push(*r);
            for (k, v) in libs.iter().filter(|(k, _)| RIVALS.contains(&k.as_str())) {
                fastest.entry(k).or_default().push(*v);
            }
        }
        if ratios.len() != layouts.len() {
            continue;
        }
        let rs: Vec<f64> = ratios.iter().map(|x| x.0).collect();
        let (lo, hi) = (
            rs.iter().copied().fold(f64::INFINITY, f64::min),
            rs.iter().copied().fold(0.0, f64::max),
        );
        // The median over layouts of each library's cycles, then the ratio.
        let med = |v: &[f64]| py::median(&v.iter().map(|x| Num::Float(*x)).collect::<Vec<_>>()).f();
        let rm = med(&rind);
        let best = fastest
            .iter()
            .filter(|(_, v)| v.len() == layouts.len())
            .map(|(k, v)| (*k, med(v)))
            .fold(None::<(&str, f64)>, |b, (k, v)| match b {
                Some((_, bv)) if bv <= v => b,
                _ => Some((k, v)),
            });
        let Some((blib, bv)) = best else { continue };
        let rspread = 100.0
            * (rind.iter().copied().fold(0.0, f64::max)
                - rind.iter().copied().fold(f64::INFINITY, f64::min))
            / rm;
        let cells: Vec<String> = ratios
            .iter()
            .map(|(r, l)| format!("{}× {l}", py::fixed(*r, 2)))
            .collect();
        lines.push(format!(
            "| {} | {} | {}× to {}× | {}× {blib} | {}% |",
            op.replace("{lib}", "…"),
            cells.join(" | "),
            py::fixed(lo, 2),
            py::fixed(hi, 2),
            py::fixed(rm / bv, 2),
            py::fixed(rspread, 1)
        ));
        moved.push(hi - lo);
    }
    if !moved.is_empty() {
        moved.sort_by(|a, b| a.partial_cmp(b).unwrap());
        lines.push(String::new());
        lines.push(format!(
            "{} operations; the ratio's range over layouts: median {}, largest {}.",
            moved.len(),
            py::fixed(moved[moved.len() / 2], 2),
            py::fixed(moved[moved.len() - 1], 2)
        ));
    }
    let text = lines.join("\n") + "\n";
    match a.out {
        Some(p) => std::fs::write(p, text).map_err(|e| e.to_string()),
        None => {
            print!("{text}");
            Ok(())
        }
    }
}
