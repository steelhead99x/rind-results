<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-C6 upstream refresh A/B at kixel-ai/rind `8a5977ad`: pass 2, 2026-11-23

Recorded 2026-11-23, 23:44 UTC to 2026-11-24 00:31 UTC (15:44 to 16:31 America/Los_Angeles). This is the second of three passes on the same ESP32-C6, the board whose MAC ends `2C:48`. All 11 flashes printed that MAC. This is a new A/B directory. No published tree or number is touched.

## Run counts

- This pass: `ctl` has **six runs**, and each of `zero`, `aes`, `p256`, `cp` and `all` has **one run**. The flash order was `ctl zero ctl aes ctl p256 ctl cp ctl all ctl`.
- All three passes are pooled in [`…-refresh-ab-pass3-2026-11-23/ab-all-passes.md`](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-pass3-2026-11-23/ab-all-passes.md). There every candidate has **three runs**, which meets the three-run rule, and `ctl` has 18.
- There were 1,634 known-answer checks over the 11 boots and 0 failures.

## Method

The method is the same as in [8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-2026-11-23](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-2026-11-23/NOTES.md):

- The images are the same, unchanged by sha256 (`images.sha256` is identical to that tree's), and so are the order and the run script. Only the output folder differs.
- The commits, fork PRs, harness and toolchain are the ones that tree lists.
- Each flash was `espflash write-bin --after watchdog-reset 0x0`. As in pass 1, espflash printed `The esp32c6 does not support watchdog reset` every time, and the script then ran `espflash reset` and captured for 120 s. [run.log](run.log) is the timeline, with serial device paths and the MAC removed.
- The tables come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped, a flash's value is the median over its complete rounds, and a run is one flash. `cargo verify --dir` re-checks every table.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial ESP-IDF bootloader line at the start. Nothing was removed. No MAC, network name or key is in them.
