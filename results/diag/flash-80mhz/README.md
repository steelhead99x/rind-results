<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: the flash clock was 40 MHz, not 80

The build asks for DIO flash at 80 MHz (`CONFIG_ESPTOOLPY_FLASHFREQ_80M`), but espflash writes the
flash clock into the image header when it flashes and does not read `sdkconfig`; its default is
40 MHz. The bootloader sets the clock from the header (`Boot SPI Speed : 40MHz` in every published
run log) and the app keeps it. Every published ESP32-S3 run before this one ran so.

Test: the published `sha2.elf` (`c1534994…`, unchanged) flashed with `--flash-freq 80mhz`
(`Boot SPI Speed : 80MHz`), one run, 2026-11-12 10:27 PST.

- Every row that streams code from flash runs in 0.520x the cycles: `sha512/*` for rind, libcrux
  and RustCrypto (1.54 M to 0.80 M cycles a block), and the short SHA-256 rows of rind and
  RustCrypto. ring (small loops, cached) and the hardware rows do not move.
- So the published ESP32-S3 numbers for cache-missing code are up to 1.92x too slow, against the
  configuration the README describes, and rows that miss and rows that do not were compared at a
  bias.

Fixed: `tools/run-all.sh` flashes with `--flash-freq 80mhz` (`FLASH_FREQ` to change it), the
report prints the clock from the run logs, `tools/verify.py` fails a set that mixes clocks, and
the ESP32-S3 set was rerun at 80 MHz (`../../RESULTS.md`). This directory keeps the test.

Made with `tools/compare.py raw diag/flash-80mhz sha2`:


## sha2

Known-answer checks: 40MHz 39/39, 80MHz 13/13. Runs: 3 and 1. Rows: 74 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 8 | 0.753 | 0.520 to 0.997 |
| libcrux | short | 4 | 0.738 | 0.520 to 0.972 |
| rind | long | 12 | 0.520 | 0.520 to 0.961 |
| rind | short | 8 | 0.532 | 0.520 to 0.762 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 0.520 | 0.520 to 0.982 |
| rustcrypto | short | 6 | 0.520 | 0.520 to 0.597 |

47 of 74 rows moved by more than max(1%, run spread):

| Row | 40MHz cycles | 80MHz cycles | B/A |
|---|---:|---:|---:|
| `sha512/libcrux/64B` | 1,530,282 | 795,123 | 0.520 |
| `sha512/libcrux/32B` | 1,530,279 | 795,144 | 0.520 |
| `sha512-256/rustcrypto/32B` | 1,548,525 | 804,762 | 0.520 |
| `sha512/rustcrypto/64B` | 1,558,450 | 809,941 | 0.520 |
| `sha512/rustcrypto/32B` | 1,558,444 | 809,938 | 0.520 |
| `sha512-256/rustcrypto/64B` | 1,547,565 | 804,285 | 0.520 |
| `sha512-256/rind/32B` | 1,551,383 | 806,408 | 0.520 |
| `sha512/rind/32B` | 1,542,727 | 801,913 | 0.520 |
| `sha512/rind/64B` | 1,542,727 | 801,916 | 0.520 |
| `sha512-256/rind/64B` | 1,550,414 | 805,940 | 0.520 |
| `sha512/libcrux/1350B` | 16,158,954 | 8,407,114 | 0.520 |
| `sha512/libcrux/8KB` | 95,143,419 | 49,501,903 | 0.520 |
| `sha512/rustcrypto/8KB` | 98,331,031 | 51,160,762 | 0.520 |
| `sha512/libcrux/1200B` | 14,695,464 | 7,645,938 | 0.520 |
| `sha512/rind/8KB` | 95,174,902 | 49,518,781 | 0.520 |
| `sha512-256/rind/1200B` | 14,731,784 | 7,664,846 | 0.520 |
| `sha512/rustcrypto/1350B` | 16,689,838 | 8,683,615 | 0.520 |
| `sha512/rind/1200B` | 14,726,308 | 7,662,007 | 0.520 |
| `sha512/libcrux/16KB` | 188,749,893 | 98,205,411 | 0.520 |
| `sha512-256/rind/8KB` | 95,171,100 | 49,516,981 | 0.520 |
| `sha512-256/rustcrypto/8KB` | 98,324,769 | 51,157,866 | 0.520 |
| `sha512-256/rustcrypto/16KB` | 195,089,965 | 101,504,466 | 0.520 |
| `sha512-256/rind/16KB` | 188,773,031 | 98,217,827 | 0.520 |
| `sha512/rustcrypto/16KB` | 195,092,515 | 101,505,841 | 0.520 |
| `sha512/rind/16KB` | 188,780,674 | 98,221,936 | 0.520 |
| `sha512/rustcrypto/1200B` | 15,177,499 | 7,896,907 | 0.520 |
| `sha512-256/rind/1350B` | 16,197,089 | 8,427,473 | 0.520 |
| `sha512/rind/1350B` | 16,189,729 | 8,423,650 | 0.520 |
| `sha512-256/rustcrypto/1200B` | 15,165,471 | 7,890,981 | 0.520 |
| `sha512-256/rustcrypto/1350B` | 16,680,693 | 8,679,474 | 0.520 |
| `sha256/rind/32B` | 66,863 | 36,374 | 0.544 |
| `sha256/rustcrypto/32B` | 49,018 | 27,840 | 0.568 |
| `sha256/rind/64B` | 72,116 | 41,516 | 0.576 |
| `sha256/rustcrypto/64B` | 61,709 | 36,860 | 0.597 |
| `sha256-updates/rind/16B` | 937,567 | 651,119 | 0.694 |
| `sha256/rind/1200B` | 180,207 | 136,940 | 0.760 |
| `sha256-updates/rind/1B` | 1,228,621 | 935,741 | 0.762 |
| `sha256/rind/1350B` | 195,050 | 151,783 | 0.778 |
| `sha256/rustcrypto/1200B` | 150,740 | 125,850 | 0.835 |
| `sha256/rustcrypto/1350B` | 166,559 | 141,535 | 0.850 |
| `sha256/rind/8KB` | 735,255 | 689,217 | 0.937 |
| `sha256/libcrux/32B` | 5,822 | 5,566 | 0.956 |
| `sha256/rind/16KB` | 1,384,140 | 1,330,344 | 0.961 |
| `sha256/rustcrypto/8KB` | 726,394 | 701,264 | 0.965 |
| `sha256/libcrux/64B` | 10,852 | 10,551 | 0.972 |
| `sha256/rustcrypto/16KB` | 1,396,642 | 1,371,512 | 0.982 |
| `sha256/libcrux/1350B` | 112,104 | 110,495 | 0.986 |

