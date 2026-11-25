<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-C6 ChaCha20-Poly1305 tag A/B, #275 against #280 at `1d8901b8`, 2026-11-22

Recorded 2026-11-22, 09:26 to 09:34 UTC (01:26 to 01:34 America/Los_Angeles), on one ESP32-C6. This is a new diagnostic directory. No published tree or number is touched.

## What this is

Fork #275's head against #280 at `1d8901b8`, flashed in the order `ctl all ctl all ctl`.

- `commit`: upstream [kixel-ai/rind `5430e8b6c600941da22ba76df8334b6a5ec1ca44`](https://github.com/kixel-ai/rind/commit/5430e8b6c600941da22ba76df8334b6a5ec1ca44), the merge base of both fork commits with upstream main. `fork_commit` is the control.

| Image | rind commit (steelhead99x/rind) | Fork PR | What it is |
|---|---|---|---|
| `ctl` | `89da51f759dd837216712705676302aa98c42625` | [#275](https://github.com/steelhead99x/rind/pull/275) | #275's head then: 26-bit Poly1305 and word-XOR ChaCha20 on 32-bit targets |
| `all` | `1d8901b8f23d5c769dc32629c4b4f044b1e80f7e` | [#280](https://github.com/steelhead99x/rind/pull/280) | #280 then (branchless Poly1305 block loop and 16-bit tag add, merged with the Xtensa ChaCha20 rotates branch) |

Both PR heads have moved since: #275 to `ad3b2fa4`, and #280 to `29594dea` by way of `9e5f0d21` and `5af26036`.

The build note made without a board compared the symbols of the two riscv32 images with llvm-nm. Only `Poly1305::finish` (the tag add) differs in size, 0x366 against 0x3ba bytes. That note is not re-checked here.

## Run counts and the three-run rule

- `ctl` has **three runs** (flashes 1, 3 and 5). `esp32c6.json`, RUNS.md and RESULTS.md are those runs.
- `all` has **two runs** (flashes 2 and 4). That is one short of the three-run rule. Meeting the rule needs **one more `all` flash** (`c6-all-1d8901b8f.elf`) on this board, between two `ctl` flashes. #280 has moved on since, so the [2026-11-23 placement check](../5430e8b6c600941da22ba76df8334b6a5ec1ca44-open-placement-2026-11-23/NOTES.md) is the follow-up that was actually run.

## Result

From [ab.md](ab.md): `all` opens 2.50 µs slower than `ctl` at every length except 51 B, where it is 2.38 µs slower: 66.00 against 68.50 µs at 16 B and 454.62 against 457.12 µs at 1500 B. It seals 0.12 to 0.25 µs slower. The RustCrypto rows are the same code in both images. Their ChaCha20-Poly1305 rows are identical except 16 B seal (0.06 µs).

On 2026-11-23 the same board ran a separately built image of `89da51f7` (`c6-t275.bin` in the placement check). It printed the same rind seal and open medians as `ctl` here, at every length. Its Ed25519 rows differ by up to 0.12%.

## Board, harness and method

- ESP32-C6 rev v0.2, 4 MB flash. It is the board whose MAC ends `2C:48`.
- CPU at 160 MHz.
- A private WS-BENCH harness (the older template). It times ChaCha20-Poly1305 seal and open (16, 51, 243, 255 and 1500 B) and Ed25519 for rind and RustCrypto/dalek.
- RFC 8439 and RFC 8032 known-answer checks run every round. There were 640 checks over the 5 boots and 0 failures.
- Built with nixpkgs rustc 1.98.1 for `riscv32imac-unknown-none-elf`, opt-level 3, fat LTO, one codegen unit.
- The ELFs were flashed with `espflash flash --chip esp32c6 --non-interactive <elf>` (espflash 4.7.0). Every flash ended `Flashing has completed!`. Each capture ran 80 s, after a 6 s wait. Their sha256 are in `images.sha256`.
- [run.log](run.log) is the timeline, with serial device paths and the MAC removed.

The numbers come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json). Rows before the first `# round complete` are dropped. A flash's value is the median over its complete rounds, and a run is one flash. `cargo verify --dir` re-checks every table.

`raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial bootloader line. Nothing was removed.
