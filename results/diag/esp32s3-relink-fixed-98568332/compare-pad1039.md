Cycles, pad1039 over pad0 (below 1 is faster in pad1039).

## aes_gcm

Known-answer checks: pad0 27/27, pad1039 27/27. Runs: 3 and 3. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 1.000 | 0.994 to 1.003 |
| libcrux | short | 24 | 0.996 | 0.969 to 1.027 |
| rind | long | 18 | 1.000 | 1.000 to 1.022 |
| rind | per op | 3 | 1.000 | 1.000 to 1.002 |
| rind | short | 36 | 1.008 | 0.990 to 1.025 |
| ring | long | 12 | 0.999 | 0.985 to 1.005 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 0.965 | 0.864 to 1.005 |
| rustcrypto | long | 12 | 0.997 | 0.971 to 1.014 |
| rustcrypto | short | 24 | 1.048 | 0.794 to 1.201 |

76 of 171 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1039 cycles | B/A |
|---|---:|---:|---:|
| `aes128-gcm/rustcrypto/quic-32B` | 54,483 | 43,261 | 0.794 |
| `aes128-gcm-open/rustcrypto/quic-32B` | 58,274 | 69,997 | 1.201 |
| `aes256-gcm-open/rustcrypto/32B` | 98,296 | 79,959 | 0.813 |
| `aes128-gcm-open/ring/quic-32B` | 30,375 | 26,242 | 0.864 |
| `aes128-gcm-open/ring/32B` | 32,302 | 28,170 | 0.872 |
| `aes128-gcm/rustcrypto/quic-64B` | 48,419 | 54,237 | 1.120 |
| `aes128-gcm-open/rustcrypto/32B` | 52,986 | 59,086 | 1.115 |
| `aes256-gcm-open/ring/quic-32B` | 37,108 | 32,971 | 0.889 |
| `aes256-gcm-open/ring/32B` | 39,034 | 34,899 | 0.894 |
| `aes128-gcm-open/ring/quic-64B` | 41,147 | 37,071 | 0.901 |
| `aes128-gcm/rustcrypto/quic-128B` | 69,944 | 76,691 | 1.096 |
| `aes128-gcm-open/rustcrypto/quic-64B` | 62,241 | 68,097 | 1.094 |
| `aes256-gcm-open/rustcrypto/quic-64B` | 93,632 | 101,800 | 1.087 |
| `aes256-gcm/rustcrypto/32B` | 62,257 | 56,910 | 0.914 |
| `aes256-gcm-open/rustcrypto/quic-32B` | 97,852 | 89,767 | 0.917 |
| `aes256-gcm-open/ring/quic-64B` | 50,612 | 46,532 | 0.919 |
| `aes128-gcm/ring/quic-64B` | 51,972 | 47,929 | 0.922 |
| `aes128-gcm-open/rustcrypto/quic-128B` | 83,967 | 90,068 | 1.073 |
| `aes128-gcm/rustcrypto/quic-256B` | 113,414 | 121,317 | 1.070 |
| `aes128-gcm-open/ring/quic-128B` | 62,809 | 58,729 | 0.935 |
| `aes256-gcm-open/rustcrypto/quic-128B` | 120,403 | 128,207 | 1.065 |
| `aes256-gcm/rustcrypto/quic-128B` | 99,638 | 105,535 | 1.059 |
| `aes128-gcm/ring/quic-32B` | 41,131 | 38,872 | 0.945 |
| `aes128-gcm-open/rustcrypto/quic-256B` | 127,310 | 134,229 | 1.054 |
| `aes256-gcm-open/ring/quic-128B` | 77,738 | 73,656 | 0.947 |
| `aes128-gcm/ring/32B` | 43,078 | 40,830 | 0.948 |
| `aes256-gcm/rustcrypto/quic-64B` | 73,948 | 77,599 | 1.049 |
| `aes128-gcm/rustcrypto/quic-512B` | 199,714 | 209,075 | 1.047 |
| `aes128-gcm-open/ring/quic-256B` | 106,123 | 102,048 | 0.962 |
| `aes256-gcm-open/rustcrypto/quic-256B` | 173,941 | 180,076 | 1.035 |
| `aes128-gcm-open/rustcrypto/quic-512B` | 214,416 | 221,358 | 1.032 |
| `aes256-gcm-open/ring/quic-256B` | 131,985 | 127,904 | 0.969 |
| `aes256-gcm-open/libcrux/32B` | 194,500 | 188,512 | 0.969 |
| `aes256-gcm/rustcrypto/1200B` | 558,157 | 541,809 | 0.971 |
| `aes128-gcm/libcrux/32B` | 76,964 | 79,023 | 1.027 |
| `aes128-gcm/ring/quic-128B` | 73,560 | 71,653 | 0.974 |
| `aes256-gcm-open/rustcrypto/1200B` | 585,182 | 570,099 | 0.974 |
| `aes256-gcm/libcrux/32B` | 149,902 | 146,045 | 0.974 |
| `aes128-gcm-open/rind/quic-512B` | 160,238 | 164,266 | 1.025 |
| `aes128-gcm/rustcrypto/32B` | 40,191 | 41,086 | 1.022 |
| `aes256-gcm/rustcrypto/1350B` | 627,538 | 613,579 | 0.978 |
| `aes128-gcm/libcrux/quic-32B` | 69,253 | 70,794 | 1.022 |
| `aes128-gcm-open/rind/1200B` | 365,063 | 373,117 | 1.022 |
| `aes192-gcm-open/rind/quic-512B` | 178,174 | 182,064 | 1.022 |
| `aes128-gcm-open/ring/quic-512B` | 192,763 | 188,684 | 0.979 |
| `aes256-gcm-open/rind/quic-512B` | 196,110 | 200,138 | 1.021 |
| `aes256-gcm/rustcrypto/quic-256B` | 151,276 | 154,269 | 1.020 |
| `aes256-gcm-open/rind/quic-256B` | 104,497 | 106,512 | 1.019 |
| `aes192-gcm-open/rind/1200B` | 406,187 | 413,968 | 1.019 |
| `aes128-gcm-open/rind/quic-256B` | 85,487 | 87,098 | 1.019 |
| `aes192-gcm-open/rind/quic-128B` | 53,403 | 54,396 | 1.019 |
| `aes128-gcm-open/rind/quic-128B` | 48,114 | 49,009 | 1.019 |
| `aes256-gcm-open/rind/1200B` | 447,311 | 455,368 | 1.018 |
| `aes192-gcm-open/rind/quic-64B` | 32,608 | 33,179 | 1.018 |
| `aes256-gcm-open/rind/32B` | 25,640 | 26,088 | 1.017 |
| `aes128-gcm-open/rind/quic-32B` | 20,084 | 20,435 | 1.017 |
| `aes256-gcm-open/rind/quic-32B` | 24,337 | 24,761 | 1.017 |
| `aes256-gcm-open/rind/quic-64B` | 35,788 | 36,408 | 1.017 |
| `aes192-gcm-open/rind/quic-32B` | 22,210 | 22,592 | 1.017 |
| `aes256-gcm-open/rind/quic-128B` | 58,691 | 59,698 | 1.017 |
| `aes128-gcm-open/rind/32B` | 21,387 | 21,754 | 1.017 |
| `aes128-gcm-open/rind/quic-64B` | 29,427 | 29,931 | 1.017 |
| `aes192-gcm-open/rind/32B` | 23,514 | 23,914 | 1.017 |
| `aes256-gcm-open/ring/quic-512B` | 240,475 | 236,392 | 0.983 |
| `aes256-gcm-open/rustcrypto/quic-512B` | 280,261 | 284,994 | 1.017 |
| `aes192-gcm-open/rind/quic-256B` | 94,991 | 96,565 | 1.017 |
| `aes256-gcm-open/libcrux/quic-128B` | 305,644 | 300,757 | 0.984 |
| `aes256-gcm-open/rustcrypto/1350B` | 656,324 | 646,053 | 0.984 |
| `aes128-gcm-open/ring/1350B` | 490,471 | 482,886 | 0.985 |
| `aes128-gcm-open/rustcrypto/1200B` | 468,427 | 475,120 | 1.014 |
| `aes128-gcm/ring/quic-256B` | 116,797 | 115,241 | 0.987 |
| `aes128-gcm/ring/quic-512B` | 203,272 | 200,740 | 0.988 |
| `aes256-gcm-open/ring/1350B` | 612,162 | 605,011 | 0.988 |
| `aes256-gcm-open/libcrux/quic-256B` | 448,441 | 443,555 | 0.989 |
| `aes256-gcm-open/libcrux/quic-32B` | 191,332 | 189,256 | 0.989 |
| `aes192-gcm/rind/quic-512B` | 181,512 | 179,666 | 0.990 |

