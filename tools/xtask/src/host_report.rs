// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask host-report`: a Linux host's repeated `admin/bench-report
//! run`s (`raw/run-<n>.json`, schema 1, unchanged) into the host's median
//! JSON (schema 1, each time the median of the runs' criterion estimates),
//! RESULTS.md and RUNS.md. Gaps follow rind's rule (every library but rind
//! and its variants is a rival, OpenSSL included), and each one over 10%
//! needs an entry in the host's gaps.toml.
//!
//! The raw files keep the host's own clock. When the host's clock was unset,
//! `raw/run-<n>.pc.txt` (`pc_started:`/`pc_finished:` from the controlling PC)
//! supplies the published start and finish times instead.

use crate::report::{Gap, load_gaps};
use crate::tables::{Tables, human};
use crate::{py, read_json};
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};

#[derive(clap::Args)]
pub struct Args {
    /// the rind checkout the runs measured; its admin/bench-report is what the
    /// tables are evaluated from
    #[arg(long)]
    rind: PathBuf,
    /// the host directory: raw/run-*.json, gaps.toml, facts.txt, NOTES.md
    dir: PathBuf,
}

pub fn run_dir(rind: PathBuf, dir: PathBuf) -> Result<(), String> {
    run(Args { rind, dir })
}

/// rind's report columns, in order, with their headings.
const COLUMNS: &[(&str, &str)] = &[
    ("rind", "rind"),
    ("aws-lc-rs", "aws-lc-rs"),
    ("ring", "ring"),
    ("rustcrypto", "RustCrypto + dalek"),
    ("graviola", "graviola"),
    ("libcrux", "libcrux"),
    ("openssl", "OpenSSL"),
];

fn heading(lib: &str) -> &str {
    COLUMNS
        .iter()
        .find(|(l, _)| *l == lib)
        .map_or(lib, |(_, h)| h)
}

/// rind's own rows (rind and variants such as rind-dit-held) are never rivals.
fn rival(lib: &str) -> bool {
    lib != "rind" && !lib.starts_with("rind-")
}

