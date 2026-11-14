# rind on an ESP32-S3

rind-bench's own benches, run unchanged but for the aws-lc-rs rows on one core of an ESP32-S3 at 240 MHz, timed with the core's cycle counter. Every operation and ID is `admin/bench-report`'s.

- rind: upstream `99b509a680cf0ad0cd8b7d9c03f7360e7fd95cef`, plus the port's patches: `0001-fix-portable-build-MD5-on-Xtensa-whose-inline-assemb.patch`, `0002-feat-bench-leave-aws-lc-rs-rows-out-on-Xtensa-and-ba.patch`, `0003-feat-build-for-bare-metal-targets-without-std.patch`, `ring-0.17.14-xtensa-poly1305-align.patch`
- Firmware images (one per bench file; `images.sha256` beside this file):

  | Image | sha256 | rind | rind-hardware | legacy rind-bench | Built |
  |---|---|---|---|---|---|
  | aes_gcm | `d3ab22144b2d0f0f…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11 23:15-23:19 PST |
  | chacha20poly1305 | `b5a7e512525c213f…` | `99b509a6` | ancestor of `8d9c1bb` | 44a8830 (2026-11-12 03:52 PST), with the ring Xtensa patch | 2026-11-12 03:56 PST |
  | cipher | `101bc29ea9e98c97…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11 23:15-23:19 PST |
  | ed25519 | `ed53714f07fe3024…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11 23:15-23:19 PST |
  | hkdf | `2ad77c9d444da6ba…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11 23:15-23:19 PST |
  | hmac | `8e8f6d7c709d6123…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11 23:15-23:19 PST |
  | mlkem768 | `75069a7bc0a506ce…` | `99b509a6` | ancestor of `8d9c1bb` | the tree after 669571c with the 160 KB stack (committed in 9b9ebcb, 2026-11-12 01:14 PST) | 2026-11-11 23:51 PST |
  | p256 | `60e0068fc2c642ad…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11 23:15-23:19 PST |
  | sha2 | `c153499481a3989c…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11 23:15-23:19 PST |
  | x25519 | `f806b1bd0f734794…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became 669571c (2026-11-11 23:32 PST) | 2026-11-11, before 23:32 PST (the first run began 00:12 PST) |
  | xchacha20poly1305 | `9e07e2b51bedad60…` | `99b509a6` | ancestor of `8d9c1bb` | 44a8830 (2026-11-12 03:52 PST), with the ring Xtensa patch | 2026-11-12 03:56 PST |

