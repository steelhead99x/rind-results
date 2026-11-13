// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! On-board run commands that used to be `scripts/orin-*.sh`, `scripts/pixel-run.sh`,
//! `scripts/pixel-run2.sh`, `scripts/mac-run.sh`, and `scripts/pi-run.sh`.
//!
//! They call rind's `admin/bench-report` through the in-memory loader. They do
//! not call `host-run.py`. Nothing here flashes a board or starts a measurement
//! unless the command is invoked.

use crate::upstream_run::{self, Relax};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

/// The published Mac and Pi host results. Not the comparable-set pin
/// (`layout::COMPARABLE_PIN`). Pass it with `--commit` to reproduce those runs.
pub const HOST_COMMIT: &str = "b66514238d449fc211d75fd14554301bb690e421";

pub fn home_dir() -> Result<PathBuf, String> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or_else(|| "HOME is unset".into())
}

pub fn orin_home() -> Result<PathBuf, String> {
    Ok(home_dir()?.join("rind-bench-orin"))
}

/// `scripts/orin-faketime.sh`: cargo's target runner. The offset is read when
/// the bench binary starts, not when this script is written.
pub fn faketime_script(home: &Path) -> String {
    let lib = home.join("faketime/usr/lib/aarch64-linux-gnu/faketime/libfaketimeMT.so.1");
    format!(
        "#!/bin/sh\nexec env LD_PRELOAD=\"{}\" FAKETIME=\"+${{FAKETIME_OFFSET:?}}s\" FAKETIME_DONT_FAKE_MONOTONIC=1 FAKETIME_DONT_RESET=1 \"$@\"\n",
        lib.display()
    )
}

pub fn cargo_vendor_config(home: &Path) -> String {
    format!(
        "[source.crates-io]\nreplace-with = \"vendored-sources\"\n\n[source.vendored-sources]\ndirectory = \"{}/vendor\"\n",
        home.display()
    )
}

/// `date -u -d @secs +%FT%T+00:00`.
pub fn pc_stamp(unix_secs: i64) -> String {
    let (days, rem) = (unix_secs.div_euclid(86400), unix_secs.rem_euclid(86400));
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

/// Busy share of each cpu, awk in `orin-run.sh`'s `percore`, as `name 0.000` lines.
pub fn percore_lines(before: &str, after: &str) -> String {
    let mut out = String::new();
    for (name, a, b) in paired_cpus(before, after) {
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
        out.push_str(&format!("  {name} {busy:.3}\n"));
    }
    out
}

fn paired_cpus<'a>(before: &'a str, after: &'a str) -> Vec<(&'a str, Vec<u64>, Vec<u64>)> {
    let parse = |text: &'a str| -> Vec<(&'a str, Vec<u64>)> {
        text.lines()
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                let name = parts.next()?;
                if !name.starts_with("cpu") || name == "cpu" {
                    return None;
                }
                let nums = parts.filter_map(|p| p.parse().ok()).collect();
                Some((name, nums))
            })
            .collect()
    };
    let a = parse(before);
    let b = parse(after);
    a.into_iter()
        .filter_map(|(name, nums)| {
            let other = b.iter().find(|(n, _)| *n == name)?.1.clone();
            Some((name, nums, other))
        })
        .collect()
}

/// Lowest `cpu1`..`cpu5` line in an orin load file (`  cpuN 0.000` only).
pub fn pick_core_from_load(text: &str) -> Option<u32> {
    let mut best: Option<(u32, f64, usize)> = None;
    for (i, line) in text.lines().enumerate() {
        let mut parts = line.split_whitespace();
        let Some(name) = parts.next() else { continue };
        let Some(busy) = parts.next() else { continue };
        if parts.next().is_some() {
            continue;
        }
        let Some(cpu) = name.strip_prefix("cpu").and_then(|s| s.parse().ok()) else {
            continue;
        };
        if !(1..=5).contains(&cpu) {
            continue;
        }
        let Ok(busy) = busy.parse::<f64>() else {
            continue;
        };
        if best.is_none_or(|(_, b, _)| busy < b) {
            best = Some((cpu, busy, i));
        }
    }
    best.map(|(c, _, _)| c)
}

pub fn df_avail(text: &str) -> Option<u64> {
    text.lines().nth(1)?.split_whitespace().nth(3)?.parse().ok()
}

pub fn mac_abort(free_gib: u64) -> bool {
    free_gib < 5
}

pub fn mac_watchdog(free_mib: u64) -> bool {
    free_mib < 1500
}

fn run(cmd: &mut Command) -> Result<(), String> {
    let status = cmd.status().map_err(|e| format!("{cmd:?}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{cmd:?}: {status}"))
    }
}

fn one_match(dir: &Path, prefix: &str, suffix: &str) -> Result<PathBuf, String> {
    let mut hits = vec![];
    for e in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let e = e.map_err(|e| e.to_string())?;
        let name = e.file_name().to_string_lossy().into_owned();
        if name.starts_with(prefix) && name.ends_with(suffix) {
            hits.push(e.path());
        }
    }
    hits.sort();
    match hits.len() {
        1 => Ok(hits.remove(0)),
        0 => Err(format!("no {prefix}*{suffix} in {}", dir.display())),
        _ => Err(format!(
            "more than one {prefix}*{suffix} in {}",
            dir.display()
        )),
    }
}

