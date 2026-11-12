// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask demo-tsv`: the OLED demo's results table from report's
//! `esp32s3.json`: one page per report operation rind and another library
//! ran, in the report's section order, as `heading<TAB>library<TAB>ns` lines.

use crate::py;
use crate::tables::Tables;
use regex::Regex;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long)]
    rind: PathBuf,
    /// every operation; by default byte-size rows only at 32B and 1350B
    #[arg(long)]
    all: bool,
    host: PathBuf,
}

const ORDER: &[&str] = &["rind", "ring", "rustcrypto", "libcrux", "esp32s3-hw"];

/// The title bar fits 25 columns of 5x8 text, less the page counter
/// ("27/27"), so long names are shortened rather than cut: a cut once merged
/// the three QUIC header-protection pages (one heading) and showed 1350B as
/// "135".
const SHORT: &[(&str, &str)] = &[
    ("quic-header-protection", "quic-hp"),
    ("xchacha20poly1305", "xchachapoly"),
    ("chacha20poly1305", "chachapoly"),
    ("aes128-gcm", "aes128gcm"),
    ("aes256-gcm", "aes256gcm"),
    ("-open", " open"),
    ("extract-expand", "ext+exp"),
    ("rustls-full-handshake-group", "tls-full"),
    ("rustls-full-handshake-key", "tls-full"),
    ("rustls-resumed-handshake", "tls-resumed"),
    ("rustls-bulk", "tls-bulk"),
    ("pkcs1-sha256", "pkcs1"),
    ("pss-sha256", "pss"),
];

fn heading(op: &str) -> String {
    let mut h = op.replace("/{lib}", "").replace("{lib}/", "");
    for (a, b) in SHORT {
        h = h.replace(a, b);
    }
    h.replace('/', " ").to_uppercase()
}

/// Title text from x=2 must end before the right-aligned counter at x=126.
fn fits(h: &str, page: &str) -> bool {
    2 + 5 * h.chars().count() < 126 - 5 * page.chars().count() - 2
}

pub fn run(args: Args) -> Result<(), String> {
    let t = Tables::load(&args.rind)?;
    let host = crate::read_json(&args.host)?;
    let mut by: Vec<(String, Vec<(String, f64)>)> = vec![];
    for r in host["results"].as_array().into_iter().flatten() {
        let op = r["operation"].as_str().unwrap_or_default().to_string();
        let lib = r["library"].as_str().unwrap_or_default().to_string();
        let ns = r["ns"].as_f64().unwrap_or(f64::NAN);
        let i = match by.iter().position(|(o, _)| *o == op) {
            Some(i) => i,
            None => {
                by.push((op, vec![]));
                by.len() - 1
            }
        };
        let libs = &mut by[i].1;
        match libs.iter_mut().find(|(l, _)| *l == lib) {
            Some(e) => e.1 = ns,
            None => libs.push((lib, ns)),
        }
    }
    let size = Regex::new(r"/(\d+[KM]?B)$").unwrap();
    let mut lines: Vec<(String, Vec<(String, f64)>)> = vec![];
    for op in t.operations() {
        let libs = by
            .iter()
            .find(|(o, _)| o == op)
            .map(|(_, l)| l.clone())
            .unwrap_or_default();
        if !libs.iter().any(|(l, _)| l == "rind") || libs.len() < 2 {
            continue;
        }
        if let Some(c) = size.captures(op) {
            if !args.all && !["32B", "1350B"].contains(&&c[1]) {
                continue;
            }
        }
        let mut bars = libs;
        bars.sort_by_key(|(l, _)| ORDER.iter().position(|o| o == l).unwrap_or(99));
        lines.push((heading(op), bars));
    }
    let heads: Vec<&String> = lines.iter().map(|(h, _)| h).collect();
    let mut dup: Vec<&String> = heads
        .iter()
        .filter(|h| heads.iter().filter(|x| x == h).count() > 1)
        .copied()
        .collect();
    dup.sort();
    dup.dedup();
    if !dup.is_empty() {
        let d: Vec<String> = dup.iter().map(|h| format!("'{h}'")).collect();
        return Err(format!(
            "duplicate OLED headings [{}]: extend SHORT",
            d.join(", ")
        ));
    }
    for (n, h) in heads.iter().enumerate() {
        if !fits(h, &format!("{}/{}", n + 1, heads.len())) {
            return Err(format!(
                "OLED heading '{h}' overlaps its page counter: extend SHORT"
            ));
        }
    }
    let mut out = String::from("# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0\n");
    out += &format!(
        "# rind-bench OLED demo pages, from {}\n",
        args.host.display()
    );
    for (h, bars) in &lines {
        for (lib, ns) in bars {
            out += &format!("{h}\t{lib}\t{}\n", py::fixed(*ns, 1));
        }
    }
    print!("{out}");
    Ok(())
}
