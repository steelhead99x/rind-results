// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask bench`: one entry point for a published set.
//!
//! `bench --board <id> --rind <path> --commit <PIN> --out <dir> [--runs 3]`
//! runs a host, an Android capture, or a bare-metal image set. It calls
//! rind's `admin/bench-report` for host runs and does not edit that file.
//! Relaxations are written to `relaxations.json`.

use crate::layout;
use crate::upstream_run::{self, Relax};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {
    /// Host id, or an explicit Android or bare-metal board.
    ///
    /// A host id is any filesystem-safe results directory name (lowercase
    /// letters, digits, `.`, `_` and `-`). The recommended id is a slug of the
    /// CPU model, for example `intel-core-ultra-9-275hx`. Omit `--board` to
    /// use that slug for this machine.
    ///
    /// Android: `pixel9pro`. Bare metal: `esp32s3`, `nrf52840`, `efr32mg24`,
    /// `rp2040`.
    #[arg(long)]
    board: Option<String>,
    /// rind checkout that was measured. It must be clean and at `--commit`.
    #[arg(long)]
    rind: PathBuf,
    /// upstream rind SHA (full or a unique prefix). Default: the comparable-set pin.
    #[arg(long, default_value = crate::layout::COMPARABLE_PIN)]
    commit: String,
    /// results directory. Default: `results/<board>/<commit>/`
    #[arg(long)]
    out: Option<PathBuf>,
    #[arg(long, default_value_t = 3)]
    runs: u32,
    /// label suffix: `results/<board>/<commit>-<label>/`
    #[arg(long)]
    label: Option<String>,
    /// cores for `taskset -c` (host runs). The Orin picks the least busy of cpu1..cpu5.
    #[arg(long)]
    cpus: Option<String>,
    /// in-memory `BUSY_SHARE`. Upstream is 0.1. Recorded, not written into rind.
    #[arg(long)]
    busy_share: Option<String>,
    /// accept the host rustc instead of the Nix pin
    #[arg(long)]
    accept_toolchain: bool,
    /// record a busy host and continue
    #[arg(long)]
    record_busy: bool,
    /// do not run admin/build-cost
    #[arg(long)]
    skip_build_cost: bool,
    /// do not apply the Orin's default relaxations
    #[arg(long)]
    strict: bool,
    /// seconds to add to the wall clock for bench binaries (Orin libfaketime)
    #[arg(long)]
    faketime_offset: Option<String>,
    /// Android capture directory (`getprop.txt`, `criterion/`, ...)
    #[arg(long)]
    capture: Option<PathBuf>,
    /// bare-metal image directory
    #[arg(long)]
    images: Option<PathBuf>,
    /// serial port for an ESP32-S3 flash
    #[arg(long)]
    port: Option<String>,
}

enum Kind {
    Host,
    Android,
    Bare,
}

fn kind(board: &str) -> Result<Kind, String> {
    match board {
        "pixel9pro" => Ok(Kind::Android),
        "esp32s3" | "nrf52840" | "efr32mg24" | "rp2040" => Ok(Kind::Bare),
        other if layout::is_filesystem_safe_host(other) => Ok(Kind::Host),
        other => Err(format!(
            "board {other:?} is not a filesystem-safe host id. \
             Use a lowercase name of letters, digits, '.', '_' and '-' \
             (at most 81 characters, starting with a letter or digit). \
             The recommended id is a slug of the CPU model, for example \
             `intel-core-ultra-9-275hx`. \
             Android: pixel9pro. Bare metal: esp32s3, nrf52840, efr32mg24, rp2040."
        )),
    }
}

fn board_name(a: &Args) -> &str {
    a.board.as_deref().unwrap_or("")
}

/// `--board` when it is set, otherwise a slug of this machine's CPU.
fn resolve_board(board: Option<&str>) -> Result<String, String> {
    let chosen = board
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(recommended_host_id);
    kind(&chosen)?;
    Ok(chosen)
}

/// The host id to publish for this machine: a filesystem-safe slug of the CPU.
fn recommended_host_id() -> String {
    cpu_slug(&cpu_model())
}

