// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! rind-results' tooling: `cargo xtask <command>` (an alias in
//! `.cargo/config.toml`). Every published table, JSON file, chart, OLED page
//! table and screenshot comes from one of these commands; `cargo verify`, a
//! separate crate that shares no code with this one, re-checks them.

mod capture;
mod charts;
mod compare;
mod demo_tsv;
mod host_run;
mod md2png;
mod py;
mod report;
mod run_all;
mod script2cast;
mod sizes;
mod tables;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(about = "rind-results tooling", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Turn captured board runs into rind's report shape, RESULTS.md and RUNS.md
    Report(report::Args),
    /// Compare two sets of board runs of the same benches, row by row
    Compare(compare::Args),
    /// Write the OLED demo's page table from a board's report JSON
    DemoTsv(demo_tsv::Args),
    /// Draw rind-over-fastest SVG charts from report JSON
    Charts(charts::Args),
    /// Attribute linked code and read-only data to the libraries raced
    Sizes(sizes::Args),
    /// Run the flashed ESP32-S3 firmware and record what it prints
    Capture(capture::Args),
    /// Run the flashed nRF52840 firmware and record what it prints
    CaptureNrf(capture::Args),
    /// Flash each ESP32-S3 image in turn and capture its runs
    RunAll(run_all::Args),
    /// Flash each nRF52840 image in turn through the UF2 bootloader and capture its runs
    RunAllNrf(run_all::Args),
    /// Screenshot a Markdown file or one section of it
    Md2png(md2png::Args),
    /// Turn a `script --log-timing` recording into an asciinema cast and a text log
    Script2cast(script2cast::Args),
    /// Run rind's own `admin/bench-report run` on this Linux host
    HostRun(host_run::Args),
    /// rustfmt --check, clippy -D warnings and the tests, for this workspace
    Check,
    /// Refresh tools/bench-tables.json from rind's admin/bench-report
    BenchTables {
        #[arg(long)]
        rind: PathBuf,
    },
}

fn main() -> ExitCode {
    let r = match Cli::parse().cmd {
        Cmd::Report(a) => report::run(a),
        Cmd::Compare(a) => compare::run(a),
        Cmd::DemoTsv(a) => demo_tsv::run(a),
        Cmd::Charts(a) => charts::run(a),
        Cmd::Sizes(a) => sizes::run(a),
        Cmd::Capture(a) => return capture_exit(capture::capture(capture::Board::Esp, a)),
        Cmd::CaptureNrf(a) => return capture_exit(capture::capture(capture::Board::Nrf, a)),
        Cmd::RunAll(a) => run_all::run(capture::Board::Esp, a),
        Cmd::RunAllNrf(a) => run_all::run(capture::Board::Nrf, a),
        Cmd::Md2png(a) => md2png::run(a),
        Cmd::Script2cast(a) => script2cast::run(a),
        Cmd::HostRun(a) => host_run::run(a),
        Cmd::Check => check(),
        Cmd::BenchTables { rind } => tables::refresh(&rind),
    };
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn capture_exit(r: Result<bool, String>) -> ExitCode {
    match r {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn check() -> Result<(), String> {
    for args in [
        &["fmt", "--all", "--check"][..],
        &[
            "clippy",
            "-q",
            "--release",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        &["test", "-q", "--release"],
    ] {
        let st = Command::new(std::env::var("CARGO").unwrap_or("cargo".into()))
            .args(args)
            .current_dir(root())
            .status()
            .map_err(|e| format!("cargo: {e}"))?;
        if !st.success() {
            return Err(format!("cargo {} failed", args.join(" ")));
        }
        println!("cargo {}: ok", args[0]);
    }
    Ok(())
}

/// The repository root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

pub fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Today's local date, `YYYY-MM-DD`.
pub fn today() -> String {
    Command::new("date")
        .arg("+%F")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Now in UTC as Python's `datetime.now(timezone.utc).isoformat(timespec="seconds")`.
pub fn utc_now() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs()) as i64;
    let (days, rem) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    // Howard Hinnant's civil_from_days.
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}+00:00",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}
