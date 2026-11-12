Cycles, pad1031 over pad0 (below 1 is faster in pad1031).

## aes_gcm

Known-answer checks: pad0 27/27, pad1031 27/27. Runs: 3 and 3. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 1.000 | 0.995 to 1.007 |
| libcrux | short | 24 | 0.994 | 0.970 to 1.009 |
| rind | long | 18 | 1.000 | 0.990 to 1.009 |
| rind | per op | 3 | 1.000 | 1.000 to 1.002 |
| rind | short | 36 | 1.004 | 0.990 to 1.011 |
| ring | long | 12 | 0.999 | 0.985 to 1.010 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 0.959 | 0.864 to 1.007 |
| rustcrypto | long | 12 | 0.994 | 0.964 to 1.012 |
| rustcrypto | short | 24 | 1.043 | 0.781 to 1.182 |

65 of 171 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `aes128-gcm/rustcrypto/quic-32B` | 54,483 | 42,576 | 0.781 |
| `aes128-gcm-open/rustcrypto/quic-32B` | 58,274 | 68,902 | 1.182 |
| `aes256-gcm-open/rustcrypto/32B` | 98,296 | 81,997 | 0.834 |
| `aes128-gcm-open/ring/quic-32B` | 30,375 | 26,242 | 0.864 |
| `aes128-gcm-open/ring/32B` | 32,302 | 28,170 | 0.872 |
| `aes256-gcm-open/rustcrypto/quic-64B` | 93,632 | 105,012 | 1.122 |
| `aes256-gcm-open/ring/quic-32B` | 37,108 | 32,971 | 0.889 |
| `aes256-gcm-open/ring/32B` | 39,034 | 34,899 | 0.894 |
| `aes128-gcm/rustcrypto/quic-64B` | 48,419 | 53,344 | 1.102 |
| `aes128-gcm-open/ring/quic-64B` | 41,147 | 37,071 | 0.901 |
| `aes256-gcm/rustcrypto/32B` | 62,257 | 56,330 | 0.905 |
| `aes128-gcm-open/rustcrypto/32B` | 52,986 | 57,886 | 1.092 |
| `aes256-gcm-open/rustcrypto/quic-128B` | 120,403 | 131,145 | 1.089 |
| `aes256-gcm/ring/quic-32B` | 43,423 | 39,637 | 0.913 |
| `aes256-gcm/ring/32B` | 45,347 | 41,602 | 0.917 |
| `aes256-gcm-open/ring/quic-64B` | 50,612 | 46,532 | 0.919 |
| `aes128-gcm-open/rustcrypto/quic-64B` | 62,241 | 67,221 | 1.080 |
| `aes256-gcm-open/rustcrypto/quic-32B` | 97,852 | 90,064 | 0.920 |
| `aes128-gcm/rustcrypto/quic-128B` | 69,944 | 75,097 | 1.074 |
| `aes256-gcm/rustcrypto/quic-128B` | 99,638 | 106,466 | 1.069 |
| `aes128-gcm-open/ring/quic-128B` | 62,809 | 58,729 | 0.935 |
| `aes256-gcm/ring/quic-64B` | 57,153 | 53,458 | 0.935 |
| `aes256-gcm/rustcrypto/quic-32B` | 67,587 | 63,473 | 0.939 |
| `aes128-gcm-open/rustcrypto/quic-128B` | 83,967 | 88,917 | 1.059 |
| `aes256-gcm/rustcrypto/quic-64B` | 73,948 | 78,253 | 1.058 |
| `aes256-gcm-open/ring/quic-128B` | 77,738 | 73,656 | 0.947 |
| `aes128-gcm/rustcrypto/quic-256B` | 113,414 | 118,785 | 1.047 |
| `aes256-gcm-open/rustcrypto/quic-256B` | 173,941 | 182,103 | 1.047 |
| `aes128-gcm/ring/quic-32B` | 41,131 | 39,392 | 0.958 |
| `aes128-gcm/ring/32B` | 43,078 | 41,326 | 0.959 |
| `aes128-gcm-open/rustcrypto/quic-256B` | 127,310 | 132,408 | 1.040 |
| `aes128-gcm-open/ring/quic-256B` | 106,123 | 102,048 | 0.962 |
| `aes256-gcm/ring/quic-128B` | 84,075 | 80,870 | 0.962 |
| `aes256-gcm/rustcrypto/1200B` | 558,157 | 538,311 | 0.964 |
| `aes256-gcm/rustcrypto/1350B` | 627,538 | 605,319 | 0.965 |
| `aes128-gcm/ring/quic-64B` | 51,972 | 50,357 | 0.969 |
| `aes256-gcm-open/ring/quic-256B` | 131,985 | 127,904 | 0.969 |
| `aes256-gcm-open/libcrux/32B` | 194,500 | 188,704 | 0.970 |
| `aes128-gcm/rustcrypto/1350B` | 523,583 | 508,596 | 0.971 |
| `aes128-gcm-open/rustcrypto/quic-512B` | 214,416 | 220,550 | 1.029 |
| `aes128-gcm/libcrux/quic-32B` | 69,253 | 67,279 | 0.971 |
| `aes256-gcm/rustcrypto/quic-256B` | 151,276 | 155,502 | 1.028 |
| `aes128-gcm/rustcrypto/quic-512B` | 199,714 | 205,202 | 1.027 |
| `aes256-gcm-open/rustcrypto/1200B` | 585,182 | 570,903 | 0.976 |
| `aes128-gcm-open/ring/quic-512B` | 192,763 | 188,684 | 0.979 |
| `aes256-gcm/libcrux/32B` | 149,902 | 146,757 | 0.979 |
| `aes128-gcm/libcrux/quic-64B` | 100,294 | 98,323 | 0.980 |
| `aes256-gcm-open/rustcrypto/quic-512B` | 280,261 | 285,639 | 1.019 |
| `aes256-gcm-open/ring/quic-512B` | 240,475 | 236,392 | 0.983 |
| `aes256-gcm/rustcrypto/quic-512B` | 257,939 | 262,206 | 1.017 |
| `aes128-gcm/libcrux/32B` | 76,964 | 75,738 | 0.984 |
| `aes256-gcm/ring/quic-256B` | 138,727 | 136,523 | 0.984 |
| `aes128-gcm-open/libcrux/quic-32B` | 96,620 | 95,101 | 0.984 |
| `aes128-gcm/ring/quic-128B` | 73,560 | 72,413 | 0.984 |
| `aes128-gcm-open/ring/1350B` | 490,471 | 482,886 | 0.985 |
| `aes128-gcm/libcrux/quic-256B` | 288,998 | 284,589 | 0.985 |
| `aes128-gcm/libcrux/quic-128B` | 162,678 | 160,412 | 0.986 |
| `aes128-gcm/libcrux/quic-512B` | 534,149 | 526,743 | 0.986 |
| `aes128-gcm-open/rustcrypto/1200B` | 468,427 | 474,043 | 1.012 |
| `aes128-gcm/rustcrypto/32B` | 40,191 | 40,671 | 1.012 |
| `aes256-gcm-open/ring/1350B` | 612,162 | 605,014 | 0.988 |
| `aes128-gcm-open/rind/quic-512B` | 160,238 | 161,957 | 1.011 |
| `aes128-gcm/ring/1200B` | 441,882 | 446,505 | 1.010 |
| `aes192-gcm/rind/quic-512B` | 181,512 | 179,621 | 0.990 |
| `aes256-gcm-open/rustcrypto/1350B` | 656,324 | 649,678 | 0.990 |

