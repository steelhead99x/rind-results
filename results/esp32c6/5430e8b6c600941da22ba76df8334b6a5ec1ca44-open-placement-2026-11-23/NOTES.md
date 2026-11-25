<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-C6 ChaCha20-Poly1305 open-path placement check, 2026-11-23

Recorded 2026-11-23, 20:30 to 20:52 UTC, on one ESP32-C6. This is a new diagnostic directory. No published tree or number is touched.

## What this is

Two fork images that execute the same riscv32 instructions on the ChaCha20-Poly1305 seal and open paths, but at different addresses. They were flashed in the order `t275 t280 t275 t280 t275`. The question is whether the 2.5 µs open slowdown seen on 2026-11-22 ([`5430e8b6...-tag-ab-2026-11-22`](../5430e8b6c600941da22ba76df8334b6a5ec1ca44-tag-ab-2026-11-22/NOTES.md)) comes from code placement or from #280's 16-bit tag add.

- `commit`: upstream [kixel-ai/rind `5430e8b6c600941da22ba76df8334b6a5ec1ca44`](https://github.com/kixel-ai/rind/commit/5430e8b6c600941da22ba76df8334b6a5ec1ca44), the merge base of both fork commits with upstream main. `fork_commit` is the control, `t275`.

| Image | rind commit (steelhead99x/rind) | Fork PR | What it is |
|---|---|---|---|
| `t275` | `89da51f759dd837216712705676302aa98c42625` | [#275](https://github.com/steelhead99x/rind/pull/275) | #275's head when the image was built: 26-bit Poly1305 and word-XOR ChaCha20 on 32-bit targets |
| `t280` | `5af26036fd81bfc8b6af585fab8560acd2aee770` | [#280](https://github.com/steelhead99x/rind/pull/280) | #280 after `9e5f0d21`, which keeps the 16-bit tag add on Xtensa only. On riscv32 it runs `t275`'s instructions at other addresses |

Both PR heads have moved since: #275 is at `ad3b2fa4` and #280 at `29594dea` (2026-11-23). These images are the commits above.

The build note for these images, which was made without a board, says this about them. Under qemu-riscv32, `t275` and `t280` execute the same number of instructions per call at every length measured (16, 243 and 1500 B, seal and open). Every function is the same size in both images, but the linker placed them at different addresses. This directory measures the boards. It does not re-check that analysis.

## Run counts and the three-run rule

- `t275` (the control) has **three runs**: flashes 1, 3 and 5. `esp32c6.json`, RUNS.md and RESULTS.md are those runs.
- `t280` has **two runs**: flashes 2 and 4. That is one short of the three-run rule. [ab.md](ab.md) shows `n` = 2. Meeting the rule needs **one more `t280` flash** on this board, between two `t275` flashes (`t275 t280 t275`). That pass was run the same evening: [pass 2](../5430e8b6c600941da22ba76df8334b6a5ec1ca44-open-placement-pass2-2026-11-23/NOTES.md). Its `ab-both-passes.md` pools both passes, giving `t275` five runs and `t280` three.
- Every `t275` row's spread over its three runs is 0.00% except `aead_seal/51B` (0.18%) and `aead_open/243B` (0.10%). Both `t280` runs print the same median on every row.

## Result

From [ab.md](ab.md): `t280` opens 1.25 µs slower than `t275` at every length. That is 66.00 to 67.25 µs at 16 B, and 454.62 to 455.88 µs at 1500 B. Seal is unchanged: 0.00 µs at every length except 255 B, which is 0.12 µs faster. The instructions are the same, so the open-only step is placement. It is about half the 2.50 µs step that `#280` at `1d8901b8` showed on 2026-11-22. The RustCrypto rows are the same code in both images. Their ChaCha20-Poly1305 rows are identical, except 16 B seal, which is 0.12 µs faster in `t280`. Ed25519 sign differs by 0.001%.

This run does not show why only open moves.

## Board, harness and method

The board and harness are the same as in the 2026-11-22 tag A/B and the [2026-11-23 refresh A/B](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-2026-11-23/NOTES.md):

- ESP32-C6 rev v0.2, 4 MB flash. It is the board whose MAC ends `2C:48`.
- CPU at 160 MHz.
- A private WS-BENCH harness (the older template, without the AES-GCM and P-256 rows). It times ChaCha20-Poly1305 seal and open (16, 51, 243, 255 and 1500 B) and Ed25519 for rind and RustCrypto/dalek.
- RFC 8439 and RFC 8032 known-answer checks run every round. There were 959 checks over the 5 boots and 0 failures.
- Built with nixpkgs rustc 1.98.1 for `riscv32imac-unknown-none-elf`, opt-level 3, fat LTO, one codegen unit.
- The images are merged and flash at 0x0. Their sha256 are in `images.sha256`.

The images were flashed with `espflash write-bin --after watchdog-reset 0x0` (espflash 4.7.0). For every flash, espflash printed `The esp32c6 does not support watchdog reset`. The script then ran `espflash reset` and captured for 120 s. [run.log](run.log) is the timeline, with serial device paths and the MAC removed.

The numbers come from [tools/ws_bench_import.py](tools/ws_bench_import.py) and [tools/import.json](tools/import.json), the same importer as the refresh A/B. Rows before the first `# round complete` are dropped. A flash's value is the median over its 47 complete rounds, and a run is one flash. `cargo verify --dir` re-checks every table against `raw/run-*.json` and `ab.json`.

`raw/b-<flash>-<image>.log` are the captures, byte for byte. They hold only `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial bootloader line at the start. Nothing was removed.
