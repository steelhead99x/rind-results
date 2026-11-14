// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn render_ours_cannot_delete_its_input_directory() {
    let dir = temp_dir("security-render-input");
    let res = dir.join("results");
    fs::create_dir_all(res.join("pi5")).unwrap();
    let sentinel = res.join("pi5/pi5.json");
    fs::write(&sentinel, "preserve raw input").unwrap();
    for out in [&res, &res.join("pi5"), &dir] {
        let result = Command::new(env!("CARGO_BIN_EXE_xtask"))
            .arg("render-ours")
            .arg("--rind")
            .arg(root().join("upstream/rind"))
            .arg("--res")
            .arg(&res)
            .arg("--out")
            .arg(out)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert_eq!(fs::read_to_string(&sentinel).unwrap(), "preserve raw input");
    }
}

#[test]
fn host_report_cannot_write_a_host_outside_the_publish_directory() {
    let dir = temp_dir("security-host-report");
    let publish = dir.join("publish");
    fs::create_dir_all(publish.join("raw")).unwrap();
    fs::write(
        publish.join("raw/run-1.json"),
        serde_json::json!({"status":"ok", "host":"../escape", "results":[]}).to_string(),
    )
    .unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("host-report")
        .arg("--rind")
        .arg(root().join("upstream/rind"))
        .arg(&publish)
        .output()
        .unwrap();
    assert!(
        !result.status.success(),
        "accepted a host with parent traversal"
    );
    assert!(!dir.join("escape.json").exists());
}

#[cfg(unix)]
#[test]
fn md2png_keeps_the_browser_sandbox() {
    use std::os::unix::fs::PermissionsExt;
    let dir = temp_dir("security-md2png");
    fs::write(dir.join("input.md"), "# Test\n").unwrap();
    let fixture = dir.join("fixture.png");
    let mut enc = png::Encoder::new(fs::File::create(&fixture).unwrap(), 1, 1);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()
        .unwrap()
        .write_image_data(&[255; 3])
        .unwrap();
    let browser = dir.join("browser");
    fs::write(
        &browser,
        "#!/bin/sh\nfor arg do\n case \"$arg\" in\n --no-sandbox) exit 23 ;;\n --screenshot=*) cp \"$SECURITY_PNG\" \"${arg#--screenshot=}\" ;;\n esac\ndone\n",
    )
    .unwrap();
    fs::set_permissions(&browser, fs::Permissions::from_mode(0o755)).unwrap();
    let out = dir.join("output.png");
    let result = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("md2png")
        .arg(dir.join("input.md"))
        .arg(&out)
        .env("CHROME", browser)
        .env("SECURITY_PNG", fixture)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(out.is_file());
}
