// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Recompute a cycle-median tree from its serial capture.
//!
//! `medians.json` can agree with `RUNS.md` and `RESULTS.md` and still not be
//! what the board printed. When `raw/capture.log` (or `capture.log`) is
//! present, every published cycle, µs field, digest, omitted row, and
//! known-answer line is read back from that log. SHA-256 and SHA-512 pattern
//! digests are hashed again. The Uno R3 Poly1305 rows are the RFC 8439 MAC of
//! the pattern bytes under the key the firmware used (`i * 17` in a byte).
//! This module does not call xtask or the firmware.

use super::Report;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256, Sha512};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

struct Bench {
    cycles: u64,
    us: Option<u64>,
    digest: Option<String>,
}

struct Capture {
    benches: Vec<(String, Bench)>,
    kats: Vec<(String, String)>,
    /// `KAT PASS` / `KAT FAIL` with no test name.
    kat_overall: Option<String>,
}

pub fn check_capture(label: &str, dir: &Path, json: &Value, clock_hz: u64, rep: &mut Report) {
    let Some(path) = capture_path(dir) else {
        return;
    };
    let rel = capture_rel(&path);
    let text = match std::fs::read(&path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => {
                rep.bad(label, format!("{rel}: not UTF-8"));
                return;
            }
        },
        Err(e) => {
            rep.bad(label, format!("{rel}: {e}"));
            return;
        }
    };
    let capture = match parse_capture(&text) {
        Ok(c) => c,
        Err(e) => {
            rep.bad(label, format!("{rel}: {e}"));
            return;
        }
    };
    let Some(ops) = json.get("ops").and_then(|v| v.as_object()) else {
        return;
    };

    let mut by_op: BTreeMap<&str, Vec<&Bench>> = BTreeMap::new();
    for (op, bench) in &capture.benches {
        by_op.entry(op.as_str()).or_default().push(bench);
    }

    let mut published = BTreeSet::new();
    let mut ops_checked = 0usize;
    for (op, rec) in ops {
        published.insert(op.as_str());
        let Some(rec) = rec.as_object() else {
            rep.bad(label, format!("{op}: ops entry is not an object"));
            continue;
        };
        let Some(rows) = by_op.get(op.as_str()) else {
            rep.bad(
                label,
                format!("{op}: medians.json publishes it, {rel} has no BENCH row"),
            );
            continue;
        };
        ops_checked += 1;
        check_cycles(label, rel, op, rec, rows, clock_hz, rep);
        check_published_digest(label, rel, op, rec, rows, rep);
    }

    let mut omitted_checked = 0usize;
    if let Some(omitted) = json.get("omitted_unreliable") {
        if let Some(obj) = omitted.as_object() {
            for (op, rec) in obj {
                if published.contains(op.as_str()) {
                    rep.bad(
                        label,
                        format!("{op}: listed both in ops and in omitted_unreliable"),
                    );
                }
                published.insert(op.as_str());
                let Some(rec) = rec.as_object() else {
                    rep.bad(label, format!("{op}: omitted entry is not an object"));
                    continue;
                };
                let Some(rows) = by_op.get(op.as_str()) else {
                    rep.bad(
                        label,
                        format!("{op}: omitted_unreliable lists it, {rel} has no BENCH row"),
                    );
                    continue;
                };
                omitted_checked += 1;
                check_cycles(label, rel, op, rec, rows, clock_hz, rep);
                if let Some(checks) = rec.get("checksums_le_hex") {
                    check_hex_list(label, rel, op, "checksums_le_hex", checks, rows, rep);
                }
                if let Some(digests) = rec.get("digests") {
                    check_hex_list(label, rel, op, "digests", digests, rows, rep);
                }
            }
        } else {
            rep.bad(label, "omitted_unreliable is not an object".into());
        }
    }

    for op in by_op.keys() {
        if !published.contains(op) {
            rep.bad(
                label,
                format!(
                    "{op}: {rel} has BENCH rows that medians.json neither publishes nor lists as omitted"
                ),
            );
        }
    }
    check_kats(label, rel, json, &capture, rep);
    finish_note(
        label,
        rel,
        capture.benches.len(),
        ops_checked,
        omitted_checked,
        rep,
    );
}

