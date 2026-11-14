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

/// A bench-report with no HOSTS attribute, as rind draws hosts after the
/// CPU-slug change. The published four stay first when they exist, and any
/// other filesystem-safe id is included. `tiers.json` and a directory that is
/// only an alias (`box`) are not hosts.
#[test]
fn cpu_slug_hosts_are_drawn_with_the_published_ones() {
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR")).join("render-ours-slug");
    let _ = fs::remove_dir_all(&tmp);
    let rind = tmp.join("rind");
    let res = tmp.join("res");
    let out = tmp.join("out");
    fs::create_dir_all(rind.join("admin")).unwrap();
    fs::create_dir_all(rind.join("benchmarks/data")).unwrap();
    fs::write(rind.join("admin/bench-report"), STUB_BENCH_REPORT).unwrap();
    for name in ["nixos", "desktop", "nixos2", "kixtop", "ovh-iad", "ovh-sea"] {
        fs::write(
            rind.join("benchmarks/data").join(format!("{name}.json")),
            host_body(name, Some("ok"), None),
        )
        .unwrap();
    }
    fs::write(
        rind.join("benchmarks/data/amd-ryzen-9-7950x.json"),
        host_body("amd-ryzen-9-7950x", Some("ok"), Some("repo slug")),
    )
    .unwrap();
    fs::write(rind.join("benchmarks/data/tiers.json"), r#"{"tiers":[]}"#).unwrap();
    fs::create_dir_all(res.join("box")).unwrap();
    fs::write(
        res.join("box/box.json"),
        host_body("xeon-vm", Some("ok"), None),
    )
    .unwrap();
    fs::create_dir_all(res.join("intel-core-ultra-9-275hx")).unwrap();
    fs::write(
        res.join("intel-core-ultra-9-275hx/intel-core-ultra-9-275hx.json"),
        host_body("intel-core-ultra-9-275hx", Some("ok"), Some("nested slug")),
    )
    .unwrap();
    fs::write(
        res.join("apple-m3-max.json"),
        host_body("apple-m3-max", Some("ok"), Some("flat json")),
    )
    .unwrap();
    fs::create_dir_all(res.join("amd-ryzen-9-5900xt")).unwrap();
    fs::write(
        res.join("amd-ryzen-9-5900xt/amd-ryzen-9-5900xt.json"),
        host_body("amd-ryzen-9-5900xt", Some("refused"), Some("busy")),
    )
    .unwrap();
    fs::create_dir_all(res.join("Jetson")).unwrap();
    fs::write(
        res.join("Jetson/Jetson.json"),
        host_body("Jetson", Some("ok"), None),
    )
    .unwrap();
    fs::create_dir_all(res.join("_bad")).unwrap();
    fs::write(
        res.join("_bad/_bad.json"),
        host_body("_bad", Some("ok"), None),
    )
    .unwrap();
    fs::write(res.join("tiers.json"), r#"{"tiers":[]}"#).unwrap();
    fs::write(
        res.join("Has Space.json"),
        host_body("has space", Some("ok"), None),
    )
    .unwrap();

    let (so, se) = xtask(
        &[
            "render-ours",
            "--rind",
            rind.to_str().unwrap(),
            "--res",
            res.to_str().unwrap(),
            "--out",
            out.to_str().unwrap(),
        ],
        &[],
    );
    assert!(
        se.contains("skip amd-ryzen-9-5900xt: refused:"),
        "stderr: {se}"
    );
    assert!(
        so.starts_with(
            "ours: ['xeon-vm', 'apple-m3-max', 'intel-core-ultra-9-275hx'] repo: ['nixos', 'desktop', 'nixos2', 'kixtop', 'ovh-iad', 'ovh-sea', 'amd-ryzen-9-7950x']\n"
        ),
        "stdout: {so}"
    );
    assert!(!so.contains("box"));
    assert!(!so.contains("Jetson"));
    assert!(!so.contains("tiers"));
    assert!(!so.contains("_bad"));
    let md = fs::read_to_string(out.join("STATS.md")).unwrap();
    assert!(md.contains("### `xeon-vm`"));
    assert!(md.contains("Cloud VM: 8 vCPUs"));
    assert!(md.contains("apple-m3-max: flat json"));
    assert!(md.contains("intel-core-ultra-9-275hx: nested slug"));
    assert!(md.contains("| x25519-ecdh | `apple-m3-max` | 1.25× (ring) |"));
    assert!(md.contains("| x25519-ecdh | `intel-core-ultra-9-275hx` | 1.50× (ring) |"));
    let apple = md.find("apple-m3-max").unwrap();
    let intel = md.find("intel-core-ultra-9-275hx").unwrap();
    assert!(
        apple < intel,
        "slug hosts stay in name order after the published ones"
    );
    let overview = fs::read_to_string(out.join("benchmarks/overview-all.svg")).unwrap();
    assert_eq!(
        overview,
        "nixos,desktop,nixos2,kixtop,ovh-iad,ovh-sea,amd-ryzen-9-7950x,xeon-vm,apple-m3-max,intel-core-ultra-9-275hx"
    );
    assert!(!overview.contains("box"));
    assert!(!overview.contains("tiers"));
    assert!(!overview.contains("5900xt"));
}

fn host_body(host: &str, status: Option<&str>, note: Option<&str>) -> String {
    let status = status.unwrap_or("ok");
    let note = match note {
        Some(n) if status == "ok" => format!(r#","note":"{n}""#),
        Some(n) => format!(r#","reason":"{n}""#),
        None => String::new(),
    };
    format!(
        r#"{{"status":"{status}","host":"{host}","results":[],"started":"2026-11-13T00:00:00Z","commit":"0123456789abcdef0123456789abcdef01234567","os":"Linux","cpu":{{"model":"{host}","arch":"x86_64"}},"toolchain":{{"rustc":"rustc 1.98.1"}}{note}}}"#
    )
}

const STUB_BENCH_REPORT: &str = r###"
REPORT = "STATS.md"
REPORT_DIR = "benchmarks"
COLUMNS = {"rind": "rind", "ring": "ring"}
OPERATIONS = ["x25519-ecdh/{lib}"]
VARIANTS = []
SLOWER = 1.1
GENERATED = "<!-- Generated by `admin/bench-report render` from benchmarks/data, benchmarks/verified.toml, and benchmarks/gaps.toml. -->"
PINNED = "Each host builds with the pinned toolchain in the Nix dev shell, and runs `admin/bench-report` only when idle. Times are criterion's estimates."

def load_verified():
    return []

def label(row):
    return row

def row_name(op):
    return op.replace("/{lib}", "")

def gaps_of(hosts, tiers):
    out = {}
    if "apple-m3-max" in hosts:
        out[("x25519-ecdh", "apple-m3-max")] = (1.25, "ring")
    if "intel-core-ultra-9-275hx" in hosts:
        out[("x25519-ecdh", "intel-core-ultra-9-275hx")] = (1.5, "ring")
    return out

def overview_svg(hosts, verified, theme):
    return ",".join(hosts.keys())

def report(hosts, tiers, verified, gaps):
    lines = ["# Benchmarks", "", GENERATED, "", PINNED, ""]
    for name, host in hosts.items():
        lines.append("#" * 3 + " `" + name + "`")
        note = host.get("note")
        if note:
            lines.append(name + ": " + note)
    lines += ["", "## CPU tiers", "", "x", "", "## Verified code", "", "y", ""]
    return {REPORT: "\n".join(lines)}
"###;
