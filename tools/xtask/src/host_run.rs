// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask host-run`: runs rind's own `admin/bench-report run` on this
//! Linux host, outside Nix: rind's pinned stable toolchain through rustup
//! (RUSTUP_TOOLCHAIN), with the dev-shell check satisfied by hand, pinned with
//! taskset to one physical core (both its SMT threads), and the host's power
//! settings recorded beside the JSON in `<out>.facts.txt`.
//!
//! bench-report itself is rind's Python; this only prepares its environment.

use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {
    rind: PathBuf,
    host: String,
    out: PathBuf,
    #[arg(default_value = "2,3")]
    cpus: String,
    /// relax rind's busy-host refusal to this share of busy CPUs (rind: 0.1);
    /// the edit is reverted afterwards and recorded in the facts file
    #[arg(long, env = "BUSY_SHARE")]
    busy_share: Option<String>,
}

fn sh(cmd: &str, args: &[&str], env: &[(&str, &str)]) -> String {
    let mut c = Command::new(cmd);
    c.args(args);
    for (k, v) in env {
        c.env(k, v);
    }
    c.output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn sys(path: &str) -> String {
    std::fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or("?".into())
}

pub fn run(a: Args) -> Result<(), String> {
    let rind = std::fs::canonicalize(&a.rind).map_err(|e| e.to_string())?;
    let out = std::path::absolute(&a.out).map_err(|e| e.to_string())?;
    let flake = std::fs::read_to_string(rind.join("flake.nix")).map_err(|e| e.to_string())?;
    let tc = Regex::new(r#"rust-bin\.stable\."([0-9.]+)""#)
        .unwrap()
        .captures(&flake)
        .map(|c| c[1].to_string())
        .ok_or("flake.nix names no pinned stable toolchain")?;
    let st = Command::new("rustup")
        .args(["toolchain", "install", &tc, "--profile", "minimal"])
        .stdout(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("rustup: {e}"))?;
    if !st.success() {
        return Err("rustup toolchain install failed".into());
    }
    std::fs::create_dir_all(out.parent().unwrap_or(Path::new("."))).map_err(|e| e.to_string())?;
    let facts = out.with_extension("facts.txt");
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let model = cpuinfo
        .lines()
        .find(|l| l.starts_with("model name"))
        .unwrap_or("")
        .to_string();
    let c0 = "/sys/devices/system/cpu/cpu0/cpufreq";
    let mut f = vec![
        format!("date: {}", sh("date", &["-u", "+%FT%TZ"], &[])),
        format!("kernel: {}", sh("uname", &["-srvm"], &[])),
        model,
        format!("governor: {}", sys(&format!("{c0}/scaling_governor"))),
        format!(
            "cpufreq boost: {}",
            sys("/sys/devices/system/cpu/cpufreq/boost")
        ),
        format!(
            "energy_performance_preference: {}",
            sys(&format!("{c0}/energy_performance_preference"))
        ),
        format!("scaling driver: {}", sys(&format!("{c0}/scaling_driver"))),
        format!("pinned to CPUs: {}", a.cpus),
        format!("load: {}", sys("/proc/loadavg")),
        format!(
            "rustc: {}",
            sh("rustc", &["-V"], &[("RUSTUP_TOOLCHAIN", &tc)])
        ),
    ];
    let commit = sh(
        "git",
        &["-C", &rind.display().to_string(), "rev-parse", "HEAD"],
        &[],
    );
    // rind refuses a host where other work keeps more than 10% of the CPUs
    // busy; --busy-share relaxes that in memory. The checkout's bench-report
    // is not edited. The relaxation is recorded next to the JSON.
    let relax = crate::upstream_run::Relax {
        busy_share: a.busy_share.clone(),
        ..crate::upstream_run::Relax::default()
    };
    if let Some(share) = &a.busy_share {
        f.push(format!(
            "BUSY_SHARE relaxed to {share} (rind: 0.1); file not edited"
        ));
    }
    std::fs::write(&facts, f.join("\n") + "\n").map_err(|e| e.to_string())?;
    let br = rind.join("admin/bench-report");
    crate::upstream_run::write_relaxations(&out.with_extension("relaxations.json"), &br, &relax)?;
    let jobs = std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .to_string();
    let target = std::env::var("CARGO_TARGET_DIR").unwrap_or("target/bench-report".into());
    let args = [
        "run",
        "--host",
        &a.host,
        "--commit",
        &commit,
        "--jobs",
        &jobs,
        "--target-dir",
        &target,
        "--out",
        &out.display().to_string(),
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();
    crate::upstream_run::run_bench_report(
        &rind,
        &args,
        &relax,
        &[("IN_RIND_DEV_SHELL", "1"), ("RUSTUP_TOOLCHAIN", &tc)],
        Some(&a.cpus),
        None,
    )
}
