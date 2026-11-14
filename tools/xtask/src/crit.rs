// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Summaries of criterion homes and PMU logs from the A/B runs, ported from
//! the Python scripts the published tables came from. Each command keeps its
//! script's arguments and prints the same bytes:
//!
//! - `crit-parse ROOT` (`scripts/crit-parse.py`)
//! - `ab-cmp ROOT BASE MODE...` (`hosts/pi5/ab-narrow-mul/scripts/cmp.py`)
//! - `ab-min ROOT BASE MODE` (`hosts/pi5/ab-narrow-mul/scripts/minc.py`)
//! - `ab-collect ROOT RUNLOG META.json` (`hosts/pi5/ab-narrow-mul/scripts/collect.py`)
//! - `argon2-pmu-sum FILE [BASE]` (`scripts/argon2-pmu-sum.py`)
//! - `ab-pmu-sum FILE` (`hosts/pi5/ab-narrow-mul/series2-layout/scripts/pmu-sum.py`)
//! - `ab-perf-sum`, in the directory of `stat-*.csv` files
//!   (`hosts/pi5/ab-narrow-mul/series2-layout/scripts/perf-sum.py`)
//!
//! `ROOT` holds criterion homes named `<mode>-r<N>`; every `new/` directory
//! under one with a `benchmark.json` is a sample, timed by its slope estimate
//! or, when that is absent, its mean. Input the scripts rejected with a
//! traceback is an error here too.

use crate::py;
use regex::Regex;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(clap::Args)]
pub struct ParseArgs {
    root: PathBuf,
}

#[derive(clap::Args)]
pub struct CmpArgs {
    root: PathBuf,
    base: String,
    modes: Vec<String>,
}

#[derive(clap::Args)]
pub struct MinArgs {
    root: PathBuf,
    base: String,
    mode: String,
}

#[derive(clap::Args)]
pub struct CollectArgs {
    root: PathBuf,
    runlog: PathBuf,
    meta: PathBuf,
}

#[derive(clap::Args)]
pub struct Argon2PmuArgs {
    file: PathBuf,
    base: Option<String>,
}

#[derive(clap::Args)]
pub struct AbPmuArgs {
    file: PathBuf,
}

/// One criterion sample: its `full_id`, the label's mode and round digits,
/// and the estimate it is timed by (slope, else mean).
struct Sample {
    id: String,
    mode: String,
    round: String,
    est: Map<String, Value>,
}

impl Sample {
    fn ns(&self) -> Result<f64, String> {
        num(&self.est, "point_estimate")
    }
}

fn num(m: &Map<String, Value>, k: &str) -> Result<f64, String> {
    m.get(k)
        .ok_or_else(|| format!("KeyError: '{k}'"))?
        .as_f64()
        .ok_or_else(|| format!("{k} is not a number"))
}

