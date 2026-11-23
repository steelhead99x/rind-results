<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 upstream refresh A/B at kixel-ai/rind `8a5977ad`, CPU at 240 MHz, 2026-11-22

Recorded 2026-11-23, 05:43 to 06:12 UTC (2026-11-22, 21:43 to 22:12 America/Los_Angeles), on one ESP32-S3. It ran straight after the [80 MHz set](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-2026-11-22/NOTES.md), on the same board, with the same commits built with the harness's `mhz240` feature. This is a new A/B directory. No published tree or number is touched.

## What this is

The flash order was `ctl zero ctl rot ctl notag ctl poly ctl all ctl` (11 flashes). The images are `s3-<image>240.bin`. In this tree the `240` suffix is dropped from the image names (`ctl` is `s3-ctl240.bin`).

- `commit` (upstream, the control): [kixel-ai/rind `8a5977adb9c32ea2ed010c400f5ffbdcdec14b47`](https://github.com/kixel-ai/rind/commit/8a5977adb9c32ea2ed010c400f5ffbdcdec14b47).
- The candidates are the same commits as in the 80 MHz set: `zero` [#294](https://github.com/steelhead99x/rind/pull/294) `1efdd366`, `rot` [#295](https://github.com/steelhead99x/rind/pull/295) `8218d57c`, `notag` `d3f3374f` plus the bench-only [tools/notag-bench-only.diff](tools/notag-bench-only.diff), `poly` [#296](https://github.com/steelhead99x/rind/pull/296) at `d3f3374f`, and `all` [#297](https://github.com/steelhead99x/rind/pull/297) at `24920d52`. The full shas are in `meta.json`, and the 80 MHz NOTES.md says what each one changes. There is no `xor` or `polyw` image at 240 MHz. #296 has since moved to `8b5e1823` and #297 to `ed34f9c3`.

## Which board

Every one of the 11 flashes printed the MAC of the board that ends **`3D:C8`**. The run's plan named another board (`…37:68`). The directory name `logs4-240-613D:C8` is the board argument the script was given.

## Run counts and the three-run rule

- The control `ctl` has **six runs** (flashes 1, 3, 5, 7, 9 and 11). `esp32s3.json`, RUNS.md and RESULTS.md are those runs. Every control row has a run spread of 0.0% except `aead_open/255B` (0.1% in RUNS.md, 0.07% in ab.md).
- Each candidate was flashed **once**, so each has **one run**. It does not meet the three-run rule. This directory is a diagnostic A/B.
- To meet the rule, each of `zero`, `rot`, `notag`, `poly` and `all` needs **two more flashes** at 240 MHz, interleaved with `ctl240`: 10 more candidate flashes.
- Each 150 s capture holds 4 or 5 complete rounds. Those rounds are not counted as runs.

## Board, harness and method

These are as in the 80 MHz set, except for the clock and the capture length.

- ESP32-S3 rev v0.2, 8 MB flash.
- CPU at 240 MHz (`CpuClock::max`). The captures do not include the boot line that names the clock. Two things show it: these are the `…240` images by sha256, and rind's ChaCha20-Poly1305 and SHA-512 rows take 1/3.00 of the 80 MHz times (ratio 2.997 to 3.001).
- The firmware is the private `firmware-s3` WS-BENCH harness. It times ChaCha20-Poly1305 seal and open, Ed25519 and SHA-512, for rind and RustCrypto/dalek.
- There were 396 known-answer checks over the 11 boots and 0 failures.
- Built with esp rustc 1.97.0-nightly (esp 1.97.0.0) for `xtensa-esp32s3-none-elf`, opt-level 3, fat LTO, one codegen unit.
- Each flash was `espflash write-bin --chip esp32s3 --after watchdog-reset 0x0` (espflash 4.7.0), and every one printed `Binary successfully written to flash!`. A 150 s capture followed. [run.log](run.log) is the timeline, with serial device paths and the MAC removed.
- The numbers come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped. A flash's value is the median over its complete rounds, and a run is one control flash. `cargo verify --dir` recomputes every table.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines. Nothing was removed.

## What to read off it

Use [ab.md](ab.md). Each candidate is one flash:

- The ChaCha20-Poly1305 ratios are the same as at 80 MHz to within 0.002. `all` seals 16 B in 0.521 of the control's time and 1500 B in 0.467. `zero` is 0.654 at 16 B, `rot` 0.851 at 1500 B, and `notag` and `poly` 0.650 and 0.651 at 1500 B.
- RustCrypto stays faster than `all` at 16 and 51 B. `all` is faster at 243 B and longer: 1500 B seal is 340.38 µs against 619.25 µs.
- Ed25519 sign is up to 2.7% slower in `notag` (1.027) and `poly` (1.022), against at most 1.03% at 80 MHz. These are one flash each, and none of the candidates targets Ed25519. A repeat would show whether the step is real.
- rind's SHA-512 rows are 0.9996 to 1.0000 of the control in every image.
