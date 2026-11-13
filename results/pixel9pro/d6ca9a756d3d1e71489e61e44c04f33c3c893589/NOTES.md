# Pixel 9 Pro comparable set at kixel-ai/rind `d6ca9a75`

Recorded 2026-11-13 (America/Los_Angeles, Pacific Standard Time).

## What this is

Three `admin/bench-report run`s on a Google Pixel 9 Pro (Tensor G4, aarch64, Android 17 in Termux) at unmodified upstream [kixel-ai/rind `d6ca9a756d3d1e71489e61e44c04f33c3c893589`](https://github.com/kixel-ai/rind/commit/d6ca9a756d3d1e71489e61e44c04f33c3c893589) (OPRF #705).

- Checkout: a fresh clone of kixel-ai/rind, `git checkout --detach d6ca9a75…`, submodules `thirdparty/cavp`, `thirdparty/cctv`, and `thirdparty/wycheproof` initialized. No fork commits and no patches. `git status --porcelain` was empty before the suite and again before timing.
- `commit` in the host JSON is that upstream SHA. There is no `fork_commit` and no `patches` list.
- Each published time is the median of the three runs' criterion estimates (`pixel9pro.json`, schema 1, each run's estimate in `runs_ns`). `ns_low` / `ns_high` are the min and max of those three estimates (`spread_is: "runs"`).
- [RUNS.md](RUNS.md) lists every run, [facts.txt](facts.txt) the hardware, [gaps.toml](gaps.toml) the two gaps over 10%.

This is not the historical `hosts/pixel9pro/` set at rind `b6651423`.

## How it was run

The suite ran in Termux. It is the same method as the later fork baseline on this phone, not `cargo xtask pixel-run --serial`. A Python wrapper, `scripts/host-run-android.py`, imported this checkout's unmodified `admin/bench-report` and ran:

`python3 scripts/host-run-android.py --host pixel9pro --commit d6ca9a756d3d1e71489e61e44c04f33c3c893589 --jobs 4 --target-dir <separate target dir> --out raw/run-N.json`

three times, with criterion `--noplot --warm-up-time 1 --measurement-time 3` and upstream's operation regex. The binaries were built on the phone. The run was not pinned to one core, so a benchmark could land on an X4, an A720, or an A520 core.

Before timing, host-kat known-answer checks ran (91 passed, 0 failed) and `cargo test --release -p rind` ran (814 passed, 0 failed). A KAT or test failure would have aborted timing.

## Conditions

The phone was on AC power the whole time (`PLUGGED_AC`). The battery still fell from 27% to 22%, and battery temperature went from 27.1 °C at the start of the suite to 35.0 °C at the end. Thermal zones could not be read. No run was flagged noisy by bench-report. 12 of 396 results had a run-to-run spread above 5%.

Suite window (America/Los_Angeles): started 10:00:37 AM PST, finished 12:14:26 PM PST. The three timing runs were 10:09:50 AM–11:09:51 AM, 11:09:55 AM–11:42:09 AM, and 11:42:11 AM–12:14:21 PM PST.

- CPU: Google Tensor G4 (Pixel 9 Pro), aarch64. Snapshots show `cpu_online` 0-7; bench-report recorded `logical_cpus` 7.
- OS: Android 17 (API 37, build CP3A.261005.005) in Termux
- Toolchain: rustc 1.96.0 (ac68faa20 2026-05-25), cargo 1.96.0, both built from a source tarball
- OpenSSL linked: OpenSSL 3.6.3 9 Jun 2026
- rind-cpuid: aes, pmull, sha2, sha3, neon, dit. The Tensor G4 also has SVE2; that is not a rind-cpuid key at this pin, and this unpatched build does not take an SVE2 ChaCha path.
- Libraries refused for missing CPU features: none
- `build_cost`: not measured (skipped by the host-run wrapper)
- The `rind (DIT held)` column is compiled only for Apple targets in rind-bench, so it is not timed here

## Gaps

OpenSSL is ahead on ChaCha20-Poly1305 at 16 KB (1.32×) and rustls bulk ChaCha20-Poly1305 (1.24×). On both rows rind is even with ring and aws-lc-rs. The reason is in [gaps.toml](gaps.toml): OpenSSL uses SVE2 on this chip and this upstream pin does not.
