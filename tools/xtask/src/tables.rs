// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! rind's report tables (`SECTIONS`, `LIBRARIES`, `CRATES`, `SLOWER`) and
//! time format, from `admin/bench-report`.
//!
//! bench-report is Python and builds `SECTIONS` with comprehensions, so the
//! tables are kept as a snapshot, `tools/bench-tables.json`, evaluated from
//! it by `cargo xtask bench-tables` and stamped with the sha256 of the file
//! it came from. Every command that uses them refuses a snapshot whose stamp
//! does not match the rind checkout it is given.

use crate::py;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::path::Path;
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
    pub fn load(rind: &Path) -> Result<Tables, String> {
        let snap: Value = serde_json::from_str(
            &std::fs::read_to_string(crate::root().join(SNAPSHOT))
                .map_err(|e| format!("{SNAPSHOT}: {e}"))?,
        )
        .map_err(|e| format!("{SNAPSHOT}: {e}"))?;
        let src = rind.join("admin/bench-report");
        let have = sha256_of(&src)?;
        if snap["source_sha256"].as_str() != Some(have.as_str()) {
            return Err(format!(
                "{} changed since {SNAPSHOT} was taken (sha256 {have}, snapshot {}); refresh it with \
                 `cargo xtask bench-tables --rind {}` and review the diff",
                src.display(),
                snap["source_sha256"],
                rind.display()
            ));
        }
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

/// `cargo xtask bench-tables --rind R`: evaluate the tables from rind's
/// bench-report (the one step that runs Python, since that is the language
/// rind keeps them in) and write the snapshot.
pub fn refresh(rind: &Path) -> Result<(), String> {
    let src = rind.join("admin/bench-report");
    let code = "import importlib.machinery as m, importlib.util as u, json, sys\n\
                l = m.SourceFileLoader('br', sys.argv[1]); s = u.spec_from_loader('br', l)\n\
                b = u.module_from_spec(s); l.exec_module(b)\n\
                print(json.dumps({'sections': b.SECTIONS, 'libraries': b.LIBRARIES, 'crates': b.CRATES, \
                'slower': b.SLOWER}))";
    let out = Command::new("python3")
        .arg("-c")
        .arg(code)
        .arg(&src)
        .output()
        .map_err(|e| format!("python3: {e}"))?;
    if !out.status.success() {
        return Err(format!("python3: {}", String::from_utf8_lossy(&out.stderr)));
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
    snap.insert("source_sha256".into(), json!(sha256_of(&src)?));
    for k in ["sections", "libraries", "crates", "slower"] {
        snap.insert(k.into(), v[k].take());
    }
    let path = crate::root().join(SNAPSHOT);
    std::fs::write(&path, py::dumps(&Value::Object(snap), Some(1)) + "\n")
        .map_err(|e| e.to_string())?;
    eprintln!("wrote {}", path.display());
    Ok(())
}