/// Python truthiness of a JSON value.
fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64() != Some(0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

/// Every sample under `root/<mode>-r<N>/`, labels in name order, as
/// `os.walk` finds them (symlinked directories are not entered).
fn samples(root: &Path) -> Result<Vec<Sample>, String> {
    let label_re = Regex::new(r"^(.*)-r(\d+)$").expect("label regex");
    let mut labels = vec![];
    for e in fs::read_dir(root).map_err(|e| format!("{}: {e}", root.display()))? {
        let e = e.map_err(|e| format!("{}: {e}", root.display()))?;
        if let Some(name) = e.file_name().to_str() {
            labels.push(name.to_string());
        }
    }
    labels.sort();
    let mut out = vec![];
    for label in labels {
        let Some(c) = label_re.captures(&label) else {
            continue;
        };
        let (mode, round) = (c[1].to_string(), c[2].to_string());
        let mut dirs = vec![];
        news(&root.join(&label), &mut dirs);
        for dp in dirs {
            let b = crate::read_json(&dp.join("benchmark.json"))?;
            let e = crate::read_json(&dp.join("estimates.json"))?;
            let id = b
                .get("full_id")
                .ok_or("KeyError: 'full_id'")?
                .as_str()
                .ok_or("full_id is not a string")?
                .to_string();
            let est = match e.get("slope") {
                Some(s) if truthy(s) => s,
                _ => e.get("mean").ok_or("KeyError: 'mean'")?,
            };
            let est = est
                .as_object()
                .ok_or("the estimate is not an object")?
                .clone();
            out.push(Sample {
                id,
                mode: mode.clone(),
                round: round.clone(),
                est,
            });
        }
    }
    Ok(out)
}

/// Directories named `new` below `dir` that hold a non-directory
/// `benchmark.json`. Unreadable directories are skipped, as `os.walk` does.
fn news(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    let mut subs: Vec<_> = rd
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .map(|e| e.path())
        .collect();
    subs.sort();
    for s in subs {
        if s.file_name().is_some_and(|n| n == "new") {
            let b = s.join("benchmark.json");
            if b.symlink_metadata().is_ok() && !b.is_dir() {
                out.push(s.clone());
            }
        }
        news(&s, out);
    }
}

/// `statistics.median` of floats.
fn median(xs: &[f64]) -> Result<f64, String> {
    if xs.is_empty() {
        return Err("statistics.StatisticsError: no median for empty data".into());
    }
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    let n = v.len();
    Ok(if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    })
}

fn min(xs: &[f64]) -> Result<f64, String> {
    xs.iter()
        .copied()
        .reduce(|a, b| if b < a { b } else { a })
        .ok_or_else(|| "ValueError: min() arg is an empty sequence".into())
}

fn max(xs: &[f64]) -> Result<f64, String> {
    xs.iter()
        .copied()
        .reduce(|a, b| if b > a { b } else { a })
        .ok_or_else(|| "ValueError: max() arg is an empty sequence".into())
}

/// Python's `/` on floats, which raises rather than return an infinity.
fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("ZeroDivisionError: float division by zero".into())
    } else {
        Ok(a / b)
    }
}

/// `max(v)/min(v)-1`.
fn spread(v: &[f64]) -> Result<f64, String> {
    Ok(div(max(v)?, min(v)?)? - 1.0)
}

/// `str.rsplit("/", 1)[0]`.
fn op_of(id: &str) -> &str {
    id.rsplit_once('/').map_or(id, |(a, _)| a)
}

/// `float(s)`: surrounding whitespace allowed.
fn float(s: &str) -> Result<f64, String> {
    py_strip(s)
        .parse::<f64>()
        .map_err(|_| format!("ValueError: could not convert string to float: '{s}'"))
}

/// `str.strip()`: Python's whitespace also covers U+001C..U+001F.
fn py_strip(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c))
}

/// A text file's lines as Python iterates them: universal newlines, each
/// line keeping its `\n`.
fn lines(path: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    Ok(text.split_inclusive('\n').map(str::to_string).collect())
}

/// `str.splitlines()` on text already read with universal newlines.
fn splitlines(text: &str) -> Vec<String> {
    const SEP: &[char] = &[
        '\n', '\u{b}', '\u{c}', '\u{1c}', '\u{1d}', '\u{1e}', '\u{85}', '\u{2028}', '\u{2029}',
    ];
    let mut out: Vec<String> = text.split(SEP).map(str::to_string).collect();
    if text.is_empty() || text.ends_with(SEP) {
        out.pop();
    }
    out
}

fn by_id_mode(s: &[Sample]) -> Result<BTreeMap<(String, String), Vec<f64>>, String> {
    let mut d: BTreeMap<(String, String), Vec<f64>> = BTreeMap::new();
    for x in s {
        d.entry((x.id.clone(), x.mode.clone()))
            .or_default()
            .push(x.ns()?);
    }
    Ok(d)
}

