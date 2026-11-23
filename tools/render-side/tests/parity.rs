// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_render-side"))
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn run(out: &Path) -> std::process::Output {
    Command::new(bin())
        .arg("--rind")
        .arg(fixture("rind-stub"))
        .arg("--out")
        .arg(out)
        .arg("--note")
        .arg("m1pro")
        .arg(fixture("notes/m1pro.note"))
        .arg(fixture("hosts/pi5.json"))
        .arg(fixture("hosts/m1pro.json"))
        .output()
        .expect("run render-side")
}

#[test]
fn fixture_matches_python_byte_for_byte() {
    let tmp = std::env::temp_dir().join(format!("render-side-parity-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let out = run(&tmp);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(stderr, "", "stderr: {stderr}");
    assert!(out.status.success(), "status {:?}", out.status);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let sheet = tmp.join("BENCHMARKS.md");
    // Python Path.resolve() follows /tmp -> /private/tmp on macOS.
    let expected_path = render_side::resolve_path(&sheet).unwrap();
    assert_eq!(stdout, format!("{}\n", expected_path.display()));
    let got_md = std::fs::read_to_string(&sheet).unwrap();
    let expected_md = std::fs::read_to_string(fixture("expected/BENCHMARKS.md")).unwrap();
    assert_eq!(got_md, expected_md);
    let got_svg = std::fs::read_to_string(tmp.join("benchmarks/overview.svg")).unwrap();
    let expected_svg = std::fs::read_to_string(fixture("expected/overview.svg")).unwrap();
    assert_eq!(got_svg, expected_svg);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn usage_exits_2_without_args() {
    let out = Command::new(bin()).output().expect("run render-side");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert!(
        String::from_utf8_lossy(&out.stderr).starts_with("usage: render-side "),
        "stderr {:?}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn cpu_slug_host_is_drawn_without_a_hosts_attribute() {
    let tmp = std::env::temp_dir().join(format!("render-side-slug-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let slug = tmp.join("intel-core-ultra-9-275hx.json");
    std::fs::write(
        &slug,
        r#"{
  "status": "ok",
  "host": "intel-core-ultra-9-275hx",
  "results": [],
  "started": "2026-11-13T00:00:00Z",
  "commit": "0123456789abcdef0123456789abcdef01234567",
  "os": "Linux",
  "cpu": {"model": "Intel(R) Core(TM) Ultra 9 275HX", "arch": "x86_64", "slug": "intel-core-ultra-9-275hx"},
  "toolchain": {"rustc": "rustc 1.98.1"}
}
"#,
    )
    .unwrap();
    let out_dir = tmp.join("out");
    let out = Command::new(bin())
        .arg("--rind")
        .arg(fixture("rind-any-host"))
        .arg("--out")
        .arg(&out_dir)
        .arg("--note")
        .arg("intel-core-ultra-9-275hx")
        .arg(fixture("notes/m1pro.note"))
        .arg(fixture("hosts/pi5.json"))
        .arg(&slug)
        .output()
        .expect("run render-side");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(stderr, "", "stderr: {stderr}");
    assert!(out.status.success(), "status {:?}", out.status);
    let sheet = std::fs::read_to_string(out_dir.join("BENCHMARKS.md")).unwrap();
    assert!(sheet.contains("Side sheet for `pi5`, `intel-core-ultra-9-275hx`."));
    assert!(
        sheet.contains("intel-core-ultra-9-275hx: Measured on a laptop that was otherwise idle.")
    );
    assert!(sheet.contains("| X25519 ECDH | `intel-core-ultra-9-275hx` | 1.50× (ring) |"));
    assert!(!sheet.contains("HOSTS"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn a_path_is_not_a_host_name() {
    let tmp = std::env::temp_dir().join(format!("render-side-bad-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let bad = tmp.join("bad.json");
    std::fs::write(
        &bad,
        r#"{
  "status": "ok",
  "host": "../x",
  "results": [],
  "started": "2026-11-13T00:00:00Z",
  "commit": "0123456789abcdef0123456789abcdef01234567",
  "os": "Linux",
  "cpu": {"model": "x", "arch": "x86_64"},
  "toolchain": {"rustc": "rustc 1.98.1"}
}
"#,
    )
    .unwrap();
    let out = Command::new(bin())
        .arg("--rind")
        .arg(fixture("rind-stub"))
        .arg("--out")
        .arg(tmp.join("out"))
        .arg(&bad)
        .output()
        .expect("run render-side");
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("is not a filesystem-safe name"),
        "stderr: {stderr}"
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn out_inside_rind_matches_python() {
    let out = Command::new(bin())
        .arg("--rind")
        .arg(fixture("rind-stub"))
        .arg("--out")
        .arg(fixture("rind-stub").join("inside"))
        .arg(fixture("hosts/pi5.json"))
        .output()
        .expect("run render-side");
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        "render-side: --out is inside the rind checkout; pick a directory outside it\n"
    );
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
}
