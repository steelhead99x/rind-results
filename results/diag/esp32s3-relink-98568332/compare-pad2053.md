Cycles, pad2053 over pad0 (below 1 is faster in pad2053).

## aes_gcm

Known-answer checks: pad0 27/27, pad2053 27/27. Runs: 3 and 3. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 1.000 | 0.997 to 1.006 |
| libcrux | short | 24 | 1.000 | 0.975 to 1.032 |
| rind | long | 18 | 1.000 | 0.992 to 1.010 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 36 | 1.004 | 0.989 to 1.012 |
| ring | long | 12 | 0.998 | 0.985 to 1.000 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 0.921 | 0.842 to 0.990 |
| rustcrypto | long | 12 | 0.997 | 0.964 to 1.025 |
| rustcrypto | short | 24 | 1.029 | 0.784 to 1.260 |

62 of 171 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad2053 cycles | B/A |
|---|---:|---:|---:|
| `aes128-gcm-open/rustcrypto/32B` | 52,986 | 66,778 | 1.260 |
| `aes128-gcm-open/rustcrypto/quic-32B` | 58,274 | 73,230 | 1.257 |
| `aes128-gcm/rustcrypto/quic-32B` | 54,483 | 42,738 | 0.784 |
| `aes256-gcm-open/rustcrypto/32B` | 98,296 | 80,224 | 0.816 |
| `aes128-gcm/ring/quic-32B` | 41,131 | 34,618 | 0.842 |
| `aes128-gcm/ring/32B` | 43,078 | 36,482 | 0.847 |
| `aes128-gcm-open/ring/quic-32B` | 30,375 | 26,242 | 0.864 |
| `aes256-gcm/rustcrypto/32B` | 62,257 | 53,833 | 0.865 |
| `aes128-gcm-open/ring/32B` | 32,302 | 28,170 | 0.872 |
| `aes128-gcm/ring/quic-64B` | 51,972 | 45,436 | 0.874 |
| `aes256-gcm-open/ring/quic-32B` | 37,108 | 32,971 | 0.889 |
| `aes256-gcm-open/rustcrypto/quic-32B` | 97,852 | 87,220 | 0.891 |
| `aes256-gcm-open/ring/32B` | 39,034 | 34,899 | 0.894 |
| `aes128-gcm/rustcrypto/quic-64B` | 48,419 | 53,508 | 1.105 |
| `aes128-gcm-open/ring/quic-64B` | 41,147 | 37,071 | 0.901 |
| `aes256-gcm/ring/quic-32B` | 43,423 | 39,127 | 0.901 |
| `aes256-gcm-open/rustcrypto/quic-64B` | 93,632 | 102,397 | 1.094 |
| `aes256-gcm/ring/32B` | 45,347 | 41,135 | 0.907 |
| `aes128-gcm/ring/quic-128B` | 73,560 | 67,335 | 0.915 |
| `aes128-gcm-open/rustcrypto/quic-64B` | 62,241 | 67,317 | 1.082 |
| `aes256-gcm-open/ring/quic-64B` | 50,612 | 46,532 | 0.919 |
| `aes256-gcm/ring/quic-64B` | 57,153 | 52,787 | 0.924 |
| `aes128-gcm/rustcrypto/quic-128B` | 69,944 | 75,001 | 1.072 |
| `aes256-gcm/rustcrypto/quic-32B` | 67,587 | 62,943 | 0.931 |
| `aes256-gcm-open/rustcrypto/quic-128B` | 120,403 | 128,499 | 1.067 |
| `aes128-gcm-open/ring/quic-128B` | 62,809 | 58,729 | 0.935 |
| `aes128-gcm-open/rustcrypto/quic-128B` | 83,967 | 89,278 | 1.063 |
| `aes128-gcm-open/rustcrypto/quic-256B` | 127,310 | 134,417 | 1.056 |
| `aes256-gcm-open/ring/quic-128B` | 77,738 | 73,656 | 0.947 |
| `aes128-gcm/ring/quic-256B` | 116,797 | 110,918 | 0.950 |
| `aes256-gcm/ring/quic-128B` | 84,075 | 80,147 | 0.953 |
| `aes128-gcm/rustcrypto/quic-256B` | 113,414 | 118,561 | 1.045 |
| `aes128-gcm-open/ring/quic-256B` | 106,123 | 102,048 | 0.962 |
| `aes256-gcm/rustcrypto/1200B` | 558,157 | 538,228 | 0.964 |
| `aes128-gcm/libcrux/32B` | 76,964 | 79,460 | 1.032 |
| `aes256-gcm-open/rustcrypto/quic-256B` | 173,941 | 179,368 | 1.031 |
| `aes256-gcm-open/ring/quic-256B` | 131,985 | 127,904 | 0.969 |
| `aes128-gcm-open/rustcrypto/quic-512B` | 214,416 | 220,708 | 1.029 |
| `aes256-gcm/rustcrypto/1350B` | 627,538 | 609,649 | 0.971 |
| `aes128-gcm/rustcrypto/32B` | 40,191 | 41,303 | 1.028 |
| `aes256-gcm/libcrux/32B` | 149,902 | 146,097 | 0.975 |
| `aes128-gcm/libcrux/quic-32B` | 69,253 | 71,004 | 1.025 |
| `aes256-gcm/ring/quic-256B` | 138,727 | 135,282 | 0.975 |
| `aes128-gcm-open/rustcrypto/1200B` | 468,427 | 479,983 | 1.025 |
| `aes128-gcm/rustcrypto/quic-512B` | 199,714 | 204,530 | 1.024 |
| `aes128-gcm/rustcrypto/1200B` | 448,639 | 459,433 | 1.024 |
| `aes128-gcm/ring/quic-512B` | 203,272 | 198,526 | 0.977 |
| `aes128-gcm-open/ring/quic-512B` | 192,763 | 188,684 | 0.979 |
| `aes256-gcm/rustcrypto/quic-512B` | 257,939 | 252,699 | 0.980 |
| `aes128-gcm/libcrux/quic-64B` | 100,294 | 102,043 | 1.017 |
| `aes256-gcm-open/ring/quic-512B` | 240,475 | 236,392 | 0.983 |
| `aes128-gcm-open/ring/1350B` | 490,471 | 482,886 | 0.985 |
| `aes256-gcm/rustcrypto/quic-128B` | 99,638 | 101,124 | 1.015 |
| `aes128-gcm-open/rind/quic-512B` | 160,238 | 162,156 | 1.012 |
| `aes256-gcm-open/ring/1350B` | 612,162 | 605,006 | 0.988 |
| `aes256-gcm-open/libcrux/32B` | 194,500 | 192,258 | 0.988 |
| `aes256-gcm-open/rustcrypto/1200B` | 585,182 | 578,675 | 0.989 |
| `aes192-gcm/rind/quic-512B` | 181,512 | 179,584 | 0.989 |
| `aes128-gcm-open/rind/1200B` | 365,063 | 368,894 | 1.010 |
| `aes192-gcm-open/rind/quic-512B` | 178,174 | 180,020 | 1.010 |
| `aes256-gcm/rind/quic-512B` | 199,544 | 197,508 | 0.990 |
| `aes256-gcm/ring/quic-512B` | 247,112 | 244,590 | 0.990 |