fn copy_no_clobber(from: &Path, to: &Path) -> Result<(), String> {
    if to.exists() {
        return Ok(());
    }
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::copy(from, to).map_err(|e| format!("{}: {e}", from.display()))?;
    Ok(())
}

/// Copy multiarch OpenSSL headers and point the `.so` links at the system libraries.
pub fn link_sysroot_openssl(sysroot: &Path, multiarch: &str) -> Result<(), String> {
    let inc = sysroot.join("usr/include");
    let src = inc.join(multiarch).join("openssl");
    let dst = inc.join("openssl");
    fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
    if src.is_dir() {
        for e in fs::read_dir(&src).map_err(|e| e.to_string())? {
            let e = e.map_err(|e| e.to_string())?;
            if e.file_name().to_string_lossy().ends_with(".h") {
                copy_no_clobber(&e.path(), &dst.join(e.file_name()))?;
            }
        }
    }
    let lib = sysroot.join("lib");
    fs::create_dir_all(&lib).map_err(|e| e.to_string())?;
    let sys = PathBuf::from("/usr/lib").join(multiarch);
    symlink(&sys.join("libssl.so.3"), &lib.join("libssl.so"))?;
    symlink(&sys.join("libcrypto.so.3"), &lib.join("libcrypto.so"))?;
    Ok(())
}

fn symlink(target: &Path, link: &Path) -> Result<(), String> {
    if link.exists() || link.symlink_metadata().is_ok() {
        fs::remove_file(link).ok();
    }
    std::os::unix::fs::symlink(target, link).map_err(|e| format!("{}: {e}", link.display()))
}

pub fn write_cargo_config(home: &Path) -> Result<(), String> {
    let dir = home.join("cargo-home");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::write(dir.join("config.toml"), cargo_vendor_config(home)).map_err(|e| e.to_string())
}

/// `scripts/orin-setup.sh`. Looks in `home` for `rind-*.tar.gz`, the rust
/// 1.98.1 aarch64 tarball, and `libssl-dev_*_arm64.deb`.
pub fn orin_setup(home: &Path) -> Result<(), String> {
    let tar = one_match(home, "rind-", ".tar.gz")?;
    let rind = home.join("rind");
    fs::remove_dir_all(&rind).ok();
    fs::create_dir_all(&rind).map_err(|e| e.to_string())?;
    run(Command::new("tar")
        .args(["xzmf"])
        .arg(&tar)
        .arg("-C")
        .arg(&rind)
        .current_dir(home))?;
    let rustc = home.join("rust/bin/rustc");
    if !is_exec(&rustc) {
        let inst = home.join("rust-inst");
        fs::remove_dir_all(&inst).ok();
        fs::create_dir_all(&inst).map_err(|e| e.to_string())?;
        let ball = one_match(home, "rust-1.98.1-aarch64-unknown-linux-gnu", ".tar.xz")?;
        run(Command::new("tar")
            .args(["xJmf"])
            .arg(&ball)
            .arg("-C")
            .arg(&inst)
            .current_dir(home))?;
        let installer = inst.join("rust-1.98.1-aarch64-unknown-linux-gnu/install.sh");
        run(Command::new("nice").args([
            "-n",
            "19",
            &installer.display().to_string(),
            &format!("--prefix={}", home.join("rust").display()),
            "--components=rustc,cargo,rust-std-aarch64-unknown-linux-gnu",
            "--disable-ldconfig",
        ]))?;
        fs::remove_dir_all(&inst).ok();
    }
    let deb = one_match(home, "libssl-dev_", "_arm64.deb")?;
    let sysroot = home.join("sysroot");
    fs::remove_dir_all(&sysroot).ok();
    fs::create_dir_all(sysroot.join("lib")).map_err(|e| e.to_string())?;
    run(Command::new("dpkg").arg("-x").arg(&deb).arg(&sysroot))?;
    link_sysroot_openssl(&sysroot, "aarch64-linux-gnu")?;
    write_cargo_config(home)?;
    let env = upstream_run::orin_env(home);
    let mut cmd = Command::new(home.join("rust/bin/rustc"));
    cmd.arg("-V");
    for (k, v) in &env {
        cmd.env(k, v);
    }
    run(&mut cmd)?;
    let mut cargo = Command::new(home.join("rust/bin/cargo"));
    cargo.arg("-V");
    for (k, v) in &env {
        cargo.env(k, v);
    }
    run(&mut cargo)
}

