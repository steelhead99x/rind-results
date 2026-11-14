// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo verify --dir`: recognise a result tree and check it with the same
//! rules as the published paths, without those paths being named in the binary.
//!
//! A directory can be any of:
//! - a host publish (`raw/run-*.json`, `<host>.json`, RESULTS.md, RUNS.md)
//! - a board publish (jsonl runs in the directory, in `raw/`, or in a sibling
//!   `raw-<name>/`, plus RESULTS.md when one was written)
//! - an `msi-001`-style `summary.md` next to a schema-1 JSON
//! - a `rind-results/ab-v1` JSON with its sibling markdown
//! - aggregated criterion tables (`id | mode | runs | median | min | max | spread`)
//! - a parent of those, including a compare-*.md over board subdirectories

use super::{Report, check_board_at, check_host_at, check_summary, load_hw_map, med};

/// Comparable set: kixel-ai/rind OPRF #705. Same constant as `xtask::layout`.
const COMPARABLE_PIN: &str = "d6ca9a756d3d1e71489e61e44c04f33c3c893589";
/// Previous comparable set. Kept, and not mixed into [`COMPARABLE_PIN`] tables.
const PREVIOUS_COMPARABLE_PIN: &str = "0b1f1469c8e17e361e3353432bc7fceee9e04214";
use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub fn verify_dir(dir: &Path, rep: &mut Report) -> bool {
    let dir = match std::fs::canonicalize(dir) {
        Ok(p) => p,
        Err(e) => {
            rep.bad(&dir.display().to_string(), format!("not a directory ({e})"));
            return false;
        }
    };
    println!("verify --dir {}", dir.display());
    walk(&dir, rep)
}

fn walk(dir: &Path, rep: &mut Report) -> bool {
    let label = dir
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| dir.display().to_string());
    let mut found = false;

    if has_host_runs(dir) {
        match host_stem(dir) {
            Some(stem) => {
                check_host_at(dir, &stem, rep);
                found = true;
            }
            None => rep.bad(
                &label,
                "raw/run-*.json but no schema-1 <host>.json to check them against".into(),
            ),
        }
    } else if let Some(raw) = board_raw(dir) {
        let md = results_md(dir, &raw);
        let svg = chart_svg(dir);
        let tsv = demo_tsv(dir, md.as_deref());
        check_board_at(
            &label,
            &raw,
            md.as_deref(),
            svg.as_deref(),
            tsv.as_deref(),
            None,
            rep,
        );
        if let Some(images) = images_json(dir, &raw) {
            check_images(&label, &images, md.as_deref(), rep);
        }
        found = true;
    }

    if let Some((json, summary)) = summary_pair(dir) {
        let name = json
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| label.clone());
        check_summary(&name, &json, &summary, rep);
        found = true;
    }

    if check_ab_dir(dir, rep) {
        found = true;
    }
    if check_agg_dir(dir, rep) {
        found = true;
    }
    if check_compares(dir, rep) {
        found = true;
    }
    if check_cycle_median_dir(dir, rep) {
        found = true;
    }

    let raw_used = board_raw(dir);
    let mut children: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter(|p| {
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            !skip_walk_child(&name)
        })
        .collect();
    children.sort();
    let history = history_dirs(&children);
    for h in &history {
        let name = h.file_name().unwrap_or_default().to_string_lossy();
        let previous =
            revision_sha(&name).is_some_and(|sha| sha_matches(sha, PREVIOUS_COMPARABLE_PIN));
        let kind = if previous {
            format!("history from the previous comparable set {PREVIOUS_COMPARABLE_PIN}")
        } else {
            "an older set".to_string()
        };
        rep.note(
            &label,
            format!("{name} is {kind}; not in {COMPARABLE_PIN} tables"),
        );
    }
    for child in children {
        if history.iter().any(|h| h == &child) {
            continue;
        }
        if raw_used.as_ref().is_some_and(|r| r == &child) {
            continue;
        }
        // A bench folder holds run-*.jsonl directly; the board check already read it.
        if jsonl_files(&child).next().is_some() {
            continue;
        }
        let interesting = !found || looks_like_tree(&child);
        if interesting && walk(&child, rep) {
            found = true;
        }
    }

    if !found {
        if jsonl_files(dir).next().is_some() {
            rep.note(
                &label,
                "raw runs only; no RESULTS.md or host JSON to check".into(),
            );
            return true;
        }
        rep.bad(
            &label,
            "no host runs, board runs, summary, ab-v1 JSON, or aggregated table".into(),
        );
    }
    found
}

fn revision_sha(name: &str) -> Option<&str> {
    let sha = name.split_once('-').map(|(s, _)| s).unwrap_or(name);
    if (7..=40).contains(&sha.len())
        && sha
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        Some(sha)
    } else {
        None
    }
}

fn sha_matches(sha: &str, pin: &str) -> bool {
    let (short, long) = if sha.len() <= pin.len() {
        (sha, pin)
    } else {
        (pin, sha)
    };
    short.len() >= 7 && long.starts_with(short)
}

/// Older pin directories, and only when the current pin is also a child.
/// A directory that holds just the previous set is not history of itself.
fn history_dirs(children: &[PathBuf]) -> Vec<PathBuf> {
    let mut saw_current = false;
    let mut revisions = vec![];
    for c in children {
        let Some(name) = c.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(sha) = revision_sha(name) else {
            continue;
        };
        if sha_matches(sha, COMPARABLE_PIN) {
            saw_current = true;
        }
        revisions.push(c.clone());
    }
    if !saw_current {
        return vec![];
    }
    revisions
        .into_iter()
        .filter(|c| {
            c.file_name()
                .and_then(|s| s.to_str())
                .and_then(revision_sha)
                .is_some_and(|sha| !sha_matches(sha, COMPARABLE_PIN))
        })
        .collect()
}

fn has_host_runs(dir: &Path) -> bool {
    host_run_files(dir).next().is_some()
}

fn host_run_files(dir: &Path) -> impl Iterator<Item = PathBuf> {
    let raw = dir.join("raw");
    std::fs::read_dir(raw)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            (name.starts_with("run-") && name.ends_with(".json")).then(|| e.path())
        })
}

fn host_stem(dir: &Path) -> Option<String> {
    let mut hits: Vec<String> = vec![];
    for e in std::fs::read_dir(dir).ok()?.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if !name.ends_with(".json") || name.ends_with("-all.json") || name == "images.json" {
            continue;
        }
        let Ok(v) = read_json(&e.path()) else {
            continue;
        };
        if v["schema"].as_i64() == Some(1) && v.get("results").and_then(|r| r.as_array()).is_some()
        {
            hits.push(name.trim_end_matches(".json").to_string());
        }
    }
    let dir_name = dir.file_name()?.to_string_lossy().into_owned();
    if hits.iter().any(|s| s == &dir_name) {
        return Some(dir_name);
    }
    if hits.len() == 1 { hits.pop() } else { None }
}

fn board_raw(dir: &Path) -> Option<PathBuf> {
    let raw = dir.join("raw");
    if has_benches(&raw) {
        return Some(raw);
    }
    if has_benches(dir) {
        return Some(dir.to_path_buf());
    }
    let name = dir.file_name()?.to_string_lossy();
    if !name.starts_with("raw-") {
        let sib = dir.parent()?.join(format!("raw-{name}"));
        if has_benches(&sib) {
            return Some(sib);
        }
    }
    None
}

/// A bench folder is `<bench>/run-*.jsonl` and nothing nested under it.
fn is_flat_bench(dir: &Path) -> bool {
    let mut jsonl = false;
    let mut nested = false;
    let Ok(rd) = std::fs::read_dir(dir) else {
        return false;
    };
    for e in rd.flatten() {
        if e.path().is_dir() {
            nested = true;
        } else {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.starts_with("run-") && n.ends_with(".jsonl") {
                jsonl = true;
            }
        }
    }
    jsonl && !nested
}