## chacha20poly1305

Known-answer checks: pad0 21/21, pad1031 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 0.975 | 0.947 to 0.997 |
| libcrux | short | 8 | 0.941 | 0.852 to 0.984 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

12 of 56 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/libcrux/quic-128B` | 64,335 | 54,840 | 0.852 |
| `chacha20poly1305-open/libcrux/32B` | 34,045 | 29,505 | 0.867 |
| `chacha20poly1305-open/libcrux/quic-256B` | 77,614 | 68,247 | 0.879 |
| `chacha20poly1305-open/libcrux/quic-512B` | 104,744 | 95,062 | 0.908 |
| `chacha20poly1305-open/libcrux/1350B` | 200,985 | 190,274 | 0.947 |
| `chacha20poly1305/libcrux/1350B` | 170,595 | 164,119 | 0.962 |
| `chacha20poly1305-open/libcrux/1200B` | 153,397 | 148,792 | 0.970 |
| `chacha20poly1305/libcrux/quic-512B` | 72,652 | 70,846 | 0.975 |
| `chacha20poly1305/libcrux/quic-256B` | 45,961 | 44,819 | 0.975 |
| `chacha20poly1305/libcrux/1200B` | 131,504 | 128,801 | 0.979 |
| `chacha20poly1305/libcrux/quic-128B` | 32,454 | 31,824 | 0.981 |
| `chacha20poly1305/libcrux/32B` | 12,097 | 11,899 | 0.984 |

## xchacha20poly1305

Known-answer checks: pad0 21/21, pad1031 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.011 | 1.003 to 1.014 |
| libcrux | short | 4 | 1.013 | 1.013 to 1.020 |
| rind | long | 3 | 1.011 | 1.003 to 1.013 |
| rind | short | 4 | 1.017 | 1.016 to 1.058 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

12 of 21 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rind/32B` | 288,632 | 305,509 | 1.058 |
| `xchacha20poly1305/libcrux/32B` | 302,388 | 308,461 | 1.020 |
| `xchacha20poly1305/rind/quic-128B` | 320,984 | 326,707 | 1.018 |
| `xchacha20poly1305/rind/quic-256B` | 333,500 | 339,223 | 1.017 |
| `xchacha20poly1305/rind/quic-512B` | 358,520 | 364,249 | 1.016 |
| `xchacha20poly1305/libcrux/1200B` | 418,971 | 425,014 | 1.014 |
| `xchacha20poly1305/libcrux/quic-128B` | 350,464 | 355,189 | 1.013 |
| `xchacha20poly1305/libcrux/quic-256B` | 363,322 | 368,074 | 1.013 |
| `xchacha20poly1305/libcrux/quic-512B` | 389,191 | 394,222 | 1.013 |
| `xchacha20poly1305/rind/1200B` | 428,783 | 434,215 | 1.013 |
| `xchacha20poly1305/rind/1350B` | 445,838 | 450,793 | 1.011 |
| `xchacha20poly1305/libcrux/1350B` | 478,861 | 484,081 | 1.011 |

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

