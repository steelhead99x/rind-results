<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-C6 upstream refresh A/B at kixel-ai/rind `8a5977ad`: pass 3, with all three passes pooled, 2026-11-23

Recorded 2026-11-24, 00:31 to 01:18 UTC (2026-11-23, 16:31 to 17:18 America/Los_Angeles). This is the third of three passes on the same ESP32-C6, the board whose MAC ends `2C:48`. All 11 flashes printed that MAC. This is a new A/B directory. No published tree or number is touched.

## Run counts and the three-run rule

- This pass: `ctl` has **six runs**, and each candidate has **one run**. `esp32c6.json`, RUNS.md, RESULTS.md and [ab.md](ab.md) are this pass only.
- [ab-all-passes.md](ab-all-passes.md) pools [pass 1](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-2026-11-23/NOTES.md), [pass 2](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-pass2-2026-11-23/NOTES.md) and this pass. It reads the logs in place. Pooled, each of `zero`, `aes`, `p256`, `cp` and `all` has **three runs**, which **meets the three-run rule**. `ctl` has **18**. `cargo verify` checks every cell of it against `ab-all-passes.json`.

## What the three passes show

From [ab-all-passes.md](ab-all-passes.md) and `ab-all-passes.json`:

- The ChaCha20-Poly1305 and AES-128-GCM medians are the same in all three passes, to within 0.066% for any image and row (the worst is `p256` 243 B seal). Control 1500 B seal is 810.50 µs and `all` 435.12 µs in every pass.
- The pass-1 ratios stand. `all` seals 1500 B at 0.537 of the control and 16 B at 0.649. `cp` seals 1500 B at 0.580. `aes` runs AES-128-GCM 1500 B seal at 0.582. `p256` signs at 0.857, and `all` at 0.864. On its own, `aes` opens 16 B ChaCha20-Poly1305 at 1.10 of the control.
- `p256_verify` (8 calls a sample) moves the most. The control's spread over its 18 runs is 0.59%. The candidates are 1.000 to 1.010 of the control, inside that spread.
- Every Ed25519 row repeats exactly (spread at most 0.003%). It is 0.992 to 1.011 of the control across the images.
- There were 1,634 known-answer checks over this pass's 11 boots and 0 failures.

## Method

The method is the same as in [8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-2026-11-23](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-2026-11-23/NOTES.md):

- The images are the same, unchanged by sha256 (`images.sha256` is identical to that tree's), and so are the order and the run script. Only the output folder differs.
- The commits, fork PRs, harness and toolchain are the ones that tree lists.
- Each flash was `espflash write-bin --after watchdog-reset 0x0`. As in pass 1, espflash printed `The esp32c6 does not support watchdog reset` every time, and the script then ran `espflash reset` and captured for 120 s. [run.log](run.log) is the timeline, with serial device paths and the MAC removed.
- The tables come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped, a flash's value is the median over its complete rounds, and a run is one flash. `cargo verify --dir` re-checks every table.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial ESP-IDF bootloader line at the start. Nothing was removed. No MAC, network name or key is in them.
