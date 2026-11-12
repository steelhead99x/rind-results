Cycles, pad1031 over pad0 (below 1 is faster in pad1031).

## aes_gcm

Known-answer checks: pad0 21/21, pad1031 21/21. Runs: 3 and 3. Rows: 167 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 12 | 1.001 | 1.000 to 1.002 |
| libcrux | short | 24 | 1.001 | 0.996 to 1.008 |
| rind | long | 18 | 0.998 | 0.997 to 0.999 |
| rind | per op | 3 | 1.001 | 0.998 to 1.001 |
| rind | short | 36 | 1.000 | 0.998 to 1.003 |
| ring | long | 12 | 0.999 | 0.999 to 1.000 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 0.999 | 0.996 to 1.002 |
| rustcrypto | long | 12 | 1.010 | 1.008 to 1.012 |
| rustcrypto | short | 24 | 1.009 | 1.007 to 1.011 |

13 of 167 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `aes128-gcm-open/rustcrypto/16KB` | 3,781,823 | 3,826,199 | 1.012 |
| `aes128-gcm-open/rustcrypto/1200B` | 292,259 | 295,609 | 1.011 |
| `aes128-gcm-open/rustcrypto/1350B` | 331,367 | 335,148 | 1.011 |
| `aes128-gcm-open/rustcrypto/quic-512B` | 129,329 | 130,751 | 1.011 |
| `aes256-gcm/rustcrypto/16KB` | 4,583,787 | 4,632,798 | 1.011 |
| `aes256-gcm-open/rustcrypto/16KB` | 4,607,540 | 4,656,549 | 1.011 |
| `aes256-gcm/rustcrypto/1200B` | 354,201 | 357,878 | 1.010 |
| `aes128-gcm-open/rustcrypto/quic-256B` | 70,437 | 71,167 | 1.010 |
| `aes256-gcm-open/rustcrypto/1200B` | 356,112 | 359,798 | 1.010 |
| `aes256-gcm/rustcrypto/1350B` | 401,697 | 405,842 | 1.010 |
| `aes256-gcm-open/rustcrypto/1350B` | 403,828 | 407,973 | 1.010 |
| `aes256-gcm/rustcrypto/quic-512B` | 156,541 | 158,139 | 1.010 |
| `aes256-gcm-open/rustcrypto/quic-512B` | 157,478 | 159,066 | 1.010 |

## chacha20poly1305

Known-answer checks: pad0 21/21, pad1031 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 8 | 1.001 | 1.000 to 1.004 |
| rind | long | 6 | 0.991 | 0.990 to 0.991 |
| rind | short | 8 | 0.997 | 0.992 to 1.001 |
| ring | long | 6 | 1.000 | 0.999 to 1.000 |
| ring | short | 8 | 0.999 | 0.992 to 1.001 |
| rustcrypto | long | 6 | 0.987 | 0.981 to 1.003 |
| rustcrypto | short | 8 | 0.988 | 0.981 to 0.995 |