fn is_exec(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

fn write_runner(home: &Path) -> Result<(), String> {
    let path = home.join("orin-faketime.sh");
    fs::write(&path, faketime_script(home)).map_err(|e| e.to_string())?;
    use std::os::unix::fs::PermissionsExt;
    let mut p = fs::metadata(&path)
        .map_err(|e| e.to_string())?
        .permissions();
    p.set_mode(0o755);
    fs::set_permissions(&path, p).map_err(|e| e.to_string())
}

fn read_proc_stat() -> Result<String, String> {
    fs::read_to_string("/proc/stat").map_err(|e| format!("/proc/stat: {e}"))
}

fn snap_body(title: &str, percore: &str) -> String {
    let mut s = format!("## {title}\n");
    let load = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    s.push_str(&format!("loadavg: {}", load.trim()));
    s.push('\n');
    s.push_str("per-cpu busy over 10 s:\n");
    s.push_str(percore);
    if let Ok(rd) = fs::read_dir("/sys/devices/system/cpu") {
        let mut cpus: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.file_name().and_then(|n| n.to_str()).is_some_and(|n| {
                    n.starts_with("cpu") && n[3..].chars().all(|c| c.is_ascii_digit())
                })
            })
            .collect();
        cpus.sort();
        for cpu in cpus {
            let freq = cpu.join("cpufreq");
            if !freq.is_dir() {
                continue;
            }
            let gov = read_trim(&freq.join("scaling_governor"));
            let cur = read_trim(&freq.join("scaling_cur_freq"));
            let max = read_trim(&freq.join("scaling_max_freq"));
            let name = cpu.file_name().unwrap_or_default().to_string_lossy();
            s.push_str(&format!("  {name} {gov} cur={cur} max={max}\n"));
        }
    }
    s.push_str("top cpu users:\n");
    if let Ok(out) = Command::new("ps")
        .args(["-eo", "pcpu,comm", "--sort=-pcpu"])
        .output()
    {
        for line in String::from_utf8_lossy(&out.stdout).lines().skip(1).take(5) {
            s.push_str("  ");
            s.push_str(line);
            s.push('\n');
        }
    }
    if let Ok(zones) = fs::read_dir("/sys/class/thermal") {
        let mut n = 0;
        let mut paths: Vec<PathBuf> = zones.flatten().map(|e| e.path()).collect();
        paths.sort();
        for z in paths {
            if n == 9 {
                break;
            }
            if !z
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("thermal_zone"))
            {
                continue;
            }
            s.push_str(&format!(
                "  thermal {} {}\n",
                read_trim(&z.join("type")),
                read_trim(&z.join("temp"))
            ));
            n += 1;
        }
    }
    s
}

