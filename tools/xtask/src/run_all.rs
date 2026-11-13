// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! `cargo xtask run-all` and `run-all-nrf`: flash each rind-hardware image in
//! turn and capture `--runs` runs of it.
//!
//! ESP32-S3: espflash writes the SPI flash clock into the image header at
//! flash time and does not read sdkconfig: without `--flash-freq` it is
//! 40 MHz, and the app keeps the bootloader's clock, so every row that misses
//! the instruction cache would run at half the flash bandwidth the build asked
//! for. The default here is 80 MHz.
//!
//! nRF52840: through the board's UF2 bootloader drive; the firmware's
//! 1200-baud touch enters it.

use crate::capture::{self, Board, NRF_PORT};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(clap::Args)]
pub struct Args {
    images: PathBuf,
    benches: Vec<String>,
    /// ESP32-S3 serial port (required for the ESP32-S3); no default
    #[arg(long, env = "PORT")]
    port: Option<String>,
    #[arg(long, env = "RUNS", default_value_t = 3)]
    runs: u32,
    /// default results/raw (ESP32-S3) or results/raw-nrf52840
    #[arg(long, env = "OUT")]
    out: Option<PathBuf>,
    #[arg(long, env = "FLASH_FREQ", default_value = "80mhz")]
    flash_freq: String,
    #[arg(long, env = "ESPFLASH", default_value = "espflash")]
    espflash: String,
    /// the nRF52840 bootloader's drive; default /run/media/$USER/T1000-E
    #[arg(long, env = "DRIVE")]
    drive: Option<PathBuf>,
}

fn benches(images: &Path, given: &[String], ext: &str) -> Result<Vec<String>, String> {
    if !given.is_empty() {
        return Ok(given.to_vec());
    }
    let mut v: Vec<String> = std::fs::read_dir(images)
        .map_err(|e| format!("{}: {e}", images.display()))?
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter_map(|n| n.strip_suffix(ext).map(String::from))
        .collect();
    v.sort();
    Ok(v)
}

fn capture_args(a: &Args, port: &str, out: PathBuf) -> capture::Args {
    use clap::Parser;
    #[derive(Parser)]
    struct W {
        #[command(flatten)]
        c: capture::Args,
    }
    let runs = a.runs.to_string();
    let out = out.display().to_string();
    W::parse_from(["capture", "--port", port, "--runs", &runs, "--out", &out]).c
}

fn wait(secs: u64, test: impl Fn() -> bool) -> bool {
    let end = Instant::now() + Duration::from_secs(secs);
    while !test() {
        if Instant::now() > end {
            return false;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    true
}

pub fn run(board: Board, a: Args) -> Result<(), String> {
    let (ext, out) = match board {
        Board::Esp => (".elf", "results/raw"),
        Board::Nrf => (".uf2", "results/raw-nrf52840"),
    };
    let out = a.out.clone().unwrap_or(out.into());
    for b in benches(&a.images, &a.benches, ext)? {
        println!("== {b}");
        let image = a.images.join(format!("{b}{ext}"));
        let port = match board {
            Board::Esp => {
                let esp_port = capture::esp_port(a.port.as_deref())?;
                let st = Command::new(&a.espflash)
                    .args([
                        "flash",
                        "--port",
                        &esp_port,
                        "--chip",
                        "esp32s3",
                        "--non-interactive",
                        "--flash-freq",
                    ])
                    .arg(&a.flash_freq)
                    .arg(&image)
                    .status()
                    .map_err(|e| format!("{}: {e}", a.espflash))?;
                if !st.success() {
                    return Err(format!("{b}: espflash failed"));
                }
                esp_port
            }
            Board::Nrf => {
                let drive = a.drive.clone().unwrap_or_else(|| {
                    PathBuf::from(format!(
                        "/run/media/{}/T1000-E",
                        std::env::var("USER").unwrap_or_default()
                    ))
                });
                if !drive.is_dir() && Path::new(NRF_PORT).exists() {
                    capture::nrf_to_bootloader(NRF_PORT);
                }
                if !wait(30, || drive.join("INFO_UF2.TXT").is_file()) {
                    return Err(format!("{b}: the bootloader drive did not appear"));
                }
                std::fs::copy(&image, drive.join(format!("{b}{ext}")))
                    .map_err(|e| format!("{b}: {e}"))?;
                // sync only this file system: a global sync can wait minutes on other disks.
                let _ = Command::new("sync")
                    .arg("-f")
                    .arg(&drive)
                    .stderr(std::process::Stdio::null())
                    .status();
                if !wait(30, || Path::new(NRF_PORT).exists()) {
                    return Err(format!("{b}: the firmware did not enumerate"));
                }
                NRF_PORT.to_string()
            }
        };
        let dir = out.join(&b);
        match capture::capture(board, capture_args(&a, &port, dir.clone())) {
            Ok(true) => {}
            Ok(false) => println!("{b}: a run failed; see {}", dir.join("runs.json").display()),
            Err(e) => println!("{b}: {e}; see {}", dir.join("runs.json").display()),
        }
    }
    Ok(())
}
