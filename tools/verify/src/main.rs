// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Independent re-check of the published numbers. Shares no code with
//! xtask: it rereads each run's raw JSONL, takes its own medians, and
//! compares them with what RESULTS.md, demo.tsv, the chart SVGs and msi-001's
//! summary.md show. It flags any cell, ratio, fastest library, unit or
//! cycles/time mismatch, and prints the cycle-counter cross-checks.
//!
//! Usage: cargo verify [--board esp32s3|nrf52840|efr32mg24|msi-001 ...]
//! Exit status 1 if anything mismatched.
//!
//! `cargo verify --dir <tree>` checks that directory instead of the published
//! paths: a host tree (`raw/run-*.json` plus its median JSON and RESULTS.md),
//! a board tree (jsonl under the directory, `raw/`, or a sibling `raw-<name>/`),
//! an `msi-001`-style `summary.md`, a `rind-results/ab-v1` directory, or a
//! parent of those. With no arguments the published paths are checked, as before.

use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod tree;

const NOT_RIVALS: &[&str] = &["rind", "esp32s3-hw"];

/// rind's admin/bench-report LIBRARIES: benchmark-ID library -> report library.
const LIBRARIES: &[(&str, &str)] = &[
    ("rind", "rind"),
    ("rind-dit-held", "rind-dit-held"),
    ("aws-lc-rs", "aws-lc-rs"),
    ("ring", "ring"),
    ("rustcrypto", "rustcrypto"),
    ("p256-rustcrypto", "rustcrypto"),
    ("p384-rustcrypto", "rustcrypto"),
    ("dalek", "rustcrypto"),
    ("graviola", "graviola"),
    ("libcrux", "libcrux"),
    ("libcrux-ml-kem", "libcrux"),
    ("openssl", "openssl"),
    ("esp32s3-hw", "esp32s3-hw"),
];

struct BoardFiles {
    raw: &'static str,
    md: &'static str,
    svg: &'static str,
    tsv: Option<&'static str>,
    hz: f64,
}

fn board_files(b: &str) -> Option<BoardFiles> {
    match b {
        "esp32s3" => Some(BoardFiles {
            raw: "results/raw",
            md: "results/RESULTS.md",
            svg: "media/charts/esp32s3-rind-vs-fastest.svg",
            tsv: Some("results/demo.tsv"),
            hz: 240_000_000.0,
        }),
        "nrf52840" => Some(BoardFiles {
            raw: "results/raw-nrf52840",
            md: "results/nrf52840/RESULTS.md",
            svg: "media/charts/nrf52840-rind-vs-fastest.svg",
            tsv: None,
            hz: 64_000_000.0,
        }),
        "efr32mg24" => Some(BoardFiles {
            raw: "results/raw-efr32mg24",
            md: "results/efr32mg24/RESULTS.md",
            svg: "media/charts/efr32mg24-rind-vs-fastest.svg",
            tsv: None,
            hz: 78_000_000.0,
        }),
        _ => None,
    }
}

/// Linux hosts published under results/<host>/ by `cargo xtask host-report`.
const HOSTS: &[&str] = &["jetson-orin-nano"];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// An insertion-ordered map, as Python's dict.
#[derive(Clone, Debug)]
struct Ordered<V>(Vec<(String, V)>);

impl<V> Default for Ordered<V> {
    fn default() -> Self {
        Ordered(vec![])
    }
}

impl<V> Ordered<V> {
    fn get(&self, k: &str) -> Option<&V> {
        self.0.iter().find(|(x, _)| x == k).map(|(_, v)| v)
    }
    fn entry(&mut self, k: &str, new: impl FnOnce() -> V) -> &mut V {
        let i = match self.0.iter().position(|(x, _)| x == k) {
            Some(i) => i,
            None => {
                self.0.push((k.to_string(), new()));
                self.0.len() - 1
            }
        };
        &mut self.0[i].1
    }
    fn insert(&mut self, k: &str, v: V) {
        match self.0.iter_mut().find(|(x, _)| x == k) {
            Some(e) => e.1 = v,
            None => self.0.push((k.to_string(), v)),
        }
    }
    fn contains(&self, k: &str) -> bool {
        self.get(k).is_some()
    }
    fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.iter().map(|(k, _)| k)
    }
}

#[derive(Clone)]
struct Agg {
    ns: f64,
    cycles: f64,
    runs: usize,
    spread: f64,
    bytes: Option<f64>,
}

#[derive(Default)]
struct Report {
    problems: Vec<String>,
    notes: Vec<String>,
}

impl Report {
    fn bad(&mut self, board: &str, what: String) {
        self.problems.push(format!("{board}: {what}"));
    }
    /// A measurement to look at, not a reporting error.
    fn note(&mut self, board: &str, what: String) {
        self.notes.push(format!("{board}: {what}"));
    }
}

fn split_id(i: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = i.split('/').collect();
    for (j, p) in parts.iter().enumerate() {
        if let Some((_, lib)) = LIBRARIES.iter().find(|(row, _)| row == p) {
            let mut op: Vec<&str> = parts[..j].to_vec();
            op.push("{lib}");
            op.extend(&parts[j + 1..]);
            return Some((op.join("/"), lib.to_string()));
        }
    }
    None
}

fn label(op: &str) -> String {
    op.replace("/{lib}", "").replace("{lib}/", "")
}

fn med(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn fv(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap_or(f64::NAN),
        Value::Bool(b) => f64::from(u8::from(*b)),
        _ => f64::NAN,
    }
}

fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64() != Some(0.0),
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
    }
}

/// Files matching `<base>/*/<prefix>*<suffix>`, sorted as strings.
fn glob_at(base: &Path, prefix: &str, suffix: &str) -> Vec<PathBuf> {
    let mut out = vec![];
    for d in std::fs::read_dir(base).into_iter().flatten().flatten() {
        if d.file_name().to_string_lossy().starts_with('.') || !d.path().is_dir() {
            continue;
        }
        for f in std::fs::read_dir(d.path()).into_iter().flatten().flatten() {
            let n = f.file_name().to_string_lossy().into_owned();
            if n.starts_with(prefix) && n.ends_with(suffix) && !n.starts_with('.') {
                out.push(f.path());
            }
        }
    }
    out.sort_by_key(|p| p.display().to_string());
    out
}

fn read(path: &Path) -> String {
    String::from_utf8_lossy(
        &std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())),
    )
    .into_owned()
}

struct Raw {
    per: Ordered<Vec<Value>>,
    starts: Vec<Value>,
    checks: Vec<((String, String), Vec<bool>)>,
    clocks: Vec<Value>,
}

fn load_raw_at(base: &Path) -> Raw {
    let mut raw = Raw {
        per: Ordered::default(),
        starts: vec![],
        checks: vec![],
        clocks: vec![],
    };
    for f in glob_at(base, "run-", ".jsonl") {
        for line in read(&f).lines() {
            let line = line.trim();
            if !line.starts_with('{') {
                continue;
            }
            let r: Value =
                serde_json::from_str(line).unwrap_or_else(|e| panic!("{}: {e}", f.display()));
            match r.get("type").and_then(|v| v.as_str()) {
                Some("result") => raw
                    .per
                    .entry(r["id"].as_str().unwrap_or_default(), Vec::new)
                    .push(r),
                Some("start") => raw.starts.push(r),
                Some("clock") => raw.clocks.push(r),
                Some("check") => {
                    let key = (s(&r["check"]), s(&r["library"]));
                    let pass = truthy(r.get("pass"));
                    match raw.checks.iter_mut().find(|(k, _)| *k == key) {
                        Some((_, v)) => v.push(pass),
                        None => raw.checks.push((key, vec![pass])),
                    }
                }
                _ => {}
            }
        }
    }
    raw
}

