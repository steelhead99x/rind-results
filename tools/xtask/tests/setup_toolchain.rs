// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask setup-toolchain` runs the commands `tools/setup-toolchain.sh`
//! ran, in the same order, with the same arguments, and leaves the same files
//! and output. Every outside tool (curl, rustup, cargo, unzip, uname, espup) is
//! a stub that logs its arguments, so nothing is downloaded or installed.
//! The expected transcripts came from the old script with these same stubs
//! (see `fixtures/setup-toolchain/README.md`).
#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Logs `NAME ARGS` to $LOG, plus the directory for unzip.
const LOGGER: &str = r#"#!/bin/sh
echo "$(basename "$0") $*" >> "$LOG"
"#;

/// curl: `-o FILE URL` writes a logging stub to FILE; the rustup installer
/// URL prints an installer that logs and puts a rustup stub in ~/.cargo/bin.
const CURL: &str = r#"#!/bin/sh
echo "curl $*" >> "$LOG"
out=
while [ $# -gt 0 ]; do
    if [ "$1" = -o ]; then out=$2; shift; fi
    url=$1
    shift
done
if [ -n "$out" ]; then
    cp "$STUBS/logger" "$out"
elif [ "$url" = https://sh.rustup.rs ]; then
    echo 'echo "rustup-init $*" >> "$LOG"'
    echo 'mkdir -p "$HOME/.cargo/bin" && cp "$STUBS/logger" "$HOME/.cargo/bin/rustup" && chmod +x "$HOME/.cargo/bin/rustup"'
fi
"#;

const UNZIP: &str = r#"#!/bin/sh
echo "unzip $* (in $(pwd))" >> "$LOG"
cp "$STUBS/logger" espflash
chmod -x espflash
"#;

const UNAME: &str = r#"#!/bin/sh
echo "uname $*" >> "$LOG"
echo x86_64
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

/// What one `setup` command does, twice in a row (the second run finds
/// espup and espflash already there), with `$T` for the scratch directory.
fn transcript(name: &str, with_rustup: bool, setup: &dyn Fn(&Path) -> Command) -> String {
    let t = scratch(name);
    let (bin, stubs, home, hw) = (
        t.join("bin"),
        t.join("stubs"),
        t.join("home"),
        t.join("rind-hardware"),
    );
    for d in [&bin, &stubs, &home, &hw] {
        std::fs::create_dir_all(d).unwrap();
    }
    write_exe(&stubs.join("logger"), LOGGER);
    write_exe(&bin.join("curl"), CURL);
    write_exe(&bin.join("unzip"), UNZIP);
    write_exe(&bin.join("uname"), UNAME);
    write_exe(&bin.join("cargo"), LOGGER);
    if with_rustup {
        write_exe(&bin.join("rustup"), LOGGER);
    }
    let mut out = String::new();
    for round in 1..=2 {
        let o = setup(&hw)
            .env_clear()
            .env("HOME", &home)
            .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
            .env("LOG", t.join("log"))
            .env("STUBS", &stubs)
            .output()
            .unwrap();
        out += &format!("== run {round}: {}\n", o.status.code().unwrap_or(-1));
        out += &String::from_utf8_lossy(&o.stdout);
        out += &String::from_utf8_lossy(&o.stderr);
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

fn xtask(hw: &Path) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_xtask"));
    c.arg("setup-toolchain").arg(hw);
    c
}

fn check(case: &str, with_rustup: bool) {
    let rustup_elsewhere = ["/usr/bin/rustup", "/bin/rustup"]
        .iter()
        .any(|p| Path::new(p).exists());
    if !with_rustup && rustup_elsewhere {
        eprintln!("skipping {case}: this host has rustup in /usr/bin or /bin");
        return;
    }
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/setup-toolchain");
    let want_path = fixtures.join(format!("{case}.txt"));
    // SETUP_TOOLCHAIN_SCRIPT=<old setup-toolchain.sh> runs the old script
    // instead, which is how the fixtures were made.
    let got = match std::env::var_os("SETUP_TOOLCHAIN_SCRIPT") {
        Some(script) => {
            let got = transcript(case, with_rustup, &|hw| {
                let mut c = Command::new("/bin/sh");
                c.arg(&script).arg(hw);
                c
            });
            std::fs::write(&want_path, &got).unwrap();
            got
        }
        None => transcript(case, with_rustup, &xtask),
    };
    let want = std::fs::read_to_string(&want_path).unwrap();
    assert_eq!(got, want, "{case}");
}

#[test]
fn matches_the_script_with_rustup() {
    check("with-rustup", true);
}

#[test]
fn matches_the_script_installing_rustup() {
    check("installs-rustup", false);
}
