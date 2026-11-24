<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 upstream refresh A/B at kixel-ai/rind `8a5977ad`, 80 MHz: third session, with all three pooled, 2026-11-23

Recorded 2026-11-23, 23:44 UTC to 2026-11-24 00:46 UTC (15:44 to 16:46 America/Los_Angeles). This is the third session on the same ESP32-S3, the board whose MAC ends `3D:C8`. All 15 flashes printed that MAC. It follows [the 2026-11-22 session](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-2026-11-22/NOTES.md) and [the 2026-11-23 repeat](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-repeat-2026-11-23/NOTES.md). This is a new A/B directory. No published tree or number is touched.

## Run counts and the three-run rule

- This session: `ctl` has **eight runs**, and each candidate has **one run**. `esp32s3.json`, RUNS.md, RESULTS.md and [ab.md](ab.md) are this session only.
- [ab-three-sessions.md](ab-three-sessions.md) pools all three sessions. Each of `zero`, `rot`, `notag`, `poly`, `all`, `xor` and `polyw` has **three runs**, which **meets the three-run rule**. `ctl` has **24**.
- There were 706 known-answer checks over this session's 15 boots and 0 failures.

## What the three sessions show

From [ab-three-sessions.md](ab-three-sessions.md):

- Every rind row's spread over its three (or 24) runs is at most 0.033% (`poly` 255 B open).
- Every session's median for every row, library and image is within 0.044% of the first session's.
- `all` seals 16 B at 0.521 of the control and 1500 B at 0.467. `zero` seals 16 B at 0.654. `rot` seals 1500 B at 0.851, `xor` at 0.885, `polyw` at 0.745, and `notag` and `poly` at 0.650 and 0.651. `xor` alone is 1.052 at 16 B seal.
- Ed25519 is within 1.03% of the control in every image. rind's SHA-512 rows are 0.9996 to 1.0000.

## Method

The method is the same as in [8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-2026-11-22](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-2026-11-22/NOTES.md):

- The images are the same, unchanged by sha256 (`images.sha256` is identical to that tree's), and so are the order and the run script. Only the output folder differs.
- The commits, fork PRs, harness and toolchain are the ones that tree lists.
- Each flash was `espflash write-bin --after watchdog-reset 0x0`, and every one printed `Binary successfully written to flash!`. [run.log](run.log) is the timeline, with serial device paths and the MAC removed.
- The tables come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped, a flash's value is the median over its complete rounds, and a run is one flash. `cargo verify --dir` re-checks every table.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines. Nothing was removed. No MAC, network name or key is in them.
