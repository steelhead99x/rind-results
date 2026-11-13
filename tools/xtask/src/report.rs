// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask report`: turns captured board runs into rind's report shape
//! and RESULTS.md.
//!
//! It takes rind's `admin/bench-report` OPERATIONS, SECTIONS, LIBRARIES and
//! time format (through [`crate::tables`]), so the IDs, library names and
//! sections are the report's. It writes:
//!
//! - `<out>/<board>.json` (`esp32s3.json`, `nrf52840.json`): one host in
//!   `admin/bench-report`'s schema 1, with `results` for every report
//!   operation. `ns` is the median over runs of each run's median; `ns_low`
//!   and `ns_high` are the lowest and highest run medians, so they show
//!   run-to-run spread, not criterion's confidence interval.
//! - `<out>/<board>-all.json`: every benchmark the firmware timed, report
//!   operation or not, with each run's statistics.
//! - `<out>/RESULTS.md` and `<out>/RUNS.md`: the tables.

use crate::py::{self, Num};
use crate::tables::{Tables, human};
use crate::{git, read_json};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, default_value = "esp32s3", value_parser = ["esp32s3", "nrf52840"])]
    board: String,
    #[arg(long, default_value = "results/raw")]
    raw: PathBuf,
    #[arg(long)]
    rind: PathBuf,
    #[arg(long)]
    firmware: PathBuf,
    #[arg(long)]
    sizes: Option<PathBuf>,
    /// JSON with toolchain and board facts
    #[arg(long)]
    meta: Option<PathBuf>,
    /// decisions on gaps over 10%, as rind's gaps.toml
    #[arg(long, default_value = "results/gaps.toml")]
    gaps: PathBuf,
    #[arg(long, default_value = "results")]
    out: PathBuf,
    /// each image's provenance (sha256, rind and rind-hardware commits, build
    /// time), JSON; without it the rind and rind-hardware checkouts' HEADs
    /// are reported
    #[arg(long)]
    images: Option<PathBuf>,
}

/// What differs between the boards; the rest of the report is the same.
struct Board {
    title: &'static str,
    model: &'static str,
    arch: &'static str,
    os: fn(Option<&Value>) -> String,
    criterion: &'static str,
    note: &'static str,
    libs: &'static [&'static str],
    intro: &'static str,
    layout: &'static str,
    repro: &'static [&'static str],
    legend: &'static str,
}

fn esp_os(start: Option<&Value>) -> String {
    let idf = match start {
        Some(s) => py_str(&s["idf"]),
        None => "?".into(),
    };
    format!("ESP-IDF {idf} (FreeRTOS), bare board")
}

fn nrf_os(_: Option<&Value>) -> String {
    "none (bare metal, no_std); the board's UF2 bootloader and S140 SoftDevice stay in flash, unused".into()
}

const ESP32S3: Board = Board {
    title: "rind on an ESP32-S3",
    model: "Espressif ESP32-S3 (Xtensa LX7), 240 MHz",
    arch: "xtensa",
    os: esp_os,
    criterion: "rind-bench-criterion-esp: CCOUNT cycles, 0.2 s warm-up, 2 ms samples, 11-51 samples per benchmark",
    note: "One core of an ESP32-S3 at 240 MHz, timed with its cycle counter on core 1; no radio, PSRAM off.",
    libs: &["rind", "ring", "rustcrypto", "libcrux", "esp32s3-hw"],
    intro: "rind-bench's own benches, run unchanged but for the aws-lc-rs rows on one core of an ESP32-S3 \
            at 240 MHz, timed with the core's cycle counter.",
    layout: "Relinking an image moves its short-input rows far more than that: see \
             `diag/layout-relink-2026-11-12/` and, at this pin, the three-layout table in \
             `diag/esp32s3-relink-fixed-98568332/relink-short.md`; read short-input ratios with the README's layout caveat.",
    repro: &[
        "git clone --recursive https://github.com/steelhead99x/rind-hardware   # the firmware",
        "git clone https://github.com/steelhead99x/rind-results",
        "rind-results/tools/setup-toolchain.sh rind-hardware   # espup, espflash, ldproxy, at these versions",
        "cd rind-hardware && cargo xtask apply-patches && cargo xtask vendor-ring",
        "cargo xtask build-images esp   # esp32/images/<bench>.elf; check sha256 against images.sha256",
        "cd ../rind-results && PORT=/dev/serial/by-id/usb-Espressif_... cargo xtask run-all ../rind-hardware/esp32/images",
        "cargo xtask report --raw results/raw --rind ../rind-hardware/rind --firmware ../rind-hardware/esp32 \
         --sizes results/sizes.json --images results/images.json --meta results/meta-esp32s3.json --out results",
        "cargo verify   # recomputes every published number from raw",
    ],
    legend: "`esp32s3-hw` is ESP-IDF's mbedTLS on the chip's AES and SHA accelerators: hardware, not a Rust \
             library. rind runs its portable Rust backend here (it has no Xtensa assembly).",
};

