// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask setup-toolchain RIND_HARDWARE_DIR`: installs, without root,
//! what building and flashing rind-hardware's ESP32-S3 images needs, at the
//! versions the published results used:
//! rustup (stable), espup 0.19.0 -> Rust "esp" 1.97.0.0 + Xtensa GCC
//! esp-14.2.0_20241119, ldproxy 0.3.5, espflash 4.7.0.
//!
//! Tools land in RIND_HARDWARE_DIR/.tools. ESP-IDF v5.5.1 itself is fetched
//! by the first firmware build into esp32/.embuild. The downloads are the
//! Linux builds, as before. This replaces `tools/setup-toolchain.sh` and runs
//! the same commands in the same order
//! (`tools/xtask/tests/setup_toolchain.rs`).

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(clap::Args)]
pub struct Args {
    /// The rind-hardware checkout
    rind_hardware: PathBuf,
}

const ESPUP: &str = "https://github.com/esp-rs/espup/releases/download/v0.19.0";
const ESPFLASH: &str = "https://github.com/esp-rs/espflash/releases/download/v4.7.0";

/// Runs `c` to completion, failing like `set -e` would.
fn check(c: &mut Command) -> Result<(), String> {
    let name = c.get_program().to_string_lossy().into_owned();
    let st = c.status().map_err(|e| format!("{name}: {e}"))?;
    if st.success() {
        Ok(())
    } else {
        Err(format!("{name} failed ({st})"))
    }
}

/// `command -v name`: an executable `name` on `path`.
fn on_path(name: &str, path: &OsString) -> bool {
    std::env::split_paths(path).any(|d| is_exe(&d.join(name)))
}

#[cfg(unix)]
fn is_exe(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    p.metadata()
        .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_exe(p: &Path) -> bool {
    p.is_file()
}

#[cfg(unix)]
fn make_exe(p: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut perm = std::fs::metadata(p)
        .map_err(|e| format!("{}: {e}", p.display()))?
        .permissions();
    perm.set_mode(perm.mode() | 0o111);
    std::fs::set_permissions(p, perm).map_err(|e| format!("{}: {e}", p.display()))
}

#[cfg(not(unix))]
fn make_exe(_: &Path) -> Result<(), String> {
    Ok(())
}

/// `uname -m`, which names the release assets.
fn machine() -> Result<String, String> {
    let o = Command::new("uname")
        .arg("-m")
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("uname: {e}"))?;
    if !o.status.success() {
        return Err("uname -m failed".into());
    }
    Ok(String::from_utf8_lossy(&o.stdout).trim().to_string())
}

/// rustup's installer, `curl ... | sh -s -- -y --profile minimal`.
fn install_rustup() -> Result<(), String> {
    let mut curl = Command::new("curl")
        .args([
            "--proto",
            "=https",
            "--tlsv1.2",
            "-sSf",
            "https://sh.rustup.rs",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("curl: {e}"))?;
    let script = curl.stdout.take().ok_or("curl: no stdout")?;
    let sh = Command::new("sh")
        .args(["-s", "--", "-y", "--profile", "minimal"])
        .stdin(script)
        .status()
        .map_err(|e| format!("sh: {e}"));
    let fetched = curl.wait().map_err(|e| format!("curl: {e}"))?;
    let sh = sh?;
    // A pipeline's status is its last command's, as in the script.
    if !sh.success() {
        return Err(format!("rustup installer failed ({sh}, curl {fetched})"));
    }
    Ok(())
}

pub fn run(a: Args) -> Result<(), String> {
    let bench = std::path::absolute(&a.rind_hardware)
        .map_err(|e| format!("{}: {e}", a.rind_hardware.display()))?;
    if !bench.is_dir() {
        return Err(format!("{}: not a directory", a.rind_hardware.display()));
    }
    let tools = bench.join(".tools");
    std::fs::create_dir_all(&tools).map_err(|e| format!("{}: {e}", tools.display()))?;
    let arch = machine()?;

    let home = PathBuf::from(std::env::var_os("HOME").ok_or("HOME is not set")?);
    let cargo_bin = home.join(".cargo/bin");
    let path = std::env::var_os("PATH").unwrap_or_default();
    if !on_path("rustup", &path) && !is_exe(&cargo_bin.join("rustup")) {
        install_rustup()?;
    }
    let path = std::env::join_paths(std::iter::once(cargo_bin).chain(std::env::split_paths(&path)))
        .map_err(|e| format!("PATH: {e}"))?;
    // Children find rustup and cargo through this PATH, like the script's
    // `export PATH=$HOME/.cargo/bin:$PATH`.
    let cmd = |name: &OsStr| {
        let mut c = Command::new(name);
        c.env("PATH", &path);
        c
    };

    check(cmd("rustup".as_ref()).args([
        "toolchain",
        "install",
        "stable",
        "--profile",
        "minimal",
        "--component",
        "rustfmt,clippy",
    ]))?;

    let espup = tools.join("espup");
    if !is_exe(&espup) {
        check(
            cmd("curl".as_ref())
                .args(["-sSfL", "-o"])
                .arg(&espup)
                .arg(format!("{ESPUP}/espup-{arch}-unknown-linux-gnu")),
        )?;
        make_exe(&espup)?;
    }
    check(
        cmd(espup.as_os_str())
            .args([
                "install",
                "--targets",
                "esp32s3",
                "--toolchain-version",
                "1.97.0.0",
            ])
            .arg("--export-file")
            .arg(tools.join("export-esp.sh")),
    )?;

    check(cmd("cargo".as_ref()).args(["install", "--locked", "ldproxy", "--version", "0.3.5"]))?;

    let espflash = tools.join("espflash");
    if !is_exe(&espflash) {
        let zip = tools.join("espflash.zip");
        check(
            cmd("curl".as_ref())
                .args(["-sSfL", "-o"])
                .arg(&zip)
                .arg(format!("{ESPFLASH}/espflash-{arch}-unknown-linux-musl.zip")),
        )?;
        check(
            cmd("unzip".as_ref())
                .args(["-o", "-q", "espflash.zip"])
                .current_dir(&tools)
                .env("PWD", &tools),
        )?;
        make_exe(&espflash)?;
        std::fs::remove_file(&zip).map_err(|e| format!("{}: {e}", zip.display()))?;
    }

    println!(
        "done: in {}, cargo xtask apply-patches && cargo xtask vendor-ring && cargo xtask build-images esp",
        bench.display()
    );
    println!("the board's USB-Serial/JTAG port needs your user in the dialout (or uucp) group");
    Ok(())
}
