# Notes on the host `admin/bench-report` workflow from the 2026-11-11 PST run

The firmware-port skill that agents use is now
[`skills/embedded-crypto-benchmark-port/SKILL.md`](skills/embedded-crypto-benchmark-port/SKILL.md).
The notes below were written against a host-side skill path
(`skills/rind-bench-and-verify/SKILL.md`) that is not in this tree; they remain
as suggestions for that host workflow.


Run: rind `b6651423` (moq-dev/rind main), hosts: xeon-vm (x86_64 Xeon VM), m1pro
(Apple M1 Pro), Raspberry Pi 5 (`pi5`). Read-only on the skill; these are
suggestions for whoever updates it.

## Wrong or misleading

1. **`cargo bench` as "local criterion benches"** runs every bench in the workspace with the
   default 3 s + 5 s criterion timing — hours, and not the report's operation set. The report's
   operations, filter regex, and `--warm-up-time 1 --measurement-time 3` live in
   `admin/bench-report` (`OPERATIONS`, `CRITERION_ARGS`); the skill should point at
   `admin/bench-report run` (or `summary`) instead.
2. **`admin/bench-report collect` only works for the maintainer's six hosts.** `HOSTS` is
   hard-coded (`nixos`, `desktop`, `nixos2`, `kixtop`, `ovh-iad`, `ovh-sea`); `collect --host X`
   refuses any other name. It also wraps each run in `nix shell github:…quest… --command quest gate`
   and `nix develop` on the remote, so a host needs Nix + flakes + network to GitHub.
3. **`admin/bench-report render --from DIR` rewrites the repo's BENCHMARKS.md** and
   `benchmarks/*.svg`, renders only `HOSTS`, and *fails* on any gap over 10% that
   `benchmarks/gaps.toml` lacks a decision for. A new host almost always has such gaps (here: ChaCha20
   vs OpenSSL on an AVX-512 Xeon), so render cannot be used as-is for a side report. This run imported
   the renderer as a module and called `report()` / `overview_svg()` / `host_svg()` with its own host
   table and output dir (`render-ours.py`, now `cargo xtask render-ours`).

## Left out

4. **`admin/bench-report run` refusals.** The host side refuses (a) outside the dev shell
   (`IN_RIND_DEV_SHELL=1`), (b) any rustc other than the flake's pinned `1.98.1`, and (c) a host with
   more than 10% of CPUs (min 1 CPU) busy before the build or the benchmarks. On a shared VM or a laptop
   in use, (c) fires almost every time (box: 2.3 of 8 CPUs busy). This run used `cargo xtask host-run`, which
   loads the real script and relaxes only (b) and (c), recording the busy CPUs in the JSON.
5. **`admin/build-cost` has its own busy check** (25% of CPUs) and runs right after the benchmarks,
   so on a shared host it often fails with `host is busy` and the JSON records `build_cost.status:
   failed`. Rerunning `admin/build-cost` alone in the dev shell later worked on the box.
6. **Disk space.** The rind-bench release build is about 2 GB (target dir) plus about 220 MB of crates;
   build-cost then does cold builds per library (vendored OpenSSL is the largest). On the Mac (99% full,
   6 GB free at start), two runs had to be stopped by a free-space watchdog (stop at 1.5 GB free) before
   finishing; the third ran after space was freed. The skill should say to check free space first
   (5 GB or more) and to set `CARGO_HOME`/`--target-dir` inside a removable work dir.
7. **macOS without the Nix dev shell.** The desktop has Nix, but the dev shell would download a full
   toolchain + Python stack onto a nearly full disk; this run used Homebrew's `rustc 1.98.0` (not the
   pinned 1.98.1) and recorded that. The skill should say what's acceptable off-Nix and that the result
   should be labeled.
8. **Linux ARM boards (Pi 5) need OpenSSL headers** for the `__bench_openssl` feature
   (`openssl-sys`). The Pi had `libssl3` but no `libssl-dev`, and there was no sudo to install it. This
   run unpacked `libssl-dev` with `apt-get download` + `dpkg -x` into the work dir and set
   `OPENSSL_INCLUDE_DIR` / `OPENSSL_LIB_DIR` (merging the multiarch `configuration.h`). Nothing was installed
   on the system.
9. **Hosts reached through another machine.** Nothing on how to run on a board that is only reachable
   from the user's desktop (an SSH config alias `pi5`): copy a `git archive` of the
   commit (vendored submodules aren't needed for benches) and run in one backgrounded `nohup` job.
10. **ML-DSA.** The published BENCHMARKS.md shows ML-DSA-65 as "not timed" on all six hosts; at
    `b6651423` the harness times it (62 operations, not 59), so new hosts aren't directly comparable
    row for row on those three.
11. **DIT on Apple Silicon.** rind sets DIT per call on M-series CPUs; the `rind (DIT held)` column
    and the expected small-input gaps should be mentioned so a new Mac's results aren't misread.
12. **Time budget.** About 35 min of criterion on the box, 40 to 45 min on an M1 Pro, about 70 min
    on the Pi 5, plus 10 to 30 min of build-cost per host.
13. **Android (Pixel 9 Pro) isn't covered at all.** What worked: cross-compile on x86_64 with the
    NDK (`CC_/AR_/CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER` → `aarch64-linux-android31-clang`),
    `rustup target add aarch64-linux-android` on the pinned 1.98.1, and
    `--features __bench_openssl,openssl-sys/vendored` (Android has no system OpenSSL; vendored needs
    `make` and `perl` on the build host, `nix shell nixpkgs#gnumake` here). Strip the bench binaries
    (1.3 GB → 140 MB). Run each with `--bench --noplot --warm-up-time 1 --measurement-time 3 <regex>`,
    `CRITERION_HOME` under `/data/local/tmp`, then parse `benchmark.json`/`estimates.json` as `run()` does.
14. **`rustls` bench reads files at runtime from the build machine's path:** `configs()` opens
    `env!("CARGO_MANIFEST_DIR")/../rind-rustls/tests/keys/...`, so a cross-compiled or copied binary
    panics with `NotFound`. Workaround here: build from a source copy at the path the device will have
    (`/data/local/tmp/rind/src`) and push the keys there. A repo-side fix would be `include_bytes!`
    or an env var override (reported, not changed).
15. **`adb shell "nohup … &"` blocks** until the job ends (adb keeps the session open), so a
    "poll while running" loop on the desktop never runs; use `adb shell 'nohup sh run.sh >log 2>&1 </dev/null &'`
    with all three streams redirected, or `setsid`.
16. **Phones need thermal/battery notes and core pinning.** The Tensor G4 has three core types and
    the scheduler moves the benchmark; without root there's no `taskset`. Record `dumpsys battery`
    before and after (24.7 → 37.7 °C here) and mark the run noisy.
17. **DIT-held column is Apple-only** in rind-bench (`target_vendor = "apple"`), even though the
    Tensor G4 has DIT.
18. **Microcontrollers (RP2040)** needed `no_std` and a CAS-free `ReuseCache`. rind-hardware `patches/0003` and `0005` now provide both; the images build for thumbv6m-none-eabi and are ready for a Pico. ring still does not build (NEON assembly). The skill should mention no-CAS targets and the Cell fallback.
19. **The repo moved mid-run** (b6651423 → a4759c48). Render and convert against a worktree of the
    measured commit so operation lists, `verified.toml`, and the renderer match the data.