const NRF52840: Board = Board {
    title: "rind on an nRF52840 (Seeed T1000-E)",
    model: "Nordic nRF52840 (Arm Cortex-M4F), 64 MHz",
    arch: "arm",
    os: nrf_os,
    criterion: "rind-bench-criterion-esp (no_std): DWT_CYCCNT cycles, 0.2 s warm-up, 2 ms samples, \
                11-51 samples per benchmark",
    note: "An nRF52840 at 64 MHz with the flash instruction cache on, timed with the DWT cycle counter; \
           USB only between benchmarks, no radio.",
    libs: &["rind", "ring", "rustcrypto", "libcrux"],
    intro: "rind-bench's own benches, run unchanged but for the aws-lc-rs rows, on the nRF52840 of a Seeed \
            T1000-E at 64 MHz, timed with the Cortex-M4's DWT cycle counter.",
    layout: LAYOUT_NRF,
    repro: &[
        "git clone --recursive https://github.com/steelhead99x/rind-hardware && cd rind-hardware",
        "cargo xtask apply-patches && cargo xtask vendor-ring && cargo xtask build-images nrf",
        "cd ../rind-results && cargo xtask run-all-nrf ../rind-hardware/nrf52840/images   # double-tap reset for the \
         UF2 drive; back up the board's firmware first",
        "cargo xtask report --board nrf52840 --raw results/raw-nrf52840 --rind ../rind-hardware/rind \
         --firmware ../rind-hardware/nrf52840 --gaps results/nrf52840/gaps.toml --images results/nrf52840/images.json \
         --out results/nrf52840",
        "cargo verify",
    ],
    legend: "rind runs its portable Rust backend here (its 32-bit Arm assembly needs ARM state and NEON, \
             which an M-profile core lacks); ring keeps its Thumb-2 assembly for SHA-2, AES and GHASH. There is no \
             hardware baseline: the chip's CryptoCell CC310 needs Nordic's closed library.",
};

/// The nRF52840's layout note; `--layout` replaces it once a relinked build has run there.
const LAYOUT_NRF: &str = "Relinked builds that move every library by about 1 KB and about 2 KB \
     (rind-hardware's `--pad 1031` and `--pad 2053`, padding ahead of the libraries since 2026-11-12, checked with `nm -n`) \
     moved no short-input ratio by more than 0.01 (`diag/nrf52840-relink-fixed-98568332/relink-short.md`: chacha20poly1305, \
     hkdf, hmac and sha2). An earlier relink of all twelve benches (`diag/nrf52840-relink-98568332/`) moved the Rust code \
     only 8 bytes, so it did not test layout. Unlike the ESP32-S3's, these rows hardly depend on layout (why is not tested).";

/// The libraries a gap is measured against: the other Rust libraries, as in
/// rind's gaps.toml. The hardware baseline is never a competitor.
const RIVALS: &[&str] = &["ring", "rustcrypto", "libcrux", "graviola", "aws-lc-rs"];

/// Python's `str()` of a JSON value, for the few places one is formatted.
fn py_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "None".into(),
        Value::Bool(b) => if *b { "True" } else { "False" }.into(),
        Value::Number(n) if n.is_f64() => py::repr(n.as_f64().unwrap()),
        other => other.to_string(),
    }
}

fn num(v: &Value) -> Num {
    Num::of(v).unwrap_or_else(|| panic!("not a number: {v}"))
}

fn f(v: &Value) -> f64 {
    num(v).f()
}

fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(_) => num(v).truthy(),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

struct Run {
    /// runs.json's entry, with `image` and maybe `flash_mhz` added.
    meta: Map<String, Value>,
    records: Vec<Value>,
}

impl Run {
    fn status(&self) -> &str {
        self.meta["status"].as_str().unwrap_or("")
    }
}

/// Every run under `raw`: one directory per firmware image, each holding
/// capture's `runs.json` and `run-<n>.jsonl`.
fn read_runs(raw: &Path) -> Result<Vec<Run>, String> {
    let speed = Regex::new(r"Boot SPI Speed\s*:\s*(\d+)MHz").unwrap();
    let mut names: Vec<String> = std::fs::read_dir(raw)
        .map_err(|e| format!("{}: {e}", raw.display()))?
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .collect();
    names.sort();
    let mut out = vec![];
    for image in names {
        let d = raw.join(&image);
        if !d.join("runs.json").is_file() {
            continue;
        }
        let runs = read_json(&d.join("runs.json"))?;
        for r in runs.as_array().cloned().unwrap_or_default() {
            let mut meta = r.as_object().cloned().unwrap_or_default();
            let n = py_str(&meta["run"]);
            let text = std::fs::read_to_string(d.join(format!("run-{n}.jsonl")))
                .map_err(|e| e.to_string())?;
            let records = text
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| serde_json::from_str(l).map_err(|e| format!("run-{n}.jsonl: {e}")))
                .collect::<Result<Vec<Value>, _>>()?;
            // `records` sits here in Python's dict before it is dropped; keep the key order.
            meta.insert("image".into(), json!(image));
            // The flash clock the bootloader reports; espflash sets it in the image header.
            let log = d.join(format!("run-{n}.log"));
            if log.is_file() {
                let text =
                    String::from_utf8_lossy(&std::fs::read(&log).map_err(|e| e.to_string())?)
                        .into_owned();
                let mhz = speed
                    .captures(&text)
                    .map(|c| json!(c[1].parse::<i64>().unwrap()));
                meta.insert("flash_mhz".into(), mhz.unwrap_or(Value::Null));
            }
            out.push(Run { meta, records });
        }
    }
    Ok(out)
}

/// Where firmware commits recorded before the public release live.
const PORT_HISTORY: &str = "rind-bench commits recorded before 2026-11-13 are from the pre-public \
    rind-bench history, published squashed as rind-hardware (https://github.com/steelhead99x/rind-hardware); \
    they are not in rind-hardware.";
/// The same note for RESULTS.md, with a link.
const PORT_HISTORY_MD: &str = "rind-bench commits recorded before 2026-11-13 are from the pre-public \
    rind-bench history, published squashed as [rind-hardware](https://github.com/steelhead99x/rind-hardware); \
    they are not in rind-hardware.";

/// The rind-hardware commit the images came from and the patches applied.
fn port_of(firmware: &Path) -> Result<Map<String, Value>, String> {
    let root = std::fs::canonicalize(firmware).map_err(|e| e.to_string())?;
    let root = root.parent().unwrap_or(&root);
    let mut patches: Vec<String> = std::fs::read_dir(root.join("patches"))
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|p| p.ends_with(".patch"))
        .collect();
    patches.sort();
    let mut m = Map::new();
    m.insert(
        "repo".into(),
        json!("https://github.com/steelhead99x/rind-hardware"),
    );
    m.insert("commit".into(), json!(git(root, &["rev-parse", "HEAD"])?));
    m.insert("history".into(), json!(PORT_HISTORY));
    m.insert("patches".into(), json!(patches));
    Ok(m)
}

