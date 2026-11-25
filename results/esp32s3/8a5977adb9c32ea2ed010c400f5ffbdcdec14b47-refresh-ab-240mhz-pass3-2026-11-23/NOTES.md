<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 upstream refresh A/B at kixel-ai/rind `8a5977ad`, 240 MHz: third session, with all three pooled, 2026-11-23

Recorded 2026-11-24, 01:16 to 01:45 UTC (2026-11-23, 17:16 to 17:45 America/Los_Angeles). This is the third 240 MHz session on the same ESP32-S3, the board whose MAC ends `3D:C8`. All 11 flashes printed that MAC. This is a new A/B directory. No published tree or number is touched.

## Run counts and the three-run rule

- This session: `ctl` has **six runs**, and each candidate has **one run**.
- [ab-three-sessions.md](ab-three-sessions.md) pools [the 2026-11-22 session](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-2026-11-22/NOTES.md), [the second session](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-pass2-2026-11-23/NOTES.md) and this one. Each of `zero`, `rot`, `notag`, `poly` and `all` has **three runs**, which **meets the three-run rule**. `ctl` has **18**.
- There were 396 known-answer checks over this session's 11 boots and 0 failures.

## What the three sessions show

From [ab-three-sessions.md](ab-three-sessions.md):

- Every rind row's spread over its runs is at most 0.10% (`poly` 243 B open).
- `all` seals 16 B at 0.521 of the control and 1500 B at 0.467. `zero` seals 16 B at 0.654, `rot` 1500 B at 0.851, and `notag` and `poly` 1500 B at 0.650 and 0.651. These match the 80 MHz ratios.
- The Ed25519 sign step that the 2026-11-22 tree left open repeats in all three sessions. `notag` is 1.027 and `poly` 1.022 of the control, with a 0% spread over each image's three runs. `rot` is 1.012. Ed25519 verify is 0.997 to 1.000. At 80 MHz the same images are at most 1.010. None of these candidates changes Ed25519 code, so the most likely cause is code layout, but this run does not show the mechanism.

## Method

The method is the same as in [8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-2026-11-22](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-2026-11-22/NOTES.md):

- The images are the same, unchanged by sha256 (`images.sha256` is identical to that tree's), and so are the order and the run script. Only the output folder differs.
- The commits, fork PRs, harness and toolchain are the ones that tree lists.
- Each flash was `espflash write-bin --after watchdog-reset 0x0`, and every one printed `Binary successfully written to flash!`. [run.log](run.log) is the timeline, with serial device paths and the MAC removed.
- The tables come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped, a flash's value is the median over its complete rounds, and a run is one flash. `cargo verify --dir` re-checks every table.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines. Nothing was removed. No MAC, network name or key is in them.