## sha2

Known-answer checks: pad0 39/39, pad1031 39/39. Runs: 3 and 3. Rows: 74 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 8 | 1.000 | 0.996 to 1.000 |
| libcrux | short | 4 | 0.999 | 0.992 to 1.003 |
| rind | long | 12 | 1.000 | 1.000 to 1.039 |
| rind | short | 8 | 1.007 | 1.000 to 1.228 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.989 to 1.000 |
| rustcrypto | short | 6 | 0.997 | 0.991 to 0.999 |

7 of 74 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `sha256/rind/32B` | 19,871 | 24,406 | 1.228 |
| `sha256/rind/64B` | 24,875 | 29,319 | 1.179 |
| `sha256/rind/1200B` | 114,294 | 118,713 | 1.039 |
| `sha256/rind/1350B` | 129,167 | 133,556 | 1.034 |
| `sha256-updates/rind/16B` | 641,418 | 652,704 | 1.018 |
| `sha256-updates/rind/1B` | 933,921 | 944,706 | 1.012 |
| `sha256/rustcrypto/1200B` | 102,135 | 101,001 | 0.989 |

## sha3

Known-answer checks: pad0 45/45, pad1031 45/45. Runs: 3 and 3. Rows: 54 in both.

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

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 0.996 | 0.978 to 0.999 |
| libcrux | short | 4 | 0.986 | 0.972 to 0.999 |
| rind | long | 24 | 1.000 | 0.999 to 1.048 |
| rind | short | 12 | 1.000 | 1.000 to 1.195 |
| ring | long | 24 | 1.000 | 1.000 to 1.000 |
| ring | short | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.980 to 1.001 |
| rustcrypto | short | 6 | 1.000 | 0.979 to 1.001 |