fn lock_versions(firmware: &Path, t: &Tables) -> Result<Value, String> {
    let text = std::fs::read_to_string(firmware.join("Cargo.lock")).map_err(|e| e.to_string())?;
    let lock: toml::Value = toml::from_str(&text).map_err(|e| e.to_string())?;
    let mut have: Vec<(String, Vec<String>)> = vec![];
    for p in lock["package"].as_array().into_iter().flatten() {
        let name = p["name"].as_str().unwrap_or_default().to_string();
        let ver = p["version"].as_str().unwrap_or_default().to_string();
        match have.iter_mut().find(|(n, _)| *n == name) {
            Some((_, v)) => v.push(ver),
            None => have.push((name, vec![ver])),
        }
    }
    let get = |c: &str| have.iter().find(|(n, _)| n == c).map(|(_, v)| v.join(", "));
    let mut libs = Map::new();
    for (lib, crates) in &t.crates {
        let mut found = Map::new();
        for c in crates {
            if let Some(v) = get(c) {
                found.insert(c.clone(), json!(v));
            }
        }
        if !found.is_empty() {
            libs.insert(lib.clone(), Value::Object(found));
        }
    }
    for c in ["esp-idf-svc", "esp-idf-sys", "esp-idf-hal"] {
        if let Some(v) = get(c) {
            let e = libs.entry("esp-idf").or_insert_with(|| json!({}));
            e.as_object_mut().unwrap().insert(c.into(), json!(v));
        }
    }
    Ok(Value::Object(libs))
}

