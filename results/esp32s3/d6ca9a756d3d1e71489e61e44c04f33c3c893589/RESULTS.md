# rind on an ESP32-S3

rind-bench's own benches, run unchanged but for the aws-lc-rs rows on one core of an ESP32-S3 at 240 MHz, timed with the core's cycle counter. Every operation and ID is `admin/bench-report`'s.

- rind: upstream `d6ca9a756d3d1e71489e61e44c04f33c3c893589`, fork `eb053e787735b1898fa64ebc91ae13b31db663ec`, plus the port's patches: `0001-fix-portable-build-MD5-on-Xtensa-whose-inline-assemb.patch` (15c8a88b7d4c), `0002-feat-bench-leave-aws-lc-rs-rows-out-on-Xtensa-and-ba.patch` (d3cb2092d5c3), `0003-feat-build-for-bare-metal-targets-without-std.patch` (3a6367cec7b3), `0004-feat-bench-leave-aws-lc-rs-rows-out-of-sha3-on-Xtensa.patch` (e82273bb3c4f), `0005-feat-reuse-cache-without-CAS-on-targets-that-lack-it.patch` (e40d8eee2902), `ring-0.17.14-xtensa-poly1305-align.patch` (f86f74e81d34)
- Firmware images (one per bench file; `images.sha256` beside this file):

  | Image | sha256 | rind | rind-hardware | legacy rind-bench | Built |
  |---|---|---|---|---|---|
  | aes_gcm | `8874eb80fef4ec50…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | chacha20poly1305 | `59697eae444a0bcc…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | cipher | `3cdca70c80ac7db8…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | ed25519 | `2b89589be03b86e1…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | hkdf | `b814ee5bdba4fd5b…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:07 PST |
  | hmac | `c82ae9f82730cfb1…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:07 PST |
  | mlkem768 | `1a47eec924dcf041…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:07 PST |
  | p256 | `8bf18693d763f559…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | sha2 | `cf69b85b333bb5b1…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | sha3 | `4117e16436fdc813…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | x25519 | `58a77cdc6b078e79…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |
  | xchacha20poly1305 | `e6b64fff5102615b…` | `eb053e78` | ancestor of `8d9c1bb` | rind | 2026-11-13 07:06 PST |

- OS: ESP-IDF v5.5.1 (FreeRTOS), bare board
- SPI flash clock, as the bootloader reported it each boot: 80 MHz
- Toolchain: rustc 1.97.0-nightly (8ea53bcd7 2026-07-08) (esp 1.97.0.0); xtensa-esp-elf GCC 14.2.0 (esp-14.2.0_20241119)
- Runs: 36 of 36 completed, one firmware image per rind-bench bench file
  - aes_gcm run 1: ok, 173 results, 2026-11-13T15:27:13+00:00 to 2026-11-13T15:28:38+00:00 (UTC)
  - aes_gcm run 2: ok, 173 results, 2026-11-13T15:28:38+00:00 to 2026-11-13T15:30:04+00:00 (UTC)
  - aes_gcm run 3: ok, 173 results, 2026-11-13T15:30:04+00:00 to 2026-11-13T15:31:29+00:00 (UTC)
  - chacha20poly1305 run 1: ok, 58 results, 2026-11-13T15:31:34+00:00 to 2026-11-13T15:31:56+00:00 (UTC)
  - chacha20poly1305 run 2: ok, 58 results, 2026-11-13T15:31:56+00:00 to 2026-11-13T15:32:18+00:00 (UTC)
  - chacha20poly1305 run 3: ok, 58 results, 2026-11-13T15:32:18+00:00 to 2026-11-13T15:32:40+00:00 (UTC)
  - cipher run 1: ok, 44 results, 2026-11-13T15:32:45+00:00 to 2026-11-13T15:33:11+00:00 (UTC)
  - cipher run 2: ok, 44 results, 2026-11-13T15:33:11+00:00 to 2026-11-13T15:33:36+00:00 (UTC)
  - cipher run 3: ok, 44 results, 2026-11-13T15:33:36+00:00 to 2026-11-13T15:34:01+00:00 (UTC)
  - ed25519 run 1: ok, 14 results, 2026-11-13T15:34:11+00:00 to 2026-11-13T15:35:04+00:00 (UTC)
  - ed25519 run 2: ok, 14 results, 2026-11-13T15:35:04+00:00 to 2026-11-13T15:35:57+00:00 (UTC)
  - ed25519 run 3: ok, 14 results, 2026-11-13T15:35:57+00:00 to 2026-11-13T15:36:51+00:00 (UTC)
  - hkdf run 1: ok, 40 results, 2026-11-13T15:36:57+00:00 to 2026-11-13T15:37:22+00:00 (UTC)
  - hkdf run 2: ok, 40 results, 2026-11-13T15:37:22+00:00 to 2026-11-13T15:37:47+00:00 (UTC)
  - hkdf run 3: ok, 40 results, 2026-11-13T15:37:47+00:00 to 2026-11-13T15:38:12+00:00 (UTC)
  - hmac run 1: ok, 104 results, 2026-11-13T15:38:17+00:00 to 2026-11-13T15:39:44+00:00 (UTC)
  - hmac run 2: ok, 104 results, 2026-11-13T15:39:44+00:00 to 2026-11-13T15:41:10+00:00 (UTC)
  - hmac run 3: ok, 104 results, 2026-11-13T15:41:10+00:00 to 2026-11-13T15:42:37+00:00 (UTC)
  - mlkem768 run 1: ok, 14 results, 2026-11-13T15:42:44+00:00 to 2026-11-13T15:43:01+00:00 (UTC)
  - mlkem768 run 2: ok, 14 results, 2026-11-13T15:43:01+00:00 to 2026-11-13T15:43:17+00:00 (UTC)
  - mlkem768 run 3: ok, 14 results, 2026-11-13T15:43:17+00:00 to 2026-11-13T15:43:34+00:00 (UTC)
  - p256 run 1: ok, 23 results, 2026-11-13T15:43:45+00:00 to 2026-11-13T15:45:23+00:00 (UTC)
  - p256 run 2: ok, 23 results, 2026-11-13T15:45:23+00:00 to 2026-11-13T15:47:01+00:00 (UTC)
  - p256 run 3: ok, 23 results, 2026-11-13T15:47:01+00:00 to 2026-11-13T15:48:40+00:00 (UTC)
  - sha2 run 1: ok, 76 results, 2026-11-13T15:48:45+00:00 to 2026-11-13T15:50:05+00:00 (UTC)
  - sha2 run 2: ok, 76 results, 2026-11-13T15:50:05+00:00 to 2026-11-13T15:51:26+00:00 (UTC)
  - sha2 run 3: ok, 76 results, 2026-11-13T15:51:26+00:00 to 2026-11-13T15:52:46+00:00 (UTC)
  - sha3 run 1: ok, 56 results, 2026-11-13T15:52:51+00:00 to 2026-11-13T15:53:23+00:00 (UTC)
  - sha3 run 2: ok, 56 results, 2026-11-13T15:53:23+00:00 to 2026-11-13T15:53:54+00:00 (UTC)
  - sha3 run 3: ok, 56 results, 2026-11-13T15:53:54+00:00 to 2026-11-13T15:54:26+00:00 (UTC)
  - x25519 run 1: ok, 11 results, 2026-11-13T15:54:35+00:00 to 2026-11-13T15:55:16+00:00 (UTC)
  - x25519 run 2: ok, 11 results, 2026-11-13T15:55:16+00:00 to 2026-11-13T15:55:57+00:00 (UTC)
  - x25519 run 3: ok, 11 results, 2026-11-13T15:55:57+00:00 to 2026-11-13T15:56:38+00:00 (UTC)
  - xchacha20poly1305 run 1: ok, 23 results, 2026-11-13T15:56:42+00:00 to 2026-11-13T15:56:52+00:00 (UTC)
  - xchacha20poly1305 run 2: ok, 23 results, 2026-11-13T15:56:52+00:00 to 2026-11-13T15:57:02+00:00 (UTC)
  - xchacha20poly1305 run 3: ok, 23 results, 2026-11-13T15:57:02+00:00 to 2026-11-13T15:57:12+00:00 (UTC)

Each cell is the median over runs of each run's median time per operation, with cycles per byte for byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. `esp32s3-hw` is ESP-IDF's mbedTLS on the chip's AES and SHA accelerators: hardware, not a Rust library. rind runs its portable Rust backend here (it has no Xtensa assembly).

## Harness checks

- Cycle counter against an independent wall clock (ESP-IDF's SYSTIMER, from the 40 MHz crystal) over a busy second at each boot (36 boots): 240.0002 to 240.0007 MHz.
- Over each benchmark's samples (1908 results), the counter timed 99.0% to 100.00% of the wall-clock window (the rest is setup and bookkeeping between samples); 0 results exceed 100.1%, which a wrapped, skipped or double-counted interval would show.
- An empty `iter` routine costs 17.0 cycles per iteration (median over 36 runs): the loop, `black_box` and, for `iter_batched`, the moved input. It is in every result, unsubtracted, as in criterion; against the fastest row here it is 0.76%.
- An empty `iter_batched` routine costs 9.0 cycles per iteration (median over 36 runs): the loop, `black_box` and, for `iter_batched`, the moved input. It is in every result, unsubtracted, as in criterion; against the fastest row here it is 0.40%.

## Headline: rind against the fastest other Rust library

| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |
|---|---|---|---|---|
| aes128-gcm/…/32B | 89.59 µs (671.9 c/B) | ring: 117.19 µs (878.9 c/B) | 0.76× | 0.0% |
| aes128-gcm/…/1350B | 1.73 ms (308.1 c/B) | ring: 2.02 ms (358.7 c/B) | 0.86× | 0.0% |
| aes128-gcm/…/16KB | 19.98 ms (292.6 c/B) | rustcrypto: 22.87 ms (335.1 c/B) | 0.87× | 0.0% |
| aes128-gcm-open/…/32B | 89.17 µs (668.8 c/B) | ring: 117.38 µs (880.3 c/B) | 0.76× | 0.0% |
| aes128-gcm-open/…/1350B | 1.72 ms (305.2 c/B) | ring: 2.01 ms (357.7 c/B) | 0.85× | 0.0% |
| aes128-gcm-open/…/16KB | 20.00 ms (292.9 c/B) | rustcrypto: 22.94 ms (336.0 c/B) | 0.87× | 0.0% |
| aes256-gcm/…/32B | 107.50 µs (806.3 c/B) | ring: 144.68 µs (1085.1 c/B) | 0.74× | 0.0% |
| aes256-gcm/…/1350B | 2.11 ms (375.1 c/B) | ring: 2.53 ms (450.6 c/B) | 0.83× | 0.5% |
| aes256-gcm/…/16KB | 24.48 ms (358.6 c/B) | rustcrypto: 26.81 ms (392.7 c/B) | 0.91× | 0.0% |
| aes256-gcm-open/…/32B | 106.89 µs (801.7 c/B) | ring: 145.41 µs (1090.6 c/B) | 0.74× | 0.0% |
| aes256-gcm-open/…/1350B | 2.10 ms (374.0 c/B) | ring: 2.52 ms (448.2 c/B) | 0.83× | 0.0% |
| aes256-gcm-open/…/16KB | 24.50 ms (359.0 c/B) | rustcrypto: 26.95 ms (394.8 c/B) | 0.91× | 0.0% |
| chacha20poly1305/…/32B | 53.64 µs (402.3 c/B) | ring: 37.05 µs (277.9 c/B) | 1.45× | 0.0% |
| chacha20poly1305/…/1350B | 596.12 µs (106.0 c/B) | ring: 470.04 µs (83.6 c/B) | 1.27× | 0.0% |
| chacha20poly1305/…/16KB | 6.71 ms (98.3 c/B) | ring: 5.34 ms (78.2 c/B) | 1.26× | 0.0% |
| xchacha20poly1305/…/32B | 1.25 ms (9371.0 c/B) | rustcrypto: 160.37 µs (1202.8 c/B) | 7.79× | 0.0% |
| xchacha20poly1305/…/1350B | 1.86 ms (331.5 c/B) | rustcrypto: 684.11 µs (121.6 c/B) | 2.73× | 0.0% |
| xchacha20poly1305/…/16KB | 7.98 ms (116.8 c/B) | rustcrypto: 6.57 ms (96.2 c/B) | 1.21× | 0.0% |
| quic-header-protection/aes128/… | 24.20 µs (5,809 c) | ring: 37.35 µs (8,963 c) | 0.65× | 0.0% |
| quic-header-protection/aes256/… | 33.14 µs (7,953 c) | ring: 51.36 µs (12 kc) | 0.65× | 0.0% |
| quic-header-protection/chacha20/… | 9.38 µs (2,250 c) | ring: 10.85 µs (2,603 c) | 0.86× | 0.0% |
| x25519-ecdh/… | 28.34 ms (6,803 kc) | rustcrypto: 28.26 ms (6,784 kc) | 1.00× | 0.0% |
| p256-ecdh/… | 49.37 ms (11,849 kc) | rustcrypto: 84.54 ms (20,289 kc) | 0.58× | 0.0% |
| ed25519-sign/… | 15.86 ms (3,807 kc) | rustcrypto: 20.12 ms (4,828 kc) | 0.79× | 0.0% |
| ed25519-verify/… | 25.11 ms (6,027 kc) | rustcrypto: 25.32 ms (6,077 kc) | 0.99× | 0.0% |
| p256-ecdsa-sign/… | 24.90 ms (5,976 kc) | rustcrypto: 27.39 ms (6,573 kc) | 0.91× | 0.0% |
| p256-ecdsa-verify/… | 43.72 ms (10,493 kc) | rustcrypto: 72.82 ms (17,476 kc) | 0.60× | 0.0% |
| sha256/…/64B | 87.98 µs (329.9 c/B) | libcrux: 43.85 µs (164.4 c/B) | 2.01× | 0.0% |
| sha256/…/1350B | 532.78 µs (94.7 c/B) | libcrux: 460.76 µs (81.9 c/B) | 1.16× | 0.0% |
| sha256/…/16KB | 5.44 ms (79.7 c/B) | libcrux: 5.33 ms (78.1 c/B) | 1.02× | 0.0% |
| sha512/…/64B | 3.34 ms (12528.7 c/B) | ring: 102.66 µs (385.0 c/B) | 32.54× | 0.0% |
| sha512/…/1350B | 35.09 ms (6238.1 c/B) | ring: 1.06 ms (189.3 c/B) | 32.96× | 0.0% |
| sha512/…/16KB | 409.16 ms (5993.6 c/B) | ring: 12.42 ms (181.9 c/B) | 32.95× | 0.0% |
| sha3-256/…/64B | 89.24 µs (334.7 c/B) | libcrux: 88.01 µs (330.0 c/B) | 1.01× | 0.0% |
| sha3-256/…/1350B | 857.96 µs (152.5 c/B) | libcrux: 891.30 µs (158.5 c/B) | 0.96× | 0.0% |
| sha3-256/…/16KB | 10.31 ms (151.0 c/B) | libcrux: 10.80 ms (158.2 c/B) | 0.95× | 0.0% |
| sha3-512/…/64B | 89.13 µs (334.2 c/B) | libcrux: 83.30 µs (312.4 c/B) | 1.07× | 0.0% |
| sha3-512/…/1350B | 1.60 ms (285.2 c/B) | libcrux: 1.81 ms (321.4 c/B) | 0.89× | 0.0% |
| sha3-512/…/16KB | 19.19 ms (281.2 c/B) | libcrux: 21.84 ms (319.9 c/B) | 0.88× | 0.0% |
| hmac-sha256/…/64B | 379.61 µs (1423.5 c/B) | ring: 140.35 µs (526.3 c/B) | 2.70× | 0.0% |
| hmac-sha256/…/1350B | 840.63 µs (149.4 c/B) | libcrux: 607.45 µs (108.0 c/B) | 1.38× | 0.0% |
| hkdf-sha256/extract-expand/… | 1.14 ms (274 kc) | ring: 231.01 µs (55 kc) | 4.93× | 0.2% |
| hkdf-sha256/expand/… | 358.37 µs (86 kc) | ring: 60.57 µs (15 kc) | 5.92× | 0.0% |
| mlkem768-keygen/… | 7.07 ms (1,697 kc) | rustcrypto: 13.82 ms (3,317 kc) | 0.51× | 0.0% |
| mlkem768-encaps/… | 7.62 ms (1,828 kc) | rustcrypto: 16.05 ms (3,852 kc) | 0.47× | 1.4% |
| mlkem768-decaps/… | 5.05 ms (1,212 kc) | rustcrypto: 18.29 ms (4,391 kc) | 0.28× | 0.1% |

## AEADs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| aes128-gcm/…/32B | 89.59 µs (671.9 c/B) | 117.19 µs (878.9 c/B) | 250.26 µs (1877.0 c/B) | 340.61 µs (2554.5 c/B) | 75.12 µs (563.4 c/B) |
| aes128-gcm/…/1350B | 1.73 ms (308.1 c/B) | 2.02 ms (358.7 c/B) | 2.29 ms (406.9 c/B) | 5.70 ms (1014.1 c/B) | 430.31 µs (76.5 c/B) |
| aes128-gcm/…/16KB | 19.98 ms (292.6 c/B) | 23.19 ms (339.7 c/B) | 22.87 ms (335.1 c/B) | 65.68 ms (962.1 c/B) | 4.43 ms (64.9 c/B) |
| aes128-gcm-open/…/32B | 89.17 µs (668.8 c/B) | 117.38 µs (880.3 c/B) | 251.89 µs (1889.2 c/B) | 463.96 µs (3479.7 c/B) | - |
| aes128-gcm-open/…/1350B | 1.72 ms (305.2 c/B) | 2.01 ms (357.7 c/B) | 2.33 ms (413.9 c/B) | 5.85 ms (1039.3 c/B) | - |
| aes128-gcm-open/…/16KB | 20.00 ms (292.9 c/B) | 23.23 ms (340.2 c/B) | 22.94 ms (336.0 c/B) | 65.80 ms (963.9 c/B) | - |
| aes256-gcm/…/32B | 107.50 µs (806.3 c/B) | 144.68 µs (1085.1 c/B) | 218.61 µs (1639.5 c/B) | 457.78 µs (3433.4 c/B) | - |
| aes256-gcm/…/1350B | 2.11 ms (375.1 c/B) | 2.53 ms (450.6 c/B) | 2.58 ms (458.7 c/B) | 6.82 ms (1212.6 c/B) | - |
| aes256-gcm/…/16KB | 24.48 ms (358.6 c/B) | 29.06 ms (425.7 c/B) | 26.81 ms (392.7 c/B) | 76.54 ms (1121.2 c/B) | - |
| aes256-gcm-open/…/32B | 106.89 µs (801.7 c/B) | 145.41 µs (1090.6 c/B) | 337.45 µs (2530.9 c/B) | 792.92 µs (5946.9 c/B) | - |
| aes256-gcm-open/…/1350B | 2.10 ms (374.0 c/B) | 2.52 ms (448.2 c/B) | 2.73 ms (485.8 c/B) | 7.03 ms (1250.1 c/B) | - |
| aes256-gcm-open/…/16KB | 24.50 ms (359.0 c/B) | 29.09 ms (426.1 c/B) | 26.95 ms (394.8 c/B) | 76.82 ms (1125.3 c/B) | - |
| chacha20poly1305/…/32B | 53.64 µs (402.3 c/B) | 37.05 µs (277.9 c/B) | 39.07 µs (293.0 c/B) | 50.44 µs (378.3 c/B) | - |
| chacha20poly1305/…/1350B | 596.12 µs (106.0 c/B) | 470.04 µs (83.6 c/B) | 563.49 µs (100.2 c/B) | 692.85 µs (123.2 c/B) | - |
| chacha20poly1305/…/16KB | 6.71 ms (98.3 c/B) | 5.34 ms (78.2 c/B) | 6.46 ms (94.6 c/B) | 6.93 ms (101.5 c/B) | - |
| xchacha20poly1305/…/32B | 1.25 ms (9371.0 c/B) | - | 160.37 µs (1202.8 c/B) | 1.25 ms (9366.7 c/B) | - |
| xchacha20poly1305/…/1350B | 1.86 ms (331.5 c/B) | - | 684.11 µs (121.6 c/B) | 1.98 ms (351.9 c/B) | - |
| xchacha20poly1305/…/16KB | 7.98 ms (116.8 c/B) | - | 6.57 ms (96.2 c/B) | 8.10 ms (118.6 c/B) | - |

## QUIC header protection

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| quic-header-protection/aes128/… | 24.20 µs (5,809 c) | 37.35 µs (8,963 c) | - | - | - |
| quic-header-protection/aes256/… | 33.14 µs (7,953 c) | 51.36 µs (12 kc) | - | - | - |
| quic-header-protection/chacha20/… | 9.38 µs (2,250 c) | 10.85 µs (2,603 c) | - | - | - |

## Key agreement

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| x25519-ecdh/… | 28.34 ms (6,803 kc) | 372.33 ms (89,360 kc) | 28.26 ms (6,784 kc) | 1461.62 ms (350,789 kc) | - |
| p256-ecdh/… | 49.37 ms (11,849 kc) | 729.84 ms (175,162 kc) | 84.54 ms (20,289 kc) | 1646.69 ms (395,205 kc) | - |

## Signatures

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| ed25519-sign/… | 15.86 ms (3,807 kc) | 21.10 ms (5,065 kc) | 20.12 ms (4,828 kc) | 1078.96 ms (258,951 kc) | - |
| ed25519-verify/… | 25.11 ms (6,027 kc) | 339.78 ms (81,548 kc) | 25.32 ms (6,077 kc) | 1367.07 ms (328,097 kc) | - |
| p256-ecdsa-sign/… | 24.90 ms (5,976 kc) | 174.28 ms (41,827 kc) | 27.39 ms (6,573 kc) | 517.39 ms (124,175 kc) | - |
| p256-ecdsa-verify/… | 43.72 ms (10,493 kc) | 727.25 ms (174,540 kc) | 72.82 ms (17,476 kc) | 1367.16 ms (328,118 kc) | - |

## Hashes and KDFs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| sha256/…/64B | 87.98 µs (329.9 c/B) | 53.97 µs (202.4 c/B) | 46.92 µs (175.9 c/B) | 43.85 µs (164.4 c/B) | 27.25 µs (102.2 c/B) |
| sha256/…/1350B | 532.78 µs (94.7 c/B) | 544.55 µs (96.8 c/B) | 490.88 µs (87.3 c/B) | 460.76 µs (81.9 c/B) | 42.95 µs (7.6 c/B) |
| sha256/…/16KB | 5.44 ms (79.7 c/B) | 6.31 ms (92.4 c/B) | 5.64 ms (82.6 c/B) | 5.33 ms (78.1 c/B) | 185.76 µs (2.7 c/B) |
| sha512/…/64B | 3.34 ms (12528.7 c/B) | 102.66 µs (385.0 c/B) | 3.38 ms (12675.2 c/B) | 3.30 ms (12393.6 c/B) | - |
| sha512/…/1350B | 35.09 ms (6238.1 c/B) | 1.06 ms (189.3 c/B) | 36.17 ms (6430.1 c/B) | 35.01 ms (6224.8 c/B) | - |
| sha512/…/16KB | 409.16 ms (5993.6 c/B) | 12.42 ms (181.9 c/B) | 422.75 ms (6192.7 c/B) | 409.10 ms (5992.6 c/B) | - |
| sha3-256/…/64B | 89.24 µs (334.7 c/B) | - | 157.14 µs (589.3 c/B) | 88.01 µs (330.0 c/B) | - |
| sha3-256/…/1350B | 857.96 µs (152.5 c/B) | - | 1.55 ms (275.1 c/B) | 891.30 µs (158.5 c/B) | - |
| sha3-256/…/16KB | 10.31 ms (151.0 c/B) | - | 18.66 ms (273.4 c/B) | 10.80 ms (158.2 c/B) | - |
| sha3-512/…/64B | 89.13 µs (334.2 c/B) | - | 157.66 µs (591.2 c/B) | 83.30 µs (312.4 c/B) | - |
| sha3-512/…/1350B | 1.60 ms (285.2 c/B) | - | 2.91 ms (518.0 c/B) | 1.81 ms (321.4 c/B) | - |
| sha3-512/…/16KB | 19.19 ms (281.2 c/B) | - | 34.93 ms (511.6 c/B) | 21.84 ms (319.9 c/B) | - |
| hmac-sha256/…/64B | 379.61 µs (1423.5 c/B) | 140.35 µs (526.3 c/B) | 200.19 µs (750.7 c/B) | 175.73 µs (659.0 c/B) | - |
| hmac-sha256/…/1350B | 840.63 µs (149.4 c/B) | 630.92 µs (112.2 c/B) | 659.53 µs (117.2 c/B) | 607.45 µs (108.0 c/B) | - |
| hkdf-sha256/extract-expand/… | 1.14 ms (274 kc) | 231.01 µs (55 kc) | 2.20 ms (528 kc) | 757.74 µs (182 kc) | - |
| hkdf-sha256/expand/… | 358.37 µs (86 kc) | 60.57 µs (15 kc) | 213.50 µs (51 kc) | 518.54 µs (124 kc) | - |

## ML-KEM

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| mlkem768-keygen/… | 7.07 ms (1,697 kc) | - | 13.82 ms (3,317 kc) | 14.15 ms (3,396 kc) | - |
| mlkem768-encaps/… | 7.62 ms (1,828 kc) | - | 16.05 ms (3,852 kc) | 21.23 ms (5,096 kc) | - |
| mlkem768-decaps/… | 5.05 ms (1,212 kc) | - | 18.29 ms (4,391 kc) | 28.22 ms (6,773 kc) | - |

## Known gaps

Every operation where another Rust library beats rind by more than 10% here, with the decision on it, as rind's `benchmarks/gaps.toml` requires; `cargo xtask report` refuses a gap without an entry in `results/gaps.toml`, and an entry that covers no gap.

| Operations | rind's time over the fastest | Fastest | Decision |
|---|---|---|---|
| `sha512/…/1350B`, `sha512/…/16KB`, `sha512/…/64B` | 32.54× to 32.96× | ring | Staging only, not a published decision. On this comparable pin (upstream d6ca9a75 plus hardware patches), SHA-512 is about 32.5–33.0× ring at 64B, 1350B and 16KB: the portable unrolled 64-bit HACL* path still streams from flash on a 16 KB I-cache. Same family as the historical 98568332 set. |
| `sha256/…/1350B`, `sha256/…/64B` | 1.16× to 2.01× | libcrux | Staging only, not a published decision. One-shot SHA-256 is 2.01× libcrux at 64B and 1.16× at 1350B on this run: 16 KB I-cache layout cost on the long HACL* body, as previously measured. |
| `hkdf-sha256/expand/…`, `hkdf-sha256/extract-expand/…`, `hmac-sha256/…/1350B`, `hmac-sha256/…/64B` | 1.38× to 5.92× | libcrux, ring | Staging only, not a published decision. This pin is upstream plus hardware patches, not PR #69's rolled/hybrid SHA-256. hmac-sha256 64B is 2.70× ring; 1350B is 1.38× libcrux; hkdf-sha256 expand is 5.92× ring and extract-expand 4.93× ring. |
| `chacha20poly1305/…/1350B`, `chacha20poly1305/…/16KB`, `chacha20poly1305/…/32B` | 1.26× to 1.45× | ring | Staging only, not a published decision. ChaCha20-Poly1305 is 1.45×, 1.27× and 1.26× ring at 32B, 1350B and 16KB: the historical Poly1305 64-bit-limb cost on Xtensa. Optional poly1305-32 was not in these default images. |
| `xchacha20poly1305/…/1350B`, `xchacha20poly1305/…/16KB`, `xchacha20poly1305/…/32B` | 1.21× to 7.79× | rustcrypto | Staging only, not a published decision. XChaCha20-Poly1305 is 7.79×, 2.73× and 1.21× RustCrypto at 32B, 1350B and 16KB: 16 KB I-cache misses on the unrolled HChaCha plus the Poly1305 cost. This pin does not include PR #69's rolled HChaCha or poly1305-32. |

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

## Confidence

Each run is a fresh boot of the same image. With 3 runs the lowest and highest run medians bound the board's median with probability 75% (distribution-free: each run median falls on either side with probability one half); `ns_low` and `ns_high` in `esp32s3.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run noise only. Relinking an image moves its short-input rows far more than that: see `diag/layout-relink-2026-11-12/` and, at this pin, the three-layout table in `diag/esp32s3-relink-fixed-98568332/relink-short.md`; read short-input ratios with the README's layout caveat.

