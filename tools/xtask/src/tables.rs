// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! rind's report tables (`SECTIONS`, `LIBRARIES`, `CRATES`, `SLOWER`) and
//! time format, from `admin/bench-report`.
//!
//! bench-report is Python and builds `SECTIONS` with comprehensions, so the
//! tables are evaluated from the rind checkout a command was given
//! (`--rind`), by running that file. `tools/bench-tables.json` is a cache of
//! one revision, stamped with the sha256 of the file it came from, and so is
//! `target/bench-tables/<sha256>.json`. A command uses whichever cache carries
//! the measured file's stamp and evaluates the file when neither does.
//! `cargo xtask bench-tables --rind` rewrites the committed cache for review.

use crate::py;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const SNAPSHOT: &str = "tools/bench-tables.json";

pub struct Tables {
    pub sections: Vec<(String, Vec<String>)>,
    pub libraries: Map<String, Value>,
    pub crates: Vec<(String, Vec<String>)>,
    pub slower: f64,
}

fn sha256_of(path: &Path) -> Result<String, String> {
    let data = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(Sha256::digest(&data)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

impl Tables {
    /// Tables from `rind`'s `admin/bench-report`, not from a snapshot of some
    /// other revision. The committed snapshot and `target/bench-tables/` are
    /// caches, used only when their stamp is this file's sha256.
    pub fn load(rind: &Path) -> Result<Tables, String> {
        let src = rind.join("admin/bench-report");
        let have = sha256_of(&src)?;
        for path in [crate::root().join(SNAPSHOT), cache_path(&have)] {
            if let Some(t) = read_stamped(&path, &have)? {
                return Ok(t);
            }
        }
        let snap = evaluate(&src)?;
        let path = cache_path(&have);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&path, py::dumps(&snap, Some(1)) + "\n")
            .map_err(|e| format!("{}: {e}", path.display()))?;
        tables_from(&snap)
    }

    pub fn operations(&self) -> impl Iterator<Item = &String> {
        self.sections.iter().flat_map(|(_, ops)| ops)
    }

    /// `operation_of`: the operation and library row of a criterion ID.
    pub fn operation_of(&self, id: &str) -> Option<(String, String)> {
        for op in self.operations() {
            let Some((pre, post)) = op.split_once("{lib}") else {
                continue;
            };
            if id.len() > pre.len() + post.len() && id.starts_with(pre) && id.ends_with(post) {
                let lib = &id[pre.len()..id.len() - post.len()];
                if !lib.contains('/') {
                    return Some((op.clone(), lib.to_string()));
                }
            }
        }
        None
    }

    pub fn library(&self, row: &str) -> String {
        self.libraries
            .get(row)
            .and_then(|v| v.as_str())
            .unwrap_or(row)
            .to_string()
    }
}

/// `human(ns)`.
pub fn human(ns: f64) -> String {
    if ns < 1e3 {
        format!("{} ns", py::fixed(ns, 0))
    } else if ns < 1e6 {
        format!("{} µs", py::fixed(ns / 1e3, 2))
    } else {
        format!("{} ms", py::fixed(ns / 1e6, 2))
    }
}

fn tables_from(snap: &Value) -> Result<Tables, String> {
    let sections = snap["sections"]
        .as_array()
        .ok_or("snapshot has no sections")?
        .iter()
        .map(|s| {
            (
                s[0].as_str().unwrap_or_default().to_string(),
                strings(&s[1]),
            )
        })
        .collect();
    let crates = snap["crates"]
        .as_object()
        .ok_or("snapshot has no crates")?
        .iter()
        .map(|(k, v)| (k.clone(), strings(v)))
        .collect();
    Ok(Tables {
        sections,
        libraries: snap["libraries"].as_object().cloned().unwrap_or_default(),
        crates,
        slower: snap["slower"].as_f64().ok_or("snapshot has no slower")?,
    })
}

/// `None` when the file is absent or stamped for a different bench-report.
fn read_stamped(path: &Path, sha: &str) -> Result<Option<Tables>, String> {
    if !path.is_file() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let snap: Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    if snap["source_sha256"].as_str() != Some(sha) {
        return Ok(None);
    }
    Ok(Some(tables_from(&snap)?))
}

fn cache_path(sha: &str) -> PathBuf {
    let dir = std::env::var_os("RIND_BENCH_TABLES_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|| crate::root().join("target/bench-tables"));
    dir.join(format!("{sha}.json"))
}

/// Evaluate `SECTIONS`, `LIBRARIES`, `CRATES` and `SLOWER` by loading rind's
/// bench-report. That file is Python, which is what builds the tables.
fn evaluate(src: &Path) -> Result<Value, String> {
    let code = "import importlib.machinery as m, importlib.util as u, json, sys\n\
                l = m.SourceFileLoader('br', sys.argv[1]); s = u.spec_from_loader('br', l)\n\
                b = u.module_from_spec(s); l.exec_module(b)\n\
                print(json.dumps({'sections': b.SECTIONS, 'libraries': b.LIBRARIES, 'crates': b.CRATES, \
                'slower': b.SLOWER}))";
    let out = Command::new("python3")
        .arg("-c")
        .arg(code)
        .arg(src)
        .output()
        .map_err(|e| format!("python3: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "evaluating {}: {}",
            src.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let mut v: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    let mut snap = Map::new();
    snap.insert(
        "note".into(),
        json!(
            "rind's admin/bench-report tables, evaluated by `cargo xtask bench-tables`; do not edit"
        ),
    );
    snap.insert("source".into(), json!("admin/bench-report"));
    snap.insert("source_sha256".into(), json!(sha256_of(src)?));
    for k in ["sections", "libraries", "crates", "slower"] {
        snap.insert(k.into(), v[k].take());
    }
    Ok(Value::Object(snap))
}

/// `cargo xtask bench-tables --rind R`: rewrite the committed cache from this
/// checkout, for review. Commands that read the tables do not need this; they
/// evaluate the checkout they were given.
pub fn refresh(rind: &Path) -> Result<(), String> {
    let src = rind.join("admin/bench-report");
    let snap = evaluate(&src)?;
    let path = crate::root().join(SNAPSHOT);
    std::fs::write(&path, py::dumps(&snap, Some(1)) + "\n").map_err(|e| e.to_string())?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_snapshot_matches_the_vendored_rind() {
        let rind = crate::root().join("upstream/rind");
        let t = Tables::load(&rind).expect("vendored rind");
        assert!(t.sections.len() > 2);
        assert!(t.operations().any(|op| op.contains("{lib}")));
        assert!(t.slower > 1.0);
    }

    #[test]
    fn a_different_bench_report_is_evaluated_and_cached_by_its_sha() {
        let dir = std::env::temp_dir().join(format!(
            "rind-tables-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let rind = dir.join("rind");
        std::fs::create_dir_all(rind.join("admin")).unwrap();
        let src = crate::root().join("upstream/rind/admin/bench-report");
        let mut text = std::fs::read_to_string(&src).unwrap();
        text.push_str("\n# rind-results cache stamp test\n");
        std::fs::write(rind.join("admin/bench-report"), &text).unwrap();
        let cache = dir.join("cache");
        // SAFETY: tests in this process do not run this case in parallel with
        // another writer of the same variable; the variable is restored after.
        let prev = std::env::var_os("RIND_BENCH_TABLES_CACHE");
        unsafe { std::env::set_var("RIND_BENCH_TABLES_CACHE", &cache) };
        let t = Tables::load(&rind).expect("evaluate");
        let sha = sha256_of(&rind.join("admin/bench-report")).unwrap();
        let cached = cache.join(format!("{sha}.json"));
        assert!(cached.is_file(), "cache {}", cached.display());
        let again = Tables::load(&rind).expect("cache hit");
        assert_eq!(t.sections.len(), again.sections.len());
        assert!(again.operations().any(|op| op.contains("{lib}")));
        match prev {
            // SAFETY: same as the set above; restoring the variable this test changed.
            Some(v) => unsafe { std::env::set_var("RIND_BENCH_TABLES_CACHE", v) },
            None => unsafe { std::env::remove_var("RIND_BENCH_TABLES_CACHE") },
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
