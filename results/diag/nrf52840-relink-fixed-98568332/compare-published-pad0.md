Cycles, pad0-head over published (below 1 is faster in pad0-head).

## chacha20poly1305

Known-answer checks: published 21/21, pad0-head 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.003 | 1.003 to 1.003 |
| libcrux | short | 8 | 1.003 | 1.000 to 1.005 |
| rind | long | 6 | 0.994 | 0.994 to 0.995 |
| rind | short | 8 | 0.996 | 0.995 to 0.998 |
| ring | long | 6 | 1.000 | 1.000 to 1.001 |
| ring | short | 8 | 1.000 | 0.998 to 1.003 |
| rustcrypto | long | 6 | 1.002 | 1.000 to 1.030 |
| rustcrypto | short | 8 | 1.002 | 1.000 to 1.004 |

2 of 56 rows moved by more than max(1%, run spread):

| Row | published cycles | pad0-head cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/rustcrypto/16KB` | 753,585 | 776,397 | 1.030 |
| `chacha20poly1305-open/rustcrypto/1200B` | 59,465 | 60,718 | 1.021 |

## hkdf

Known-answer checks: published 12/12, pad0-head 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 0.996 to 1.003 |
| expand-aes-256-gcm-key | per op | 8 | 0.999 | 0.996 to 1.001 |
| extract-expand | per op | 15 | 1.000 | 0.996 to 1.003 |

0 of 38 rows moved by more than max(1%, run spread).

## hmac

Known-answer checks: published 12/12, pad0-head 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.001 | 0.998 to 1.004 |
| libcrux | short | 4 | 1.001 | 0.998 to 1.004 |
| rind | long | 24 | 0.998 | 0.997 to 1.004 |
| rind | short | 12 | 1.000 | 0.998 to 1.005 |
| ring | long | 24 | 1.000 | 0.999 to 1.001 |
| ring | short | 12 | 0.998 | 0.995 to 1.000 |
| rustcrypto | long | 12 | 1.004 | 0.994 to 1.007 |
| rustcrypto | short | 6 | 1.003 | 0.994 to 1.006 |

0 of 102 rows moved by more than max(1%, run spread).

## sha2

Known-answer checks: published 33/33, pad0-head 33/33. Runs: 3 and 3. Rows: 68 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.001 | 0.998 to 1.004 |
| libcrux | short | 4 | 1.001 | 0.999 to 1.005 |
| rind | long | 12 | 0.999 | 0.998 to 1.003 |
| rind | short | 8 | 1.000 | 0.998 to 1.062 |
| ring | long | 12 | 1.000 | 1.000 to 1.001 |
| ring | short | 6 | 1.000 | 1.000 to 1.009 |
| rustcrypto | long | 12 | 0.999 | 0.999 to 1.012 |
| rustcrypto | short | 6 | 0.999 | 0.998 to 1.011 |

6 of 68 rows moved by more than max(1%, run spread):

| Row | published cycles | pad0-head cycles | B/A |
|---|---:|---:|---:|
| `sha256-updates/rind/1B` | 714,707 | 759,019 | 1.062 |
| `sha256/rustcrypto/16KB` | 917,275 | 928,324 | 1.012 |
| `sha256/rustcrypto/8KB` | 460,827 | 466,372 | 1.012 |
| `sha256/rustcrypto/1350B` | 79,307 | 80,251 | 1.012 |
| `sha256/rustcrypto/1200B` | 68,614 | 69,426 | 1.012 |
| `sha256/rustcrypto/64B` | 7,903 | 7,987 | 1.011 |

