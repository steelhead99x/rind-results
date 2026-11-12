<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: relinking the same benches (layout sensitivity, 2026-11-12)

The `aes_gcm`, `sha2` and `chacha20poly1305` images rebuilt from the same rind (99b509a6 plus
patches) and the same bench files, with only the harness changed (rind-bench 0463305's wall-clock
and empty-call checks), so every function lands at a different address. Images:
`aes_gcm.elf` `74c9929f…`, `sha2.elf` `cc89876f…`, `chacha20poly1305.elf` `578699f6…`, one run each,
2026-11-12 10:09-10:13 PST, 40 MHz flash clock like the published set.

What moved:

- ring's rows barely move (its C is small loops that stay cached), and neither do 8-16 KB rows.
- Short-input rows of RustCrypto, libcrux **and rind** move a lot: `sha256/rustcrypto/32B` 0.12x
  (49,018 to 5,979 cycles), `sha256/rind/32B` 0.48x, `chacha20poly1305-open/libcrux/32B` 1.83x,
  `chacha20poly1305-open/rind/32B` 0.59x, `aes256-gcm/rustcrypto/quic-64B` 0.54x.
  The earlier relink (`../aes_gcm-oled`) had left every rind row within 2%; this one did not, so
  that was luck, not a property of rind.
- No headline verdict changes at the 1.1x gap line (21 operations), but magnitudes do:
  `sha256/…/64B` goes from 6.65x to 3.44x libcrux, `aes128-gcm/…/32B` from 0.68x to 0.53x ring,
  and `aes128-gcm-open/…/16KB`'s fastest rival changes from ring to RustCrypto.

Cause: a 16 KB instruction cache in front of flash. Whether a short call's code and its callees
collide in the cache depends on where the linker put them. A 32 KB cache (`../sha2-icache32`)
removes most of it.

## Headline ratios, published layout against this one

| Operation | published: rind/fastest | relinked: rind/fastest | verdict |
|---|---|---|---|
| aes128-gcm-open/…/1350B | 0.84x (ring) | 0.88x (ring) | ok |
| aes128-gcm-open/…/16KB | 0.86x (ring) | 0.86x (rustcrypto) | ok |
| aes128-gcm-open/…/32B | 0.74x (ring) | 0.79x (ring) | ok |
| aes128-gcm/…/1350B | 0.84x (ring) | 0.82x (ring) | ok |
| aes128-gcm/…/16KB | 0.86x (rustcrypto) | 0.87x (rustcrypto) | ok |
| aes128-gcm/…/32B | 0.68x (ring) | 0.53x (ring) | ok |
| aes256-gcm-open/…/1350B | 0.82x (ring) | 0.86x (ring) | ok |
| aes256-gcm-open/…/16KB | 0.89x (rustcrypto) | 0.90x (rustcrypto) | ok |
| aes256-gcm-open/…/32B | 0.72x (ring) | 0.76x (ring) | ok |
| aes256-gcm/…/1350B | 0.83x (ring) | 0.81x (ring) | ok |
| aes256-gcm/…/16KB | 0.89x (rustcrypto) | 0.90x (rustcrypto) | ok |
| aes256-gcm/…/32B | 0.59x (ring) | 0.61x (ring) | ok |
| chacha20poly1305/…/1350B | 1.31x (ring) | 1.30x (ring) | gap |
| chacha20poly1305/…/16KB | 1.27x (ring) | 1.26x (ring) | gap |
| chacha20poly1305/…/32B | 1.49x (ring) | 1.51x (ring) | gap |
| sha256/…/1350B | 1.74x (libcrux) | 1.44x (libcrux) | gap |
| sha256/…/16KB | 1.08x (libcrux) | 1.04x (libcrux) | ok |
| sha256/…/64B | 6.65x (libcrux) | 3.44x (libcrux) | gap |
| sha512/…/1350B | 63.36x (ring) | 63.32x (ring) | gap |
| sha512/…/16KB | 63.34x (ring) | 63.30x (ring) | gap |
| sha512/…/64B | 62.62x (ring) | 62.50x (ring) | gap |

