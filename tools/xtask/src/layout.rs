// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Where a new publish is written.
//!
//! `results/<host>/<upstream-sha>/` holds one measured commit:
//! `<host>.json`, `meta.json`, `NOTES.md`, `run.log`, `raw/`, `RESULTS.md`.
//! An A/B or a diagnostic is `results/<host>/<upstream-sha>-<label>/`.
//! Trees published before this layout stay where they are; see `results/README.md`.

use std::path::{Path, PathBuf};

pub fn host_id(host: &str) -> Result<String, String> {
    if host.is_empty()
        || !host
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        || host.starts_with('-')
        || host.ends_with('-')
    {
        return Err(format!(
            "host id {host:?} is not a lowercase results directory name"
        ));
    }
    Ok(host.to_string())
}

/// The revision directory: the upstream sha as given, or `<sha>-<label>`.
pub fn revision(upstream_sha: &str, label: Option<&str>) -> Result<String, String> {
    let sha = upstream_sha.trim();
    if !(7..=40).contains(&sha.len())
        || !sha
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        return Err(format!(
            "upstream sha {upstream_sha:?} is not 7 to 40 lower-case hex digits"
        ));
    }
    match label.map(str::trim).filter(|s| !s.is_empty()) {
        None => Ok(sha.to_string()),
        Some(label) => {
            if !label
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                || label.starts_with('-')
                || label.ends_with('-')
            {
                return Err(format!(
                    "label {label:?} is not a lowercase directory suffix"
                ));
            }
            Ok(format!("{sha}-{label}"))
        }
    }
}

pub fn publish_dir(
    repo: &Path,
    host: &str,
    upstream_sha: &str,
    label: Option<&str>,
) -> Result<PathBuf, String> {
    Ok(repo
        .join("results")
        .join(host_id(host)?)
        .join(revision(upstream_sha, label)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_dir_is_host_then_sha_then_optional_label() {
        let repo = Path::new("/repo");
        assert_eq!(
            publish_dir(repo, "jetson-orin-nano", "0b1f1469", None).unwrap(),
            PathBuf::from("/repo/results/jetson-orin-nano/0b1f1469")
        );
        assert_eq!(
            publish_dir(
                repo,
                "esp32s3",
                "0b1f1469aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                Some("icache32")
            )
            .unwrap(),
            PathBuf::from(
                "/repo/results/esp32s3/0b1f1469aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-icache32"
            )
        );
    }

    #[test]
    fn publish_dir_refuses_a_path_component() {
        assert!(publish_dir(Path::new("."), "Jetson", "0b1f1469", None).is_err());
        assert!(publish_dir(Path::new("."), "pi5", "0b1f1469", Some("../x")).is_err());
        assert!(publish_dir(Path::new("."), "pi5", "HEAD", None).is_err());
    }
}
