<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# pi5-nofan-1p5ghz: how this set was measured

The board is a Raspberry Pi 5 Model B Rev 1.0 (BCM2712, four Cortex-A76 cores, 8 GB, ARMv8 AES/PMULL/SHA-2 extensions, no SHA-3 extension, no DIT). The OS is Debian 13 (trixie), kernel 6.18.60+rpt-rpi-2712. See [facts.txt](facts.txt) for the rest of the hardware and toolchain.

## The 1.5 GHz clock

The board has no fan. A boot-time systemd service holds every cpufreq policy at 1.5 GHz (`scaling_min_freq` = `scaling_max_freq` = 1500000; the core is rated 2.4 GHz) so the SoC stays out of thermal throttling. The service was left in place. Every timed sample in this set ran at 1.5 GHz, and the `performance` governor set for the rounds made no difference to the clock. That is why the host is named `pi5-nofan-1p5ghz`: these absolute times are not comparable with a Pi 5 at 2.4 GHz, and they are not the `hosts/pi5/` set. The id is filesystem-safe, which is what a host id has to be, and the 1.5 GHz cap stays in the name so a 2.4 GHz Cortex-A76 measurement can sit beside it. Ratios between libraries at the same clock are what carry over.

A 2.4 GHz Pi 5 run at the same commit, with the same binaries and criterion arguments, was still in progress when this set was written. Over its first 3 of 5 rounds its times were about 1.60× faster than these on every operation compared, so the ratios hold at both clocks. That figure is **provisional**: the other run is not published here, and the 1.60× is to be replaced by its five-round median.

## How it was run

This set was not produced by rind's `admin/bench-report run`. The rind-bench criterion harnesses were cross-compiled on a macOS host for `aarch64-unknown-linux-gnu` with rustc/cargo 1.98.1, Homebrew clang 22, a Pi 5 sysroot, and rust-lld, then run on the board itself. `build_cost` was not measured.

- Five full rounds of the report's operation set (24 harnesses, 68 operations, 396 results per round), 2026-11-13 11:47 AM to 3:26 PM America/Los_Angeles (the board clock is on the same zone).
- Each harness was pinned to cpu3 with `taskset`. The harness order rotated each round.
- Criterion used the report's arguments plus `--sample-size 10`: `--noplot --warm-up-time 1 --measurement-time 3 --sample-size 10`.
- A lock directory on the board kept other agents' timed work off it during each round. Between rounds the driver waited while another lane held the lock; those waits are in [run.log](run.log) and no round overlapped them.
- The board also runs an idle Hailo accelerator service and an idle camera and audio stack. They were not stopped. No run was flagged noisy: other busy CPUs were 0.00–0.01 of a 1.0 limit in every round.

The libraries timed are rind, aws-lc-rs, ring, RustCrypto (with dalek), graviola, libcrux, and the Pi's system OpenSSL 3.5.7 (the binaries were linked against OpenSSL 3.5.9 headers and libraries from the sysroot and load 3.5.7 at run time).

Each published time is the median of all five rounds' criterion estimates. [RUNS.md](RUNS.md) lists every run. The raw files are [raw/run-1.json](raw/run-1.json) through [raw/run-5.json](raw/run-5.json), with each round's criterion output in `raw/run-<n>.log`.

## A power loss during round 3

The board lost power at about 1:41 PM, during the first attempt at round 3 (its last write was at 13:40:45 board time, in the `mlkem768` harness, 296 of 396 estimates done). It booted again at 13:50. `get_throttled` showed no undervoltage bit before or after. That partial round is not published and is in no median. Round 3 was then rerun in full from 13:52:32, followed by rounds 4 and 5. [run.log](run.log) marks the spot.

## Temperature and throttling

`vcgencmd` was sampled every 15 s from 12:02 PM to the end (round 1's first 15 minutes are covered only by its settled and after readings in the table in RESULTS.md). During the timed rounds the SoC read 48.8–75.2 °C and the clock read 1.5 GHz in every sample.

`get_throttled` read `0xe0000` in rounds 1 and 2: frequency capping, throttling and the soft temperature limit had occurred at some point since the board's previous boot, days earlier. None of the live bits (0–3) was set in any sample. After the power loss the register read `0x0` for rounds 3 to 5.

## Noise

Round-to-round spread per result, (slowest − fastest) / fastest: median 0.2%, 90th percentile 1.4%. One result is over 10%: `quic-header-protection/aes128/ring`, 13.9%, from its round-3 value (35.35 ns against 31.04–32.84 ns in the other rounds). It is left in; the published median, 31.70 ns, is the median of all five because `cargo verify` checks that the host JSON matches the median of the runs. The next largest spread is 8.9% (`sha256/graviola/64B`). rind's own rows are all within 2.7%.

## What the numbers say

rind is the fastest library on 45 of the 68 operations and fastest or within 5% on 49. The others are aws-lc-rs on 19, RustCrypto on 3, and ring on 1.

On stock `a6577cb4` the symmetric, hash, KDF and post-quantum rows lead or are within 3% of the fastest other library (SHA-512, at 1.02–1.03×, is the closest). The gaps over 10% (see [gaps.toml](gaps.toml)) are all public-key or handshake rows, and all against aws-lc-rs: X25519 2.32×, RSA-2048 about 1.69×, P-256/P-384/P-521 ECDH 1.54–1.66×, Ed25519 about 1.45×, ECDSA verify 1.19–1.32×, and the rustls handshakes that use them. These match the wide-multiplier rows of [`hosts/pi5/ab-narrow-mul/`](../../../hosts/pi5/ab-narrow-mul/README.md), measured on a Pi 5 at the same 1.5 GHz cap. For example, X25519 is 252.76 µs here and 252.7 µs there. RSA-2048 PKCS#1 sign is 5194.1 µs here, 0.3% under the A/B's wide figure (5208.5 µs in `ab-narrow-mul.md`, shown as 5209.2 µs on the aarch64 perf deck). The A/B shows the narrow-multiplier schedules (steelhead99x/rind PR #33 and follow-ups, not in `a6577cb4`) closing these gaps on this core.