/// `dir` is one capture (`raw/<bench>/run-*.jsonl`) and not a parent of several
/// result trees. A directory such as `results/diag` holds many experiments;
/// folding them into one board mixes clocks, pins, and the parent's RESULTS.md.
fn has_benches(dir: &Path) -> bool {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return false;
    };
    let mut any = false;
    for e in rd.flatten() {
        let p = e.path();
        if !p.is_dir() || e.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if p.join("RESULTS.md").is_file()
            || p.join("summary.md").is_file()
            || p.join("raw").is_dir()
        {
            return false;
        }
        if is_flat_bench(&p) {
            any = true;
        } else if jsonl_files(&p).next().is_some() {
            return false;
        }
    }
    any
}

fn jsonl_files(dir: &Path) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            (name.starts_with("run-") && name.ends_with(".jsonl")).then(|| e.path())
        })
}

fn results_md(dir: &Path, raw: &Path) -> Option<PathBuf> {
    let mut cands = vec![dir.join("RESULTS.md")];
    if raw != dir {
        cands.push(raw.join("RESULTS.md"));
    }
    // `raw-<board>/` sits beside `<board>/RESULTS.md`. The parent of the
    // directory being checked has its own report; don't attach that here.
    if let Some(name) = raw.file_name().and_then(|n| n.to_str()) {
        if let Some(board) = name.strip_prefix("raw-") {
            if let Some(parent) = raw.parent() {
                cands.push(parent.join(board).join("RESULTS.md"));
            }
        }
    }
    cands.into_iter().find(|p| p.is_file())
}

fn chart_svg(dir: &Path) -> Option<PathBuf> {
    // A chart belongs to this tree when it sits in the directory, or when the
    // directory is the published board (media/charts/<name>-rind-vs-fastest.svg).
    // A JSON named esp32s3.json inside some other tree is not that chart.
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.ends_with("-rind-vs-fastest.svg") {
                return Some(e.path());
            }
        }
    }
    let name = dir.file_name()?.to_str()?;
    let in_repo = super::root()
        .join("media/charts")
        .join(format!("{name}-rind-vs-fastest.svg"));
    in_repo.is_file().then_some(in_repo)
}

/// `demo.tsv` belongs to the directory that holds it, or sits beside RESULTS.md.
/// A sibling `raw-<board>/` must not pick up another board's table in the parent.
fn demo_tsv(dir: &Path, md: Option<&Path>) -> Option<PathBuf> {
    let own = dir.join("demo.tsv");
    if own.is_file() {
        return Some(own);
    }
    md.and_then(|m| {
        let sib = m.with_file_name("demo.tsv");
        sib.is_file().then_some(sib)
    })
}

fn images_json(dir: &Path, raw: &Path) -> Option<PathBuf> {
    let mut cands = vec![dir.join("images.json")];
    if let Some(parent) = raw.parent() {
        if let Some(name) = raw.file_name().and_then(|n| n.to_str()) {
            if let Some(board) = name.strip_prefix("raw-") {
                cands.push(parent.join(board).join("images.json"));
            }
        }
    }
    cands.into_iter().find(|p| p.is_file())
}

fn summary_pair(dir: &Path) -> Option<(PathBuf, PathBuf)> {
    let summary = dir.join("summary.md");
    if !summary.is_file() {
        return None;
    }
    let mut hits: Vec<PathBuf> = vec![];
    for e in std::fs::read_dir(dir).ok()?.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if !name.ends_with(".json") {
            continue;
        }
        let Ok(v) = read_json(&e.path()) else {
            continue;
        };
        if v["schema"].as_i64() == Some(1) && v.get("results").and_then(|r| r.as_array()).is_some()
        {
            hits.push(e.path());
        }
    }
    if hits.len() == 1 {
        hits.pop().map(|j| (j, summary))
    } else {
        None
    }
}

fn skip_walk_child(name: &str) -> bool {
    name.starts_with('.')
        || matches!(
            name,
            "checks" | "raw" | "firmware" | "host" | "src" | "target" | "venv"
        )
}

fn looks_like_tree(dir: &Path) -> bool {
    has_host_runs(dir)
        || board_raw(dir).is_some()
        || dir.join("summary.md").is_file()
        || dir.join("RESULTS.md").is_file()
        || ab_jsons(dir).next().is_some()
        || has_agg_table(dir)
        || compare_mds(dir).next().is_some()
        || is_cycle_median_tree(dir)
}

/// A published cycle-median tree (Arduino Uno R3 and similar): medians.json
/// plus RUNS.md and RESULTS.md, with optional firmware/ and host/ sources.
fn is_cycle_median_tree(dir: &Path) -> bool {
    dir.join("medians.json").is_file()
        && dir.join("RUNS.md").is_file()
        && dir.join("RESULTS.md").is_file()
}

fn check_cycle_median_dir(dir: &Path, rep: &mut Report) -> bool {
    if !is_cycle_median_tree(dir) {
        return false;
    }
    let label = dir
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| dir.display().to_string());
    check_cycle_median(&label, dir, rep);
    true
}