fn finish_note(
    label: &str,
    rel: &str,
    rows: usize,
    ops_checked: usize,
    omitted_checked: usize,
    rep: &mut Report,
) {
    let msg = format!(
        "{rel}: {rows} BENCH rows, {ops_checked} published ops, {omitted_checked} omitted ops recomputed"
    );
    println!("{label}: {msg}");
    rep.note(label, msg);
}

fn capture_path(dir: &Path) -> Option<std::path::PathBuf> {
    let raw = dir.join("raw/capture.log");
    if raw.is_file() {
        return Some(raw);
    }
    let top = dir.join("capture.log");
    if top.is_file() {
        return Some(top);
    }
    None
}

fn capture_rel(path: &Path) -> &'static str {
    if path
        .parent()
        .and_then(|p| p.file_name())
        .is_some_and(|n| n == "raw")
    {
        "raw/capture.log"
    } else {
        "capture.log"
    }
}

fn parse_capture(text: &str) -> Result<Capture, String> {
    let mut benches = Vec::new();
    let mut kats = Vec::new();
    let mut kat_overall = None;
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim().trim_end_matches('\r');
        if let Some(rest) = line.strip_prefix("BENCH ") {
            benches.push(parse_bench(n + 1, rest)?);
        } else if let Some(rest) = line.strip_prefix("KAT ") {
            let mut parts = rest.split_whitespace();
            let Some(first) = parts.next() else {
                return Err(format!("line {}: KAT has no status", n + 1));
            };
            if let Some(status) = parts.next() {
                if parts.next().is_some() {
                    return Err(format!("line {}: KAT has extra fields", n + 1));
                }
                kats.push((first.to_string(), status.to_string()));
            } else {
                kat_overall = Some(first.to_string());
            }
        }
    }
    Ok(Capture {
        benches,
        kats,
        kat_overall,
    })
}

fn parse_bench(line_no: usize, rest: &str) -> Result<(String, Bench), String> {
    let mut parts = rest.split_whitespace();
    let Some(op) = parts.next() else {
        return Err(format!("line {line_no}: BENCH has no operation"));
    };
    if op.contains('=') {
        return Err(format!(
            "line {line_no}: BENCH operation {op} looks like a field"
        ));
    }
    let mut cycles = None;
    let mut us = None;
    let mut digest = None;
    for tok in parts {
        let Some((key, value)) = tok.split_once('=') else {
            return Err(format!(
                "line {line_no}: BENCH {op} token {tok} is not key=value"
            ));
        };
        match key {
            "cycles" => {
                let parsed = parse_u64(line_no, op, key, value)?;
                if cycles.is_some() {
                    return Err(format!("line {line_no}: BENCH {op} repeats cycles"));
                }
                cycles = Some(parsed);
            }
            "us" => {
                let parsed = parse_u64(line_no, op, key, value)?;
                if us.is_some() {
                    return Err(format!("line {line_no}: BENCH {op} repeats us"));
                }
                us = Some(parsed);
            }
            "digest" => {
                if value.is_empty()
                    || value.len() % 2 != 0
                    || !value.chars().all(|c| c.is_ascii_hexdigit())
                {
                    return Err(format!("line {line_no}: BENCH {op} digest is not hex"));
                }
                if digest.is_some() {
                    return Err(format!("line {line_no}: BENCH {op} repeats digest"));
                }
                digest = Some(value.to_ascii_lowercase());
            }
            other => {
                return Err(format!(
                    "line {line_no}: BENCH {op} has unknown field {other}"
                ));
            }
        }
    }
    let Some(cycles) = cycles else {
        return Err(format!("line {line_no}: BENCH {op} has no cycles"));
    };
    Ok((op.to_string(), Bench { cycles, us, digest }))
}

fn parse_u64(line_no: usize, op: &str, key: &str, value: &str) -> Result<u64, String> {
    value
        .parse()
        .map_err(|_| format!("line {line_no}: BENCH {op} {key}={value} is not an integer"))
}

