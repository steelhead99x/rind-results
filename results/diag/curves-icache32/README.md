<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: X25519 and P-256 with a 32 KB instruction cache

> **Rerun at 80 MHz (2026-11-12).** [`../esp32s3-icache32-80mhz-98568332/`](../esp32s3-icache32-80mhz-98568332/) repeats this at the published pin (98568332) and flash clock (80 MHz), three runs each, with the same findings (`x25519-ecdh/ring` 0.105x, `x25519-ecdh/libcrux` 0.061x, ring's P-256 0.085x-0.132x, libcrux's P-256 0.96x).

Why do ring's and libcrux's curve operations take 30-150x more cycles on the ESP32-S3 than on the
nRF52840 (ring `x25519-ecdh` 167 M cycles here, 3.06 M there)? Rebuilt with
`sdkconfig.icache32` (32 KB instruction cache, 32 B lines; a diagnostic, not the published
configuration): `x25519.elf` `45d878d2…`, `p256.elf` `4040aa42…`, from rind 99b509a6, one run each,
2026-11-12 10:24-10:27 PST, 40 MHz flash clock like the published set.

- **Instruction-cache thrash, for ring and libcrux's X25519**: `x25519-ecdh/ring` 0.067x
  (167 M to 11.3 M cycles), `x25519-ecdh/libcrux` 0.044x (673 M to 29 M), ring's P-256 0.047-0.076x
  (`p256-ecdh/ring` 332 M to 16 M). Their field arithmetic is big straight-line code that does not
  fit 16 KB and streams from flash on every call.
- **Not the cache, for libcrux's P-256**: 0.96x (754 M to 723 M). It builds at opt-level `s`
  (its field inversion overflows `l32r` at 3, see rind-bench's README), and stays ~50x rind.
- rind and dalek gain 6-21%: they fit already.

So the published ESP32-S3 curve rows for ring and libcrux measure the cache, and the gap.toml
reasons that call rind's wins there "large" should be read with that. Made with
`tools/compare.py raw diag/curves-icache32 x25519 p256`:


## x25519

Known-answer checks: published 12/12, icache32 4/4. Runs: 3 and 1. Rows: 9 in both; only in icache32: 2.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 2 | 0.851 | 0.820 to 0.883 |
| libcrux | per op | 2 | 0.044 | 0.044 to 0.044 |
| rind | per op | 3 | 0.878 | 0.793 to 0.939 |
| ring | per op | 2 | 0.501 | 0.067 to 0.934 |

9 of 9 rows moved by more than max(1%, run spread):

| Row | published cycles | icache32 cycles | B/A |
|---|---:|---:|---:|
| `x25519-ecdh/libcrux` | 672,831,383 | 29,435,761 | 0.044 |
| `x25519-keygen/libcrux` | 336,534,091 | 14,730,305 | 0.044 |
| `x25519-ecdh/ring` | 167,160,273 | 11,264,410 | 0.067 |
| `x25519-keygen/rind` | 1,920,413 | 1,523,778 | 0.793 |
| `x25519-keygen/dalek` | 1,849,399 | 1,515,604 | 0.820 |
| `x25519-ecdh/rind` | 7,248,527 | 6,367,553 | 0.878 |
| `x25519-ecdh/dalek` | 7,203,368 | 6,359,489 | 0.883 |
| `x25519-keygen/ring` | 5,346,321 | 4,992,631 | 0.934 |
| `x25519-ecdh-stored/rind` | 5,151,040 | 4,835,933 | 0.939 |

Rows only in icache32:

| Row | cycles | spread | c/B |
|---|---:|---:|---:|
| `harness/empty/iter` | 17 | 0.00% | |
| `harness/empty/iter_batched` | 9 | 0.00% | |

## p256

Known-answer checks: published 12/12, icache32 4/4. Runs: 3 and 1. Rows: 21 in both; only in icache32: 2.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 0.959 | 0.957 to 0.960 |
| p256-rustcrypto | per op | 2 | 0.910 | 0.881 to 0.938 |
| rind | per op | 9 | 0.861 | 0.805 to 0.969 |
| ring | per op | 4 | 0.053 | 0.047 to 0.076 |
| rustcrypto | per op | 2 | 0.878 | 0.773 to 0.982 |

21 of 21 rows moved by more than max(1%, run spread):

| Row | published cycles | icache32 cycles | B/A |
|---|---:|---:|---:|
| `p256-ecdsa-verify/ring` | 331,503,079 | 15,532,303 | 0.047 |
| `p256-ecdh/ring` | 332,010,470 | 16,174,455 | 0.049 |
| `p256-keygen/ring` | 76,798,131 | 4,422,156 | 0.058 |
| `p256-ecdsa-sign/ring` | 78,171,361 | 5,906,413 | 0.076 |
| `p256-ecdsa-sign/rustcrypto` | 8,272,404 | 6,395,825 | 0.773 |
| `p256-ecdsa-keygen/rind` | 5,970,693 | 4,807,545 | 0.805 |
| `p256-keygen/rind` | 5,376,432 | 4,498,986 | 0.837 |
| `p256-ecdh-stored/rind` | 9,581,667 | 8,030,015 | 0.838 |
| `p256-ecdsa-sign/rind` | 8,943,069 | 7,647,793 | 0.855 |
| `p256-ecdh/rind` | 14,964,337 | 12,888,764 | 0.861 |
| `p256-ecdsa-sign-digest/rind` | 8,695,695 | 7,635,124 | 0.878 |
| `p256-keygen/p256-rustcrypto` | 4,800,497 | 4,230,141 | 0.881 |
| `p256-ecdsa-verify/rind` | 12,750,926 | 11,324,356 | 0.888 |
| `p256-ecdh/p256-rustcrypto` | 21,233,744 | 19,923,630 | 0.938 |
| `p256-ecdsa-sign/libcrux` | 235,789,730 | 225,710,999 | 0.957 |
| `p256-ecdsa-verify/libcrux` | 625,267,181 | 599,187,118 | 0.958 |
| `p256-ecdh/libcrux` | 754,200,113 | 723,059,786 | 0.959 |
| `p256-ecdsa-verify-digest/rind` | 9,392,140 | 9,008,848 | 0.959 |
| `p256-keygen/libcrux` | 228,665,704 | 219,583,239 | 0.960 |
| `p256-ecdsa-verify-parsed/rind` | 9,824,251 | 9,516,640 | 0.969 |
| `p256-ecdsa-verify/rustcrypto` | 18,382,113 | 18,049,239 | 0.982 |

Rows only in icache32:

| Row | cycles | spread | c/B |
|---|---:|---:|---:|
| `harness/empty/iter` | 17 | 0.00% | |
| `harness/empty/iter_batched` | 9 | 0.00% | |

