<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 upstream refresh A/B at kixel-ai/rind `8a5977ad`, 80 MHz: next-day repeat on the same board, 2026-11-23

Recorded 2026-11-23, 22:37 to 23:40 UTC (14:37 to 15:40 America/Los_Angeles). This is a new A/B directory. No published tree or number is touched.

## What this is

This repeats [`8a5977ad...-refresh-ab-80mhz-2026-11-22`](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-2026-11-22/NOTES.md) about 18 hours later, on the **same board**. It is the ESP32-S3 whose MAC ends `3D:C8`, and all 15 flashes printed that MAC.

The repeat used the same 15 images, unchanged by sha256 (`images.sha256` is identical), the same script, and the same order: `ctl zero ctl rot ctl notag ctl poly ctl all ctl xor ctl polyw ctl`. The commits, fork PRs, harness and toolchain are as in that tree's NOTES.md:

- `ctl`: kixel-ai/rind `8a5977adb9c32ea2ed010c400f5ffbdcdec14b47`
- `zero`: #294 `1efdd366`
- `xor`: #295 `ec4a7428`
- `rot`: #295 `8218d57c`
- `polyw`: #296 `b0f260c0`
- `notag`: `d3f3374f` plus [tools/notag-bench-only.diff](tools/notag-bench-only.diff)
- `poly`: #296 at `d3f3374f`
- `all`: #297 at `24920d52`

## Run counts and the three-run rule

- This session: `ctl` has **eight runs** (flashes 1, 3, …, 15), and each candidate has **one run**. `esp32s3.json`, RUNS.md, RESULTS.md and [ab.md](ab.md) are this session only.
- [ab-both-days.md](ab-both-days.md) pools both sessions. It reads this tree's logs and the 2026-11-22 tree's logs in place. Pooled, `ctl` has **16 runs** and each of the seven candidates has **two runs**. `cargo verify` checks it like any ab-v1 file.
- Two sessions alone are not enough for the three-run rule. A third session on the same board was run the same evening. [`…-refresh-ab-80mhz-pass3-2026-11-23/ab-three-sessions.md`](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-pass3-2026-11-23/ab-three-sessions.md) pools all three, and gives every candidate three runs.

## Same-board repeatability

From [ab-both-days.json](ab-both-days.json), comparing each session's median for every row, every library and every image:

- The two sessions agree to within **0.044%**. The worst row is RustCrypto 51 B open in `ctl`, 142.19 against 142.25 µs.
- For rind's ChaCha20-Poly1305 rows the worst is 0.033% (`poly` 255 B open, 376.50 against 376.62 µs).
- The control matches the first session: 16 B seal 205.88 µs in both, 1500 B seal 2184.25 against 2184.22 µs, and RustCrypto 1500 B seal 1857.50 µs in both.
- Within this session, every control row has a run spread of 0.0% over the eight control runs (RUNS.md).

This shows that the board and the method repeat from one day to the next. It does not show that another board would give the same numbers.

## Method

The method is the same as in the 2026-11-22 tree:

- Each flash was `espflash write-bin --chip esp32s3 --after watchdog-reset 0x0` (espflash 4.7.0), and every one printed `Binary successfully written to flash!`.
- A 240 s capture at 115200 baud followed, holding 6 or 7 complete rounds.
- There were 706 known-answer checks over 15 boots and 0 failures.
- The tables come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped, a flash's value is the median over its complete rounds, and a run is one flash.
- [run.log](run.log) is the timeline, with serial device paths and the MAC removed.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines. Nothing was removed.
