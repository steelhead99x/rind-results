<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 ChaCha20-Poly1305 stack A/B (#275, #278, #279, #280), CPU at 80 MHz, 2026-11-21

Recorded 2026-11-22, 04:54 to 05:07 UTC (2026-11-21, 20:54 to 21:07 America/Los_Angeles), on one ESP32-S3. This is a new diagnostic directory. No published tree or number is touched.

## What this is

The fork's stacked ESP32-S3 ChaCha20-Poly1305 branches, each flashed once between control flashes, in the order `ctl zero ctl chacha ctl all ctl all240 ctl` (9 flashes).

- `commit`: upstream [kixel-ai/rind `5430e8b6c600941da22ba76df8334b6a5ec1ca44`](https://github.com/kixel-ai/rind/commit/5430e8b6c600941da22ba76df8334b6a5ec1ca44), the merge base of every image's commit with upstream main. `fork_commit` is the control.

| Image | rind commit (steelhead99x/rind) | Fork PR | What it is |
|---|---|---|---|
| `ctl` | `05498de75934e3e44778b18d0485188383f76c95` | [#275](https://github.com/steelhead99x/rind/pull/275) | #275 then: 26-bit Poly1305 and word-XOR ChaCha20 on 32-bit targets |
| `zero` | `b3a799f30fd6c16af59b911523b263da1e2e2700` | [#278](https://github.com/steelhead99x/rind/pull/278) | #278 then: the key wipe as one memset, stacked on #275 |
| `chacha` | `42a18ef6b5665e37174abbecf5fd35fb33f25497` | [#279](https://github.com/steelhead99x/rind/pull/279) | #279 then: ChaCha20 rotates as `ssai`/`src` on Xtensa, stacked on #278 |
| `all` | `2d7fac0e5baabc46e65c4c6710f35399dac664c4` | [#280](https://github.com/steelhead99x/rind/pull/280) | #280 then: branchless Poly1305 block loop and 16-bit tag add, stacked on #279 |

The PR heads have all moved since (#275 `ad3b2fa4`, #278 `459deafe`, #279 `f01d9b52`, #280 `29594dea` on 2026-11-23). These images are the commits above. They are older than the upstream-refresh branches (#294 to #297) in [`8a5977ad...-refresh-ab-80mhz-2026-11-22`](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-80mhz-2026-11-22/NOTES.md). A first attempt with older images, earlier the same evening, failed its known-answer checks in `chacha` and `all`, and is not published.

## Run counts and the three-run rule

- `ctl` has **five runs** (flashes 1, 3, 5, 7 and 9). `esp32s3.json`, RUNS.md and RESULTS.md are those runs.
- `zero`, `chacha` and `all` were flashed **once** each. They do not meet the three-run rule. [ab.md](ab.md) shows `n` = 1. To meet the rule, each needs **two more flashes**, interleaved with `ctl`. These commits are superseded, so a rerun belongs on the refresh branches instead.
- Flash 8 was `all` built at 240 MHz (`s3-all240-2d7fac0e5.elf`). No 240 MHz control was flashed in this run, so it has nothing to be compared against. Its log is kept as `raw/b-8-all240.log`, and it is left out of every table here.

## Board, harness and method

- ESP32-S3 rev v0.2 (`Chip type: esp32s3 (revision v0.2)`), 8 MB flash, on USB-Serial/JTAG. It is the board whose MAC ends **`37:68`**. `run.log` keeps only that suffix, and drops the serial device paths.
- CPU at 80 MHz, the esp-hal default clock. The captures do not include the boot line that names the clock.
- Firmware: a private copy of the student repo's `firmware-s3` WS-BENCH harness (with an open-in-scratch fix). It is not published here. It times rind and RustCrypto (ed25519-dalek for Ed25519) ChaCha20-Poly1305 seal and open (16, 51, 243, 255 and 1500 B) and Ed25519 sign and verify. Known-answer checks run every round (RFC 8439 and RFC 8032, both libraries): 406 checks over the 8 boots analysed, 0 failures, and no `FAILED` line.
- Built with esp rustc 1.97.0-nightly (esp 1.97.0.0) for `xtensa-esp32s3-none-elf`, opt-level 3, fat LTO. `images.sha256` lists the ELFs. They are not committed.
- Each ELF was flashed with `espflash flash --chip esp32s3 --non-interactive <elf>` (espflash 4.7.0). Every flash ended `Flashing has completed!`. A 6 s wait and a 75 s raw capture at 115200 baud followed.
- The numbers come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json), the same importer as the other ESP32 A/B trees. Rows before the first `# round complete` are dropped. A flash's value is the median over its complete rounds, and a run is one control flash. `cargo verify --dir` recomputes `esp32s3.json`, RESULTS.md, RUNS.md and every `ab.md` cell.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial ESP-IDF bootloader line at the start. Nothing was removed. No MAC, network name or key is in them.

## What to read off it

Use [ab.md](ab.md). Each candidate is one flash:

- `all` seals 1500 B in 0.740 of `ctl`'s time and 16 B in 0.535. At 1500 B that is 1020.88 µs, and `ctl` is already faster than RustCrypto there (0.74).
- The stack adds up step by step at 16 B seal: `zero` 0.604, `chacha` 0.566, `all` 0.535.
- The control here is #275, not upstream main, so these ratios are not the refresh A/B's ratios.