21 operations, 0 change verdict at the 1.1x line

## Every row

Made with `tools/compare.py raw diag/layout-relink-2026-11-12 aes_gcm sha2 chacha20poly1305`:


## aes_gcm

Known-answer checks: published 27/27, relinked 9/9. Runs: 3 and 1. Rows: 171 in both; only in relinked: 2.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 3 | 1.004 | 1.000 to 1.004 |
| esp32s3-hw | short | 1 | 1.021 | 1.021 to 1.021 |
| libcrux | long | 12 | 1.001 | 0.968 to 1.029 |
| libcrux | short | 24 | 1.020 | 0.738 to 1.607 |
| rind | long | 18 | 1.001 | 1.000 to 1.037 |
| rind | per op | 3 | 1.000 | 1.000 to 1.000 |
| rind | short | 36 | 1.014 | 1.000 to 1.042 |
| ring | long | 12 | 1.000 | 0.982 to 1.025 |
| ring | per op | 2 | 1.000 | 1.000 to 1.000 |
| ring | short | 24 | 0.983 | 0.967 to 1.309 |
| rustcrypto | long | 12 | 0.978 | 0.887 to 1.012 |
| rustcrypto | short | 24 | 0.790 | 0.536 to 1.206 |

115 of 171 rows moved by more than max(1%, run spread):

