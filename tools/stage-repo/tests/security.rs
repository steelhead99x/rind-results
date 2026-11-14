// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stage-security-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&dir).unwrap();
    dir
}

fn stage(root: &Path, host: &str) -> std::process::Output {
    let input = root.join("input.json");
    fs::write(&input, serde_json::json!({"host": host}).to_string()).unwrap();
    Command::new(env!("CARGO_BIN_EXE_stage-repo"))
        .arg("--out")
        .arg(root.join("out"))
        .arg("--json")
        .arg(input)
        .output()
        .unwrap()
}

#[test]
fn host_names_cannot_escape_the_staging_directory() {
    let root = temp_dir("host");
    let absolute = root.join("absolute");
    for host in [
        "../escaped",
        "a/b",
        "a\\b",
        ".",
        "..",
        absolute.to_str().unwrap(),
    ] {
        let result = stage(&root, host);
        assert!(!result.status.success(), "accepted unsafe host {host:?}");
        assert!(!root.join("escaped").exists(), "wrote outside --out");
        assert!(!absolute.exists(), "accepted an absolute host path");
    }
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn render_symlinks_cannot_publish_files_outside_the_input_tree() {
    use std::os::unix::fs::symlink;
    let root = temp_dir("render-link");
    let render = root.join("render");
    fs::create_dir_all(render.join("benchmarks")).unwrap();
    fs::write(render.join("BENCHMARKS.md"), "# Report\n").unwrap();
    fs::write(root.join("private.txt"), "private audit sentinel").unwrap();
    symlink(root.join("private.txt"), render.join("benchmarks/leak.txt")).unwrap();
    let input = root.join("input.json");
    fs::write(&input, "{\"host\":\"sample\"}").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_stage-repo"))
        .arg("--out")
        .arg(root.join("out"))
        .arg("--json")
        .arg(input)
        .arg("--render")
        .arg(render)
        .output()
        .unwrap();
    assert!(!result.status.success(), "followed an input symlink");
    assert!(!root.join("out/benchmarks/leak.txt").exists());
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn output_symlinks_cannot_redirect_staged_writes() {
    use std::os::unix::fs::symlink;
    let root = temp_dir("output-link");
    fs::create_dir_all(root.join("out/sample")).unwrap();
    let sentinel = root.join("sentinel");
    fs::write(&sentinel, "keep").unwrap();
    symlink(&sentinel, root.join("out/sample/NOTES.md")).unwrap();
    let result = stage(&root, "sample");
    assert!(!result.status.success(), "followed an output symlink");
    assert_eq!(fs::read_to_string(&sentinel).unwrap(), "keep");
    fs::remove_dir_all(root).unwrap();
}