fn json_u64s(value: &Value) -> Option<Vec<u64>> {
    let arr = value.as_array()?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let n = item
            .as_u64()
            .or_else(|| item.as_i64().filter(|n| *n >= 0).map(|n| n as u64))?;
        out.push(n);
    }
    Some(out)
}

fn check_cycles(
    label: &str,
    rel: &str,
    op: &str,
    rec: &Map<String, Value>,
    rows: &[&Bench],
    clock_hz: u64,
    rep: &mut Report,
) {
    let Some(cycles_v) = rec.get("cycles_all") else {
        rep.bad(label, format!("{op}: missing cycles_all"));
        return;
    };
    let Some(cycles) = json_u64s(cycles_v) else {
        rep.bad(label, format!("{op}: cycles_all is not a list of integers"));
        return;
    };
    if cycles.len() != rows.len() {
        rep.bad(
            label,
            format!(
                "{op}: medians.json has {} cycles, {rel} has {} BENCH rows",
                cycles.len(),
                rows.len()
            ),
        );
        return;
    }
    for (i, (want, row)) in cycles.iter().zip(rows).enumerate() {
        if *want != row.cycles {
            rep.bad(
                label,
                format!(
                    "{op} run {}: medians.json cycles {want}, {rel} has {}",
                    i + 1,
                    row.cycles
                ),
            );
        }
        if clock_hz == 0 {
            continue;
        }
        let Some(us) = row.us else {
            continue;
        };
        match cycles_to_us(row.cycles, clock_hz) {
            Some(want_us) if us != want_us => rep.bad(
                label,
                format!(
                    "{op} run {}: {rel} us is {us}, cycles/clock is {want_us}",
                    i + 1
                ),
            ),
            Some(_) => {}
            None => rep.bad(
                label,
                format!(
                    "{op} run {}: cycles {} do not convert at {clock_hz} Hz",
                    i + 1,
                    row.cycles
                ),
            ),
        }
    }
}

fn cycles_to_us(cycles: u64, clock_hz: u64) -> Option<u64> {
    cycles.checked_mul(1_000_000)?.checked_div(clock_hz)
}

fn check_published_digest(
    label: &str,
    rel: &str,
    op: &str,
    rec: &Map<String, Value>,
    rows: &[&Bench],
    rep: &mut Report,
) {
    let verified = match rec.get("digest_verified") {
        None => false,
        Some(Value::Bool(v)) => *v,
        Some(_) => {
            rep.bad(label, format!("{op}: digest_verified is not a bool"));
            false
        }
    };
    let digest = match rec.get("digest") {
        None => {
            if verified {
                rep.bad(
                    label,
                    format!("{op}: digest_verified is set and digest is missing"),
                );
            }
            return;
        }
        Some(Value::String(s)) => s.to_ascii_lowercase(),
        Some(_) => {
            rep.bad(label, format!("{op}: digest is not a string"));
            return;
        }
    };
    for (i, row) in rows.iter().enumerate() {
        match &row.digest {
            Some(got) if got == &digest => {}
            Some(got) => rep.bad(
                label,
                format!(
                    "{op} run {}: {rel} digest {got} != medians.json {digest}",
                    i + 1
                ),
            ),
            None => rep.bad(label, format!("{op} run {}: {rel} has no digest", i + 1)),
        }
    }
    if !verified {
        return;
    }
    match recompute_digest(op) {
        Ok(Some(want)) if want == digest => {}
        Ok(Some(want)) => rep.bad(
            label,
            format!("{op}: digest_verified digest {digest} != recomputed {want}"),
        ),
        Ok(None) => rep.bad(
            label,
            format!("{op}: digest_verified is set and this verifier has no pattern rule for it"),
        ),
        Err(e) => rep.bad(label, format!("{op}: {e}")),
    }
}