12 of 56 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/rustcrypto/1350B` | 68,956 | 67,628 | 0.981 |
| `chacha20poly1305-open/rustcrypto/quic-512B` | 27,894 | 27,364 | 0.981 |
| `chacha20poly1305-open/rustcrypto/quic-256B` | 15,893 | 15,602 | 0.982 |
| `chacha20poly1305-open/rustcrypto/quic-128B` | 9,894 | 9,723 | 0.983 |
| `chacha20poly1305/rustcrypto/16KB` | 807,682 | 796,150 | 0.986 |
| `chacha20poly1305/rustcrypto/1200B` | 62,843 | 61,994 | 0.986 |
| `chacha20poly1305/rustcrypto/1350B` | 71,735 | 70,767 | 0.987 |
| `chacha20poly1305/rustcrypto/quic-512B` | 28,823 | 28,480 | 0.988 |
| `chacha20poly1305/rustcrypto/quic-256B` | 16,280 | 16,091 | 0.988 |
| `chacha20poly1305/rustcrypto/quic-128B` | 10,008 | 9,895 | 0.989 |
| `chacha20poly1305/rind/16KB` | 900,589 | 891,142 | 0.990 |
| `chacha20poly1305-open/rind/16KB` | 924,229 | 914,810 | 0.990 |

## cipher

Known-answer checks: pad0 42/42, pad1031 42/42. Runs: 3 and 3. Rows: 42 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| aes128 | per op | 2 | 1.000 | 0.999 to 1.000 |
| aes256 | per op | 2 | 1.000 | 0.999 to 1.000 |
| chacha20 | per op | 2 | 1.001 | 0.985 to 1.017 |
| rind | long | 27 | 1.002 | 1.000 to 1.005 |
| rind | short | 9 | 1.003 | 0.999 to 1.004 |

2 of 42 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `quic-header-protection/chacha20/ring` | 1,528 | 1,553 | 1.017 |
| `quic-header-protection/chacha20/rind` | 1,497 | 1,474 | 0.985 |

## ed25519

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 3 | 1.000 | 1.000 to 1.001 |
| libcrux | per op | 3 | 1.000 | 1.000 to 1.001 |
| rind | per op | 3 | 1.000 | 1.000 to 1.001 |
| ring | per op | 3 | 1.005 | 0.998 to 1.005 |

0 of 12 rows moved by more than max(1%, run spread).

## hkdf

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.001 | 0.993 to 1.006 |
| expand-aes-256-gcm-key | per op | 8 | 0.999 | 0.997 to 1.003 |
| extract-expand | per op | 15 | 1.002 | 0.993 to 1.007 |

0 of 38 rows moved by more than max(1%, run spread).

## hmac

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.001 | 0.998 to 1.004 |
| libcrux | short | 4 | 1.001 | 0.997 to 1.004 |
| rind | long | 24 | 0.998 | 0.997 to 1.004 |
| rind | short | 12 | 1.001 | 0.998 to 1.005 |
| ring | long | 24 | 1.000 | 0.997 to 1.018 |
| ring | short | 12 | 0.993 | 0.988 to 1.003 |
| rustcrypto | long | 12 | 1.001 | 0.991 to 1.006 |
| rustcrypto | short | 6 | 1.001 | 0.992 to 1.005 |

11 of 102 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `hmac-sha1/ring/reused-key/16KB` | 489,398 | 497,971 | 1.018 |
| `hmac-sha1/ring/16KB` | 495,219 | 503,807 | 1.017 |
| `hmac-sha1/ring/reused-key/8KB` | 248,630 | 252,851 | 1.017 |
| `hmac-sha1/ring/8KB` | 254,451 | 258,687 | 1.017 |
| `hmac-sha1/ring/reused-key/1350B` | 47,408 | 48,002 | 1.013 |
| `hmac-sha256/ring/reused-key/64B` | 11,676 | 11,531 | 0.988 |
| `hmac-sha1/ring/1350B` | 53,186 | 53,840 | 1.012 |
| `hmac-sha256/ring/reused-key/32B` | 9,156 | 9,043 | 0.988 |
| `hmac-sha1/ring/reused-key/32B` | 7,836 | 7,748 | 0.989 |
| `hmac-sha1/ring/reused-key/1200B` | 41,779 | 42,238 | 1.011 |
| `hmac-sha256/ring/64B` | 18,858 | 18,658 | 0.989 |

## mlkem768

Known-answer checks: pad0 9/9, pad1031 9/9. Runs: 3 and 3. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux-ml-kem | per op | 4 | 1.000 | 0.999 to 1.002 |
| rind | per op | 4 | 1.001 | 1.000 to 1.004 |
| rustcrypto | per op | 4 | 0.990 | 0.988 to 0.991 |

3 of 12 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `mlkem768-keygen/rustcrypto` | 3,196,781 | 3,157,323 | 0.988 |
| `mlkem768-combined/rustcrypto` | 9,573,966 | 9,474,546 | 0.990 |
| `mlkem768-decaps/rustcrypto` | 3,159,637 | 3,127,686 | 0.990 |

## p256

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 1.001 | 1.000 to 1.001 |
| p256-rustcrypto | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | per op | 9 | 0.999 | 0.994 to 1.001 |
| ring | per op | 4 | 1.000 | 0.999 to 1.000 |
| rustcrypto | per op | 2 | 1.000 | 1.000 to 1.000 |

0 of 21 rows moved by more than max(1%, run spread).

## sha2

Known-answer checks: pad0 33/33, pad1031 33/33. Runs: 3 and 3. Rows: 68 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.002 | 1.001 to 1.004 |
| libcrux | short | 4 | 1.003 | 1.002 to 1.004 |
| rind | long | 12 | 1.001 | 1.001 to 1.002 |
| rind | short | 8 | 1.001 | 0.993 to 1.006 |
| ring | long | 12 | 1.000 | 1.000 to 1.002 |
| ring | short | 6 | 0.995 | 0.995 to 1.020 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.001 |
| rustcrypto | short | 6 | 1.000 | 0.999 to 1.001 |

2 of 68 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `sha256/ring/32B` | 4,338 | 4,427 | 1.020 |
| `sha256/ring/64B` | 6,796 | 6,921 | 1.018 |

## sha3

Known-answer checks: pad0 45/45, pad1031 45/45. Runs: 3 and 3. Rows: 54 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 12 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 6 | 1.001 | 0.998 to 1.001 |
| rind | long | 12 | 1.000 | 1.000 to 1.001 |
| rind | short | 6 | 0.999 | 0.999 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.000 |

0 of 54 rows moved by more than max(1%, run spread).

## x25519

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 9 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 2 | 1.003 | 1.002 to 1.004 |
| libcrux | per op | 2 | 0.998 | 0.998 to 0.998 |
| rind | per op | 3 | 1.003 | 1.001 to 1.005 |
| ring | per op | 2 | 0.999 | 0.998 to 1.000 |

0 of 9 rows moved by more than max(1%, run spread).

## xchacha20poly1305

Known-answer checks: pad0 21/21, pad1031 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.000 | 0.999 to 1.002 |
| libcrux | short | 4 | 0.992 | 0.988 to 1.001 |
| rind | long | 3 | 1.001 | 0.999 to 1.004 |
| rind | short | 4 | 1.003 | 0.996 to 1.004 |
| rustcrypto | long | 3 | 1.004 | 1.000 to 1.022 |
| rustcrypto | short | 4 | 1.000 | 0.996 to 1.008 |

1 of 21 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rustcrypto/16KB` | 945,315 | 966,413 | 1.022 |

