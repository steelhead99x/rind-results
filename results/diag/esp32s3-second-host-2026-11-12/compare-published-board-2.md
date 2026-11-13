Cycles, board-2 over published (below 1 is faster in board-2).

## aes_gcm

Known-answer checks: published 27/27, board-2 45/45. Runs: 3 and 5. Rows: 171 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 1 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 12 | 1.000 | 0.999 to 1.002 |
| libcrux | short | 24 | 1.002 | 0.991 to 1.027 |
| rind | long | 18 | 1.000 | 0.982 to 1.000 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 36 | 1.000 | 0.979 to 1.000 |
| ring | long | 12 | 1.000 | 0.983 to 1.008 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 0.990 | 0.834 to 1.008 |
| rustcrypto | long | 12 | 1.001 | 0.992 to 1.020 |
| rustcrypto | short | 24 | 1.017 | 0.840 to 1.134 |

58 of 171 rows moved by more than max(1%, run spread):

| Row | published cycles | board-2 cycles | B/A |
|---|---:|---:|---:|
| `aes128-gcm/ring/quic-32B` | 41,131 | 34,317 | 0.834 |
| `aes128-gcm/rustcrypto/quic-64B` | 48,419 | 40,688 | 0.840 |
| `aes128-gcm/ring/32B` | 43,078 | 36,237 | 0.841 |
| `aes128-gcm/rustcrypto/quic-32B` | 54,483 | 46,148 | 0.847 |
| `aes256-gcm/rustcrypto/32B` | 62,257 | 70,612 | 1.134 |
| `aes128-gcm/ring/quic-64B` | 51,972 | 45,127 | 0.868 |
| `aes128-gcm/rustcrypto/quic-128B` | 69,944 | 62,407 | 0.892 |
| `aes256-gcm/ring/quic-32B` | 43,423 | 38,868 | 0.895 |
| `aes256-gcm/ring/32B` | 45,347 | 40,790 | 0.899 |
| `aes256-gcm/rustcrypto/quic-32B` | 67,587 | 74,326 | 1.100 |
| `aes128-gcm/ring/quic-128B` | 73,560 | 66,744 | 0.907 |
| `aes256-gcm/ring/quic-64B` | 57,153 | 52,453 | 0.918 |
| `aes256-gcm/rustcrypto/quic-64B` | 73,948 | 79,189 | 1.071 |
| `aes128-gcm/rustcrypto/quic-256B` | 113,414 | 106,100 | 0.936 |
| `aes128-gcm/ring/quic-256B` | 116,797 | 109,980 | 0.942 |
| `aes256-gcm/ring/quic-128B` | 84,075 | 79,392 | 0.944 |
| `aes256-gcm/rustcrypto/quic-128B` | 99,638 | 104,764 | 1.051 |
| `aes256-gcm-open/rustcrypto/quic-32B` | 97,852 | 102,416 | 1.047 |
| `aes128-gcm-open/rustcrypto/32B` | 52,986 | 50,595 | 0.955 |
| `aes128-gcm-open/rustcrypto/quic-64B` | 62,241 | 64,749 | 1.040 |
| `aes256-gcm/rustcrypto/quic-256B` | 151,276 | 157,215 | 1.039 |
| `aes256-gcm/ring/quic-256B` | 138,727 | 133,989 | 0.966 |
| `aes128-gcm/ring/quic-512B` | 203,272 | 196,451 | 0.966 |
| `aes128-gcm/rustcrypto/quic-512B` | 199,714 | 193,024 | 0.967 |
| `aes256-gcm-open/rustcrypto/32B` | 98,296 | 101,563 | 1.033 |
| `aes128-gcm-open/rustcrypto/quic-32B` | 58,274 | 60,101 | 1.031 |
| `aes128-gcm-open/rustcrypto/quic-128B` | 83,967 | 86,385 | 1.029 |
| `aes128-gcm/libcrux/quic-32B` | 69,253 | 71,092 | 1.027 |
| `aes256-gcm/rustcrypto/quic-512B` | 257,939 | 264,714 | 1.026 |
| `aes128-gcm/libcrux/32B` | 76,964 | 78,866 | 1.025 |
| `aes192-gcm/rind/quic-512B` | 181,512 | 177,763 | 0.979 |
| `aes128-gcm-open/rustcrypto/quic-256B` | 127,310 | 129,926 | 1.021 |
| `aes128-gcm/rustcrypto/1200B` | 448,639 | 457,457 | 1.020 |
| `aes256-gcm/rind/quic-512B` | 199,544 | 195,699 | 0.981 |
| `aes256-gcm/ring/quic-512B` | 247,112 | 242,448 | 0.981 |
| `aes256-gcm/rind/quic-256B` | 106,169 | 104,246 | 0.982 |
| `aes128-gcm/libcrux/quic-128B` | 162,678 | 165,586 | 1.018 |
| `aes128-gcm-open/rind/1350B` | 419,288 | 411,922 | 0.982 |
| `aes128-gcm/rind/quic-256B` | 86,740 | 85,235 | 0.983 |
| `aes128-gcm/ring/1350B` | 500,058 | 491,446 | 0.983 |
| `aes128-gcm/rind/quic-128B` | 48,779 | 47,943 | 0.983 |
| `aes256-gcm/rind/quic-32B` | 24,630 | 24,226 | 0.984 |
| `aes256-gcm/rind/quic-64B` | 36,248 | 35,657 | 0.984 |
| `aes192-gcm/rind/quic-64B` | 33,012 | 32,477 | 0.984 |
| `aes256-gcm/rind/quic-128B` | 59,481 | 58,520 | 0.984 |
| `aes128-gcm/rind/quic-32B` | 20,300 | 19,973 | 0.984 |
| `aes192-gcm/rind/quic-32B` | 22,456 | 22,100 | 0.984 |
| `aes128-gcm/rind/quic-64B` | 29,766 | 29,296 | 0.984 |
| `aes128-gcm/libcrux/quic-64B` | 100,294 | 101,869 | 1.016 |
| `aes192-gcm/rind/quic-256B` | 96,239 | 94,739 | 0.984 |
| `aes128-gcm/rind/quic-512B` | 162,326 | 159,818 | 0.985 |
| `aes192-gcm/rind/quic-128B` | 54,064 | 53,231 | 0.985 |
| `aes128-gcm/ring/1200B` | 441,882 | 435,331 | 0.985 |
| `aes256-gcm-open/rind/1350B` | 512,078 | 504,710 | 0.986 |
| `aes128-gcm-open/rustcrypto/quic-512B` | 214,416 | 217,248 | 1.013 |
| `aes192-gcm-open/rind/1350B` | 464,275 | 458,316 | 0.987 |
| `aes256-gcm-open/libcrux/32B` | 194,500 | 196,654 | 1.011 |
| `aes256-gcm-open/libcrux/quic-128B` | 305,644 | 308,887 | 1.011 |