## chacha20poly1305

Known-answer checks: pad0 21/21, pad1039 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 0.987 | 0.963 to 0.999 |
| libcrux | short | 8 | 0.960 | 0.914 to 0.996 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

10 of 56 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1039 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/libcrux/quic-128B` | 64,335 | 58,803 | 0.914 |
| `chacha20poly1305-open/libcrux/quic-256B` | 77,614 | 72,195 | 0.930 |
| `chacha20poly1305-open/libcrux/quic-512B` | 104,744 | 98,986 | 0.945 |
| `chacha20poly1305/libcrux/quic-128B` | 32,454 | 31,165 | 0.960 |
| `chacha20poly1305/libcrux/quic-256B` | 45,961 | 44,136 | 0.960 |
| `chacha20poly1305/libcrux/quic-512B` | 72,652 | 69,915 | 0.962 |
| `chacha20poly1305/libcrux/1350B` | 170,595 | 164,289 | 0.963 |
| `chacha20poly1305-open/libcrux/1350B` | 200,985 | 196,456 | 0.977 |
| `chacha20poly1305/libcrux/1200B` | 131,504 | 128,801 | 0.979 |
| `chacha20poly1305/libcrux/32B` | 12,097 | 11,899 | 0.984 |

## xchacha20poly1305

Known-answer checks: pad0 21/21, pad1039 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 0.997 | 0.994 to 1.001 |
| libcrux | short | 4 | 0.994 | 0.994 to 0.995 |
| rind | long | 3 | 1.000 | 0.998 to 1.006 |
| rind | short | 4 | 0.998 | 0.998 to 1.012 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

1 of 21 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1039 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rind/32B` | 288,632 | 291,982 | 1.012 |