/// `crit-parse.py`'s time cell.
fn crit_fmt(ns: f64) -> String {
    if ns >= 1e6 {
        format!("{:.3} ms", ns / 1e6)
    } else if ns >= 1e3 {
        format!("{:.2} us", ns / 1e3)
    } else {
        format!("{ns:.0} ns")
    }
}

pub fn parse(a: ParseArgs) -> Result<(), String> {
    let d = by_id_mode(&samples(&a.root)?)?;
    println!("| id | mode | runs | median | min | max | spread |\n|---|---|---|---|---|---|---|");
    for ((id, mode), v) in &d {
        println!(
            "| {id} | {mode} | {} | {} | {} | {} | {:.1}% |",
            v.len(),
            crit_fmt(median(v)?),
            crit_fmt(min(v)?),
            crit_fmt(max(v)?),
            100.0 * spread(v)?
        );
    }
    Ok(())
}

pub fn cmp(a: CmpArgs) -> Result<(), String> {
    let d = by_id_mode(&samples(&a.root)?)?;
    let us = |ns: f64| format!("{:.1}", ns / 1e3);
    let all: Vec<&String> = std::iter::once(&a.base).chain(&a.modes).collect();
    let ops: BTreeSet<&str> = d.keys().map(|(id, _)| op_of(id)).collect();
    let join = |v: Vec<String>| v.join(" | ");
    println!(
        "| op | {} | {} | aws-lc-rs µs | {} |",
        join(all.iter().map(|m| format!("{m} µs (n, spread)")).collect()),
        join(a.modes.iter().map(|m| format!("{m}/{}", a.base)).collect()),
        join(all.iter().map(|m| format!("{m}/aws")).collect()),
    );
    println!(
        "{}|",
        "|---".repeat(2 + 2 * a.modes.len() + 2 + a.modes.len())
    );
    for op in ops {
        let aws_id = format!("{op}/aws-lc-rs");
        let aws: Vec<f64> = d
            .iter()
            .filter(|((id, _), _)| *id == aws_id)
            .flat_map(|(_, v)| v.iter().copied())
            .collect();
        let rind_id = format!("{op}/rind");
        let mut meds: HashMap<&str, f64> = HashMap::new();
        let mut cells = vec![];
        for m in &all {
            match d.get(&(rind_id.clone(), (*m).clone())) {
                Some(v) if !v.is_empty() => {
                    let md = median(v)?;
                    meds.insert(m.as_str(), md);
                    cells.push(format!(
                        "{} ({}, {:.1}%)",
                        us(md),
                        v.len(),
                        100.0 * spread(v)?
                    ));
                }
                _ => cells.push("–".into()),
            }
        }
        let mut rat = vec![];
        for m in &a.modes {
            rat.push(match (meds.get(m.as_str()), meds.get(a.base.as_str())) {
                (Some(x), Some(b)) => format!("{:.3}", div(*x, *b)?),
                _ => "–".into(),
            });
        }
        // `if a`: a zero median counts as no aws-lc-rs row, as in the script.
        let aws_med = if aws.is_empty() {
            None
        } else {
            Some(median(&aws)?).filter(|x| *x != 0.0)
        };
        let mut ra = vec![];
        for m in &all {
            ra.push(match (aws_med, meds.get(m.as_str())) {
                (Some(x), Some(md)) => format!("{:.2}", md / x),
                _ => "–".into(),
            });
        }
        println!(
            "| {op} | {} | {} | {} | {} |",
            join(cells),
            join(rat),
            aws_med.map_or("–".into(), us),
            join(ra)
        );
    }
    Ok(())
}

