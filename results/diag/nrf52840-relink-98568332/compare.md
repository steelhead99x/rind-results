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

