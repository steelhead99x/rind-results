<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 board runs for rind#94, #87 and #85, 2026-11-18

Raw serial logs and cycle counts from one ESP32-S3 board for three steelhead99x/rind pull requests: [#94](https://github.com/steelhead99x/rind/pull/94) (SHA-512 compressor in IRAM), [#87](https://github.com/steelhead99x/rind/pull/87) (Poly1305 in 26-bit limbs on 32-bit targets) and [#85](https://github.com/steelhead99x/rind/pull/85) (Xtensa lint). This is the data behind the board sections of those PR bodies. It is a new diagnostic directory. No published tree or number is touched, and it is separate from the 2026-11-17 SHA-512 rerun in `esp32s3-pr94-sha512-iram-2026-11-17`.

## What was measured

| | |
|---|---|
| Board | ESP32-S3 rev v0.2 (`chip revision: v0.2` in every log), 8 MB flash, USB-Serial/JTAG on a Linux x86_64 host |
| Clock | CPU 240 MHz (`cpu_hz` 240000000 in all 50 start records). The `clock` record counted 240,000,234 to 240,001,069 CCOUNT cycles per second of the 40 MHz timer. Flash DIO at 80 MHz (`Boot SPI Speed : 80MHz` in all 50 logs) |
| Run window | 2026-11-18 23:34 to 2026-11-19 00:25 PST, one board, nothing else on the serial port (`run.log` is the flash and capture timeline) |
| Firmware | rind-hardware `esp32` bench harness at `7a14b38` plus [tools/harness-brief-a.patch](tools/harness-brief-a.patch), ESP-IDF v5.5.1 (`idf` in every start record), esp rustc 1.97.0-nightly (8ea53bcd7 2026-07-08, 1.97.0.0), release profile, `-Zbuild-std`, espflash 4.7.0 `--flash-freq 80mhz` |
| Bootloader | The `ESP-IDF v6.1` second stage bootloader line in each log is the bootloader espflash 4.7.0 writes with the app. The application is built with ESP-IDF v5.5.1 |
| Method | Interleaved A/B. Each round flashes every variant of a bench once, in a rotating order, resets the board by RTS, and logs the boot until `done`. 5 rounds per bench. Cycle counts are CCOUNT medians per row from the harness. The tables take the median of the 5 rounds |
| Known-answer checks | `checks-done` with `failed: 0` (and `poly-kat-done` with `failed: 0` in the Poly1305 images) in all 50 boots. Over a variant's 5 boots, 165 checks pass in each sha2 and poly1305 image and 20 in each hmac image |
| Image check | Each variant's image has one set of `esp_image` segment sizes in all of its logs (`tools/analyze.py` prints this), and the sizes differ between variants of a bench, except that `pr94h-*` match `pr94-*` (same sizes, different ELF hashes) |

Each `run-N.log` is the whole serial output of one boot, one line per serial line, with the seconds since the capture opened. `RB:` lines are the harness's records, and `run-N.jsonl` is the same boot's records, one JSON object per line. `tools/analyze.py <bench> <variants> <base>` makes each `ratios-*.tsv` from these directories, run from this directory, and `tools/gen.py` makes `tables.md`.

## Variants

All commits are in [steelhead99x/rind](https://github.com/steelhead99x/rind). "Bench patches" are rind-hardware's `patches/0001` (MD5 on Xtensa) and the edited `0002` (aws-lc-rs rows left out on Xtensa) in this tree's patch, plus `dropvg` (removes the verified-garbage rows, which do not build for Xtensa). They do not apply to the #85, #87 and `3654a771` heads, which carry their own equivalents. See [build-status.txt](build-status.txt) and [build-status-pr94-head-3654a771.txt](build-status-pr94-head-3654a771.txt).

| Directory (`<variant>-<sha>`) | rind commit | Benches | Bench patches | Cargo features | ELF sha256 (first 12) |
|---|---|---|---|---|---|
| `main` | fork `main` `b9958b33a385d2aac9b4aecc8a42e974d7185112` | sha2, hmac | 0001, 0002, dropvg | none | sha2 4bb98e31d89b, hmac aa9ccb193603 |
| `pr94-iram` | #94 `e874533ad4de2cdc4310db1c1d3f66bbd8857010` | sha2, hmac | 0001, 0002, dropvg | none (IRAM is the PR's default) | sha2 18fab8eb6a9e, hmac 8b4a7d63863f |
| `pr94-flash` | #94 `e874533a` | sha2, hmac | 0001, 0002, dropvg | `esp32s3-sha512-flash` | sha2 95a6bfe5d4af, hmac dfca9049192e |
| `pr94h-iram` | #94 `3654a771223af07fa8350f1e5e0f49dac3c044d4` (head after it merged fork main) | sha2 | dropvg only | none | sha2 553482cd7f88 |
| `pr94h-flash` | #94 `3654a771` | sha2 | dropvg only | `esp32s3-sha512-flash` | sha2 3c54419a5dc5 |
| `pr85` | #85 `0fa33a9271a180b88a653ae1c8195473fe4c3b21` | poly1305 | none | none | poly1305 510788d73894 |
| `pr87` | #87 `68277ff7cc45b933b087e18c630add6eed280091` (the head `97d69f54` differs from it only in `.cargo/mutants.toml`) | poly1305 | none | none | poly1305 93f3e5d75a72 |

The ELFs are not committed (about 235 MB). `images.sha256` has the sha256 of all of them.

The Poly1305 images carry a bench-only shim from [poly-expose.patch.sh](tools/harness-brief-a.patch), appended to the rind working tree and not part of any PR, because rind's Poly1305 is `pub(crate)`. It calls the portable `Poly1305` directly. On the #85 base that is the HACL* code, on #87 it is the 26-bit code. The `poly1305` bench ([tools/polybench.rs](tools/polybench.rs)) times it and the portable ChaCha20-Poly1305 seal over a pattern message with a 32 B AAD. Each `pb` record is the median of 31 samples of 200 (64 B), 20 (1200 B) or 2 (16 KB) Poly1305 calls, or of 50, 50, 5 or 1 seal calls (32 B, 64 B, 1200 B, 16 KB). [tools/kat.rs](tools/kat.rs) holds the SHA-512, SHA-384, SHA-512/256 and HMAC-SHA384 checks added for #94 and the tag differential used by #87.

## #94: SHA-512 in IRAM against main (e874533a)

Cycles, median of 5 rounds. A ratio below 1 is faster than main. Full rows, for every library in the image, are in [ratios-pr94-sha2.tsv](ratios-pr94-sha2.tsv) and [ratios-pr94-hmac.tsv](ratios-pr94-hmac.tsv).

| Row (rind) | main cycles | IRAM (this PR) | IRAM / main | `esp32s3-sha512-flash` | flash / main |
|---|---:|---:|---:|---:|---:|
| SHA-512 32B | 26,747 | 24,544 | 0.918 | 345,895 | 12.932 |
| SHA-512 64B | 26,857 | 24,555 | 0.914 | 346,318 | 12.895 |
| SHA-512 1200B | 252,324 | 229,872 | 0.911 | 2,922,769 | 11.583 |
| SHA-512 1350B | 277,409 | 252,786 | 0.911 | 3,205,984 | 11.557 |
| SHA-512 8KB | 1,631,380 | 1,486,589 | 0.911 | 18,517,066 | 11.351 |
| SHA-512 16KB | 3,234,638 | 2,947,474 | 0.911 | 36,670,525 | 11.337 |
| SHA-512/256 16KB | 3,234,667 | 2,947,938 | 0.911 | 36,674,100 | 11.338 |
| HMAC-SHA384 64B | 107,680 | 98,752 | 0.917 | 1,264,248 | 11.741 |
| HMAC-SHA384 1200B | 333,415 | 304,297 | 0.913 | 3,811,930 | 11.433 |
| HMAC-SHA384 16KB | 3,315,560 | 3,021,746 | 0.911 | 37,597,243 | 11.340 |
| HMAC-SHA384 reused key 16KB | 3,260,391 | 2,970,963 | 0.911 | 37,005,776 | 11.350 |

- rind's SHA-512, SHA-512/256 and HMAC-SHA384 rows are 0.911 to 0.918 of main with the IRAM placement. The largest spread of any rind row across the 5 rounds is 0.051%.
- With `esp32s3-sha512-flash` the same unrolled block runs from flash and is 11.3 to 13.1 times slower than main's rolled loop. The cause was not isolated with a counter.
- Rows of other libraries move with code layout: in the sha2 image libcrux SHA-256 is 0.525 to 0.700 of main with IRAM and 0.309 to 0.390 with the opt-out, and RustCrypto SHA-256 is 1.012 to 1.329 and 1.024 to 1.655. rind's one-shot SHA-256 rows are 0.999 to 1.000 of main, and its SHA-256 update rows are 0.982 (1 B) and 0.997 (16 B).
- Head `3654a771` (only fork main merged in): [ratios-pr94-sha2-head-3654a771-iram.tsv](ratios-pr94-sha2-head-3654a771-iram.tsv) and [ratios-pr94-sha2-head-3654a771-flash.tsv](ratios-pr94-sha2-head-3654a771-flash.tsv) compare it with `e874533a` row for row. SHA-512 16 KB is 2,947,474 against 2,947,466 cycles (IRAM) and 36,670,525 against 36,669,991 (opt-out).

## #87: Poly1305 26-bit limbs against the #85 base

Cycles, median of 5 rounds, from [ratios-pr87-poly1305.tsv](ratios-pr87-poly1305.tsv). The largest spread of any row is 0.043%.

| Row | #85 base (HACL*) cycles | this PR cycles | ratio |
|---|---:|---:|---:|
| Poly1305 64 B | 5,786 | 2,745 | 0.474 |
| Poly1305 1200 B | 73,832 | 41,384 | 0.561 |
| Poly1305 16 KB | 982,929 | 557,411 | 0.567 |
| ChaCha20-Poly1305 seal 32 B | 16,575 | 8,986 | 0.542 |
| ChaCha20-Poly1305 seal 64 B | 18,697 | 10,272 | 0.549 |
| ChaCha20-Poly1305 seal 1200 B | 130,082 | 92,368 | 0.710 |
| ChaCha20-Poly1305 seal 16 KB | 1,613,115 | 1,183,661 | 0.734 |

## #85: Xtensa lint

[lint-85-esp-1.98.1.0.log](lint-85-esp-1.98.1.0.log) is `sh/bare-metal-test.sh` (`TEST_EXIT 0`) and `ESP_TOOLCHAIN=esp-1.98.1 sh/bare-metal.sh xtensa` (`EXIT 0`) at `0fa33a92` with Espressif Rust 1.98.1.0 (rustc 183f762d6, clippy 0.1.98). The `error:` lines in it are the negative cases that `bare-metal-test.sh` expects. The `pr85` board runs above are the same head: it builds, boots and passes all 165 checks. No timing is claimed for #85.

## Not here

- The ELFs and the espflash output (`flash-*.log`).
- Any HKDF run. The 2026-11-17 HKDF rows were not rerun.

## Layout

```text
<variant>-<sha>/<bench>/run-{1..5}.{log,jsonl}   raw serial output and its RB: records, one file per boot
run.log                                    flash and capture timeline
images.sha256                              sha256 of the 10 ELFs that were flashed
build-status*.txt                          tool versions, patch and image status
ratios-*.tsv  tables.md                    from tools/analyze.py and tools/gen.py
lint-85-esp-1.98.1.0.log                   #85 lint
tools/                                     build.sh run.sh capture.py analyze.py gen.py, harness patch, kat.rs, polybench.rs, dropvg patches
```
