Cycles, board-2 over board-1 (below 1 is faster in board-2).

## aes_gcm

Known-answer checks: board-1 45/45, board-2 45/45. Runs: 5 and 5. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 24 | 1.000 | 1.000 to 1.000 |
| rind | long | 18 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 36 | 1.000 | 1.000 to 1.000 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.001 |
| rustcrypto | short | 24 | 1.000 | 0.977 to 1.003 |

0 of 171 rows moved by more than max(1%, run spread).

## chacha20poly1305

Known-answer checks: board-1 35/35, board-2 35/35. Runs: 5 and 5. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.000 | 1.000 to 1.011 |
| libcrux | short | 8 | 1.000 | 1.000 to 1.000 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

0 of 56 rows moved by more than max(1%, run spread).

## cipher

Known-answer checks: board-1 80/80, board-2 80/80. Runs: 5 and 5. Rows: 42 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| aes128 | per op | 2 | 1.000 | 1.000 to 1.000 |
| aes256 | per op | 2 | 1.000 | 1.000 to 1.000 |
| chacha20 | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | long | 27 | 1.000 | 1.000 to 1.000 |
| rind | short | 9 | 1.000 | 1.000 to 1.000 |

0 of 42 rows moved by more than max(1%, run spread).

## ed25519

Known-answer checks: board-1 20/20, board-2 20/20. Runs: 5 and 5. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 3 | 1.000 | 1.000 to 1.000 |
| libcrux | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| ring | per op | 3 | 1.000 | 1.000 to 1.000 |

0 of 12 rows moved by more than max(1%, run spread).

## hkdf

Known-answer checks: board-1 20/20, board-2 20/20. Runs: 5 and 5. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 1.000 to 1.000 |
| expand-aes-256-gcm-key | per op | 8 | 1.000 | 1.000 to 1.000 |
| extract-expand | per op | 15 | 1.000 | 1.000 to 1.003 |

0 of 38 rows moved by more than max(1%, run spread).

## hmac

Known-answer checks: board-1 20/20, board-2 20/20. Runs: 5 and 5. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.000 |
| rind | long | 24 | 1.000 | 1.000 to 1.000 |
| rind | short | 12 | 1.000 | 1.000 to 1.000 |
| ring | long | 24 | 1.000 | 1.000 to 1.000 |
| ring | short | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.003 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.000 |

0 of 102 rows moved by more than max(1%, run spread).

## mlkem768

Known-answer checks: board-1 15/15, board-2 15/15. Runs: 5 and 5. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux-ml-kem | per op | 4 | 1.000 | 1.000 to 1.000 |
| rind | per op | 4 | 1.000 | 1.000 to 1.000 |
| rustcrypto | per op | 4 | 1.000 | 1.000 to 1.000 |

0 of 12 rows moved by more than max(1%, run spread).

## p256

Known-answer checks: board-1 20/20, board-2 20/20. Runs: 5 and 5. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 1.000 | 1.000 to 1.000 |
| p256-rustcrypto | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | per op | 9 | 1.000 | 1.000 to 1.000 |
| ring | per op | 4 | 1.000 | 1.000 to 1.000 |
| rustcrypto | per op | 2 | 1.000 | 1.000 to 1.000 |

0 of 21 rows moved by more than max(1%, run spread).

## sha2

Known-answer checks: board-1 65/65, board-2 65/65. Runs: 5 and 5. Rows: 74 in both.

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

Known-answer checks: board-1 75/75, board-2 75/75. Runs: 5 and 5. Rows: 54 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 12 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 6 | 1.000 | 1.000 to 1.000 |
| rind | long | 12 | 1.000 | 1.000 to 1.000 |
| rind | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.000 |

0 of 54 rows moved by more than max(1%, run spread).

## x25519

Known-answer checks: board-1 20/20, board-2 20/20. Runs: 5 and 5. Rows: 9 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |

0 of 9 rows moved by more than max(1%, run spread).

## xchacha20poly1305

Known-answer checks: board-1 35/35, board-2 35/35. Runs: 5 and 5. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.000 |
| rind | long | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 4 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

0 of 21 rows moved by more than max(1%, run spread).

