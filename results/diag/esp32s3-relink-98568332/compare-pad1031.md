Cycles, pad1031 over pad0 (below 1 is faster in pad1031).

## aes_gcm

Known-answer checks: pad0 27/27, pad1031 27/27. Runs: 3 and 3. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 0.998 | 0.969 to 1.015 |
| libcrux | short | 24 | 0.961 | 0.645 to 1.279 |
| rind | long | 18 | 1.000 | 0.992 to 1.010 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 36 | 1.004 | 0.989 to 1.012 |
| ring | long | 12 | 0.994 | 0.985 to 0.999 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 0.921 | 0.842 to 0.988 |
| rustcrypto | long | 12 | 0.996 | 0.959 to 1.028 |
| rustcrypto | short | 24 | 1.030 | 0.787 to 1.261 |

84 of 171 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `aes256-gcm/libcrux/32B` | 149,902 | 96,636 | 0.645 |
| `aes256-gcm/libcrux/quic-32B` | 148,002 | 97,575 | 0.659 |
| `aes128-gcm/libcrux/quic-32B` | 69,253 | 88,591 | 1.279 |
| `aes256-gcm/libcrux/quic-64B` | 186,418 | 135,617 | 0.727 |
| `aes128-gcm/libcrux/32B` | 76,964 | 97,051 | 1.261 |
| `aes128-gcm-open/rustcrypto/32B` | 52,986 | 66,802 | 1.261 |
| `aes128-gcm-open/rustcrypto/quic-32B` | 58,274 | 73,230 | 1.257 |
| `aes256-gcm-open/libcrux/32B` | 194,500 | 151,968 | 0.781 |
| `aes128-gcm/rustcrypto/quic-32B` | 54,483 | 42,877 | 0.787 |
| `aes256-gcm-open/libcrux/quic-32B` | 191,332 | 151,728 | 0.793 |
| `aes256-gcm/libcrux/quic-128B` | 265,800 | 213,028 | 0.801 |
| `aes128-gcm/libcrux/quic-64B` | 100,294 | 119,512 | 1.192 |
| `aes256-gcm-open/rustcrypto/32B` | 98,296 | 80,224 | 0.816 |
| `aes256-gcm-open/libcrux/quic-64B` | 227,050 | 190,220 | 0.838 |
| `aes128-gcm/ring/quic-32B` | 41,131 | 34,618 | 0.842 |
| `aes128-gcm/ring/32B` | 43,078 | 36,482 | 0.847 |
| `aes256-gcm-open/libcrux/quic-128B` | 305,644 | 261,840 | 0.857 |
| `aes128-gcm-open/ring/quic-32B` | 30,375 | 26,242 | 0.864 |
| `aes128-gcm-open/ring/32B` | 32,302 | 28,170 | 0.872 |
| `aes128-gcm/ring/quic-64B` | 51,972 | 45,436 | 0.874 |
| `aes256-gcm-open/ring/quic-32B` | 37,108 | 32,971 | 0.889 |
| `aes128-gcm/rustcrypto/quic-64B` | 48,419 | 53,681 | 1.109 |
| `aes256-gcm-open/rustcrypto/quic-32B` | 97,852 | 87,220 | 0.891 |
| `aes128-gcm/libcrux/quic-128B` | 162,678 | 180,178 | 1.108 |
| `aes256-gcm-open/ring/32B` | 39,034 | 34,899 | 0.894 |
| `aes256-gcm/rustcrypto/32B` | 62,257 | 55,785 | 0.896 |
| `aes128-gcm-open/ring/quic-64B` | 41,147 | 37,071 | 0.901 |
| `aes256-gcm/ring/quic-32B` | 43,423 | 39,127 | 0.901 |
| `aes256-gcm/libcrux/quic-256B` | 411,612 | 371,454 | 0.902 |
| `aes256-gcm-open/rustcrypto/quic-64B` | 93,632 | 102,397 | 1.094 |
| `aes256-gcm/ring/32B` | 45,347 | 41,137 | 0.907 |
| `aes128-gcm/ring/quic-128B` | 73,560 | 67,335 | 0.915 |
| `aes128-gcm-open/rustcrypto/quic-64B` | 62,241 | 67,262 | 1.081 |
| `aes256-gcm-open/ring/quic-64B` | 50,612 | 46,532 | 0.919 |
| `aes256-gcm-open/libcrux/quic-256B` | 448,441 | 413,343 | 0.922 |
| `aes256-gcm/ring/quic-64B` | 57,153 | 52,788 | 0.924 |
| `aes128-gcm/rustcrypto/quic-128B` | 69,944 | 75,232 | 1.076 |
| `aes256-gcm/rustcrypto/quic-32B` | 67,587 | 62,943 | 0.931 |
| `aes256-gcm-open/rustcrypto/quic-128B` | 120,403 | 128,500 | 1.067 |
| `aes128-gcm-open/ring/quic-128B` | 62,809 | 58,729 | 0.935 |
| `aes128-gcm-open/rustcrypto/quic-128B` | 83,967 | 89,215 | 1.062 |
| `aes128-gcm/libcrux/quic-256B` | 288,998 | 305,778 | 1.058 |
| `aes256-gcm/libcrux/quic-512B` | 694,335 | 654,423 | 0.943 |
| `aes256-gcm-open/ring/quic-128B` | 77,738 | 73,656 | 0.947 |
| `aes128-gcm/ring/quic-256B` | 116,797 | 110,918 | 0.950 |
| `aes128-gcm/rustcrypto/quic-256B` | 113,414 | 119,023 | 1.049 |
| `aes128-gcm-open/rustcrypto/quic-256B` | 127,310 | 133,535 | 1.049 |
| `aes256-gcm-open/libcrux/quic-512B` | 733,804 | 698,820 | 0.952 |
| `aes256-gcm/ring/quic-128B` | 84,075 | 80,151 | 0.953 |
| `aes256-gcm/rustcrypto/1200B` | 558,157 | 535,351 | 0.959 |
| `aes128-gcm-open/ring/quic-256B` | 106,123 | 102,048 | 0.962 |
| `aes256-gcm/rustcrypto/1350B` | 627,538 | 606,688 | 0.967 |
| `aes256-gcm-open/rustcrypto/quic-256B` | 173,941 | 179,368 | 1.031 |
| `aes128-gcm/rustcrypto/32B` | 40,191 | 41,441 | 1.031 |
| `aes256-gcm-open/ring/quic-256B` | 131,985 | 127,904 | 0.969 |
| `aes128-gcm-open/libcrux/quic-32B` | 96,620 | 93,645 | 0.969 |
| `aes256-gcm/libcrux/1200B` | 1,458,584 | 1,413,756 | 0.969 |
| `aes128-gcm-open/rustcrypto/quic-512B` | 214,416 | 220,675 | 1.029 |
| `aes128-gcm-open/rustcrypto/1200B` | 468,427 | 481,553 | 1.028 |
| `aes128-gcm/rustcrypto/quic-512B` | 199,714 | 205,222 | 1.028 |
| `aes256-gcm/rustcrypto/quic-512B` | 257,939 | 250,960 | 0.973 |
| `aes128-gcm-open/libcrux/32B` | 105,114 | 102,294 | 0.973 |
| `aes128-gcm/libcrux/quic-512B` | 534,149 | 547,935 | 1.026 |
| `aes256-gcm-open/libcrux/1200B` | 1,506,472 | 1,468,149 | 0.975 |
| `aes256-gcm/ring/quic-256B` | 138,727 | 135,290 | 0.975 |
| `aes128-gcm/ring/quic-512B` | 203,272 | 198,440 | 0.976 |
| `aes256-gcm/libcrux/1350B` | 1,656,747 | 1,618,398 | 0.977 |
| `aes128-gcm-open/libcrux/quic-64B` | 127,600 | 124,778 | 0.978 |
| `aes128-gcm-open/ring/quic-512B` | 192,763 | 188,684 | 0.979 |
| `aes256-gcm-open/ring/quic-512B` | 240,475 | 236,392 | 0.983 |
| `aes128-gcm/rustcrypto/1200B` | 448,639 | 456,167 | 1.017 |
| `aes128-gcm-open/libcrux/quic-128B` | 189,806 | 186,797 | 0.984 |
| `aes128-gcm-open/ring/1350B` | 490,471 | 482,886 | 0.985 |
| `aes128-gcm/libcrux/1200B` | 1,198,385 | 1,216,665 | 1.015 |
| `aes256-gcm/rustcrypto/quic-128B` | 99,638 | 101,124 | 1.015 |
| `aes256-gcm/ring/quic-512B` | 247,112 | 244,044 | 0.988 |
| `aes128-gcm-open/rind/quic-512B` | 160,238 | 162,156 | 1.012 |
| `aes256-gcm-open/ring/1350B` | 612,162 | 605,006 | 0.988 |
| `aes256-gcm-open/rustcrypto/1200B` | 585,182 | 578,675 | 0.989 |
| `aes128-gcm/libcrux/1350B` | 1,355,285 | 1,370,244 | 1.011 |
| `aes192-gcm/rind/quic-512B` | 181,512 | 179,584 | 0.989 |
| `aes128-gcm-open/rind/1200B` | 365,063 | 368,894 | 1.010 |
| `aes192-gcm-open/rind/quic-512B` | 178,174 | 180,020 | 1.010 |
| `aes256-gcm/rind/quic-512B` | 199,544 | 197,508 | 0.990 |