## chacha20poly1305

Known-answer checks: pad0 21/21, pad2053 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 0.993 | 0.960 to 0.999 |
| libcrux | short | 8 | 0.987 | 0.968 to 0.999 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

7 of 56 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad2053 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305/libcrux/1350B` | 170,595 | 163,846 | 0.960 |
| `chacha20poly1305/libcrux/quic-256B` | 45,961 | 44,497 | 0.968 |
| `chacha20poly1305/libcrux/quic-128B` | 32,454 | 31,428 | 0.968 |
| `chacha20poly1305/libcrux/quic-512B` | 72,652 | 70,427 | 0.969 |
| `chacha20poly1305/libcrux/1200B` | 131,504 | 128,801 | 0.979 |
| `chacha20poly1305/libcrux/32B` | 12,097 | 11,894 | 0.983 |
| `chacha20poly1305-open/libcrux/1350B` | 200,985 | 198,964 | 0.990 |

## cipher

Known-answer checks: pad0 48/48, pad2053 48/48. Runs: 3 and 3. Rows: 42 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| aes128 | per op | 2 | 1.000 | 1.000 to 1.000 |
| aes256 | per op | 2 | 1.000 | 1.000 to 1.000 |
| chacha20 | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | long | 27 | 1.000 | 1.000 to 1.000 |
| rind | short | 9 | 1.000 | 1.000 to 1.000 |

0 of 42 rows moved by more than max(1%, run spread).

## ed25519

Known-answer checks: pad0 12/12, pad2053 12/12. Runs: 3 and 3. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 3 | 1.000 | 0.999 to 1.001 |
| libcrux | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 1.000 | 0.999 to 1.001 |
| ring | per op | 3 | 1.001 | 1.000 to 1.003 |

0 of 12 rows moved by more than max(1%, run spread).

## hkdf

Known-answer checks: pad0 12/12, pad2053 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 0.998 to 1.080 |
| expand-aes-256-gcm-key | per op | 8 | 1.005 | 0.998 to 1.014 |
| extract-expand | per op | 15 | 1.000 | 0.968 to 1.070 |

7 of 38 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad2053 cycles | B/A |
|---|---:|---:|---:|
| `hkdf-sha256/expand/libcrux` | 107,026 | 115,535 | 1.080 |
| `hkdf-sha256/extract-expand/rind` | 304,940 | 326,214 | 1.070 |
| `hkdf-sha256/extract-expand/libcrux` | 211,040 | 204,220 | 0.968 |
| `hkdf-sha256/expand/rind` | 100,705 | 103,055 | 1.023 |
| `hkdf-sha256/expand/rustcrypto` | 24,312 | 24,763 | 1.019 |
| `hkdf-sha256/expand-aes-256-gcm-key/rind` | 381,328 | 386,542 | 1.014 |
| `hkdf-sha1/extract-expand/rustcrypto` | 29,892 | 29,571 | 0.989 |

## hmac

Known-answer checks: pad0 12/12, pad2053 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 0.998 | 0.988 to 0.999 |
| libcrux | short | 4 | 0.992 | 0.983 to 1.000 |
| rind | long | 24 | 1.000 | 0.999 to 1.047 |
| rind | short | 12 | 1.001 | 1.000 to 1.075 |
| ring | long | 24 | 1.000 | 1.000 to 1.000 |
| ring | short | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.991 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 0.990 to 1.000 |

13 of 102 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad2053 cycles | B/A |
|---|---:|---:|---:|
| `hmac-sha256/rind/reused-key/32B` | 34,931 | 37,563 | 1.075 |
| `hmac-sha256/rind/32B` | 68,683 | 73,735 | 1.074 |
| `hmac-sha256/rind/64B` | 74,147 | 78,406 | 1.057 |
| `hmac-sha256/rind/reused-key/64B` | 40,044 | 42,308 | 1.057 |
| `hmac-sha256/rind/1200B` | 160,736 | 168,333 | 1.047 |
| `hmac-sha256/rind/1350B` | 179,030 | 183,192 | 1.023 |
| `hmac-sha256/rind/reused-key/1200B` | 121,407 | 124,033 | 1.022 |
| `hmac-sha256/rind/reused-key/1350B` | 136,248 | 138,874 | 1.019 |
| `hmac-sha256/libcrux/32B` | 36,874 | 36,256 | 0.983 |
| `hmac-sha256/libcrux/64B` | 42,174 | 41,487 | 0.984 |
| `hmac-sha256/libcrux/1200B` | 130,943 | 129,328 | 0.988 |
| `hmac-sha256/libcrux/1350B` | 145,787 | 144,172 | 0.989 |
| `hmac-sha256/rustcrypto/32B` | 42,476 | 42,049 | 0.990 |

## mlkem768

Known-answer checks: pad0 9/9, pad2053 9/9. Runs: 3 and 3. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux-ml-kem | per op | 4 | 1.002 | 1.000 to 1.009 |
| rind | per op | 4 | 1.000 | 0.999 to 1.002 |
| rustcrypto | per op | 4 | 1.000 | 1.000 to 1.003 |

0 of 12 rows moved by more than max(1%, run spread).

## p256

Known-answer checks: pad0 12/12, pad2053 12/12. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 0.999 | 0.999 to 0.999 |
| p256-rustcrypto | per op | 2 | 1.002 | 1.001 to 1.003 |
| rind | per op | 9 | 0.999 | 0.997 to 1.000 |
| ring | per op | 4 | 1.008 | 1.003 to 1.013 |
| rustcrypto | per op | 2 | 1.022 | 1.000 to 1.044 |

3 of 21 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad2053 cycles | B/A |
|---|---:|---:|---:|
| `p256-ecdsa-sign/rustcrypto` | 6,420,520 | 6,699,862 | 1.044 |
| `p256-ecdsa-sign/ring` | 41,289,166 | 41,819,317 | 1.013 |
| `p256-keygen/ring` | 40,190,689 | 40,694,625 | 1.013 |

## sha2

Known-answer checks: pad0 39/39, pad2053 39/39. Runs: 3 and 3. Rows: 74 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 8 | 1.000 | 0.997 to 1.001 |
| libcrux | short | 4 | 0.999 | 0.996 to 1.002 |
| rind | long | 12 | 1.000 | 1.000 to 1.037 |
| rind | short | 8 | 1.005 | 1.000 to 1.224 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.989 to 1.000 |
| rustcrypto | short | 6 | 0.999 | 0.990 to 1.000 |

6 of 74 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad2053 cycles | B/A |
|---|---:|---:|---:|
| `sha256/rind/32B` | 19,871 | 24,313 | 1.224 |
| `sha256/rind/64B` | 24,875 | 29,156 | 1.172 |
| `sha256/rind/1200B` | 114,294 | 118,544 | 1.037 |
| `sha256/rind/1350B` | 129,167 | 133,429 | 1.033 |
| `sha256-updates/rind/16B` | 641,418 | 650,203 | 1.014 |
| `sha256/rustcrypto/1200B` | 102,135 | 100,981 | 0.989 |

## sha3

Known-answer checks: pad0 45/45, pad2053 45/45. Runs: 3 and 3. Rows: 54 in both.

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

Known-answer checks: pad0 12/12, pad2053 12/12. Runs: 3 and 3. Rows: 9 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 2 | 1.000 | 1.000 to 1.001 |
| libcrux | per op | 2 | 1.003 | 1.003 to 1.003 |
| rind | per op | 3 | 1.000 | 0.996 to 1.000 |
| ring | per op | 2 | 0.999 | 0.998 to 1.001 |

0 of 9 rows moved by more than max(1%, run spread).

## xchacha20poly1305

Known-answer checks: pad0 21/21, pad2053 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.000 | 0.999 to 1.000 |
| libcrux | short | 4 | 0.997 | 0.996 to 1.000 |
| rind | long | 3 | 1.010 | 1.002 to 1.010 |
| rind | short | 4 | 1.013 | 1.011 to 1.057 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

6 of 21 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad2053 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rind/32B` | 288,632 | 305,070 | 1.057 |
| `xchacha20poly1305/rind/quic-128B` | 320,984 | 325,110 | 1.013 |
| `xchacha20poly1305/rind/quic-256B` | 333,500 | 337,629 | 1.012 |
| `xchacha20poly1305/rind/quic-512B` | 358,520 | 362,637 | 1.011 |
| `xchacha20poly1305/rind/1200B` | 428,783 | 433,263 | 1.010 |
| `xchacha20poly1305/rind/1350B` | 445,838 | 450,402 | 1.010 |

