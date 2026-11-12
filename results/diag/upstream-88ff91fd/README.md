<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: upstream rind 88ff91fd against the pinned 99b509a6

rind's main moved 19 commits past the pin by 2026-11-12 (to `88ff91fd`, 2026-11-12 09:03 PST):
64-bit limbs on 64-bit targets (#614), generic `p256_mul32`/`p384_mul32` code, rewritten
Keccak/SHA-3, ML-KEM `sample_ntt`, SHA-NI assembly, and `admin/bench-report` gaining SHA3-256/512
rows at 64 B, 1350 B and 16 KB. The bench files are byte-identical between the two commits, and
patches 0001-0004 apply to both. The three benches those commits touch on this target were rebuilt
at 88ff91fd (`sha3.elf` `1fe911fe…`, `p256.elf` `929f1435…`, `mlkem768.elf` `9d191f3c…`, rind-bench
0463305's harness) and run 3 times each, 2026-11-12 10:13-10:24 PST. The SHA-3 baseline at the pin is
`pin-99b509a6/sha3` (`sha3.elf` `07f200c6…`, the same harness, 3 runs, 10:28-10:30 PST). All at the 40 MHz flash
clock, so comparable with each other and with the 40 MHz published set, not with the 80 MHz one.

- **SHA-3: rind 2.2x faster** (0.452-0.469x cycles), from behind libcrux to ahead of it:
  `sha3-256/…/16KB` rind 151 c/B against libcrux 158 and RustCrypto 273 (it was 332). For scale,
  CycloneCRYPTO's published ESP32-S3 software SHA3-256 is 1.10 MB/s, ~218 c/B.
- **ML-KEM-768: rind keygen 0.61x, encaps 0.64x, decaps unchanged**: keygen 1.96 M, encaps
  2.16 M, decaps 1.50 M cycles, against RustCrypto's 3.64 M / 4.27 M / 4.88 M.
- **P-256: rind signing 1.09x slower** (`p256-ecdsa-sign/rind` 8.94 M to 9.79 M), the other rind
  rows within 3%. The new generic 32-bit field code is slightly slower here.

When the pin moves to 88ff91fd or later, these are the rows that change.

## SHA-3, pin against head


## sha3

Known-answer checks: 99b509a6 45/45, 88ff91fd 45/45. Runs: 3 and 3. Rows: 54 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 12 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 6 | 1.000 | 1.000 to 1.000 |
| rind | long | 12 | 0.455 | 0.452 to 0.457 |
| rind | short | 6 | 0.467 | 0.465 to 0.469 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.000 |

18 of 54 rows moved by more than max(1%, run spread):

| Row | 99b509a6 cycles | 88ff91fd cycles | B/A |
|---|---:|---:|---:|
| `sha3-512/rind/16KB` | 10,182,300 | 4,606,604 | 0.452 |
| `sha3-512/rind/8KB` | 5,091,854 | 2,304,085 | 0.453 |
| `sha3-512/rind/1350B` | 849,381 | 385,006 | 0.453 |
| `sha3-512/rind/1200B` | 760,026 | 344,547 | 0.453 |
| `sha3-384/rind/16KB` | 7,075,867 | 3,211,229 | 0.454 |
| `sha3-384/rind/8KB` | 3,538,211 | 1,606,954 | 0.454 |
| `sha3-384/rind/1200B` | 538,216 | 245,063 | 0.455 |
| `sha3-256/rind/16KB` | 5,433,202 | 2,474,130 | 0.455 |
| `sha3-384/rind/1350B` | 583,244 | 265,657 | 0.455 |
| `sha3-256/rind/8KB` | 2,739,479 | 1,248,312 | 0.456 |
| `sha3-256/rind/1350B` | 450,175 | 205,910 | 0.457 |
| `sha3-256/rind/1200B` | 405,182 | 185,360 | 0.457 |
| `sha3-512/rind/32B` | 45,620 | 21,226 | 0.465 |
| `sha3-256/rind/32B` | 45,608 | 21,254 | 0.466 |
| `sha3-384/rind/32B` | 45,612 | 21,294 | 0.467 |
| `sha3-512/rind/64B` | 45,772 | 21,390 | 0.467 |
| `sha3-256/rind/64B` | 45,760 | 21,418 | 0.468 |
| `sha3-384/rind/64B` | 45,764 | 21,458 | 0.469 |

## ML-KEM-768 and P-256, published (pin) against head


## mlkem768

Known-answer checks: 99b509a6 9/9, 88ff91fd 9/9. Runs: 3 and 3. Rows: 12 in both; only in 88ff91fd: 2.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux-ml-kem | per op | 4 | 0.993 | 0.988 to 0.999 |
| rind | per op | 4 | 0.658 | 0.610 to 1.011 |
| rustcrypto | per op | 4 | 0.999 | 0.998 to 1.002 |

4 of 12 rows moved by more than max(1%, run spread):

| Row | 99b509a6 cycles | 88ff91fd cycles | B/A |
|---|---:|---:|---:|
| `mlkem768-keygen/rind` | 3,207,475 | 1,957,906 | 0.610 |
| `mlkem768-encaps/rind` | 3,398,294 | 2,156,871 | 0.635 |
| `mlkem768-combined/rind` | 8,254,124 | 5,627,831 | 0.682 |
| `mlkem768-decaps/rind` | 1,486,255 | 1,503,076 | 1.011 |

Rows only in 88ff91fd:

| Row | cycles | spread | c/B |
|---|---:|---:|---:|
| `harness/empty/iter` | 17 | 0.00% | |
| `harness/empty/iter_batched` | 9 | 0.00% | |

## p256

Known-answer checks: 99b509a6 12/12, 88ff91fd 12/12. Runs: 3 and 3. Rows: 21 in both; only in 88ff91fd: 2.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 0.998 | 0.997 to 0.999 |
| p256-rustcrypto | per op | 2 | 0.997 | 0.988 to 1.006 |
| rind | per op | 9 | 0.990 | 0.974 to 1.095 |
| ring | per op | 4 | 0.989 | 0.986 to 0.990 |
| rustcrypto | per op | 2 | 0.988 | 0.979 to 0.996 |

11 of 21 rows moved by more than max(1%, run spread):

| Row | 99b509a6 cycles | 88ff91fd cycles | B/A |
|---|---:|---:|---:|
| `p256-ecdsa-sign/rind` | 8,943,069 | 9,793,635 | 1.095 |
| `p256-ecdsa-sign-digest/rind` | 8,695,695 | 9,504,862 | 1.093 |
| `p256-ecdh-stored/rind` | 9,581,667 | 9,335,049 | 0.974 |
| `p256-ecdsa-sign/rustcrypto` | 8,272,404 | 8,096,987 | 0.979 |
| `p256-ecdsa-verify/rind` | 12,750,926 | 12,520,597 | 0.982 |
| `p256-ecdh/rind` | 14,964,337 | 14,737,205 | 0.985 |
| `p256-ecdsa-sign/ring` | 78,171,361 | 77,042,017 | 0.986 |
| `p256-ecdh/p256-rustcrypto` | 21,233,744 | 20,969,410 | 0.988 |
| `p256-ecdsa-verify/ring` | 331,503,079 | 327,729,571 | 0.989 |
| `p256-ecdh/ring` | 332,010,470 | 328,543,413 | 0.990 |
| `p256-ecdsa-verify-digest/rind` | 9,392,140 | 9,297,616 | 0.990 |

Rows only in 88ff91fd:

| Row | cycles | spread | c/B |
|---|---:|---:|---:|
| `harness/empty/iter` | 17 | 0.00% | |
| `harness/empty/iter_batched` | 9 | 0.00% | |

