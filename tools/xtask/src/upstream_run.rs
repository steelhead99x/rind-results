// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Run rind's `admin/bench-report run` without writing that file.
//!
//! Relaxations (a busier host, the host's own rustc, skipping build-cost)
//! are applied in memory by a loader and recorded as JSON. The checkout's
//! `admin/bench-report` is only read.

use serde_json::{Map, Value, json};
use std::path::Path;
use std::process::Command;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Relax {
    /// `BUSY_SHARE` in the measured process. Upstream's value is 0.1.
    pub busy_share: Option<String>,
    /// Accept the rustc on `PATH` instead of refusing a non-pinned one.
    pub accept_toolchain: bool,
    /// Record busy CPUs and continue, instead of refusing a busy host.
    pub record_busy: bool,
    /// Do not run `admin/build-cost`.
    pub skip_build_cost: bool,
    /// Delete `CARGO_TARGET_DIR` before build-cost, as host-run.py did.
    pub free_target: bool,
}

impl Relax {
    pub fn any(&self) -> bool {
        self.busy_share.is_some()
            || self.accept_toolchain
            || self.record_busy
            || self.skip_build_cost
            || self.free_target
    }
}

/// The loader. It is not a file in the repo: upstream's script stays the
/// program that selects benchmarks, and this only binds the relaxations.
const LOADER: &str = r#"
import importlib.machinery, importlib.util, os, shutil, sys
path = sys.argv[1]
loader = importlib.machinery.SourceFileLoader("bench_report", path)
spec = importlib.util.spec_from_loader("bench_report", loader)
br = importlib.util.module_from_spec(spec)
loader.exec_module(br)
share = os.environ.get("RIND_BUSY_SHARE")
if share:
    br.BUSY_SHARE = float(share)
if os.environ.get("RIND_ACCEPT_TOOLCHAIN") == "1":
    def toolchain(env):
        v = br.output(["rustc", "-vV"], env)
        return ["cargo"], {"rustc": v.splitlines()[0], "cargo": br.output(["cargo", "-V"], env)}, "openssl"
    br.toolchain = toolchain
if os.environ.get("RIND_RECORD_BUSY") == "1":
    def check_idle(when):
        busy = br.busy_cpus()
        n = br.cpu_count() if hasattr(br, "cpu_count") else 1
        limit = br.busy_limit() if hasattr(br, "busy_limit") else 0.0
        print(f"host-run: {busy:.1f} of {n} CPUs busy {when} (harness limit {limit:.1f}); continuing", file=sys.stderr, flush=True)
        return round(float(busy), 2)
    br.check_idle = check_idle
orig = getattr(br, "build_cost", None)
def build_cost(env, jobs, refused):
    if os.environ.get("RIND_FREE_TARGET") == "1":
        shutil.rmtree(env["CARGO_TARGET_DIR"], ignore_errors=True)
    if os.environ.get("RIND_SKIP_BUILD_COST") == "1":
        return {"status": "failed", "started": "", "reason": "skipped by host-run (HOSTRUN_SKIP_BUILD_COST)"}
    return orig(env, jobs, refused)
if orig is not None and (os.environ.get("RIND_SKIP_BUILD_COST") == "1" or os.environ.get("RIND_FREE_TARGET") == "1"):
    br.build_cost = build_cost
sys.argv = ["bench-report"] + sys.argv[2:]
br.main()
"#;