pub fn run(args: Args) -> Result<(), String> {
    let board = match args.board.as_str() {
        "nrf52840" => &NRF52840,
        _ => &ESP32S3,
    };
    let t = Tables::load(&args.rind)?;
    let runs = read_runs(&args.raw)?;
    let ok_runs: Vec<&Run> = runs.iter().filter(|r| r.status() == "ok").collect();
    let meta = match &args.meta {
        Some(p) => read_json(p)?,
        None => json!({}),
    };

    // id -> per-run result records
    let mut per_id: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    let mut checks: BTreeMap<(String, String), Vec<bool>> = BTreeMap::new();
    let mut skipped = Map::new();
    let mut start: Option<&Value> = None;
    let (mut clocks, mut timed) = (vec![], vec![]);
    for r in &ok_runs {
        for rec in &r.records {
            let ty = rec.get("type").and_then(|v| v.as_str()).unwrap_or("");
            if ty == "clock" {
                clocks.push(f(&rec["cycles"]) * 1e9 / f(&rec["wall_ns"]));
            }
            if ty == "result" && rec.get("wall_ns").is_some_and(truthy) {
                // The share of the sampling window the counter timed, at the
                // nominal clock: under 1 by the setup and bookkeeping between
                // samples, and never over 1 unless the counter is wrong.
                timed.push((f(&rec["sampled_cycles"]), f(&rec["wall_ns"])));
            }
            match ty {
                "result" => per_id
                    .entry(rec["id"].as_str().unwrap().into())
                    .or_default()
                    .push(rec),
                "check" => checks
                    .entry((py_str(&rec["check"]), py_str(&rec["library"])))
                    .or_default()
                    .push(truthy(&rec["pass"])),
                "skipped" => {
                    skipped.insert(
                        py_str(&rec["id"]),
                        rec.get("reason").cloned().unwrap_or(Value::Null),
                    );
                }
                "start" => start = Some(rec),
                _ => {}
            }
        }
    }

    let mut everything = Map::new();
    let mut results = vec![];
    for (full_id, recs) in &per_id {
        let medians: Vec<Num> = recs.iter().map(|x| num(&x["ns"])).collect();
        let med = py::median(&medians);
        let (lo, hi) = (py::min(&medians), py::max(&medians));
        let cycles = py::median(
            &recs
                .iter()
                .map(|x| num(&x["cycles"]["median"]))
                .collect::<Vec<_>>(),
        );
        let cpb0 = recs[0]
            .get("cycles_per_byte")
            .cloned()
            .unwrap_or(Value::Null);
        let cpb = if truthy(&cpb0) {
            py::median(
                &recs
                    .iter()
                    .map(|x| num(&x["cycles_per_byte"]))
                    .collect::<Vec<_>>(),
            )
            .value()
        } else {
            cpb0
        };
        let spread = 100.0 * (hi.f() - lo.f()) / med.f();
        let per_run: Vec<Value> = recs
            .iter()
            .map(|x| {
                let mut m = Map::new();
                for k in [
                    "ns",
                    "ns_low",
                    "ns_high",
                    "cycles",
                    "samples",
                    "iters_per_sample",
                ] {
                    m.insert(k.into(), x[k].clone());
                }
                Value::Object(m)
            })
            .collect();
        let throughput = recs[0].get("throughput").cloned().unwrap_or(Value::Null);
        let mut agg = Map::new();
        agg.insert("id".into(), json!(full_id));
        agg.insert("ns".into(), med.value());
        agg.insert("ns_low".into(), lo.value());
        agg.insert("ns_high".into(), hi.value());
        agg.insert("runs".into(), json!(recs.len()));
        agg.insert("run_spread_pct".into(), json!(spread));
        agg.insert("throughput".into(), throughput.clone());
        agg.insert("cycles".into(), cycles.value());
        agg.insert("cycles_per_byte".into(), cpb.clone());
        agg.insert("per_run".into(), Value::Array(per_run));
        everything.insert(full_id.clone(), Value::Object(agg));
        if let Some((op, row)) = t.operation_of(full_id) {
            let mut r = Map::new();
            r.insert("operation".into(), json!(op));
            r.insert("library".into(), json!(t.library(&row)));
            r.insert("id".into(), json!(full_id));
            r.insert("ns".into(), med.value());
            r.insert("ns_low".into(), lo.value());
            r.insert("ns_high".into(), hi.value());
            r.insert("throughput".into(), throughput);
            r.insert("cycles".into(), cycles.value());
            r.insert("cycles_per_byte".into(), cpb);
            r.insert("run_spread_pct".into(), json!(spread));
            results.push(Value::Object(r));
        }
    }

    let commit = git(&args.rind, &["rev-parse", "HEAD"])?;
    let mut host = Map::new();
    host.insert("schema".into(), json!(1));
    host.insert("host".into(), json!(args.board));
    host.insert("commit".into(), json!(commit));
    host.insert(
        "upstream_commit".into(),
        json!(git(&args.rind, &["merge-base", "HEAD", "origin/main"])?),
    );
    host.insert(
        "started".into(),
        ok_runs
            .first()
            .map_or(Value::Null, |r| r.meta["started"].clone()),
    );
    host.insert(
        "finished".into(),
        ok_runs
            .last()
            .map_or(Value::Null, |r| r.meta["finished"].clone()),
    );
    host.insert(
        "cpu".into(),
        json!({"model": board.model, "arch": board.arch, "logical_cpus": 1}),
    );
    host.insert("os".into(), json!((board.os)(start)));
    host.insert(
        "toolchain".into(),
        meta.get("toolchain").cloned().unwrap_or(json!({})),
    );
    host.insert("libraries".into(), lock_versions(&args.firmware, &t)?);
    host.insert(
        "rind_cpu".into(),
        json!({"backend": "portable", "has_aes_hardware": false}),
    );
    host.insert(
        "status".into(),
        json!(if ok_runs.len() == runs.len() {
            "ok"
        } else {
            "partial"
        }),
    );
    host.insert("criterion_args".into(), json!([board.criterion]));
    host.insert(
        "load".into(),
        json!({"runs": runs.len(), "ok_runs": ok_runs.len()}),
    );
    host.insert(
        "note".into(),
        meta.get("note").cloned().unwrap_or(json!(board.note)),
    );
    host.insert("port".into(), Value::Object(port_of(&args.firmware)?));
    host.insert("results".into(), Value::Array(results));
    let mut ext = ".elf".to_string();
    if let Some(p) = &args.images {
        let prov = read_json(p)?;
        let port = host["port"].as_object_mut().unwrap();
        port.insert("images".into(), prov["images"].clone());
        // The patches the images were built with, not those in the tree now.
        if let Some(p) = prov.get("patches") {
            port.insert("patches".into(), p.clone());
        }
        if let Some(e) = prov.get("file_ext").and_then(|v| v.as_str()) {
            ext = e.into();
        }
        let commits: BTreeSet<String> = prov["images"]
            .as_object()
            .into_iter()
            .flatten()
            .map(|(_, i)| py_str(&i["rind"]))
            .collect();
        let c = if commits.len() == 1 {
            json!(commits.iter().next().unwrap())
        } else {
            json!(commits)
        };
        host.insert("commit".into(), c.clone());
        host.insert("upstream_commit".into(), c);
    }
    let flash: BTreeSet<i64> = runs
        .iter()
        .filter_map(|r| {
            r.meta
                .get("flash_mhz")
                .and_then(|v| v.as_i64())
                .filter(|m| *m != 0)
        })
        .collect();
    if !flash.is_empty() {
        host.insert("flash_mhz".into(), json!(flash));
    }
    if !clocks.is_empty() {
        host.insert(
            "clock_check".into(),
            json!({"checks": clocks.len(), "min_hz": fmin(&clocks), "max_hz": fmax(&clocks)}),
        );
    }
    if !timed.is_empty() {
        let hz = f(&start.ok_or("results but no start record")?["cpu_hz"]);
        let timed: Vec<f64> = timed.iter().map(|(c, w)| c * 1e9 / hz / w).collect();
        let cc = host
            .entry("clock_check")
            .or_insert_with(|| json!({}))
            .as_object_mut()
            .unwrap();
        cc.insert("results".into(), json!(timed.len()));
        cc.insert("timed_min".into(), json!(fmin(&timed)));
        cc.insert("timed_max".into(), json!(fmax(&timed)));
        cc.insert(
            "over".into(),
            json!(timed.iter().filter(|x| **x > 1.001).count()),
        );
    }
    std::fs::create_dir_all(&args.out).map_err(|e| e.to_string())?;
    if args.images.is_some() {
        // images.sha256 (sha256sum format) may list more files than ran (ELFs beside UF2s): write it
        // when missing, else require every image that ran to be in it.
        let path = args.out.join("images.sha256");
        let imgs = host["port"]["images"]
            .as_object()
            .cloned()
            .unwrap_or_default();
        if path.exists() {
            let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let listed: BTreeSet<&str> = text
                .lines()
                .filter_map(|l| l.split_whitespace().next())
                .collect();
            let mut missing: Vec<&String> = imgs
                .iter()
                .filter(|(_, i)| !listed.contains(i["sha256"].as_str().unwrap_or("")))
                .map(|(n, _)| n)
                .collect();
            missing.sort();
            if !missing.is_empty() {
                let m: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
                return Err(format!(
                    "{} lacks the images that ran: {}",
                    path.display(),
                    m.join(", ")
                ));
            }
        } else {
            let mut names: Vec<&String> = imgs.keys().collect();
            names.sort();
            let text: String = names
                .iter()
                .map(|n| format!("{}  images/{n}{ext}\n", py_str(&imgs[*n]["sha256"])))
                .collect();
            std::fs::write(&path, text).map_err(|e| e.to_string())?;
        }
    }
    let host = Value::Object(host);
    write(
        &args.out.join(format!("{}.json", args.board)),
        py::dumps(&host, Some(1)) + "\n",
    )?;
    let all_runs: Vec<Value> = runs.iter().map(|r| Value::Object(r.meta.clone())).collect();
    let all_checks: Vec<Value> = checks
        .iter()
        .map(|((c, lib), p)| json!({"check": c, "library": lib, "pass": p.iter().all(|x| *x), "runs": p.len()}))
        .collect();
    let mut all = Map::new();
    all.insert("runs".into(), Value::Array(all_runs));
    all.insert("checks".into(), Value::Array(all_checks));
    all.insert("skipped".into(), Value::Object(skipped.clone()));
    all.insert("benchmarks".into(), Value::Object(everything.clone()));
    write(
        &args.out.join(format!("{}-all.json", args.board)),
        py::dumps(&Value::Object(all), Some(1)) + "\n",
    )?;

    let sizes = match &args.sizes {
        Some(p) if p.exists() => Some(read_json(p)?),
        _ => None,
    };
    let gaps = load_gaps(&t, &host, &args.gaps)?;
    let ctx = Ctx {
        t: &t,
        host: &host,
        everything: &everything,
        board,
        meta: &meta,
    };
    ctx.write_markdown(&args.out, &checks, &skipped, &runs, sizes.as_ref(), &gaps)
}

