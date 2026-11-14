// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn missing_parent_cannot_bypass_the_checkout_write_guard() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("render-security-parent");
    let _ = fs::remove_dir_all(&root);
    let rind = root.join("rind");
    fs::create_dir_all(rind.join("admin")).unwrap();
    fs::copy(
        fixture("rind-stub/admin/bench-report"),
        rind.join("admin/bench-report"),
    )
    .unwrap();
    let output = root.join("missing/../rind/generated");
    let result = Command::new(env!("CARGO_BIN_EXE_render-side"))
        .arg("--rind")
        .arg(&rind)
        .arg("--out")
        .arg(output)
        .arg(fixture("hosts/pi5.json"))
        .output()
        .unwrap();
    assert!(!result.status.success(), "wrote into the rind checkout");
    assert!(!rind.join("generated").exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn host_names_cannot_be_used_as_chart_paths() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("render-security-host");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let mut host: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(fixture("hosts/pi5.json")).unwrap()).unwrap();
    for name in ["../escape", "a/b", "a\\b", ".", ".."] {
        host["host"] = name.into();
        fs::write(root.join("host.json"), host.to_string()).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_render-side"))
            .arg("--rind")
            .arg(fixture("rind-stub"))
            .arg("--out")
            .arg(root.join("out"))
            .arg(root.join("host.json"))
            .output()
            .unwrap();
        assert!(!result.status.success(), "accepted unsafe host {name:?}");
    }
    fs::remove_dir_all(root).unwrap();
}
