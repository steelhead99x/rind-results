// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! rind-results' tooling: `cargo xtask <command>` (an alias in
//! `.cargo/config.toml`). Every published table, JSON file, chart, OLED page
//! table and screenshot comes from one of these commands; `cargo verify`, a
//! separate crate that shares no code with this one, re-checks them.

mod bench;
mod boards;
mod capture;
mod charts;
mod compare;
mod demo_tsv;
mod host_report;
mod host_run;
mod layout;
mod legacy_map;
mod md2png;
mod py;
mod relink;
mod report;
mod run_all;
mod script2cast;
mod sizes;
mod tables;
mod upstream_run;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(about = "rind-results tooling", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Turn captured board runs into rind's report shape, RESULTS.md and RUNS.md
    Report(report::Args),
    /// Compare two sets of board runs of the same benches, row by row
    Compare(compare::Args),
    /// rind over the fastest other library across relinked (layout-shifted) builds
    Relink(relink::Args),
    /// Write the OLED demo's page table from a board's report JSON
    DemoTsv(demo_tsv::Args),
    /// Draw rind-over-fastest SVG charts from report JSON
    Charts(charts::Args),
    /// Attribute linked code and read-only data to the libraries raced
    Sizes(sizes::Args),
    /// Run the flashed ESP32-S3 firmware and record what it prints
    Capture(capture::Args),
    /// Run the flashed nRF52840 firmware and record what it prints
    CaptureNrf(capture::Args),
    /// Flash each ESP32-S3 image in turn and capture its runs
    RunAll(run_all::Args),
    /// Flash each nRF52840 image in turn through the UF2 bootloader and capture its runs
    RunAllNrf(run_all::Args),
    /// Screenshot a Markdown file or one section of it
    Md2png(md2png::Args),
    /// Turn a `script --log-timing` recording into an asciinema cast and a text log
    Script2cast(script2cast::Args),
    /// A Linux host's repeated bench-report runs into its median JSON, RESULTS.md and RUNS.md
    HostReport(host_report::Args),
    /// Run rind's own `admin/bench-report run` on this Linux host
    HostRun(host_run::Args),
    /// rustfmt --check, clippy -D warnings and the tests, for this workspace
    Check,
    /// Rewrite tools/bench-tables.json from a rind checkout, for review.
    /// Report commands evaluate the checkout they are given and do not need this.
    BenchTables {
        #[arg(long)]
        rind: PathBuf,
    },
    /// Measure one board: host, Android capture, or bare-metal images.
    Bench(bench::Args),
    /// Unpack the staged Jetson toolchain under ~/rind-bench-orin.
    OrinSetup {
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// One Jetson run. `n` is the run number; `core` pins that CPU, or the quietest of cpu1..cpu5.
    OrinRun {
        n: u32,
        core: Option<u32>,
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// Three Jetson runs. Run 1 picks the core unless `--core` is set; runs 2 and 3 reuse it.
    OrinRuns {
        #[arg(long)]
        core: Option<u32>,
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// Push the Pixel benches with adb, then pull the capture. `--serial` is required.
    PixelRun {
        #[arg(long)]
        serial: String,
        #[arg(long)]
        home: Option<PathBuf>,
        /// do not wait for ~/rind-bench-run/status
        #[arg(long)]
        no_wait: bool,
    },
    /// Pixel rustls second pass. `--serial` is required.
    PixelRun2 {
        #[arg(long)]
        serial: String,
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// M1 Pro run with the free-space watchdog.
    /// `--commit` defaults to the comparable-set pin. The published numbers are
    /// `--commit b66514238d449fc211d75fd14554301bb690e421`.
    MacRun {
        #[arg(long)]
        home: Option<PathBuf>,
        #[arg(long, default_value = layout::COMPARABLE_PIN)]
        commit: String,
    },
    /// Pi 5 run, unpacking libssl-dev locally if pkg-config cannot see it.
    /// `--commit` defaults to the comparable-set pin. The published numbers are
    /// `--commit b66514238d449fc211d75fd14554301bb690e421`.
    PiRun {
        #[arg(long)]
        home: Option<PathBuf>,
        #[arg(long, default_value = layout::COMPARABLE_PIN)]
        commit: String,
    },
    /// Re-check a result tree. With no flags, the published set.
    Verify {
        #[arg(long)]
        dir: Vec<PathBuf>,
        #[arg(long)]
        board: Vec<String>,
    },
    /// Turn an Android criterion capture into a host JSON.
    AndroidConvert {
        #[arg(long)]
        rind: PathBuf,
        #[arg(long)]
        capture: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        host: String,
        #[arg(long)]
        commit: String,
        #[arg(long)]
        rustc: String,
        #[arg(long)]
        cargo: String,
        #[arg(long)]
        target: String,
        #[arg(long)]
        cpu_model: String,
        #[arg(long)]
        logical_cpus: i64,
        #[arg(long, default_value = "")]
        openssl_linked: String,
    },
    /// Render host JSON with rind's bench-report tables, into a side directory.
    Render {
        #[arg(long)]
        rind: PathBuf,
        #[arg(long)]
        out: PathBuf,
        /// HOST FILE, repeated
        #[arg(long, num_args = 2, value_names = ["HOST", "FILE"])]
        note: Vec<String>,
        json: Vec<PathBuf>,
    },
    /// Stage a results tree from host JSON. Remaining arguments are stage-repo's.
    Stage {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        args: Vec<String>,
    },
    /// Print `results/<host>/<upstream-sha>[-<label>]/`, the directory a new publish uses.
    PublishDir {
        #[arg(long)]
        host: String,
        #[arg(long)]
        commit: String,
        #[arg(long)]
        label: Option<String>,
    },
}

fn main() -> ExitCode {
    let r = match Cli::parse().cmd {
        Cmd::Report(a) => report::run(a),
        Cmd::Compare(a) => compare::run(a),
        Cmd::DemoTsv(a) => demo_tsv::run(a),
        Cmd::Relink(a) => relink::run(a),
        Cmd::Charts(a) => charts::run(a),
        Cmd::Sizes(a) => sizes::run(a),
        Cmd::Capture(a) => return capture_exit(capture::capture(capture::Board::Esp, a)),
        Cmd::CaptureNrf(a) => return capture_exit(capture::capture(capture::Board::Nrf, a)),
        Cmd::RunAll(a) => run_all::run(capture::Board::Esp, a),
        Cmd::RunAllNrf(a) => run_all::run(capture::Board::Nrf, a),
        Cmd::Md2png(a) => md2png::run(a),
        Cmd::Script2cast(a) => script2cast::run(a),
        Cmd::HostReport(a) => host_report::run(a),
        Cmd::HostRun(a) => host_run::run(a),
        Cmd::Check => check(),
        Cmd::BenchTables { rind } => tables::refresh(&rind),
        Cmd::Bench(a) => bench::run(a),
        Cmd::OrinSetup { home } => (|| {
            let home = match home {
                Some(h) => h,
                None => boards::orin_home()?,
            };
            boards::orin_setup(&home)
        })(),
        Cmd::OrinRun { n, core, home } => (|| {
            let home = match home {
                Some(h) => h,
                None => boards::orin_home()?,
            };
            boards::orin_run(&home, n, core).map(|_| ())
        })(),
        Cmd::OrinRuns { core, home } => (|| {
            let home = match home {
                Some(h) => h,
                None => boards::orin_home()?,
            };
            boards::orin_runs(&home, core)
        })(),
        Cmd::PixelRun {
            serial,
            home,
            no_wait,
        } => (|| {
            let base = boards::home_dir()?.join("rind-bench-run");
            let work = home.unwrap_or_else(|| base.join("pixel"));
            boards::pixel_run(boards::PixelOpts {
                serial,
                pkg_tar: work.join("../pixel-pkg.tar.gz"),
                remote: "/data/local/tmp/rind".into(),
                second_pass: false,
                wait_for: if no_wait {
                    None
                } else {
                    Some(base.join("status"))
                },
                work,
            })
        })(),
        Cmd::PixelRun2 { serial, home } => (|| {
            let base = boards::home_dir()?.join("rind-bench-run");
            let work = home.unwrap_or_else(|| base.join("pixel2"));
            boards::pixel_run(boards::PixelOpts {
                serial,
                pkg_tar: work.join("../pixel-pkg2.tar.gz"),
                remote: "/data/local/tmp/rind".into(),
                second_pass: true,
                wait_for: None,
                work,
            })
        })(),
        Cmd::MacRun { home, commit } => (|| {
            let home = home.unwrap_or(boards::home_dir()?.join("rind-bench-run"));
            boards::mac_run(&home, &commit)
        })(),
        Cmd::PiRun { home, commit } => (|| {
            let home = home.unwrap_or(boards::home_dir()?.join("rind-bench-run"));
            boards::pi_run(&home, &commit)
        })(),
        Cmd::Verify { dir, board } => run_verify(&dir, &board),
        Cmd::AndroidConvert {
            rind,
            capture,
            out,
            host,
            commit,
            rustc,
            cargo,
            target,
            cpu_model,
            logical_cpus,
            openssl_linked,
        } => {
            let args = android_convert::ConvertArgs {
                rind,
                capture,
                out,
                host,
                commit,
                rustc,
                cargo,
                target,
                cpu_model,
                logical_cpus,
                openssl_linked,
            };
            match android_convert::convert(&args) {
                Ok(converted) => match android_convert::write_out(&args, &converted) {
                    Ok(()) => {
                        print!("{}", converted.stdout);
                        Ok(())
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        }
        Cmd::Render {
            rind,
            out,
            note,
            json,
        } => {
            let mut notes = vec![];
            let mut it = note.into_iter();
            while let (Some(host), Some(file)) = (it.next(), it.next()) {
                notes.push((host, PathBuf::from(file)));
            }
            match render_side::render(&render_side::RenderArgs {
                rind,
                out,
                notes,
                json,
            }) {
                Ok(rendered) => {
                    println!("{}", rendered.sheet.display());
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Cmd::Stage { args } => {
            let code = stage_repo::main_with_args(args);
            if code == ExitCode::SUCCESS {
                Ok(())
            } else {
                Err("stage-repo failed".into())
            }
        }
        Cmd::PublishDir {
            host,
            commit,
            label,
        } => layout::publish_dir(&root(), &host, &commit, label.as_deref())
            .map(|p| println!("{}", p.display())),
    };
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn run_verify(dirs: &[PathBuf], boards: &[String]) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let sib = exe.with_file_name("verify");
    let mut cmd = if sib.is_file() {
        Command::new(sib)
    } else {
        let mut c = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()));
        c.args(["run", "-q", "--release", "-p", "verify", "--"]);
        c
    };
    for d in dirs {
        cmd.arg("--dir").arg(d);
    }
    for b in boards {
        cmd.arg("--board").arg(b);
    }
    let status = cmd.status().map_err(|e| format!("verify: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("verify: {status}"))
    }
}

fn capture_exit(r: Result<bool, String>) -> ExitCode {
    match r {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn check() -> Result<(), String> {
    for args in [
        &["fmt", "--all", "--check"][..],
        &[
            "clippy",
            "-q",
            "--release",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        &["test", "-q", "--release"],
    ] {
        let st = Command::new(std::env::var("CARGO").unwrap_or("cargo".into()))
            .args(args)
            .current_dir(root())
            .status()
            .map_err(|e| format!("cargo: {e}"))?;
        if !st.success() {
            return Err(format!("cargo {} failed", args.join(" ")));
        }
        println!("cargo {}: ok", args[0]);
    }
    Ok(())
}

/// The repository root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

pub fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Today's local date, `YYYY-MM-DD`.
pub fn today() -> String {
    Command::new("date")
        .arg("+%F")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Now in UTC as Python's `datetime.now(timezone.utc).isoformat(timespec="seconds")`.
pub fn utc_now() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs()) as i64;
    let (days, rem) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    // Howard Hinnant's civil_from_days.
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}+00:00",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}