pub fn min_rounds(a: MinArgs) -> Result<(), String> {
    let d = by_id_mode(&samples(&a.root)?)?;
    let (base, mode) = (&a.base, &a.mode);
    println!(
        "| op | {base} min µs (n) | {mode} min µs (n) | {mode}/{base} (min) |\n|---|---|---|---|"
    );
    let ids: BTreeSet<&String> = d
        .keys()
        .map(|(id, _)| id)
        .filter(|id| id.ends_with("/rind"))
        .collect();
    let none = vec![];
    for id in ids {
        let x = d.get(&(id.clone(), base.clone())).unwrap_or(&none);
        let y = d.get(&(id.clone(), mode.clone())).unwrap_or(&none);
        let (mx, my) = (min(x)?, min(y)?);
        println!(
            "| {} | {:.1} ({}) | {:.1} ({}) | {:.3} |",
            op_of(id),
            mx / 1e3,
            x.len(),
            my / 1e3,
            y.len(),
            div(my, mx)?
        );
    }
    Ok(())
}

pub fn collect(a: CollectArgs) -> Result<(), String> {
    let s = samples(&a.root)?;
    let mut meta = match crate::read_json(&a.meta)? {
        Value::Object(m) => m,
        _ => return Err("AttributeError: META is not a JSON object".into()),
    };
    let mut rows = vec![];
    for x in &s {
        let (op, lib) =
            x.id.rsplit_once('/')
                .ok_or("ValueError: not enough values to unpack (expected 2, got 1)")?;
        let round: i64 = x.round.parse().map_err(|e| format!("round: {e}"))?;
        let ci = x
            .est
            .get("confidence_interval")
            .ok_or("KeyError: 'confidence_interval'")?;
        let bound = |k: &str| ci.get(k).cloned().ok_or_else(|| format!("KeyError: '{k}'"));
        let mut r = Map::new();
        r.insert("operation".into(), format!("{op}/{{lib}}").into());
        r.insert("library".into(), lib.into());
        r.insert("id".into(), x.id.clone().into());
        r.insert("mode".into(), x.mode.clone().into());
        r.insert("round".into(), round.into());
        r.insert(
            "ns".into(),
            x.est
                .get("point_estimate")
                .cloned()
                .ok_or("KeyError: 'point_estimate'")?,
        );
        r.insert("ns_low".into(), bound("lower_bound")?);
        r.insert("ns_high".into(), bound("upper_bound")?);
        rows.push((x.id.clone(), x.mode.clone(), round, r));
    }
    rows.sort_by(|a, b| (&a.0, &a.1, a.2).cmp(&(&b.0, &b.1, b.2)));
    let log = fs::read_to_string(&a.runlog)
        .map_err(|e| format!("{}: {e}", a.runlog.display()))?
        .replace("\r\n", "\n")
        .replace('\r', "\n");
    meta.insert("schema".into(), "rind-results/ab-v1".into());
    meta.insert(
        "criterion_args".into(),
        Value::from(vec!["--warm-up-time", "1", "--measurement-time", "3"]),
    );
    meta.insert("log".into(), Value::from(splitlines(&log)));
    meta.insert(
        "results".into(),
        Value::Array(rows.into_iter().map(|r| Value::Object(r.3)).collect()),
    );
    print!("{}", py::dumps(&Value::Object(meta), Some(1)));
    Ok(())
}

type Row = HashMap<String, f64>;

