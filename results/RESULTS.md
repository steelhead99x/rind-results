# rind on an ESP32-S3

rind-bench's own benches, run unchanged but for the aws-lc-rs rows on one core of an ESP32-S3 at 240 MHz, timed with the core's cycle counter. Every operation and ID is `admin/bench-report`'s.

- rind: upstream `985683329fb98aebfaae6c493f205f20070a8fca`, plus the port's patches: `0001-fix-portable-build-MD5-on-Xtensa-whose-inline-assemb.patch`, `0002-feat-bench-leave-aws-lc-rs-rows-out-on-Xtensa-and-ba.patch`, `0003-feat-build-for-bare-metal-targets-without-std.patch`, `0004-feat-bench-leave-aws-lc-rs-rows-out-of-sha3-on-Xtensa.patch`, `ring-0.17.14-xtensa-poly1305-align.patch`
- Firmware images (one per bench file; `images.sha256` beside this file):

  | Image | sha256 | rind | rind-bench | Built |
  |---|---|---|---|---|
  | aes_gcm | `1771b27e2a4a75d0…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:45 PST |
  | chacha20poly1305 | `1331c65f878b3dca…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:45 PST |
  | cipher | `e00623454a2c1261…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:46 PST |
  | ed25519 | `1aeb17c0979a346b…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:47 PST |
  | hkdf | `673e2a41db3e73e1…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:48 PST |
  | hmac | `00a8fcc2cfbdec50…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:47 PST |
  | mlkem768 | `3047ddbe6a0bb25c…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:48 PST |
  | p256 | `148ecb56e10bca06…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:47 PST |
  | sha2 | `71f44e761c67c486…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:47 PST |
  | sha3 | `99e9d1506224a2d3…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:47 PST |
  | x25519 | `cff9f01e8f94eb40…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:46 PST |
  | xchacha20poly1305 | `d10fe0547dc40eef…` | `98568332` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:45 PST |

- OS: ESP-IDF v5.5.1 (FreeRTOS), bare board
- SPI flash clock, as the bootloader reported it each boot: 80 MHz
- Toolchain: rustc 1.97.0-nightly (8ea53bcd7 2026-07-08) (esp 1.97.0.0); xtensa-esp-elf GCC 14.2.0 (esp-14.2.0_20241119)
- Runs: 36 of 36 completed, one firmware image per rind-bench bench file
  - aes_gcm run 1: ok, 173 results, 2026-11-12T20:01:15+00:00 to 2026-11-12T20:02:40+00:00 (UTC)
  - aes_gcm run 2: ok, 173 results, 2026-11-12T20:02:40+00:00 to 2026-11-12T20:04:05+00:00 (UTC)
  - aes_gcm run 3: ok, 173 results, 2026-11-12T20:04:05+00:00 to 2026-11-12T20:05:30+00:00 (UTC)
  - chacha20poly1305 run 1: ok, 58 results, 2026-11-12T20:05:35+00:00 to 2026-11-12T20:05:57+00:00 (UTC)
  - chacha20poly1305 run 2: ok, 58 results, 2026-11-12T20:05:57+00:00 to 2026-11-12T20:06:19+00:00 (UTC)
  - chacha20poly1305 run 3: ok, 58 results, 2026-11-12T20:06:19+00:00 to 2026-11-12T20:06:41+00:00 (UTC)
  - cipher run 1: ok, 44 results, 2026-11-12T20:06:48+00:00 to 2026-11-12T20:07:14+00:00 (UTC)
  - cipher run 2: ok, 44 results, 2026-11-12T20:07:14+00:00 to 2026-11-12T20:07:39+00:00 (UTC)
  - cipher run 3: ok, 44 results, 2026-11-12T20:07:39+00:00 to 2026-11-12T20:08:05+00:00 (UTC)
  - ed25519 run 1: ok, 14 results, 2026-11-12T20:08:14+00:00 to 2026-11-12T20:09:07+00:00 (UTC)
  - ed25519 run 2: ok, 14 results, 2026-11-12T20:09:07+00:00 to 2026-11-12T20:10:01+00:00 (UTC)
  - ed25519 run 3: ok, 14 results, 2026-11-12T20:10:01+00:00 to 2026-11-12T20:10:54+00:00 (UTC)
  - hkdf run 1: ok, 40 results, 2026-11-12T20:11:00+00:00 to 2026-11-12T20:11:25+00:00 (UTC)
  - hkdf run 2: ok, 40 results, 2026-11-12T20:11:25+00:00 to 2026-11-12T20:11:49+00:00 (UTC)
  - hkdf run 3: ok, 40 results, 2026-11-12T20:11:49+00:00 to 2026-11-12T20:12:14+00:00 (UTC)
  - hmac run 1: ok, 104 results, 2026-11-12T20:12:19+00:00 to 2026-11-12T20:13:46+00:00 (UTC)
  - hmac run 2: ok, 104 results, 2026-11-12T20:13:46+00:00 to 2026-11-12T20:15:12+00:00 (UTC)
  - hmac run 3: ok, 104 results, 2026-11-12T20:15:12+00:00 to 2026-11-12T20:16:39+00:00 (UTC)
  - mlkem768 run 1: ok, 14 results, 2026-11-12T20:16:46+00:00 to 2026-11-12T20:17:02+00:00 (UTC)
  - mlkem768 run 2: ok, 14 results, 2026-11-12T20:17:02+00:00 to 2026-11-12T20:17:19+00:00 (UTC)
  - mlkem768 run 3: ok, 14 results, 2026-11-12T20:17:19+00:00 to 2026-11-12T20:17:36+00:00 (UTC)
  - p256 run 1: ok, 23 results, 2026-11-12T20:17:47+00:00 to 2026-11-12T20:19:25+00:00 (UTC)
  - p256 run 2: ok, 23 results, 2026-11-12T20:19:25+00:00 to 2026-11-12T20:21:03+00:00 (UTC)
  - p256 run 3: ok, 23 results, 2026-11-12T20:21:03+00:00 to 2026-11-12T20:22:42+00:00 (UTC)
  - sha2 run 1: ok, 76 results, 2026-11-12T20:22:47+00:00 to 2026-11-12T20:24:07+00:00 (UTC)
  - sha2 run 2: ok, 76 results, 2026-11-12T20:24:07+00:00 to 2026-11-12T20:25:28+00:00 (UTC)
  - sha2 run 3: ok, 76 results, 2026-11-12T20:25:28+00:00 to 2026-11-12T20:26:48+00:00 (UTC)
  - sha3 run 1: ok, 56 results, 2026-11-12T20:26:52+00:00 to 2026-11-12T20:27:24+00:00 (UTC)
  - sha3 run 2: ok, 56 results, 2026-11-12T20:27:24+00:00 to 2026-11-12T20:27:56+00:00 (UTC)
  - sha3 run 3: ok, 56 results, 2026-11-12T20:27:56+00:00 to 2026-11-12T20:28:28+00:00 (UTC)
  - x25519 run 1: ok, 11 results, 2026-11-12T20:28:35+00:00 to 2026-11-12T20:29:16+00:00 (UTC)
  - x25519 run 2: ok, 11 results, 2026-11-12T20:29:16+00:00 to 2026-11-12T20:29:57+00:00 (UTC)
  - x25519 run 3: ok, 11 results, 2026-11-12T20:29:57+00:00 to 2026-11-12T20:30:38+00:00 (UTC)
  - xchacha20poly1305 run 1: ok, 23 results, 2026-11-12T20:30:43+00:00 to 2026-11-12T20:30:53+00:00 (UTC)
  - xchacha20poly1305 run 2: ok, 23 results, 2026-11-12T20:30:53+00:00 to 2026-11-12T20:31:03+00:00 (UTC)
  - xchacha20poly1305 run 3: ok, 23 results, 2026-11-12T20:31:03+00:00 to 2026-11-12T20:31:12+00:00 (UTC)

Each cell is the median over runs of each run's median time per operation, with cycles per byte for byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. `esp32s3-hw` is ESP-IDF's mbedTLS on the chip's AES and SHA accelerators: hardware, not a Rust library. rind runs its portable Rust backend here (it has no Xtensa assembly).

## Harness checks

- Cycle counter against an independent wall clock (ESP-IDF's SYSTIMER, from the 40 MHz crystal) over a busy second at each boot (36 boots): 240.0000 to 240.0007 MHz.
- Over each benchmark's samples (1908 results), the counter timed 98.9% to 100.00% of the wall-clock window (the rest is setup and bookkeeping between samples); 0 results exceed 100.1%, which a wrapped, skipped or double-counted interval would show.
- An empty `iter` routine costs 17.0 cycles per iteration (median over 36 runs): the loop, `black_box` and, for `iter_batched`, the moved input. It is in every result, unsubtracted, as in criterion; against the fastest row here it is 0.76%.
- An empty `iter_batched` routine costs 9.0 cycles per iteration (median over 36 runs): the loop, `black_box` and, for `iter_batched`, the moved input. It is in every result, unsubtracted, as in criterion; against the fastest row here it is 0.40%.

## Headline: rind against the fastest other Rust library

| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |
|---|---|---|---|---|
| aes128-gcm/…/32B | 88.65 µs (664.9 c/B) | rustcrypto: 167.46 µs (1256.0 c/B) | 0.53× | 0.0% |
| aes128-gcm/…/1350B | 1.71 ms (304.4 c/B) | ring: 2.08 ms (370.4 c/B) | 0.82× | 0.0% |
| aes128-gcm/…/16KB | 19.96 ms (292.3 c/B) | rustcrypto: 22.79 ms (333.9 c/B) | 0.88× | 0.0% |
| aes128-gcm-open/…/32B | 89.11 µs (668.4 c/B) | ring: 134.59 µs (1009.4 c/B) | 0.66× | 0.0% |
| aes128-gcm-open/…/1350B | 1.75 ms (310.6 c/B) | ring: 2.04 ms (363.3 c/B) | 0.85× | 0.0% |
| aes128-gcm-open/…/16KB | 20.00 ms (292.9 c/B) | rustcrypto: 22.89 ms (335.3 c/B) | 0.87× | 0.0% |
| aes256-gcm/…/32B | 106.37 µs (797.8 c/B) | ring: 188.95 µs (1417.1 c/B) | 0.56× | 0.0% |
| aes256-gcm/…/1350B | 2.10 ms (373.2 c/B) | ring: 2.58 ms (458.9 c/B) | 0.81× | 0.0% |
| aes256-gcm/…/16KB | 24.46 ms (358.3 c/B) | rustcrypto: 26.88 ms (393.7 c/B) | 0.91× | 0.0% |
| aes256-gcm-open/…/32B | 106.83 µs (801.2 c/B) | ring: 162.64 µs (1219.8 c/B) | 0.66× | 0.0% |
| aes256-gcm-open/…/1350B | 2.13 ms (379.3 c/B) | ring: 2.55 ms (453.5 c/B) | 0.84× | 0.0% |
| aes256-gcm-open/…/16KB | 24.50 ms (359.0 c/B) | rustcrypto: 27.05 ms (396.2 c/B) | 0.91× | 0.0% |
| chacha20poly1305/…/32B | 53.66 µs (402.5 c/B) | ring: 37.05 µs (277.9 c/B) | 1.45× | 0.0% |
| chacha20poly1305/…/1350B | 596.35 µs (106.0 c/B) | ring: 470.04 µs (83.6 c/B) | 1.27× | 0.0% |
| chacha20poly1305/…/16KB | 6.71 ms (98.3 c/B) | ring: 5.34 ms (78.2 c/B) | 1.26× | 0.0% |
| xchacha20poly1305/…/32B | 1.20 ms (9019.8 c/B) | rustcrypto: 160.37 µs (1202.8 c/B) | 7.50× | 0.0% |
| xchacha20poly1305/…/1350B | 1.86 ms (330.2 c/B) | rustcrypto: 684.08 µs (121.6 c/B) | 2.72× | 0.0% |
| xchacha20poly1305/…/16KB | 7.97 ms (116.8 c/B) | rustcrypto: 6.57 ms (96.2 c/B) | 1.21× | 0.0% |
| quic-header-protection/aes128/… | 24.20 µs (5,809 c) | ring: 37.35 µs (8,963 c) | 0.65× | 0.0% |
| quic-header-protection/aes256/… | 33.14 µs (7,953 c) | ring: 51.36 µs (12 kc) | 0.65× | 0.0% |
| quic-header-protection/chacha20/… | 9.38 µs (2,250 c) | ring: 10.85 µs (2,603 c) | 0.86× | 0.0% |
| x25519-ecdh/… | 28.29 ms (6,790 kc) | rustcrypto: 28.27 ms (6,786 kc) | 1.00× | 0.0% |
| p256-ecdh/… | 49.34 ms (11,841 kc) | rustcrypto: 84.29 ms (20,230 kc) | 0.59× | 0.0% |
| ed25519-sign/… | 15.86 ms (3,807 kc) | rustcrypto: 20.11 ms (4,827 kc) | 0.79× | 0.0% |
| ed25519-verify/… | 25.11 ms (6,027 kc) | rustcrypto: 25.32 ms (6,077 kc) | 0.99× | 0.0% |
| p256-ecdsa-sign/… | 25.98 ms (6,235 kc) | rustcrypto: 26.75 ms (6,421 kc) | 0.97× | 0.0% |
| p256-ecdsa-verify/… | 43.82 ms (10,518 kc) | rustcrypto: 72.78 ms (17,468 kc) | 0.60× | 0.0% |
| sha256/…/64B | 103.65 µs (388.7 c/B) | libcrux: 43.85 µs (164.4 c/B) | 2.36× | 0.0% |
| sha256/…/1350B | 538.19 µs (95.7 c/B) | libcrux: 460.76 µs (81.9 c/B) | 1.17× | 0.0% |
| sha256/…/16KB | 5.42 ms (79.4 c/B) | libcrux: 5.33 ms (78.1 c/B) | 1.02× | 0.1% |
| sha512/…/64B | 3.34 ms (12534.5 c/B) | ring: 102.66 µs (385.0 c/B) | 32.56× | 0.0% |
| sha512/…/1350B | 35.09 ms (6238.9 c/B) | ring: 1.06 ms (189.3 c/B) | 32.96× | 0.0% |
| sha512/…/16KB | 409.17 ms (5993.6 c/B) | ring: 12.42 ms (181.9 c/B) | 32.95× | 0.0% |
| sha3-256/…/64B | 89.24 µs (334.7 c/B) | libcrux: 88.01 µs (330.0 c/B) | 1.01× | 0.0% |
| sha3-256/…/1350B | 857.96 µs (152.5 c/B) | libcrux: 891.30 µs (158.5 c/B) | 0.96× | 0.0% |
| sha3-256/…/16KB | 10.31 ms (151.0 c/B) | libcrux: 10.80 ms (158.2 c/B) | 0.95× | 0.0% |
| sha3-512/…/64B | 89.13 µs (334.2 c/B) | libcrux: 83.30 µs (312.4 c/B) | 1.07× | 0.0% |
| sha3-512/…/1350B | 1.60 ms (285.2 c/B) | libcrux: 1.81 ms (321.4 c/B) | 0.89× | 0.0% |
| sha3-512/…/16KB | 19.19 ms (281.2 c/B) | libcrux: 21.84 ms (319.9 c/B) | 0.88× | 0.0% |
| hmac-sha256/…/64B | 308.95 µs (1158.5 c/B) | ring: 140.35 µs (526.3 c/B) | 2.20× | 0.0% |
| hmac-sha256/…/1350B | 745.96 µs (132.6 c/B) | libcrux: 607.45 µs (108.0 c/B) | 1.23× | 0.0% |
| hkdf-sha256/extract-expand/… | 1.27 ms (305 kc) | ring: 231.01 µs (55 kc) | 5.50× | 0.0% |
| hkdf-sha256/expand/… | 419.60 µs (101 kc) | ring: 60.57 µs (15 kc) | 6.93× | 0.0% |
| mlkem768-keygen/… | 7.00 ms (1,681 kc) | rustcrypto: 13.81 ms (3,315 kc) | 0.51× | 0.0% |
| mlkem768-encaps/… | 7.61 ms (1,827 kc) | rustcrypto: 16.05 ms (3,852 kc) | 0.47× | 0.0% |
| mlkem768-decaps/… | 5.20 ms (1,249 kc) | rustcrypto: 18.31 ms (4,393 kc) | 0.28× | 0.1% |

## AEADs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| aes128-gcm/…/32B | 88.65 µs (664.9 c/B) | 179.49 µs (1346.2 c/B) | 167.46 µs (1256.0 c/B) | 320.68 µs (2405.1 c/B) | 75.12 µs (563.4 c/B) |
| aes128-gcm/…/1350B | 1.71 ms (304.4 c/B) | 2.08 ms (370.4 c/B) | 2.18 ms (387.8 c/B) | 5.65 ms (1003.9 c/B) | 430.31 µs (76.5 c/B) |
| aes128-gcm/…/16KB | 19.96 ms (292.3 c/B) | 23.26 ms (340.8 c/B) | 22.79 ms (333.9 c/B) | 65.63 ms (961.3 c/B) | 4.43 ms (64.9 c/B) |
| aes128-gcm-open/…/32B | 89.11 µs (668.4 c/B) | 134.59 µs (1009.4 c/B) | 220.78 µs (1655.8 c/B) | 437.97 µs (3284.8 c/B) | - |
| aes128-gcm-open/…/1350B | 1.75 ms (310.6 c/B) | 2.04 ms (363.3 c/B) | 2.27 ms (403.1 c/B) | 5.76 ms (1023.4 c/B) | - |
| aes128-gcm-open/…/16KB | 20.00 ms (292.9 c/B) | 23.24 ms (340.5 c/B) | 22.89 ms (335.3 c/B) | 65.75 ms (963.2 c/B) | - |
| aes256-gcm/…/32B | 106.37 µs (797.8 c/B) | 188.95 µs (1417.1 c/B) | 259.40 µs (1945.5 c/B) | 624.59 µs (4684.4 c/B) | - |
| aes256-gcm/…/1350B | 2.10 ms (373.2 c/B) | 2.58 ms (458.9 c/B) | 2.61 ms (464.8 c/B) | 6.90 ms (1227.2 c/B) | - |
| aes256-gcm/…/16KB | 24.46 ms (358.3 c/B) | 29.10 ms (426.2 c/B) | 26.88 ms (393.7 c/B) | 76.63 ms (1122.5 c/B) | - |
| aes256-gcm-open/…/32B | 106.83 µs (801.2 c/B) | 162.64 µs (1219.8 c/B) | 409.57 µs (3071.8 c/B) | 810.41 µs (6078.1 c/B) | - |
| aes256-gcm-open/…/1350B | 2.13 ms (379.3 c/B) | 2.55 ms (453.5 c/B) | 2.73 ms (486.2 c/B) | 7.04 ms (1252.1 c/B) | - |
| aes256-gcm-open/…/16KB | 24.50 ms (359.0 c/B) | 29.10 ms (426.3 c/B) | 27.05 ms (396.2 c/B) | 76.83 ms (1125.5 c/B) | - |
| chacha20poly1305/…/32B | 53.66 µs (402.5 c/B) | 37.05 µs (277.9 c/B) | 39.07 µs (293.0 c/B) | 50.40 µs (378.0 c/B) | - |
| chacha20poly1305/…/1350B | 596.35 µs (106.0 c/B) | 470.04 µs (83.6 c/B) | 563.49 µs (100.2 c/B) | 710.81 µs (126.4 c/B) | - |
| chacha20poly1305/…/16KB | 6.71 ms (98.3 c/B) | 5.34 ms (78.2 c/B) | 6.46 ms (94.6 c/B) | 6.94 ms (101.6 c/B) | - |
| xchacha20poly1305/…/32B | 1.20 ms (9019.8 c/B) | - | 160.37 µs (1202.8 c/B) | 1.26 ms (9449.6 c/B) | - |
| xchacha20poly1305/…/1350B | 1.86 ms (330.2 c/B) | - | 684.08 µs (121.6 c/B) | 2.00 ms (354.7 c/B) | - |
| xchacha20poly1305/…/16KB | 7.97 ms (116.8 c/B) | - | 6.57 ms (96.2 c/B) | 8.12 ms (119.0 c/B) | - |

## QUIC header protection

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| quic-header-protection/aes128/… | 24.20 µs (5,809 c) | 37.35 µs (8,963 c) | - | - | - |
| quic-header-protection/aes256/… | 33.14 µs (7,953 c) | 51.36 µs (12 kc) | - | - | - |
| quic-header-protection/chacha20/… | 9.38 µs (2,250 c) | 10.85 µs (2,603 c) | - | - | - |

## Key agreement

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| x25519-ecdh/… | 28.29 ms (6,790 kc) | 372.33 ms (89,359 kc) | 28.27 ms (6,786 kc) | 1461.14 ms (350,673 kc) | - |
| p256-ecdh/… | 49.34 ms (11,841 kc) | 731.15 ms (175,475 kc) | 84.29 ms (20,230 kc) | 1647.58 ms (395,420 kc) | - |

## Signatures

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| ed25519-sign/… | 15.86 ms (3,807 kc) | 21.10 ms (5,064 kc) | 20.11 ms (4,827 kc) | 1078.99 ms (258,958 kc) | - |
| ed25519-verify/… | 25.11 ms (6,027 kc) | 339.79 ms (81,549 kc) | 25.32 ms (6,077 kc) | 1367.03 ms (328,086 kc) | - |
| p256-ecdsa-sign/… | 25.98 ms (6,235 kc) | 172.04 ms (41,289 kc) | 26.75 ms (6,421 kc) | 517.78 ms (124,266 kc) | - |
| p256-ecdsa-verify/… | 43.82 ms (10,518 kc) | 728.87 ms (174,928 kc) | 72.78 ms (17,468 kc) | 1367.60 ms (328,223 kc) | - |

## Hashes and KDFs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| sha256/…/64B | 103.65 µs (388.7 c/B) | 53.97 µs (202.4 c/B) | 46.92 µs (175.9 c/B) | 43.85 µs (164.4 c/B) | 27.25 µs (102.2 c/B) |
| sha256/…/1350B | 538.19 µs (95.7 c/B) | 544.55 µs (96.8 c/B) | 490.88 µs (87.3 c/B) | 460.76 µs (81.9 c/B) | 42.95 µs (7.6 c/B) |
| sha256/…/16KB | 5.42 ms (79.4 c/B) | 6.31 ms (92.4 c/B) | 5.64 ms (82.6 c/B) | 5.33 ms (78.1 c/B) | 185.76 µs (2.7 c/B) |
| sha512/…/64B | 3.34 ms (12534.5 c/B) | 102.66 µs (385.0 c/B) | 3.38 ms (12675.4 c/B) | 3.30 ms (12393.7 c/B) | - |
| sha512/…/1350B | 35.09 ms (6238.9 c/B) | 1.06 ms (189.3 c/B) | 36.17 ms (6430.1 c/B) | 35.02 ms (6224.9 c/B) | - |
| sha512/…/16KB | 409.17 ms (5993.6 c/B) | 12.42 ms (181.9 c/B) | 422.76 ms (6192.7 c/B) | 409.10 ms (5992.6 c/B) | - |
| sha3-256/…/64B | 89.24 µs (334.7 c/B) | - | 157.14 µs (589.3 c/B) | 88.01 µs (330.0 c/B) | - |
| sha3-256/…/1350B | 857.96 µs (152.5 c/B) | - | 1.55 ms (275.1 c/B) | 891.30 µs (158.5 c/B) | - |
| sha3-256/…/16KB | 10.31 ms (151.0 c/B) | - | 18.66 ms (273.4 c/B) | 10.80 ms (158.2 c/B) | - |
| sha3-512/…/64B | 89.13 µs (334.2 c/B) | - | 157.66 µs (591.2 c/B) | 83.30 µs (312.4 c/B) | - |
| sha3-512/…/1350B | 1.60 ms (285.2 c/B) | - | 2.91 ms (518.0 c/B) | 1.81 ms (321.4 c/B) | - |
| sha3-512/…/16KB | 19.19 ms (281.2 c/B) | - | 34.93 ms (511.6 c/B) | 21.84 ms (319.9 c/B) | - |
| hmac-sha256/…/64B | 308.95 µs (1158.5 c/B) | 140.35 µs (526.3 c/B) | 200.19 µs (750.7 c/B) | 175.73 µs (659.0 c/B) | - |
| hmac-sha256/…/1350B | 745.96 µs (132.6 c/B) | 630.92 µs (112.2 c/B) | 659.53 µs (117.2 c/B) | 607.45 µs (108.0 c/B) | - |
| hkdf-sha256/extract-expand/… | 1.27 ms (305 kc) | 231.01 µs (55 kc) | 2.18 ms (523 kc) | 879.34 µs (211 kc) | - |
| hkdf-sha256/expand/… | 419.60 µs (101 kc) | 60.57 µs (15 kc) | 101.30 µs (24 kc) | 445.94 µs (107 kc) | - |

## ML-KEM

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| mlkem768-keygen/… | 7.00 ms (1,681 kc) | - | 13.81 ms (3,315 kc) | 14.07 ms (3,378 kc) | - |
| mlkem768-encaps/… | 7.61 ms (1,827 kc) | - | 16.05 ms (3,852 kc) | 21.10 ms (5,064 kc) | - |
| mlkem768-decaps/… | 5.20 ms (1,249 kc) | - | 18.31 ms (4,393 kc) | 28.13 ms (6,751 kc) | - |

## Known gaps

Every operation where another Rust library beats rind by more than 10% here, with the decision on it, as rind's `benchmarks/gaps.toml` requires; `cargo xtask report` refuses a gap without an entry in `results/gaps.toml`, and an entry that covers no gap.

| Operations | rind's time over the fastest | Fastest | Decision |
|---|---|---|---|
| `sha512/…/1350B`, `sha512/…/16KB`, `sha512/…/64B` | 32.56× to 32.96× | ring | Not compute: rind's portable SHA-512 (libcrux-sha2, the same code as the libcrux row, which rind matches within 1%) and RustCrypto's are fully unrolled 64-bit code, about 22,000 Xtensa instructions per block, far past the 16 KB (or 32 KB) instruction cache, so every block streams its code from DIO flash: about 0.80 M cycles a block at the 80 MHz flash clock (1.54 M at 40 MHz, results/diag/flash-80mhz), which is the flash's bandwidth. ring's C is a small loop that stays cached. A 32 KB cache changed nothing. A fix would be a rolled compression loop on targets without 64-bit registers, or placing it in IRAM. |
| `sha256/…/1350B`, `sha256/…/64B` | 1.17× to 2.36× | libcrux | Instruction-cache misses: with ESP-IDF's default 16 KB instruction cache, rind's one-shot SHA-256 path (its wrapper over libcrux-sha2, plus padding and wiping) misses the cache on every call, a fixed ~60 µs here (103.6 µs against libcrux's 43.8 µs at 64B). With a 32 KB cache rind's 64B is 1.06x libcrux (results/diag/esp32s3-hmac-hkdf-repin-2026-11-12/icache32, 80 MHz; sha2-icache32 at 40 MHz agrees). The per-block cost matches libcrux's (16KB within 2%). How large the fixed cost is depends on layout: a 12-byte shift of the code took 64B from 2.36x to 3.55x libcrux (results/diag/esp32s3-relink-98568332). |
| `hkdf-sha256/expand/…`, `hkdf-sha256/extract-expand/…`, `hmac-sha256/…/1350B`, `hmac-sha256/…/64B` | 1.23× to 6.93× | libcrux, ring | The SHA-256 instruction-cache misses above, paid on each of HMAC's inner and outer hashes and HKDF's blocks; the same code computes as fast as anyone's once it fits. With a 32 KB instruction cache rind is the fastest or within 2%: hmac-sha256 64B 115.6 µs against ring's 140.3 µs, hkdf-sha256 expand 50.2 µs against RustCrypto's 49.2 µs and ring's 60.6 µs (results/diag/esp32s3-hmac-hkdf-repin-2026-11-12/icache32, 80 MHz, identical within 0.05% to hmac-icache32 and hkdf-icache32 at rind 99b509a). With 16 KB the misses depend on where the linker puts rind's HMAC code relative to libcrux's 12.6 KB sha256_update, which takes half the PC samples: the repin to 98568332 changed no line of rind's HMAC, HKDF or SHA-256 code and no dependency, and every function on the path kept its size, but other rind code moved rind's HMAC functions 2,644 bytes closer to sha256_update in the hmac image (25,068 in the hkdf image), and hmac-sha256 64B went from 507 µs to 309 µs (3.61x to 2.20x ring) while hkdf-sha256 expand went from 322 µs to 420 µs (5.31x to 6.93x ring). Layout, not a code change (results/diag/esp32s3-hmac-hkdf-repin-2026-11-12). |
| `chacha20poly1305/…/1350B`, `chacha20poly1305/…/16KB`, `chacha20poly1305/…/32B` | 1.26× to 1.45× | ring | Compute, in Poly1305: rind's portable backend uses libcrux's HACL Poly1305, whose 64-bit limbs and u64 multiplies are emulated on 32-bit Xtensa, while ring's C Poly1305 uses 32x32->64 products (mul32x32_64). A FreeRTOS tick PC profile of the 16KB seal (results/diag/esp32s3-chacha-profile, 400 samples each) puts 58% of rind's time in Poly1305 against 32% of ring's; rind's ChaCha20 is a little faster than ring's. Neither doubling the flash clock (16KB 6.72 ms against ring's 5.34 ms, the same as at 40 MHz) nor a 32 KB instruction cache (results/diag/chacha20poly1305-icache32) moves the 1350B and 16KB gaps. At 32B rind is 54.4 µs against ring's 37.1 µs (open 54.6 µs against 38.3 µs). A fix would be a 32-bit radix-2^26 Poly1305 on targets without 64-bit multiplies. |
| `xchacha20poly1305/…/1350B`, `xchacha20poly1305/…/16KB`, `xchacha20poly1305/…/32B` | 1.21× to 7.50× | rustcrypto | Instruction-cache misses: rind's and libcrux's XChaCha20-Poly1305 seal draws its nonce from the RNG and walks more code than ESP-IDF's default 16 KB instruction cache holds, a fixed ~1.0 ms a call here (1.20 ms against RustCrypto's 160 µs at 32B; ~2 ms at the 40 MHz flash clock). With a 32 KB cache (results/diag/xchacha20poly1305-icache32, 40 MHz) rind was 1.10x RustCrypto at 32B, 1.05x at 1350B and 1.04x at 16KB. Not the RNG: rng-diag counts about 27k cycles for rind::rand::fill(24 B) and 36k cycles of RNG inside a 600k-cycle seal (results/diag/esp32s3-xchacha-rngdiag), and a seeded SplitMix64 RNG gives the same overhead (results/diag/esp32s3-xchacha-seeded). The 16KB residue past the cache is the Poly1305 gap above. |

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
| sha3-256/16KB-agrees-with-rind | libcrux | pass (3 runs) |
| sha3-256/16KB-agrees-with-rind | rustcrypto | pass (3 runs) |
| sha3-256/fips202-abc | libcrux | pass (3 runs) |
| sha3-256/fips202-abc | rind | pass (3 runs) |
| sha3-256/fips202-abc | rustcrypto | pass (3 runs) |
| sha3-384/16KB-agrees-with-rind | libcrux | pass (3 runs) |
| sha3-384/16KB-agrees-with-rind | rustcrypto | pass (3 runs) |
| sha3-384/fips202-abc | libcrux | pass (3 runs) |
| sha3-384/fips202-abc | rind | pass (3 runs) |
| sha3-384/fips202-abc | rustcrypto | pass (3 runs) |
| sha3-512/16KB-agrees-with-rind | libcrux | pass (3 runs) |
| sha3-512/16KB-agrees-with-rind | rustcrypto | pass (3 runs) |
| sha3-512/fips202-abc | libcrux | pass (3 runs) |
| sha3-512/fips202-abc | rind | pass (3 runs) |
| sha3-512/fips202-abc | rustcrypto | pass (3 runs) |
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
| libcrux | 1,152,936 |
| shared | 406,395 |
| rind | 327,135 |
| rustcrypto | 261,647 |
| ring | 143,860 |
| esp32s3-hw | 13,114 |

## Confidence

Each run is a fresh boot of the same image. With 3 runs the lowest and highest run medians bound the board's median with probability 75% (distribution-free: each run median falls on either side with probability one half); `ns_low` and `ns_high` in `esp32s3.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run noise only. Relinking an image moves its short-input rows far more than that: see `diag/layout-relink-2026-11-12/` and, at this pin, the three-layout table in `diag/esp32s3-relink-98568332/relink-short.md`; read short-input ratios with the README's layout caveat.