pub fn relaxations_value(upstream: &Path, relax: &Relax) -> Value {
    let mut applied = Vec::new();
    if let Some(share) = &relax.busy_share {
        applied.push(json!({
            "name": "BUSY_SHARE",
            "upstream": 0.1,
            "applied": share,
        }));
    }
    if relax.accept_toolchain {
        applied.push(json!({
            "name": "toolchain",
            "upstream": "pinned rustc only",
            "applied": "host rustc accepted",
        }));
    }
    if relax.record_busy {
        applied.push(json!({
            "name": "check_idle",
            "upstream": "refuse when busy",
            "applied": "record and continue",
        }));
    }
    if relax.skip_build_cost {
        applied.push(json!({
            "name": "build_cost",
            "upstream": "admin/build-cost",
            "applied": "skipped",
        }));
    }
    if relax.free_target {
        applied.push(json!({
            "name": "build_cost",
            "upstream": "keep the target dir",
            "applied": "target dir removed before build-cost",
        }));
    }
    let mut obj = Map::new();
    obj.insert("schema".into(), json!(1));
    obj.insert("mutated_upstream".into(), json!(false));
    obj.insert("upstream".into(), json!(upstream.display().to_string()));
    obj.insert("relaxations".into(), Value::Array(applied));
    Value::Object(obj)
}

pub fn write_relaxations(path: &Path, upstream: &Path, relax: &Relax) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    let body = crate::py::dumps(&relaxations_value(upstream, relax), Some(1)) + "\n";
    std::fs::write(path, body).map_err(|e| format!("{}: {e}", path.display()))
}

/// `args` is what follows `admin/bench-report`, for example
/// `["run", "--host", "pi5", ...]`. The script file is not modified.
pub fn run_bench_report(
    rind: &Path,
    args: &[String],
    relax: &Relax,
    env: &[(&str, &str)],
    cpus: Option<&str>,
    log: Option<&Path>,
) -> Result<(), String> {
    let script = rind.join("admin/bench-report");
    if !script.is_file() {
        return Err(format!("no {}", script.display()));
    }
    let before = std::fs::read(&script).map_err(|e| format!("{}: {e}", script.display()))?;
    let mut cmd = match cpus {
        Some(cpus) => {
            let mut c = Command::new("taskset");
            c.args(["-c", cpus, "python3"]);
            c
        }
        None => Command::new("python3"),
    };
    if relax.any() {
        cmd.arg("-c").arg(LOADER).arg(&script);
        if let Some(share) = &relax.busy_share {
            cmd.env("RIND_BUSY_SHARE", share);
        }
        if relax.accept_toolchain {
            cmd.env("RIND_ACCEPT_TOOLCHAIN", "1");
        }
        if relax.record_busy {
            cmd.env("RIND_RECORD_BUSY", "1");
        }
        if relax.skip_build_cost {
            cmd.env("RIND_SKIP_BUILD_COST", "1");
        }
        if relax.free_target {
            cmd.env("RIND_FREE_TARGET", "1");
        }
    } else {
        cmd.arg(&script);
    }
    cmd.args(args);
    cmd.current_dir(rind);
    for (k, v) in env {
        cmd.env(k, v);
    }
    if let Some(log) = log {
        if let Some(parent) = log.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        let file = std::fs::File::create(log).map_err(|e| format!("{}: {e}", log.display()))?;
        cmd.stdout(std::process::Stdio::from(
            file.try_clone().map_err(|e| e.to_string())?,
        ));
        cmd.stderr(std::process::Stdio::from(file));
    }
    let status = cmd.status().map_err(|e| format!("python3: {e}"))?;
    let after = std::fs::read(&script).map_err(|e| format!("{}: {e}", script.display()))?;
    if before != after {
        return Err(format!(
            "{} was modified; the bench loader must not write it",
            script.display()
        ));
    }
    if status.success() {
        Ok(())
    } else {
        Err(format!("bench-report {}: {status}", args.join(" ")))
    }
}

/// `git` HEAD must be `commit` (a full sha or a unique prefix) and the tree clean.
pub fn require_clean_commit(rind: &Path, commit: &str) -> Result<String, String> {
    let head = crate::git(rind, &["rev-parse", "HEAD"])?;
    let status = crate::git(rind, &["status", "--porcelain"])?;
    if !status.is_empty() {
        return Err(format!(
            "{} is dirty; a published run measures a committed tree",
            rind.display()
        ));
    }
    let want = crate::git(
        rind,
        &["rev-parse", "--verify", &format!("{commit}^{{commit}}")],
    )?;
    if want != head {
        return Err(format!("{} HEAD is {head}, not {commit}", rind.display()));
    }
    Ok(head)
}