fn command_stdout(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn cpu_model_darwin() -> Option<String> {
    if std::env::consts::OS != "macos" {
        return None;
    }
    let brand = command_stdout("sysctl", &["-n", "machdep.cpu.brand_string"]);
    if brand.is_empty() { None } else { Some(brand) }
}

/// The brand string from Linux `/proc/cpuinfo`, ignoring the numeric `model`.
fn cpu_model_from_cpuinfo(text: &str) -> Option<String> {
    for line in text.lines() {
        if line.starts_with("model name") || line.starts_with("Model") {
            let model = line
                .split_once(':')
                .map(|(_, v)| v.trim())
                .unwrap_or("")
                .to_string();
            if !model.is_empty() {
                return Some(model);
            }
        }
    }
    None
}

fn cpu_model_from_lscpu(text: &str) -> Option<String> {
    for line in text.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if key.trim().eq_ignore_ascii_case("model name") {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

/// The raw CPU brand string: macOS `sysctl`, Linux `/proc/cpuinfo`, `lscpu`,
/// then `uname -m`.
fn cpu_model() -> String {
    if let Some(brand) = cpu_model_darwin() {
        return brand;
    }
    if let Ok(text) = std::fs::read_to_string("/proc/cpuinfo") {
        if let Some(model) = cpu_model_from_cpuinfo(&text) {
            return model;
        }
    }
    if let Some(model) = cpu_model_from_lscpu(&command_stdout("lscpu", &[])) {
        return model;
    }
    let machine = command_stdout("uname", &["-m"]);
    if machine.is_empty() {
        "unknown".into()
    } else {
        machine
    }
}

const SLUG_SKIP: &[&str] = &["cpu", "processor", "processors"];

fn replace_all(pattern: &str, text: &str) -> String {
    Regex::new(pattern)
        .expect("slug pattern")
        .replace_all(text, " ")
        .into_owned()
}

/// A filesystem-safe host name for a CPU brand string.
///
/// `Intel(R) Core(TM) Ultra 9 275HX` becomes `intel-core-ultra-9-275hx`,
/// `AMD Ryzen 9 7950X 16-Core Processor` becomes `amd-ryzen-9-7950x`, and
/// `Apple M3 Max` becomes `apple-m3-max`.
fn cpu_slug(model: &str) -> String {
    let mut text = model.to_ascii_lowercase();
    text = replace_all(r"\((?:r|tm|c)\)|[®™]", &text);
    text = replace_all(r"@?\s*\d+(?:\.\d+)?\s*ghz\b", &text);
    text = replace_all(r"\b\d+\s*-?\s*cores?\b", &text);
    text = replace_all(r"[^a-z0-9]+", &text);
    let slug = text
        .split_whitespace()
        .filter(|part| !SLUG_SKIP.contains(part))
        .collect::<Vec<_>>()
        .join("-");
    let slug = if slug.len() > 80 {
        let cut = &slug[..80];
        cut.rsplit_once('-')
            .map(|(head, _)| head.to_string())
            .filter(|head| !head.is_empty())
            .unwrap_or_else(|| cut.to_string())
    } else {
        slug
    };
    if layout::is_filesystem_safe_host(&slug) {
        slug
    } else {
        "unknown".into()
    }
}

fn relax_for(a: &Args) -> Relax {
    let orin = board_name(a) == "jetson-orin-nano" && !a.strict;
    Relax {
        busy_share: a.busy_share.clone(),
        accept_toolchain: a.accept_toolchain || orin,
        record_busy: a.record_busy || orin,
        skip_build_cost: a.skip_build_cost || orin,
        free_target: false,
    }
}

pub fn run(mut a: Args) -> Result<(), String> {
    a.board = Some(resolve_board(a.board.as_deref())?);
    let head = upstream_run::require_clean_commit(&a.rind, &a.commit)?;
    let out = match &a.out {
        Some(p) => p.clone(),
        None => layout::publish_dir(&crate::root(), board_name(&a), &head, a.label.as_deref())?,
    };
    layout::refuse_history_out(&out)?;
    std::fs::create_dir_all(out.join("raw")).map_err(|e| e.to_string())?;
    let relax = relax_for(&a);
    upstream_run::write_relaxations(
        &out.join("relaxations.json"),
        &a.rind.join("admin/bench-report"),
        &relax,
    )?;
    write_meta(&out, board_name(&a), &head, a.runs, &relax)?;
    match kind(board_name(&a))? {
        Kind::Host => host_runs(&a, &out, &head, &relax),
        Kind::Android => android(&a, &out, &head),
        Kind::Bare => bare(&a, &out),
    }
}

fn write_meta(
    out: &Path,
    board: &str,
    upstream: &str,
    runs: u32,
    relax: &Relax,
) -> Result<(), String> {
    let body = serde_json::json!({
        "schema": 1,
        "host": board,
        "upstream_commit": upstream,
        "fork_commit": serde_json::Value::Null,
        "patches": [],
        "runs": runs,
        "criterion_args": ["--noplot", "--warm-up-time", "1", "--measurement-time", "3"],
        "spread_is": "runs",
        "metric_units": "ns",
        "relaxed": relax.any(),
    });
    std::fs::write(
        out.join("meta.json"),
        serde_json::to_string_pretty(&body).unwrap() + "\n",
    )
    .map_err(|e| e.to_string())
}

fn host_runs(a: &Args, out: &Path, head: &str, relax: &Relax) -> Result<(), String> {
    let board = board_name(a).to_string();
    let cpus = match &a.cpus {
        Some(c) => c.clone(),
        None if board == "jetson-orin-nano" => upstream_run::sample_least_busy_core()?.to_string(),
        None => "2,3".into(),
    };
    let jobs = std::thread::available_parallelism()
        .map(|n| n.get().to_string())
        .unwrap_or_else(|_| "1".into());
    let target = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| out.join("target").display().to_string());
    let mut env = vec![("IN_RIND_DEV_SHELL".to_string(), "1".to_string())];
    if let Ok(home) = std::env::var("HOME") {
        let orin = PathBuf::from(home).join("rind-bench-orin");
        if board == "jetson-orin-nano" && orin.is_dir() {
            env.extend(upstream_run::orin_env(&orin));
            if let Some(offset) = &a.faketime_offset {
                let runner = out.join("faketime-runner.sh");
                std::fs::write(&runner, upstream_run::faketime_runner(&orin, offset))
                    .map_err(|e| e.to_string())?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut p = std::fs::metadata(&runner)
                        .map_err(|e| e.to_string())?
                        .permissions();
                    p.set_mode(0o755);
                    std::fs::set_permissions(&runner, p).map_err(|e| e.to_string())?;
                }
                env.push((
                    "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER".into(),
                    runner.display().to_string(),
                ));
            }
        }
    }
    let env_ref: Vec<(&str, &str)> = env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let log = out.join("run.log");
    for n in 1..=a.runs {
        let dest = out.join("raw").join(format!("run-{n}.json"));
        let args = [
            "run",
            "--host",
            &board,
            "--commit",
            head,
            "--jobs",
            &jobs,
            "--target-dir",
            &target,
            "--out",
            &dest.display().to_string(),
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
        let mut line = format!("run {n} cpus={cpus}\n");
        match upstream_run::run_bench_report(&a.rind, &args, relax, &env_ref, Some(&cpus), None) {
            Ok(()) => line.push_str("ok\n"),
            Err(e) => {
                line.push_str(&e);
                line.push('\n');
                append(&log, &line)?;
                return Err(e);
            }
        }
        append(&log, &line)?;
    }
    Ok(())
}

fn append(path: &Path, text: &str) -> Result<(), String> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(text.as_bytes()).map_err(|e| e.to_string())
}

fn android(a: &Args, out: &Path, head: &str) -> Result<(), String> {
    let board = board_name(a);
    let Some(capture) = &a.capture else {
        return Err(
            "pixel9pro needs --capture <dir> of an adb pull (getprop.txt, stat-before, \
             stat-after, criterion/), or `cargo xtask pixel-run --serial <id>` to collect one. \
             Parsing matches admin/bench-report run()."
                .into(),
        );
    };
    let args = android_convert::ConvertArgs {
        rind: a.rind.clone(),
        capture: capture.clone(),
        out: out.join(format!("{board}.json")),
        host: board.to_string(),
        commit: head.to_string(),
        rustc: "rustc 1.98.1 (48a229cea 2026-09-01)".into(),
        cargo: "cargo 1.98.1 (797e8a9bc 2026-08-05)".into(),
        target: "aarch64-linux-android".into(),
        cpu_model: "Google Tensor G4 (Pixel 9 Pro)".into(),
        logical_cpus: 8,
        openssl_linked: String::new(),
    };
    let converted = android_convert::convert(&args)?;
    android_convert::write_out(&args, &converted)?;
    print!("{}", converted.stdout);
    Ok(())
}

fn bare(a: &Args, out: &Path) -> Result<(), String> {
    let board = board_name(a);
    let Some(images) = &a.images else {
        return Err(format!(
            "{board} needs --images <dir> of flashed firmware. esp32s3 also needs --port."
        ));
    };
    let which = match board {
        "esp32s3" => crate::capture::Board::Esp,
        "nrf52840" | "efr32mg24" | "rp2040" => crate::capture::Board::Nrf,
        _ => return Err("not a bare-metal board".into()),
    };
    if board == "esp32s3" && a.port.is_none() {
        return Err("esp32s3 needs --port (or PORT)".into());
    }
    // efr32 and rp2040 do not share the nRF UF2 runner. Name that instead of flashing the wrong board.
    if board != "esp32s3" && board != "nrf52840" {
        return Err(format!(
            "{board} images are in {}; flash them with the board's runner, then `cargo xtask report --dir {}`",
            images.display(),
            out.display()
        ));
    }
    crate::run_all::run_with(which, images, &out.join("raw"), a.runs, a.port.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orin_defaults_record_relaxations_without_a_flag() {
        let a = Args {
            board: Some("jetson-orin-nano".into()),
            rind: PathBuf::new(),
            commit: String::new(),
            out: None,
            runs: 3,
            label: None,
            cpus: None,
            busy_share: None,
            accept_toolchain: false,
            record_busy: false,
            skip_build_cost: false,
            strict: false,
            faketime_offset: None,
            capture: None,
            images: None,
            port: None,
        };
        let r = relax_for(&a);
        assert!(r.accept_toolchain && r.record_busy && r.skip_build_cost);
        let strict = Args { strict: true, ..a };
        let r = relax_for(&strict);
        assert!(!r.accept_toolchain && !r.record_busy && !r.skip_build_cost);
    }

    #[test]
    fn any_filesystem_safe_host_is_a_host_and_boards_stay_boards() {
        for id in [
            "xeon-vm",
            "m1pro",
            "pi5",
            "msi-001",
            "jetson-orin-nano",
            "intel-core-ultra-9-275hx",
            "amd-ryzen-9-7950x",
            "my_host",
        ] {
            assert!(matches!(kind(id), Ok(Kind::Host)), "{id}");
        }
        assert!(matches!(kind("pixel9pro"), Ok(Kind::Android)));
        assert!(matches!(kind("esp32s3"), Ok(Kind::Bare)));
        assert!(matches!(kind("nrf52840"), Ok(Kind::Bare)));
        assert!(matches!(kind("efr32mg24"), Ok(Kind::Bare)));
        assert!(matches!(kind("rp2040"), Ok(Kind::Bare)));
        for id in ["", "Jetson", "foo/bar", "../x", "-bad", "has space"] {
            let Err(err) = kind(id) else {
                panic!("{id} was accepted");
            };
            assert!(err.contains("filesystem-safe"), "{id}: {err}");
            assert!(!err.contains("unknown board"), "{id}: {err}");
        }
    }

    #[test]
    fn recommended_host_id_is_a_slug_of_the_cpu() {
        assert_eq!(
            cpu_slug("Intel(R) Core(TM) Ultra 9 275HX"),
            "intel-core-ultra-9-275hx"
        );
        assert_eq!(
            cpu_slug("AMD Ryzen 9 7950X 16-Core Processor"),
            "amd-ryzen-9-7950x"
        );
        assert_eq!(cpu_slug("Apple M3 Max"), "apple-m3-max");
        assert_eq!(cpu_slug("Intel(R) Xeon(R) CPU @ 2.50GHz"), "intel-xeon");
        let info =
            "processor\t: 0\nmodel\t\t: 207\nmodel name\t: Intel(R) Core(TM) Ultra 9 275HX\n";
        assert_eq!(
            cpu_model_from_cpuinfo(info).as_deref(),
            Some("Intel(R) Core(TM) Ultra 9 275HX")
        );
        let lscpu = "CPU(s): 24\nModel name: AMD Ryzen 9 7950X 16-Core Processor\n";
        assert_eq!(
            cpu_model_from_lscpu(lscpu).as_deref(),
            Some("AMD Ryzen 9 7950X 16-Core Processor")
        );
        let id = recommended_host_id();
        assert_eq!(id, cpu_slug(&cpu_model()));
        assert!(matches!(kind(&id), Ok(Kind::Host)), "{id}");
        assert_eq!(resolve_board(None).unwrap(), id);
        assert_eq!(resolve_board(Some("  ")).unwrap(), id);
        assert_eq!(
            resolve_board(Some(" intel-core-ultra-9-275hx ")).unwrap(),
            "intel-core-ultra-9-275hx"
        );
        assert!(resolve_board(Some("../x")).is_err());
    }
}
