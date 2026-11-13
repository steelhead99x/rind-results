// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask setup-toolchain` runs the commands `tools/setup-toolchain.sh`
//! ran, in the same order, with the same arguments, checks the same SHA-256
//! pins, and leaves the same files and output. Every outside tool (curl,
//! rustup, rustup-init, cargo, unzip, uname, espup) is a stub that logs its
//! arguments, so nothing is downloaded or installed. The expected transcripts
//! came from the old script with these same stubs (see
//! `fixtures/setup-toolchain/README.md`).
#![cfg(unix)]

use sha2::{Digest, Sha256};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Logs `NAME ARGS` to $LOG. As rustup-init it also installs a rustup stub.
const LOGGER: &str = r#"#!/bin/sh
echo "$(basename "$0") $*" >> "$LOG"
if [ "$(basename "$0")" = rustup-init ]; then
    mkdir -p "$HOME/.cargo/bin"
    cp "$STUBS/logger" "$HOME/.cargo/bin/rustup"
    chmod +x "$HOME/.cargo/bin/rustup"
fi
"#;

/// curl: logs, then writes the logging stub to the `-o` file.
const CURL: &str = r#"#!/bin/sh
echo "curl $*" >> "$LOG"
while [ $# -gt 0 ]; do
    if [ "$1" = -o ]; then cp "$STUBS/logger" "$2"; shift; fi
    shift
done
"#;

const UNZIP: &str = r#"#!/bin/sh
echo "unzip $* (in $(pwd))" >> "$LOG"
cp "$STUBS/logger" espflash
chmod -x espflash
"#;

const UNAME: &str = r#"#!/bin/sh
echo "uname $*" >> "$LOG"
echo "$UNAME_M"
"#;

/// The old script needed sha256sum; macOS has shasum.
const SHA256SUM: &str = r#"#!/bin/sh
if [ -x /usr/bin/sha256sum ]; then exec /usr/bin/sha256sum "$@"; fi
exec shasum -a 256 "$@"
"#;

fn write_exe(p: &Path, body: &str) {
    std::fs::write(p, body).unwrap();
    std::fs::set_permissions(p, std::fs::Permissions::from_mode(0o755)).unwrap();
}

fn scratch(name: &str) -> PathBuf {
    let t = std::env::temp_dir().join(format!(
        "xtask-setup-toolchain-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&t);
    std::fs::create_dir_all(&t).unwrap();
    t
}

struct Case {
    name: &'static str,
    with_rustup: bool,
    arch: &'static str,
    /// The pin whose hash is wrong, if any.
    bad_pin: Option<&'static str>,
}

/// Pins for the stub downloads (all the logging stub), for x86_64.
fn checksums(bad_pin: Option<&str>) -> String {
    let good: String = Sha256::digest(LOGGER.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let mut out = String::from("# test pins\n");
    for name in [
        "rustup-init-1.29.3-x86_64-unknown-linux-gnu",
        "espup-0.19.0-x86_64-unknown-linux-gnu",
        "espflash-4.7.0-x86_64-unknown-linux-musl.zip",
    ] {
        let hash = if bad_pin == Some(name) {
            "0".repeat(64)
        } else {
            good.clone()
        };
        out += &format!("{hash}  {name}\n");
    }
    out
}

/// What one `setup` command does, twice in a row (the second run finds
/// what the first left), with `$T` for the scratch directory. stderr is left
/// out: the error messages are worded differently.
fn transcript(case: &Case, setup: &dyn Fn(&Path, &Path) -> Command) -> String {
    let t = scratch(case.name);
    let (bin, stubs, home, hw, tools) = (
        t.join("bin"),
        t.join("stubs"),
        t.join("home"),
        t.join("rind-hardware"),
        t.join("tools"),
    );
    for d in [&bin, &stubs, &home, &hw, &tools] {
        std::fs::create_dir_all(d).unwrap();
    }
    let sums = tools.join("checksums.sha256");
    std::fs::write(&sums, checksums(case.bad_pin)).unwrap();
    write_exe(&stubs.join("logger"), LOGGER);
    write_exe(&bin.join("curl"), CURL);
    write_exe(&bin.join("unzip"), UNZIP);
    write_exe(&bin.join("uname"), UNAME);
    write_exe(&bin.join("sha256sum"), SHA256SUM);
    write_exe(&bin.join("cargo"), LOGGER);
    if case.with_rustup {
        write_exe(&bin.join("rustup"), LOGGER);
    }
    let mut out = String::new();
    for round in 1..=2 {
        let o = setup(&hw, &sums)
            .env_clear()
            .env("HOME", &home)
            .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
            .env("LOG", t.join("log"))
            .env("STUBS", &stubs)
            .env("UNAME_M", case.arch)
            .output()
            .unwrap();
        out += &format!("== run {round}: exit {}\n", o.status.code().unwrap_or(-1));
        out += &String::from_utf8_lossy(&o.stdout);
    }
    out += "== log\n";
    out += &std::fs::read_to_string(t.join("log")).unwrap_or_default();
    out += "== files\n";
    let mut files = Vec::new();
    for d in [hw.join(".tools"), home.join(".cargo/bin")] {
        for e in std::fs::read_dir(&d).into_iter().flatten() {
            let p = e.unwrap().path();
            let exe = p.metadata().unwrap().permissions().mode() & 0o111 != 0;
            files.push(format!("{} {}", p.display(), if exe { "x" } else { "-" }));
        }
    }
    files.sort();
    out += &(files.join("\n") + "\n");
    let _ = std::fs::remove_dir_all(&t);
    out.replace(&t.display().to_string(), "$T")
}

fn xtask(hw: &Path, sums: &Path) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_xtask"));
    c.arg("setup-toolchain")
        .arg(hw)
        .arg("--checksums")
        .arg(sums);
    c
}

fn check(case: Case) {
    let rustup_elsewhere = ["/usr/bin/rustup", "/bin/rustup"]
        .iter()
        .any(|p| Path::new(p).exists());
    if !case.with_rustup && rustup_elsewhere {
        eprintln!(
            "skipping {}: this host has rustup in /usr/bin or /bin",
            case.name
        );
        return;
    }
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/setup-toolchain");
    let want_path = fixtures.join(format!("{}.txt", case.name));
    // SETUP_TOOLCHAIN_SCRIPT=<old setup-toolchain.sh> runs the old script
    // instead (copied next to the test pins, where it looks for them), which
    // is how the fixtures were made.
    let got = match std::env::var_os("SETUP_TOOLCHAIN_SCRIPT") {
        Some(script) => {
            let got = transcript(&case, &|hw, sums| {
                let copy = sums.with_file_name("setup-toolchain.sh");
                std::fs::copy(&script, &copy).unwrap();
                let mut c = Command::new("/bin/sh");
                c.arg(&copy).arg(hw);
                c
            });
            std::fs::write(&want_path, &got).unwrap();
            got
        }
        None => transcript(&case, &xtask),
    };
    let want = std::fs::read_to_string(&want_path).unwrap();
    assert_eq!(got, want, "{}", case.name);
}

#[test]
fn matches_the_script_with_rustup() {
    check(Case {
        name: "with-rustup",
        with_rustup: true,
        arch: "x86_64",
        bad_pin: None,
    });
}

#[test]
fn matches_the_script_installing_rustup() {
    check(Case {
        name: "installs-rustup",
        with_rustup: false,
        arch: "x86_64",
        bad_pin: None,
    });
}

#[test]
fn deletes_a_download_that_does_not_match_its_pin() {
    check(Case {
        name: "espup-mismatch",
        with_rustup: true,
        arch: "x86_64",
        bad_pin: Some("espup-0.19.0-x86_64-unknown-linux-gnu"),
    });
}

#[test]
fn refuses_an_arch_without_pins() {
    check(Case {
        name: "unknown-arch",
        with_rustup: true,
        arch: "armv7l",
        bad_pin: None,
    });
}

#[test]
fn tools_checksums_pin_every_download() {
    let sums =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../checksums.sha256"))
            .unwrap();
    for arch in ["x86_64", "aarch64"] {
        for name in [
            format!("rustup-init-1.29.3-{arch}-unknown-linux-gnu"),
            format!("espup-0.19.0-{arch}-unknown-linux-gnu"),
            format!("espflash-4.7.0-{arch}-unknown-linux-musl.zip"),
        ] {
            let hash = sums
                .lines()
                .filter(|l| !l.starts_with('#'))
                .find_map(|l| {
                    let mut f = l.split_whitespace();
                    let (h, n) = (f.next()?, f.next()?);
                    (n == name).then_some(h)
                })
                .unwrap_or_else(|| panic!("no pin for {name}"));
            assert!(
                hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit()),
                "{name}: {hash}"
            );
        }
    }
}