/// Least-busy core among cpu1..cpu5, from two `/proc/stat` snapshots.
/// `cpu0` is skipped: it takes most of the interrupts on the Orin.
pub fn least_busy_core(before: &str, after: &str) -> Option<u32> {
    let a = stat_map(before);
    let b = stat_map(after);
    let mut best: Option<(u32, f64)> = None;
    for cpu in 1..=5 {
        let name = format!("cpu{cpu}");
        let (Some(a), Some(b)) = (a.get(&name), b.get(&name)) else {
            continue;
        };
        if a.len() < 8 || b.len() < 8 {
            continue;
        }
        let total0: u64 = a.iter().take(8).sum();
        let total1: u64 = b.iter().take(8).sum();
        let idle0 = a[3] + a[4];
        let idle1 = b[3] + b[4];
        let d = total1.saturating_sub(total0);
        let busy = if d == 0 {
            0.0
        } else {
            (d.saturating_sub(idle1.saturating_sub(idle0))) as f64 / d as f64
        };
        if best.is_none_or(|(_, b)| busy < b) {
            best = Some((cpu, busy));
        }
    }
    best.map(|(c, _)| c)
}

fn stat_map(text: &str) -> std::collections::BTreeMap<String, Vec<u64>> {
    let mut out = std::collections::BTreeMap::new();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let Some(name) = parts.next() else { continue };
        if !name.starts_with("cpu") || name == "cpu" {
            continue;
        }
        let nums: Vec<u64> = parts.filter_map(|p| p.parse().ok()).collect();
        out.insert(name.to_string(), nums);
    }
    out
}

pub fn faketime_runner(home: &Path, offset_secs: &str) -> String {
    format!(
        "#!/bin/sh\nexec env LD_PRELOAD=\"{lib}\" FAKETIME=\"+{offset}s\" FAKETIME_DONT_FAKE_MONOTONIC=1 FAKETIME_DONT_RESET=1 \"$@\"\n",
        lib = home
            .join("faketime/usr/lib/aarch64-linux-gnu/faketime/libfaketimeMT.so.1")
            .display(),
        offset = offset_secs,
    )
}