fn check_hex_list(
    label: &str,
    rel: &str,
    op: &str,
    field: &str,
    value: &Value,
    rows: &[&Bench],
    rep: &mut Report,
) {
    let Some(list) = value.as_array() else {
        rep.bad(label, format!("{op}: {field} is not an array"));
        return;
    };
    if list.len() != rows.len() {
        rep.bad(
            label,
            format!(
                "{op}: {field} has {} values, {rel} has {} BENCH rows",
                list.len(),
                rows.len()
            ),
        );
        return;
    }
    for (i, (item, row)) in list.iter().zip(rows).enumerate() {
        let Some(want) = item.as_str() else {
            rep.bad(label, format!("{op}: {field}[{i}] is not a string"));
            continue;
        };
        let want = want.to_ascii_lowercase();
        match &row.digest {
            Some(got) if got == &want => {}
            Some(got) => rep.bad(
                label,
                format!(
                    "{op} run {}: {field} is {want}, {rel} digest is {got}",
                    i + 1
                ),
            ),
            None => rep.bad(
                label,
                format!("{op} run {}: {field} is set and {rel} has no digest", i + 1),
            ),
        }
    }
}

fn check_kats(label: &str, rel: &str, json: &Value, capture: &Capture, rep: &mut Report) {
    let Some(kats_v) = json.get("kats") else {
        if capture.kats.iter().any(|(_, s)| s != "PASS")
            || capture.kat_overall.as_deref() == Some("FAIL")
        {
            rep.bad(
                label,
                format!("{rel} reports a KAT failure and medians.json has no kats"),
            );
        }
        return;
    };
    let Some(arr) = kats_v.as_array() else {
        rep.bad(label, "kats is not an array".into());
        return;
    };
    let mut recorded: Vec<(String, String)> = Vec::new();
    for (i, item) in arr.iter().enumerate() {
        let Some(pair) = item.as_array() else {
            rep.bad(label, format!("kats[{i}] is not a pair"));
            continue;
        };
        if pair.len() != 2 {
            rep.bad(label, format!("kats[{i}] is not a name/status pair"));
            continue;
        }
        let (Some(name), Some(status)) = (pair[0].as_str(), pair[1].as_str()) else {
            rep.bad(label, format!("kats[{i}] is not a name/status pair"));
            continue;
        };
        recorded.push((name.to_string(), status.to_string()));
    }
    if recorded.len() != capture.kats.len()
        || recorded.iter().zip(&capture.kats).any(|(a, b)| a != b)
    {
        rep.bad(
            label,
            format!(
                "kats {:?} != {rel} {:?}",
                recorded
                    .iter()
                    .map(|(n, s)| format!("{n} {s}"))
                    .collect::<Vec<_>>(),
                capture
                    .kats
                    .iter()
                    .map(|(n, s)| format!("{n} {s}"))
                    .collect::<Vec<_>>()
            ),
        );
    }
    if let Some(overall) = &capture.kat_overall {
        let any_fail = recorded.iter().any(|(_, s)| s != "PASS");
        if overall == "FAIL" && !any_fail {
            rep.bad(
                label,
                format!("{rel} says KAT FAIL and every recorded known-answer test passed"),
            );
        }
        if overall == "PASS" && any_fail {
            rep.bad(
                label,
                format!("{rel} says KAT PASS and a recorded known-answer test did not"),
            );
        }
    }
}

enum Family {
    Sha256,
    Sha512,
    Poly1305,
}

fn pattern_op(op: &str) -> Option<(Family, usize)> {
    let (name, size) = op.split_once('/')?;
    if op.split('/').nth(2).is_some() {
        return None;
    }
    let family = match name {
        "sha256" => Family::Sha256,
        "sha512" => Family::Sha512,
        "poly1305" => Family::Poly1305,
        _ => return None,
    };
    Some((family, parse_size(size)?))
}

/// `32B` is 32 bytes. `16KB` is 16 KiB, matching the firmware's `16 * 1024`.
fn parse_size(size: &str) -> Option<usize> {
    if let Some(n) = size.strip_suffix("KB") {
        return n.parse::<usize>().ok()?.checked_mul(1024);
    }
    let n = size.strip_suffix('B')?;
    if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    n.parse().ok()
}

fn pattern_bytes(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i & 0xff) as u8).collect()
}

/// Key byte `i` is `(i as u8) * 17`, the key `bench_poly1305` in the Uno R3
/// firmware passes to Poly1305. The message is [`pattern_bytes`].
fn uno_poly_key() -> [u8; 32] {
    std::array::from_fn(|i| (i as u8).wrapping_mul(17))
}

