// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

use std::fs;
use std::path::Path;
use std::process::Command;

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let dest = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &dest);
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }
}

#[test]
fn duplicated_summary_rows_cannot_hide_a_missing_result() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let relative = "results/pi5-nofan-1p5ghz/a6577cb4a4b48af06385f8d748f3892c07cc229e";
    let temp = Path::new(env!("CARGO_TARGET_TMPDIR")).join("security-duplicate-summary");
    let _ = fs::remove_dir_all(&temp);
    let capture = temp.join(relative);
    copy_tree(&root.join(relative), &capture);
    let check = || {
        Command::new(env!("CARGO_BIN_EXE_verify"))
            .arg("--dir")
            .arg(&capture)
            .output()
            .unwrap()
    };
    assert!(
        check().status.success(),
        "published fixture must pass first"
    );
    let json = capture.join("pi5-nofan-1p5ghz.json");
    let mut host: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&json).unwrap()).unwrap();
    host["results"][0] = host["results"][1].clone();
    fs::write(json, host.to_string()).unwrap();
    let result = check();
    assert!(
        !result.status.success(),
        "accepted a duplicated summary row"
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("duplicate result"));
    fs::remove_dir_all(temp).unwrap();
}