fn fmin(xs: &[f64]) -> f64 {
    xs.iter()
        .copied()
        .fold(f64::INFINITY, |a, b| if b < a { b } else { a })
}

fn fmax(xs: &[f64]) -> f64 {
    xs.iter()
        .copied()
        .fold(f64::NEG_INFINITY, |a, b| if b > a { b } else { a })
}

fn write(path: &Path, text: String) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Each operation where another Rust library beats rind by more than
/// admin/bench-report's SLOWER (1.1), as (operation, ratio, library), in the
/// order the results first name the operation.
fn gaps_of(t: &Tables, host: &Value) -> Vec<(String, f64, String)> {
    let mut by: Vec<(String, Vec<(String, f64)>)> = vec![];
    for r in host["results"].as_array().into_iter().flatten() {
        let op = py_str(&r["operation"]);
        let lib = py_str(&r["library"]);
        let ns = f(&r["ns"]);
        let libs = match by.iter_mut().find(|(o, _)| *o == op) {
            Some((_, l)) => l,
            None => {
                by.push((op, vec![]));
                &mut by.last_mut().unwrap().1
            }
        };
        match libs.iter_mut().find(|(l, _)| *l == lib) {
            Some(e) => e.1 = ns,
            None => libs.push((lib, ns)),
        }
    }
    let mut out = vec![];
    for (op, libs) in by {
        let others: Vec<&(String, f64)> = libs
            .iter()
            .filter(|(l, _)| RIVALS.contains(&l.as_str()))
            .collect();
        let Some(rind) = libs.iter().find(|(l, _)| l == "rind") else {
            continue;
        };
        if others.is_empty() {
            continue;
        }
        let best = first_min(&others);
        if rind.1 / best.1 > t.slower {
            out.push((op, rind.1 / best.1, best.0.clone()));
        }
    }
    out
}

fn first_min<'a>(xs: &[&'a (String, f64)]) -> &'a (String, f64) {
    let mut best = xs[0];
    for x in &xs[1..] {
        if x.1 < best.1 {
            best = x;
        }
    }
    best
}

/// `fnmatch.fnmatchcase` for the patterns gaps.toml uses: `*`, `?` and `[...]`.
fn fnmatch(name: &str, pat: &str) -> bool {
    let mut re = String::from("^");
    let mut chars = pat.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' => re.push_str("(?s:.*)"),
            '?' => re.push('.'),
            '[' => {
                let mut class = String::new();
                let mut closed = false;
                for d in chars.by_ref() {
                    if d == ']' && !class.is_empty() {
                        closed = true;
                        break;
                    }
                    class.push(d);
                }
                if closed {
                    let class = class
                        .strip_prefix('!')
                        .map_or(class.clone(), |r| format!("^{r}"));
                    re.push_str(&format!("[{}]", class.replace('\\', "\\\\")));
                } else {
                    re.push_str(&regex::escape(&format!("[{class}")));
                }
            }
            c => re.push_str(&regex::escape(&c.to_string())),
        }
    }
    re.push('$');
    Regex::new(&re).is_ok_and(|r| r.is_match(name))
}

type Gap = (toml::Table, Vec<(String, f64, String)>);

/// rind's gaps.toml rule on this host: every gap needs an entry with a quest
/// or a reason, and every entry must cover a gap.
fn load_gaps(t: &Tables, host: &Value, path: &Path) -> Result<Vec<Gap>, String> {
    let mut found = gaps_of(t, host);
    let mut entries = vec![];
    if path.exists() {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let doc: toml::Table =
            toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if let Some(a) = doc.get("gap").and_then(|g| g.as_array()) {
            entries = a.iter().filter_map(|e| e.as_table().cloned()).collect();
        }
    }
    let (mut out, mut problems) = (vec![], vec![]);
    let me = py_str(&host["host"]);
    for (i, e) in entries.into_iter().enumerate() {
        let i = i + 1;
        let allowed = ["operations", "on", "quest", "reason"];
        if e.keys().any(|k| !allowed.contains(&k.as_str()))
            || !e.contains_key("operations")
            || !e.contains_key("on")
        {
            problems.push(format!(
                "entry {i}: needs operations and on, and no other keys but quest or reason"
            ));
        }
        if e.contains_key("quest") == e.contains_key("reason") {
            problems.push(format!("entry {i}: needs a quest or a reason, not both"));
        }
        let list = |k: &str| -> Vec<String> {
            e.get(k)
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default()
        };
        if !list("on").contains(&me) {
            continue;
        }
        let ops = list("operations");
        let (covered, rest): (Vec<_>, Vec<_>) = found
            .into_iter()
            .partition(|(op, _, _)| ops.iter().any(|g| fnmatch(op, g)));
        found = rest;
        if covered.is_empty() {
            problems.push(format!(
                "entry {i} ({}): names no gap over 10%; update or remove it",
                ops.join(", ")
            ));
        }
        out.push((e, covered));
    }
    if !found.is_empty() {
        found.sort_by(|a, b| a.0.cmp(&b.0));
        let list: Vec<String> = found
            .iter()
            .map(|(op, r, lib)| format!("{op} ({}x {lib})", py::fixed(*r, 2)))
            .collect();
        problems.push(format!(
            "gaps over 10% without a decision: {}",
            list.join(", ")
        ));
    }
    if !problems.is_empty() {
        return Err(format!("{}:\n  {}", path.display(), problems.join("\n  ")));
    }
    Ok(out)
}

