// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask setup-toolchain RIND_HARDWARE_DIR`: installs, without root,
//! what building and flashing rind-hardware's ESP32-S3 images needs, at the
//! versions the published results used:
//! rustup-init 1.29.3, then the stable toolchain;
//! espup 0.19.0 -> Rust "esp" 1.97.0.0 + Xtensa GCC esp-14.2.0_20241119;
//! ldproxy 0.3.5; espflash 4.7.0.
//!
//! Tools land in RIND_HARDWARE_DIR/.tools. ESP-IDF v5.5.1 itself is fetched
//! by the first firmware build into esp32/.embuild. The downloads are the
//! Linux builds. Every downloaded binary is checked against
//! `tools/checksums.sha256` before it is executed or marked executable; a
//! mismatch deletes the file and stops.
//!
//! This replaces `tools/setup-toolchain.sh` and runs the same commands in the
//! same order (`tools/xtask/tests/setup_toolchain.rs`).

use sha2::{Digest, Sha256};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(clap::Args)]
pub struct Args {
    /// The rind-hardware checkout
    rind_hardware: PathBuf,
    /// SHA-256 pins for the downloads [default: tools/checksums.sha256]
    #[arg(long, hide = true)]
    checksums: Option<PathBuf>,
}

const RUSTUP_VERSION: &str = "1.29.3";
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

/// The pinned SHA-256 for `name`: the first field of the line whose second
/// field is `name` (`#` lines are comments).
fn pinned<'a>(sums: &'a str, name: &str) -> Option<&'a str> {
    sums.lines().filter(|l| !l.starts_with('#')).find_map(|l| {
        let mut f = l.split_whitespace();
        let (hash, n) = (f.next()?, f.next()?);
        (n == name).then_some(hash)
    })
}

/// Checks `dir/base` against the pin named `name`, printing `base: OK` as
/// `sha256sum -c` did. Anything else deletes the file.
fn verify(sums: &str, dir: &Path, base: &str, name: &str) -> Result<(), String> {
    let file = dir.join(base);
    let Some(want) = pinned(sums, name) else {
        let _ = std::fs::remove_file(&file);
        return Err(format!("no checksum recorded for {name}"));
    };
    let got = std::fs::read(&file).map(|b| {
        Sha256::digest(&b)
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect::<String>()
    });
    if got.as_deref().ok() == Some(want) {
        println!("{base}: OK");
        return Ok(());
    }
    println!("{base}: FAILED");
    let _ = std::fs::remove_file(&file);
    Err(format!("{base}: sha256 does not match {name}"))
}

pub fn run(a: Args) -> Result<(), String> {
    run_inner(a).map_err(|e| format!("setup-toolchain: {e}"))
}

fn run_inner(a: Args) -> Result<(), String> {
    let sums_path = a
        .checksums
        .unwrap_or_else(|| crate::root().join("tools/checksums.sha256"));
    let sums =
        std::fs::read_to_string(&sums_path).map_err(|e| format!("{}: {e}", sums_path.display()))?;
    let bench = std::path::absolute(&a.rind_hardware)
        .map_err(|e| format!("{}: {e}", a.rind_hardware.display()))?;
    if !bench.is_dir() {
        return Err(format!("{}: not a directory", a.rind_hardware.display()));
    }
    let tools = bench.join(".tools");
    std::fs::create_dir_all(&tools).map_err(|e| format!("{}: {e}", tools.display()))?;
    let arch = machine()?;
    if arch != "x86_64" && arch != "aarch64" {
        return Err(format!("no pinned checksum for arch {arch}"));
    }
    // Every download: `curl --proto =https --tlsv1.2 -fsSL -o FILE URL`.
    let fetch = |file: &Path, url: &str, path: Option<&OsString>| {
        let mut c = Command::new("curl");
        if let Some(p) = path {
            c.env("PATH", p);
        }
        check(
            c.args(["--proto", "=https", "--tlsv1.2", "-fsSL", "-o"])
                .arg(file)
                .arg(url),
        )
    };

    let home = PathBuf::from(std::env::var_os("HOME").ok_or("HOME is not set")?);
    let cargo_bin = home.join(".cargo/bin");
    let path = std::env::var_os("PATH").unwrap_or_default();
    if !on_path("rustup", &path) && !is_exe(&cargo_bin.join("rustup")) {
        let triple = format!("{arch}-unknown-linux-gnu");
        let init = tools.join("rustup-init");
        fetch(
            &init,
            &format!(
                "https://static.rust-lang.org/rustup/archive/{RUSTUP_VERSION}/{triple}/rustup-init"
            ),
            None,
        )?;
        verify(
            &sums,
            &tools,
            "rustup-init",
            &format!("rustup-init-{RUSTUP_VERSION}-{triple}"),
        )?;
        make_exe(&init)?;
        check(Command::new(&init).args(["-y", "--profile", "minimal"]))?;
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
        fetch(
            &espup,
            &format!("{ESPUP}/espup-{arch}-unknown-linux-gnu"),
            Some(&path),
        )?;
        verify(
            &sums,
            &tools,
            "espup",
            &format!("espup-0.19.0-{arch}-unknown-linux-gnu"),
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
        let zip_name = format!("espflash-{arch}-unknown-linux-musl.zip");
        let zip = tools.join("espflash.zip");
        fetch(&zip, &format!("{ESPFLASH}/{zip_name}"), Some(&path))?;
        verify(
            &sums,
            &tools,
            "espflash.zip",
            &format!("espflash-4.7.0-{arch}-unknown-linux-musl.zip"),
        )?;
        check(
            cmd("unzip".as_ref())
                .args(["-o", "-q", "espflash.zip"])
                .current_dir(&tools)
                .env("PWD", &tools),
        )?;
        std::fs::remove_file(&zip).map_err(|e| format!("{}: {e}", zip.display()))?;
        make_exe(&espflash)?;
    }

    println!(
        "done: in {}, cargo xtask apply-patches && cargo xtask vendor-ring && cargo xtask build-images esp",
        bench.display()
    );
    println!("the board's USB-Serial/JTAG port needs your user in the dialout (or uucp) group");
    Ok(())
}
