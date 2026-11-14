// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Where a new publish is written.
//!
//! `results/<host>/<upstream-sha>/` holds one measured commit:
//! `<host>.json`, `meta.json`, `NOTES.md`, `run.log`, `raw/`, `RESULTS.md`.
//! An A/B or a diagnostic is `results/<host>/<upstream-sha>-<label>/`.
//! Trees published before this layout stay where they are; see `results/README.md`.

use std::path::{Path, PathBuf};

/// A host id is one results-directory component: the lowercase form of a
/// filesystem-safe name (letters, digits, `.`, `_` and `-`), 1 to 81
/// characters, starting with a letter or digit. Upper case is refused so two
/// ids cannot differ only by case. This is the same shape as rind's
/// `check_host_name`, restricted to lower case for the directory.
pub fn is_filesystem_safe_host(host: &str) -> bool {
    let mut chars = host.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return false;
    }
    let mut n = 1usize;
    for c in chars {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-')) {
            return false;
        }
        n += 1;
        if n > 81 {
            return false;
        }
    }
    true
}

pub fn host_id(host: &str) -> Result<String, String> {
    if !is_filesystem_safe_host(host) {
        return Err(format!(
            "host id {host:?} is not a filesystem-safe results directory name \
             (lowercase letters, digits, '.', '_' and '-', at most 81 characters, \
             starting with a letter or digit)"
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

/// Comparable set: kixel-ai/rind OPRF #705. Full 40-character SHA.
/// `tools/verify` keeps the same two constants; the crates share no code.
pub const COMPARABLE_PIN: &str = "d6ca9a756d3d1e71489e61e44c04f33c3c893589";

/// Previous comparable set. Directories named with this sha are history:
/// never deleted, overwritten, or mixed into [`COMPARABLE_PIN`] tables.
pub const PREVIOUS_COMPARABLE_PIN: &str = "0b1f1469c8e17e361e3353432bc7fceee9e04214";

/// The current pin's directory, and the older pin directories beside it.
pub struct TablePick {
    pub dir: PathBuf,
    /// Older comparable sets. Their numbers are not part of `dir`'s tables.
    pub history: Vec<PathBuf>,
}

/// `<sha>` or `<sha>-<label>` yields the sha. Anything else is not a pin directory.
pub fn revision_sha(name: &str) -> Option<&str> {
    let sha = name.split_once('-').map(|(s, _)| s).unwrap_or(name);
    if (7..=40).contains(&sha.len())
        && sha
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        Some(sha)
    } else {
        None
    }
}

/// `sha` and `pin` name the same commit when one is a prefix of the other.
pub fn sha_matches(sha: &str, pin: &str) -> bool {
    let (short, long) = if sha.len() <= pin.len() {
        (sha, pin)
    } else {
        (pin, sha)
    };
    short.len() >= 7 && long.starts_with(short)
}

pub fn is_previous_comparable_set(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .and_then(revision_sha)
        .is_some_and(|sha| sha_matches(sha, PREVIOUS_COMPARABLE_PIN))
}

/// Refuse to write a previous comparable-set directory.
pub fn refuse_history_out(out: &Path) -> Result<(), String> {
    if is_previous_comparable_set(out) {
        return Err(format!(
            "{} is the previous comparable set ({PREVIOUS_COMPARABLE_PIN}); it stays as history \
             and is not overwritten or mixed into {COMPARABLE_PIN} tables",
            out.display()
        ));
    }
    Ok(())
}

fn child_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    v.sort();
    v
}

/// When `dir` holds the current pin and any other upstream sha, tables use the
/// current pin. Other sha directories are history. A directory with only the
/// previous pin is left alone, so that set can still be checked on its own.
pub fn table_dir(dir: &Path) -> Option<TablePick> {
    let mut current = vec![];
    let mut history = vec![];
    for c in child_dirs(dir) {
        let Some(name) = c.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(sha) = revision_sha(name) else {
            continue;
        };
        if sha_matches(sha, COMPARABLE_PIN) {
            current.push(c);
        } else {
            history.push(c);
        }
    }
    if current.is_empty() || history.is_empty() {
        return None;
    }
    let picked = current
        .iter()
        .find(|p| p.file_name().and_then(|s| s.to_str()) == Some(COMPARABLE_PIN))
        .or_else(|| {
            current.iter().find(|p| {
                p.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|n| revision_sha(n) == Some(n))
            })
        })
        .or_else(|| current.first())?
        .clone();
    Some(TablePick {
        dir: picked,
        history,
    })
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

    #[test]
    fn host_id_accepts_a_cpu_slug_and_refuses_a_path() {
        assert!(host_id("intel-core-ultra-9-275hx").is_ok());
        assert!(host_id("amd-ryzen-9-7950x").is_ok());
        assert!(host_id("my_host").is_ok());
        assert!(host_id("cpu.rev").is_ok());
        assert!(host_id(&"a".repeat(81)).is_ok());
        assert!(host_id("Jetson").is_err());
        assert!(host_id("../x").is_err());
        assert!(host_id("foo/bar").is_err());
        assert!(host_id("-x").is_err());
        assert!(host_id("").is_err());
        assert!(host_id(&"a".repeat(82)).is_err());
    }

    #[test]
    fn current_pin_tables_leave_the_previous_set_as_history() {
        let root = std::env::temp_dir().join(format!("rind-pin-sets-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let current = root.join(COMPARABLE_PIN);
        let labeled = root.join(format!("{COMPARABLE_PIN}-icache"));
        let previous = root.join(PREVIOUS_COMPARABLE_PIN);
        let previous_labeled = root.join(format!("{PREVIOUS_COMPARABLE_PIN}-relink"));
        for d in [&current, &labeled, &previous, &previous_labeled] {
            std::fs::create_dir_all(d).unwrap();
        }
        std::fs::create_dir_all(root.join("notes")).unwrap();
        let pick = table_dir(&root).unwrap();
        assert_eq!(pick.dir, current);
        let hist: Vec<_> = pick
            .history
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            hist,
            vec![
                PREVIOUS_COMPARABLE_PIN.to_string(),
                format!("{PREVIOUS_COMPARABLE_PIN}-relink"),
            ]
        );
        assert!(refuse_history_out(&previous).is_err());
        assert!(refuse_history_out(&previous_labeled).is_err());
        assert!(refuse_history_out(&current).is_ok());
        assert!(refuse_history_out(&labeled).is_ok());
        // The previous pin on its own is still that set, not dropped.
        let only = root.join("only-old");
        std::fs::create_dir_all(only.join(PREVIOUS_COMPARABLE_PIN)).unwrap();
        assert!(table_dir(&only).is_none());
        let _ = std::fs::remove_dir_all(&root);
    }
}
