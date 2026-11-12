<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: rechecking the published images (2026-11-12)

The published ELFs (sha256 as in `../../images.json`, checked before flashing) flashed again and
run once each, 2026-11-12 10:00-10:08 PST, the board and settings unchanged (espflash's default
40 MHz flash clock, as the published runs). The question: does a published median repeat?

**Yes.** All 334 rows of `aes_gcm`, `chacha20poly1305`, `sha2`, `p256` and `mlkem768` land within
max(1%, run spread) of the published value (ratio range 0.997-1.012), and every known-answer
check passes (9/9, 7/7, 13/13, 4/4, 3/3 per run). Run-to-run noise on one image is not where the
uncertainty in these numbers is; layout and the flash clock are (`../layout-relink-2026-11-12/`,
`../flash-80mhz/`).

Made with `tools/compare.py raw diag/esp32s3-recheck-2026-11-12 aes_gcm chacha20poly1305 sha2 p256 mlkem768`:


## aes_gcm

Known-answer checks: published 27/27, recheck 9/9. Runs: 3 and 1. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 1.000 | 0.999 to 1.006 |
| libcrux | short | 24 | 1.000 | 0.999 to 1.001 |
| rind | long | 18 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 36 | 1.000 | 1.000 to 1.000 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.999 to 1.003 |
| rustcrypto | short | 24 | 1.000 | 0.997 to 1.012 |

0 of 171 rows moved by more than max(1%, run spread).

## chacha20poly1305

Known-answer checks: published 21/21, recheck 7/7. Runs: 3 and 1. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.000 | 1.000 to 1.002 |
| libcrux | short | 8 | 1.000 | 1.000 to 1.000 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

0 of 56 rows moved by more than max(1%, run spread).

## sha2

Known-answer checks: published 39/39, recheck 13/13. Runs: 3 and 1. Rows: 74 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 8 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.000 |
| rind | long | 12 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.000 |

0 of 74 rows moved by more than max(1%, run spread).

## p256

Known-answer checks: published 12/12, recheck 4/4. Runs: 3 and 1. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 1.000 | 1.000 to 1.000 |
| p256-rustcrypto | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | per op | 9 | 1.000 | 1.000 to 1.000 |
| ring | per op | 4 | 1.000 | 1.000 to 1.000 |
| rustcrypto | per op | 2 | 1.000 | 1.000 to 1.000 |

0 of 21 rows moved by more than max(1%, run spread).

## mlkem768

Known-answer checks: published 9/9, recheck 3/3. Runs: 3 and 1. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux-ml-kem | per op | 4 | 1.000 | 1.000 to 1.000 |
| rind | per op | 4 | 1.000 | 0.999 to 1.001 |
| rustcrypto | per op | 4 | 1.000 | 1.000 to 1.000 |

0 of 12 rows moved by more than max(1%, run spread).

