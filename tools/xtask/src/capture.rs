// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask capture` and `capture-nrf`: run the flashed rind-hardware
//! firmware and record what it prints.
//!
//! Each run writes the console log and the `RB:` JSON lines to
//! `<out>/run-<n>.log` and `<out>/run-<n>.jsonl`, and every run's outcome to
//! `<out>/runs.json`. A run that reboots, panics or times out is recorded as
//! failed, never retried silently. Flash the image first; see rind-hardware's
//! README.
//!
//! - ESP32-S3: each run resets the board through the USB Serial/JTAG port's
//!   RTS line and reads until the firmware's `{"type":"done"}` line.
//! - nRF52840: the firmware enumerates as a USB CDC-ACM port ("rind-bench
//!   nRF52840") and waits for `rind-bench:go` before it starts. Each run
//!   restarts it (300 baud, then drop DTR), waits for it to come back and for
//!   ModemManager to finish probing the new port (it would read some of the
//!   output), sends the go line, and reads until the done line. A run whose
//!   first record is not the start record is failed: another reader took
//!   lines.
//!
//! `--replay LOG` rebuilds a run's `.jsonl` from its `.log` instead, which is
//! how the records' serialization is checked against the published runs.

use crate::py;
use serde_json::{Value, json};
use serialport::{ClearBuffer, SerialPort, TTYPort};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub const NRF_PORT: &str = "/dev/serial/by-id/usb-rind-bench_rind-bench_nRF52840_rind-bench-if00";
const PANIC_MARKS: &[&str] = &[
    "abort() was called",
    "Guru Meditation",
    "panicked at",
    "Rebooting...",
];
const GO: &[u8] = b"rind-bench:go\n";
const SETTLE: Duration = Duration::from_secs(15);

#[derive(Clone, Copy, PartialEq)]
pub enum Board {
    Esp,
    Nrf,
}

#[derive(clap::Args)]
pub struct Args {
    /// default <PORT> (ESP32-S3) or the rind-bench CDC port (nRF52840)
    #[arg(long)]
    port: Option<String>,
    #[arg(long, default_value_t = 3)]
    runs: u32,
    /// default results/raw (ESP32-S3) or results/raw-nrf52840
    #[arg(long)]
    out: Option<PathBuf>,
    /// seconds; default 5400 (ESP32-S3) or 7200
    #[arg(long)]
    timeout: Option<u64>,
    /// print the `.jsonl` a run's `.log` gives, and capture nothing
    #[arg(long)]
    replay: Option<PathBuf>,
}

pub struct Outcome {
    lines: Vec<String>,
    records: Vec<Value>,
    status: &'static str,
    reason: Option<String>,
}

fn open(path: &str, baud: u32) -> serialport::Result<TTYPort> {
    serialport::new(path, baud)
        .timeout(Duration::from_millis(500))
        .open_native()
}