fn recompute_digest(op: &str) -> Result<Option<String>, String> {
    let Some((family, len)) = pattern_op(op) else {
        return Ok(None);
    };
    let msg = pattern_bytes(len);
    let digest = match family {
        Family::Sha256 => sha256_hex(&msg),
        Family::Sha512 => sha512_hex(&msg),
        Family::Poly1305 => hex(&poly1305(&uno_poly_key(), &msg)?),
    };
    Ok(Some(digest))
}

fn sha256_hex(data: &[u8]) -> String {
    hex(&Sha256::digest(data))
}

fn sha512_hex(data: &[u8]) -> String {
    hex(&Sha512::digest(data))
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}

/// RFC 8439 Poly1305. `r` is clamped; the tag is the low 128 bits of `h + s`
/// after `h` is fully reduced modulo `2^130 - 5`.
fn poly1305(key: &[u8; 32], msg: &[u8]) -> Result<[u8; 16], String> {
    let r = u128::from_le_bytes(key[..16].try_into().unwrap())
        & 0x0fff_fffc_0fff_fffc_0fff_fffc_0fff_ffff;
    let s = u128::from_le_bytes(key[16..32].try_into().unwrap());
    let mut lo = 0u128;
    let mut hi = 0u8;
    let mut rest = msg;
    while rest.len() >= 16 {
        let (block, tail) = rest.split_at(16);
        (lo, hi) = add_block(lo, hi, block, false)?;
        (lo, hi) = mul_mod(lo, hi, r)?;
        rest = tail;
    }
    if !rest.is_empty() {
        (lo, hi) = add_block(lo, hi, rest, true)?;
        (lo, hi) = mul_mod(lo, hi, r)?;
    }
    // `mul_mod` already reduced each block. Empty input never entered that
    // path, and the tag is only the low 128 bits, so the high limb is dropped.
    let (lo, _) = fully_reduce(lo, hi)?;
    let (tag, _) = lo.overflowing_add(s);
    Ok(tag.to_le_bytes())
}

fn add_block(lo: u128, hi: u8, block: &[u8], partial: bool) -> Result<(u128, u8), String> {
    let mut n = 0u128;
    for (i, b) in block.iter().enumerate() {
        n |= u128::from(*b) << (8 * i);
    }
    let (n, n_hi) = if partial {
        let bit = 8 * block.len();
        if bit >= 128 {
            return Err("poly1305 partial block is 16 bytes".into());
        }
        let (n, carry) = n.overflowing_add(1u128 << bit);
        if carry {
            return Err("poly1305 partial hibit overflowed".into());
        }
        (n, 0u8)
    } else {
        // A full block adds 2^128, which sits above the low 128 bits.
        (n, 1u8)
    };
    let (sum, carry) = lo.overflowing_add(n);
    let sum_hi = hi
        .checked_add(n_hi)
        .and_then(|h| h.checked_add(u8::from(carry)))
        .ok_or("poly1305 accumulator overflow")?;
    Ok((sum, sum_hi))
}

fn mul_mod(lo: u128, hi: u8, r: u128) -> Result<(u128, u8), String> {
    let (p_lo, p_hi) = mul_u128(lo, r);
    let hi_term = u128::from(hi)
        .checked_mul(r)
        .ok_or("poly1305 limb overflow")?;
    let (hi_sum, overflow) = p_hi.overflowing_add(hi_term);
    if overflow {
        return Err("poly1305 product overflow".into());
    }
    reduce_product(p_lo, hi_sum)
}

fn reduce_product(lo: u128, hi: u128) -> Result<(u128, u8), String> {
    let low_hi = u8::try_from(hi & 3).unwrap_or(0);
    let rest = hi >> 2;
    let add = rest.checked_mul(5).ok_or("poly1305 fold overflow")?;
    let (sum_lo, carry) = lo.overflowing_add(add);
    let sum_hi = low_hi + u8::from(carry);
    fully_reduce(sum_lo, sum_hi)
}