fn s(v: &Value) -> String {
    v.as_str().map_or_else(|| v.to_string(), String::from)
}

/// The SPI flash clock each ESP32-S3 boot reported, MHz -> logs: one set of
/// runs must not mix them, since every row that misses the instruction cache
/// scales with it.
fn flash_clocks_at(base: &Path) -> BTreeMap<i64, Vec<PathBuf>> {
    let re = Regex::new(r"Boot SPI Speed\s*:\s*(\d+)MHz").unwrap();
    let mut out: BTreeMap<i64, Vec<PathBuf>> = BTreeMap::new();
    for f in glob_at(base, "run-", ".log") {
        if let Some(c) = re.captures(&read(&f)) {
            out.entry(c[1].parse().unwrap()).or_default().push(f);
        }
    }
    out
}

/// A shown number and half its last digit's unit.
fn num_with_precision(s: &str) -> Option<(f64, f64)> {
    let s = s.replace(',', "");
    let v: f64 = s.parse().ok()?;
    let dec = s.split_once('.').map_or(0, |(_, d)| d.len());
    Some((v, 0.5 * 10f64.powi(-(dec as i32))))
}

struct Cell {
    ns: f64,
    ns_tol: f64,
    kind: Option<String>,
    value: f64,
    tol: f64,
}

/// '88.65 µs (664.9 c/B)' -> time, its tolerance, and the cycles figure.
fn parse_cell(c: &str, re: &Regex) -> Option<Cell> {
    let m = re.captures(c.trim())?;
    let (t, tt) = num_with_precision(&m[1])?;
    let u = match &m[2] {
        "ns" => 1.0,
        "µs" | "us" => 1e3,
        "ms" => 1e6,
        _ => 1e9,
    };
    let mut out = Cell {
        ns: t * u,
        ns_tol: tt * u,
        kind: None,
        value: 0.0,
        tol: 0.0,
    };
    if let Some(g3) = m.get(3) {
        let (mut v, mut vt) = num_with_precision(g3.as_str())?;
        let mut k = m[4].to_string();
        if k == "kc" {
            (v, vt, k) = (v * 1e3, vt * 1e3, "c".into());
        }
        (out.kind, out.value, out.tol) = (Some(k), v, vt);
    }
    Some(out)
}

fn close(shown: f64, tol: f64, mine: f64) -> bool {
    (shown - mine).abs() <= tol + 1e-4 * mine.abs()
}

/// The fastest rival in insertion order (the first of equals), as Python's `min(d, key=d.get)`.
fn fastest(t: &Ordered<Agg>, keep: impl Fn(&str) -> bool) -> Option<(&str, f64)> {
    let mut best: Option<(&str, f64)> = None;
    for (l, a) in &t.0 {
        if keep(l) && best.is_none_or(|(_, b)| a.ns < b) {
            best = Some((l, a.ns));
        }
    }
    best
}

fn is_rival(l: &str) -> bool {
    !NOT_RIVALS.contains(&l) && !l.starts_with("rind-")
}