## cipher

Known-answer checks: pad0 48/48, pad1039 48/48. Runs: 3 and 3. Rows: 42 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| aes128 | per op | 2 | 1.000 | 1.000 to 1.000 |
| aes256 | per op | 2 | 1.000 | 1.000 to 1.000 |
| chacha20 | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | long | 27 | 1.000 | 1.000 to 1.000 |
| rind | short | 9 | 1.000 | 1.000 to 1.000 |

0 of 42 rows moved by more than max(1%, run spread).

## sha2

Known-answer checks: pad0 39/39, pad1039 39/39. Runs: 3 and 3. Rows: 74 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 8 | 1.002 | 1.000 to 1.009 |
| libcrux | short | 4 | 1.002 | 1.001 to 1.005 |
| rind | long | 12 | 1.000 | 1.000 to 1.022 |
| rind | short | 8 | 1.001 | 0.588 to 1.386 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 0.999 | 0.989 to 0.999 |
| rustcrypto | short | 6 | 0.997 | 0.991 to 0.999 |

7 of 74 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1039 cycles | B/A |
|---|---:|---:|---:|
| `sha256-updates/rind/16B` | 641,418 | 376,896 | 0.588 |
| `sha256/rind/32B` | 19,871 | 27,543 | 1.386 |
| `sha256/rind/64B` | 24,875 | 32,498 | 1.306 |
| `sha256-updates/rind/1B` | 933,921 | 668,559 | 0.716 |
| `sha256/rind/1200B` | 114,294 | 116,833 | 1.022 |
| `sha256/rind/1350B` | 129,167 | 131,689 | 1.020 |
| `sha256/rustcrypto/1200B` | 102,135 | 101,026 | 0.989 |

