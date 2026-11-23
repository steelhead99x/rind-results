<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 SHA-512 placement A/B for fork #94, CPU at 80 MHz, 2026-11-21

Recorded 2026-11-22, 07:31 to 07:46 UTC (2026-11-21, 23:31 to 23:46 America/Los_Angeles), on one ESP32-S3. This is a new diagnostic directory. No published tree or number is touched. It is not the 2026-11-17 SHA-512 run in [`results/diag/esp32s3-pr94-sha512-iram-2026-11-17`](../../diag/esp32s3-pr94-sha512-iram-2026-11-17/README.md), which used another harness and cycle counts.

## What this is

Three SHA-512 builds from fork [#94](https://github.com/steelhead99x/rind/pull/94), flashed in the order `control rolled control unrolled control` (5 flashes).

- `commit`: upstream [kixel-ai/rind `529ff5c0cf6bb4fdb51659699c921cca8e5a824a`](https://github.com/kixel-ai/rind/commit/529ff5c0cf6bb4fdb51659699c921cca8e5a824a), the merge base of both fork commits with upstream main. `fork_commit` is the control.

| Image | rind commit (steelhead99x/rind) | What it is |
|---|---|---|
| `control` | `c1c5c8247e69eb5b9777d81be1f76bd0ec621ee5` | #94 before the change: the old default, unrolled SHA-512 running from flash |
| `rolled` | `7cbb9a70ebb84c1f4d68981a0493b17b1e9b583c` | #94 at `7cbb9a70`: the rolled loop in flash, the new default |
| `unrolled` | `7cbb9a70ebb84c1f4d68981a0493b17b1e9b583c` plus [tools/iram-bench-only-rind.diff](tools/iram-bench-only-rind.diff) | `esp32s3-sha512-unrolled`, with a bench-only patch that drops `target_os = "espidf"` from the two `link_section` cfgs so the block lands in IRAM. Not in the PR |

#94's head is now `fc4b8cd7`.

## Run counts and the three-run rule

- `control` has **three runs** (flashes 1, 3 and 5). `esp32s3.json`, RUNS.md and RESULTS.md are those runs.
- `rolled` and `unrolled` were flashed **once** each. They do not meet the three-run rule. To meet it, each needs **two more flashes**, interleaved with `control`.
- Each 170 s capture holds one complete round after the first. The first round runs the RFC 6234 million-`a` check and is dropped. So each flash's value is one or two rounds of the firmware's medians (the second, partial round adds the rows it reached).

## Board, harness and method

- ESP32-S3 rev v0.2 (`Chip type: esp32s3 (revision v0.2)`), 8 MB flash, on USB-Serial/JTAG. It is the board whose MAC ends **`37:68`**. `run.log` keeps only that suffix, and drops the serial device paths.
- CPU at 80 MHz, the esp-hal default clock. The captures do not include the boot line that names the clock.
- Firmware: a private copy of the student repo's `firmware-s3` WS-BENCH harness, with SHA-512 rows and checks added. It is not published here. It times rind and RustCrypto (ed25519-dalek for Ed25519) ChaCha20-Poly1305 seal and open, Ed25519, and SHA-512 (16, 243, 1500 and 16384 B). Known-answer checks run every round (RFC 8439, RFC 8032 and FIPS 180, both libraries; the million-`a` check once per boot): 69 checks over the 5 boots analysed, 0 failures, and no `FAILED` line.
- Built with esp rustc 1.97.0-nightly (esp 1.97.0.0) for `xtensa-esp32s3-none-elf`, opt-level 3, fat LTO. `images.sha256` lists the ELFs. They are not committed.
- Each ELF was flashed with `espflash flash --chip esp32s3 --non-interactive <elf>` (espflash 4.7.0). Every flash ended `Flashing has completed!`. A 6 s wait and a 170 s raw capture at 115200 baud followed.
- The numbers come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json), the same importer as the other ESP32 A/B trees. Rows before the first `# round complete` are dropped. A flash's value is the median over its complete rounds, and a run is one control flash. `cargo verify --dir` recomputes `esp32s3.json`, RESULTS.md, RUNS.md and every `ab.md` cell.
- `raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial ESP-IDF bootloader line at the start. Nothing was removed. No MAC, network name or key is in them.

## What to read off it

Use [ab.md](ab.md). `rolled` and `unrolled` are one flash each:

- `rolled` takes 1.087 to 1.090 of `control`'s time on SHA-512. `unrolled` (IRAM) takes 0.9996 to 1.0001 of it.
- In this harness and build, the old unrolled-in-flash control is not the 11 to 13 times slower row that the build note expected. It is level with the IRAM build.
- The ChaCha20-Poly1305 rows are level (within 0.06%) in all three images. The Ed25519 rows are not: `rolled` signs in 0.825 of `control`'s time. `control` and the other two images are different commits, so the Ed25519 difference is not a SHA-512 result.
