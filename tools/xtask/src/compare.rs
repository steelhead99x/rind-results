// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask compare`: two sets of board runs of the same benches, row by
//! row: a relinked build against the published one, a newer rind against the
//! pinned one, a cache configuration against the default.
//!
//! A_RAW and B_RAW hold `<bench>/run-*.jsonl` as capture writes them. Each
//! row's value is the median over runs of each run's median cycles. The
//! Markdown has, per bench: the known-answer checks of both sides, a
//! per-library summary of B/A split by input size (short: under 1 KB or no
//! size; long: 1 KB and up), and every row whose B/A falls outside max(1%,
//! either side's run spread).

use crate::py::{self, Num};
use regex::Regex;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(clap::Args)]
pub struct Args {
    a: PathBuf,
    b: PathBuf,
    #[arg(required = true)]
    benches: Vec<String>,
    #[arg(long, default_value = "A")]
    label_a: String,
    #[arg(long, default_value = "B")]
    label_b: String,
    #[arg(long)]
    out: Option<PathBuf>,
}

/// id -> (median cycles, run spread %, runs), and every check's result.
type Rows = BTreeMap<String, (f64, f64, usize)>;

fn load(raw: &Path, bench: &str) -> (Rows, Vec<bool>) {
    let mut files: Vec<PathBuf> = std::fs::read_dir(raw.join(bench))
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy();
            n.starts_with("run-") && n.ends_with(".jsonl")
        })
        .collect();
    files.sort();
    let mut rows: BTreeMap<String, Vec<Num>> = BTreeMap::new();
    let mut checks = vec![];
    for f in files {
        let text = String::from_utf8_lossy(&std::fs::read(&f).unwrap_or_default()).into_owned();
        for line in text.lines() {
            let Ok(r) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            match r.get("type").and_then(|v| v.as_str()) {
                Some("result") => rows
                    .entry(r["id"].as_str().unwrap_or_default().into())
                    .or_default()
                    .push(Num::of(&r["cycles"]["median"]).unwrap()),
                Some("check") => checks.push(r["pass"].as_bool().unwrap_or(false)),
                _ => {}
            }
        }
    }
    let out = rows
        .into_iter()
        .map(|(k, v)| {
            let m = py::median(&v).f();
            let spread = if m != 0.0 {
                100.0 * (py::max(&v).f() - py::min(&v).f()) / m
            } else {
                0.0
            };
            (k, (m, spread, v.len()))
        })
        .collect();
    (out, checks)
}

fn size_of(id: &str, re: &Regex) -> Option<f64> {
    let last = id.rsplit('/').next().unwrap_or(id);
    let c = re.captures(last)?;
    let n: f64 = c[1].parse().ok()?;
    Some(n * if &c[2] == "KB" { 1024.0 } else { 1.0 })
}

pub fn run(args: Args) -> Result<(), String> {
    let re = Regex::new(r"(\d+)(KB|B)$").unwrap();
    let (la, lb) = (&args.label_a, &args.label_b);
    let mut l = vec![
        format!("Cycles, {lb} over {la} (below 1 is faster in {lb})."),
        String::new(),
    ];
    for bench in &args.benches {
        let (a, ca) = load(&args.a, bench);
        let (b, cb) = load(&args.b, bench);
        let common: Vec<&String> = a
            .keys()
            .filter(|k| b.contains_key(*k) && !k.starts_with("harness/"))
            .collect();
        let only_b: BTreeSet<&String> = b.keys().filter(|k| !a.contains_key(*k)).collect();
        let runs = |m: &Rows| common.iter().map(|k| m[*k].2).max().unwrap_or(0);
        l.push(format!("## {bench}"));
        l.push(String::new());
        l.push(format!(
            "Known-answer checks: {la} {}/{}, {lb} {}/{}. Runs: {} and {}. Rows: {} in both{}.",
            ca.iter().filter(|x| **x).count(),
            ca.len(),
            cb.iter().filter(|x| **x).count(),
            cb.len(),
            runs(&a),
            runs(&b),
            common.len(),
            if only_b.is_empty() {
                String::new()
            } else {
                format!("; only in {lb}: {}", only_b.len())
            }
        ));
        l.push(String::new());
        let mut groups: BTreeMap<(String, &str), Vec<Num>> = BTreeMap::new();
        for k in &common {
            let p: Vec<&str> = k.split('/').collect();
            let lib = if p.len() > 2 { p[1] } else { p[p.len() - 1] };
            let kind = match size_of(k, &re) {
                None => "per op",
                Some(s) if s >= 1024.0 => "long",
                Some(_) => "short",
            };
            groups
                .entry((lib.to_string(), kind))
                .or_default()
                .push(Num::Float(b[*k].0 / a[*k].0));
        }
        l.push("| Library | Inputs | Rows | Median | Range |".into());
        l.push("|---|---|---:|---:|---|".into());
        for ((lib, kind), v) in &groups {
            l.push(format!(
                "| {lib} | {kind} | {} | {} | {} to {} |",
                v.len(),
                py::fixed(py::median(v).f(), 3),
                py::fixed(py::min(v).f(), 3),
                py::fixed(py::max(v).f(), 3)
            ));
        }
        let mut moved: Vec<(f64, &String)> = common
            .iter()
            .map(|k| (b[*k].0 / a[*k].0, *k))
            .filter(|(r, k)| (r - 1.0).abs() * 100.0 > 1f64.max(a[*k].1).max(b[*k].1))
            .collect();
        l.push(String::new());
        l.push(format!(
            "{} of {} rows moved by more than max(1%, run spread){}",
            moved.len(),
            common.len(),
            if moved.is_empty() { "." } else { ":" }
        ));
        l.push(String::new());
        if !moved.is_empty() {
            l.push(format!("| Row | {la} cycles | {lb} cycles | B/A |"));
            l.push("|---|---:|---:|---:|".into());
            moved.sort_by(|x, y| (y.0 - 1.0).abs().partial_cmp(&(x.0 - 1.0).abs()).unwrap());
            for (r, k) in moved {
                l.push(format!(
                    "| `{k}` | {} | {} | {} |",
                    py::fixed_commas(a[k].0, 0),
                    py::fixed_commas(b[k].0, 0),
                    py::fixed(r, 3)
                ));
            }
            l.push(String::new());
        }
        if !only_b.is_empty() {
            l.push(format!("Rows only in {lb}:"));
            l.push(String::new());
            l.push("| Row | cycles | spread | c/B |".into());
            l.push("|---|---:|---:|---:|".into());
            for k in only_b {
                let (c, s, _) = b[k];
                l.push(match size_of(k, &re) {
                    Some(n) if n != 0.0 => format!(
                        "| `{k}` | {} | {}% | {} |",
                        py::fixed_commas(c, 0),
                        py::fixed(s, 2),
                        py::fixed(c / n, 1)
                    ),
                    _ => format!(
                        "| `{k}` | {} | {}% | |",
                        py::fixed_commas(c, 0),
                        py::fixed(s, 2)
                    ),
                });
            }
            l.push(String::new());
        }
    }
    let text = l.join("\n") + "\n";
    match &args.out {
        Some(p) => std::fs::write(p, text).map_err(|e| e.to_string()),
        None => {
            println!("{text}");
            Ok(())
        }
    }
}
