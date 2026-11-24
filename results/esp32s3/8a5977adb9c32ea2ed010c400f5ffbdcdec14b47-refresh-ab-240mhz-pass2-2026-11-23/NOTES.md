<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 upstream refresh A/B at kixel-ai/rind `8a5977ad`, 240 MHz: second session, 2026-11-23

Recorded 2026-11-24, 00:46 to 01:16 UTC (2026-11-23, 16:46 to 17:16 America/Los_Angeles). This is the second 240 MHz session on the same ESP32-S3, the board whose MAC ends `3D:C8`. All 11 flashes printed that MAC. This is a new A/B directory. No published tree or number is touched.

## Run counts

- This session: `ctl` has **six runs**, and each of `zero`, `rot`, `notag`, `poly` and `all` has **one run**. The flash order was `ctl zero ctl rot ctl notag ctl poly ctl all ctl`.
- All three 240 MHz sessions are pooled in [`…-refresh-ab-240mhz-pass3-2026-11-23/ab-three-sessions.md`](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-pass3-2026-11-23/ab-three-sessions.md). There every candidate has **three runs**, which meets the three-run rule.
- There were 396 known-answer checks over the 11 boots and 0 failures.

## Method

The method is the same as in [8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-2026-11-22](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-2026-11-22/NOTES.md):

- The images are the same, unchanged by sha256 (`images.sha256` is identical to that tree's), and so are the order and the run script. Only the output folder differs.
- The commits, fork PRs, harness and toolchain are the ones that tree lists.
- Each flash was `espflash write-bin --after watchdog-reset 0x0`, and every one printed `Binary successfully written to flash!`. [run.log](run.log) is the timeline, with serial device paths and the MAC removed.
- The tables come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped, a flash's value is the median over its complete rounds, and a run is one flash. `cargo verify --dir` re-checks every table.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines. Nothing was removed. No MAC, network name or key is in them.
