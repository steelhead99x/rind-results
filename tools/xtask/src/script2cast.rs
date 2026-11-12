// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask script2cast`: turns a `script --log-timing T -c CMD LOG`
//! recording into an asciinema v2 cast (play with `asciinema play`) and a
//! plain text log, replacing the home directory with `~` and dropping the
//! user and host names.

use crate::py;
use regex::Regex;
use serde_json::json;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Args {
    log: PathBuf,
    timing: PathBuf,
    out_stem: String,
    title: Option<String>,
}

pub fn run(a: Args) -> Result<(), String> {
    let title = a.title.clone().unwrap_or_else(|| {
        PathBuf::from(&a.out_stem)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    });
    let data = std::fs::read(&a.log).map_err(|e| format!("{}: {e}", a.log.display()))?;
    // script's "Script started" header line
    let start = data
        .iter()
        .position(|b| *b == b'\n')
        .ok_or("no header line")?
        + 1;
    let home = std::env::var("HOME").map_err(|_| "HOME is not set")?;
    let user = std::path::Path::new(&home)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let host = std::fs::read_to_string("/proc/sys/kernel/hostname").map_err(|e| e.to_string())?;
    let host = host.trim();
    let ure = Regex::new(&format!(r"\b{}\b", regex::escape(&user))).unwrap();
    let hre = Regex::new(&format!(r"\b{}\b", regex::escape(host))).unwrap();
    let clean = |s: &str| -> String {
        let s = s.replace(&home, "~");
        let s = ure.replace_all(&s, "user");
        hre.replace_all(&s, "host").into_owned()
    };
    let timing = std::fs::read_to_string(&a.timing).map_err(|e| e.to_string())?;
    let (mut events, mut t, mut pos) = (vec![], 0.0f64, start);
    for line in timing.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (kind, delay, n) = match parts.as_slice() {
            // advanced format: type delay bytes
            [k, d, n] if "OI".contains(k) => (*k, d.parse::<f64>(), n.parse::<usize>()),
            // classic format: delay bytes
            [d, n] => ("O", d.parse::<f64>(), n.parse::<usize>()),
            _ => continue,
        };
        let (Ok(delay), Ok(n)) = (delay, n) else {
            return Err(format!("bad timing line {line:?}"));
        };
        t += delay;
        if kind == "O" {
            let end = (pos + n).min(data.len());
            let chunk = &data[pos.min(end)..end];
            pos += n;
            let stamp: f64 = format!("{t:.3}").parse().unwrap();
            events.push(json!([stamp, "o", clean(&String::from_utf8_lossy(chunk))]));
        }
    }
    let mut cast = py::dumps(
        &json!({"version": 2, "width": 120, "height": 36, "title": title, "env": {"TERM": "xterm-256color"}}),
        None,
    ) + "\n";
    for e in &events {
        cast += &(py::dumps(e, None) + "\n");
    }
    std::fs::write(format!("{}.cast", a.out_stem), cast).map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&data[start..]).into_owned();
    let text = match text.rfind("\nScript done") {
        Some(i) => &text[..i],
        None => &text[..],
    };
    std::fs::write(
        format!("{}.txt", a.out_stem),
        clean(text).replace("\r\n", "\n"),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