struct Ctx<'a> {
    t: &'a Tables,
    host: &'a Value,
    everything: &'a Map<String, Value>,
    board: &'a Board,
    meta: &'a Value,
}

fn cell(r: Option<&Value>) -> String {
    let Some(r) = r else { return "-".into() };
    let mut s = human(f(&r["ns"]));
    if r.get("cycles_per_byte").is_some_and(truthy) {
        s += &format!(" ({} c/B)", py::fixed(f(&r["cycles_per_byte"]), 1));
    } else {
        let c = f(&r["cycles"]);
        s += &if c >= 1e4 {
            format!(" ({} kc)", py::fixed_commas(c / 1e3, 0))
        } else {
            format!(" ({} c)", py::fixed_commas(c, 0))
        };
    }
    s
}

fn label(op: &str) -> String {
    op.replace("{lib}", "…")
}

impl Ctx<'_> {
    /// The harness's own cost and its clock check, when the firmware recorded them.
    fn harness_lines(&self) -> Vec<String> {
        let mut out = vec![];
        let mut empty: Vec<(String, &Value)> = self
            .everything
            .iter()
            .filter(|(k, _)| k.starts_with("harness/empty/"))
            .map(|(k, v)| (k.rsplit('/').next().unwrap().to_string(), v))
            .collect();
        let cc = self.host.get("clock_check");
        if empty.is_empty() && cc.is_none() {
            return out;
        }
        out.push("## Harness checks".into());
        out.push(String::new());
        if let Some(cc) = cc.filter(|c| c.get("checks").is_some()) {
            out.push(format!(
                "- Cycle counter against an independent wall clock (ESP-IDF's SYSTIMER, from the 40 MHz \
                 crystal) over a busy second at each boot ({} boots): {} to {} MHz.",
                py_str(&cc["checks"]),
                py::fixed(f(&cc["min_hz"]) / 1e6, 4),
                py::fixed(f(&cc["max_hz"]) / 1e6, 4)
            ));
        }
        if let Some(cc) = cc.filter(|c| c.get("results").is_some()) {
            out.push(format!(
                "- Over each benchmark's samples ({} results), the counter timed {}% to {}% of the wall-clock \
                 window (the rest is setup and bookkeeping between samples); {} results exceed 100.1%, which a \
                 wrapped, skipped or double-counted interval would show.",
                py_str(&cc["results"]),
                py::fixed(100.0 * f(&cc["timed_min"]), 1),
                py::fixed(100.0 * f(&cc["timed_max"]), 2),
                py_str(&cc["over"])
            ));
        }
        // Python's dict would keep the last of two equal keys; the IDs are unique per routine.
        empty.sort_by(|a, b| a.0.cmp(&b.0));
        let fastest = self
            .everything
            .iter()
            .filter(|(k, _)| !k.starts_with("harness/"))
            .map(|(_, e)| f(&e["cycles"]))
            .fold(f64::INFINITY, f64::min);
        for (k, v) in empty {
            out.push(format!(
                "- An empty `{k}` routine costs {} cycles per iteration (median over {} runs): the loop, \
                 `black_box` and, for `iter_batched`, the moved input. It is in every result, unsubtracted, \
                 as in criterion; against the fastest row here it is {}%.",
                py::fixed(f(&v["cycles"]), 1),
                py_str(&v["runs"]),
                py::fixed(100.0 * f(&v["cycles"]) / fastest, 2)
            ));
        }
        out.push(String::new());
        out
    }

    fn images_lines(&self) -> Vec<String> {
        let port = &self.host["port"];
        let Some(imgs) = port
            .get("images")
            .and_then(|v| v.as_object())
            .filter(|m| !m.is_empty())
        else {
            return vec![
                format!(
                    "- Firmware: [rind-hardware]({}) `{}`",
                    py_str(&port["repo"]),
                    py_str(&port["commit"])
                ),
                format!("  {PORT_HISTORY_MD}"),
            ];
        };
        let mut out = vec![
            "- Firmware images (one per bench file; `images.sha256` beside this file):".to_string(),
            String::new(),
            "  | Image | sha256 | rind | rind-bench | Built |".into(),
            "  |---|---|---|---|---|".into(),
        ];
        let mut names: Vec<&String> = imgs.keys().collect();
        names.sort();
        for name in names {
            let i = &imgs[name];
            let sha = py_str(&i["sha256"]);
            let rind = py_str(&i["rind"]);
            out.push(format!(
                "  | {name} | `{}…` | `{}` | {} | {} |",
                &sha[..16.min(sha.len())],
                &rind[..8.min(rind.len())],
                py_str(&i["rind_bench"]),
                py_str(&i["built"])
            ));
        }
        out.push(String::new());
        out.push(format!("  {PORT_HISTORY_MD}"));
        out.push(String::new());
        out
    }

    fn write_markdown(
        &self,
        out: &Path,
        checks: &BTreeMap<(String, String), Vec<bool>>,
        skipped: &Map<String, Value>,
        runs: &[Run],
        sizes: Option<&Value>,
        gaps: &[Gap],
    ) -> Result<(), String> {
        let (board, host) = (self.board, self.host);
        let libs = board.libs;
        let results: Vec<&Value> = host["results"].as_array().into_iter().flatten().collect();
        let by = |op: &str, lib: &str| -> Option<&Value> {
            results
                .iter()
                .rev()
                .find(|r| r["operation"] == op && r["library"] == lib)
                .copied()
        };
        let mut lines: Vec<String> = vec![];
        let mut w = |s: String| lines.push(s);
        w(format!("# {}", board.title));
        w(String::new());
        w(format!(
            "{} Every operation and ID is `admin/bench-report`'s.",
            board.intro
        ));
        w(String::new());
        let up = match &host["upstream_commit"] {
            Value::Array(a) => {
                a.iter()
                    .map(|c| format!("`{}`", py_str(c)))
                    .collect::<Vec<_>>()
                    .join(", ")
                    + " (per image, below)"
            }
            v => format!("`{}`", py_str(v)),
        };
        let patches: Vec<String> = host["port"]["patches"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|p| format!("`{}`", py_str(p)))
            .collect();
        w(format!(
            "- rind: upstream {up}, plus the port's patches: {}",
            patches.join(", ")
        ));
        for line in self.images_lines() {
            w(line);
        }
        w(format!("- OS: {}", py_str(&host["os"])));
        if let Some(fl) = host
            .get("flash_mhz")
            .and_then(|v| v.as_array())
            .filter(|a| !a.is_empty())
        {
            let mhz: Vec<String> = fl.iter().map(|m| format!("{} MHz", py_str(m))).collect();
            let default = fl.len() == 1 && fl[0] == 40;
            w(format!(
                "- SPI flash clock, as the bootloader reported it each boot: {}{}",
                mhz.join(", "),
                if default {
                    " (espflash's default; see the README)"
                } else {
                    ""
                }
            ));
        }
        if let Some(tc) = host
            .get("toolchain")
            .and_then(|v| v.as_object())
            .filter(|m| !m.is_empty())
        {
            w(format!(
                "- Toolchain: {}; {}",
                tc.get("rustc").map_or("?".into(), py_str),
                tc.get("gcc").map_or(String::new(), py_str)
            ));
        }
        w(format!(
            "- Runs: {} of {} completed, one firmware image per rind-bench bench file",
            runs.iter().filter(|r| r.status() == "ok").count(),
            runs.len()
        ));
        for r in runs {
            let m = &r.meta;
            let reason = m
                .get("reason")
                .filter(|v| truthy(v))
                .map_or(String::new(), |v| format!(", {}", py_str(v)));
            w(format!(
                "  - {} run {}: {}, {} results, {} to {} (UTC){reason}",
                py_str(&m["image"]),
                py_str(&m["run"]),
                py_str(&m["status"]),
                py_str(&m["results"]),
                py_str(&m["started"]),
                py_str(&m["finished"])
            ));
        }
        w(String::new());
        w(format!(
            "Each cell is the median over runs of each run's median time per operation, with cycles per byte for \
             byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. {}",
            board.legend
        ));
        w(String::new());
        for line in self.harness_lines() {
            w(line);
        }
        w("## Headline: rind against the fastest other Rust library".into());
        w(String::new());
        w("| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |"
            .into());
        w("|---|---|---|---|---|".into());
        for (_, ops) in &self.t.sections {
            for op in ops {
                let Some(ours) = by(op, "rind") else { continue };
                let others: Vec<(&str, &Value)> = libs[1..4]
                    .iter()
                    .filter_map(|lib| by(op, lib).map(|r| (*lib, r)))
                    .collect();
                if others.is_empty() {
                    continue;
                }
                let mut best = others[0];
                for o in &others[1..] {
                    if f(&o.1["ns"]) < f(&best.1["ns"]) {
                        best = *o;
                    }
                }
                let ratio = f(&ours["ns"]) / f(&best.1["ns"]);
                w(format!(
                    "| {} | {} | {}: {} | {}× | {}% |",
                    label(op),
                    cell(Some(ours)),
                    best.0,
                    cell(Some(best.1)),
                    py::fixed(ratio, 2),
                    py::fixed(f(&ours["run_spread_pct"]), 1)
                ));
            }
        }
        w(String::new());
        for (section, ops) in &self.t.sections {
            let rows: Vec<&String> = ops
                .iter()
                .filter(|op| libs.iter().any(|lib| by(op, lib).is_some()))
                .collect();
            if rows.is_empty() {
                continue;
            }
            w(format!("## {section}"));
            w(String::new());
            w(format!("| Operation | {} |", libs.join(" | ")));
            w(format!("|---|{}", "---|".repeat(libs.len())));
            for op in rows {
                let cells: Vec<String> = libs.iter().map(|lib| cell(by(op, lib))).collect();
                w(format!("| {} | {} |", label(op), cells.join(" | ")));
            }
            w(String::new());
        }
        w("## Known gaps".into());
        w(String::new());
        w("Every operation where another Rust library beats rind by more than 10% here, with the decision on it, \
           as rind's `benchmarks/gaps.toml` requires; `tools/report.py` refuses a gap without an entry in \
           `results/gaps.toml`, and an entry that covers no gap."
            .replace("`tools/report.py`", "`cargo xtask report`"));
        w(String::new());
        if gaps.is_empty() {
            w("None.".into());
        } else {
            w("| Operations | rind's time over the fastest | Fastest | Decision |".into());
            w("|---|---|---|---|".into());
            for (e, covered) in gaps {
                let mut ratios: Vec<f64> = covered.iter().map(|(_, r, _)| *r).collect();
                ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let (lo, hi) = (ratios[0], ratios[ratios.len() - 1]);
                let spread = format!("{}×", py::fixed(lo, 2))
                    + &if hi - lo >= 0.005 {
                        format!(" to {}×", py::fixed(hi, 2))
                    } else {
                        String::new()
                    };
                let fastest: BTreeSet<&str> = covered.iter().map(|(_, _, l)| l.as_str()).collect();
                let decision = match e
                    .get("reason")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                {
                    Some(r) => r.to_string(),
                    None => format!(
                        "quest: {}",
                        e.get("quest").and_then(|v| v.as_str()).unwrap_or("")
                    ),
                };
                let mut ops: Vec<&str> = covered.iter().map(|(o, _, _)| o.as_str()).collect();
                ops.sort();
                let ops: Vec<String> = ops.iter().map(|op| format!("`{}`", label(op))).collect();
                w(format!(
                    "| {} | {spread} | {} | {decision} |",
                    ops.join(", "),
                    fastest.into_iter().collect::<Vec<_>>().join(", ")
                ));
            }
        }
        w(String::new());
        w("## Known-answer checks".into());
        w(String::new());
        w("Run on the board before any timing, every run. A library failing an operation's check is not raced there."
            .into());
        w(String::new());
        w("| Check | Library | Result |".into());
        w("|---|---|---|".into());
        for ((c, lib), passes) in checks {
            let res = if passes.iter().all(|x| *x) {
                "pass"
            } else {
                "FAIL"
            };
            w(format!("| {c} | {lib} | {res} ({} runs) |", passes.len()));
        }
        w(String::new());
        if !skipped.is_empty() {
            let mut keys: Vec<(&String, &Value)> = skipped.iter().collect();
            keys.sort_by(|a, b| a.0.cmp(b.0));
            let s: Vec<String> = keys
                .iter()
                .map(|(k, v)| format!("`{k}` ({})", py_str(v)))
                .collect();
            w(format!("Skipped benchmarks: {}", s.join(", ")));
            w(String::new());
        }
        if let Some(sizes) = sizes {
            w("## Code size".into());
            w(String::new());
            w(sizes.get("note").map_or(String::new(), py_str));
            w(String::new());
            w("| Library | Bytes of linked code and read-only data |".into());
            w("|---|---|".into());
            let mut l: Vec<(&String, i64)> = sizes["libraries"]
                .as_object()
                .into_iter()
                .flatten()
                .map(|(k, v)| (k, v.as_i64().unwrap_or(0)))
                .collect();
            l.sort_by(|a, b| a.0.cmp(b.0));
            l.sort_by_key(|x| -x.1);
            for (lib, n) in l {
                w(format!("| {lib} | {} |", py::int_commas(n)));
            }
            w(String::new());
        }
        w("## Confidence".into());
        w(String::new());
        let n = results
            .iter()
            .map(|r| {
                self.everything[r["id"].as_str().unwrap()]["runs"]
                    .as_i64()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0);
        if n >= 2 {
            let p = 100.0 * (1.0 - 2f64.powi((1 - n) as i32));
            w(format!(
                "Each run is a fresh boot of the same image. With {n} runs the lowest and highest run medians \
                 bound the board's median with probability {}% (distribution-free: each run median falls on \
                 either side with probability one half); `ns_low` and `ns_high` in `{}.json` are that interval, \
                 and `RUNS.md` lists every run. That interval covers run-to-run noise only. {}",
                py::fixed(p, 0),
                py_str(&host["host"]),
                self.meta
                    .get("layout")
                    .map_or(board.layout.to_string(), py_str)
            ));
        }
        w(String::new());
        w("## Reproduce".into());
        w(String::new());
        w("```sh".into());
        for line in board.repro {
            w(line.to_string());
        }
        w("```".into());
        w(String::new());
        if let Some(notes) = self.meta.get("markdown_notes").filter(|v| truthy(v)) {
            w(py_str(notes));
            w(String::new());
        }
        w(format!(
            "Generated by `cargo xtask report` on {}; every benchmark, with each run's statistics, is in `{}-all.json`.",
            crate::today(),
            py_str(&host["host"])
        ));
        write(&out.join("RESULTS.md"), lines.join("\n") + "\n")?;

        let title = board.title.strip_prefix("rind on ").unwrap_or(board.title);
        let n = n.max(0) as usize;
        let mut rl = vec![
            format!("# Every run, {title}"),
            String::new(),
            "Each run's median cycles per call for every report row, the median over runs (published), and the \
             spread, (max - min) / median. Generated by `cargo xtask report`."
                .to_string(),
            String::new(),
            format!(
                "| Benchmark | {} | median | spread |",
                (1..=n).map(|i| format!("run {i}")).collect::<Vec<_>>().join(" | ")
            ),
            format!("|---|{}", "---:|".repeat(n + 2)),
        ];
        let mut ids: Vec<&str> = results.iter().map(|r| r["id"].as_str().unwrap()).collect();
        ids.sort();
        for id in ids {
            let e = &self.everything[id];
            let per = e["per_run"].as_array().unwrap();
            let mut cells: Vec<String> = per
                .iter()
                .map(|x| py::fixed_commas(f(&x["cycles"]["median"]), 0))
                .collect();
            cells.resize(n.max(per.len()), String::new());
            rl.push(format!(
                "| `{id}` | {} | {} | {}% |",
                cells.join(" | "),
                py::fixed_commas(f(&e["cycles"]), 0),
                py::fixed(f(&e["run_spread_pct"]), 2)
            ));
        }
        write(
            &out.join("RUNS.md"),
            "<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->\n".to_string()
                + &rl.join("\n")
                + "\n",
        )
    }
}