- OS: ESP-IDF v5.5.1 (FreeRTOS), bare board
- SPI flash clock, as the bootloader reported it each boot: 40 MHz (espflash's default; see the README)
- Toolchain: rustc 1.97.0-nightly (8ea53bcd7 2026-07-08) (esp 1.97.0.0); xtensa-esp-elf GCC 14.2.0 (esp-14.2.0_20241119)
- Runs: 33 of 33 completed, one firmware image per rind-bench bench file
  - aes_gcm run 1: ok, 171 results, 2026-11-12T07:19:43+00:00 to 2026-11-12T07:21:08+00:00 (UTC)
  - aes_gcm run 2: ok, 171 results, 2026-11-12T07:21:08+00:00 to 2026-11-12T07:22:33+00:00 (UTC)
  - aes_gcm run 3: ok, 171 results, 2026-11-12T07:22:33+00:00 to 2026-11-12T07:23:57+00:00 (UTC)
  - chacha20poly1305 run 1: ok, 56 results, 2026-11-12T17:09:21+00:00 to 2026-11-12T17:09:42+00:00 (UTC)
  - chacha20poly1305 run 2: ok, 56 results, 2026-11-12T17:09:42+00:00 to 2026-11-12T17:10:03+00:00 (UTC)
  - chacha20poly1305 run 3: ok, 56 results, 2026-11-12T17:10:03+00:00 to 2026-11-12T17:10:24+00:00 (UTC)
  - cipher run 1: ok, 42 results, 2026-11-12T07:24:15+00:00 to 2026-11-12T07:24:39+00:00 (UTC)
  - cipher run 2: ok, 42 results, 2026-11-12T07:24:39+00:00 to 2026-11-12T07:25:03+00:00 (UTC)
  - cipher run 3: ok, 42 results, 2026-11-12T07:25:03+00:00 to 2026-11-12T07:25:28+00:00 (UTC)
  - ed25519 run 1: ok, 12 results, 2026-11-12T07:25:37+00:00 to 2026-11-12T07:27:10+00:00 (UTC)
  - ed25519 run 2: ok, 12 results, 2026-11-12T07:27:10+00:00 to 2026-11-12T07:28:43+00:00 (UTC)
  - ed25519 run 3: ok, 12 results, 2026-11-12T07:28:43+00:00 to 2026-11-12T07:30:15+00:00 (UTC)
  - hkdf run 1: ok, 38 results, 2026-11-12T07:30:22+00:00 to 2026-11-12T07:30:52+00:00 (UTC)
  - hkdf run 2: ok, 38 results, 2026-11-12T07:30:52+00:00 to 2026-11-12T07:31:23+00:00 (UTC)
  - hkdf run 3: ok, 38 results, 2026-11-12T07:31:23+00:00 to 2026-11-12T07:31:53+00:00 (UTC)
  - hmac run 1: ok, 102 results, 2026-11-12T07:31:58+00:00 to 2026-11-12T07:33:56+00:00 (UTC)
  - hmac run 2: ok, 102 results, 2026-11-12T07:33:56+00:00 to 2026-11-12T07:35:55+00:00 (UTC)
  - hmac run 3: ok, 102 results, 2026-11-12T07:35:55+00:00 to 2026-11-12T07:37:53+00:00 (UTC)
  - mlkem768 run 1: ok, 12 results, 2026-11-12T07:57:01+00:00 to 2026-11-12T07:57:20+00:00 (UTC)
  - mlkem768 run 2: ok, 12 results, 2026-11-12T07:57:20+00:00 to 2026-11-12T07:57:38+00:00 (UTC)
  - mlkem768 run 3: ok, 12 results, 2026-11-12T07:57:38+00:00 to 2026-11-12T07:57:57+00:00 (UTC)
  - p256 run 1: ok, 21 results, 2026-11-12T07:58:08+00:00 to 2026-11-12T08:00:54+00:00 (UTC)
  - p256 run 2: ok, 21 results, 2026-11-12T08:00:54+00:00 to 2026-11-12T08:03:39+00:00 (UTC)
  - p256 run 3: ok, 21 results, 2026-11-12T08:03:39+00:00 to 2026-11-12T08:06:25+00:00 (UTC)
  - sha2 run 1: ok, 74 results, 2026-11-12T08:06:30+00:00 to 2026-11-12T08:08:28+00:00 (UTC)
  - sha2 run 2: ok, 74 results, 2026-11-12T08:08:28+00:00 to 2026-11-12T08:10:26+00:00 (UTC)
  - sha2 run 3: ok, 74 results, 2026-11-12T08:10:26+00:00 to 2026-11-12T08:12:24+00:00 (UTC)
  - x25519 run 1: ok, 9 results, 2026-11-12T08:12:31+00:00 to 2026-11-12T08:13:41+00:00 (UTC)
  - x25519 run 2: ok, 9 results, 2026-11-12T08:13:41+00:00 to 2026-11-12T08:14:50+00:00 (UTC)
  - x25519 run 3: ok, 9 results, 2026-11-12T08:14:50+00:00 to 2026-11-12T08:15:59+00:00 (UTC)
  - xchacha20poly1305 run 1: ok, 21 results, 2026-11-12T17:10:29+00:00 to 2026-11-12T17:10:38+00:00 (UTC)
  - xchacha20poly1305 run 2: ok, 21 results, 2026-11-12T17:10:38+00:00 to 2026-11-12T17:10:47+00:00 (UTC)
  - xchacha20poly1305 run 3: ok, 21 results, 2026-11-12T17:10:47+00:00 to 2026-11-12T17:10:56+00:00 (UTC)

Each cell is the median over runs of each run's median time per operation, with cycles per byte for byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. `esp32s3-hw` is ESP-IDF's mbedTLS on the chip's AES and SHA accelerators: hardware, not a Rust library. rind runs its portable Rust backend here (it has no Xtensa assembly).

## Headline: rind against the fastest other Rust library

| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |
|---|---|---|---|---|
| aes128-gcm/…/32B | 88.65 µs (664.9 c/B) | ring: 130.73 µs (980.5 c/B) | 0.68× | 0.0% |
| aes128-gcm/…/1350B | 1.71 ms (304.4 c/B) | ring: 2.03 ms (361.0 c/B) | 0.84× | 0.0% |
| aes128-gcm/…/16KB | 19.96 ms (292.3 c/B) | rustcrypto: 23.13 ms (338.8 c/B) | 0.86× | 0.0% |
| aes128-gcm-open/…/32B | 89.11 µs (668.3 c/B) | ring: 119.88 µs (899.1 c/B) | 0.74× | 0.0% |
| aes128-gcm-open/…/1350B | 1.72 ms (305.1 c/B) | ring: 2.04 ms (363.5 c/B) | 0.84× | 0.0% |
| aes128-gcm-open/…/16KB | 20.00 ms (292.9 c/B) | ring: 23.26 ms (340.7 c/B) | 0.86× | 0.0% |
| aes256-gcm/…/32B | 106.37 µs (797.8 c/B) | ring: 179.13 µs (1343.5 c/B) | 0.59× | 0.0% |
| aes256-gcm/…/1350B | 2.10 ms (373.2 c/B) | ring: 2.54 ms (450.7 c/B) | 0.83× | 0.0% |
| aes256-gcm/…/16KB | 24.46 ms (358.3 c/B) | rustcrypto: 27.37 ms (400.9 c/B) | 0.89× | 0.0% |
| aes256-gcm-open/…/32B | 106.83 µs (801.2 c/B) | ring: 147.91 µs (1109.4 c/B) | 0.72× | 0.0% |
| aes256-gcm-open/…/1350B | 2.10 ms (373.9 c/B) | ring: 2.55 ms (454.0 c/B) | 0.82× | 0.0% |
| aes256-gcm-open/…/16KB | 24.50 ms (359.0 c/B) | rustcrypto: 27.43 ms (401.8 c/B) | 0.89× | 0.0% |
| chacha20poly1305/…/32B | 55.30 µs (414.7 c/B) | ring: 37.04 µs (277.8 c/B) | 1.49× | 0.0% |
| chacha20poly1305/…/1350B | 616.50 µs (109.6 c/B) | ring: 470.03 µs (83.6 c/B) | 1.31× | 0.0% |
| chacha20poly1305/…/16KB | 6.77 ms (99.2 c/B) | ring: 5.34 ms (78.2 c/B) | 1.27× | 0.0% |
| xchacha20poly1305/…/32B | 2.09 ms (15706.3 c/B) | rustcrypto: 160.38 µs (1202.8 c/B) | 13.06× | 0.0% |
| xchacha20poly1305/…/1350B | 2.81 ms (500.4 c/B) | rustcrypto: 684.11 µs (121.6 c/B) | 4.11× | 0.0% |
| xchacha20poly1305/…/16KB | 8.90 ms (130.3 c/B) | rustcrypto: 6.57 ms (96.2 c/B) | 1.35× | 0.0% |
| quic-header-protection/aes128/… | 24.19 µs (5,807 c) | ring: 37.32 µs (8,958 c) | 0.65× | 0.0% |
| quic-header-protection/aes256/… | 33.13 µs (7,951 c) | ring: 51.34 µs (12 kc) | 0.65× | 0.0% |
| quic-header-protection/chacha20/… | 9.37 µs (2,248 c) | ring: 10.83 µs (2,598 c) | 0.87× | 0.0% |
| x25519-ecdh/… | 30.20 ms (7,249 kc) | rustcrypto: 30.01 ms (7,203 kc) | 1.01× | 0.0% |
| p256-ecdh/… | 62.35 ms (14,964 kc) | rustcrypto: 88.47 ms (21,234 kc) | 0.70× | 0.0% |
| ed25519-sign/… | 24.86 ms (5,965 kc) | ring: 28.36 ms (6,807 kc) | 0.88× | 0.0% |
| ed25519-verify/… | 30.86 ms (7,406 kc) | rustcrypto: 31.28 ms (7,506 kc) | 0.99× | 0.0% |
| p256-ecdsa-sign/… | 37.26 ms (8,943 kc) | rustcrypto: 34.47 ms (8,272 kc) | 1.08× | 0.0% |
| p256-ecdsa-verify/… | 53.13 ms (12,751 kc) | rustcrypto: 76.59 ms (18,382 kc) | 0.69× | 0.0% |
| sha256/…/64B | 300.48 µs (1126.8 c/B) | libcrux: 45.22 µs (169.6 c/B) | 6.65× | 0.0% |
| sha256/…/1350B | 812.71 µs (144.5 c/B) | libcrux: 467.10 µs (83.0 c/B) | 1.74× | 0.0% |
| sha256/…/16KB | 5.77 ms (84.5 c/B) | libcrux: 5.34 ms (78.3 c/B) | 1.08× | 0.0% |
| sha512/…/64B | 6.43 ms (24105.1 c/B) | ring: 102.65 µs (385.0 c/B) | 62.62× | 0.0% |
| sha512/…/1350B | 67.46 ms (11992.4 c/B) | ring: 1.06 ms (189.3 c/B) | 63.36× | 0.0% |
| sha512/…/16KB | 786.59 ms (11522.3 c/B) | ring: 12.42 ms (181.9 c/B) | 63.34× | 0.0% |
| hmac-sha256/…/64B | 928.40 µs (3481.5 c/B) | ring: 140.37 µs (526.4 c/B) | 6.61× | 0.0% |
| hmac-sha256/…/1350B | 1.42 ms (251.6 c/B) | libcrux: 618.65 µs (110.0 c/B) | 2.29× | 0.1% |
| hkdf-sha256/extract-expand/… | 1.97 ms (473 kc) | ring: 231.04 µs (55 kc) | 8.52× | 0.0% |
| hkdf-sha256/expand/… | 636.79 µs (153 kc) | ring: 60.60 µs (15 kc) | 10.51× | 0.0% |
| mlkem768-keygen/… | 13.36 ms (3,207 kc) | rustcrypto: 15.20 ms (3,647 kc) | 0.88× | 0.0% |
| mlkem768-encaps/… | 14.16 ms (3,398 kc) | rustcrypto: 17.83 ms (4,279 kc) | 0.79× | 0.0% |
| mlkem768-decaps/… | 6.19 ms (1,486 kc) | rustcrypto: 20.28 ms (4,868 kc) | 0.31× | 0.3% |

## AEADs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| aes128-gcm/…/32B | 88.65 µs (664.9 c/B) | 130.73 µs (980.5 c/B) | 383.28 µs (2874.6 c/B) | 314.61 µs (2359.6 c/B) | 75.11 µs (563.3 c/B) |
| aes128-gcm/…/1350B | 1.71 ms (304.4 c/B) | 2.03 ms (361.0 c/B) | 2.44 ms (434.7 c/B) | 5.65 ms (1005.0 c/B) | 428.75 µs (76.2 c/B) |
| aes128-gcm/…/16KB | 19.96 ms (292.3 c/B) | 23.20 ms (339.9 c/B) | 23.13 ms (338.8 c/B) | 65.62 ms (961.2 c/B) | 4.43 ms (64.9 c/B) |
| aes128-gcm-open/…/32B | 89.11 µs (668.3 c/B) | 119.88 µs (899.1 c/B) | 574.18 µs (4306.4 c/B) | 705.55 µs (5291.6 c/B) | - |
| aes128-gcm-open/…/1350B | 1.72 ms (305.1 c/B) | 2.04 ms (363.5 c/B) | 2.75 ms (489.0 c/B) | 6.08 ms (1081.3 c/B) | - |
| aes128-gcm-open/…/16KB | 20.00 ms (292.9 c/B) | 23.26 ms (340.7 c/B) | 23.30 ms (341.4 c/B) | 66.01 ms (966.9 c/B) | - |
| aes256-gcm/…/32B | 106.37 µs (797.8 c/B) | 179.13 µs (1343.5 c/B) | 624.79 µs (4685.9 c/B) | 532.99 µs (3997.5 c/B) | - |
| aes256-gcm/…/1350B | 2.10 ms (373.2 c/B) | 2.54 ms (450.7 c/B) | 3.11 ms (552.6 c/B) | 7.15 ms (1270.8 c/B) | - |
| aes256-gcm/…/16KB | 24.46 ms (358.3 c/B) | 29.07 ms (425.8 c/B) | 27.37 ms (400.9 c/B) | 76.74 ms (1124.2 c/B) | - |
| aes256-gcm-open/…/32B | 106.83 µs (801.2 c/B) | 147.91 µs (1109.4 c/B) | 624.68 µs (4685.1 c/B) | 1.02 ms (7667.2 c/B) | - |
| aes256-gcm-open/…/1350B | 2.10 ms (373.9 c/B) | 2.55 ms (454.0 c/B) | 3.25 ms (577.4 c/B) | 7.38 ms (1311.2 c/B) | - |
| aes256-gcm-open/…/16KB | 24.50 ms (359.0 c/B) | 29.12 ms (426.6 c/B) | 27.43 ms (401.8 c/B) | 77.11 ms (1129.5 c/B) | - |
| chacha20poly1305/…/32B | 55.30 µs (414.7 c/B) | 37.04 µs (277.8 c/B) | 39.08 µs (293.1 c/B) | 84.92 µs (636.9 c/B) | - |
| chacha20poly1305/…/1350B | 616.50 µs (109.6 c/B) | 470.03 µs (83.6 c/B) | 563.50 µs (100.2 c/B) | 871.53 µs (154.9 c/B) | - |
| chacha20poly1305/…/16KB | 6.77 ms (99.2 c/B) | 5.34 ms (78.2 c/B) | 6.46 ms (94.6 c/B) | 6.96 ms (102.0 c/B) | - |
| xchacha20poly1305/…/32B | 2.09 ms (15706.3 c/B) | - | 160.38 µs (1202.8 c/B) | 2.18 ms (16327.9 c/B) | - |
| xchacha20poly1305/…/1350B | 2.81 ms (500.4 c/B) | - | 684.11 µs (121.6 c/B) | 3.13 ms (555.9 c/B) | - |
| xchacha20poly1305/…/16KB | 8.90 ms (130.3 c/B) | - | 6.57 ms (96.2 c/B) | 9.02 ms (132.1 c/B) | - |

## QUIC header protection

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| quic-header-protection/aes128/… | 24.19 µs (5,807 c) | 37.32 µs (8,958 c) | - | - | - |
| quic-header-protection/aes256/… | 33.13 µs (7,951 c) | 51.34 µs (12 kc) | - | - | - |
| quic-header-protection/chacha20/… | 9.37 µs (2,248 c) | 10.83 µs (2,598 c) | - | - | - |

## Key agreement

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| x25519-ecdh/… | 30.20 ms (7,249 kc) | 696.50 ms (167,160 kc) | 30.01 ms (7,203 kc) | 2803.46 ms (672,831 kc) | - |
| p256-ecdh/… | 62.35 ms (14,964 kc) | 1383.38 ms (332,010 kc) | 88.47 ms (21,234 kc) | 3142.50 ms (754,200 kc) | - |

## Signatures

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| ed25519-sign/… | 24.86 ms (5,965 kc) | 28.36 ms (6,807 kc) | 33.01 ms (7,922 kc) | 2070.39 ms (496,894 kc) | - |
| ed25519-verify/… | 30.86 ms (7,406 kc) | 646.16 ms (155,079 kc) | 31.28 ms (7,506 kc) | 2621.86 ms (629,247 kc) | - |
| p256-ecdsa-sign/… | 37.26 ms (8,943 kc) | 325.71 ms (78,171 kc) | 34.47 ms (8,272 kc) | 982.46 ms (235,790 kc) | - |
| p256-ecdsa-verify/… | 53.13 ms (12,751 kc) | 1381.26 ms (331,503 kc) | 76.59 ms (18,382 kc) | 2605.28 ms (625,267 kc) | - |

## Hashes and KDFs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| sha256/…/64B | 300.48 µs (1126.8 c/B) | 53.96 µs (202.4 c/B) | 257.12 µs (964.2 c/B) | 45.22 µs (169.6 c/B) | 27.24 µs (102.2 c/B) |
| sha256/…/1350B | 812.71 µs (144.5 c/B) | 544.55 µs (96.8 c/B) | 694.00 µs (123.4 c/B) | 467.10 µs (83.0 c/B) | 42.94 µs (7.6 c/B) |
| sha256/…/16KB | 5.77 ms (84.5 c/B) | 6.31 ms (92.4 c/B) | 5.82 ms (85.2 c/B) | 5.34 ms (78.3 c/B) | 185.75 µs (2.7 c/B) |
| sha512/…/64B | 6.43 ms (24105.1 c/B) | 102.65 µs (385.0 c/B) | 6.49 ms (24350.8 c/B) | 6.38 ms (23910.7 c/B) | - |
| sha512/…/1350B | 67.46 ms (11992.4 c/B) | 1.06 ms (189.3 c/B) | 69.54 ms (12362.8 c/B) | 67.33 ms (11969.6 c/B) | - |
| sha512/…/16KB | 786.59 ms (11522.3 c/B) | 12.42 ms (181.9 c/B) | 812.89 ms (11907.5 c/B) | 786.46 ms (11520.4 c/B) | - |
| hmac-sha256/…/64B | 928.40 µs (3481.5 c/B) | 140.37 µs (526.4 c/B) | 564.99 µs (2118.7 c/B) | 190.21 µs (713.3 c/B) | - |
| hmac-sha256/…/1350B | 1.42 ms (251.6 c/B) | 630.95 µs (112.2 c/B) | 1.05 ms (186.3 c/B) | 618.65 µs (110.0 c/B) | - |
| hkdf-sha256/extract-expand/… | 1.97 ms (473 kc) | 231.04 µs (55 kc) | 3.99 ms (959 kc) | 1.46 ms (349 kc) | - |
| hkdf-sha256/expand/… | 636.79 µs (153 kc) | 60.60 µs (15 kc) | 88.05 µs (21 kc) | 882.80 µs (212 kc) | - |

## ML-KEM

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| mlkem768-keygen/… | 13.36 ms (3,207 kc) | - | 15.20 ms (3,647 kc) | 21.59 ms (5,181 kc) | - |
| mlkem768-encaps/… | 14.16 ms (3,398 kc) | - | 17.83 ms (4,279 kc) | 34.74 ms (8,338 kc) | - |
| mlkem768-decaps/… | 6.19 ms (1,486 kc) | - | 20.28 ms (4,868 kc) | 47.37 ms (11,368 kc) | - |

## Known gaps

Every operation where another Rust library beats rind by more than 10% here, with the decision on it, as rind's `benchmarks/gaps.toml` requires; `tools/report.py` refuses a gap without an entry in `results/gaps.toml`, and an entry that covers no gap.

| Operations | rind's time over the fastest | Fastest | Decision |
|---|---|---|---|
| `sha512/…/1350B`, `sha512/…/16KB`, `sha512/…/64B` | 62.62× to 63.36× | ring | Not compute: rind's portable SHA-512 (libcrux-sha2, the same code as the libcrux row, which rind matches within 1%) and RustCrypto's are fully unrolled 64-bit code, about 22,000 Xtensa instructions per block, far past the 16 KB (or 32 KB) instruction cache, so every block streams its code from DIO flash: about 1.5 M cycles a block, which is the flash's bandwidth. ring's C is a small loop that stays cached. A 32 KB cache changed nothing. A fix would be a rolled compression loop on targets without 64-bit registers, or placing it in IRAM. |
| `sha256/…/1350B`, `sha256/…/64B` | 1.74× to 6.65× | libcrux | Instruction-cache misses: with ESP-IDF's default 16 KB instruction cache, rind's one-shot SHA-256 path (its wrapper over libcrux-sha2, plus padding and wiping) misses the cache on every call, a fixed ~250 µs. Rebuilt with a 32 KB cache (sdkconfig.icache32, a diagnostic run in results/diag), sha256/rind/64B went from 300 µs to 46 µs, 1.06x libcrux's 43 µs, and 1350B to 459 µs, 1.01x libcrux. The per-block cost matches libcrux's at 16 KB too (16KB rows within 10%). |
| `hkdf-sha256/expand/…`, `hkdf-sha256/extract-expand/…`, `hmac-sha256/…/1350B`, `hmac-sha256/…/64B` | 2.29× to 10.51× | libcrux, ring | The SHA-256 instruction-cache misses above, paid on each of HMAC's inner and outer hashes and HKDF's blocks. With a 32 KB instruction cache (results/diag/hmac-icache32, hkdf-icache32) rind is the fastest: hmac-sha256 64B 116 µs against ring's 140 µs, 1350B 529 µs against libcrux's 529 µs and ring's 631 µs, hkdf-sha256 expand 50 µs against RustCrypto's 49 µs and ring's 61 µs, extract-expand 193 µs against ring's 231 µs. |
| `chacha20poly1305/…/1350B`, `chacha20poly1305/…/16KB`, `chacha20poly1305/…/32B` | 1.27× to 1.49× | ring | Compute, in Poly1305: rind's portable backend uses libcrux's HACL Poly1305, whose 64-bit limbs and u64 multiplies are emulated on 32-bit Xtensa, while ring's C Poly1305 uses 32x32->64 products (mul32x32_64). A FreeRTOS tick PC profile of the 16KB seal (results/diag/esp32s3-chacha-profile, 400 samples each) puts 58% of rind's 6.77 ms in Poly1305 (about 3.9 ms) against 32% of ring's 5.34 ms (about 1.7 ms); rind's ChaCha20 (about 2.7 ms) is a little faster than ring's (about 3.2 ms). A 32 KB instruction cache leaves the 1350B and 16KB gaps unchanged (results/diag/chacha20poly1305-icache32: 16KB 6710 µs against ring's 5336 µs). At 32B the cache also costs rind (55.3 µs, 53.7 µs with 32 KB) and open is slower than seal (93.6 µs against ring's 38.4 µs, 54.1 µs with 32 KB). A fix would be a 32-bit radix-2^26 Poly1305 on targets without 64-bit multiplies. |
| `xchacha20poly1305/…/1350B`, `xchacha20poly1305/…/16KB`, `xchacha20poly1305/…/32B` | 1.35× to 13.06× | rustcrypto | Instruction-cache misses: rind's and libcrux's XChaCha20-Poly1305 seal draws its nonce from the RNG and walks more code than ESP-IDF's default 16 KB instruction cache holds, a fixed ~2 ms a call. Rebuilt with a 32 KB cache (results/diag/xchacha20poly1305-icache32) rind is 1.10x RustCrypto at 32B (176.0 µs against 160.4 µs), 1.05x at 1350B and 1.04x at 16KB. Not the RNG: rng-diag counts about 27k cycles for rind::rand::fill(24 B) and 36k cycles of RNG inside a 600k-cycle seal (results/diag/esp32s3-xchacha-rngdiag), and a seeded SplitMix64 RNG gives the same overhead (results/diag/esp32s3-xchacha-seeded). The 16KB residue past the cache is the Poly1305 gap above. |

## Known-answer checks

Run on the board before any timing, every run. A library failing an operation's check is not raced there.

| Check | Library | Result |
|---|---|---|
| aes128-gcm/1350B-agrees-with-rind | esp32s3-hw | pass (6 runs) |
| aes128-gcm/1350B-agrees-with-rind | libcrux | pass (6 runs) |
| aes128-gcm/1350B-agrees-with-rind | ring | pass (6 runs) |
| aes128-gcm/1350B-agrees-with-rind | rustcrypto | pass (6 runs) |
| aes128-gcm/nist-gcm-tc2 | esp32s3-hw | pass (6 runs) |
| aes128-gcm/nist-gcm-tc2 | libcrux | pass (6 runs) |
| aes128-gcm/nist-gcm-tc2 | rind | pass (6 runs) |
| aes128-gcm/nist-gcm-tc2 | ring | pass (6 runs) |
| aes128-gcm/nist-gcm-tc2 | rustcrypto | pass (6 runs) |
| chacha20poly1305/1350B-agrees-with-rind | libcrux | pass (9 runs) |
| chacha20poly1305/1350B-agrees-with-rind | ring | pass (9 runs) |
| chacha20poly1305/1350B-agrees-with-rind | rustcrypto | pass (9 runs) |
| chacha20poly1305/rfc8439-2.8.2 | libcrux | pass (9 runs) |
| chacha20poly1305/rfc8439-2.8.2 | rind | pass (9 runs) |
| chacha20poly1305/rfc8439-2.8.2 | ring | pass (9 runs) |
| chacha20poly1305/rfc8439-2.8.2 | rustcrypto | pass (9 runs) |
| ed25519/rfc8032-test1 | dalek | pass (3 runs) |
| ed25519/rfc8032-test1 | libcrux | pass (3 runs) |
| ed25519/rfc8032-test1 | rind | pass (3 runs) |
| ed25519/rfc8032-test1 | ring | pass (3 runs) |
| hmac-sha256/rfc4231-tc2 | libcrux | pass (6 runs) |
| hmac-sha256/rfc4231-tc2 | rind | pass (6 runs) |
| hmac-sha256/rfc4231-tc2 | ring | pass (6 runs) |
| hmac-sha256/rfc4231-tc2 | rustcrypto | pass (6 runs) |
| mlkem768/round-trip | libcrux-ml-kem | pass (3 runs) |
| mlkem768/round-trip | rind | pass (3 runs) |
| mlkem768/round-trip | rustcrypto | pass (3 runs) |
| p256-ecdh/agrees-with-rind | ring | pass (3 runs) |
| p256-ecdh/rfc5903-8.1 | libcrux | pass (3 runs) |
| p256-ecdh/rfc5903-8.1 | rind | pass (3 runs) |
| p256-ecdh/rfc5903-8.1 | rustcrypto | pass (3 runs) |
| sha256/16KB-agrees-with-rind | esp32s3-hw | pass (3 runs) |
| sha256/16KB-agrees-with-rind | libcrux | pass (3 runs) |
| sha256/16KB-agrees-with-rind | ring | pass (3 runs) |
| sha256/16KB-agrees-with-rind | rustcrypto | pass (3 runs) |
| sha256/fips180-abc | esp32s3-hw | pass (3 runs) |
| sha256/fips180-abc | libcrux | pass (3 runs) |
| sha256/fips180-abc | rind | pass (3 runs) |
| sha256/fips180-abc | ring | pass (3 runs) |
| sha256/fips180-abc | rustcrypto | pass (3 runs) |
| sha512/fips180-abc | libcrux | pass (3 runs) |
| sha512/fips180-abc | rind | pass (3 runs) |
| sha512/fips180-abc | ring | pass (3 runs) |
| sha512/fips180-abc | rustcrypto | pass (3 runs) |
| x25519/agrees-with-rind | ring | pass (3 runs) |
| x25519/rfc7748-6.1 | dalek | pass (3 runs) |
| x25519/rfc7748-6.1 | libcrux | pass (3 runs) |
| x25519/rfc7748-6.1 | rind | pass (3 runs) |

## Code size

Bytes of code and read-only data, summed over symbols across every image, each counted once, by the crate its demangled name starts with. `shared` is curve25519-dalek, fiat-crypto and libcrux-hacl-rs/sha2/poly1305: rind's portable backend and the dalek or libcrux rows both link them. Generic core/alloc code instantiated for a library counts for core, so each is a floor. `esp32s3-hw` is ESP-IDF's mbedTLS AES/GCM/SHA wrappers around the accelerators.

| Library | Bytes of linked code and read-only data |
|---|---|
| libcrux | 1,152,290 |
| shared | 406,395 |
| rind | 338,864 |
| rustcrypto | 260,468 |
| ring | 143,933 |
| esp32s3-hw | 13,114 |

## Confidence

Each run is a fresh boot of the same image. With 3 runs the lowest and highest run medians bound the board's median with probability 75% (distribution-free: each run median falls on either side with probability one half); `ns_low` and `ns_high` in `esp32s3.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run noise only. Relinking an image moves its short-input rows far more than that: see `diag/layout-relink-2026-11-12/`, and read short-input ratios with the README's layout caveat.

The `legacy rind-bench` column and the `legacy_rind_bench` field are provenance from the private `steelhead99x/rind-bench` history that built these images. That repository was archived on 2026-11-12 and published as [`steelhead99x/rind-hardware`](https://github.com/steelhead99x/rind-hardware) (squash root `8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3`). Each image's `rind_hardware` field is the public commit when the trees match (the RP2040 set maps to `8d9c1bb`), or `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3` when the recorded tree is earlier in the squashed history. The SHA recorded at build time is in `legacy_rind_bench`. See `tools/legacy-rind-bench-map.json`. The Python tools in the commands below produced this set and have since been replaced by `cargo xtask`. To rebuild today, follow the current RESULTS.md using `rind-hardware`.

## Reproduce

```sh
git clone --recursive https://github.com/steelhead99x/rind-hardware   # then called rind-bench; public as rind-hardware
git clone https://github.com/steelhead99x/rind-results
(cd rind-results && cargo xtask setup-toolchain ../rind-hardware)   # espup, espflash, ldproxy, at these versions
cd rind-hardware && scripts/apply-patches.sh && scripts/vendor-ring.sh
(cd esp32 && ./build-images.sh)   # images/<bench>.elf; check sha256 against images.sha256
cd ../rind-results && PORT=/dev/serial/by-id/usb-Espressif_... tools/run-all.sh ../rind-hardware/esp32/images
python3 tools/report.py --raw results/raw --rind ../rind-hardware/rind --firmware ../rind-hardware/esp32 --sizes results/sizes.json --images results/images.json --meta results/meta-esp32s3.json --out results
python3 tools/verify.py   # recomputes every published number from raw
```

Generated by `tools/report.py` on 2026-11-12; every benchmark, with each run's statistics, is in `esp32s3-all.json`.