pub fn argon2_pmu_sum(a: Argon2PmuArgs) -> Result<(), String> {
    let mut rows: HashMap<(String, String, String), Vec<Row>> = HashMap::new();
    let mut vs: Vec<String> = vec![];
    let mut sizes: BTreeSet<String> = BTreeSet::new();
    for l in lines(&a.file)? {
        if l.starts_with('#') {
            println!("{}", py_strip(&l));
            continue;
        }
        if l.contains("DEBUG") {
            continue;
        }
        let p: Vec<&str> = l.strip_suffix('\n').unwrap_or(&l).split('\t').collect();
        if p.len() < 4 {
            return Err(format!(
                "ValueError: not enough values to unpack (expected 4, got {})",
                p.len()
            ));
        }
        let (v, size, imp) = (p[1], p[2], p[3]);
        let mut kv = Row::new();
        for x in &p[4..] {
            let parts: Vec<&str> = x.split('=').collect();
            let [k, val] = parts[..] else {
                return Err(format!(
                    "ValueError: dictionary update sequence element has length {}; 2 is required",
                    parts.len()
                ));
            };
            kv.insert(k.to_string(), float(val)?);
        }
        let key = (size.to_string(), v.to_string(), imp.to_string());
        if !rows.contains_key(&key) && !vs.iter().any(|x| x == v) {
            vs.push(v.to_string());
        }
        sizes.insert(size.to_string());
        rows.entry(key).or_default().push(kv);
    }
    // sorted(..., key=lambda s: int(s[:-3])); ties (which the script left to
    // set order) fall back to name order.
    let mut sized = vec![];
    for s in sizes {
        let n = s.chars().count().saturating_sub(3);
        let head: String = s.chars().take(n).collect();
        let k: i64 = py_strip(&head)
            .parse()
            .map_err(|_| format!("ValueError: invalid literal for int() with base 10: '{head}'"))?;
        sized.push((k, s));
    }
    sized.sort();
    let col = |r: &[Row], k: &str| -> Result<Vec<f64>, String> {
        r.iter()
            .map(|x| x.get(k).copied().ok_or_else(|| format!("KeyError: '{k}'")))
            .collect()
    };
    let base = a.base.filter(|b| !b.is_empty());
    for (_, s) in sized {
        println!(
            "\n## {s}  (n = alternations; spread = (max-min)/median of the per-alternation medians)"
        );
        for imp in ["rind", "rustcrypto"] {
            for v in &vs {
                let Some(r) = rows.get(&(s.clone(), v.clone(), imp.to_string())) else {
                    continue;
                };
                let w = col(r, "wall_ms")?;
                let m = median(&w)?;
                let mut line = format!(
                    "{imp:10} {v:14} n={} med={m:8.3} ms min={:8.3} spread={:5.1}%",
                    w.len(),
                    min(&w)?,
                    div(max(&w)? - min(&w)?, m)? * 100.0
                );
                let br = base
                    .as_ref()
                    .and_then(|b| rows.get(&(s.clone(), b.clone(), imp.to_string())));
                if let Some(br) = br {
                    line += &format!(" ratio={:.3}", div(m, median(&col(br, "wall_ms")?)?)?);
                }
                if r[0].contains_key("cycles/blk") {
                    for k in [
                        "cycles/blk",
                        "inst/blk",
                        "stall_be/blk",
                        "stall_fe/blk",
                        "l1d_refill/blk",
                        "ipc",
                        "ghz",
                    ] {
                        line += &format!(" {}={:.2}", k.replace("/blk", ""), median(&col(r, k)?)?);
                    }
                }
                println!("{line}");
            }
        }
    }
    Ok(())
}

pub fn ab_pmu_sum(a: AbPmuArgs) -> Result<(), String> {
    let mut d: BTreeMap<(String, String), HashMap<String, Vec<f64>>> = BTreeMap::new();
    for l in lines(&a.file)? {
        let p: Vec<&str> = l.strip_suffix('\n').unwrap_or(&l).split('\t').collect();
        if p.len() < 3 || !p[2].contains('=') {
            continue;
        }
        let m = d.entry((p[1].to_string(), p[0].to_string())).or_default();
        for kv in &p[2..] {
            let parts: Vec<&str> = kv.split('=').collect();
            let [k, v] = parts[..] else {
                return Err(format!(
                    "ValueError: expected 2 values to unpack, got {}",
                    parts.len()
                ));
            };
            m.entry(k.to_string()).or_default().push(float(v)?);
        }
    }
    let keys = ["ns", "cycles", "inst", "br_mis", "l1i_refill", "stall_fe"];
    println!("| op | variant | n | {} | ns spread |", keys.join(" | "));
    println!("{}|", "|---".repeat(keys.len() + 4));
    let none = vec![];
    for ((op, v), m) in &d {
        let get = |k: &str| m.get(k).unwrap_or(&none);
        let ns = get("ns");
        let mut cells = vec![];
        for k in keys {
            cells.push(format!("{:.1}", median(get(k))?));
        }
        println!(
            "| {op} | {v} | {} | {} | {:.2}% |",
            ns.len(),
            cells.join(" | "),
            spread(ns)? * 100.0
        );
    }
    Ok(())
}