/// The Orin staged toolchain (`cargo xtask orin-setup`): nothing is installed system-wide.
pub fn orin_env(home: &Path) -> Vec<(String, String)> {
    vec![
        (
            "PATH".into(),
            format!(
                "{}/rust/bin:{}",
                home.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        ),
        (
            "CARGO_HOME".into(),
            home.join("cargo-home").display().to_string(),
        ),
        ("CARGO_NET_OFFLINE".into(), "true".into()),
        (
            "OPENSSL_INCLUDE_DIR".into(),
            home.join("sysroot/usr/include").display().to_string(),
        ),
        (
            "OPENSSL_LIB_DIR".into(),
            home.join("sysroot/lib").display().to_string(),
        ),
        ("OPENSSL_NO_PKG_CONFIG".into(), "1".into()),
        ("IN_RIND_DEV_SHELL".into(), "1".into()),
        (
            "CARGO_TARGET_DIR".into(),
            home.join("target").display().to_string(),
        ),
    ]
}

pub fn sample_least_busy_core() -> Result<u32, String> {
    let before = std::fs::read_to_string("/proc/stat").map_err(|e| format!("/proc/stat: {e}"))?;
    std::thread::sleep(std::time::Duration::from_secs(10));
    let after = std::fs::read_to_string("/proc/stat").map_err(|e| format!("/proc/stat: {e}"))?;
    least_busy_core(&before, &after).ok_or_else(|| "cpu1..cpu5 not in /proc/stat".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_relaxes_in_memory_and_leaves_the_file() {
        let dir = std::env::temp_dir().join(format!("rind-loader-{}", std::process::id()));
        let rind = dir.join("rind");
        std::fs::create_dir_all(rind.join("admin")).unwrap();
        let script = "\
BUSY_SHARE = 0.1
def output(cmd, env=None):
    return 'rustc 1.0.0\\nrelease: 1.0.0'
def busy_cpus():
    return 1.25
def cpu_count():
    return 8
def busy_limit():
    return 1.0
def toolchain(env):
    return 'pinned'
def check_idle(when):
    return 'refuse'
def build_cost(env, jobs, refused):
    return 'ran'
def main():
    print(f'busy={BUSY_SHARE} idle={check_idle(\"x\")} cost={build_cost(None, 1, None)} tool={toolchain({})}')
";
        let path = rind.join("admin/bench-report");
        std::fs::write(&path, script).unwrap();
        let before = std::fs::read(&path).unwrap();
        let relax = Relax {
            busy_share: Some("0.25".into()),
            accept_toolchain: true,
            record_busy: true,
            skip_build_cost: true,
            ..Relax::default()
        };
        assert!(relax.any());
        let mut child = std::process::Command::new("python3");
        // Drive the loader the same way run_bench_report does, capturing stdout.
        let status = child
            .arg("-c")
            .arg(LOADER)
            .arg(&path)
            .arg("run")
            .env("RIND_BUSY_SHARE", "0.25")
            .env("RIND_ACCEPT_TOOLCHAIN", "1")
            .env("RIND_RECORD_BUSY", "1")
            .env("RIND_SKIP_BUILD_COST", "1")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&status.stdout);
        let stderr = String::from_utf8_lossy(&status.stderr);
        assert!(status.status.success(), "{stderr}{stdout}");
        assert!(stdout.contains("busy=0.25"), "{stdout}");
        assert!(stdout.contains("idle=1.25"), "{stdout}");
        assert!(
            stdout.contains("skipped by host-run (HOSTRUN_SKIP_BUILD_COST)"),
            "{stdout}"
        );
        assert_eq!(std::fs::read(&path).unwrap(), before);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn least_busy_core_skips_cpu0() {
        let line = |name: &str, busy_user: u64, idle: u64| {
            format!("{name} {busy_user} 0 0 {idle} 0 0 0 0\n")
        };
        let before = format!(
            "{}{}",
            line("cpu0", 0, 100),
            line("cpu1", 0, 100) + &line("cpu2", 0, 100)
        );
        // cpu1 did 50 of 100 busy, cpu2 did 0 of 100.
        let after = format!(
            "{}{}{}",
            line("cpu0", 90, 110),
            line("cpu1", 50, 150),
            line("cpu2", 0, 200)
        );
        assert_eq!(least_busy_core(&before, &after), Some(2));
    }

    #[test]
    fn clean_commit_rejects_a_dirty_tree() {
        let dir = std::env::temp_dir().join(format!("rind-clean-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert!(
            std::process::Command::new("git")
                .args(["init"])
                .current_dir(&dir)
                .status()
                .unwrap()
                .success()
        );
        std::fs::write(dir.join("f"), "a\n").unwrap();
        assert!(
            std::process::Command::new("git")
                .args(["add", "f"])
                .current_dir(&dir)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            std::process::Command::new("git")
                .args([
                    "-c",
                    "user.email=t@example.com",
                    "-c",
                    "user.name=t",
                    "commit",
                    "-m",
                    "init"
                ])
                .current_dir(&dir)
                .status()
                .unwrap()
                .success()
        );
        let head = crate::git(&dir, &["rev-parse", "HEAD"]).unwrap();
        assert_eq!(require_clean_commit(&dir, &head[..12]).unwrap(), head);
        std::fs::write(dir.join("f"), "b\n").unwrap();
        assert!(require_clean_commit(&dir, &head).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
