// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Maps recorded private `steelhead99x/rind-bench` commits to public
//! `steelhead99x/rind-hardware` commits. `cargo verify` has its own copy:
//! the two crates share no code.

use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::path::Path;

/// Squash root of steelhead99x/rind-hardware.
pub const SQUASH: &str = "8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3";
pub const MAP_PATH: &str = "tools/legacy-rind-bench-map.json";

pub struct Entry {
    /// Public SHA when the trees match (or match aside from the publish-time
    /// rename/scrub). `None` means an ancestor with no public tree.
    pub rind_hardware: Option<String>,
}

pub struct LegacyMap {
    by: BTreeMap<String, Entry>,
}

impl LegacyMap {
    pub fn load() -> Result<Self, String> {
        let path = crate::root().join(MAP_PATH);
        load_from(&path)
    }

    pub fn lookup(&self, recorded: &str) -> Option<&Entry> {
        let sha = recorded_sha(recorded)?;
        if let Some(e) = self.by.get(&sha) {
            return Some(e);
        }
        if sha.len() >= 7 {
            self.by.get(&sha[..7])
        } else {
            None
        }
    }

    /// Value for the `rind_hardware` JSON field.
    pub fn hardware_field(&self, recorded: &str) -> String {
        match self
            .lookup(recorded)
            .and_then(|e| e.rind_hardware.as_deref())
        {
            Some(h) => h.to_string(),
            None => format!("ancestor of {SQUASH}"),
        }
    }

    /// Short cell for a RESULTS.md provenance table.
    pub fn hardware_cell(&self, recorded: &str) -> String {
        match self
            .lookup(recorded)
            .and_then(|e| e.rind_hardware.as_deref())
        {
            Some(h) => format!("`{}`", &h[..7.min(h.len())]),
            None => format!("ancestor of `{}`", &SQUASH[..7]),
        }
    }
}

pub fn load_from(path: &Path) -> Result<LegacyMap, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let v: Value = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut by = BTreeMap::new();
    for c in v["commits"].as_array().into_iter().flatten() {
        let legacy = c["legacy"].as_str().unwrap_or_default().to_string();
        if legacy.is_empty() {
            continue;
        }
        let hw = c["rind_hardware"].as_str().map(str::to_string);
        let e = Entry {
            rind_hardware: hw.clone(),
        };
        by.insert(legacy.clone(), e);
        if legacy.len() >= 7 {
            by.entry(legacy[..7].to_string()).or_insert(Entry {
                rind_hardware: hw.clone(),
            });
        }
        for a in c["aliases"].as_array().into_iter().flatten() {
            let a = a.as_str().unwrap_or_default().to_string();
            if a.is_empty() {
                continue;
            }
            by.insert(
                a.clone(),
                Entry {
                    rind_hardware: hw.clone(),
                },
            );
            if a.len() >= 7 {
                by.entry(a[..7].to_string()).or_insert(Entry {
                    rind_hardware: hw.clone(),
                });
            }
        }
    }
    Ok(LegacyMap { by })
}

/// The last 7–40 hex token in a recorded `rind_bench` string.
pub fn recorded_sha(s: &str) -> Option<String> {
    let mut last = None;
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i].is_ascii_hexdigit() && !bytes[i].is_ascii_uppercase() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_hexdigit() && !bytes[i].is_ascii_uppercase()
            {
                i += 1;
            }
            let n = i - start;
            if (7..41).contains(&n) {
                last = Some(s[start..i].to_string());
            }
        } else {
            i += 1;
        }
    }
    last
}

/// Fill `rind_hardware` and `legacy_rind_bench` on one image object.
pub fn enrich_image(img: &mut Value, map: &LegacyMap) {
    let Some(obj) = img.as_object_mut() else {
        return;
    };
    let recorded = obj
        .get("legacy_rind_bench")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            obj.get("rind_bench")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        });
    let Some(recorded) = recorded else {
        return;
    };
    obj.insert(
        "rind_hardware".into(),
        Value::String(map.hardware_field(&recorded)),
    );
    obj.insert("legacy_rind_bench".into(), Value::String(recorded.clone()));
    obj.entry("rind_bench")
        .or_insert_with(|| Value::String(recorded));
}

pub fn enrich_images(images: Value, map: &LegacyMap) -> Value {
    let Some(obj) = images.as_object() else {
        return images;
    };
    let mut out = Map::new();
    for (k, v) in obj {
        let mut v = v.clone();
        enrich_image(&mut v, map);
        out.insert(k.clone(), v);
    }
    Value::Object(out)
}

/// RESULTS.md cell for the rind-hardware column of an image object.
pub fn hardware_cell_of(img: &Value, map: &LegacyMap) -> String {
    let hw = img
        .get("rind_hardware")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if hw.is_empty() {
        let rec = img
            .get("legacy_rind_bench")
            .or_else(|| img.get("rind_bench"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        return map.hardware_cell(rec);
    }
    if let Some(rest) = hw.strip_prefix("ancestor of ") {
        let short = &rest[..7.min(rest.len())];
        return format!("ancestor of `{short}`");
    }
    format!("`{}`", &hw[..7.min(hw.len())])
}

pub fn legacy_cell_of(img: &Value) -> String {
    img.get("legacy_rind_bench")
        .or_else(|| img.get("rind_bench"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}