pub fn ab_perf_sum() -> Result<(), String> {
    let mut d: BTreeMap<(String, String), HashMap<String, Vec<f64>>> = BTreeMap::new();
    let mut files = vec![];
    for e in fs::read_dir(".").map_err(|e| format!(".: {e}"))? {
        let e = e.map_err(|e| format!(".: {e}"))?;
        if let Some(n) = e.file_name().to_str() {
            if n.starts_with("stat-") && n.ends_with(".csv") && n.len() >= "stat-.csv".len() {
                files.push(n.to_string());
            }
        }
    }
    for f in files {
        let stem: String = {
            let n = f.chars().count() - 4;
            f.chars().take(n).collect()
        };
        let p: Vec<&str> = stem.split('-').collect();
        let v = p.get(1).ok_or("IndexError: list index out of range")?;
        let op = if p.len() > 3 {
            p[2..p.len() - 1].join("-")
        } else {
            String::new()
        };
        // The script's defaultdict makes a row only once a counter line is read.
        for l in lines(Path::new(&f))? {
            let q: Vec<&str> = py_strip(&l).split(',').collect();
            if q.len() > 3 && !q[0].is_empty() && !q[2].is_empty() {
                let x = float(q[0])?;
                d.entry((op.clone(), v.to_string()))
                    .or_default()
                    .entry(q[2].to_string())
                    .or_default()
                    .push(x);
            }
        }
    }
    println!(
        "| op | binary | runs | IPC | backend stall cycles / kinst | L1D misses / kinst | L1I misses / kinst | branch misses / kinst |"
    );
    println!("|---|---|---|---|---|---|---|---|");
    let none = vec![];
    for ((op, v), m) in &d {
        let get = |k: &str| m.get(k).unwrap_or(&none);
        let i = median(get("instructions:u"))?;
        let c = median(get("cycles:u"))?;
        let g = |k: &str| -> Result<f64, String> { Ok(div(median(get(k))?, i)? * 1000.0) };
        let n = get("cycles:u").len();
        println!(
            "| {op} | {v} | {n} | {:.3} | {:.1} | {:.3} | {:.4} | {:.4} |",
            div(i, c)?,
            g("stalled-cycles-backend:u")?,
            g("L1-dcache-load-misses")?,
            g("L1-icache-load-misses")?,
            g("branch-misses:u")?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitlines_matches_python() {
        assert_eq!(splitlines(""), Vec::<String>::new());
        assert_eq!(splitlines("a\n"), ["a"]);
        assert_eq!(splitlines("a\n\nb"), ["a", "", "b"]);
        assert_eq!(splitlines("a\u{b}b\u{2028}c\n\n"), ["a", "b", "c", ""]);
    }

    #[test]
    fn crit_fmt_ranges() {
        assert_eq!(crit_fmt(812.5), "812 ns");
        assert_eq!(crit_fmt(1000.0), "1.00 us");
        assert_eq!(crit_fmt(2.5e6), "2.500 ms");
    }

    #[test]
    fn op_of_last_slash() {
        assert_eq!(op_of("p256/sign/rind"), "p256/sign");
        assert_eq!(op_of("plain"), "plain");
    }

    #[test]
    fn median_even_and_odd() {
        assert_eq!(median(&[3.0, 1.0, 2.0]).unwrap(), 2.0);
        assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]).unwrap(), 2.5);
        assert!(median(&[]).is_err());
    }
}