| Row | published cycles | relinked cycles | B/A |
|---|---:|---:|---:|
| `aes256-gcm/libcrux/32B` | 127,919 | 205,510 | 1.607 |
| `aes256-gcm/rustcrypto/quic-64B` | 180,781 | 96,936 | 0.536 |
| `aes256-gcm/rustcrypto/quic-32B` | 166,166 | 97,114 | 0.584 |
| `aes256-gcm/rustcrypto/quic-128B` | 205,728 | 123,276 | 0.599 |
| `aes256-gcm/rustcrypto/32B` | 149,950 | 91,619 | 0.611 |
| `aes128-gcm/rustcrypto/quic-64B` | 137,736 | 88,913 | 0.646 |
| `aes256-gcm/rustcrypto/quic-256B` | 262,049 | 176,049 | 0.672 |
| `aes128-gcm/ring/quic-32B` | 29,412 | 38,497 | 1.309 |
| `aes128-gcm/rustcrypto/quic-128B` | 158,940 | 110,115 | 0.693 |
| `aes256-gcm/libcrux/quic-32B` | 164,707 | 215,130 | 1.306 |
| `aes128-gcm/ring/32B` | 31,376 | 40,412 | 1.288 |
| `aes128-gcm-open/libcrux/quic-32B` | 178,706 | 131,911 | 0.738 |
| `aes128-gcm/rustcrypto/32B` | 91,988 | 68,841 | 0.748 |
| `aes128-gcm-open/rustcrypto/quic-64B` | 171,521 | 130,513 | 0.761 |
| `aes128-gcm/rustcrypto/quic-256B` | 202,548 | 154,202 | 0.761 |
| `aes256-gcm-open/libcrux/32B` | 245,350 | 302,988 | 1.235 |
| `aes128-gcm/ring/quic-64B` | 40,270 | 49,317 | 1.225 |
| `aes128-gcm-open/libcrux/quic-64B` | 209,411 | 162,576 | 0.776 |
| `aes256-gcm/libcrux/quic-64B` | 205,624 | 251,471 | 1.223 |
| `aes256-gcm-open/libcrux/quic-32B` | 248,140 | 303,084 | 1.221 |
| `aes128-gcm-open/rustcrypto/quic-128B` | 192,653 | 151,835 | 0.788 |
| `aes256-gcm/rustcrypto/quic-512B` | 361,516 | 285,595 | 0.790 |
| `aes128-gcm/rustcrypto/quic-512B` | 306,442 | 242,370 | 0.791 |
| `aes256-gcm-open/rustcrypto/32B` | 149,923 | 180,855 | 1.206 |
| `aes128-gcm-open/rustcrypto/32B` | 137,803 | 113,068 | 0.821 |
| `aes128-gcm-open/rustcrypto/quic-256B` | 235,058 | 194,254 | 0.826 |
| `aes128-gcm-open/libcrux/quic-128B` | 270,748 | 223,834 | 0.827 |
| `aes128-gcm-open/libcrux/32B` | 169,331 | 140,368 | 0.829 |
| `aes128-gcm/rustcrypto/quic-32B` | 112,748 | 93,673 | 0.831 |
| `aes256-gcm/libcrux/quic-128B` | 294,901 | 343,501 | 1.165 |
| `aes128-gcm/ring/quic-128B` | 61,840 | 70,957 | 1.147 |
| `aes128-gcm-open/rustcrypto/quic-512B` | 324,421 | 279,255 | 0.861 |
| `aes128-gcm-open/libcrux/quic-256B` | 393,304 | 346,428 | 0.881 |
| `aes256-gcm/rustcrypto/1200B` | 659,588 | 585,014 | 0.887 |
| `aes256-gcm-open/libcrux/quic-64B` | 305,017 | 338,799 | 1.111 |
| `aes256-gcm/libcrux/quic-256B` | 440,224 | 488,164 | 1.109 |
| `aes128-gcm-open/rustcrypto/1200B` | 604,600 | 538,774 | 0.891 |
| `aes256-gcm/rustcrypto/1350B` | 746,012 | 665,534 | 0.892 |
| `aes128-gcm/libcrux/quic-32B` | 74,862 | 67,279 | 0.899 |
| `aes256-gcm-open/rustcrypto/quic-128B` | 238,745 | 215,220 | 0.901 |
| `aes256-gcm-open/rustcrypto/quic-64B` | 210,699 | 190,304 | 0.903 |
| `aes256-gcm-open/libcrux/quic-128B` | 376,297 | 410,082 | 1.090 |
| `aes128-gcm/ring/quic-256B` | 105,076 | 114,243 | 1.087 |
| `aes128-gcm/libcrux/quic-64B` | 106,130 | 97,720 | 0.921 |
| `aes128-gcm/rustcrypto/1200B` | 529,276 | 489,750 | 0.925 |
| `aes128-gcm-open/libcrux/quic-512B` | 638,455 | 591,612 | 0.927 |
| `aes128-gcm-open/rustcrypto/1350B` | 660,208 | 615,564 | 0.932 |
| `aes256-gcm/libcrux/quic-512B` | 722,629 | 771,025 | 1.067 |
| `aes128-gcm-open/rustcrypto/quic-32B` | 158,677 | 148,310 | 0.935 |
| `aes256-gcm-open/libcrux/quic-256B` | 519,058 | 552,879 | 1.065 |
| `aes256-gcm-open/rustcrypto/quic-256B` | 288,911 | 273,549 | 0.947 |
| `aes128-gcm/ring/quic-512B` | 191,555 | 200,596 | 1.047 |
| `aes128-gcm/libcrux/quic-128B` | 168,221 | 160,412 | 0.954 |
| `aes128-gcm-open/rind/quic-512B` | 160,238 | 166,998 | 1.042 |
| `aes256-gcm-open/libcrux/quic-512B` | 806,278 | 838,365 | 1.040 |
| `aes128-gcm-open/rind/quic-32B` | 20,083 | 20,880 | 1.040 |
| `aes128-gcm-open/rind/32B` | 21,386 | 22,233 | 1.040 |
| `aes128-gcm-open/rind/quic-64B` | 29,426 | 30,555 | 1.038 |
| `aes192-gcm-open/rind/quic-512B` | 178,174 | 184,937 | 1.038 |
| `aes128-gcm-open/rind/1200B` | 365,062 | 378,583 | 1.037 |
| `aes256-gcm-open/rind/quic-512B` | 196,110 | 203,344 | 1.037 |
| `aes128-gcm-open/rind/quic-128B` | 48,113 | 49,805 | 1.035 |
| `aes256-gcm-open/rind/quic-256B` | 104,496 | 108,113 | 1.035 |
| `aes256-gcm-open/rustcrypto/quic-512B` | 387,388 | 374,256 | 0.966 |
| `aes192-gcm-open/rind/1200B` | 406,186 | 419,710 | 1.033 |
| `aes128-gcm/rustcrypto/1350B` | 586,795 | 567,426 | 0.967 |
| `aes256-gcm/ring/quic-32B` | 41,068 | 39,713 | 0.967 |
| `aes128-gcm-open/rind/1350B` | 411,921 | 425,440 | 1.033 |
| `aes256-gcm-open/rind/1200B` | 447,310 | 461,773 | 1.032 |
| `aes192-gcm-open/rind/quic-128B` | 53,402 | 55,093 | 1.032 |
| `aes128-gcm-open/rind/quic-256B` | 85,486 | 88,190 | 1.032 |
| `aes128-gcm-open/libcrux/1350B` | 1,459,813 | 1,413,756 | 0.968 |
| `aes256-gcm-open/rind/32B` | 25,639 | 26,444 | 1.031 |
| `aes256-gcm-open/rind/quic-32B` | 24,336 | 25,098 | 1.031 |
| `aes256-gcm-open/rind/quic-64B` | 35,787 | 36,900 | 1.031 |
| `aes256-gcm-open/rind/quic-128B` | 58,690 | 60,499 | 1.031 |
| `aes256-gcm/ring/32B` | 42,991 | 41,672 | 0.969 |
| `aes192-gcm-open/rind/quic-64B` | 32,607 | 33,574 | 1.030 |
| `aes192-gcm-open/rind/1350B` | 458,315 | 471,838 | 1.030 |
| `aes256-gcm/libcrux/1200B` | 1,475,920 | 1,519,219 | 1.029 |
| `aes192-gcm-open/rind/quic-32B` | 22,210 | 22,855 | 1.029 |
| `aes192-gcm-open/rind/32B` | 23,513 | 24,190 | 1.029 |
| `aes256-gcm-open/rind/1350B` | 504,709 | 519,172 | 1.029 |
| `aes192-gcm-open/rind/quic-256B` | 94,990 | 97,695 | 1.028 |
| `aes256-gcm-open/rustcrypto/quic-32B` | 198,319 | 193,045 | 0.973 |
| `aes128-gcm/ring/1350B` | 487,312 | 499,734 | 1.025 |
| `aes256-gcm/ring/quic-64B` | 54,608 | 53,220 | 0.975 |
| `aes128-gcm-open/libcrux/1200B` | 1,289,296 | 1,260,261 | 0.977 |
| `aes256-gcm-open/libcrux/1200B` | 1,572,583 | 1,606,077 | 1.021 |
| `aes128-gcm/ring/1200B` | 429,989 | 439,116 | 1.021 |
| `aes128-gcm/esp32s3-hw/32B` | 18,026 | 18,407 | 1.021 |
| `aes128-gcm-open/ring/32B` | 28,772 | 28,170 | 0.979 |
| `aes128-gcm-open/ring/quic-32B` | 26,801 | 26,242 | 0.979 |
| `aes128-gcm-open/ring/quic-64B` | 37,852 | 37,071 | 0.979 |
| `aes256-gcm/libcrux/1350B` | 1,715,554 | 1,750,939 | 1.021 |
| `aes128-gcm-open/ring/quic-512B` | 192,572 | 188,684 | 0.980 |
| `aes256-gcm-open/ring/quic-256B` | 130,495 | 127,904 | 0.980 |
| `aes128-gcm-open/ring/quic-256B` | 103,993 | 102,048 | 0.981 |
| `aes128-gcm-open/ring/quic-128B` | 59,843 | 58,729 | 0.981 |
| `aes256-gcm/ring/1350B` | 608,481 | 619,463 | 1.018 |
| `aes128-gcm-open/ring/1200B` | 434,716 | 426,887 | 0.982 |
| `aes256-gcm/ring/quic-128B` | 81,693 | 80,265 | 0.983 |
| `aes256-gcm-open/ring/quic-128B` | 74,953 | 73,656 | 0.983 |
| `aes256-gcm-open/ring/32B` | 35,499 | 34,899 | 0.983 |
| `aes256-gcm-open/ring/quic-32B` | 33,528 | 32,971 | 0.983 |
| `aes128-gcm/libcrux/quic-256B` | 289,402 | 284,589 | 0.983 |
| `aes256-gcm-open/ring/quic-64B` | 47,312 | 46,532 | 0.984 |
| `aes256-gcm-open/ring/quic-512B` | 240,278 | 236,392 | 0.984 |
| `aes128-gcm-open/ring/1350B` | 490,717 | 482,886 | 0.984 |
| `aes256-gcm-open/libcrux/1350B` | 1,770,073 | 1,797,504 | 1.015 |
| `aes128-gcm/libcrux/quic-512B` | 540,704 | 532,893 | 0.986 |
| `aes256-gcm-open/ring/1200B` | 542,527 | 534,699 | 0.986 |
| `aes256-gcm-open/ring/1350B` | 612,838 | 605,014 | 0.987 |
| `aes256-gcm-open/rustcrypto/1350B` | 779,472 | 788,976 | 1.012 |
| `aes256-gcm/rustcrypto/16KB` | 6,568,472 | 6,491,666 | 0.988 |

