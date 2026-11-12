Cycles, pad2053 over pad0 (below 1 is faster in pad2053).

## chacha20poly1305

Known-answer checks: pad0 21/21, pad2053 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 8 | 1.000 | 1.000 to 1.000 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 0.999 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 0.999 to 1.000 |
| rustcrypto | long | 6 | 0.999 | 0.998 to 1.001 |
| rustcrypto | short | 8 | 1.000 | 0.998 to 1.001 |

0 of 56 rows moved by more than max(1%, run spread).

## hkdf

Known-answer checks: pad0 12/12, pad2053 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 0.998 to 1.003 |
| expand-aes-256-gcm-key | per op | 8 | 1.000 | 0.998 to 1.002 |
| extract-expand | per op | 15 | 1.000 | 0.997 to 1.002 |

0 of 38 rows moved by more than max(1%, run spread).

## hmac

Known-answer checks: pad0 12/12, pad2053 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 0.999 | 0.996 to 1.003 |
| libcrux | short | 4 | 0.999 | 0.997 to 1.002 |
| rind | long | 24 | 1.002 | 0.996 to 1.003 |
| rind | short | 12 | 1.001 | 0.995 to 1.004 |
| ring | long | 24 | 1.000 | 1.000 to 1.002 |
| ring | short | 12 | 1.000 | 0.998 to 1.002 |
| rustcrypto | long | 12 | 0.998 | 0.994 to 1.003 |
| rustcrypto | short | 6 | 1.000 | 0.999 to 1.002 |

0 of 102 rows moved by more than max(1%, run spread).

## sha2

Known-answer checks: pad0 33/33, pad2053 33/33. Runs: 3 and 3. Rows: 68 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.000 |
| rind | long | 12 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 0.991 to 1.003 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.001 | 0.996 to 1.001 |
| rustcrypto | long | 12 | 1.001 | 0.999 to 1.001 |
| rustcrypto | short | 6 | 1.001 | 0.997 to 1.001 |

0 of 68 rows moved by more than max(1%, run spread).

