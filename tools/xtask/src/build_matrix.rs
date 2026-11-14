// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask build-matrix [TARGET] [CRATE]...`: release-builds each
//! `crates/tv-*` member of `baremetal/rp2040/builds` as a `no_std` rlib for
//! TARGET (default `thumbv6m-none-eabi`). If a build fails in getrandom
//! asking for a custom backend, it is built again with
//! `RUSTFLAGS='--cfg getrandom_backend="custom"'`. Prints one TSV line per
//! crate: crate, PASS or FAIL, `getrandom-custom=yes|no`, and the first error
//! line (240 characters at most). Each build's output is in
//! `logs/<target>/<crate>.log`. Ported from `build-matrix.sh`, with the same
//! output; `cargo` is the one on `PATH`, as the script ran it.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(clap::Args)]
pub struct Args {
    /// the builds workspace [default: baremetal/rp2040/builds]
    #[arg(long)]
    dir: Option<PathBuf>,
    #[arg(default_value = "thumbv6m-none-eabi")]
    target: String,
    /// crates to build [default: every entry of crates/, in name order]
    crates: Vec<String>,
}

const CUSTOM: &str = "--cfg getrandom_backend=\"custom\"";

/// One `cargo build`, its stdout and stderr both written to `log`.
fn build(dir: &Path, target: &str, krate: &str, log: &Path, rustflags: Option<&str>) -> bool {
    let run = || -> std::io::Result<bool> {
        let f = File::create(log)?;
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--release", "--target", target, "-p", krate])
            .current_dir(dir)
            .stdin(Stdio::inherit())
            .stdout(f.try_clone()?)
            .stderr(f);
        if let Some(flags) = rustflags {
            cmd.env("RUSTFLAGS", flags);
        }
        Ok(cmd.status()?.success())
    };
    match run() {
        Ok(ok) => ok,
        Err(e) => {
            let _ = fs::write(log, format!("build-matrix: cargo: {e}\n"));
            false
        }
    }
}

/// The script's retry test: the log names getrandom and asks for a custom
/// backend or says the target is not supported.
fn wants_custom(log: &str) -> bool {
    let lower = log.to_lowercase();
    log.contains("getrandom")
        && (lower.contains("custom backend") || lower.contains("not supported"))
}

/// The first line starting with `error`, else the first that mentions an
/// error in any case, cut to 240 characters.
fn first_error(log: &str) -> String {
    let line = log
        .lines()
        .find(|l| l.starts_with("error"))
        .or_else(|| log.lines().find(|l| l.to_lowercase().contains("error")))
        .unwrap_or("");
    line.chars().take(240).collect()
}

pub fn run(a: Args) -> Result<(), String> {
    let dir = a
        .dir
        .unwrap_or_else(|| crate::root().join("baremetal/rp2040/builds"));
    let crates = if a.crates.is_empty() {
        let cd = dir.join("crates");
        let rd = fs::read_dir(&cd).map_err(|e| format!("build-matrix: {}: {e}", cd.display()))?;
        let mut v: Vec<String> = rd
            .flatten()
            .filter_map(|e| e.file_name().to_str().map(str::to_string))
            .filter(|n| !n.starts_with('.'))
            .collect();
        v.sort();
        v
    } else {
        a.crates
    };
    let logs = dir.join("logs").join(&a.target);
    fs::create_dir_all(&logs).map_err(|e| format!("{}: {e}", logs.display()))?;
    let mut out = std::io::stdout();
    for c in crates {
        let log = logs.join(format!("{c}.log"));
        let read = || {
            fs::read(&log)
                .map(|b| String::from_utf8_lossy(&b).into_owned())
                .unwrap_or_default()
        };
        let mut cfg = "no";
        let mut ok = build(&dir, &a.target, &c, &log, None);
        if !ok && wants_custom(&read()) {
            cfg = "yes";
            ok = build(&dir, &a.target, &c, &log, Some(CUSTOM));
        }
        let (res, err) = if ok {
            ("PASS", String::new())
        } else {
            ("FAIL", first_error(&read()))
        };
        writeln!(out, "{c}\t{res}\tgetrandom-custom={cfg}\t{err}").map_err(|e| e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_needs_getrandom_and_a_backend_message() {
        assert!(wants_custom("getrandom: custom Backend required"));
        assert!(wants_custom("error: target is NOT SUPPORTED by getrandom"));
        assert!(!wants_custom("getrandom failed"));
        assert!(!wants_custom("custom backend"));
    }

    #[test]
    fn first_error_prefers_a_leading_error() {
        assert_eq!(first_error("x Error y\nerror: z\n"), "error: z");
        assert_eq!(first_error("note: linker ERROR\n"), "note: linker ERROR");
        assert_eq!(first_error("nothing\n"), "");
        assert_eq!(first_error(&format!("error{}", "x".repeat(300))).len(), 240);
    }
}
