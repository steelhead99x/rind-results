// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Parity of `cargo xtask render-ours` with `scripts/render-ours.py`.
//!
//! Both runs draw with the vendored `upstream/rind/admin/bench-report`, so
//! they need `python3` (3.11 or later, for `tomllib`). The hosts are the
//! published `hosts/<host>/<host>.json`, laid out as the script read them
//! (`box/box.json` for xeon-vm). `fixtures/render-ours/` holds the SHA-256
//! of every file the script wrote (Python 3.12, at df3cd46), and what it
//! printed:
//!
//! - `all-hosts`: the four hosts. Its SVGs are also the published
//!   `benchmarks/*.svg`, byte for byte.
//! - `skipped`: m1pro missing and pi5 refused, with `RES` from the
//!   environment, a trailing slash, and the default `OUT`.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str) -> String {
    let p = root()
        .join("tools/xtask/tests/fixtures/render-ours")
        .join(name);
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

/// `RES` as the script read it, without `skip` hosts and with `refuse`
/// hosts marked refused.
fn res(dir: &Path, skip: &[&str], refuse: &[&str]) {
    let _ = fs::remove_dir_all(dir);
    for (host, file) in [
        ("xeon-vm", "box"),
        ("m1pro", "m1pro"),
        ("pi5", "pi5"),
        ("pixel9pro", "pixel9pro"),
    ] {
        if skip.contains(&host) {
            continue;
        }
        let src = root().join(format!("hosts/{host}/{host}.json"));
        let mut h: Value = serde_json::from_str(&fs::read_to_string(src).unwrap()).unwrap();
        if refuse.contains(&host) {
            h["status"] = "refused".into();
            h["reason"] = "busy: load 2.1".into();
        }
        fs::create_dir_all(dir.join(file)).unwrap();
        fs::write(dir.join(file).join(format!("{file}.json")), h.to_string()).unwrap();
    }
}

/// `shasum -a 256` lines for every file under `dir`, by path.
fn sums(dir: &Path) -> String {
    fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(base, &p, out);
            } else {
                let rel = p.strip_prefix(base).unwrap().to_str().unwrap().to_string();
                let hex: String = Sha256::digest(fs::read(&p).unwrap())
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect();
                out.push((rel, hex));
            }
        }
    }
    let mut v = vec![];
    walk(dir, dir, &mut v);
    v.sort();
    v.into_iter().map(|(p, h)| format!("{h}  {p}\n")).collect()
}

fn xtask(args: &[&str], env: &[(&str, &Path)]) -> (String, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_xtask"));
    cmd.args(args).env_remove("RES").env_remove("OUT");
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("run xtask");
    let (so, se) = (
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
    );
    assert!(out.status.success(), "xtask {args:?}: {se}");
    (so, se)
}

#[test]
fn all_hosts_match_the_script_and_the_published_svgs() {
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR")).join("render-ours-all");
    let (r, out) = (tmp.join("res"), tmp.join("out"));
    res(&r, &[], &[]);
    let rind = root().join("upstream/rind");
    let (so, se) = xtask(
        &[
            "render-ours",
            "--rind",
            rind.to_str().unwrap(),
            "--res",
            r.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ],
        &[],
    );
    assert_eq!(se, "");
    assert_eq!(so, fixture("all-hosts.stdout"));
    assert_eq!(sums(&out), fixture("all-hosts.sha256"));
    let mut n = 0;
    for e in fs::read_dir(out.join("benchmarks")).unwrap() {
        let p = e.unwrap().path();
        let published = root().join("benchmarks").join(p.file_name().unwrap());
        assert_eq!(
            fs::read(&p).unwrap(),
            fs::read(&published).unwrap(),
            "{}",
            p.display()
        );
        n += 1;
    }
    assert_eq!(n, 12);
}

#[test]
fn skipped_hosts_match_the_script() {
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR")).join("render-ours-skip");
    let r = tmp.join("res");
    res(&r, &["m1pro"], &["pi5"]);
    // A stale file in OUT is removed first.
    fs::create_dir_all(r.join("render")).unwrap();
    fs::write(r.join("render/stale.txt"), "old").unwrap();
    let rind = root().join("upstream/rind");
    let slash = PathBuf::from(format!("{}/", r.display()));
    let (so, se) = xtask(
        &["render-ours", "--rind", rind.to_str().unwrap()],
        &[("RES", &slash)],
    );
    assert_eq!(se, fixture("skipped.stderr"));
    assert_eq!(so, fixture("skipped.stdout"));
    assert_eq!(sums(&r.join("render")), fixture("skipped.sha256"));
}