## Reproduce

```sh
git clone --recursive https://github.com/steelhead99x/rind-hardware   # the firmware port
git clone https://github.com/steelhead99x/rind-results
(cd rind-results && cargo xtask setup-toolchain ../rind-hardware)   # espup, espflash, ldproxy, at these versions
cd rind-hardware && cargo xtask apply-patches && cargo xtask vendor-ring
cargo xtask build-images esp   # esp32/images/<bench>.elf; check sha256 against images.sha256
cd ../rind-results && PORT=/dev/serial/by-id/usb-Espressif_... cargo xtask run-all ../rind-hardware/esp32/images
cargo xtask report --raw results/raw --rind ../rind-hardware/rind --firmware ../rind-hardware/esp32 --sizes results/sizes.json --images results/images.json --meta results/meta-esp32s3.json --out results
cargo verify   # recomputes every published number from raw
```

The published images were built from `steelhead99x/rind-bench` (private, now archived).              The firmware repo is now the public [steelhead99x/rind-hardware](https://github.com/steelhead99x/rind-hardware);              its first commit (`8d9c1bb`) is a squash of that history. Each image's `rind_hardware` field              is the public commit when the trees match (the RP2040 set maps to `8d9c1bb`), or              `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3` when the recorded tree is earlier              in the squashed history. The SHA recorded at build time is in `legacy_rind_bench`.              See `tools/legacy-rind-bench-map.json`. Build new images from rind-hardware.

Generated by `cargo xtask report` on 2026-11-13; every benchmark, with each run's statistics, is in `esp32s3-all.json`.