20 of 102 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `hmac-sha256/rind/reused-key/32B` | 34,931 | 41,725 | 1.195 |
| `hmac-sha256/rind/reused-key/64B` | 40,044 | 46,506 | 1.161 |
| `hmac-sha256/rind/32B` | 68,683 | 74,148 | 1.080 |
| `hmac-sha256/rind/64B` | 74,147 | 78,912 | 1.064 |
| `hmac-sha256/rind/reused-key/1200B` | 121,407 | 127,238 | 1.048 |
| `hmac-sha256/rind/reused-key/1350B` | 136,248 | 142,079 | 1.043 |
| `hmac-sha256/rind/1200B` | 160,736 | 166,871 | 1.038 |
| `hmac-sha256/rind/1350B` | 179,030 | 185,661 | 1.037 |
| `hmac-sha256/libcrux/64B` | 42,174 | 40,990 | 0.972 |
| `hmac-sha256/libcrux/32B` | 36,874 | 35,858 | 0.972 |
| `hmac-sha256/libcrux/1200B` | 130,943 | 128,018 | 0.978 |
| `hmac-sha256/rustcrypto/32B` | 42,476 | 41,577 | 0.979 |
| `hmac-sha256/libcrux/1350B` | 145,787 | 142,862 | 0.980 |
| `hmac-sha256/rustcrypto/64B` | 48,046 | 47,104 | 0.980 |
| `hmac-sha256/rustcrypto/1200B` | 142,610 | 139,825 | 0.980 |
| `hmac-sha256/rustcrypto/1350B` | 158,286 | 155,501 | 0.982 |
| `hmac-sha256/rustcrypto/8KB` | 739,549 | 726,876 | 0.983 |
| `hmac-sha256/rustcrypto/16KB` | 1,420,045 | 1,402,605 | 0.988 |
| `hmac-sha256/libcrux/8KB` | 688,897 | 680,689 | 0.988 |
| `hmac-sha256/rind/8KB` | 719,383 | 727,503 | 1.011 |

## hkdf

Known-answer checks: pad0 12/12, pad1031 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 0.997 to 1.021 |
| expand-aes-256-gcm-key | per op | 8 | 1.000 | 0.994 to 1.018 |
| extract-expand | per op | 15 | 0.999 | 0.922 to 1.057 |

7 of 38 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1031 cycles | B/A |
|---|---:|---:|---:|
| `hkdf-sha256/extract-expand/libcrux` | 211,040 | 194,602 | 0.922 |
| `hkdf-sha256/extract-expand/rind` | 304,940 | 322,310 | 1.057 |
| `hkdf-sha256/expand/libcrux` | 107,026 | 109,267 | 1.021 |
| `hkdf-sha256/expand/rustcrypto` | 24,312 | 24,799 | 1.020 |
| `hkdf-sha256/expand-aes-256-gcm-key/rind` | 381,328 | 388,250 | 1.018 |
| `hkdf-sha1/extract-expand/rustcrypto` | 29,892 | 29,434 | 0.985 |
| `hkdf-sha256/expand/rind` | 100,705 | 102,115 | 1.014 |