Rows only in relinked:

| Row | cycles | spread | c/B |
|---|---:|---:|---:|
| `harness/empty/iter` | 17 | 0.00% | |
| `harness/empty/iter_batched` | 9 | 0.00% | |

## sha2

Known-answer checks: published 39/39, relinked 13/13. Runs: 3 and 1. Rows: 74 in both; only in relinked: 2.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| esp32s3-hw | long | 4 | 1.000 | 1.000 to 1.001 |
| esp32s3-hw | short | 2 | 1.001 | 1.000 to 1.001 |
| libcrux | long | 8 | 0.999 | 0.995 to 1.001 |
| libcrux | short | 4 | 0.992 | 0.975 to 0.996 |
| rind | long | 12 | 0.999 | 0.811 to 1.000 |
| rind | short | 8 | 0.985 | 0.478 to 1.000 |
| ring | long | 12 | 1.000 | 1.000 to 1.000 |
| ring | short | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 12 | 0.999 | 0.691 to 0.999 |
| rustcrypto | short | 6 | 0.999 | 0.122 to 1.002 |

16 of 74 rows moved by more than max(1%, run spread):

| Row | published cycles | relinked cycles | B/A |
|---|---:|---:|---:|
| `sha256/rustcrypto/32B` | 49,018 | 5,979 | 0.122 |
| `sha256/rustcrypto/64B` | 61,709 | 11,446 | 0.185 |
| `sha256/rind/32B` | 66,863 | 31,934 | 0.478 |
| `sha256/rind/64B` | 72,116 | 36,865 | 0.511 |
| `sha256/rustcrypto/1200B` | 150,740 | 104,127 | 0.691 |
| `sha256/rustcrypto/1350B` | 166,559 | 119,803 | 0.719 |
| `sha256/rind/1200B` | 180,207 | 146,064 | 0.811 |
| `sha256/rind/1350B` | 195,050 | 160,976 | 0.825 |
| `sha256/rind/8KB` | 735,255 | 692,126 | 0.941 |
| `sha256/rustcrypto/8KB` | 726,394 | 691,761 | 0.952 |
| `sha256/rind/16KB` | 1,384,140 | 1,332,862 | 0.963 |
| `sha256-updates/rind/1B` | 1,228,621 | 1,184,334 | 0.964 |
| `sha256-updates/rind/16B` | 937,567 | 911,016 | 0.972 |
| `sha256/rustcrypto/16KB` | 1,396,642 | 1,362,009 | 0.975 |
| `sha256/libcrux/32B` | 5,822 | 5,678 | 0.975 |
| `sha256/libcrux/64B` | 10,852 | 10,709 | 0.987 |

