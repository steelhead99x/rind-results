// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! A cached standalone verifier must not bypass the CI gate's Cargo build.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn verifier_failure(name: &str, failing_args: &str) {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let xtask = dir.join("xtask");
    fs::copy(env!("CARGO_BIN_EXE_xtask"), &xtask).unwrap();
    // An old standalone verifier would accept the tree. The current checkout's
    // Cargo verification fails, as it would after adding a failing check.
    executable(&dir.join("verify"), "#!/bin/sh\nexit 0\n");
    let cargo = dir.join("cargo");
    executable(
        &cargo,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$CI_TEST_LOG\"\nif [ \"$*\" = \"$CI_TEST_FAIL\" ]; then exit 42; fi\nexit 0\n",
    );
    let log = dir.join("cargo.log");
    let out = Command::new(xtask)
        .arg("ci")
        .env("CARGO", cargo)
        .env("CI_TEST_LOG", &log)
        .env("CI_TEST_FAIL", failing_args)
        .output()
        .unwrap();
    let calls = fs::read_to_string(log).unwrap();
    assert!(
        calls.lines().any(|line| line == failing_args),
        "CI bypassed Cargo verification: {calls}"
    );
    assert!(!out.status.success(), "CI ignored a failed verification");
    assert!(
        String::from_utf8_lossy(&out.stderr).contains(&format!("cargo {failing_args} failed")),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !calls.lines().any(|line| line.starts_with("deny")),
        "CI continued after verification failed: {calls}"
    );
}

#[test]
fn ci_rebuilds_the_verifier_even_with_a_cached_sibling() {
    verifier_failure("ci-cached-verifier", "verify");
}

#[test]
fn ci_uses_cargo_for_result_trees_and_stops_on_a_mismatch() {
    verifier_failure("ci-cached-verifier-dir", "verify --dir results");
}