## chacha20poly1305

Known-answer checks: published 21/21, board-2 35/35. Runs: 3 and 5. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.000 | 0.988 to 1.028 |
| libcrux | short | 8 | 1.016 | 0.988 to 1.072 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

8 of 56 rows moved by more than max(1%, run spread):

| Row | published cycles | board-2 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/libcrux/quic-128B` | 64,335 | 68,987 | 1.072 |
| `chacha20poly1305-open/libcrux/32B` | 34,045 | 36,296 | 1.066 |
| `chacha20poly1305-open/libcrux/quic-512B` | 104,744 | 109,118 | 1.042 |
| `chacha20poly1305-open/libcrux/quic-256B` | 77,614 | 80,315 | 1.035 |
| `chacha20poly1305-open/libcrux/1200B` | 153,397 | 157,731 | 1.028 |
| `chacha20poly1305-open/libcrux/1350B` | 200,985 | 206,106 | 1.025 |
| `chacha20poly1305/libcrux/quic-512B` | 72,652 | 71,761 | 0.988 |
| `chacha20poly1305/libcrux/1350B` | 170,595 | 168,568 | 0.988 |

## cipher

Known-answer checks: published 48/48, board-2 80/80. Runs: 3 and 5. Rows: 42 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| aes128 | per op | 2 | 1.000 | 1.000 to 1.000 |
| aes256 | per op | 2 | 1.000 | 1.000 to 1.000 |
| chacha20 | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | long | 27 | 1.000 | 1.000 to 1.000 |
| rind | short | 9 | 1.000 | 1.000 to 1.000 |

0 of 42 rows moved by more than max(1%, run spread).

## ed25519

Known-answer checks: published 12/12, board-2 20/20. Runs: 3 and 5. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 3 | 0.999 | 0.998 to 1.000 |
| libcrux | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 0.999 | 0.998 to 1.000 |
| ring | per op | 3 | 1.000 | 1.000 to 1.005 |

0 of 12 rows moved by more than max(1%, run spread).

## hkdf

Known-answer checks: published 12/12, board-2 20/20. Runs: 3 and 5. Rows: 38 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| expand | per op | 15 | 1.000 | 0.833 to 1.074 |
| expand-aes-256-gcm-key | per op | 8 | 1.000 | 0.826 to 1.011 |
| extract-expand | per op | 15 | 1.000 | 0.817 to 1.252 |

9 of 38 rows moved by more than max(1%, run spread):

| Row | published cycles | board-2 cycles | B/A |
|---|---:|---:|---:|
| `hkdf-sha1/extract-expand/rustcrypto` | 29,892 | 37,416 | 1.252 |
| `hkdf-sha256/extract-expand/rind` | 304,940 | 249,181 | 0.817 |
| `hkdf-sha1/expand-aes-256-gcm-key/rind` | 54,952 | 45,385 | 0.826 |
| `hkdf-sha256/expand/rind` | 100,705 | 83,934 | 0.833 |
| `hkdf-sha256/expand/libcrux` | 107,026 | 114,975 | 1.074 |
| `hkdf-sha256/expand/rustcrypto` | 24,312 | 25,585 | 1.052 |
| `hkdf-sha256/extract-expand/libcrux` | 211,040 | 216,386 | 1.025 |
| `hkdf-sha256/extract-expand/rustcrypto` | 522,947 | 529,696 | 1.013 |
| `hkdf-sha256/expand-aes-256-gcm-key/rind` | 381,328 | 385,396 | 1.011 |

## hmac

Known-answer checks: published 12/12, board-2 20/20. Runs: 3 and 5. Rows: 102 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 8 | 1.001 | 1.000 to 1.031 |
| libcrux | short | 4 | 1.048 | 0.999 to 1.111 |
| rind | long | 24 | 1.000 | 1.000 to 1.269 |
| rind | short | 12 | 1.000 | 0.999 to 1.941 |
| ring | long | 24 | 1.000 | 1.000 to 1.000 |
| ring | short | 12 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 0.992 to 1.038 |
| rustcrypto | short | 6 | 1.001 | 1.000 to 1.174 |

20 of 102 rows moved by more than max(1%, run spread):

| Row | published cycles | board-2 cycles | B/A |
|---|---:|---:|---:|
| `hmac-sha256/rind/reused-key/32B` | 34,931 | 67,797 | 1.941 |
| `hmac-sha256/rind/reused-key/64B` | 40,044 | 72,648 | 1.814 |
| `hmac-sha256/rind/32B` | 68,683 | 115,208 | 1.677 |
| `hmac-sha256/rind/64B` | 74,147 | 119,939 | 1.618 |
| `hmac-sha256/rind/1200B` | 160,736 | 203,947 | 1.269 |
| `hmac-sha256/rind/reused-key/1200B` | 121,407 | 153,163 | 1.262 |
| `hmac-sha256/rind/reused-key/1350B` | 136,248 | 168,716 | 1.238 |
| `hmac-sha256/rind/1350B` | 179,030 | 218,788 | 1.222 |
| `hmac-sha256/rustcrypto/32B` | 42,476 | 49,883 | 1.174 |
| `hmac-sha256/rustcrypto/64B` | 48,046 | 55,386 | 1.153 |
| `hmac-sha256/libcrux/32B` | 36,874 | 40,973 | 1.111 |
| `hmac-sha256/libcrux/64B` | 42,174 | 46,269 | 1.097 |
| `hmac-sha256/rind/reused-key/8KB` | 669,665 | 700,844 | 1.047 |
| `hmac-sha256/rind/8KB` | 719,383 | 749,297 | 1.042 |
| `hmac-sha256/rustcrypto/1350B` | 158,286 | 164,289 | 1.038 |
| `hmac-sha256/libcrux/1200B` | 130,943 | 134,950 | 1.031 |
| `hmac-sha256/rustcrypto/1200B` | 142,610 | 146,591 | 1.028 |
| `hmac-sha256/libcrux/1350B` | 145,787 | 149,794 | 1.027 |
| `hmac-sha256/rind/reused-key/16KB` | 1,303,679 | 1,335,905 | 1.025 |
| `hmac-sha256/rind/16KB` | 1,354,465 | 1,387,451 | 1.024 |

## mlkem768

Known-answer checks: published 9/9, board-2 15/15. Runs: 3 and 5. Rows: 12 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux-ml-kem | per op | 4 | 1.000 | 1.000 to 1.000 |
| rind | per op | 4 | 1.000 | 1.000 to 1.000 |
| rustcrypto | per op | 4 | 1.000 | 1.000 to 1.000 |

0 of 12 rows moved by more than max(1%, run spread).

## p256

Known-answer checks: published 12/12, board-2 20/20. Runs: 3 and 5. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 1.000 | 1.000 to 1.000 |
| p256-rustcrypto | per op | 2 | 1.000 | 0.999 to 1.001 |
| rind | per op | 9 | 0.998 | 0.934 to 1.009 |
| ring | per op | 4 | 1.005 | 1.003 to 1.008 |
| rustcrypto | per op | 2 | 1.000 | 1.000 to 1.000 |

2 of 21 rows moved by more than max(1%, run spread):

| Row | published cycles | board-2 cycles | B/A |
|---|---:|---:|---:|
| `p256-ecdsa-sign-digest/rind` | 6,109,776 | 5,707,422 | 0.934 |
| `p256-ecdsa-sign/rind` | 6,234,613 | 5,865,103 | 0.941 |

## sha2

Known-answer checks: published 39/39, board-2 65/65. Runs: 3 and 5. Rows: 74 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.000 |
| esp32s3-hw | short | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | long | 8 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.000 |
| rind | long | 12 | 1.000 | 1.000 to 1.121 |
| rind | short | 8 | 1.000 | 0.639 to 1.639 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 1.000 | 1.000 to 1.001 |
| rustcrypto | short | 6 | 1.000 | 1.000 to 1.001 |

8 of 74 rows moved by more than max(1%, run spread):

| Row | published cycles | board-2 cycles | B/A |
|---|---:|---:|---:|
| `sha256/rind/32B` | 19,871 | 32,569 | 1.639 |
| `sha256/rind/64B` | 24,875 | 37,671 | 1.514 |
| `sha256-updates/rind/16B` | 641,418 | 409,995 | 0.639 |
| `sha256-updates/rind/1B` | 933,921 | 701,966 | 0.752 |
| `sha256/rind/1200B` | 114,294 | 128,121 | 1.121 |
| `sha256/rind/1350B` | 129,167 | 143,010 | 1.107 |
| `sha256/rind/8KB` | 663,026 | 679,325 | 1.025 |
| `sha256/rind/16KB` | 1,300,097 | 1,317,306 | 1.013 |

## sha3

Known-answer checks: published 45/45, board-2 75/75. Runs: 3 and 5. Rows: 54 in both.

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

Known-answer checks: published 12/12, board-2 20/20. Runs: 3 and 5. Rows: 9 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 2 | 1.000 | 1.000 to 1.000 |
| libcrux | per op | 2 | 1.000 | 1.000 to 1.000 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |

0 of 9 rows moved by more than max(1%, run spread).

## xchacha20poly1305

Known-answer checks: published 21/21, board-2 35/35. Runs: 3 and 5. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 4 | 1.000 | 1.000 to 1.000 |
| rind | long | 3 | 0.968 | 0.956 to 0.991 |
| rind | short | 4 | 0.946 | 0.942 to 0.949 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

6 of 21 rows moved by more than max(1%, run spread):

| Row | published cycles | board-2 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rind/quic-128B` | 320,984 | 302,207 | 0.942 |
| `xchacha20poly1305/rind/quic-256B` | 333,500 | 314,729 | 0.944 |
| `xchacha20poly1305/rind/quic-512B` | 358,520 | 339,752 | 0.948 |
| `xchacha20poly1305/rind/32B` | 288,632 | 273,935 | 0.949 |
| `xchacha20poly1305/rind/1200B` | 428,783 | 410,129 | 0.956 |
| `xchacha20poly1305/rind/1350B` | 445,838 | 431,687 | 0.968 |