/// Fold `2^130 ≡ 5` until the value is strictly less than `2^130 - 5`.
fn fully_reduce(mut lo: u128, mut hi: u8) -> Result<(u128, u8), String> {
    for _ in 0..8 {
        let over = hi >= 4 || (hi == 3 && lo >= u128::MAX - 4);
        if !over {
            return Ok((lo, hi));
        }
        let (nlo, carry) = lo.overflowing_add(5);
        lo = nlo;
        hi = hi
            .checked_add(u8::from(carry))
            .and_then(|h| h.checked_sub(4))
            .ok_or("poly1305 reduction overflow")?;
    }
    Err("poly1305 reduction did not finish".into())
}

fn mul_u128(a: u128, b: u128) -> (u128, u128) {
    let a0 = a & 0xffff_ffff_ffff_ffff;
    let a1 = a >> 64;
    let b0 = b & 0xffff_ffff_ffff_ffff;
    let b1 = b >> 64;
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;
    let mid = (p00 >> 64) + (p01 & 0xffff_ffff_ffff_ffff) + (p10 & 0xffff_ffff_ffff_ffff);
    let lo = (p00 & 0xffff_ffff_ffff_ffff) | ((mid & 0xffff_ffff_ffff_ffff) << 64);
    let hi = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn scratch() -> PathBuf {
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!("rind-verify-capture-{t}-{n}"));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn write_tree(dir: &Path, json: &str, capture: &str) {
        std::fs::create_dir_all(dir.join("raw")).unwrap();
        std::fs::write(dir.join("medians.json"), json).unwrap();
        std::fs::write(dir.join("raw/capture.log"), capture).unwrap();
        std::fs::write(
            dir.join("RUNS.md"),
            "| operation | run 1 | median | spread |\n|---|---:|---:|---:|\n| `sha256/32B` | 160 | 160 | 0.00% |\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("RESULTS.md"),
            "| Operation | cycles | range | µs |\n|---|---:|---:|---:|\n| sha256/32B | 160 | 160–160 | 10 |\n",
        )
        .unwrap();
    }

    const SHA256_32: &str = "630dcd2966c4336691125448bbb25b4ff412a49c732db2c8abc1b8581bd710dd";

    fn sha_tree(digest: &str, verified: bool) -> String {
        format!(
            "{{\n  \"clock_hz\": 16000000,\n  \"kats\": [[\"s256-empty\", \"PASS\"]],\n  \"ops\": {{\n    \"sha256/32B\": {{\n      \"n\": 1,\n      \"cycles_all\": [160],\n      \"cycles_median\": 160,\n      \"cycles_min\": 160,\n      \"cycles_max\": 160,\n      \"us_median\": 10,\n      \"digest\": \"{digest}\",\n      \"digest_verified\": {verified}\n    }}\n  }}\n}}\n"
        )
    }

    fn sha_log(digest: &str) -> String {
        format!(
            "KAT s256-empty PASS\nKAT PASS\nBENCH sha256/32B cycles=160 us=10 digest={digest}\nDONE\n"
        )
    }

    fn verify(dir: &Path) -> super::super::Report {
        let mut rep = super::super::Report::default();
        assert!(
            super::super::tree::verify_dir(dir, &mut rep),
            "{}",
            dir.display()
        );
        rep
    }

    #[test]
    fn sha256_and_sha512_known_answers() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha512_hex(b"abc"),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
    }

    #[test]
    fn pattern_sha_matches_the_published_digests() {
        assert_eq!(sha256_hex(&pattern_bytes(32)), SHA256_32);
        assert_eq!(
            sha256_hex(&pattern_bytes(1350)),
            "62321a91a4688356ea0649f25cd5dca2dba13cc5e1c05e53bf3ab21d862614d2"
        );
        assert_eq!(
            sha512_hex(&pattern_bytes(32)),
            "3d94eea49c580aef816935762be049559d6d1440dede12e6a125f1841fff8e6fa9d71862a3e5746b571be3d187b0041046f52ebd850c7cbd5fde8ee38473b649"
        );
    }

    #[test]
    fn mul_u128_covers_the_top_word() {
        assert_eq!(mul_u128(2, 3), (6, 0));
        assert_eq!(mul_u128(u128::MAX, u128::MAX), (1, u128::MAX - 1));
    }

    #[test]
    fn poly1305_rfc8439_vector() {
        // RFC 8439 §2.5.2. 34 bytes, so one partial block.
        let key = hex_bytes("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let tag = poly1305(&key, b"Cryptographic Forum Research Group").unwrap();
        assert_eq!(hex(&tag), "a8061dc1305136c6c22b8baf0c0127a9");
        let zeros = [0u8; 32];
        assert_eq!(hex(&poly1305(&zeros, b"").unwrap()), "00".repeat(16));
    }

    #[test]
    fn uno_poly1305_matches_the_published_tags() {
        let key = uno_poly_key();
        assert_eq!(
            hex(&poly1305(&key, &pattern_bytes(16)).unwrap()),
            "75759b271925466c9b7f67f32bb42eec"
        );
        assert_eq!(
            hex(&poly1305(&key, &pattern_bytes(1350)).unwrap()),
            "c502a707021a00fe889b38efcc94aaa2"
        );
    }

    #[test]
    fn top_level_capture_log_is_accepted() {
        let dir = scratch();
        write_tree(&dir, &sha_tree(SHA256_32, true), &sha_log(SHA256_32));
        let log = std::fs::read_to_string(dir.join("raw/capture.log")).unwrap();
        std::fs::remove_dir_all(dir.join("raw")).unwrap();
        std::fs::write(dir.join("capture.log"), log).unwrap();
        let rep = verify(&dir);
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        assert!(
            rep.notes
                .iter()
                .any(|n| n.contains("capture.log") && !n.contains("raw/")),
            "{:?}",
            rep.notes
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn capture_accepts_the_sha256_pattern() {
        let dir = scratch();
        write_tree(&dir, &sha_tree(SHA256_32, true), &sha_log(SHA256_32));
        let rep = verify(&dir);
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        assert!(rep.notes.iter().any(|n| n.contains("1 BENCH rows")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn repeated_bench_field_cannot_select_the_later_cycle() {
        let dir = scratch();
        // The tables agree on 161. The log prints cycles twice; the later value
        // is 161. Taking the last field would hide 160.
        let json = sha_tree(SHA256_32, true)
            .replace("\"cycles_all\": [160]", "\"cycles_all\": [161]")
            .replace("\"cycles_median\": 160", "\"cycles_median\": 161")
            .replace("\"cycles_min\": 160", "\"cycles_min\": 161")
            .replace("\"cycles_max\": 160", "\"cycles_max\": 161");
        let log = sha_log(SHA256_32).replace("cycles=160", "cycles=160 cycles=161");
        write_tree(&dir, &json, &log);
        std::fs::write(
            dir.join("RUNS.md"),
            "| operation | run 1 | median | spread |\n|---|---:|---:|---:|\n| `sha256/32B` | 161 | 161 | 0.00% |\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("RESULTS.md"),
            "| Operation | cycles | range | µs |\n|---|---:|---:|---:|\n| sha256/32B | 161 | 161–161 | 10 |\n",
        )
        .unwrap();
        let rep = verify(&dir);
        assert!(
            rep.problems.iter().any(|p| p.contains("repeats cycles")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn non_string_digest_is_rejected() {
        let dir = scratch();
        let json = sha_tree(SHA256_32, false)
            .replace(&format!("\"digest\": \"{SHA256_32}\""), "\"digest\": 1");
        write_tree(&dir, &json, &sha_log(SHA256_32));
        let rep = verify(&dir);
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("sha256/32B") && p.contains("digest is not a string")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn capture_rejects_a_cycle_the_tables_still_agree_on() {
        let dir = scratch();
        // JSON, RUNS.md, and RESULTS.md all say 160. The log says 161.
        // µs stays 10, which is also 161 / 16, so only the cycle disagrees.
        write_tree(
            &dir,
            &sha_tree(SHA256_32, true),
            &sha_log(SHA256_32).replace("cycles=160", "cycles=161"),
        );
        let rep = verify(&dir);
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("sha256/32B") && p.contains("160") && p.contains("161")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn capture_rejects_a_digest_that_is_not_the_pattern() {
        let dir = scratch();
        let wrong = "00".repeat(32);
        write_tree(&dir, &sha_tree(&wrong, true), &sha_log(&wrong));
        let rep = verify(&dir);
        assert!(
            rep.problems.iter().any(|p| p.contains("digest_verified")
                && p.contains(&wrong)
                && p.contains(SHA256_32)),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unverified_digest_is_compared_to_the_log_only() {
        let dir = scratch();
        let wrong = "00".repeat(32);
        write_tree(&dir, &sha_tree(&wrong, false), &sha_log(&wrong));
        let rep = verify(&dir);
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn capture_digest_must_match_the_log() {
        let dir = scratch();
        let wrong = "00".repeat(32);
        write_tree(&dir, &sha_tree(&wrong, false), &sha_log(SHA256_32));
        let rep = verify(&dir);
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("digest") && p.contains(&wrong) && p.contains(SHA256_32)),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn capture_us_must_be_cycles_over_the_clock() {
        let dir = scratch();
        let log = sha_log(SHA256_32).replace("us=10", "us=11");
        write_tree(&dir, &sha_tree(SHA256_32, true), &log);
        let rep = verify(&dir);
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("us is 11") && p.contains("10")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn omitted_checksum_must_match_the_log() {
        let dir = scratch();
        let json = r#"{
  "clock_hz": 16000000,
  "ops": {
    "sha256/32B": {
      "n": 1,
      "cycles_all": [160],
      "cycles_median": 160,
      "cycles_min": 160,
      "cycles_max": 160,
      "us_median": 10
    }
  },
  "omitted_unreliable": {
    "chacha20/64B": {
      "reason": "unstable",
      "cycles_all": [34911],
      "checksums_le_hex": ["441e0000"]
    }
  }
}
"#;
        let log = format!(
            "BENCH sha256/32B cycles=160 us=10 digest={SHA256_32}\nBENCH chacha20/64B cycles=34911 us=2181 digest=441e0000\n"
        );
        write_tree(&dir, json, &log);
        // The headline table only has sha256. The omitted row is not in it.
        let rep = verify(&dir);
        assert!(rep.problems.is_empty(), "{}", rep.problems.join("\n"));

        let bad = log.replace("441e0000", "00000000");
        std::fs::write(dir.join("raw/capture.log"), bad).unwrap();
        let rep = verify(&dir);
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("checksums_le_hex") && p.contains("chacha20/64B")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_unlisted_bench_row_is_a_mismatch() {
        let dir = scratch();
        let log = format!(
            "{}\nBENCH chacha20/64B cycles=1 us=0 digest=00\n",
            sha_log(SHA256_32)
        );
        write_tree(&dir, &sha_tree(SHA256_32, true), &log);
        let rep = verify(&dir);
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("chacha20/64B") && p.contains("omitted")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn kat_failure_must_be_recorded() {
        let dir = scratch();
        let log = sha_log(SHA256_32).replace("KAT s256-empty PASS", "KAT s256-empty FAIL");
        write_tree(&dir, &sha_tree(SHA256_32, true), &log);
        let rep = verify(&dir);
        assert!(
            rep.problems.iter().any(|p| p.contains("s256-empty")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn published_capture_cycle_edit_is_a_mismatch() {
        let src = super::super::root().join("results/uno-r3");
        let dir = scratch();
        std::fs::create_dir_all(dir.join("raw")).unwrap();
        for name in ["medians.json", "RUNS.md", "RESULTS.md", "medians.md"] {
            std::fs::copy(src.join(name), dir.join(name)).unwrap();
        }
        let log = std::fs::read_to_string(src.join("raw/capture.log")).unwrap();
        let edited = log.replacen("cycles=61048", "cycles=61049", 1);
        assert_ne!(log, edited);
        std::fs::write(dir.join("raw/capture.log"), edited).unwrap();
        let rep = verify(&dir);
        assert!(
            rep.problems
                .iter()
                .any(|p| p.contains("sha256/32B") && p.contains("61048") && p.contains("61049")),
            "{:?}",
            rep.problems
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn hex_bytes(s: &str) -> [u8; 32] {
        let mut out = [0u8; 32];
        for i in 0..32 {
            out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
        }
        out
    }
}