fn py_list<T: std::fmt::Display>(xs: &[T]) -> String {
    format!(
        "[{}]",
        xs.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn check_board(board: &str, rep: &mut Report) {
    let bf = board_files(board).expect("known board");
    let raw_path = root().join(bf.raw);
    let md = root().join(bf.md);
    let svg = root().join(bf.svg);
    let tsv = bf.tsv.map(|p| root().join(p));
    check_board_at(
        board,
        &raw_path,
        Some(&md),
        Some(&svg),
        tsv.as_deref(),
        Some(bf.hz),
        rep,
    );
}

/// Re-check one board tree. `hz` is the counter rate a published board must
/// report; `None` (used by `--dir`) takes the rate from start records that agree.
fn check_board_at(
    board: &str,
    raw_path: &Path,
    md: Option<&Path>,
    svg: Option<&Path>,
    tsv: Option<&Path>,
    hz: Option<f64>,
    rep: &mut Report,
) {
    let raw = load_raw_at(raw_path);
    let reported_hz: Vec<i64> = {
        let mut v: Vec<i64> = raw
            .starts
            .iter()
            .filter_map(|s| {
                s.get("cpu_hz").and_then(|c| {
                    c.as_i64()
                        .or_else(|| c.as_u64().map(|u| u as i64))
                        .or_else(|| c.as_f64().map(|f| f as i64))
                })
            })
            .collect();
        v.sort();
        v.dedup();
        v
    };
    let hz_expect = if let Some(h) = hz {
        h
    } else if reported_hz.len() == 1 {
        reported_hz[0] as f64
    } else if reported_hz.is_empty() {
        rep.bad(board, "start records have no cpu_hz".into());
        0.0
    } else {
        rep.bad(
            board,
            format!(
                "cpu_hz in start records differs: {}",
                reported_hz
                    .iter()
                    .map(|h| h.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
        0.0
    };
    let fc = flash_clocks_at(raw_path);
    if fc.len() > 1 {
        let mix: Vec<String> = fc
            .iter()
            .map(|(k, v)| format!("{k} MHz x{}", v.len()))
            .collect();
        rep.bad(board, format!("runs mix flash clocks: {}", mix.join(", ")));
    } else if let Some((mhz, logs)) = fc.iter().next() {
        println!(
            "{board}: flash clock {mhz} MHz in all {} run logs",
            logs.len()
        );
    }
    for c in &raw.clocks {
        if hz_expect <= 0.0 {
            break;
        }
        let rate = fv(&c["cycles"]) * 1e9 / fv(&c["wall_ns"]);
        if (rate / hz_expect - 1.0).abs() > 0.001 {
            rep.bad(
                board,
                format!(
                    "cycle counter ran at {:.3} MHz against the wall clock",
                    rate / 1e6
                ),
            );
        }
    }
    if !raw.clocks.is_empty() {
        let first: Vec<String> = raw
            .clocks
            .iter()
            .take(3)
            .map(|c| format!("{:.4}", fv(&c["cycles"]) * 1e3 / fv(&c["wall_ns"])))
            .collect();
        println!(
            "{board}: {} boot clock checks, {} MHz ...",
            raw.clocks.len(),
            first.join(", ")
        );
    }
    let mut hz_seen: Vec<String> = raw
        .starts
        .iter()
        .map(|s| s.get("cpu_hz").map_or("None".into(), |v| v.to_string()))
        .collect();
    hz_seen.sort();
    hz_seen.dedup();
    if hz.is_some() && hz_seen != [format!("{}", hz_expect as i64)] {
        rep.bad(
            board,
            format!(
                "cpu_hz in start records {{{}}}, expected {}",
                hz_seen.join(", "),
                hz_expect as i64
            ),
        );
    }
    let mut agg: Ordered<Agg> = Ordered::default();
    let mut worst: (f64, Option<String>) = (0.0, None);
    for (i, recs) in &raw.per.0 {
        let ns: Vec<f64> = recs.iter().map(|r| fv(&r["ns"])).collect();
        let cyc: Vec<f64> = recs.iter().map(|r| fv(&r["cycles"]["median"])).collect();
        // cycles vs wall time, per run
        for r in recs {
            if hz_expect > 0.0 {
                let want = fv(&r["cycles"]["median"]) * 1e9 / hz_expect;
                let d = (fv(&r["ns"]) - want).abs() / want;
                if d > worst.0 {
                    worst = (d, Some(i.clone()));
                }
                if d > 0.01 {
                    rep.bad(
                        board,
                        format!(
                            "{i}: ns {:.1} vs cycles/{:.0}MHz {want:.1} ({:.2}%)",
                            fv(&r["ns"]),
                            hz_expect / 1e6,
                            100.0 * d
                        ),
                    );
                }
                let bytes = r.get("throughput").and_then(|t| t.get("Bytes"));
                if let Some(b) = bytes.filter(|_| truthy(r.get("cycles_per_byte"))) {
                    let cpb = fv(&r["cycles"]["median"]) / fv(b);
                    if (cpb - fv(&r["cycles_per_byte"])).abs() > 0.01 * cpb {
                        rep.bad(
                            board,
                            format!("{i}: cycles_per_byte {} vs {cpb:.3}", r["cycles_per_byte"]),
                        );
                    }
                }
                if truthy(r.get("wall_ns"))
                    && fv(&r["sampled_cycles"]) * 1e9 / hz_expect > 1.001 * fv(&r["wall_ns"])
                {
                    rep.bad(
                        board,
                        format!("{i}: counted more cycles than the wall clock allows"),
                    );
                }
            }
            let c = &r["cycles"];
            let (mn, md, mx, p5, p95) = (
                fv(&c["min"]),
                fv(&c["median"]),
                fv(&c["max"]),
                fv(&c["p5"]),
                fv(&c["p95"]),
            );
            if !(mn <= md && md <= mx && p5 <= md && md <= p95) {
                rep.bad(board, format!("{i}: cycle percentiles out of order {c}"));
            }
        }
        let bytes = recs[0]
            .get("throughput")
            .and_then(|t| t.get("Bytes"))
            .map(fv);
        let (mx, mn) = (
            ns.iter().copied().fold(f64::MIN, f64::max),
            ns.iter().copied().fold(f64::MAX, f64::min),
        );
        agg.insert(
            i,
            Agg {
                ns: med(&ns),
                cycles: med(&cyc),
                runs: recs.len(),
                spread: 100.0 * (mx - mn) / med(&ns),
                bytes,
            },
        );
    }
    let mut table: Ordered<Ordered<Agg>> = Ordered::default();
    for (i, a) in &agg.0 {
        if let Some((op, lib)) = split_id(i) {
            table.entry(&op, Ordered::default).insert(&lib, a.clone());
        }
    }
    let mut runs: Vec<usize> = agg.0.iter().map(|(_, a)| a.runs).collect();
    runs.sort();
    runs.dedup();
    println!(
        "{board}: {} ids, runs per id {}, worst ns-vs-cycles {:.4}% ({})",
        agg.0.len(),
        py_list(&runs),
        100.0 * worst.0,
        worst.1.as_deref().unwrap_or("None")
    );
    let failed: Vec<String> = raw
        .checks
        .iter()
        .filter(|(_, v)| !v.iter().all(|x| *x))
        .map(|((c, l), _)| format!("('{c}', '{l}')"))
        .collect();
    let results: usize = raw.checks.iter().map(|(_, v)| v.len()).sum();
    println!(
        "{board}: {} known-answer checks, {results} results, failing: {}",
        raw.checks.len(),
        if failed.is_empty() {
            "none".to_string()
        } else {
            py_list(&failed)
        }
    );
    if !failed.is_empty() {
        rep.bad(board, format!("failing checks {}", py_list(&failed)));
    }

    // time must grow with input size, or the work may have been optimised out
    let shape_re = Regex::new(r"[\d]+[KM]?B$").unwrap();
    let mut sized: Ordered<Vec<(f64, f64)>> = Ordered::default();
    let mut groups: Ordered<Vec<(f64, f64)>> = Ordered::default();
    for (i, a) in &agg.0 {
        let Some(b) = a.bytes.filter(|b| *b != 0.0) else {
            continue;
        };
        let parts: Vec<&str> = i.split('/').collect();
        let head = parts[..parts.len() - 1].join("/");
        // "quic-" sizes are another message shape
        let shape = shape_re.replace(parts[parts.len() - 1], "");
        upsert(sized.entry(&format!("{head}/{shape}"), Vec::new), b, a.ns);
        upsert(groups.entry(&head, Vec::new), b, a.cycles / b);
    }
    for (g, d) in &sized.0 {
        let mut d = d.clone();
        d.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        for w in d.windows(2) {
            let ((s0, t0), (s1, t1)) = (w[0], w[1]);
            if t1 < 0.98 * t0 {
                rep.note(
                    board,
                    format!(
                        "{g}: {} B takes {:.1}% less time than {} B",
                        s1,
                        100.0 * (1.0 - t1 / t0),
                        s0
                    ),
                );
            }
        }
    }
    // cycles/byte across sizes: a larger input should not cost more per byte
    let mut gs: Vec<&(String, Vec<(f64, f64)>)> = groups.0.iter().collect();
    gs.sort_by(|a, b| a.0.cmp(&b.0));
    for (g, d) in gs {
        let mut d = d.clone();
        d.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        for w in d.windows(2) {
            let ((s0, c0), (s1, c1)) = (w[0], w[1]);
            if c1 > 1.05 * c0 && s1 >= 1024.0 {
                println!(
                    "  note {board}: {g} costs more per byte at {s1} B ({c1:.1} c/B) than at {s0} B ({c0:.1})"
                );
            }
        }
    }

    if let Some(path) = md {
        check_markdown(board, path, &table, rep);
    } else {
        println!("{board}: no RESULTS.md; raw rows checked, tables skipped");
    }
    if let Some(path) = svg {
        check_chart(board, path, &table, rep);
    }
    if let Some(path) = tsv {
        check_tsv(board, path, &table, rep);
    }
}

/// Inserts (size, value), replacing an equal size as a dict would.
fn upsert(d: &mut Vec<(f64, f64)>, k: f64, v: f64) {
    match d.iter_mut().find(|(x, _)| *x == k) {
        Some(e) => e.1 = v,
        None => d.push((k, v)),
    }
}

fn check_markdown(board: &str, md: &Path, table: &Ordered<Ordered<Agg>>, rep: &mut Report) {
    let cell_re = Regex::new(r"^([\d.,]+) (ns|µs|us|ms|s)(?: \(([\d.,]+) (c/B|c|kc)\))?").unwrap();
    let text = read(md);
    let mut header: Option<Vec<String>> = None;
    let (mut cells, mut rows) = (0, 0);
    let empty = Ordered::default();
    for line in text.lines() {
        if line.starts_with("| Operation |") {
            header = Some(
                line.trim_matches('|')
                    .split('|')
                    .map(|h| h.trim().to_string())
                    .collect(),
            );
            continue;
        }
        if !line.starts_with("| ") || header.is_none() || line.starts_with("|---") {
            if !line.starts_with('|') {
                header = None;
            }
            continue;
        }
        let hdr = header.as_ref().unwrap();
        let f: Vec<String> = line
            .trim_matches('|')
            .split('|')
            .map(|x| x.trim().to_string())
            .collect();
        let op = f[0].replace('…', "{lib}");
        if !op.contains("{lib}") {
            continue;
        }
        rows += 1;
        let t = table.get(&op).unwrap_or(&empty);
        let pairs: Vec<(String, String)> =
            if hdr.len() >= 3 && hdr[1] == "rind" && hdr[2] == "fastest other Rust library" {
                if f.len() < 5 {
                    rep.bad(board, format!("{op}: short fastest-other row"));
                    continue;
                }
                let Some((fl, fl_ns)) = fastest(t, is_rival) else {
                    rep.bad(
                        board,
                        format!("{op}: RESULTS names another library, raw has no rival"),
                    );
                    continue;
                };
                let Some(rind) = t.get("rind").cloned() else {
                    rep.bad(
                        board,
                        format!("{op}: RESULTS names rind, raw has no rind row"),
                    );
                    continue;
                };
                let shown_lib = f[2].split(':').next().unwrap_or("");
                if shown_lib != fl {
                    rep.bad(
                        board,
                        format!("{op}: fastest shown {shown_lib}, raw says {fl}"),
                    );
                }
                let ratio = rind.ns / fl_ns;
                let (r, rt) =
                    num_with_precision(f[3].trim_end_matches('×')).unwrap_or((f64::NAN, 0.0));
                if !close(r, rt, ratio) {
                    rep.bad(board, format!("{op}: ratio shown {}, raw {ratio:.4}", f[3]));
                }
                let (sp, spt) =
                    num_with_precision(f[4].trim_end_matches('%')).unwrap_or((f64::NAN, 0.0));
                if !close(sp, spt, rind.spread) {
                    rep.bad(
                        board,
                        format!("{op}: spread shown {}, raw {:.3}", f[4], rind.spread),
                    );
                }
                let theirs = f[2].split_once(": ").map_or("", |(_, b)| b).to_string();
                vec![("rind".into(), f[1].clone()), (fl.to_string(), theirs)]
            } else {
                hdr[1..]
                    .iter()
                    .cloned()
                    .zip(f[1..].iter().cloned())
                    .collect()
            };
        for (lib, c) in pairs {
            let i = op.replace("{lib}", &lib);
            if c == "-" {
                if t.contains(&lib) {
                    rep.bad(board, format!("{i}: shown '-' but raw has it"));
                }
                continue;
            }
            let Some(p) = parse_cell(&c, &cell_re) else {
                rep.bad(board, format!("{i}: unparsed cell '{c}'"));
                continue;
            };
            let Some(a) = t.get(&lib) else {
                rep.bad(board, format!("{i}: shown {c} but not in raw"));
                continue;
            };
            cells += 1;
            if !close(p.ns, p.ns_tol, a.ns) {
                rep.bad(
                    board,
                    format!("{i}: time shown {c}, raw median {:.1} ns", a.ns),
                );
            }
            let bytes = a.bytes.unwrap_or(f64::NAN);
            match p.kind.as_deref() {
                Some("c/B") if !close(p.value, p.tol, a.cycles / bytes) => rep.bad(
                    board,
                    format!("{i}: c/B shown {}, raw {:.2}", p.value, a.cycles / bytes),
                ),
                Some("c") if !close(p.value, p.tol, a.cycles) => {
                    rep.bad(board, format!("{i}: cycles shown {c}, raw {:.0}", a.cycles))
                }
                _ => {}
            }
        }
    }
    println!("{board}: RESULTS.md {rows} rows, {cells} cells checked");
}

fn check_chart(board: &str, svg_path: &Path, table: &Ordered<Ordered<Agg>>, rep: &mut Report) {
    // chart: op label then "ratio lib"
    let mut by_label: Ordered<&Ordered<Agg>> = Ordered::default();
    for (op, t) in &table.0 {
        by_label.insert(&label(op), t);
    }
    let svg = read(svg_path);
    let tag = Regex::new(r"<[^>]*>").unwrap();
    let lab: Vec<String> = Regex::new(r"<text[^>]*>[^<]*</text>")
        .unwrap()
        .find_iter(&svg)
        .map(|m| tag.replace_all(m.as_str(), "").into_owned())
        .collect();
    let bar = Regex::new(r"^([\d.]+)x (\S+)$").unwrap();
    let mut n = 0;
    for w in lab.windows(2) {
        let (a, b) = (&w[0], &w[1]);
        let Some(m) = bar.captures(b) else { continue };
        if !a.contains('/') && !by_label.contains(a) {
            continue;
        }
        let Some(t) = by_label.get(a) else {
            rep.bad(board, format!("chart label {a} not found in raw"));
            continue;
        };
        let Some((fl, fl_ns)) = fastest(t, is_rival) else {
            rep.bad(
                board,
                format!("chart {a}: no other Rust library in the raw runs"),
            );
            continue;
        };
        let Some(rind) = t.get("rind") else {
            rep.bad(board, format!("chart {a}: no rind row in the raw runs"));
            continue;
        };
        let r = rind.ns / fl_ns;
        n += 1;
        let shown: f64 = m[1].parse().unwrap_or(f64::NAN);
        if &m[2] != fl || (shown - r).abs() > 0.005 + 1e-9 {
            rep.bad(board, format!("chart {a}: shows {b}, raw {r:.3}x {fl}"));
        }
    }
    println!("{board}: chart {n} bars checked");
}

fn check_tsv(board: &str, tsv: &Path, table: &Ordered<Ordered<Agg>>, rep: &mut Report) {
    // Pages are runs of adjacent lines with one heading, as the firmware
    // reads them; each must equal exactly one report operation's row.
    let mut pages: Vec<(String, Ordered<f64>)> = vec![];
    for line in read(tsv).split_inclusive('\n') {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.trim_end_matches('\n').split('\t').collect();
        let [h, lib, ns] = f[..] else {
            panic!("{}: bad line {line:?}", tsv.display())
        };
        if pages.last().is_none_or(|p| p.0 != h) {
            pages.push((h.to_string(), Ordered::default()));
        }
        let page = pages.last_mut().unwrap();
        if page.1.contains(lib) {
            rep.bad(
                board,
                format!("demo.tsv page '{h}' has {lib} twice (merged pages?)"),
            );
        }
        page.1.insert(lib, ns.parse().unwrap_or(f64::NAN));
    }
    let heads: Vec<&String> = pages.iter().map(|(h, _)| h).collect();
    let mut dups: Vec<&String> = heads
        .iter()
        .filter(|h| heads.iter().filter(|x| x == h).count() > 1)
        .copied()
        .collect();
    dups.sort();
    dups.dedup();
    for h in dups {
        rep.bad(
            board,
            format!(
                "demo.tsv heading '{h}' on {} pages",
                heads.iter().filter(|x| **x == h).count()
            ),
        );
    }
    let mut n = 0;
    let total = pages.len();
    for (k, (h, libs)) in pages.iter().enumerate() {
        let counter = format!("{}/{total}", k + 1);
        if 2 + 5 * h.chars().count() >= 126 - 5 * counter.len() - 2 {
            rep.bad(
                board,
                format!("demo.tsv heading '{h}' overlaps its page counter"),
            );
        }
        let matches: Vec<&String> = table
            .0
            .iter()
            .filter(|(_, t)| {
                let mut a: Vec<&String> = t.keys().collect();
                let mut b: Vec<&String> = libs.keys().collect();
                a.sort();
                b.sort();
                a == b
                    && libs
                        .0
                        .iter()
                        .all(|(l, v)| (t.get(l).unwrap().ns - v).abs() <= 1e-3 * v)
            })
            .map(|(op, _)| op)
            .collect();
        if matches.len() != 1 {
            let m = if matches.is_empty() {
                "no report row".to_string()
            } else {
                py_list(&matches.iter().map(|m| format!("'{m}'")).collect::<Vec<_>>())
            };
            rep.bad(board, format!("demo.tsv page '{h}': matches {m}"));
        } else {
            n += libs.0.len();
        }
    }
    let (mut ratios, mut wins) = (vec![], 0);
    for (_, libs) in &pages {
        let riv: Vec<f64> = libs
            .0
            .iter()
            .filter(|(l, _)| !NOT_RIVALS.contains(&l.as_str()))
            .map(|(_, v)| *v)
            .collect();
        if let (Some(rind), false) = (libs.get("rind"), riv.is_empty()) {
            let best = riv.iter().copied().fold(f64::INFINITY, f64::min);
            ratios.push(rind / best);
            wins += usize::from(*rind <= best);
        }
    }
    let g = (ratios.iter().map(|r: &f64| r.ln()).sum::<f64>() / ratios.len() as f64).exp();
    println!(
        "{board}: demo.tsv {n} bars on {total} pages checked; OLED summary should read {wins}/{}, geomean {g:.3}",
        ratios.len()
    );
}

fn check_msi(rep: &mut Report) {
    check_summary(
        "msi-001",
        &root().join("results/host/msi-001.json"),
        &root().join("results/host/summary.md"),
        rep,
    );
}

/// One schema-1 host JSON against rind's `summary` table (`summary.md`).
fn check_summary(name: &str, json_path: &Path, summary: &Path, rep: &mut Report) {
    let d: Value = serde_json::from_str(&read(json_path))
        .unwrap_or_else(|e| panic!("{}: {e}", json_path.display()));
    let mut by: Ordered<Ordered<f64>> = Ordered::default();
    for r in d["results"].as_array().into_iter().flatten() {
        by.entry(&s(&r["operation"]), Ordered::default)
            .insert(&s(&r["library"]), fv(&r["ns"]));
    }
    let row = Regex::new(r"^\| (\S+) \| ([\d.,]+ \S+), ([\d.]+)× (\S+) \|").unwrap();
    let cell_re = Regex::new(r"^([\d.,]+) (ns|µs|us|ms|s)(?: \(([\d.,]+) (c/B|c|kc)\))?").unwrap();
    let mut n = 0;
    for line in read(summary).lines() {
        let Some(m) = row.captures(line) else {
            continue;
        };
        let shown_op = &m[1];
        let ops: Vec<&String> = by.keys().filter(|o| label(o) == shown_op).collect();
        if ops.len() != 1 {
            let l = py_list(&ops.iter().map(|o| format!("'{o}'")).collect::<Vec<_>>());
            rep.bad(name, format!("summary row {shown_op}: matched {l}"));
            continue;
        }
        let t = by.get(ops[0]).unwrap();
        let mut best: Option<(&str, f64)> = None;
        for (l, v) in &t.0 {
            if l != "rind" && !l.starts_with("rind") && best.is_none_or(|(_, b)| *v < b) {
                best = Some((l, *v));
            }
        }
        let Some((fl, fl_ns)) = best else {
            rep.bad(
                name,
                format!("{shown_op}: summary names another library, raw has no rival"),
            );
            continue;
        };
        let Some(rind) = t.get("rind").copied() else {
            rep.bad(
                name,
                format!("{shown_op}: summary names rind, raw has no rind row"),
            );
            continue;
        };
        let ratio = rind / fl_ns;
        let p = parse_cell(&m[2], &cell_re).expect("a time");
        n += 1;
        if !close(p.ns, p.ns_tol, rind) {
            rep.bad(name, format!("{shown_op}: time {} vs {rind:.1} ns", &m[2]));
        }
        let shown: f64 = m[3].parse().unwrap_or(f64::NAN);
        if &m[4] != fl || (shown - ratio).abs() > 0.005 + 1e-9 {
            rep.bad(
                name,
                format!(
                    "{shown_op}: shows {}x {}, raw {ratio:.3}x {fl}",
                    &m[3], &m[4]
                ),
            );
        }
    }
    let noisy = d
        .get("load")
        .and_then(|l| l.get("noisy"))
        .or_else(|| d.get("noisy"));
    let noisy = match noisy {
        None | Some(Value::Null) => "None".to_string(),
        Some(Value::Bool(true)) => "True".into(),
        Some(Value::Bool(false)) => "False".into(),
        Some(v) => v.to_string(),
    };
    println!("{name}: summary.md {n} rows checked, noisy={noisy}");
}

/// Maps recorded private rind-bench SHAs to public rind-hardware commits.
/// Duplicated from xtask: the two crates share no code.
const PROVENANCE_MAP: &str = "tools/legacy-rind-bench-map.json";
const SQUASH: &str = "8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3";
const IMAGE_JSON: &[&str] = &[
    "results/images.json",
    "results/nrf52840/images.json",
    "results/diag/esp32s3-40mhz-2026-11-11/images.json",
    "results/diag/esp32s3-99b509a-80mhz-2026-11-12/images.json",
    "results/diag/nrf52840-99b509a-2026-11-12/images.json",
    "hosts/rp2040/images.json",
];
const RESULTS_MD: &[&str] = &[
    "results/RESULTS.md",
    "results/nrf52840/RESULTS.md",
    "results/diag/esp32s3-40mhz-2026-11-11/RESULTS.md",
    "results/diag/esp32s3-99b509a-80mhz-2026-11-12/RESULTS.md",
    "results/diag/nrf52840-99b509a-2026-11-12/RESULTS.md",
];

fn recorded_sha(s: &str) -> Option<String> {
    let mut last = None;
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i].is_ascii_hexdigit() && !bytes[i].is_ascii_uppercase() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_hexdigit() && !bytes[i].is_ascii_uppercase()
            {
                i += 1;
            }
            if (7..41).contains(&(i - start)) {
                last = Some(s[start..i].to_string());
            }
        } else {
            i += 1;
        }
    }
    last
}

struct HwMap(BTreeMap<String, Option<String>>);

fn load_hw_map() -> HwMap {
    let v: Value = serde_json::from_str(&read(&root().join(PROVENANCE_MAP))).expect("map json");
    let mut by = BTreeMap::new();
    for c in v["commits"].as_array().into_iter().flatten() {
        let legacy = c["legacy"].as_str().unwrap_or_default();
        let hw = c["rind_hardware"].as_str().map(str::to_string);
        if legacy.is_empty() {
            continue;
        }
        by.insert(legacy.to_string(), hw.clone());
        if legacy.len() >= 7 {
            by.entry(legacy[..7].to_string()).or_insert(hw.clone());
        }
        for a in c["aliases"].as_array().into_iter().flatten() {
            let a = a.as_str().unwrap_or_default();
            if a.is_empty() {
                continue;
            }
            by.insert(a.to_string(), hw.clone());
            if a.len() >= 7 {
                by.entry(a[..7].to_string()).or_insert(hw.clone());
            }
        }
    }
    HwMap(by)
}

fn expected_hw(map: &HwMap, recorded: &str) -> String {
    let sha = recorded_sha(recorded).unwrap_or_default();
    let hit = map.0.get(&sha).cloned().flatten().or_else(|| {
        (sha.len() >= 7)
            .then(|| map.0.get(&sha[..7]).cloned().flatten())
            .flatten()
    });
    match hit {
        Some(h) => h,
        None => format!("ancestor of {SQUASH}"),
    }
}

fn hw_cell(hw: &str) -> String {
    if let Some(rest) = hw.strip_prefix("ancestor of ") {
        format!("ancestor of `{}`", &rest[..7.min(rest.len())])
    } else {
        format!("`{}`", &hw[..7.min(hw.len())])
    }
}

/// A current provenance document names the upstream pin in `commit` and the
/// patched tree in `fork_commit`, and lists patches as `{name, sha256}`.
/// Published documents list patch filenames and have no `commit` field.
fn new_provenance(doc: &Value) -> bool {
    doc.get("commit").and_then(|v| v.as_str()).is_some()
        && doc.get("fork_commit").and_then(|v| v.as_str()).is_some()
}

fn check_patch_list(label: &str, patches: &Value, rep: &mut Report) {
    if patches.is_null() {
        return;
    }
    let Some(arr) = patches.as_array() else {
        rep.bad(label, "patches is not a list".into());
        return;
    };
    for (i, p) in arr.iter().enumerate() {
        match p {
            Value::String(s) if !s.is_empty() => {}
            Value::Object(o) => {
                let name = o.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let sha = o.get("sha256").and_then(|v| v.as_str()).unwrap_or("");
                let hex = sha.len() == 64 && sha.chars().all(|c| c.is_ascii_hexdigit());
                if name.is_empty() || !hex {
                    rep.bad(label, format!("patches[{i}] needs name and a sha256"));
                }
            }
            _ => rep.bad(
                label,
                format!("patches[{i}] is not a filename or {{name, sha256}}"),
            ),
        }
    }
}

/// `commit` is the upstream SHA. `images.*.rind` is the fork commit when
/// `fork_commit` is set, and the recorded rind sha otherwise.
fn check_upstream_pin(label: &str, doc: &Value, rep: &mut Report) {
    let Some(imgs) = doc.get("images").and_then(|v| v.as_object()) else {
        return;
    };
    let upstream = doc.get("commit").and_then(|v| v.as_str());
    let fork = doc.get("fork_commit").and_then(|v| v.as_str());
    if upstream.is_none() && fork.is_none() {
        return;
    }
    for (name, i) in imgs {
        if let (Some(up), Some(c)) = (upstream, i.get("commit").and_then(|v| v.as_str())) {
            if c != up {
                rep.bad(label, format!("{name}: commit {c} != upstream {up}"));
            }
        }
        if let Some(fork) = fork {
            if let Some(rind) = i.get("rind").and_then(|v| v.as_str()) {
                if rind != fork {
                    rep.bad(label, format!("{name}: rind {rind} != fork_commit {fork}"));
                }
            }
            if let Some(f) = i.get("fork_commit").and_then(|v| v.as_str()) {
                if f != fork {
                    rep.bad(label, format!("{name}: fork_commit {f} != {fork}"));
                }
            }
        }
    }
}

fn check_provenance(rep: &mut Report) {
    let map = load_hw_map();
    let mut images = 0;
    for rel in IMAGE_JSON {
        let path = root().join(rel);
        let v: Value = serde_json::from_str(&read(&path)).expect("images json");
        check_patch_list(rel, &v["patches"], rep);
        check_upstream_pin(rel, &v, rep);
        let fresh = new_provenance(&v);
        let Some(imgs) = v["images"].as_object() else {
            rep.bad("provenance", format!("{rel}: no images object"));
            continue;
        };
        for (name, i) in imgs {
            images += 1;
            let rec = i
                .get("legacy_rind_bench")
                .and_then(|x| x.as_str())
                .unwrap_or("");
            if rec.is_empty() {
                // A current document names commit and fork_commit instead of a
                // legacy rind-bench SHA. Published documents still have one.
                if fresh {
                    continue;
                }
                rep.bad(
                    "provenance",
                    format!("{rel}:{name}: missing legacy_rind_bench"),
                );
                continue;
            }
            let want = expected_hw(&map, rec);
            let have = i
                .get("rind_hardware")
                .and_then(|x| x.as_str())
                .unwrap_or("");
            if have != want {
                rep.bad(
                    "provenance",
                    format!("{rel}:{name}: rind_hardware {have:?}, map says {want}"),
                );
            }
            let alias = i.get("rind_bench").and_then(|x| x.as_str()).unwrap_or("");
            if !alias.is_empty() && alias != rec {
                rep.bad(
                    "provenance",
                    format!("{rel}:{name}: rind_bench does not match legacy_rind_bench"),
                );
            }
        }
    }
    let mut rows = 0;
    for rel in RESULTS_MD {
        let text = read(&root().join(rel));
        // Find the sibling images.json (same directory).
        let img_path = root().join(rel).with_file_name("images.json");
        let v: Value =
            serde_json::from_str(&read(&img_path)).expect("images json beside RESULTS.md");
        check_patch_list(rel, &v["patches"], rep);
        check_upstream_pin(rel, &v, rep);
        if let Some(up) = v.get("commit").and_then(|c| c.as_str()) {
            if !text.contains(up) {
                rep.bad(
                    "provenance",
                    format!("{rel}: RESULTS.md does not name upstream commit {up}"),
                );
            }
        }
        if let Some(fork) = v.get("fork_commit").and_then(|c| c.as_str()) {
            if !text.contains(fork) {
                rep.bad(
                    "provenance",
                    format!("{rel}: RESULTS.md does not name fork commit {fork}"),
                );
            }
        }
        let imgs = v["images"].as_object().expect("images");
        let mut in_table = false;
        for line in text.lines() {
            if line.contains("| Image |") && line.contains("legacy rind-bench") {
                in_table = true;
                continue;
            }
            if in_table && line.starts_with("  |---") {
                continue;
            }
            if in_table && line.starts_with("  | ") {
                let cols: Vec<&str> = line
                    .trim()
                    .trim_matches('|')
                    .split('|')
                    .map(str::trim)
                    .collect();
                // name, sha256, rind, rind-hardware, legacy, built
                if cols.len() < 6 {
                    rep.bad("provenance", format!("{rel}: short provenance row: {line}"));
                    continue;
                }
                let name = cols[0];
                let Some(i) = imgs.get(name) else {
                    rep.bad(
                        "provenance",
                        format!("{rel}: table names {name}, not in images.json"),
                    );
                    continue;
                };
                rows += 1;
                let rec = i
                    .get("legacy_rind_bench")
                    .and_then(|x| x.as_str())
                    .unwrap_or("");
                let want_hw = hw_cell(
                    i.get("rind_hardware")
                        .and_then(|x| x.as_str())
                        .unwrap_or(""),
                );
                if cols[3] != want_hw {
                    rep.bad(
                        "provenance",
                        format!("{rel}:{name}: hardware col {:?}, want {want_hw}", cols[3]),
                    );
                }
                if cols[4] != rec {
                    rep.bad(
                        "provenance",
                        format!("{rel}:{name}: legacy col {:?}, want {rec}", cols[4]),
                    );
                }
                continue;
            }
            in_table = false;
        }
    }
    println!("provenance: {images} images, {rows} RESULTS.md rows checked");
}

/// A Linux host's directory under results/: raw/run-*.json (bench-report's
/// schema 1) recomputed into medians and compared with the median JSON,
/// RESULTS.md (headline and every-library tables, the win count) and RUNS.md.
fn check_host(name: &str, rep: &mut Report) {
    check_host_at(&root().join("results").join(name), name, rep);
}

/// `name` is the host id, which is also the median JSON's file stem
/// (`<name>.json`) for the trees published under `results/<name>/`.
fn check_host_at(dir: &Path, name: &str, rep: &mut Report) {
    let heading = |l: &str| -> String {
        match l {
            "rustcrypto" => "RustCrypto + dalek".into(),
            "openssl" => "OpenSSL".into(),
            other => other.into(),
        }
    };
    let mut runs: Vec<(u32, Value)> = vec![];
    for e in std::fs::read_dir(dir.join("raw"))
        .into_iter()
        .flatten()
        .flatten()
    {
        let n = e.file_name().to_string_lossy().into_owned();
        if let Some(k) = n.strip_prefix("run-").and_then(|x| x.strip_suffix(".json")) {
            if let Ok(k) = k.parse() {
                runs.push((k, serde_json::from_str(&read(&e.path())).expect("run json")));
            }
        }
    }
    runs.sort_by_key(|r| r.0);
    if runs.is_empty() {
        rep.bad(name, "no raw/run-*.json".into());
        return;
    }
    // operation -> library -> each run's estimate, in first-seen order.
    let mut by: Ordered<Ordered<Vec<f64>>> = Ordered::default();
    for (_, v) in &runs {
        for r in v["results"].as_array().into_iter().flatten() {
            by.entry(&s(&r["operation"]), Ordered::default)
                .entry(&s(&r["library"]), Vec::new)
                .push(fv(&r["ns"]));
        }
    }
    let spread = |xs: &[f64]| {
        let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (hi - lo) / med(xs) * 100.0
    };
    let mut n = 0;
    for (op, libs) in &by.0 {
        for (l, xs) in &libs.0 {
            if xs.len() != runs.len() {
                rep.bad(
                    name,
                    format!("{op} {l}: in {} of {} runs", xs.len(), runs.len()),
                );
            }
        }
        n += libs.0.len();
    }
    // The median JSON.
    let host: Value =
        serde_json::from_str(&read(&dir.join(format!("{name}.json")))).expect("host json");
    let mut seen = 0;
    for r in host["results"].as_array().into_iter().flatten() {
        let (op, l) = (s(&r["operation"]), s(&r["library"]));
        match by.get(&op).and_then(|t| t.get(&l)) {
            Some(xs) => {
                seen += 1;
                if (fv(&r["ns"]) - med(xs)).abs() > 1e-9 * med(xs) {
                    rep.bad(
                        name,
                        format!(
                            "{name}.json {op} {l}: ns {} vs median {}",
                            fv(&r["ns"]),
                            med(xs)
                        ),
                    );
                }
            }
            None => rep.bad(name, format!("{name}.json {op} {l}: not in the raw runs")),
        }
    }
    if seen != n {
        rep.bad(
            name,
            format!("{name}.json has {seen} results, the raw runs {n}"),
        );
    }
    // RESULTS.md.
    let md = read(&dir.join("RESULTS.md"));
    let cell_re = Regex::new(r"^([\d.,]+) (ns|µs|us|ms|s)").unwrap();
    let head = Regex::new(r"^\| (\S+) \| ([\d.,]+ \S+) \| ([^,|]+), ([\d.,]+ \S+) \| ([\d.]+)× \| ([\d.]+)%, ([\d.]+)% \|$").unwrap();
    let (mut rows, mut wins, mut timed) = (0, 0, 0);
    for (op, libs) in &by.0 {
        let Some(rind) = libs.get("rind") else {
            continue;
        };
        let mut best: Option<(&str, &Vec<f64>)> = None;
        for (l, xs) in &libs.0 {
            if l != "rind" && !l.starts_with("rind-") && best.is_none_or(|(_, b)| med(xs) < med(b))
            {
                best = Some((l, xs));
            }
        }
        let Some((bl, bx)) = best else { continue };
        timed += 1;
        wins += usize::from(med(rind) <= med(bx));
        let want = label(op);
        let Some(m) = md
            .lines()
            .filter_map(|x| head.captures(x))
            .find(|m| m[1] == *want)
        else {
            rep.bad(name, format!("RESULTS.md: no headline row for {want}"));
            continue;
        };
        rows += 1;
        let t = |c: &str| parse_cell(c, &cell_re).expect("a time");
        let (rc, bc) = (t(&m[2]), t(&m[4]));
        if !close(rc.ns, rc.ns_tol, med(rind)) || !close(bc.ns, bc.ns_tol, med(bx)) {
            rep.bad(
                name,
                format!(
                    "{want}: shows {} and {}, raw {:.1} and {:.1} ns",
                    &m[2],
                    &m[4],
                    med(rind),
                    med(bx)
                ),
            );
        }
        let ratio = med(rind) / med(bx);
        let shown: f64 = m[5].parse().unwrap_or(f64::NAN);
        if m[3] != heading(bl) || (shown - ratio).abs() > 0.005 + 1e-9 {
            rep.bad(
                name,
                format!("{want}: shows {}x {}, raw {ratio:.3}x {bl}", &m[5], &m[3]),
            );
        }
        for (shown, xs) in [(&m[6], rind), (&m[7], bx)] {
            let v: f64 = shown.parse().unwrap_or(f64::NAN);
            if (v - spread(xs)).abs() > 0.05 + 1e-9 {
                rep.bad(
                    name,
                    format!("{want}: spread shows {v}%, raw {:.2}%", spread(xs)),
                );
            }
        }
    }
    let tally = format!("rind is the fastest on {wins} of {timed} operations");
    if !md.contains(&tally) {
        rep.bad(name, format!("RESULTS.md does not say: {tally}"));
    }
    // The every-library table: its header names the columns.
    let mut cols: Vec<String> = vec![];
    let mut cells = 0;
    for line in md.lines() {
        let parts: Vec<&str> = line.trim_matches('|').split(" | ").map(str::trim).collect();
        if line.starts_with("| Operation | rind | aws-lc-rs") {
            cols = parts[1..].iter().map(|x| x.to_string()).collect();
            continue;
        }
        if cols.is_empty() || !line.starts_with("| ") || parts.len() != cols.len() + 1 {
            continue;
        }
        let Some((op, libs)) = by.0.iter().find(|(o, _)| label(o) == parts[0]) else {
            continue;
        };
        for (c, h) in parts[1..].iter().zip(&cols) {
            let l = libs.0.iter().find(|(l, _)| heading(l) == *h);
            match (l, *c) {
                (None, "-") => {}
                (None, c) => rep.bad(name, format!("{op} {h}: shows {c}, not in raw")),
                (Some(_), "-") => rep.bad(name, format!("{op} {h}: raw has it, table shows -")),
                (Some((_, xs)), c) => {
                    let p = parse_cell(c, &cell_re).expect("a time");
                    cells += 1;
                    if !close(p.ns, p.ns_tol, med(xs)) {
                        rep.bad(
                            name,
                            format!("{op} {h}: shows {c}, raw median {:.1} ns", med(xs)),
                        );
                    }
                }
            }
        }
    }
    // RUNS.md: each run's estimate and the median.
    let runs_md = read(&dir.join("RUNS.md"));
    let mut run_rows = 0;
    for line in runs_md.lines() {
        let parts: Vec<&str> = line.trim_matches('|').split(" | ").map(str::trim).collect();
        if parts.len() != runs.len() + 4 || !line.starts_with("| ") {
            continue;
        }
        let Some((_, libs)) = by.0.iter().find(|(o, _)| label(o) == parts[0]) else {
            continue;
        };
        let Some((l, xs)) = libs.0.iter().find(|(l, _)| heading(l) == parts[1]) else {
            continue;
        };
        run_rows += 1;
        for (i, x) in xs.iter().enumerate() {
            let p = parse_cell(parts[2 + i], &cell_re).expect("a time");
            if !close(p.ns, p.ns_tol, *x) {
                rep.bad(
                    name,
                    format!(
                        "RUNS.md {} {l} run {}: {} vs {x:.1} ns",
                        parts[0],
                        i + 1,
                        parts[2 + i]
                    ),
                );
            }
        }
        let p = parse_cell(parts[2 + xs.len()], &cell_re).expect("a time");
        if !close(p.ns, p.ns_tol, med(xs)) {
            rep.bad(
                name,
                format!("RUNS.md {} {l}: median {}", parts[0], parts[2 + xs.len()]),
            );
        }
    }
    if run_rows != n {
        rep.bad(
            name,
            format!("RUNS.md has {run_rows} rows, the raw runs {n}"),
        );
    }
    println!(
        "{name}: {} runs, {n} results; {name}.json, {rows} headline rows, {cells} table cells and {run_rows} RUNS.md rows checked",
        runs.len()
    );
}

fn main() -> ExitCode {
    let mut boards = vec![];
    let mut dirs = vec![];
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--board" => boards.push(args.next().expect("--board needs a value")),
            s if s.starts_with("--board=") => boards.push(s["--board=".len()..].to_string()),
            "--dir" => dirs.push(args.next().expect("--dir needs a value")),
            s if s.starts_with("--dir=") => dirs.push(s["--dir=".len()..].to_string()),
            "-h" | "--help" => {
                println!(
                    "usage: cargo verify [--board esp32s3|nrf52840|efr32mg24|msi-001|<host under results/> ...]"
                );
                println!("       cargo verify --dir <output tree> [--dir <output tree> ...]");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("verify: unknown argument {other}");
                return ExitCode::from(2);
            }
        }
    }
    let asked_board = !boards.is_empty();
    if boards.is_empty() {
        boards = vec![
            "esp32s3".into(),
            "nrf52840".into(),
            "efr32mg24".into(),
            "msi-001".into(),
        ];
        for h in HOSTS {
            if root().join("results").join(h).join("raw").is_dir() {
                boards.push((*h).into());
            }
        }
    }
    // `--dir` alone checks those trees and leaves the published set alone.
    if !dirs.is_empty() && !asked_board {
        boards.clear();
    }
    let mut rep = Report::default();
    for b in &boards {
        if b == "msi-001" {
            check_msi(&mut rep);
        } else if HOSTS.contains(&b.as_str()) {
            check_host(b, &mut rep);
        } else if board_files(b).is_some() {
            check_board(b, &mut rep);
        } else {
            eprintln!("verify: unknown board {b}");
            return ExitCode::from(2);
        }
    }
    if dirs.is_empty() || asked_board {
        check_provenance(&mut rep);
    }
    for d in &dirs {
        tree::verify_dir(Path::new(d), &mut rep);
    }
    for n in &rep.notes {
        println!("NOTE {n}");
    }
    for p in &rep.problems {
        println!("MISMATCH {p}");
    }
    println!("{} mismatches", rep.problems.len());
    if rep.problems.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod provenance_tests {
    use super::*;

    fn doc_new() -> Value {
        serde_json::json!({
            "commit": "0b1f1469c8e17e361e3353432bc7fceee9e04214",
            "fork_commit": "09fa48d964fc3aa6ef6eb35490d9d49e357a53bd",
            "patches": [
                {"name": "0001-a.patch", "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"},
                {"name": "poly1305-32/0001-b.patch", "sha256": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"}
            ],
            "images": {
                "aes": {
                    "rind": "09fa48d964fc3aa6ef6eb35490d9d49e357a53bd",
                    "commit": "0b1f1469c8e17e361e3353432bc7fceee9e04214",
                    "fork_commit": "09fa48d964fc3aa6ef6eb35490d9d49e357a53bd"
                }
            }
        })
    }

    #[test]
    fn object_patches_and_the_upstream_pin_pass() {
        let doc = doc_new();
        let mut rep = Report::default();
        check_patch_list("t", &doc["patches"], &mut rep);
        check_upstream_pin("t", &doc, &mut rep);
        assert!(rep.problems.is_empty(), "{:?}", rep.problems);
        assert!(new_provenance(&doc));
    }

    #[test]
    fn filename_patch_lists_still_pass() {
        let doc = serde_json::json!({
            "patches": ["0001-a.patch", "ring-0.17.14-xtensa-poly1305-align.patch"]
        });
        let mut rep = Report::default();
        check_patch_list("t", &doc["patches"], &mut rep);
        check_upstream_pin("t", &doc, &mut rep);
        assert!(rep.problems.is_empty(), "{:?}", rep.problems);
        assert!(!new_provenance(&doc));
    }

    #[test]
    fn image_rind_must_be_the_fork_when_fork_commit_is_set() {
        let doc = serde_json::json!({
            "commit": "0b1f1469c8e17e361e3353432bc7fceee9e04214",
            "fork_commit": "09fa48d964fc3aa6ef6eb35490d9d49e357a53bd",
            "images": {"aes": {"rind": "985683329fb98aebfaae6c493f205f20070a8fca"}}
        });
        let mut rep = Report::default();
        check_upstream_pin("t", &doc, &mut rep);
        assert!(
            rep.problems.iter().any(|p| p.contains("fork_commit")),
            "{:?}",
            rep.problems
        );
    }
}