## sha3

Known-answer checks: pad0 45/45, pad1039 45/45. Runs: 3 and 3. Rows: 54 in both.

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

Known-answer checks: pad0 12/12, pad1039 12/12. Runs: 3 and 3. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 0.998 | 0.987 to 0.999 |
| libcrux | short | 4 | 0.990 | 0.980 to 1.000 |
| rind | long | 24 | 1.000 | 0.981 to 1.047 |
| rind | short | 12 | 1.000 | 0.941 to 1.190 |
| ring | long | 24 | 1.000 | 1.000 to 1.000 |
| ring | short | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.061 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.209 |

17 of 102 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1039 cycles | B/A |
|---|---:|---:|---:|
| `hmac-sha256/rustcrypto/32B` | 42,476 | 51,340 | 1.209 |
| `hmac-sha256/rind/reused-key/32B` | 34,931 | 41,572 | 1.190 |
| `hmac-sha256/rustcrypto/64B` | 48,046 | 56,949 | 1.185 |
| `hmac-sha256/rind/reused-key/64B` | 40,044 | 46,256 | 1.155 |
| `hmac-sha256/rustcrypto/1200B` | 142,610 | 151,331 | 1.061 |
| `hmac-sha256/rind/32B` | 68,683 | 64,625 | 0.941 |
| `hmac-sha256/rind/64B` | 74,147 | 69,853 | 0.942 |
| `hmac-sha256/rustcrypto/1350B` | 158,286 | 167,333 | 1.057 |
| `hmac-sha256/rind/reused-key/1200B` | 121,407 | 127,127 | 1.047 |
| `hmac-sha256/rind/reused-key/1350B` | 136,248 | 141,968 | 1.042 |
| `hmac-sha256/libcrux/64B` | 42,174 | 41,310 | 0.980 |
| `hmac-sha256/libcrux/32B` | 36,874 | 36,124 | 0.980 |
| `hmac-sha256/rind/1200B` | 160,736 | 157,673 | 0.981 |
| `hmac-sha256/rind/1350B` | 179,030 | 176,416 | 0.985 |
| `hmac-sha256/libcrux/1200B` | 130,943 | 129,290 | 0.987 |
| `hmac-sha256/libcrux/1350B` | 145,787 | 144,134 | 0.989 |
| `hmac-sha256/rustcrypto/8KB` | 739,549 | 747,180 | 1.010 |

## hkdf

Known-answer checks: pad0 12/12, pad1039 12/12. Runs: 3 and 3. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 0.997 to 1.049 |
| expand-aes-256-gcm-key | per op | 8 | 1.005 | 1.000 to 1.057 |
| extract-expand | per op | 15 | 1.000 | 0.969 to 1.054 |

8 of 38 rows moved by more than max(1%, run spread):

| Row | pad0 cycles | pad1039 cycles | B/A |
|---|---:|---:|---:|
| `hkdf-sha1/expand-aes-256-gcm-key/rind` | 54,952 | 58,101 | 1.057 |
| `hkdf-sha256/extract-expand/rind` | 304,940 | 321,289 | 1.054 |
| `hkdf-sha256/expand/libcrux` | 107,026 | 112,244 | 1.049 |
| `hkdf-sha1/extract-expand/rustcrypto` | 29,892 | 28,960 | 0.969 |
| `hkdf-sha256/expand/rind` | 100,705 | 103,388 | 1.027 |
| `hkdf-sha256/expand/rustcrypto` | 24,312 | 24,917 | 1.025 |
| `hkdf-sha256/expand-aes-256-gcm-key/rind` | 381,328 | 390,090 | 1.023 |
| `hkdf-sha256/extract-expand/libcrux` | 211,040 | 207,354 | 0.983 |

