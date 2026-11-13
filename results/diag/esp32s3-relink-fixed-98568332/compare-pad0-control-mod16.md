Cycles, pad0-control over published (below 1 is faster in pad0-control).

## aes_gcm

Known-answer checks: published 27/27, pad0-control 27/27. Runs: 3 and 3. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 1.000 | 0.998 to 1.001 |
| libcrux | short | 24 | 1.000 | 1.000 to 1.000 |
| rind | long | 18 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 36 | 1.000 | 1.000 to 1.000 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 1.000 | 1.000 to 1.003 |
| rustcrypto | long | 12 | 1.000 | 0.999 to 1.001 |
| rustcrypto | short | 24 | 1.000 | 0.999 to 1.004 |

0 of 171 rows moved by more than max(1%, run spread).

## chacha20poly1305

Known-answer checks: published 21/21, pad0-control 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 8 | 1.000 | 1.000 to 1.000 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

0 of 56 rows moved by more than max(1%, run spread).

## xchacha20poly1305

Known-answer checks: published 21/21, pad0-control 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.001 |
| rind | long | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 4 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

0 of 21 rows moved by more than max(1%, run spread).

## cipher

Known-answer checks: published 48/48, pad0-control 48/48. Runs: 3 and 3. Rows: 42 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| aes128 | per op | 2 | 1.000 | 1.000 to 1.000 |
| aes256 | per op | 2 | 1.000 | 1.000 to 1.000 |
| chacha20 | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | long | 27 | 1.000 | 1.000 to 1.000 |
| rind | short | 9 | 1.000 | 1.000 to 1.000 |

0 of 42 rows moved by more than max(1%, run spread).

## sha2

Known-answer checks: published 39/39, pad0-control 39/39. Runs: 3 and 3. Rows: 74 in both.

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

## sha3

Known-answer checks: published 45/45, pad0-control 45/45. Runs: 3 and 3. Rows: 54 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 12 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 6 | 1.000 | 1.000 to 1.000 |
| rind | long | 12 | 1.000 | 1.000 to 1.000 |
| rind | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.000 |

0 of 54 rows moved by more than max(1%, run spread).

## hmac

Known-answer checks: published 12/12, pad0-control 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.000 |
| rind | long | 24 | 1.000 | 1.000 to 1.000 |
| rind | short | 12 | 1.000 | 1.000 to 1.000 |
| ring | long | 24 | 1.000 | 1.000 to 1.000 |
| ring | short | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.000 |

0 of 102 rows moved by more than max(1%, run spread).

## hkdf

Known-answer checks: published 12/12, pad0-control 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 1.000 to 1.000 |
| expand-aes-256-gcm-key | per op | 8 | 1.000 | 1.000 to 1.000 |
| extract-expand | per op | 15 | 1.000 | 0.999 to 1.000 |

0 of 38 rows moved by more than max(1%, run spread).