/// `statistics.median`.
fn median(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

struct Row {
    op: String,
    lib: String,
    runs: Vec<f64>,
    /// the result of the run whose estimate is the median (lowest index on ties)
    pick: Value,
}

fn pc_times(path: &Path) -> (Option<String>, Option<String>) {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let get = |k: &str| {
        text.lines()
            .find_map(|l| l.strip_prefix(k))
            .map(|v| v.trim().to_string())
    };
    (get("pc_started:"), get("pc_finished:"))
}

fn label(op: &str) -> String {
    op.replace("/{lib}", "")
}

pub fn run(a: Args) -> Result<(), String> {
    let t = Tables::load(&a.rind)?;
    let raw = a.dir.join("raw");
    let mut names: Vec<String> = std::fs::read_dir(&raw)
        .map_err(|e| format!("{}: {e}", raw.display()))?
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.starts_with("run-") && n.ends_with(".json"))
        .collect();
    names.sort_by_key(|n| {
        n.trim_start_matches("run-")
            .trim_end_matches(".json")
            .parse::<u32>()
            .unwrap_or(u32::MAX)
    });
    let mut runs = vec![];
    for n in &names {
        let v = read_json(&raw.join(n))?;
        if v["status"] != "ok" {
            return Err(format!("{n}: status {}, not ok", v["status"]));
        }
        runs.push((n.trim_end_matches(".json").to_string(), v));
    }
    if runs.is_empty() {
        return Err(format!("no raw/run-*.json in {}", a.dir.display()));
    }
    let first = &runs[0].1;
    let host = first["host"].as_str().unwrap_or_default().to_string();
    for (n, v) in &runs {
        for k in [
            "host",
            "commit",
            "cpu",
            "toolchain",
            "criterion_args",
            "refused",
        ] {
            if v[k] != first[k] {
                return Err(format!("{n}: {k} differs from {}", runs[0].0));
            }
        }
    }

    // Every (operation, library) every run timed, in rind's operation order.
    let mut rows: Vec<Row> = vec![];
    for op in t.operations() {
        let mut libs: Vec<String> = vec![];
        for (_, v) in &runs {
            for r in v["results"].as_array().into_iter().flatten() {
                let lib = r["library"].as_str().unwrap_or_default().to_string();
                if r["operation"] == op.as_str() && !libs.contains(&lib) {
                    libs.push(lib);
                }
            }
        }
        libs.sort();
        for lib in libs {
            let mut got = vec![];
            for (n, v) in &runs {
                let r = v["results"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .find(|r| r["operation"] == op.as_str() && r["library"] == lib.as_str())
                    .ok_or(format!("{n}: no {op} for {lib}"))?;
                got.push(r.clone());
            }
            let ns: Vec<f64> = got
                .iter()
                .map(|r| r["ns"].as_f64().unwrap_or(f64::NAN))
                .collect();
            let m = median(&ns);
            let i = (0..ns.len())
                .min_by(|&x, &y| (ns[x] - m).abs().total_cmp(&(ns[y] - m).abs()))
                .unwrap();
            rows.push(Row {
                op: op.clone(),
                lib,
                runs: ns,
                pick: got[i].clone(),
            });
        }
    }

    // The median host JSON, in bench-report's schema.
    let mut out: Map<String, Value> = first.as_object().cloned().unwrap_or_default();
    let (start, _) = pc_times(&raw.join(format!("{}.pc.txt", runs[0].0)));
    let (_, end) = pc_times(&raw.join(format!("{}.pc.txt", runs[runs.len() - 1].0)));
    if let Some(s) = start {
        out.insert("started".into(), json!(s));
    }
    let last = &runs[runs.len() - 1].1;
    out.insert(
        "finished".into(),
        end.map_or(last["finished"].clone(), |e| json!(e)),
    );
    let noisy = runs.iter().any(|(_, v)| v["load"]["noisy"] == true);
    let mut load = first["load"].as_object().cloned().unwrap_or_default();
    load.insert("noisy".into(), json!(noisy));
    load.insert(
        "runs".into(),
        Value::Array(runs.iter().map(|(_, v)| v["load"].clone()).collect()),
    );
    out.insert("load".into(), Value::Object(load));
    out.insert("runs".into(), json!(runs.len()));
    out.insert(
        "results".into(),
        Value::Array(
            rows.iter()
                .map(|r| {
                    let mut o = r.pick.as_object().cloned().unwrap_or_default();
                    o.insert("ns".into(), json!(median(&r.runs)));
                    o.insert("runs_ns".into(), json!(r.runs));
                    Value::Object(o)
                })
                .collect(),
        ),
    );
    let out = Value::Object(out);
    let gaps = load_gaps(&t, &out, &a.dir.join("gaps.toml"), &rival)?;
    let json_path = a.dir.join(format!("{host}.json"));
    std::fs::write(&json_path, py::dumps(&out, Some(1)) + "\n")
        .map_err(|e| format!("{}: {e}", json_path.display()))?;

    write_results(&a.dir, &host, &runs, &rows, &out, &gaps)?;
    write_runs(&a.dir, &host, &runs, &rows)?;
    println!(
        "{}: {} runs, {} results, {} gaps decided",
        host,
        runs.len(),
        rows.len(),
        gaps.len()
    );
    Ok(())
}

fn spread(xs: &[f64]) -> f64 {
    let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (hi - lo) / median(xs) * 100.0
}

fn write_results(
    dir: &Path,
    host: &str,
    runs: &[(String, Value)],
    rows: &[Row],
    out: &Value,
    gaps: &[Gap],
) -> Result<(), String> {
    let commit = out["commit"].as_str().unwrap_or_default();
    let mut w: Vec<String> = vec![
        "<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->".into(),
        "<!-- Generated by `cargo xtask host-report` from raw/run-*.json and gaps.toml; edit those, not this. -->".into(),
        format!(
            "# rind on `{host}`: {}",
            out["cpu"]["model"].as_str().unwrap_or("?")
        ),
        String::new(),
        format!(
            "{} runs of rind's own `admin/bench-report run` at [`{}`](https://github.com/moq-dev/rind/commit/{commit}), \
             each pinned to one core. Every time is the median of the runs' criterion estimates (`{host}.json`, \
             schema 1, with each run's estimate in `runs_ns`); [RUNS.md](RUNS.md) lists every run, \
             [facts.txt](facts.txt) the hardware and load, and [NOTES.md](NOTES.md) how the host was run and \
             what to read into the numbers.",
            runs.len(),
            &commit[..commit.len().min(8)]
        ),
        String::new(),
        format!(
            "- CPU: {} ({}, {} logical CPUs)",
            out["cpu"]["model"].as_str().unwrap_or("?"),
            out["cpu"]["arch"].as_str().unwrap_or("?"),
            out["cpu"]["logical_cpus"]
        ),
        format!("- OS: {}", out["os"].as_str().unwrap_or("?")),
        format!(
            "- Toolchain: {}",
            out["toolchain"]["rustc"].as_str().unwrap_or("?")
        ),
        format!(
            "- OpenSSL linked: {}",
            out["libraries"]["openssl"]["linked"]
                .as_str()
                .unwrap_or("?")
        ),
    ];
    if let Some(c) = out["rind_cpu"]["cpuid"].as_object() {
        let on: Vec<&str> = c
            .iter()
            .filter(|(_, v)| **v == true || **v == 1)
            .map(|(k, _)| k.as_str())
            .collect();
        let off: Vec<&str> = c
            .iter()
            .filter(|(_, v)| **v == false || **v == 0)
            .map(|(k, _)| k.as_str())
            .collect();
        w.push(format!(
            "- CPU features rind reads (rind-cpuid): {}; absent: {}",
            on.join(", "),
            if off.is_empty() {
                "none".into()
            } else {
                off.join(", ")
            }
        ));
    }
    let refused = out["refused"].as_object().map_or(0, |o| o.len());
    w.push(format!(
        "- Libraries refused for missing CPU features: {}",
        if refused == 0 {
            "none".to_string()
        } else {
            out["refused"].to_string()
        }
    ));
    w.push(format!(
        "- Noisy (bench-report's flag, any run): {}",
        if out["load"]["noisy"] == true {
            "yes"
        } else {
            "no"
        }
    ));
    w.push(String::new());
    w.push("| Run | Other busy CPUs before build | before run | during run (harness limit) | noisy | results |".into());
    w.push("|---|---|---|---|---|---|".into());
    for (n, v) in runs {
        let l = &v["load"];
        w.push(format!(
            "| {n} | {} | {} | {} ({}) | {} | {} |",
            l["busy_cpus_before_build"],
            l["busy_cpus_before_run"],
            l["other_busy_cpus_during_run"],
            l["limit_cpus"],
            if l["noisy"] == true { "yes" } else { "no" },
            v["results"].as_array().map_or(0, |a| a.len())
        ));
    }
    w.push(String::new());

    w.push("## rind against the fastest other library".into());
    w.push(String::new());
    w.push(
        "Below 1.00× rind is faster. Spread is (slowest − fastest run) over the median, for rind and for the \
         fastest other library."
            .into(),
    );
    w.push(String::new());
    w.push("| Operation | rind | Fastest other | rind / fastest | Spread (rind, other) |".into());
    w.push("|---|---|---|---|---|".into());
    let mut ops: Vec<&str> = vec![];
    for r in rows {
        if !ops.contains(&r.op.as_str()) {
            ops.push(&r.op);
        }
    }
    let (mut wins, mut timed) = (0, 0);
    for op in &ops {
        let libs: Vec<&Row> = rows.iter().filter(|r| r.op == *op).collect();
        let Some(rind) = libs.iter().find(|r| r.lib == "rind") else {
            continue;
        };
        let others: Vec<&&Row> = libs.iter().filter(|r| rival(&r.lib)).collect();
        let Some(best) = others.iter().copied().reduce(|a, b| {
            if median(&b.runs) < median(&a.runs) {
                b
            } else {
                a
            }
        }) else {
            continue;
        };
        let (rm, bm) = (median(&rind.runs), median(&best.runs));
        timed += 1;
        wins += usize::from(rm <= bm);
        w.push(format!(
            "| {} | {} | {}, {} | {}× | {}%, {}% |",
            label(op),
            human(rm),
            heading(&best.lib),
            human(bm),
            py::fixed(rm / bm, 2),
            py::fixed(spread(&rind.runs), 1),
            py::fixed(spread(&best.runs), 1)
        ));
    }
    w.push(String::new());
    w.push(format!(
        "rind is the fastest on {wins} of {timed} operations it shares with another library."
    ));
    w.push(String::new());

    w.push("## Every library".into());
    w.push(String::new());
    w.push(format!(
        "| Operation | {} |",
        COLUMNS
            .iter()
            .map(|(_, h)| *h)
            .collect::<Vec<_>>()
            .join(" | ")
    ));
    w.push(format!("|---|{}", "---|".repeat(COLUMNS.len())));
    for op in &ops {
        let cells: Vec<String> = COLUMNS
            .iter()
            .map(|(lib, _)| {
                rows.iter()
                    .find(|r| r.op == *op && r.lib == *lib)
                    .map_or("-".into(), |r| human(median(&r.runs)))
            })
            .collect();
        w.push(format!("| {} | {} |", label(op), cells.join(" | ")));
    }
    let extra: Vec<&str> = rows
        .iter()
        .map(|r| r.lib.as_str())
        .filter(|l| !COLUMNS.iter().any(|(c, _)| c == l))
        .collect();
    if !extra.is_empty() {
        w.push(String::new());
        w.push(format!("Rows not in rind's columns: {}.", extra.join(", ")));
    }
    w.push(String::new());

    w.push("## Gaps".into());
    w.push(String::new());
    w.push(
        "Each operation where another library is more than 10% faster than rind, by rind's rule (OpenSSL \
         counts), with its reason from [gaps.toml](gaps.toml). `cargo xtask host-report` refuses a gap \
         without an entry and an entry that covers no gap."
            .into(),
    );
    w.push(String::new());
    if gaps.is_empty() {
        w.push("None.".into());
    } else {
        w.push("| Operations | rind over the fastest | Reason |".into());
        w.push("|---|---|---|".into());
        for (e, covered) in gaps {
            let ops: Vec<String> = covered
                .iter()
                .map(|(op, _, _)| format!("`{}`", label(op)))
                .collect();
            let rs: Vec<String> = covered
                .iter()
                .map(|(_, r, lib)| format!("{}× {}", py::fixed(*r, 2), heading(lib)))
                .collect();
            let why = e
                .get("reason")
                .or_else(|| e.get("quest"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            w.push(format!(
                "| {} | {} | {} |",
                ops.join(", "),
                rs.join("; "),
                why
            ));
        }
    }
    let p = dir.join("RESULTS.md");
    std::fs::write(&p, w.join("\n") + "\n").map_err(|e| format!("{}: {e}", p.display()))
}

fn write_runs(
    dir: &Path,
    host: &str,
    runs: &[(String, Value)],
    rows: &[Row],
) -> Result<(), String> {
    let mut w: Vec<String> = vec![
        "<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->".into(),
        "<!-- Generated by `cargo xtask host-report` from raw/run-*.json; edit those, not this. -->".into(),
        format!("# `{host}`: every run"),
        String::new(),
        "Each run's criterion estimate, the median published in RESULTS.md, and the spread \
         ((slowest − fastest) / median)."
            .into(),
        String::new(),
        format!(
            "| Operation | Library | {} | Median | Spread |",
            runs.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(" | ")
        ),
        format!("|---|---|{}---|---|", "---|".repeat(runs.len())),
    ];
    for r in rows {
        w.push(format!(
            "| {} | {} | {} | {} | {}% |",
            label(&r.op),
            heading(&r.lib),
            r.runs
                .iter()
                .map(|x| human(*x))
                .collect::<Vec<_>>()
                .join(" | "),
            human(median(&r.runs)),
            py::fixed(spread(&r.runs), 1)
        ));
    }
    let p = dir.join("RUNS.md");
    std::fs::write(&p, w.join("\n") + "\n").map_err(|e| format!("{}: {e}", p.display()))
}