## chacha20poly1305

Known-answer checks: pad0 21/21, pad1031 21/21. Runs: 3 and 3. Rows: 56 in both.

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

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305/libcrux/1350B` | 170,595 | 163,846 | 0.960 |
| `chacha20poly1305/libcrux/quic-256B` | 45,961 | 44,497 | 0.968 |
| `chacha20poly1305/libcrux/quic-128B` | 32,454 | 31,428 | 0.968 |
| `chacha20poly1305/libcrux/quic-512B` | 72,652 | 70,427 | 0.969 |
| `chacha20poly1305/libcrux/1200B` | 131,504 | 128,801 | 0.979 |
| `chacha20poly1305/libcrux/32B` | 12,097 | 11,894 | 0.983 |
| `chacha20poly1305-open/libcrux/1350B` | 200,985 | 198,964 | 0.990 |

## cipher

Known-answer checks: pad0 48/48, pad1031 48/48. Runs: 3 and 3. Rows: 42 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| aes128 | per op | 2 | 1.000 | 1.000 to 1.000 |
| aes256 | per op | 2 | 1.000 | 1.000 to 1.000 |
| chacha20 | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | long | 27 | 1.000 | 1.000 to 1.000 |
| rind | short | 9 | 1.000 | 1.000 to 1.000 |

0 of 42 rows moved by more than max(1%, run spread).

## hkdf

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 0.998 to 1.080 |
| expand-aes-256-gcm-key | per op | 8 | 1.005 | 0.998 to 1.014 |
| extract-expand | per op | 15 | 1.000 | 0.986 to 1.069 |

7 of 38 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `hkdf-sha256/expand/libcrux` | 107,026 | 115,535 | 1.080 |
| `hkdf-sha256/extract-expand/rind` | 304,940 | 326,031 | 1.069 |
| `hkdf-sha256/expand/rind` | 100,705 | 102,874 | 1.022 |
| `hkdf-sha256/expand/rustcrypto` | 24,312 | 24,683 | 1.015 |
| `hkdf-sha256/extract-expand/libcrux` | 211,040 | 208,039 | 0.986 |
| `hkdf-sha256/expand-aes-256-gcm-key/rind` | 381,328 | 386,572 | 1.014 |
| `hkdf-sha1/extract-expand/rustcrypto` | 29,892 | 29,571 | 0.989 |

## hmac

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 0.998 | 0.957 to 0.999 |
| libcrux | short | 4 | 0.973 | 0.874 to 1.000 |
| rind | long | 24 | 1.000 | 0.999 to 1.047 |
| rind | short | 12 | 1.001 | 1.000 to 1.077 |
| ring | long | 24 | 1.000 | 1.000 to 1.000 |
| ring | short | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.989 to 1.000 |
| rustcrypto | short | 6 | 1.000 | 0.990 to 1.000 |

12 of 102 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `hmac-sha256/libcrux/32B` | 36,874 | 32,221 | 0.874 |
| `hmac-sha256/rind/reused-key/32B` | 34,931 | 37,620 | 1.077 |
| `hmac-sha256/rind/32B` | 68,683 | 73,735 | 1.074 |
| `hmac-sha256/rind/64B` | 74,147 | 79,061 | 1.066 |
| `hmac-sha256/rind/reused-key/64B` | 40,044 | 42,308 | 1.057 |
| `hmac-sha256/rind/1200B` | 160,736 | 168,225 | 1.047 |
| `hmac-sha256/libcrux/1200B` | 130,943 | 125,292 | 0.957 |
| `hmac-sha256/rind/1350B` | 179,030 | 183,084 | 1.023 |
| `hmac-sha256/rind/reused-key/8KB` | 669,665 | 677,765 | 1.012 |
| `hmac-sha256/rustcrypto/16KB` | 1,420,045 | 1,404,300 | 0.989 |
| `hmac-sha256/rustcrypto/8KB` | 739,549 | 731,730 | 0.989 |
| `hmac-sha256/rustcrypto/32B` | 42,476 | 42,050 | 0.990 |

## sha2

Known-answer checks: pad0 39/39, pad1031 39/39. Runs: 3 and 3. Rows: 74 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 8 | 1.000 | 0.997 to 1.001 |
| libcrux | short | 4 | 0.999 | 0.997 to 1.002 |
| rind | long | 12 | 1.000 | 1.000 to 1.102 |
| rind | short | 8 | 1.008 | 1.000 to 1.630 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.989 to 1.000 |
| rustcrypto | short | 6 | 0.999 | 0.990 to 1.000 |

8 of 74 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `sha256/rind/32B` | 19,871 | 32,387 | 1.630 |
| `sha256/rind/64B` | 24,875 | 37,202 | 1.496 |
| `sha256/rind/1200B` | 114,294 | 125,916 | 1.102 |
| `sha256/rind/1350B` | 129,167 | 140,683 | 1.089 |
| `sha256-updates/rind/16B` | 641,418 | 654,475 | 1.020 |
| `sha256-updates/rind/1B` | 933,921 | 946,768 | 1.014 |
| `sha256/rind/8KB` | 663,026 | 671,783 | 1.013 |
| `sha256/rustcrypto/1200B` | 102,135 | 100,981 | 0.989 |

## xchacha20poly1305

Known-answer checks: pad0 21/21, pad1031 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.000 | 0.997 to 1.000 |
| libcrux | short | 4 | 0.996 | 0.996 to 1.000 |
| rind | long | 3 | 1.010 | 1.002 to 1.010 |
| rind | short | 4 | 1.013 | 1.011 to 1.057 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

6 of 21 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rind/32B` | 288,632 | 305,061 | 1.057 |
| `xchacha20poly1305/rind/quic-128B` | 320,984 | 325,119 | 1.013 |
| `xchacha20poly1305/rind/quic-256B` | 333,500 | 337,623 | 1.012 |
| `xchacha20poly1305/rind/quic-512B` | 358,520 | 362,640 | 1.011 |
| `xchacha20poly1305/rind/1200B` | 428,783 | 433,281 | 1.010 |
| `xchacha20poly1305/rind/1350B` | 445,838 | 450,390 | 1.010 |