fn parse_u64_commas(s: &str) -> Option<u64> {
    let trimmed = s.trim().trim_matches('`');
    let token = trimmed.split_whitespace().next().unwrap_or(trimmed);
    let digits: String = token.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

fn median_u64(xs: &[u64]) -> Option<u64> {
    if xs.is_empty() {
        return None;
    }
    let mut v = xs.to_vec();
    v.sort_unstable();
    let n = v.len();
    if n % 2 == 1 {
        Some(v[n / 2])
    } else {
        Some((v[n / 2 - 1] + v[n / 2]) / 2)
    }
}

fn md_tables(text: &str) -> Vec<(Vec<String>, Vec<Vec<String>>)> {
    let mut out = Vec::new();
    let mut header: Option<Vec<String>> = None;
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut seen_sep = false;
    for line in text.lines() {
        let t = line.trim();
        if !(t.starts_with('|') && t.ends_with('|')) {
            if let Some(h) = header.take() {
                if !rows.is_empty() {
                    out.push((h, std::mem::take(&mut rows)));
                }
            }
            seen_sep = false;
            continue;
        }
        let cells: Vec<String> = t
            .trim_matches('|')
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();
        if cells
            .iter()
            .all(|c| c.chars().all(|ch| ch == '-' || ch == ':'))
        {
            seen_sep = true;
            continue;
        }
        if header.is_none() {
            header = Some(cells);
            seen_sep = false;
            continue;
        }
        if !seen_sep && rows.is_empty() {
            // a new header without a separator; close previous (shouldn't happen)
            header = Some(cells);
            continue;
        }
        rows.push(cells);
    }
    if let Some(h) = header {
        if !rows.is_empty() {
            out.push((h, rows));
        }
    }
    out
}

fn header_has(header: &[String], needle: &str) -> bool {
    header
        .iter()
        .any(|h| h.to_ascii_lowercase().contains(needle))
}

fn col_index(header: &[String], needles: &[&str]) -> Option<usize> {
    header.iter().position(|h| {
        let l = h.to_ascii_lowercase();
        needles.iter().any(|n| l.contains(n))
    })
}

struct CycleOp {
    cycles_all: Vec<u64>,
    cycles_median: u64,
    cycles_min: u64,
    cycles_max: u64,
    us_median: u64,
}

fn load_cycle_ops(
    json: &Value,
    label: &str,
    clock_hz: u64,
    rep: &mut Report,
) -> BTreeMap<String, CycleOp> {
    let mut out = BTreeMap::new();
    let Some(ops) = json.get("ops").and_then(|v| v.as_object()) else {
        rep.bad(label, "medians.json has no ops object".into());
        return out;
    };
    for (name, rec) in ops {
        let Some(all) = rec.get("cycles_all").and_then(|v| v.as_array()) else {
            rep.bad(label, format!("{name}: missing cycles_all"));
            continue;
        };
        let mut cycles_all = Vec::new();
        for (i, v) in all.iter().enumerate() {
            match v.as_u64() {
                Some(n) => cycles_all.push(n),
                None => {
                    // JSON may store these as i64
                    if let Some(n) = v.as_i64().filter(|n| *n >= 0) {
                        cycles_all.push(n as u64);
                    } else {
                        rep.bad(label, format!("{name}: cycles_all[{i}] is not an integer"));
                    }
                }
            }
        }
        let n_field = rec.get("n").and_then(|v| v.as_u64()).unwrap_or(0);
        if n_field != 0 && n_field as usize != cycles_all.len() {
            rep.bad(
                label,
                format!(
                    "{name}: n is {n_field}, but cycles_all has {} values",
                    cycles_all.len()
                ),
            );
        }
        let Some(want_med) = median_u64(&cycles_all) else {
            rep.bad(label, format!("{name}: empty cycles_all"));
            continue;
        };
        let want_min = *cycles_all.iter().min().unwrap();
        let want_max = *cycles_all.iter().max().unwrap();
        let got_med = rec.get("cycles_median").and_then(|v| v.as_u64());
        let got_min = rec.get("cycles_min").and_then(|v| v.as_u64());
        let got_max = rec.get("cycles_max").and_then(|v| v.as_u64());
        let got_us = rec.get("us_median").and_then(|v| v.as_u64());
        if got_med != Some(want_med) {
            rep.bad(
                label,
                format!("{name}: cycles_median is {got_med:?}, median of cycles_all is {want_med}"),
            );
        }
        if got_min != Some(want_min) {
            rep.bad(
                label,
                format!("{name}: cycles_min is {got_min:?}, min of cycles_all is {want_min}"),
            );
        }
        if got_max != Some(want_max) {
            rep.bad(
                label,
                format!("{name}: cycles_max is {got_max:?}, max of cycles_all is {want_max}"),
            );
        }
        if let Some(want_us) = want_med.saturating_mul(1_000_000).checked_div(clock_hz) {
            if got_us != Some(want_us) {
                rep.bad(
                    label,
                    format!("{name}: us_median is {got_us:?}, cycles/clock is {want_us}"),
                );
            }
        }
        out.insert(
            name.clone(),
            CycleOp {
                cycles_all,
                cycles_median: got_med.unwrap_or(want_med),
                cycles_min: got_min.unwrap_or(want_min),
                cycles_max: got_max.unwrap_or(want_max),
                us_median: got_us.unwrap_or(0),
            },
        );
    }
    out
}

fn check_cycle_median(label: &str, dir: &Path, rep: &mut Report) {
    let json_path = dir.join("medians.json");
    let Ok(json) = read_json(&json_path) else {
        rep.bad(label, "medians.json: not JSON".into());
        return;
    };
    let clock_hz = json.get("clock_hz").and_then(|v| v.as_u64()).unwrap_or(0);
    if clock_hz == 0 {
        rep.bad(label, "medians.json has no clock_hz".into());
    }
    let ops = load_cycle_ops(&json, label, clock_hz, rep);
    if ops.is_empty() {
        return;
    }

    let runs = super::read(&dir.join("RUNS.md"));
    let mut runs_checked = 0usize;
    let mut seen = std::collections::BTreeSet::new();
    for (header, rows) in md_tables(&runs) {
        if !(header_has(&header, "median") && header_has(&header, "spread")) {
            continue;
        }
        let Some(op_i) = col_index(&header, &["operation"]) else {
            continue;
        };
        let Some(med_i) = col_index(&header, &["median"]) else {
            continue;
        };
        let Some(spread_i) = col_index(&header, &["spread"]) else {
            continue;
        };
        let run_cols: Vec<usize> = header
            .iter()
            .enumerate()
            .filter(|(_, h)| {
                let l = h.to_ascii_lowercase();
                l.starts_with("run") || l.contains("run ")
            })
            .map(|(i, _)| i)
            .collect();
        for row in rows {
            if row.len() <= med_i.max(spread_i).max(op_i) {
                continue;
            }
            let name = row[op_i].trim().trim_matches('`').to_string();
            let Some(op) = ops.get(&name) else {
                continue;
            };
            seen.insert(name.clone());
            runs_checked += 1;
            let Some(shown_med) = parse_u64_commas(&row[med_i]) else {
                rep.bad(label, format!("RUNS.md {name}: median is not an integer"));
                continue;
            };
            if shown_med != op.cycles_median {
                rep.bad(
                    label,
                    format!(
                        "RUNS.md {name}: median is {shown_med}, medians.json has {}",
                        op.cycles_median
                    ),
                );
            }
            let mut shown_runs = Vec::new();
            for i in &run_cols {
                if *i < row.len() {
                    if let Some(v) = parse_u64_commas(&row[*i]) {
                        shown_runs.push(v);
                    }
                }
            }
            if !shown_runs.is_empty() && shown_runs != op.cycles_all {
                rep.bad(
                    label,
                    format!("RUNS.md {name}: per-run cycles do not match medians.json"),
                );
            }
            let spread_text = row[spread_i].trim().trim_end_matches('%');
            if let Ok(shown) = spread_text.replace(',', "").parse::<f64>() {
                let spread = if op.cycles_median == 0 {
                    0.0
                } else {
                    (op.cycles_max - op.cycles_min) as f64 / op.cycles_median as f64 * 100.0
                };
                let shown_half = 0.5 * 10f64.powi(-decimals(spread_text));
                let rounds_to_zero = shown == 0.0 && spread < 0.005 + 1e-12;
                if !rounds_to_zero
                    && (shown + shown_half + 1e-9 < spread || shown - shown_half - 1e-9 > spread)
                {
                    rep.bad(
                        label,
                        format!(
                            "RUNS.md {name}: spread shows {shown}%, (max-min)/median is {spread:.4}%"
                        ),
                    );
                }
            }
        }
        break; // only the first published (median/spread) table
    }
    for name in ops.keys() {
        if !seen.contains(name) {
            rep.bad(label, format!("RUNS.md is missing published op {name}"));
        }
    }

    let mut md_checked = 0usize;
    for rel in ["RESULTS.md", "medians.md"] {
        let path = dir.join(rel);
        if !path.is_file() {
            continue;
        }
        let text = super::read(&path);
        for (header, rows) in md_tables(&text) {
            if !(header_has(&header, "cycles")
                && (header_has(&header, "µs") || header_has(&header, "us")))
            {
                continue;
            }
            // Prefer the headline table whose cycle column is this board's, not a comparison.
            // Skip tables that also have another board's cycle column if the first cycle
            // column is not paired with a µs column for the same board... We check every
            // row whose operation is in ops, against the first cycles column and first µs column.
            let Some(op_i) = col_index(&header, &["operation"]) else {
                continue;
            };
            let Some(cyc_i) = col_index(&header, &["cycles"]) else {
                continue;
            };
            let Some(us_i) = col_index(&header, &["µs", "us"]) else {
                continue;
            };
            // Comparison tables list other boards after the first cycles column.
            // Only treat this as the headline table when the first cycles column
            // sits before any later board column — still verify matching ops.
            let mut n = 0usize;
            for row in &rows {
                if row.len() <= op_i.max(cyc_i).max(us_i) {
                    continue;
                }
                let name = row[op_i].trim().trim_matches('`').to_string();
                let Some(op) = ops.get(&name) else {
                    continue;
                };
                n += 1;
                if let Some(shown) = parse_u64_commas(&row[cyc_i]) {
                    if shown != op.cycles_median {
                        rep.bad(
                            label,
                            format!(
                                "{rel} {name}: cycles are {shown}, medians.json has {}",
                                op.cycles_median
                            ),
                        );
                    }
                }
                if let Some(shown) = parse_u64_commas(&row[us_i]) {
                    if shown != op.us_median {
                        rep.bad(
                            label,
                            format!(
                                "{rel} {name}: µs are {shown}, medians.json has {}",
                                op.us_median
                            ),
                        );
                    }
                }
                if let Some(range_i) = col_index(&header, &["range"]) {
                    if range_i < row.len() {
                        let range = row[range_i].replace(',', "");
                        let sep = if range.contains('–') { '–' } else { '-' };
                        let parts: Vec<&str> = range.split(sep).collect();
                        if parts.len() == 2 {
                            if let (Some(lo), Some(hi)) =
                                (parse_u64_commas(parts[0]), parse_u64_commas(parts[1]))
                            {
                                if lo != op.cycles_min || hi != op.cycles_max {
                                    rep.bad(
                                        label,
                                        format!(
                                            "{rel} {name}: range is {}, medians.json is {}–{}",
                                            row[range_i], op.cycles_min, op.cycles_max
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            if n > 0 {
                md_checked += n;
                break;
            }
        }
    }
    println!(
        "{label}: {} cycle-median ops, {runs_checked} RUNS.md rows, {md_checked} RESULTS/medians cells checked",
        ops.len()
    );
}

/// An aggregated criterion table, not every Markdown file. A README next to
/// logs is a note, not a result tree.
fn has_agg_table(dir: &Path) -> bool {
    for path in [dir.to_path_buf(), dir.join("raw")] {
        let Ok(rd) = std::fs::read_dir(&path) else {
            continue;
        };
        for e in rd.flatten() {
            if e.path().extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let text = super::read(&e.path());
            if text.contains("| id |") && text.contains("| mode |") && text.contains("| spread |") {
                return true;
            }
        }
    }
    false
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

/// Images that record a legacy rind-bench SHA must map the way the published
/// trees do. Images without that field (built from rind-hardware itself) are
/// left alone.
fn check_images(label: &str, images: &Path, results_md: Option<&Path>, rep: &mut Report) {
    let Ok(v) = read_json(images) else {
        rep.bad(label, format!("{}: not JSON", images.display()));
        return;
    };
    super::check_patch_list(label, &v["patches"], rep);
    super::check_upstream_pin(label, &v, rep);
    let Some(imgs) = v["images"].as_object() else {
        return;
    };
    let map = load_hw_map();
    let mut checked = 0;
    for (name, i) in imgs {
        let rec = i
            .get("legacy_rind_bench")
            .and_then(|x| x.as_str())
            .unwrap_or("");
        if rec.is_empty() {
            continue;
        }
        checked += 1;
        let want = super::expected_hw(&map, rec);
        let have = i
            .get("rind_hardware")
            .and_then(|x| x.as_str())
            .unwrap_or("");
        if have != want {
            rep.bad(
                label,
                format!("{name}: rind_hardware {have:?}, map says {want}"),
            );
        }
    }
    if checked == 0 {
        return;
    }
    let Some(md_path) = results_md else {
        println!("{label}: provenance {checked} legacy images, no RESULTS.md table");
        return;
    };
    let text = super::read(md_path);
    let mut in_table = false;
    let mut rows = 0;
    for line in text.lines() {
        if line.contains("| Image |") && line.contains("legacy rind-bench") {
            in_table = true;
            continue;
        }
        if in_table && (line.starts_with("  |---") || line.starts_with("|---")) {
            continue;
        }
        if in_table && (line.starts_with("  | ") || line.starts_with("| ")) {
            let cols: Vec<&str> = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect();
            if cols.len() < 6 {
                in_table = false;
                continue;
            }
            let name = cols[0];
            let Some(i) = imgs.get(name) else { continue };
            let rec = i
                .get("legacy_rind_bench")
                .and_then(|x| x.as_str())
                .unwrap_or("");
            if rec.is_empty() {
                continue;
            }
            rows += 1;
            let want_hw = super::hw_cell(
                i.get("rind_hardware")
                    .and_then(|x| x.as_str())
                    .unwrap_or(""),
            );
            if cols[3] != want_hw {
                rep.bad(
                    label,
                    format!("{name}: hardware col {:?}, want {want_hw}", cols[3]),
                );
            }
            if cols[4] != rec {
                rep.bad(
                    label,
                    format!("{name}: legacy col {:?}, want {rec}", cols[4]),
                );
            }
            continue;
        }
        in_table = false;
    }
    println!("{label}: provenance {checked} legacy images, {rows} RESULTS.md rows");
}

fn ab_jsons(dir: &Path) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if !name.ends_with(".json") {
                return None;
            }
            let v = read_json(&e.path()).ok()?;
            (v["schema"].as_str() == Some("rind-results/ab-v1")).then(|| e.path())
        })
}

fn check_ab_dir(dir: &Path, rep: &mut Report) -> bool {
    let mut any = false;
    for path in ab_jsons(dir) {
        any = true;
        check_ab(&path, rep);
    }
    any
}

fn check_ab(path: &Path, rep: &mut Report) {
    let label = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let Ok(v) = read_json(path) else {
        rep.bad(&label, format!("{}: not JSON", path.display()));
        return;
    };
    // (operation, library, mode) -> one ns per round, in round order.
    let mut rows: BTreeMap<(String, String, String), Vec<f64>> = BTreeMap::new();
    for r in v["results"].as_array().into_iter().flatten() {
        let key = (
            r["operation"].as_str().unwrap_or("").to_string(),
            r["library"].as_str().unwrap_or("").to_string(),
            r["mode"].as_str().unwrap_or("").to_string(),
        );
        rows.entry(key).or_default().push(super::fv(&r["ns"]));
    }
    let md = path.with_extension("md");
    if !md.is_file() {
        rep.bad(&label, format!("no {}", md.display()));
        return;
    }
    let text = super::read(&md);
    let mut header: Vec<String> = vec![];
    let mut checked = 0;
    for line in text.lines() {
        if !line.starts_with('|') {
            header.clear();
            continue;
        }
        let cells: Vec<String> = line
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_string())
            .collect();
        if cells.first().is_some_and(|c| c == "op" || c == "Operation") {
            header = cells;
            continue;
        }
        if header.is_empty() || cells.len() != header.len() || cells[0].starts_with('-') {
            continue;
        }
        let op_label = &cells[0];
        let ops: Vec<&String> = rows
            .keys()
            .map(|(op, _, _)| op)
            .filter(|op| super::label(op) == *op_label)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        if ops.len() != 1 {
            rep.bad(
                &label,
                format!("{op_label}: matched {} operations", ops.len()),
            );
            continue;
        }
        let op = ops[0];
        for (col, cell) in header.iter().zip(cells.iter()).skip(1) {
            if let Some((mode, scale)) = mode_of(col) {
                checked += 1;
                check_mode_cell(&label, op, mode, scale, cell, &rows, rep);
            } else if let Some((lib, unit)) = lib_of(col) {
                checked += 1;
                check_lib_cell(&label, op, lib, unit, cell, &rows, rep);
            } else if let Some((num, den)) = ratio_of(col) {
                checked += 1;
                check_ratio_cell(&label, op, num, den, cell, &rows, rep);
            }
        }
    }
    println!("{label}: ab-v1 {checked} cells checked");
}

fn mode_of(col: &str) -> Option<(&str, f64)> {
    let (mode, rest) = col.split_once(' ')?;
    if !rest.contains("(n, spread)") {
        return None;
    }
    let unit = rest.split_whitespace().next()?;
    Some((mode, unit_scale(unit)?))
}

fn lib_of(col: &str) -> Option<(&str, f64)> {
    let (name, unit) = col.rsplit_once(' ')?;
    Some((name, unit_scale(unit)?))
}

fn ratio_of(col: &str) -> Option<(&str, &str)> {
    let (a, b) = col.split_once('/')?;
    if a.contains(' ') || b.contains(' ') {
        return None;
    }
    Some((a, b))
}

fn unit_scale(unit: &str) -> Option<f64> {
    Some(match unit {
        "ns" => 1.0,
        "µs" | "us" => 1e3,
        "ms" => 1e6,
        "s" => 1e9,
        _ => return None,
    })
}

fn samples<'a>(
    rows: &'a BTreeMap<(String, String, String), Vec<f64>>,
    op: &str,
    lib: &str,
    mode: &str,
) -> Option<&'a [f64]> {
    rows.get(&(op.to_string(), lib.to_string(), mode.to_string()))
        .map(|v| v.as_slice())
}

fn check_mode_cell(
    label: &str,
    op: &str,
    mode: &str,
    scale: f64,
    cell: &str,
    rows: &BTreeMap<(String, String, String), Vec<f64>>,
    rep: &mut Report,
) {
    let xs = samples(rows, op, "rind", mode);
    let dash = cell == "–" || cell == "-" || cell == "—";
    if dash {
        if xs.is_some() {
            rep.bad(label, format!("{op} {mode}: shown '{cell}' but raw has it"));
        }
        return;
    }
    let Some(xs) = xs else {
        rep.bad(label, format!("{op} {mode}: shown '{cell}' but not in raw"));
        return;
    };
    let re = Regex::new(r"^([\d.]+) \((\d+), ([\d.]+)%\)$").unwrap();
    let Some(m) = re.captures(cell.trim()) else {
        rep.bad(label, format!("{op} {mode}: unparsed '{cell}'"));
        return;
    };
    let shown: f64 = m[1].parse().unwrap_or(f64::NAN);
    let n: usize = m[2].parse().unwrap_or(0);
    let spread: f64 = m[3].parse().unwrap_or(f64::NAN);
    let med_ns = med(xs);
    if !super::close(
        shown * scale,
        0.5 * scale * 10f64.powi(-decimals(&m[1])),
        med_ns,
    ) {
        rep.bad(
            label,
            format!("{op} {mode}: shows {cell}, raw median {med_ns:.1} ns"),
        );
    }
    if n != xs.len() {
        rep.bad(
            label,
            format!("{op} {mode}: shows {n} runs, raw has {}", xs.len()),
        );
    }
    let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    // These tables print (max − min) / min, not over the median.
    let raw_spread = if lo == 0.0 {
        0.0
    } else {
        (hi - lo) / lo * 100.0
    };
    if (spread - raw_spread).abs() > 0.05 + 1e-9 {
        rep.bad(
            label,
            format!("{op} {mode}: spread shows {spread}%, raw {raw_spread:.2}%"),
        );
    }
}

fn decimals(s: &str) -> i32 {
    s.split_once('.').map_or(0, |(_, d)| d.len() as i32)
}

fn check_lib_cell(
    label: &str,
    op: &str,
    lib: &str,
    scale: f64,
    cell: &str,
    rows: &BTreeMap<(String, String, String), Vec<f64>>,
    rep: &mut Report,
) {
    let groups: Vec<&[f64]> = rows
        .iter()
        .filter(|((o, l, _), _)| o == op && l == lib)
        .map(|(_, xs)| xs.as_slice())
        .collect();
    if groups.is_empty() {
        rep.bad(label, format!("{op} {lib}: shown '{cell}' but not in raw"));
        return;
    }
    let shown: f64 = cell.replace(',', "").parse().unwrap_or(f64::NAN);
    let tol = 0.5 * scale * 10f64.powi(-decimals(cell.trim()));
    // One number for every mode is the median of every sample of that library.
    let pooled: Vec<f64> = groups.iter().flat_map(|xs| xs.iter().copied()).collect();
    let ok = super::close(shown * scale, tol, med(&pooled));
    if !ok {
        rep.bad(
            label,
            format!("{op} {lib}: shows {cell}, no mode median matches"),
        );
    }
}

fn library_name(name: &str) -> &str {
    match name {
        "aws" => "aws-lc-rs",
        other => other,
    }
}

/// The single library column: the median of every sample of that library.
fn lib_level(
    rows: &BTreeMap<(String, String, String), Vec<f64>>,
    op: &str,
    lib: &str,
) -> Option<f64> {
    let pooled: Vec<f64> = rows
        .iter()
        .filter(|((o, l, _), _)| o == op && l == lib)
        .flat_map(|(_, xs)| xs.iter().copied())
        .collect();
    if pooled.is_empty() {
        None
    } else {
        Some(med(&pooled))
    }
}

fn check_ratio_cell(
    label: &str,
    op: &str,
    num: &str,
    den: &str,
    cell: &str,
    rows: &BTreeMap<(String, String, String), Vec<f64>>,
    rep: &mut Report,
) {
    let dash = cell == "–" || cell == "-" || cell == "—";
    let rind_num = samples(rows, op, "rind", num).map(med);
    let rind_den = samples(rows, op, "rind", den).map(med);
    let lib_den = lib_level(rows, op, library_name(den));
    let (a, b) = if let (Some(a), Some(b)) = (rind_num, rind_den) {
        (a, b)
    } else if let (Some(a), Some(b)) = (rind_num, lib_den) {
        // `nist/aws` is rind's nist median over the aws-lc-rs column.
        (a, b)
    } else {
        if !dash {
            rep.bad(
                label,
                format!("{op} {num}/{den}: shown {cell}, missing a mode"),
            );
        }
        return;
    };
    if dash {
        rep.bad(label, format!("{op} {num}/{den}: shown dash, raw has both"));
        return;
    }
    let shown: f64 = cell.parse().unwrap_or(f64::NAN);
    let ratio = a / b;
    // Half the last displayed digit, and at least 0.0005 so 0.672 accepts 0.6718.
    let tol = (0.5 * 10f64.powi(-decimals(cell))).max(0.0005);
    if (shown - ratio).abs() > tol + 1e-12 {
        rep.bad(
            label,
            format!("{op} {num}/{den}: shows {cell}, raw {ratio:.4}"),
        );
    }
}

fn agg_mds(dir: &Path) -> impl Iterator<Item = PathBuf> {
    let mut paths = vec![dir.to_path_buf(), dir.join("raw")];
    let extra: Vec<PathBuf> = paths
        .drain(..)
        .flat_map(|d| {
            std::fs::read_dir(d)
                .into_iter()
                .flatten()
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
                .collect::<Vec<_>>()
        })
        .collect();
    extra.into_iter()
}

fn check_agg_dir(dir: &Path, rep: &mut Report) -> bool {
    let mut any = false;
    for path in agg_mds(dir) {
        if check_agg(&path, rep) {
            any = true;
        }
    }
    any
}

/// `| id | mode | runs | median | min | max | spread |` tables. The spread
/// column is (max / min − 1), the same quantity the argon2 raw tables publish.
fn check_agg(path: &Path, rep: &mut Report) -> bool {
    let text = super::read(path);
    let mut on = false;
    let mut n = 0;
    let label = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let time = Regex::new(r"^([\d.,]+) (ns|µs|us|ms|s)$").unwrap();
    for line in text.lines() {
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() >= 7
            && cells[0] == "id"
            && cells[1] == "mode"
            && cells[3] == "median"
            && cells[6] == "spread"
        {
            on = true;
            continue;
        }
        if !on || !line.starts_with('|') || cells.len() < 7 || cells[0].starts_with('-') {
            if !line.starts_with('|') {
                on = false;
            }
            continue;
        }
        let Some((med_ns, _)) = time_ns_tol(cells[3], &time) else {
            continue;
        };
        let Some((min_ns, min_half)) = time_ns_tol(cells[4], &time) else {
            continue;
        };
        let Some((max_ns, max_half)) = time_ns_tol(cells[5], &time) else {
            continue;
        };
        n += 1;
        if min_ns > med_ns + min_half + 1e-6 || med_ns > max_ns + max_half + 1e-6 {
            rep.bad(
                &label,
                format!("{} {}: median outside min/max", cells[0], cells[1]),
            );
        }
        // min and max are rounded, so (max/min − 1) has a range, not one value.
        let min_lo = (min_ns - min_half).max(0.0);
        let min_hi = min_ns + min_half;
        let max_lo = (max_ns - max_half).max(0.0);
        let max_hi = max_ns + max_half;
        let spread_lo = if max_lo <= min_hi || min_hi == 0.0 {
            0.0
        } else {
            (max_lo / min_hi - 1.0) * 100.0
        };
        let spread_hi = if min_lo == 0.0 {
            f64::INFINITY
        } else {
            (max_hi / min_lo - 1.0) * 100.0
        };
        let shown_text = cells[6].trim_end_matches('%');
        let shown: f64 = shown_text.parse().unwrap_or(f64::NAN);
        let shown_half = 0.5 * 10f64.powi(-decimals(shown_text));
        if shown + shown_half + 1e-9 < spread_lo || shown - shown_half - 1e-9 > spread_hi {
            rep.bad(
                &label,
                format!(
                    "{} {}: spread shows {}%, (max/min − 1) is {:.2}%–{:.2}%",
                    cells[0], cells[1], cells[6], spread_lo, spread_hi
                ),
            );
        }
    }
    if n > 0 {
        println!("{label}: {n} aggregated rows checked");
    }
    n > 0
}

fn time_ns_tol(cell: &str, re: &Regex) -> Option<(f64, f64)> {
    let m = re.captures(cell)?;
    let v: f64 = m[1].replace(',', "").parse().ok()?;
    let scale = unit_scale(&m[2])?;
    let half = 0.5 * scale * 10f64.powi(-decimals(&m[1]));
    Some((v * scale, half))
}

fn compare_mds(dir: &Path) -> impl Iterator<Item = PathBuf> {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            (name.starts_with("compare-") && name.ends_with(".md")).then(|| e.path())
        })
}

fn check_compares(dir: &Path, rep: &mut Report) -> bool {
    let mut any = false;
    for path in compare_mds(dir) {
        any = true;
        check_compare(dir, &path, rep);
    }
    any
}

fn check_compare(dir: &Path, path: &Path, rep: &mut Report) {
    let label = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let text = super::read(path);
    let head = Regex::new(
        r"Known-answer checks: (\S+) (\d+)/(\d+), (\S+) (\d+)/(\d+)\. Runs: (\d+) and (\d+)\. Rows: (\d+)",
    )
    .unwrap();
    let moved_re = Regex::new(r"(\d+) of (\d+) rows moved").unwrap();
    let mut section = String::new();
    let mut claims = 0;
    for line in text.lines() {
        if let Some(name) = line.strip_prefix("## ") {
            section = name.trim().to_string();
            continue;
        }
        let Some(m) = head.captures(line) else {
            continue;
        };
        claims += 1;
        let boards = [
            (
                m[1].to_string(),
                m[2].parse::<usize>().unwrap_or(0),
                m[3].parse::<usize>().unwrap_or(0),
                m[7].parse::<usize>().unwrap_or(0),
            ),
            (
                m[4].to_string(),
                m[5].parse::<usize>().unwrap_or(0),
                m[6].parse::<usize>().unwrap_or(0),
                m[8].parse::<usize>().unwrap_or(0),
            ),
        ];
        let rows_claimed: usize = m[9].parse().unwrap_or(0);
        let mut stats = vec![];
        for (name, pass, total, runs) in &boards {
            let bench = side_dir(dir, path, name, &section);
            let st = bench_stat(&bench);
            stats.push(st.clone());
            if st.runs != *runs {
                rep.bad(
                    &label,
                    format!(
                        "{section} {name}: {} run logs, compare says {runs}",
                        st.runs
                    ),
                );
            }
            if st.checks != *total || st.passed != *pass {
                rep.bad(
                    &label,
                    format!(
                        "{section} {name}: {}/{} known-answer results, compare says {pass}/{total}",
                        st.passed, st.checks
                    ),
                );
            }
            if st.ids != rows_claimed {
                rep.bad(
                    &label,
                    format!(
                        "{section} {name}: {} rows, compare says {rows_claimed}",
                        st.ids
                    ),
                );
            }
        }
        if let Some(rest) = text.split(line).nth(1) {
            let window: String = rest.lines().take(40).collect::<Vec<_>>().join("\n");
            if let Some(mm) = moved_re.captures(&window) {
                let shown: usize = mm[1].parse().unwrap_or(0);
                let of: usize = mm[2].parse().unwrap_or(0);
                if of != rows_claimed {
                    rep.bad(
                        &label,
                        format!("{section}: moved-row denominator {of}, rows {rows_claimed}"),
                    );
                }
                if stats.len() == 2 {
                    let got = moved_rows(&stats[0], &stats[1]);
                    if got != shown {
                        rep.bad(
                            &label,
                            format!("{section}: {got} rows moved, compare says {shown}"),
                        );
                    }
                }
            }
        }
    }
    if claims > 0 {
        println!("{label}: {claims} compare sections checked");
    }
}

#[derive(Clone)]
struct BenchStat {
    runs: usize,
    ids: usize,
    checks: usize,
    passed: usize,
    /// id -> (ns per run, cycles per run)
    rows: BTreeMap<String, (Vec<f64>, Vec<f64>)>,
}

fn bench_stat(bench: &Path) -> BenchStat {
    let mut st = BenchStat {
        runs: 0,
        ids: 0,
        checks: 0,
        passed: 0,
        rows: BTreeMap::new(),
    };
    let mut files: Vec<PathBuf> = jsonl_files(bench).collect();
    files.sort();
    st.runs = files.len();
    for f in files {
        let text = super::read(&f);
        for line in text.lines() {
            let line = line.trim();
            if !line.starts_with('{') {
                continue;
            }
            let Ok(r) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            match r.get("type").and_then(|v| v.as_str()) {
                Some("result") => {
                    let id = r["id"].as_str().unwrap_or("").to_string();
                    // `cargo xtask compare` leaves harness/* out of the row count.
                    if id.starts_with("harness/") {
                        continue;
                    }
                    let e = st.rows.entry(id).or_default();
                    e.0.push(super::fv(&r["ns"]));
                    e.1.push(super::fv(&r["cycles"]["median"]));
                }
                Some("check") => {
                    st.checks += 1;
                    let pass = match r.get("pass") {
                        Some(Value::Bool(b)) => *b,
                        Some(Value::Number(n)) => n.as_f64() != Some(0.0),
                        Some(Value::String(s)) => !s.is_empty(),
                        _ => false,
                    };
                    st.passed += usize::from(pass);
                }
                _ => {}
            }
        }
    }
    st.ids = st.rows.len();
    st
}

fn moved_rows(a: &BenchStat, b: &BenchStat) -> usize {
    let mut n = 0;
    for (id, (_, cyc_a)) in &a.rows {
        let Some((_, cyc_b)) = b.rows.get(id) else {
            continue;
        };
        if cyc_a.is_empty() || cyc_b.is_empty() {
            continue;
        }
        // Same rule as `cargo xtask compare`: cycle medians, threshold
        // max(1%, either side's cycle spread).
        let ratio = med(cyc_b) / med(cyc_a);
        let spread = spread_of(cyc_a).max(spread_of(cyc_b));
        if (ratio - 1.0).abs() > spread.max(0.01) + 1e-12 {
            n += 1;
        }
    }
    n
}

/// Where a compare side's `<bench>/run-*.jsonl` actually lives.
///
/// The files name the sides. The directories often use another name:
/// `published` and an ESP `pad0` are `results/raw/<bench>` (an nRF `published`
/// is `results/raw-nrf52840/<bench>`); `icache32` is `<dir>/<bench>/`;
/// `pad0-head` is `<dir>/pad0/<bench>/`; the side `pad0-control` in
/// `compare-pad0-control-mod16.md` is `<dir>/pad0-control-mod16/<bench>/`.
fn side_dir(compare_dir: &Path, compare_file: &Path, side: &str, section: &str) -> PathBuf {
    let file = compare_file
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    if file == "compare-pad0-control-mod16.md" && side == "pad0-control" {
        let alt = compare_dir.join("pad0-control-mod16").join(section);
        if jsonl_files(&alt).next().is_some() {
            return alt;
        }
    }
    let direct = compare_dir.join(side).join(section);
    if jsonl_files(&direct).next().is_some() {
        return direct;
    }
    if side == "icache32" {
        let here = compare_dir.join(section);
        if jsonl_files(&here).next().is_some() {
            return here;
        }
    }
    if side == "pad0-head" {
        let head = compare_dir.join("pad0").join(section);
        if jsonl_files(&head).next().is_some() {
            return head;
        }
    }
    if side == "published" || side == "pad0" {
        let published = published_raw(compare_dir).join(section);
        if jsonl_files(&published).next().is_some() {
            return published;
        }
    }
    direct
}

fn published_raw(dir: &Path) -> PathBuf {
    let nrf = dir
        .components()
        .any(|c| c.as_os_str().to_string_lossy().contains("nrf52840"));
    super::root().join(if nrf {
        "results/raw-nrf52840"
    } else {
        "results/raw"
    })
}

fn spread_of(xs: &[f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    let m = med(xs);
    if m == 0.0 {
        return 0.0;
    }
    let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (hi - lo) / m
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn scratch() -> PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!("rind-verify-dir-{n}"));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn host_tree_out_of_repo_matches() {
        let dir = scratch();
        std::fs::create_dir_all(dir.join("raw")).unwrap();
        let run = r#"{
 "schema": 1,
 "host": "tiny",
 "status": "ok",
 "results": [
  {"operation": "aes128-gcm/{lib}/32B", "library": "rind", "ns": 100.0},
  {"operation": "aes128-gcm/{lib}/32B", "library": "aws-lc-rs", "ns": 200.0}
 ]
}
"#;
        std::fs::write(dir.join("raw/run-1.json"), run).unwrap();
        std::fs::write(dir.join("tiny.json"), run).unwrap();
        std::fs::write(
            dir.join("RESULTS.md"),
            "\
| Operation | rind | Fastest other | rind / fastest | Spread (rind, other) |
|---|---|---|---|---|
| aes128-gcm/32B | 100 ns | aws-lc-rs, 200 ns | 0.50× | 0.0%, 0.0% |

rind is the fastest on 1 of 1 operations it shares with another library.

| Operation | rind | aws-lc-rs |
|---|---|---|
| aes128-gcm/32B | 100 ns | 200 ns |
",
        )
        .unwrap();
        std::fs::write(
            dir.join("RUNS.md"),
            "\
| Operation | Library | run-1 | Median | Spread |
|---|---|---|---|---|
| aes128-gcm/32B | rind | 100 ns | 100 ns | 0.0% |
| aes128-gcm/32B | aws-lc-rs | 200 ns | 200 ns | 0.0% |
",
        )
        .unwrap();
        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep), "{}", dir.display());
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn host_tree_flags_a_rewritten_median() {
        let dir = scratch();
        std::fs::create_dir_all(dir.join("raw")).unwrap();
        std::fs::write(
            dir.join("raw/run-1.json"),
            r#"{"schema":1,"host":"tiny","status":"ok","results":[{"operation":"aes128-gcm/{lib}/32B","library":"rind","ns":100.0},{"operation":"aes128-gcm/{lib}/32B","library":"aws-lc-rs","ns":200.0}]}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("tiny.json"),
            r#"{"schema":1,"host":"tiny","status":"ok","results":[{"operation":"aes128-gcm/{lib}/32B","library":"rind","ns":150.0},{"operation":"aes128-gcm/{lib}/32B","library":"aws-lc-rs","ns":200.0}]}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("RESULTS.md"),
            "rind is the fastest on 0 of 1 operations\n",
        )
        .unwrap();
        std::fs::write(dir.join("RUNS.md"), "").unwrap();
        let mut rep = Report::default();
        verify_dir(&dir, &mut rep);
        assert!(
            rep.problems.iter().any(|p| p.contains("tiny.json")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn sibling_raw_board_and_compare() {
        let root = scratch();
        let board = root.join("demo");
        let raw = root.join("raw-demo");
        write_bench(&raw.join("aes"), 240_000_000, 240.0, 1000.0);
        std::fs::create_dir_all(&board).unwrap();
        std::fs::write(
            board.join("RESULTS.md"),
            "\
| Operation | rind |
|---|---|
| aes128-gcm/…/32B | 1.00 µs |
",
        )
        .unwrap();
        let mut rep = Report::default();
        assert!(verify_dir(&board, &mut rep), "{:?}", rep.problems);
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));

        let a = root.join("board-1");
        let b = root.join("board-2");
        write_bench(&a.join("aes"), 240_000_000, 240.0, 1000.0);
        write_bench(&b.join("aes"), 240_000_000, 240.0, 1000.0);
        // One known-answer line each, so the compare claim is 1/1.
        append_check(&a.join("aes/run-1.jsonl"));
        append_check(&b.join("aes/run-1.jsonl"));
        std::fs::write(
            root.join("compare-boards.md"),
            "\
## aes

Known-answer checks: board-1 1/1, board-2 1/1. Runs: 1 and 1. Rows: 1 in both.

0 of 1 rows moved by more than max(1%, run spread).
",
        )
        .unwrap();
        let mut rep = Report::default();
        assert!(verify_dir(&root, &mut rep));
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        let _ = std::fs::remove_dir_all(&root);
    }

    fn write_bench(bench: &Path, hz: u64, cycles: f64, ns: f64) {
        std::fs::create_dir_all(bench).unwrap();
        std::fs::write(
            bench.join("run-1.jsonl"),
            format!(
                "{{\"type\":\"start\",\"cpu_hz\":{hz}}}\n{{\"type\":\"result\",\"id\":\"aes128-gcm/rind/32B\",\"ns\":{ns},\"cycles\":{{\"min\":{cycles},\"median\":{cycles},\"max\":{cycles},\"p5\":{cycles},\"p95\":{cycles}}}}}\n"
            ),
        )
        .unwrap();
    }

    fn append_check(path: &Path) {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().append(true).open(path).unwrap();
        writeln!(
            f,
            r#"{{"type":"check","check":"kat","library":"rind","pass":true}}"#
        )
        .unwrap();
    }

    #[test]
    fn ab_v1_cells_match_the_rounds() {
        let dir = scratch();
        let json = r#"{
 "schema": "rind-results/ab-v1",
 "results": [
  {"operation":"ed25519-sign/{lib}","library":"rind","mode":"wide","round":1,"ns":51500.0},
  {"operation":"ed25519-sign/{lib}","library":"rind","mode":"wide","round":2,"ns":51500.0},
  {"operation":"ed25519-sign/{lib}","library":"rind","mode":"narrow","round":1,"ns":34600.0},
  {"operation":"ed25519-sign/{lib}","library":"rind","mode":"narrow","round":2,"ns":34600.0},
  {"operation":"ed25519-sign/{lib}","library":"aws-lc-rs","mode":"wide","round":1,"ns":35400.0},
  {"operation":"ed25519-sign/{lib}","library":"aws-lc-rs","mode":"narrow","round":1,"ns":35400.0}
 ]
}
"#;
        std::fs::write(dir.join("ab.json"), json).unwrap();
        std::fs::write(
            dir.join("ab.md"),
            "\
| op | wide µs (n, spread) | narrow µs (n, spread) | narrow/wide | aws-lc-rs µs |
|---|---|---|---|---|
| ed25519-sign | 51.5 (2, 0.0%) | 34.6 (2, 0.0%) | 0.672 | 35.4 |
",
        )
        .unwrap();
        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep));
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn aggregated_table_spread_is_max_over_min() {
        let dir = scratch();
        std::fs::create_dir_all(dir.join("raw")).unwrap();
        std::fs::write(
            dir.join("raw/round.md"),
            "\
| id | mode | runs | median | min | max | spread |
|---|---|---|---|---|---|---|
| 16MiB-1/rind | neon | 5 | 4.367 ms | 4.283 ms | 4.433 ms | 3.5% |
",
        )
        .unwrap();
        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep));
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_parent_of_experiments_is_not_one_board() {
        let dir = scratch();
        std::fs::create_dir_all(dir.join("exp-a")).unwrap();
        std::fs::write(dir.join("exp-a/run-1.jsonl"), "{}\n").unwrap();
        std::fs::create_dir_all(dir.join("exp-b/raw/aes")).unwrap();
        std::fs::write(dir.join("exp-b/raw/aes/run-1.jsonl"), "{}\n").unwrap();
        std::fs::write(dir.join("exp-b/RESULTS.md"), "# one experiment\n").unwrap();
        std::fs::write(
            dir.join("RESULTS.md"),
            "\
| Operation | rind | fastest other Rust library | ratio | spread |
|---|---|---|---|---|
| aes128-gcm/…/{lib}/32B | 1 ns | missing: 2 ns | 0.50× | 0% |
",
        )
        .unwrap();
        assert!(board_raw(&dir).is_none());
        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep));
        assert!(
            rep.problems.iter().all(|p| !p.contains("fastest shown")),
            "parent report was checked against mixed runs: {}",
            rep.problems.join("\n")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn agg(spread_ok: bool) -> String {
        let max = if spread_ok { "10 ns" } else { "20 ns" };
        format!(
            "\
| id | mode | runs | median | min | max | spread |
|---|---|---|---|---|---|---|
| a | b | 1 | 10 ns | 10 ns | {max} | 0.00% |
"
        )
    }

    #[test]
    fn both_pins_use_the_current_set_and_list_the_previous_as_history() {
        let dir = scratch();
        let current = dir.join(COMPARABLE_PIN);
        let previous = dir.join(PREVIOUS_COMPARABLE_PIN);
        std::fs::create_dir_all(&current).unwrap();
        std::fs::create_dir_all(&previous).unwrap();
        std::fs::write(current.join("table.md"), agg(true)).unwrap();
        std::fs::write(previous.join("table.md"), agg(false)).unwrap();

        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep));
        assert!(
            rep.problems.is_empty(),
            "previous pin leaked into the check: {}",
            rep.problems.join("\n")
        );
        assert!(
            rep.notes.iter().any(|n| {
                n.contains(PREVIOUS_COMPARABLE_PIN)
                    && n.contains("previous comparable set")
                    && n.contains(COMPARABLE_PIN)
            }),
            "{:?}",
            rep.notes
        );

        let mut old = Report::default();
        assert!(verify_dir(&previous, &mut old));
        assert!(
            old.problems.iter().any(|p| p.contains("spread")),
            "the previous set is still checked on its own: {:?}",
            old.problems
        );

        let only = scratch();
        let lone = only.join(PREVIOUS_COMPARABLE_PIN);
        std::fs::create_dir_all(&lone).unwrap();
        std::fs::write(lone.join("table.md"), agg(false)).unwrap();
        let mut lone_rep = Report::default();
        assert!(verify_dir(&only, &mut lone_rep));
        assert!(
            lone_rep.problems.iter().any(|p| p.contains("spread")),
            "a host with only the previous pin is still checked: {:?}",
            lone_rep.problems
        );

        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_dir_all(&only);
    }

    #[test]
    fn published_compare_files_resolve_their_side_directories() {
        let root = super::super::root();
        let files = [
            "results/diag/esp32s3-icache32-80mhz-98568332/compare-p256.md",
            "results/diag/esp32s3-relink-fixed-98568332/compare-pad1039.md",
            "results/diag/esp32s3-relink-fixed-98568332/compare-pad0-control-mod16.md",
            "results/diag/nrf52840-relink-fixed-98568332/compare-published-pad0.md",
        ];
        for rel in files {
            let path = root.join(rel);
            let dir = path.parent().unwrap();
            let mut rep = Report::default();
            check_compare(dir, &path, &mut rep);
            assert!(
                rep.problems.is_empty(),
                "{rel}: {}",
                rep.problems.join("\n")
            );
        }
    }

    fn write_cycle_tree(dir: &Path, median: u64, runs: [u64; 5]) {
        std::fs::create_dir_all(dir.join("firmware/src")).unwrap();
        std::fs::write(
            dir.join("firmware/src/main.rs"),
            "// source, not a result tree\n",
        )
        .unwrap();
        let min = *runs.iter().min().unwrap();
        let max = *runs.iter().max().unwrap();
        let json = format!(
            "{{\n  \"schema\": 1,\n  \"host\": \"uno-r3\",\n  \"clock_hz\": 16000000,\n  \"ops\": {{\n    \"sha256/32B\": {{\n      \"n\": 5,\n      \"cycles_all\": [{0}, {1}, {2}, {3}, {4}],\n      \"cycles_median\": {5},\n      \"cycles_min\": {6},\n      \"cycles_max\": {7},\n      \"us_median\": {8}\n    }}\n  }}\n}}\n",
            runs[0],
            runs[1],
            runs[2],
            runs[3],
            runs[4],
            median,
            min,
            max,
            median / 16
        );
        std::fs::write(dir.join("medians.json"), json).unwrap();
        std::fs::write(
            dir.join("RUNS.md"),
            format!(
                "| operation | run 1 | run 2 | run 3 | run 4 | run 5 | median | spread |\n|---|---:|---:|---:|---:|---:|---:|---:|\n| `sha256/32B` | {0} | {1} | {2} | {3} | {4} | {5} | 0.00% |\n",
                runs[0], runs[1], runs[2], runs[3], runs[4], median
            ),
        )
        .unwrap();
        std::fs::write(
            dir.join("RESULTS.md"),
            format!(
                "| Operation | Uno R3 cycles (n=5) | range | Uno R3 µs |\n|---|---:|---:|---:|\n| sha256/32B | {median} | {min}–{max} | {us} |\n",
                median = median,
                min = min,
                max = max,
                us = median / 16
            ),
        )
        .unwrap();
    }

    #[test]
    fn cycle_median_tree_skips_firmware_and_checks_tables() {
        let dir = scratch();
        write_cycle_tree(&dir, 61048, [61048, 61048, 61048, 61048, 61048]);
        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep), "{}", dir.display());
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cycle_median_tree_flags_a_rewritten_median() {
        let dir = scratch();
        write_cycle_tree(&dir, 61048, [61048, 61048, 61048, 61048, 61048]);
        std::fs::write(
            dir.join("RESULTS.md"),
            "| Operation | Uno R3 cycles (n=5) | range | Uno R3 µs |\n|---|---:|---:|---:|\n| sha256/32B | 99999 | 61048–61048 | 3815 |\n",
        )
        .unwrap();
        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep), "{}", dir.display());
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("RESULTS.md") && p.contains("99999")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn published_uno_r3_cycle_medians_match() {
        let dir = super::super::root().join("results/uno-r3");
        let mut rep = Report::default();
        assert!(verify_dir(&dir, &mut rep), "{}", rep.problems.join("\n"));
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
    }
}