fn read_trim(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// One Jetson run: `scripts/orin-run.sh <n> [core]`.
pub fn orin_run(home: &Path, n: u32, core: Option<u32>) -> Result<u32, String> {
    let rind = home.join("rind");
    let raw = home.join("raw");
    fs::create_dir_all(&raw).map_err(|e| e.to_string())?;
    let before = read_proc_stat()?;
    std::thread::sleep(Duration::from_secs(10));
    let after = read_proc_stat()?;
    let percore = percore_lines(&before, &after);
    let mut load = snap_body(&format!("before run {n}"), &percore);
    let core = match core {
        Some(c) => c,
        None => pick_core_from_load(&percore).ok_or("cpu1..cpu5 not in /proc/stat")?,
    };
    load.push_str(&format!("pinned to cpu{core}\n"));
    let offset = std::env::var("FAKETIME_OFFSET")
        .ok()
        .filter(|s| !s.is_empty());
    if offset.is_some() {
        write_runner(home)?;
        load.push_str(&format!(
            "libfaketime runner: wall clock +{}s for the bench binaries, monotonic untouched\n",
            offset.as_deref().unwrap_or("")
        ));
    }
    let load_path = raw.join(format!("run-{n}.load.txt"));
    fs::write(&load_path, &load).map_err(|e| e.to_string())?;
    let commit = fs::read_to_string(rind.join(".commit")).map_err(|e| {
        format!(
            "{}: {e} (orin-run reads .commit)",
            rind.join(".commit").display()
        )
    })?;
    let commit = commit.trim().to_string();
    let mut env = upstream_run::orin_env(home);
    if let Some(offset) = &offset {
        env.push(("FAKETIME_OFFSET".into(), offset.clone()));
        env.push((
            "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER".into(),
            home.join("orin-faketime.sh").display().to_string(),
        ));
    }
    let env_ref: Vec<(&str, &str)> = env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let target = env
        .iter()
        .find(|(k, _)| k == "CARGO_TARGET_DIR")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| home.join("target").display().to_string());
    let out = raw.join(format!("run-{n}.json"));
    let args = [
        "run",
        "--host",
        "jetson-orin-nano",
        "--commit",
        &commit,
        "--jobs",
        "2",
        "--target-dir",
        &target,
        "--out",
        &out.display().to_string(),
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();
    let relax = Relax {
        accept_toolchain: true,
        record_busy: true,
        skip_build_cost: true,
        ..Relax::default()
    };
    let log = raw.join(format!("run-{n}.log"));
    let rc = upstream_run::run_bench_report(
        &rind,
        &args,
        &relax,
        &env_ref,
        Some(&core.to_string()),
        Some(&log),
    );
    let code = if rc.is_ok() { 0 } else { 1 };
    let after_stat = read_proc_stat().unwrap_or_default();
    std::thread::sleep(Duration::from_secs(10));
    let after2 = read_proc_stat().unwrap_or_default();
    let mut tail = format!("exit: {code}\n");
    tail.push_str(&snap_body(
        &format!("after run {n}"),
        &percore_lines(&after_stat, &after2),
    ));
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(&load_path)
        .map_err(|e| e.to_string())?;
    f.write_all(tail.as_bytes()).map_err(|e| e.to_string())?;
    println!("run {n} rc={code} core={core}");
    rc.map(|()| core)
}

/// `scripts/orin-runs.sh`. Run 1 picks the core unless `core` is set; runs 2 and 3 reuse it.
pub fn orin_runs(home: &Path, mut core: Option<u32>) -> Result<(), String> {
    let raw = home.join("raw");
    fs::create_dir_all(&raw).map_err(|e| e.to_string())?;
    let offset: i64 = std::env::var("FAKETIME_OFFSET")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let preset = core;
    for n in 1..=3 {
        let start = pc_now(offset);
        fs::write(
            raw.join(format!("run-{n}.pc.txt")),
            format!("pc_started: {start}\n"),
        )
        .map_err(|e| e.to_string())?;
        let used = orin_run(home, n, core)?;
        let end = pc_now(offset);
        let mut f = fs::OpenOptions::new()
            .append(true)
            .open(raw.join(format!("run-{n}.pc.txt")))
            .map_err(|e| e.to_string())?;
        writeln!(f, "pc_finished: {end}").map_err(|e| e.to_string())?;
        if preset.is_none() {
            core = Some(used);
        }
    }
    println!("runs-done");
    Ok(())
}

fn pc_now(offset: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    pc_stamp(now + offset)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Adb {
    pub args: Vec<String>,
    pub stdout: Option<String>,
    pub append: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PixelStep {
    Adb(Adb),
    /// First pass only: wait until `$remote/done` exists.
    Poll,
}

pub struct PixelOpts {
    pub serial: String,
    pub work: PathBuf,
    pub pkg_tar: PathBuf,
    pub remote: String,
    pub second_pass: bool,
    pub wait_for: Option<PathBuf>,
}

/// One absolute phone path that is safe to interpolate into `adb shell`
/// and to pass as an adb push/pull argument. Letters, digits, `.`, `_`,
/// and `-` only, so the remote shell cannot see a second command.
fn remote_path(path: &str) -> Result<&str, String> {
    let mut segs = path.split('/');
    if !path.starts_with('/') || segs.next() != Some("") {
        return Err(remote_path_err());
    }
    let mut any = false;
    for seg in segs {
        any = true;
        if seg.is_empty() || seg == "." || seg == ".." {
            return Err(remote_path_err());
        }
        let safe = seg
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
        if !safe {
            return Err(remote_path_err());
        }
    }
    if any {
        Ok(path)
    } else {
        Err(remote_path_err())
    }
}

fn remote_path_err() -> String {
    "pixel remote path must be one absolute path of letters, digits, '.', '_' and '-'".into()
}

/// Product model and Android release/SDK only. A full `getprop` includes
/// serials and radio addresses. Values stay in getprop's `[key]: [value]`
/// form so android-convert can parse the capture.
const PIXEL_GETPROP: &str = concat!(
    "for k in ro.product.model ro.build.version.release ro.build.version.sdk; do ",
    r#"printf '[%s]: [%s]\n' "$k" "$(getprop "$k")"; done"#,
);

/// The adb and tar steps of `pixel-run.sh` or `pixel-run2.sh`, in order.
/// Sleeps and the status-file wait are not in this list; the runner does those.
pub fn pixel_adbs(o: &PixelOpts) -> Result<Vec<PixelStep>, String> {
    let s = &o.serial;
    let p = remote_path(&o.remote)?;
    let adb = |args: &[&str]| {
        PixelStep::Adb(Adb {
            args: args.iter().map(|s| (*s).to_string()).collect(),
            stdout: None,
            append: false,
        })
    };
    let mut v = vec![adb(&["-s", s, "get-state"])];
    let capture = |shell: &str, file: &str| {
        PixelStep::Adb(Adb {
            args: vec!["-s".into(), s.into(), "shell".into(), shell.into()],
            stdout: Some(file.into()),
            append: false,
        })
    };
    v.push(capture("dumpsys battery", "battery-before.txt"));
    if !o.second_pass {
        v.push(capture(PIXEL_GETPROP, "getprop.txt"));
        v.push(capture("cat /proc/cpuinfo; uname -a", "cpuinfo.txt"));
    }
    v.push(adb(&[
        "-s",
        s,
        "shell",
        &format!("rm -rf {p} && mkdir -p {p}"),
    ]));
    if o.second_pass {
        v.push(adb(&[
            "-s",
            s,
            "push",
            "pkg/bin",
            "pkg/src",
            "pkg/run2.sh",
            &format!("{p}/"),
        ]));
        v.push(adb(&[
            "-s",
            s,
            "shell",
            &format!("chmod 755 {p}/run2.sh {p}/bin/*; cd {p} && sh ./run2.sh > {p}/run.log 2>&1"),
        ]));
    } else {
        v.push(adb(&[
            "-s",
            s,
            "push",
            "pkg/bin",
            "pkg/run.sh",
            &format!("{p}/"),
        ]));
        v.push(adb(&[
            "-s",
            s,
            "shell",
            &format!(
                "chmod 755 {p}/run.sh {p}/bin/*; cd {p} && nohup sh ./run.sh > {p}/run.log 2>&1 &"
            ),
        ]));
        v.push(PixelStep::Poll);
    }
    v.push(capture("dumpsys battery", "battery-after.txt"));
    let pulls = if o.second_pass {
        [
            "run.log",
            "started",
            "finished",
            "stat-before",
            "stat-after",
            "criterion.tar",
        ]
        .as_slice()
    } else {
        [
            "run.log",
            "cpuid.json",
            "cpuid.err",
            "started",
            "finished",
            "stat-before",
            "stat-after",
            "criterion.tar",
        ]
        .as_slice()
    };
    for f in pulls {
        v.push(adb(&["-s", s, "pull", &format!("{p}/{f}"), "."]));
    }
    v.push(adb(&["-s", s, "shell", &format!("rm -rf {p}")]));
    v.push(PixelStep::Adb(Adb {
        args: vec!["-s".into(), s.into(), "shell".into(), format!("ls {p}")],
        stdout: Some("cleanup-check.txt".into()),
        append: false,
    }));
    Ok(v)
}

fn poll_phone(o: &PixelOpts) -> Result<(), String> {
    let p = remote_path(&o.remote)?;
    loop {
        let mut probe = Command::new("adb");
        probe.args(["-s", &o.serial, "shell", &format!("test -f {p}/done")]);
        if probe.status().map(|s| s.success()).unwrap_or(false) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_secs(60));
        let mut tail = Command::new("adb");
        tail.args(["-s", &o.serial, "shell", &format!("tail -1 {p}/run.log")]);
        if let Ok(out) = tail.output() {
            let line: String = String::from_utf8_lossy(&out.stdout)
                .chars()
                .take(120)
                .collect();
            fs::write(o.work.join("progress.txt"), format!("{line}\n")).ok();
        }
        let mut bat = Command::new("adb");
        bat.args(["-s", &o.serial, "shell", "dumpsys battery"]);
        if let Ok(out) = bat.output() {
            let text = String::from_utf8_lossy(&out.stdout);
            let kept = text
                .lines()
                .filter(|l| l.contains("temperature") || l.contains("level"))
                .collect::<Vec<_>>()
                .join(" ");
            let mut f = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(o.work.join("battery-during.txt"))
                .map_err(|e| e.to_string())?;
            let _ = writeln!(f, "{kept}");
        }
    }
}

pub fn pixel_run(o: PixelOpts) -> Result<(), String> {
    remote_path(&o.remote)?;
    if o.serial.is_empty() || o.serial == "<phone-serial>" {
        return Err(
            "pixel-run needs --serial (the old script's <phone-serial> placeholder)".into(),
        );
    }
    fs::create_dir_all(&o.work).map_err(|e| e.to_string())?;
    if let Some(wait) = &o.wait_for {
        eprintln!("waiting for desktop run");
        while !wait.is_file() {
            std::thread::sleep(Duration::from_secs(30));
        }
        eprintln!("desktop run finished");
        std::thread::sleep(Duration::from_secs(60));
    }
    let status = o.work.join("status");
    // Unpack before the first adb that needs pkg/. get-state is first.
    if let Err(e) = adb_status(&o.serial) {
        fs::write(&status, "no-device\n").ok();
        return Err(e);
    }
    let pkg = o.work.join("pkg");
    fs::remove_dir_all(&pkg).ok();
    fs::create_dir_all(&pkg).map_err(|e| e.to_string())?;
    run(Command::new("tar")
        .args(["xzf"])
        .arg(&o.pkg_tar)
        .arg("-C")
        .arg(&pkg)
        .current_dir(&o.work))?;
    for step in pixel_adbs(&o)?.into_iter().skip(1) {
        let PixelStep::Adb(step) = step else {
            poll_phone(&o)?;
            continue;
        };
        let mut cmd = Command::new("adb");
        cmd.args(&step.args).current_dir(&o.work);
        if let Some(name) = &step.stdout {
            let path = o.work.join(name);
            let file = fs::File::create(&path).map_err(|e| e.to_string())?;
            cmd.stdout(Stdio::from(file));
        }
        if step.args.iter().any(|a| a == "push") {
            let log = fs::File::create(o.work.join("push.log")).map_err(|e| e.to_string())?;
            cmd.stdout(Stdio::from(log.try_clone().map_err(|e| e.to_string())?));
            cmd.stderr(Stdio::from(log));
            if !cmd.status().map_err(|e| e.to_string())?.success() {
                fs::write(&status, "push-failed\n").ok();
                return Err("push failed".into());
            }
            continue;
        }
        // Pulls are best-effort, matching the shell's `|| true` via `>/dev/null`.
        if step.args.iter().any(|a| a == "pull") {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
            let _ = cmd.status();
            continue;
        }
        if run(&mut cmd).is_err() && step.args.iter().any(|a| a == "get-state") {
            fs::write(&status, "no-device\n").ok();
            return Err("phone not connected".into());
        }
    }
    let archive = if o.second_pass {
        "pixel-result2.tar.gz"
    } else {
        "pixel-result.tar.gz"
    };
    let files = if o.second_pass {
        vec![
            "run.log",
            "started",
            "finished",
            "stat-before",
            "stat-after",
            "criterion.tar",
            "battery-before.txt",
            "battery-after.txt",
        ]
    } else {
        vec![
            "run.log",
            "cpuid.json",
            "cpuid.err",
            "started",
            "finished",
            "stat-before",
            "stat-after",
            "criterion.tar",
            "battery-before.txt",
            "battery-after.txt",
            "battery-during.txt",
            "getprop.txt",
            "cpuinfo.txt",
        ]
    };
    let mut tar = Command::new("tar");
    tar.arg("czf")
        .arg(archive)
        .args(&files)
        .current_dir(&o.work);
    let _ = tar.status();
    fs::remove_dir_all(o.work.join("pkg")).ok();
    fs::write(&status, "done\n").map_err(|e| e.to_string())?;
    Ok(())
}

fn adb_status(serial: &str) -> Result<(), String> {
    let ok = Command::new("adb")
        .args(["-s", serial, "get-state"])
        .status()
        .map_err(|e| format!("adb: {e}"))?
        .success();
    if ok {
        Ok(())
    } else {
        Err("phone not connected".into())
    }
}

fn host_relax(free_target: bool) -> Relax {
    Relax {
        accept_toolchain: true,
        record_busy: true,
        free_target,
        ..Relax::default()
    }
}

fn note_published_host_commit(commit: &str) {
    if crate::layout::sha_matches(commit, HOST_COMMIT) {
        eprintln!(
            "host commit {commit} is the published b6651423 set, not the comparable pin {}",
            crate::layout::COMPARABLE_PIN
        );
    }
}

/// `scripts/mac-run.sh`. The default commit is the comparable-set pin.
/// `HOST_COMMIT` is the published `b6651423` run.
pub fn mac_run(home: &Path, commit: &str) -> Result<(), String> {
    note_published_host_commit(commit);
    let status = home.join("status");
    let free = df_avail_cmd(home, &["-g"])?;
    if mac_abort(free) {
        fs::write(&status, "aborted\n").ok();
        return Err(format!("under 5 GiB free ({free}G)"));
    }
    unpack_src(home)?;
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = stop.clone();
    let watch_home = home.to_path_buf();
    let watch_status = status.clone();
    let watchdog = std::thread::spawn(move || {
        while !flag.load(std::sync::atomic::Ordering::Relaxed) {
            std::thread::sleep(Duration::from_secs(20));
            if watch_status.is_file() {
                return;
            }
            if let Ok(mib) = df_avail_cmd(&watch_home, &["-m"]) {
                if mac_watchdog(mib) {
                    eprintln!("WATCHDOG: {mib}MB free, stopping");
                    let _ = Command::new("pkill")
                        .args(["-f", "admin/bench-report run --host m1pro"])
                        .status();
                    let _ = fs::remove_dir_all(watch_home.join("target"));
                    let _ = fs::remove_dir_all(watch_home.join("src/.tmp"));
                    fs::write(&watch_status, "lowdisk\n").ok();
                    return;
                }
            }
        }
    });
    let rind = home.join("src");
    let out = home.join("m1pro.json");
    let relax = host_relax(true);
    let args = host_args("m1pro", commit, 8, &home.join("target"), &out);
    let env = [
        ("CARGO_HOME", home.join("cargo").display().to_string()),
        ("HOSTRUN_FREE_TARGET", "1".into()),
    ];
    let env_ref: Vec<(&str, &str)> = env.iter().map(|(k, v)| (*k, v.as_str())).collect();
    let rc = upstream_run::run_bench_report(&rind, &args, &relax, &env_ref, None, None);
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    let _ = watchdog.join();
    let _ = fs::remove_dir_all(home.join("target"));
    let _ = fs::remove_dir_all(home.join("src/.tmp"));
    if !status.is_file() {
        let msg = match &rc {
            Ok(()) => "done rc=0\n",
            Err(_) => "done rc=1\n",
        };
        fs::write(&status, msg).ok();
    }
    rc
}

/// `scripts/pi-run.sh`. The default commit is the comparable-set pin.
/// `HOST_COMMIT` is the published `b6651423` run.
pub fn pi_run(home: &Path, commit: &str) -> Result<(), String> {
    note_published_host_commit(commit);
    let status = home.join("status");
    let _ = fs::remove_file(&status);
    if Command::new("openssl")
        .arg("version")
        .stdout(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        // present
    } else {
        fs::write(&status, "no-openssl-cli\n").ok();
        return Err("openssl CLI missing".into());
    }
    let mut env_extra: Vec<(String, String)> = vec![];
    if !Command::new("pkg-config")
        .args(["--exists", "openssl"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!(
            "libssl-dev not installed; unpacking it locally into {}/sysroot",
            home.display()
        );
        fs::remove_dir_all(home.join("sysroot")).ok();
        fs::remove_dir_all(home.join("debs")).ok();
        fs::create_dir_all(home.join("debs")).map_err(|e| e.to_string())?;
        fs::create_dir_all(home.join("sysroot")).map_err(|e| e.to_string())?;
        if run(Command::new("apt-get")
            .args(["download", "libssl-dev"])
            .current_dir(home.join("debs")))
        .is_err()
        {
            fs::write(&status, "no-libssl-dev\n").ok();
            return Err("apt-get download libssl-dev failed".into());
        }
        let deb = one_match(&home.join("debs"), "libssl-dev_", ".deb")?;
        run(Command::new("dpkg")
            .arg("-x")
            .arg(&deb)
            .arg(home.join("sysroot")))?;
        let multi = Command::new("gcc")
            .arg("-print-multiarch")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
        link_sysroot_openssl(&home.join("sysroot"), &multi)?;
        env_extra.push((
            "OPENSSL_INCLUDE_DIR".into(),
            home.join("sysroot/usr/include").display().to_string(),
        ));
        env_extra.push((
            "OPENSSL_LIB_DIR".into(),
            home.join("sysroot/lib").display().to_string(),
        ));
        env_extra.push(("OPENSSL_NO_PKG_CONFIG".into(), "1".into()));
    }
    unpack_src(home)?;
    let args = host_args(
        "pi5",
        commit,
        4,
        &home.join("target"),
        &home.join("pi5.json"),
    );
    let mut env = vec![(
        "CARGO_HOME".into(),
        home.join("cargo").display().to_string(),
    )];
    env.extend(env_extra);
    let env_ref: Vec<(&str, &str)> = env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let rc = upstream_run::run_bench_report(
        &home.join("src"),
        &args,
        &host_relax(false),
        &env_ref,
        None,
        None,
    );
    let msg = match &rc {
        Ok(()) => "done rc=0\n",
        Err(_) => "done rc=1\n",
    };
    fs::write(&status, msg).ok();
    rc
}

fn host_args(host: &str, commit: &str, jobs: u32, target: &Path, out: &Path) -> Vec<String> {
    [
        "run",
        "--host",
        host,
        "--commit",
        commit,
        "--jobs",
        &jobs.to_string(),
        "--target-dir",
        &target.display().to_string(),
        "--out",
        &out.display().to_string(),
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn unpack_src(home: &Path) -> Result<(), String> {
    let src = home.join("src");
    fs::remove_dir_all(&src).ok();
    fs::create_dir_all(&src).map_err(|e| e.to_string())?;
    run(Command::new("tar")
        .args(["xzf", "rind-src.tar.gz", "-C", "src"])
        .current_dir(home))
}

fn df_avail_cmd(home: &Path, flags: &[&str]) -> Result<u64, String> {
    let out = Command::new("df")
        .args(flags)
        .arg(home)
        .output()
        .map_err(|e| format!("df: {e}"))?;
    if !out.status.success() {
        return Err(format!("df: {}", String::from_utf8_lossy(&out.stderr)));
    }
    df_avail(&String::from_utf8_lossy(&out.stdout)).ok_or_else(|| "df: no available column".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_and_pi_default_to_the_comparable_pin() {
        assert_eq!(
            crate::layout::COMPARABLE_PIN,
            "d6ca9a756d3d1e71489e61e44c04f33c3c893589"
        );
        assert_ne!(HOST_COMMIT, crate::layout::COMPARABLE_PIN);
        assert!(crate::layout::sha_matches(
            "d6ca9a75",
            crate::layout::COMPARABLE_PIN
        ));
        assert!(!crate::layout::sha_matches(
            HOST_COMMIT,
            crate::layout::COMPARABLE_PIN
        ));
    }

    #[test]
    fn faketime_script_reads_the_offset_at_exec() {
        let text = faketime_script(Path::new("/home/me/rind-bench-orin"));
        assert!(text.contains("LD_PRELOAD=\"/home/me/rind-bench-orin/faketime/usr/lib/aarch64-linux-gnu/faketime/libfaketimeMT.so.1\""));
        assert!(text.contains("FAKETIME=\"+${FAKETIME_OFFSET:?}s\""));
        assert!(text.contains("FAKETIME_DONT_FAKE_MONOTONIC=1"));
        assert!(text.contains("FAKETIME_DONT_RESET=1"));
    }

    #[test]
    fn cargo_config_points_at_the_vendored_tree() {
        let text = cargo_vendor_config(Path::new("/home/me/rind-bench-orin"));
        assert!(text.contains("replace-with = \"vendored-sources\""));
        assert!(text.contains("directory = \"/home/me/rind-bench-orin/vendor\""));
    }

    #[test]
    fn pc_stamp_is_utc() {
        assert_eq!(pc_stamp(0), "1970-01-01T00:00:00+00:00");
        assert_eq!(pc_stamp(1_700_000_000), "2023-11-14T22:13:20+00:00");
    }

    #[test]
    fn percore_and_load_pick_the_quiet_core() {
        let line = |name: &str, user: u64, idle: u64| format!("{name} {user} 0 0 {idle} 0 0 0 0\n");
        let before = format!(
            "{}{}",
            line("cpu0", 0, 100),
            line("cpu1", 0, 100) + &line("cpu2", 0, 100)
        );
        let after = format!(
            "{}{}{}",
            line("cpu0", 90, 110),
            line("cpu1", 50, 150),
            line("cpu2", 0, 200)
        );
        let text = percore_lines(&before, &after);
        assert!(text.contains("  cpu1 "), "{text}");
        assert_eq!(pick_core_from_load(&text), Some(2));
    }

    #[test]
    fn df_and_mac_thresholds() {
        let sample = "Filesystem 1G-blocks Used Available\n/dev/disk 100 80 20 80%\n";
        assert_eq!(df_avail(sample), Some(20));
        assert!(mac_abort(4));
        assert!(!mac_abort(5));
        assert!(mac_watchdog(1499));
        assert!(!mac_watchdog(1500));
    }

    #[test]
    fn pixel_first_pass_pushes_run_sh_and_pulls_cpuid() {
        let o = PixelOpts {
            serial: "ABC".into(),
            work: PathBuf::from("/tmp/pixel"),
            pkg_tar: PathBuf::from("/tmp/pixel-pkg.tar.gz"),
            remote: "/data/local/tmp/rind".into(),
            second_pass: false,
            wait_for: None,
        };
        let flat: Vec<String> = pixel_adbs(&o)
            .expect("remote")
            .iter()
            .map(|s| match s {
                PixelStep::Poll => "poll".into(),
                PixelStep::Adb(a) => a.args.join(" "),
            })
            .collect();
        assert_eq!(flat[0], "-s ABC get-state");
        assert!(
            flat.iter()
                .any(|s| s.contains("push") && s.contains("pkg/run.sh"))
        );
        assert!(flat.iter().any(|s| s.contains("nohup sh ./run.sh")));
        let start = flat.iter().position(|s| s.contains("nohup")).unwrap();
        assert_eq!(flat[start + 1], "poll");
        assert!(
            flat.iter()
                .any(|s| s.contains("pull") && s.contains("cpuid.json"))
        );
        assert!(
            flat.iter()
                .any(|s| s.contains("rm -rf /data/local/tmp/rind"))
        );
        assert!(flat.iter().any(|s| s.contains(PIXEL_GETPROP)));
        assert!(
            flat.iter()
                .all(|s| { !s.split_whitespace().any(|w| w == "getprop") })
        );
    }

    #[test]
    fn pixel_second_pass_pushes_src_and_run2() {
        let o = PixelOpts {
            serial: "ABC".into(),
            work: PathBuf::from("/tmp/pixel2"),
            pkg_tar: PathBuf::from("/tmp/pixel-pkg2.tar.gz"),
            remote: "/data/local/tmp/rind".into(),
            second_pass: true,
            wait_for: None,
        };
        let flat: Vec<String> = pixel_adbs(&o)
            .expect("remote")
            .iter()
            .filter_map(|s| match s {
                PixelStep::Adb(a) => Some(a.args.join(" ")),
                PixelStep::Poll => None,
            })
            .collect();
        assert!(
            flat.iter()
                .any(|s| s.contains("pkg/run2.sh") && s.contains("pkg/src"))
        );
        assert!(flat.iter().any(|s| s.contains("sh ./run2.sh")));
        assert!(flat.iter().all(|s| !s.contains("cpuid.json")));
        assert!(flat.iter().all(|s| !s.contains("ro.product.model")));
    }

    #[test]
    fn pixel_remote_path_rejects_shell_metacharacters() {
        let sample = |remote: &str, second_pass: bool| PixelOpts {
            serial: "ABC".into(),
            work: PathBuf::from("/tmp/pixel"),
            pkg_tar: PathBuf::from("/tmp/pixel-pkg.tar.gz"),
            remote: remote.into(),
            second_pass,
            wait_for: None,
        };
        for remote in [
            "data/local/tmp/rind",
            "/data/local/tmp/rind;rm",
            "/data/local/tmp/$(id)",
            "/data/../etc",
            "/data/local/tmp/rind rind",
            "",
            "/data/local/tmp/rind/",
            "/data//local",
            "/data/local/tmp/rind'",
        ] {
            assert!(pixel_adbs(&sample(remote, false)).is_err(), "{remote}");
        }
        assert!(pixel_adbs(&sample("/data/local/tmp/rind", false)).is_ok());
    }

    #[test]
    fn openssl_links_do_not_clobber_headers() {
        let dir = std::env::temp_dir().join(format!("rind-ssl-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let sysroot = dir.join("sysroot");
        let src = sysroot.join("usr/include/aarch64-linux-gnu/openssl");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("ssl.h"), "new\n").unwrap();
        let dst = sysroot.join("usr/include/openssl");
        fs::create_dir_all(&dst).unwrap();
        fs::write(dst.join("ssl.h"), "kept\n").unwrap();
        link_sysroot_openssl(&sysroot, "aarch64-linux-gnu").unwrap();
        assert_eq!(fs::read_to_string(dst.join("ssl.h")).unwrap(), "kept\n");
        let link = fs::read_link(sysroot.join("lib/libssl.so")).unwrap();
        assert_eq!(
            link,
            PathBuf::from("/usr/lib/aarch64-linux-gnu/libssl.so.3")
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn rind_tar_extracts_with_the_setup_flags() {
        let dir = std::env::temp_dir().join(format!("rind-tar-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(dir.join("src/hello"), "hi\n").unwrap();
        assert!(
            Command::new("tar")
                .args(["czf", "rind-abc.tar.gz", "-C", "src", "hello"])
                .current_dir(&dir)
                .status()
                .unwrap()
                .success()
        );
        let home = dir.clone();
        let tar = one_match(&home, "rind-", ".tar.gz").unwrap();
        let rind = home.join("rind");
        fs::create_dir_all(&rind).unwrap();
        assert!(
            Command::new("tar")
                .args(["xzmf"])
                .arg(&tar)
                .arg("-C")
                .arg(&rind)
                .status()
                .unwrap()
                .success()
        );
        assert_eq!(fs::read_to_string(rind.join("hello")).unwrap(), "hi\n");
        write_cargo_config(&home).unwrap();
        let cfg = fs::read_to_string(home.join("cargo-home/config.toml")).unwrap();
        assert_eq!(cfg, cargo_vendor_config(&home));
        let _ = fs::remove_dir_all(&dir);
    }
}