Rows only in relinked:

| Row | cycles | spread | c/B |
|---|---:|---:|---:|
| `harness/empty/iter` | 17 | 0.00% | |
| `harness/empty/iter_batched` | 9 | 0.00% | |

## chacha20poly1305

Known-answer checks: published 21/21, relinked 7/7. Runs: 3 and 1. Rows: 56 in both; only in relinked: 2.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.008 | 0.910 to 1.181 |
| libcrux | short | 8 | 0.925 | 0.604 to 1.825 |
| rind | long | 6 | 0.985 | 0.911 to 0.995 |
| rind | short | 8 | 0.903 | 0.591 to 1.009 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

24 of 56 rows moved by more than max(1%, run spread):

| Row | published cycles | relinked cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/libcrux/32B` | 40,436 | 73,799 | 1.825 |
| `chacha20poly1305-open/rind/32B` | 22,454 | 13,270 | 0.591 |
| `chacha20poly1305/libcrux/32B` | 20,380 | 12,310 | 0.604 |
| `chacha20poly1305/libcrux/quic-128B` | 65,975 | 44,891 | 0.680 |
| `chacha20poly1305-open/rind/quic-128B` | 29,888 | 20,397 | 0.682 |
| `chacha20poly1305/libcrux/quic-256B` | 80,336 | 58,584 | 0.729 |
| `chacha20poly1305-open/rind/quic-256B` | 42,820 | 32,966 | 0.770 |
| `chacha20poly1305/libcrux/quic-512B` | 107,366 | 84,933 | 0.791 |
| `chacha20poly1305-open/libcrux/1200B` | 166,669 | 196,773 | 1.181 |
| `chacha20poly1305-open/rind/quic-512B` | 69,268 | 58,139 | 0.839 |
| `chacha20poly1305-open/libcrux/quic-128B` | 94,566 | 106,972 | 1.131 |
| `chacha20poly1305/libcrux/1350B` | 209,166 | 190,302 | 0.910 |
| `chacha20poly1305-open/rind/1200B` | 142,267 | 129,570 | 0.911 |
| `chacha20poly1305-open/libcrux/quic-256B` | 109,602 | 118,519 | 1.081 |
| `chacha20poly1305-open/rind/1350B` | 158,974 | 146,275 | 0.920 |
| `chacha20poly1305-open/libcrux/quic-512B` | 138,964 | 147,112 | 1.059 |
| `chacha20poly1305-open/libcrux/1350B` | 238,857 | 250,156 | 1.047 |
| `chacha20poly1305/libcrux/1200B` | 139,239 | 134,274 | 0.964 |
| `chacha20poly1305/rind/quic-256B` | 33,820 | 32,710 | 0.967 |
| `chacha20poly1305/rind/quic-512B` | 59,659 | 57,718 | 0.967 |
| `chacha20poly1305/rind/quic-128B` | 20,851 | 20,216 | 0.970 |
| `chacha20poly1305/rind/1200B` | 131,361 | 128,717 | 0.980 |
| `chacha20poly1305-open/libcrux/16KB` | 1,706,975 | 1,733,727 | 1.016 |
| `chacha20poly1305/rind/1350B` | 147,961 | 146,412 | 0.990 |

Rows only in relinked:

| Row | cycles | spread | c/B |
|---|---:|---:|---:|
| `harness/empty/iter` | 17 | 0.00% | |
| `harness/empty/iter_batched` | 9 | 0.00% | |

