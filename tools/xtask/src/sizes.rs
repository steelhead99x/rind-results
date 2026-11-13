// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask sizes`: attributes the code and read-only data linked into
//! rind-hardware's images to the libraries raced, by demangled symbol.
//!
//! Every function or read-only object in any image counts once, under the
//! crate its demangled path starts with (a trait impl, `<crate::Type as
//! ...>`, counts for the type's crate). Generic code from core/alloc
//! instantiated for a library's types counts for core, so each figure is a
//! floor. Crates rind's portable backend shares with competitors are their
//! own group, `shared`.

use crate::py;
use regex::Regex;
use serde_json::{Map, Value, json};
use std::path::PathBuf;
use std::process::Command;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, default_value = "xtensa-esp32s3-elf-nm")]
    nm: String,
    images: PathBuf,
}

const GROUPS: &[(&str, &[&str])] = &[
    ("rind", &["rind"]),
    ("ring", &["ring"]),
    (
        "rustcrypto",
        &[
            "aes_gcm",
            "aes",
            "ghash",
            "polyval",
            "universal_hash",
            "aead",
            "cipher",
            "ctr",
            "chacha20",
            "chacha20poly1305",
            "poly1305",
            "sha2",
            "sha1",
            "hmac",
            "hkdf",
            "digest",
            "block_buffer",
            "p256",
            "primeorder",
            "elliptic_curve",
            "crypto_bigint",
            "ecdsa",
            "rfc6979",
            "ff",
            "group",
            "ml_kem",
            "module_lattice",
            "kem",
            "keccak",
            "sha3",
            "ed25519_dalek",
            "x25519_dalek",
            "ed25519",
            "signature",
            "subtle",
            "crypto_common",
            "inout",
            "hybrid_array",
        ],
    ),
    (
        "libcrux",
        &[
            "libcrux_aesgcm",
            "libcrux_aes",
            "libcrux_chacha20poly1305",
            "libcrux_ecdh",
            "libcrux_ed25519",
            "libcrux_hkdf",
            "libcrux_hmac",
            "libcrux_ml_kem",
            "libcrux_p256",
            "libcrux_curve25519",
            "libcrux_sha3",
            "libcrux_secrets",
            "libcrux_traits",
            "libcrux_intrinsics",
            "libcrux_platform",
        ],
    ),
    (
        "shared",
        &[
            "curve25519_dalek",
            "fiat_crypto",
            "libcrux_hacl_rs",
            "libcrux_sha2",
            "libcrux_poly1305",
        ],
    ),
];

fn group_of(crate_: &str) -> Option<&'static str> {
    GROUPS
        .iter()
        .find(|(_, cs)| cs.contains(&crate_))
        .map(|(g, _)| *g)
}

fn crate_of(name: &str, re: &Regex) -> Option<String> {
    let name = name.trim_start_matches(['<', '&', '*']);
    let name = name.strip_prefix("mut ").unwrap_or(name);
    let name = name.strip_prefix("dyn ").unwrap_or(name);
    if let Some(c) = re.captures(name) {
        return Some(c[1].to_string());
    }
    if name.starts_with("ring_core_") {
        return Some("ring".into());
    }
    if ["esp_aes", "esp_gcm", "esp_sha", "mbedtls_"]
        .iter()
        .any(|p| name.starts_with(p))
    {
        return Some("esp32s3-hw".into());
    }
    None
}

pub fn run(args: Args) -> Result<(), String> {
    let re = Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)::").unwrap();
    let mut elfs: Vec<String> = std::fs::read_dir(&args.images)
        .map_err(|e| format!("{}: {e}", args.images.display()))?
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".elf"))
        .collect();
    elfs.sort();
    // (group, crate, name) -> largest size, in first-seen order
    let mut seen: Vec<((String, String, String), i64)> = vec![];
    let mut index = std::collections::HashMap::new();
    let mut per_image = Map::new();
    for elf in &elfs {
        let out = Command::new(&args.nm)
            .args(["-S", "-C", "--size-sort"])
            .arg(args.images.join(elf))
            .output()
            .map_err(|e| format!("{}: {e}", args.nm))?;
        if !out.status.success() {
            return Err(format!("{} failed on {elf}", args.nm));
        }
        let text = String::from_utf8_lossy(&out.stdout);
        let mut image: Vec<(String, i64)> = vec![];
        for line in text.lines() {
            let parts: Vec<&str> = line.splitn(4, ' ').collect();
            // Python's `parts[2] not in "tTrR"` is a substring test.
            if parts.len() != 4 || !"tTrR".contains(parts[2]) {
                continue;
            }
            let Ok(size) = i64::from_str_radix(parts[1], 16) else {
                continue;
            };
            let name = parts[3];
            let Some(crate_) = crate_of(name, &re) else {
                continue;
            };
            let group = if crate_ == "esp32s3-hw" {
                Some("esp32s3-hw")
            } else {
                group_of(&crate_)
            };
            let Some(group) = group else { continue };
            match image.iter_mut().find(|(g, _)| g == group) {
                Some(e) => e.1 += size,
                None => image.push((group.to_string(), size)),
            }
            // A symbol's hash suffix makes the same function in two images one key.
            let key = (group.to_string(), crate_, name.to_string());
            match index.get(&key) {
                Some(&i) => {
                    let e: &mut (_, i64) = &mut seen[i];
                    e.1 = e.1.max(size);
                }
                None => {
                    index.insert(key.clone(), seen.len());
                    seen.push((key, size));
                }
            }
        }
        let m: Map<String, Value> = image.into_iter().map(|(g, n)| (g, json!(n))).collect();
        per_image.insert(
            elf.strip_suffix(".elf").unwrap().to_string(),
            Value::Object(m),
        );
    }
    let mut libraries: Vec<(String, i64)> = vec![];
    let mut crates: Vec<(String, i64)> = vec![];
    for ((group, crate_, _), size) in &seen {
        for (list, k) in [(&mut libraries, group), (&mut crates, crate_)] {
            match list.iter_mut().find(|(g, _)| g == k) {
                Some(e) => e.1 += size,
                None => list.push((k.clone(), *size)),
            }
        }
    }
    libraries.sort_by(|a, b| a.0.cmp(&b.0));
    crates.sort_by_key(|x| -x.1);
    let mut doc = Map::new();
    doc.insert(
        "note".into(),
        json!(
            "Bytes of code and read-only data, summed over symbols across every image, each counted once, \
             by the crate its demangled name starts with. `shared` is curve25519-dalek, fiat-crypto and \
             libcrux-hacl-rs/sha2/poly1305: rind's portable backend and the dalek or libcrux rows both link \
             them. Generic core/alloc code instantiated for a library counts for core, so each is a floor. \
             `esp32s3-hw` is ESP-IDF's mbedTLS AES/GCM/SHA wrappers around the accelerators."
        ),
    );
    doc.insert(
        "libraries".into(),
        Value::Object(libraries.into_iter().map(|(k, v)| (k, json!(v))).collect()),
    );
    doc.insert(
        "crates".into(),
        Value::Object(crates.into_iter().map(|(k, v)| (k, json!(v))).collect()),
    );
    doc.insert("per_image".into(), Value::Object(per_image));
    println!("{}", py::dumps(&Value::Object(doc), Some(1)));
    Ok(())
}
