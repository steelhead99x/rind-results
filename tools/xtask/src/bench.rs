// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask bench`: one entry point for a published set.
//!
//! `bench --board <id> --rind <path> --commit <PIN> --out <dir> [--runs 3]`
//! runs a host, an Android capture, or a bare-metal image set. It calls
//! rind's `admin/bench-report` for host runs and does not edit that file.
//! Relaxations are written to `relaxations.json`.

use crate::layout;
use crate::upstream_run::{self, Relax};
use std::path::{Path, PathBuf};

#[derive(clap::Args)]
pub struct Args {
    /// `esp32s3`, `nrf52840`, `efr32mg24`, `rp2040`, `jetson-orin-nano`,
    /// `pi5`, `m1pro`, `xeon-vm`, `msi-001`, or `pixel9pro`
    #[arg(long)]
    board: String,
    /// rind checkout that was measured. It must be clean and at `--commit`.
    #[arg(long)]
    rind: PathBuf,
    /// upstream rind SHA (full or a unique prefix)
    #[arg(long)]
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
        "xeon-vm" | "m1pro" | "pi5" | "msi-001" | "jetson-orin-nano" => Ok(Kind::Host),
        "pixel9pro" => Ok(Kind::Android),
        "esp32s3" | "nrf52840" | "efr32mg24" | "rp2040" => Ok(Kind::Bare),
        other => Err(format!(
            "unknown board {other}. Host: jetson-orin-nano, pi5, m1pro, xeon-vm, msi-001. \
             Android: pixel9pro. Bare metal: esp32s3, nrf52840, efr32mg24, rp2040."
        )),
    }
}

fn relax_for(a: &Args) -> Relax {
    let orin = a.board == "jetson-orin-nano" && !a.strict;
    Relax {
        busy_share: a.busy_share.clone(),
        accept_toolchain: a.accept_toolchain || orin,
        record_busy: a.record_busy || orin,
        skip_build_cost: a.skip_build_cost || orin,
        free_target: false,
    }
}

pub fn run(a: Args) -> Result<(), String> {
    let head = upstream_run::require_clean_commit(&a.rind, &a.commit)?;
    let out = match &a.out {
        Some(p) => p.clone(),
        None => layout::publish_dir(&crate::root(), &a.board, &head, a.label.as_deref())?,
    };
    std::fs::create_dir_all(out.join("raw")).map_err(|e| e.to_string())?;
    let relax = relax_for(&a);
    upstream_run::write_relaxations(
        &out.join("relaxations.json"),
        &a.rind.join("admin/bench-report"),
        &relax,
    )?;
    write_meta(&out, &a.board, &head, a.runs, &relax)?;
    match kind(&a.board)? {
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
    let cpus = match &a.cpus {
        Some(c) => c.clone(),
        None if a.board == "jetson-orin-nano" => {
            upstream_run::sample_least_busy_core()?.to_string()
        }
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
        if a.board == "jetson-orin-nano" && orin.is_dir() {
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
            &a.board,
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
        match upstream_run::run_bench_report(&a.rind, &args, relax, &env_ref, Some(&cpus)) {
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
    let Some(capture) = &a.capture else {
        return Err(
            "pixel9pro needs --capture <dir> (getprop.txt, stat-before, stat-after, criterion/). \
             The adb push and pull stay in scripts/pixel-run.sh; this command parses the capture \
             the way admin/bench-report run() does."
                .into(),
        );
    };
    let args = android_convert::ConvertArgs {
        rind: a.rind.clone(),
        capture: capture.clone(),
        out: out.join(format!("{}.json", a.board)),
        host: a.board.clone(),
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
    let Some(images) = &a.images else {
        return Err(format!(
            "{} needs --images <dir> of flashed firmware. esp32s3 also needs --port.",
            a.board
        ));
    };
    let board = match a.board.as_str() {
        "esp32s3" => crate::capture::Board::Esp,
        "nrf52840" | "efr32mg24" | "rp2040" => crate::capture::Board::Nrf,
        _ => return Err("not a bare-metal board".into()),
    };
    if a.board == "esp32s3" && a.port.is_none() {
        return Err("esp32s3 needs --port (or PORT)".into());
    }
    // efr32 and rp2040 do not share the nRF UF2 runner. Name that instead of flashing the wrong board.
    if a.board != "esp32s3" && a.board != "nrf52840" {
        return Err(format!(
            "{} images are in {}; flash them with the board's runner, then `cargo xtask report --dir {}`",
            a.board,
            images.display(),
            out.display()
        ));
    }
    crate::run_all::run_with(board, images, &out.join("raw"), a.runs, a.port.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orin_defaults_record_relaxations_without_a_flag() {
        let a = Args {
            board: "jetson-orin-nano".into(),
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
}