fn read_some(port: &mut TTYPort, buf: &mut Vec<u8>) -> std::io::Result<()> {
    let mut chunk = [0u8; 4096];
    match port.read(&mut chunk) {
        Ok(n) => {
            buf.extend_from_slice(&chunk[..n]);
            Ok(())
        }
        Err(e)
            if e.kind() == std::io::ErrorKind::TimedOut
                || e.kind() == std::io::ErrorKind::Interrupted =>
        {
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Splits complete lines off `buf` as Python's `decode("utf-8", "replace").rstrip("\r")`.
fn take_line(buf: &mut Vec<u8>) -> Option<String> {
    let i = buf.iter().position(|b| *b == b'\n')?;
    let raw: Vec<u8> = buf.drain(..=i).collect();
    let s = String::from_utf8_lossy(&raw[..raw.len() - 1]).into_owned();
    Some(s.trim_end_matches('\r').to_string())
}

fn stamp(started: Instant) -> String {
    format!("{:10.3}", started.elapsed().as_secs_f64())
}

fn timeout_reason(t: u64) -> Option<String> {
    Some(format!("no done line within {t} s"))
}

fn run_esp(path: &str, timeout: u64) -> Result<Outcome, String> {
    let mut o = Outcome {
        lines: vec![],
        records: vec![],
        status: "timeout",
        reason: timeout_reason(timeout),
    };
    let mut port = open(path, 115200).map_err(|e| format!("{path}: {e}"))?;
    port.clear(ClearBuffer::Input).map_err(|e| e.to_string())?;
    // The USB Serial/JTAG peripheral resets the chip when RTS is asserted with
    // DTR clear, as esptool's hard reset does.
    port.write_data_terminal_ready(false)
        .map_err(|e| e.to_string())?;
    port.write_request_to_send(true)
        .map_err(|e| e.to_string())?;
    std::thread::sleep(Duration::from_millis(100));
    port.write_request_to_send(false)
        .map_err(|e| e.to_string())?;
    let started = Instant::now();
    let mut buf = vec![];
    let mut booted = false;
    while started.elapsed().as_secs_f64() < timeout as f64 {
        read_some(&mut port, &mut buf).map_err(|e| e.to_string())?;
        while let Some(line) = take_line(&mut buf) {
            o.lines.push(format!("{} {line}", stamp(started)));
            if let Some(rec) = line.strip_prefix("RB:") {
                let rec: Value = match serde_json::from_str(rec) {
                    Ok(r) => r,
                    Err(e) => {
                        return Ok(Outcome {
                            status: "failed",
                            reason: Some(format!("bad RB line: {e}")),
                            ..o
                        });
                    }
                };
                let ty = rec
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                o.records.push(rec);
                if ty == "start" {
                    booted = true;
                }
                if ty == "done" {
                    return Ok(Outcome {
                        status: "ok",
                        reason: None,
                        ..o
                    });
                }
            } else if booted && PANIC_MARKS.iter().any(|m| line.contains(m)) {
                o.status = "failed";
                o.reason = Some(line.trim().to_string());
                // Keep the backtrace that follows.
                let end = Instant::now() + Duration::from_secs(2);
                while Instant::now() < end {
                    let _ = read_some(&mut port, &mut buf);
                }
                o.lines
                    .extend(String::from_utf8_lossy(&buf).lines().map(String::from));
                return Ok(o);
            }
        }
    }
    Ok(o)
}

fn wait_for(path: &str, present: bool, secs: u64) -> bool {
    let end = Instant::now() + Duration::from_secs(secs);
    while Instant::now() < end {
        if Path::new(path).exists() == present {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    false
}

fn restart_nrf(path: &str) -> Result<(), String> {
    // The firmware restarts when DTR drops on a port set to 300 baud.
    if !wait_for(path, true, 20) {
        return Err(format!("{path} is not there; is the image flashed?"));
    }
    // It may restart as soon as the line coding arrives, before DTR rises, so
    // the port can vanish under us here.
    if let Ok(mut p) = open(path, 300) {
        let _ = p.write_data_terminal_ready(true);
        std::thread::sleep(Duration::from_millis(100));
        let _ = p.write_data_terminal_ready(false);
    }
    wait_for(path, false, 5);
    if !wait_for(path, true, 20) {
        return Err(format!("{path} did not come back after the restart"));
    }
    std::thread::sleep(SETTLE);
    Ok(())
}

/// 1200 baud, then drop DTR: the nRF firmware resets into the UF2 bootloader.
pub fn nrf_to_bootloader(path: &str) {
    if let Ok(mut p) = open(path, 1200) {
        let _ = p.write_data_terminal_ready(false);
    }
}

fn run_nrf(path: &str, timeout: u64) -> Result<Outcome, String> {
    let mut o = Outcome {
        lines: vec![],
        records: vec![],
        status: "timeout",
        reason: timeout_reason(timeout),
    };
    restart_nrf(path)?;
    let mut port = open(path, 115200).map_err(|e| format!("{path}: {e}"))?;
    std::io::Write::write_all(&mut port, GO).map_err(|e| e.to_string())?;
    let started = Instant::now();
    let mut buf = vec![];
    while started.elapsed().as_secs_f64() < timeout as f64 {
        if let Err(e) = read_some(&mut port, &mut buf) {
            return Ok(Outcome {
                status: "failed",
                reason: Some(format!("port lost: {e}")),
                ..o
            });
        }
        while let Some(line) = take_line(&mut buf) {
            o.lines.push(format!("{} {line}", stamp(started)));
            let Some(rec) = line.strip_prefix("RB:") else {
                continue;
            };
            let rec: Value = match serde_json::from_str(rec) {
                Ok(r) => r,
                Err(e) => {
                    return Ok(Outcome {
                        status: "failed",
                        reason: Some(format!("bad RB line: {e}")),
                        ..o
                    });
                }
            };
            let ty = rec
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let msg = rec
                .get("message")
                .and_then(|v| v.as_str())
                .map(String::from);
            o.records.push(rec);
            if o.records[0].get("type").and_then(|v| v.as_str()) != Some("start") {
                let reason = Some("the first record was not the start record".into());
                return Ok(Outcome {
                    status: "failed",
                    reason,
                    ..o
                });
            }
            if ty == "done" {
                return Ok(Outcome {
                    status: "ok",
                    reason: None,
                    ..o
                });
            }
            if ty == "panic" {
                return Ok(Outcome {
                    status: "failed",
                    reason: msg,
                    ..o
                });
            }
        }
    }
    Ok(o)
}

fn jsonl(records: &[Value]) -> String {
    records.iter().map(|r| py::dumps(r, None) + "\n").collect()
}

/// The `.jsonl` a run's `.log` gives: its `RB:` lines, re-serialized.
fn replay(log: &Path) -> Result<String, String> {
    let text =
        String::from_utf8_lossy(&std::fs::read(log).map_err(|e| e.to_string())?).into_owned();
    let mut out = String::new();
    for line in text.split('\n') {
        // "%10.3f " stamps; the backtrace lines after a panic have none.
        let Some((_, rest)) = line.trim_start().split_once(' ') else {
            continue;
        };
        if let Some(rec) = rest.strip_prefix("RB:") {
            let v: Value =
                serde_json::from_str(rec).map_err(|e| format!("{}: {e}", log.display()))?;
            out += &(py::dumps(&v, None) + "\n");
        }
    }
    Ok(out)
}

/// Captures `runs` runs; returns whether every one was ok.
pub fn capture(board: Board, args: Args) -> Result<bool, String> {
    if let Some(log) = &args.replay {
        print!("{}", replay(log)?);
        return Ok(true);
    }
    let (port, out, timeout) = match board {
        Board::Esp => ("<PORT>", "results/raw", 5400),
        Board::Nrf => (NRF_PORT, "results/raw-nrf52840", 7200),
    };
    let port = args.port.unwrap_or(port.into());
    let out = args.out.unwrap_or(out.into());
    let timeout = args.timeout.unwrap_or(timeout);
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let mut summary = vec![];
    for n in 1..=args.runs {
        let began = crate::utc_now();
        eprintln!("run {n}: started {began}");
        let o = match board {
            Board::Esp => run_esp(&port, timeout),
            Board::Nrf => run_nrf(&port, timeout),
        };
        // A run that cannot start (the port is gone, the board did not come
        // back) is recorded as failed like any other, and runs.json still
        // gets written.
        let o = o.unwrap_or_else(|e| Outcome {
            lines: vec![],
            records: vec![],
            status: "failed",
            reason: Some(e),
        });
        let finished = crate::utc_now();
        std::fs::write(out.join(format!("run-{n}.log")), o.lines.join("\n") + "\n")
            .map_err(|e| e.to_string())?;
        std::fs::write(out.join(format!("run-{n}.jsonl")), jsonl(&o.records))
            .map_err(|e| e.to_string())?;
        let results = o
            .records
            .iter()
            .filter(|r| r.get("type").and_then(|v| v.as_str()) == Some("result"))
            .count();
        eprintln!(
            "run {n}: {} ({results} results) {}",
            o.status,
            o.reason.as_deref().unwrap_or("")
        );
        summary.push(
            json!({"run": n, "started": began, "finished": finished, "status": o.status,
                            "reason": o.reason, "results": results}),
        );
    }
    let ok = summary.iter().all(|s| s["status"] == "ok");
    std::fs::write(
        out.join("runs.json"),
        py::dumps(&Value::Array(summary), Some(2)) + "\n",
    )
    .map_err(|e| e.to_string())?;
    Ok(ok)
}
