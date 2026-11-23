<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 upstream refresh A/B at kixel-ai/rind `8a5977ad`, CPU at 80 MHz, 2026-11-22

Recorded 2026-11-23, 04:41 to 05:43 UTC (2026-11-22, 20:41 to 21:43 America/Los_Angeles), on one ESP32-S3. This is a new A/B directory. No published tree or number is touched. The same images at 240 MHz are in [`8a5977ad...-refresh-ab-240mhz-2026-11-22`](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-240mhz-2026-11-22/NOTES.md).

## What this is

An interleaved A/B of steelhead99x/rind draft branches against upstream main, with the control flashed between every pair of candidates: `ctl zero ctl rot ctl notag ctl poly ctl all ctl xor ctl polyw ctl` (15 flashes).

- `commit` (upstream, the control): [kixel-ai/rind `8a5977adb9c32ea2ed010c400f5ffbdcdec14b47`](https://github.com/kixel-ai/rind/commit/8a5977adb9c32ea2ed010c400f5ffbdcdec14b47).

| Image | rind commit | Fork PR | What it adds to the control |
|---|---|---|---|
| `ctl` | `8a5977adb9c32ea2ed010c400f5ffbdcdec14b47` | none | upstream main as it is (HACL* Poly1305 in u64 limbs, HACL* ChaCha20; rolled SHA-512 is already upstream's default, #944) |
| `zero` | `1efdd366ada969b4b1269dccf2e693004146b2ef` | [#294](https://github.com/steelhead99x/rind/pull/294) | the key wipe as one memset on Xtensa |
| `xor` | `ec4a74283044095ef187e9d076ceba20c97102d4` | [#295](https://github.com/steelhead99x/rind/pull/295), first commit | the ChaCha20 keystream XORed a word at a time from registers |
| `rot` | `8218d57cf8100e37bffba79b6d5406d3f9b61ba0` | [#295](https://github.com/steelhead99x/rind/pull/295) head | `xor`, plus ChaCha20 rotates as `ssai`/`src` |
| `polyw` | `b0f260c0a80aa171f4b19b7faf7b6f2973a30532` | [#296](https://github.com/steelhead99x/rind/pull/296), first commit | 26-bit Poly1305 in u32 limbs, sums still in u64 |
| `notag` | `d3f3374f04cf004c5e9982007d99175313a1b4d7` plus [tools/notag-bench-only.diff](tools/notag-bench-only.diff) | none (bench only) | the branchless Poly1305 block loop, with the u64 tag add put back. Not on any branch |
| `poly` | `d3f3374f04cf004c5e9982007d99175313a1b4d7` | [#296](https://github.com/steelhead99x/rind/pull/296) at the time | the branchless block loop and the 16-bit-halves tag add |
| `all` | `24920d52cd59f5ed88ea46ae468361af61aa3474` | [#297](https://github.com/steelhead99x/rind/pull/297) at the time | `zero`, `rot` and `poly` merged |

On 2026-11-23, #294 and #295 still had these heads. #296 has moved to `8b5e1823` and #297 to `ed34f9c3`. The images measured the commits above.

## Which board

Every one of the 15 flashes printed the MAC of the board that ends **`3D:C8`** (`run.log` keeps only that suffix). The run's plan named another board (by-id `…37:68`). The log directory's name, `logs4-80-613D:C8`, is the board argument the script was given. This set is from the `3D:C8` board. The same is true of the 240 MHz set.

## Run counts and the three-run rule

- The control `ctl` has **eight runs** (flashes 1, 3, 5, 7, 9, 11, 13 and 15). `esp32s3.json`, RUNS.md and RESULTS.md are those runs. Every control row has a run spread of 0.00% except `aead_seal/1500B` (0.01%).
- Each candidate (`zero`, `rot`, `notag`, `poly`, `all`, `xor`, `polyw`) was flashed **once**, so each has **one run**. It does not meet the three-run rule. [ab.md](ab.md) shows `n` = 1 and a spread of 0 for each, which says nothing about repeatability. This directory is a diagnostic A/B. It is not a comparable-set publish of any candidate.
- To meet the rule, each of the seven candidates needs **two more flashes**, interleaved with `ctl`: 14 more candidate flashes.
- Within a flash the firmware repeats every row once per round, 6 or 7 complete rounds in a 240 s capture. Those rounds are not counted as runs.

## Board and harness

- ESP32-S3 rev v0.2 (`Chip type: esp32s3 (revision v0.2)`), 40 MHz crystal, 8 MB flash, on USB-Serial/JTAG.
- CPU at 80 MHz, the esp-hal default clock. The 240 MHz images are built with the harness's `mhz240` feature. The captures do not include the boot line that names the clock. Two things show it: these are the 80 MHz images by sha256, and rind's ChaCha20-Poly1305 and SHA-512 rows take 2.997 to 3.001 times as long as in the 240 MHz set.
- Firmware: a private copy of the student repo's `firmware-s3` WS-BENCH harness, built against each rind worktree by path. It is not published here. It times rind and RustCrypto (ed25519-dalek for Ed25519) ChaCha20-Poly1305 seal and open (16, 51, 243, 255 and 1500 B), Ed25519 sign and verify, and SHA-512 (16, 243, 1500 and 16384 B). Open decrypts a scratch copy of the sealed message.
- Known-answer checks: RFC 8439 ChaCha20-Poly1305, RFC 8032 Ed25519 and FIPS 180 SHA-512 run every round, for both libraries. The RFC 6234 million-`a` SHA-512 check runs once per boot. There were 706 checks over the 15 boots and 0 failures. No `FAILED` line appears.
- Built with esp rustc 1.97.0-nightly (`8ea53bcd7`, esp 1.97.0.0) for `xtensa-esp32s3-none-elf`, opt-level 3, fat LTO, one codegen unit. The images are merged (`espflash save-image --merge`, re-merged and checked by the harness repo's `merge-image` and `verify-merged`). They flash at 0x0. `images.sha256` lists the `.bin` and `.elf` of every image in this set.

## How it was run

Each flash was `espflash write-bin --chip esp32s3 --after watchdog-reset 0x0 s3-<image>.bin` (espflash 4.7.0). Every flash printed `Binary successfully written to flash!`. After it came a 6 s wait and a 240 s raw capture at 115200 baud. No capture needed the script's fallback reset. [run.log](run.log) is the timeline, with the serial device paths and the MAC removed.

## How the numbers are made

[tools/ws_bench_import.py](tools/ws_bench_import.py) with [tools/import.json](tools/import.json) writes `raw/run-*.json`, `esp32s3.json`, RUNS.md, RESULTS.md, `ab.json`, `ab.md` and `meta.json` from the logs in `raw/`. To reproduce, run `python3 -I tools/ws_bench_import.py tools/import.json` from this directory.

- The importer drops the rows before the first `# round complete`. That covers the partial first round, which includes the million-`a` check.
- A flash's value for a row is the median over its complete rounds of the printed `median_ns`. A run is one control flash, and the control's published time is the median of the eight.
- `cargo verify --dir` recomputes every table from `raw/run-*.json` and `ab.json`.

`raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines. Nothing was removed. No MAC, network name or key is in them.

## What to read off it

Use [ab.md](ab.md). Each candidate is one flash:

- `all` seals 16 B in 0.521 of the control's time and 1500 B in 0.467. RustCrypto stays faster than `all` at 16 and 51 B. `all` is faster at 243 B and longer: 1500 B seal is 1020.88 µs against 1857.50 µs.
- `zero` gives the most at short lengths (0.654 at 16 B seal, 0.967 at 1500 B).
- `xor` on its own is slower at short lengths (1.052 at 16 B) and faster at 1500 B (0.885). `rot` is 1.014 at 16 B and 0.851 at 1500 B.
- `notag` and `poly` are within 0.4% of each other at every length (0.650 and 0.651 of the control at 1500 B seal). `polyw` is 0.745 there.
- Ed25519 moves by at most 1.03% in any image.
- rind's SHA-512 rows are 0.9996 to 1.0000 of the control in every image.
