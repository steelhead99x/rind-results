<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-C6 ChaCha20-Poly1305 open-path placement check: pass 2, with both passes pooled, 2026-11-23

Recorded 2026-11-24, 01:49 to 02:02 UTC (2026-11-23, 17:49 to 18:02 America/Los_Angeles). This is the second pass on the same ESP32-C6, the board whose MAC ends `2C:48`. All 3 flashes printed that MAC. This is a new diagnostic directory. No published tree or number is touched.

## Run counts and the three-run rule

- This pass flashed `t275 t280 t275`. On its own it has **two control runs** and **one `t280` run**. `esp32c6.json`, RUNS.md, RESULTS.md and [ab.md](ab.md) are this pass only, so on its own it does not meet the three-run rule.
- [ab-both-passes.md](ab-both-passes.md) pools [pass 1](../5430e8b6c600941da22ba76df8334b6a5ec1ca44-open-placement-2026-11-23/NOTES.md) and this pass, reading the logs in place. Pooled, `t275` has **five runs** and `t280` **three**, which **meets the three-run rule**. `cargo verify` checks every cell of it.
- There were 575 known-answer checks over the 3 boots and 0 failures. No `FAILED` line appears.

## Result

From [ab-both-passes.md](ab-both-passes.md):

- `t280` opens **1.25 µs** slower than `t275` at every length, as in pass 1: 66.00 against 67.25 µs at 16 B, and 454.62 against 455.88 µs at 1500 B.
- Seal is unchanged, apart from 255 B, where it is 0.12 µs faster.
- Every `t280` row has a 0.00% spread over its three runs.
- The two images execute the same riscv32 instructions at different addresses, so the open-only step is placement. This pass does not show why only open moves.

## Method

The method is the same as in [pass 1](../5430e8b6c600941da22ba76df8334b6a5ec1ca44-open-placement-2026-11-23/NOTES.md):

- The images are the same, unchanged by sha256 (`images.sha256`), and so is the run script. Only the output folder differs.
- Each flash was `espflash write-bin --after watchdog-reset 0x0` (espflash 4.7.0). espflash printed `The esp32c6 does not support watchdog reset` every time. The script then ran `espflash reset` (the `reset-N` lines in [run.log](run.log)) and captured for 120 s. That second capture, which has full rows, is the published log.
- run.log has serial device paths and the MAC removed.
- The tables come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped, a flash's value is the median over its complete rounds, and a run is one flash.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial ESP-IDF bootloader line at the start. Nothing was removed.