## Reproduce

```sh
git clone --recursive https://github.com/steelhead99x/rind-bench   # private; the port
git clone https://github.com/steelhead99x/rind-results
rind-results/tools/setup-toolchain.sh rind-bench   # espup, espflash, ldproxy, at these versions
cd rind-bench && cargo xtask apply-patches && cargo xtask vendor-ring
cargo xtask build-images esp   # esp32/images/<bench>.elf; check sha256 against images.sha256
cd ../rind-results && PORT=/dev/serial/by-id/usb-Espressif_... cargo xtask run-all ../rind-bench/esp32/images
cargo xtask report --raw results/raw --rind ../rind-bench/rind --firmware ../rind-bench/esp32 --sizes results/sizes.json --images results/images.json --meta results/meta-esp32s3.json --out results
cargo verify   # recomputes every published number from raw
```

## SHA-3

At this pin `admin/bench-report` reports SHA3-256 and SHA3-512, and rind's Keccak rewrite (88ff91fd) closes the gap the 99b509a set recorded outside the report (2.1x libcrux's time at 16KB; `diag/esp32s3-99b509a-80mhz-2026-11-12/`). At 16KB, cycles per byte: SHA3-256 rind 151.0, libcrux 158.2, RustCrypto 273.4; SHA3-384 196.0 / 223.2 / 355.8; SHA3-512 281.2 / 319.9 / 511.6. At 64B rind takes at most 1.07x libcrux's time. For scale, CycloneCRYPTO publishes ~218 c/B (1.10 MB/s) for software SHA3-256 on this chip.

## Short inputs across relinks

`diag/esp32s3-relink-98568332/relink-short.md` runs all twelve benches in two more layouts (rind-bench's `--pad 1031` and `--pad 2053`, three runs each) and gives each short-input row's ratio in all three layouts and from the medians over layouts. Most rows move by under 0.1; rind's one-shot SHA-256 at 64B moves most, 2.36x to 3.55x libcrux's time (2.78x from the layout medians), and the AES-GCM 32B rows 0.53x-0.66x here against 0.58x-0.77x in both other layouts. Read the table above's short-input ratios with those ranges.

Generated by `cargo xtask report` on 2026-11-12; every benchmark, with each run's statistics, is in `esp32s3-all.json`.
