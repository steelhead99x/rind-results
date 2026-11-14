// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Parity of `cargo xtask build-matrix` with `build-matrix.sh`.
//!
//! A stand-in `cargo` on `PATH` answers per crate the way the real build
//! fails or passes (a getrandom backend error fixed by the custom cfg, one
//! the cfg does not fix, getrandom named without a backend message, an
//! `Error` mid-line, no error text, a 300-character error, an unknown
//! crate). `fixtures/build-matrix/` is what `build-matrix.sh` printed and
//! logged (at df3cd46) with the same stand-in: `default` (no arguments),
//! `subset` (a target and three crates), and `rustflags` (RUSTFLAGS already
//! set, which the first attempt inherits and the retry replaces).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

const FAKE_CARGO: &str = r##"#!/bin/sh
echo "fake cargo $* RUSTFLAGS=${RUSTFLAGS-<unset>}"
c=$6
case "$c" in
tv-a-ok) echo "   Compiling $c v0.1.0"; exit 0 ;;
tv-b-gr)
  case "${RUSTFLAGS-}" in *getrandom_backend*) echo "    Finished release"; exit 0 ;; esac
  echo "error: the wasm32 and none targets are not supported by default" >&2
  echo "  --> getrandom-0.3.3/src/backends.rs:168:9" >&2
  exit 101 ;;
tv-c-gr-still)
  echo "warning: getrandom needs a Custom Backend here" >&2
  case "${RUSTFLAGS-}" in
  *getrandom_backend*) echo "error[E0425]: cannot find function \`__getrandom_v03_custom\` in this scope" >&2 ;;
  *) echo "error: getrandom: no backend" >&2 ;;
  esac
  exit 101 ;;
tv-d-gr-other) echo "error: failed to select a version for getrandom" >&2; exit 101 ;;
tv-e-warn) echo "  = note: rust-lld: Error: undefined symbol: __aeabi_memcpy" >&2; exit 101 ;;
tv-f-quiet) echo "something went wrong" >&2; exit 1 ;;
tv-g-long) printf 'error: %0300d\n' 0 >&2; exit 101 ;;
esac
echo "error: package ID specification \`$c\` did not match any packages" >&2
exit 101
"##;

const CRATES: &[&str] = &[
    "tv-a-ok",
    "tv-b-gr",
    "tv-c-gr-still",
    "tv-d-gr-other",
    "tv-e-warn",
    "tv-f-quiet",
    "tv-g-long",
];

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/build-matrix")
}

/// The stand-in cargo, written once before any test runs a build: writing
/// an executable while another thread forks can make its exec fail with
/// "text file busy" on Linux.
fn fake_bin() -> &'static Path {
    static BIN: OnceLock<PathBuf> = OnceLock::new();
    BIN.get_or_init(|| {
        let bin = Path::new(env!("CARGO_TARGET_TMPDIR")).join("build-matrix/bin");
        let _ = fs::remove_dir_all(&bin);
        fs::create_dir_all(&bin).unwrap();
        let cargo = bin.join("cargo");
        fs::write(&cargo, FAKE_CARGO).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&cargo, fs::Permissions::from_mode(0o755)).unwrap();
        }
        bin
    })
}

/// A builds workspace with empty crate directories.
fn workspace(name: &str) -> PathBuf {
    let ws = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("build-matrix")
        .join(name);
    let _ = fs::remove_dir_all(&ws);
    for c in CRATES {
        fs::create_dir_all(ws.join("crates").join(c)).unwrap();
    }
    fs::write(ws.join("crates/.hidden"), "").unwrap();
    ws
}

fn files(dir: &Path) -> Vec<(String, String)> {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(base, &p, out);
            } else {
                let rel = p.strip_prefix(base).unwrap().to_str().unwrap().to_string();
                out.push((rel, fs::read_to_string(&p).unwrap()));
            }
        }
    }
    let mut v = vec![];
    walk(dir, dir, &mut v);
    v.sort();
    v
}

fn check(name: &str, args: &[&str], rustflags: Option<&str>) {
    let (bin, ws) = (fake_bin().to_path_buf(), workspace(name));
    let path = std::env::join_paths(
        std::iter::once(bin).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_xtask"));
    cmd.arg("build-matrix")
        .arg("--dir")
        .arg(&ws)
        .args(args)
        .env("PATH", path)
        .env_remove("RUSTFLAGS");
    if let Some(f) = rustflags {
        cmd.env("RUSTFLAGS", f);
    }
    let out = cmd.output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let want = fs::read_to_string(fixtures().join(format!("{name}.stdout"))).unwrap();
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        want,
        "{name} stdout"
    );
    assert_eq!(
        files(&ws.join("logs")),
        files(&fixtures().join(name).join("logs")),
        "{name} logs"
    );
}

#[cfg(unix)]
#[test]
fn no_arguments_matches_the_script() {
    check("default", &[], None);
}

#[cfg(unix)]
#[test]
fn a_target_and_crates_match_the_script() {
    check(
        "subset",
        &[
            "riscv32imac-unknown-none-elf",
            "tv-a-ok",
            "tv-b-gr",
            "tv-z-missing",
        ],
        None,
    );
}

#[cfg(unix)]
#[test]
fn inherited_rustflags_match_the_script() {
    check(
        "rustflags",
        &["thumbv7em-none-eabihf", "tv-a-ok", "tv-b-gr"],
        Some("-Cdebuginfo=0"),
    );
}

#[test]
fn a_missing_crates_directory_is_an_error() {
    let w = Path::new(env!("CARGO_TARGET_TMPDIR")).join("build-matrix/no-crates");
    let _ = fs::remove_dir_all(&w);
    fs::create_dir_all(&w).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["build-matrix", "--dir"])
        .arg(&w)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("crates"));
}
