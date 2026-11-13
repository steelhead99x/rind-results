# rind on an ESP32-S3

rind-bench's own benches, run unchanged but for the aws-lc-rs rows on one core of an ESP32-S3 at 240 MHz, timed with the core's cycle counter. Every operation and ID is `admin/bench-report`'s.

- rind: upstream `99b509a680cf0ad0cd8b7d9c03f7360e7fd95cef`, plus the port's patches: `0001-fix-portable-build-MD5-on-Xtensa-whose-inline-assemb.patch`, `0002-feat-bench-leave-aws-lc-rs-rows-out-on-Xtensa-and-ba.patch`, `0003-feat-build-for-bare-metal-targets-without-std.patch`, `0004-feat-bench-leave-aws-lc-rs-rows-out-of-sha3-on-Xtensa.patch`, `ring-0.17.14-xtensa-poly1305-align.patch`
- Firmware images (one per bench file; `images.sha256` beside this file):

  | Image | sha256 | rind | rind-hardware | legacy rind-bench | Built |
  |---|---|---|---|---|---|
  | aes_gcm | `e5235b433188e179…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | chacha20poly1305 | `f581ff142514edea…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | cipher | `7d75475e4e93adb7…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | ed25519 | `81569af99e208f40…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | hkdf | `a2544a9b9b7956b6…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | hmac | `44b5e8bbb69022d0…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | mlkem768 | `bcfd05e275511f45…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | p256 | `0ca6a233201372f8…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | sha2 | `9b352830d35438ff…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | sha3 | `cb776505583c9be1…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | x25519 | `0d5dd95d542a3c7a…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |
  | xchacha20poly1305 | `11af9bb8ea4824df…` | `99b509a6` | ancestor of `8d9c1bb` | 0463305 (2026-11-12 10:35 PST), clean tree | 2026-11-12 10:35-10:38 PST |

- OS: ESP-IDF v5.5.1 (FreeRTOS), bare board
- SPI flash clock, as the bootloader reported it each boot: 80 MHz
- Toolchain: rustc 1.97.0-nightly (8ea53bcd7 2026-07-08) (esp 1.97.0.0); xtensa-esp-elf GCC 14.2.0 (esp-14.2.0_20241119)
- Runs: 36 of 36 completed, one firmware image per rind-bench bench file
  - aes_gcm run 1: ok, 173 results, 2026-11-12T18:38:24+00:00 to 2026-11-12T18:39:49+00:00 (UTC)
  - aes_gcm run 2: ok, 173 results, 2026-11-12T18:39:49+00:00 to 2026-11-12T18:41:14+00:00 (UTC)
  - aes_gcm run 3: ok, 173 results, 2026-11-12T18:41:15+00:00 to 2026-11-12T18:42:40+00:00 (UTC)
  - chacha20poly1305 run 1: ok, 58 results, 2026-11-12T18:42:46+00:00 to 2026-11-12T18:43:09+00:00 (UTC)
  - chacha20poly1305 run 2: ok, 58 results, 2026-11-12T18:43:09+00:00 to 2026-11-12T18:43:31+00:00 (UTC)
  - chacha20poly1305 run 3: ok, 58 results, 2026-11-12T18:43:31+00:00 to 2026-11-12T18:43:54+00:00 (UTC)
  - cipher run 1: ok, 44 results, 2026-11-12T18:43:59+00:00 to 2026-11-12T18:44:25+00:00 (UTC)
  - cipher run 2: ok, 44 results, 2026-11-12T18:44:25+00:00 to 2026-11-12T18:44:50+00:00 (UTC)
  - cipher run 3: ok, 44 results, 2026-11-12T18:44:50+00:00 to 2026-11-12T18:45:16+00:00 (UTC)
  - ed25519 run 1: ok, 14 results, 2026-11-12T18:45:26+00:00 to 2026-11-12T18:46:19+00:00 (UTC)
  - ed25519 run 2: ok, 14 results, 2026-11-12T18:46:19+00:00 to 2026-11-12T18:47:13+00:00 (UTC)
  - ed25519 run 3: ok, 14 results, 2026-11-12T18:47:13+00:00 to 2026-11-12T18:48:07+00:00 (UTC)
  - hkdf run 1: ok, 40 results, 2026-11-12T18:48:14+00:00 to 2026-11-12T18:48:39+00:00 (UTC)
  - hkdf run 2: ok, 40 results, 2026-11-12T18:48:39+00:00 to 2026-11-12T18:49:05+00:00 (UTC)
  - hkdf run 3: ok, 40 results, 2026-11-12T18:49:05+00:00 to 2026-11-12T18:49:30+00:00 (UTC)
  - hmac run 1: ok, 104 results, 2026-11-12T18:49:35+00:00 to 2026-11-12T18:51:02+00:00 (UTC)
  - hmac run 2: ok, 104 results, 2026-11-12T18:51:02+00:00 to 2026-11-12T18:52:28+00:00 (UTC)
  - hmac run 3: ok, 104 results, 2026-11-12T18:52:28+00:00 to 2026-11-12T18:53:55+00:00 (UTC)
  - mlkem768 run 1: ok, 14 results, 2026-11-12T18:54:02+00:00 to 2026-11-12T18:54:20+00:00 (UTC)
  - mlkem768 run 2: ok, 14 results, 2026-11-12T18:54:20+00:00 to 2026-11-12T18:54:39+00:00 (UTC)
  - mlkem768 run 3: ok, 14 results, 2026-11-12T18:54:39+00:00 to 2026-11-12T18:54:57+00:00 (UTC)
  - p256 run 1: ok, 23 results, 2026-11-12T18:55:09+00:00 to 2026-11-12T18:56:48+00:00 (UTC)
  - p256 run 2: ok, 23 results, 2026-11-12T18:56:48+00:00 to 2026-11-12T18:58:27+00:00 (UTC)
  - p256 run 3: ok, 23 results, 2026-11-12T18:58:27+00:00 to 2026-11-12T19:00:06+00:00 (UTC)
  - sha2 run 1: ok, 76 results, 2026-11-12T19:00:10+00:00 to 2026-11-12T19:01:31+00:00 (UTC)
  - sha2 run 2: ok, 76 results, 2026-11-12T19:01:31+00:00 to 2026-11-12T19:02:52+00:00 (UTC)
  - sha2 run 3: ok, 76 results, 2026-11-12T19:02:52+00:00 to 2026-11-12T19:04:13+00:00 (UTC)
  - sha3 run 1: ok, 56 results, 2026-11-12T19:04:17+00:00 to 2026-11-12T19:04:53+00:00 (UTC)
  - sha3 run 2: ok, 56 results, 2026-11-12T19:04:53+00:00 to 2026-11-12T19:05:29+00:00 (UTC)
  - sha3 run 3: ok, 56 results, 2026-11-12T19:05:29+00:00 to 2026-11-12T19:06:05+00:00 (UTC)
  - x25519 run 1: ok, 11 results, 2026-11-12T19:06:13+00:00 to 2026-11-12T19:06:54+00:00 (UTC)
  - x25519 run 2: ok, 11 results, 2026-11-12T19:06:54+00:00 to 2026-11-12T19:07:35+00:00 (UTC)
  - x25519 run 3: ok, 11 results, 2026-11-12T19:07:35+00:00 to 2026-11-12T19:08:16+00:00 (UTC)
  - xchacha20poly1305 run 1: ok, 23 results, 2026-11-12T19:08:21+00:00 to 2026-11-12T19:08:31+00:00 (UTC)
  - xchacha20poly1305 run 2: ok, 23 results, 2026-11-12T19:08:31+00:00 to 2026-11-12T19:08:41+00:00 (UTC)
  - xchacha20poly1305 run 3: ok, 23 results, 2026-11-12T19:08:41+00:00 to 2026-11-12T19:08:51+00:00 (UTC)

Each cell is the median over runs of each run's median time per operation, with cycles per byte for byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. `esp32s3-hw` is ESP-IDF's mbedTLS on the chip's AES and SHA accelerators: hardware, not a Rust library. rind runs its portable Rust backend here (it has no Xtensa assembly).

## Harness checks

- Cycle counter against an independent wall clock (ESP-IDF's SYSTIMER, from the 40 MHz crystal) over a busy second at each boot (36 boots): 240.0000 to 240.0007 MHz.
- Over each benchmark's samples (1908 results), the counter timed 98.6% to 100.00% of the wall-clock window (the rest is setup and bookkeeping between samples); 0 results exceed 100.1%, which a wrapped, skipped or double-counted interval would show.
- An empty `iter` routine costs 17.0 cycles per iteration (median over 36 runs): the loop, `black_box` and, for `iter_batched`, the moved input. It is in every result, unsubtracted, as in criterion; against the fastest row here it is 0.76%.
- An empty `iter_batched` routine costs 9.0 cycles per iteration (median over 36 runs): the loop, `black_box` and, for `iter_batched`, the moved input. It is in every result, unsubtracted, as in criterion; against the fastest row here it is 0.40%.

## Headline: rind against the fastest other Rust library

| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |
|---|---|---|---|---|
| aes128-gcm/…/32B | 88.65 µs (664.9 c/B) | ring: 142.52 µs (1068.9 c/B) | 0.62× | 0.0% |
| aes128-gcm/…/1350B | 1.71 ms (304.4 c/B) | ring: 2.05 ms (364.1 c/B) | 0.84× | 0.0% |
| aes128-gcm/…/16KB | 19.96 ms (292.3 c/B) | rustcrypto: 22.81 ms (334.2 c/B) | 0.87× | 0.0% |
| aes128-gcm-open/…/32B | 89.87 µs (674.0 c/B) | ring: 117.38 µs (880.3 c/B) | 0.77× | 0.0% |
| aes128-gcm-open/…/1350B | 1.73 ms (307.7 c/B) | ring: 2.01 ms (357.7 c/B) | 0.86× | 0.0% |
| aes128-gcm-open/…/16KB | 20.01 ms (293.2 c/B) | rustcrypto: 22.96 ms (336.4 c/B) | 0.87× | 0.0% |
| aes256-gcm/…/32B | 106.37 µs (797.8 c/B) | ring: 159.01 µs (1192.6 c/B) | 0.67× | 0.0% |
| aes256-gcm/…/1350B | 2.10 ms (373.2 c/B) | rustcrypto: 2.55 ms (453.1 c/B) | 0.82× | 0.0% |
| aes256-gcm/…/16KB | 24.46 ms (358.3 c/B) | rustcrypto: 26.85 ms (393.4 c/B) | 0.91× | 0.0% |
| aes256-gcm-open/…/32B | 107.63 µs (807.2 c/B) | ring: 145.41 µs (1090.6 c/B) | 0.74× | 0.0% |
| aes256-gcm-open/…/1350B | 2.12 ms (376.4 c/B) | ring: 2.52 ms (448.2 c/B) | 0.84× | 0.0% |
| aes256-gcm-open/…/16KB | 24.52 ms (359.2 c/B) | rustcrypto: 27.03 ms (396.0 c/B) | 0.91× | 0.0% |
| chacha20poly1305/…/32B | 54.40 µs (408.0 c/B) | ring: 37.05 µs (277.9 c/B) | 1.47× | 0.0% |
| chacha20poly1305/…/1350B | 600.98 µs (106.8 c/B) | ring: 470.04 µs (83.6 c/B) | 1.28× | 0.0% |
| chacha20poly1305/…/16KB | 6.72 ms (98.5 c/B) | ring: 5.34 ms (78.2 c/B) | 1.26× | 0.0% |
| xchacha20poly1305/…/32B | 1.20 ms (9016.7 c/B) | rustcrypto: 160.38 µs (1202.8 c/B) | 7.50× | 0.0% |
| xchacha20poly1305/…/1350B | 1.80 ms (320.3 c/B) | rustcrypto: 684.10 µs (121.6 c/B) | 2.63× | 0.0% |
| xchacha20poly1305/…/16KB | 7.92 ms (116.0 c/B) | rustcrypto: 6.57 ms (96.2 c/B) | 1.20× | 0.0% |
| quic-header-protection/aes128/… | 24.20 µs (5,809 c) | ring: 37.35 µs (8,963 c) | 0.65× | 0.0% |
| quic-header-protection/aes256/… | 33.14 µs (7,953 c) | ring: 51.36 µs (12 kc) | 0.65× | 0.0% |
| quic-header-protection/chacha20/… | 9.38 µs (2,250 c) | ring: 10.85 µs (2,603 c) | 0.86× | 0.0% |
| x25519-ecdh/… | 28.34 ms (6,801 kc) | rustcrypto: 28.23 ms (6,776 kc) | 1.00× | 0.0% |
| p256-ecdh/… | 50.10 ms (12,023 kc) | rustcrypto: 84.33 ms (20,239 kc) | 0.59× | 0.0% |
| ed25519-sign/… | 15.77 ms (3,784 kc) | rustcrypto: 20.03 ms (4,807 kc) | 0.79× | 0.0% |
| ed25519-verify/… | 25.11 ms (6,025 kc) | rustcrypto: 25.31 ms (6,075 kc) | 0.99× | 0.0% |
| p256-ecdsa-sign/… | 24.32 ms (5,837 kc) | rustcrypto: 26.87 ms (6,449 kc) | 0.91× | 0.0% |
| p256-ecdsa-verify/… | 44.36 ms (10,647 kc) | rustcrypto: 72.76 ms (17,463 kc) | 0.61× | 0.0% |
| sha256/…/64B | 100.20 µs (375.7 c/B) | libcrux: 43.83 µs (164.4 c/B) | 2.29× | 0.0% |
| sha256/…/1350B | 567.64 µs (100.9 c/B) | libcrux: 460.65 µs (81.9 c/B) | 1.23× | 0.0% |
| sha256/…/16KB | 5.44 ms (79.6 c/B) | libcrux: 5.33 ms (78.1 c/B) | 1.02× | 0.0% |
| sha512/…/64B | 3.34 ms (12520.2 c/B) | ring: 102.66 µs (385.0 c/B) | 32.52× | 0.0% |
| sha512/…/1350B | 35.09 ms (6238.5 c/B) | ring: 1.06 ms (189.3 c/B) | 32.96× | 0.0% |
| sha512/…/16KB | 409.16 ms (5993.6 c/B) | ring: 12.42 ms (181.9 c/B) | 32.95× | 0.0% |
| hmac-sha256/…/64B | 507.14 µs (1901.8 c/B) | ring: 140.35 µs (526.3 c/B) | 3.61× | 0.0% |
| hmac-sha256/…/1350B | 947.07 µs (168.4 c/B) | libcrux: 604.70 µs (107.5 c/B) | 1.57× | 0.0% |
| hkdf-sha256/extract-expand/… | 1.14 ms (273 kc) | ring: 231.01 µs (55 kc) | 4.92× | 0.0% |
| hkdf-sha256/expand/… | 321.92 µs (77 kc) | ring: 60.57 µs (15 kc) | 5.31× | 0.0% |
| mlkem768-keygen/… | 11.80 ms (2,832 kc) | rustcrypto: 13.82 ms (3,316 kc) | 0.85× | 0.1% |
| mlkem768-encaps/… | 12.49 ms (2,998 kc) | rustcrypto: 16.05 ms (3,851 kc) | 0.78× | 0.0% |
| mlkem768-decaps/… | 6.03 ms (1,448 kc) | rustcrypto: 18.40 ms (4,415 kc) | 0.33× | 0.0% |

## AEADs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| aes128-gcm/…/32B | 88.65 µs (664.9 c/B) | 142.52 µs (1068.9 c/B) | 202.57 µs (1519.3 c/B) | 315.57 µs (2366.8 c/B) | 76.70 µs (575.2 c/B) |
| aes128-gcm/…/1350B | 1.71 ms (304.4 c/B) | 2.05 ms (364.1 c/B) | 2.18 ms (387.7 c/B) | 5.62 ms (999.2 c/B) | 430.31 µs (76.5 c/B) |
| aes128-gcm/…/16KB | 19.96 ms (292.3 c/B) | 23.26 ms (340.7 c/B) | 22.81 ms (334.2 c/B) | 65.62 ms (961.2 c/B) | 4.43 ms (64.9 c/B) |
| aes128-gcm-open/…/32B | 89.87 µs (674.0 c/B) | 117.38 µs (880.3 c/B) | 288.41 µs (2163.1 c/B) | 452.40 µs (3393.0 c/B) | - |
| aes128-gcm-open/…/1350B | 1.73 ms (307.7 c/B) | 2.01 ms (357.7 c/B) | 2.28 ms (405.4 c/B) | 5.76 ms (1023.7 c/B) | - |
| aes128-gcm-open/…/16KB | 20.01 ms (293.2 c/B) | 23.23 ms (340.2 c/B) | 22.96 ms (336.4 c/B) | 65.75 ms (963.2 c/B) | - |
| aes256-gcm/…/32B | 106.37 µs (797.8 c/B) | 159.01 µs (1192.6 c/B) | 260.05 µs (1950.3 c/B) | 603.44 µs (4525.8 c/B) | - |
| aes256-gcm/…/1350B | 2.10 ms (373.2 c/B) | 2.55 ms (453.7 c/B) | 2.55 ms (453.1 c/B) | 6.93 ms (1232.6 c/B) | - |
| aes256-gcm/…/16KB | 24.46 ms (358.3 c/B) | 29.08 ms (426.0 c/B) | 26.85 ms (393.4 c/B) | 76.61 ms (1122.2 c/B) | - |
| aes256-gcm-open/…/32B | 107.63 µs (807.2 c/B) | 145.41 µs (1090.6 c/B) | 448.30 µs (3362.3 c/B) | 793.99 µs (5955.0 c/B) | - |
| aes256-gcm-open/…/1350B | 2.12 ms (376.4 c/B) | 2.52 ms (448.2 c/B) | 2.82 ms (501.1 c/B) | 7.02 ms (1248.1 c/B) | - |
| aes256-gcm-open/…/16KB | 24.52 ms (359.2 c/B) | 29.09 ms (426.1 c/B) | 27.03 ms (396.0 c/B) | 76.80 ms (1125.0 c/B) | - |
| chacha20poly1305/…/32B | 54.40 µs (408.0 c/B) | 37.05 µs (277.9 c/B) | 39.07 µs (293.0 c/B) | 50.45 µs (378.4 c/B) | - |
| chacha20poly1305/…/1350B | 600.98 µs (106.8 c/B) | 470.04 µs (83.6 c/B) | 563.49 µs (100.2 c/B) | 702.94 µs (125.0 c/B) | - |
| chacha20poly1305/…/16KB | 6.72 ms (98.5 c/B) | 5.34 ms (78.2 c/B) | 6.46 ms (94.6 c/B) | 6.94 ms (101.6 c/B) | - |
| xchacha20poly1305/…/32B | 1.20 ms (9016.7 c/B) | - | 160.38 µs (1202.8 c/B) | 1.22 ms (9173.0 c/B) | - |
| xchacha20poly1305/…/1350B | 1.80 ms (320.3 c/B) | - | 684.10 µs (121.6 c/B) | 2.00 ms (356.3 c/B) | - |
| xchacha20poly1305/…/16KB | 7.92 ms (116.0 c/B) | - | 6.57 ms (96.2 c/B) | 8.11 ms (118.8 c/B) | - |

## QUIC header protection

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| quic-header-protection/aes128/… | 24.20 µs (5,809 c) | 37.35 µs (8,963 c) | - | - | - |
| quic-header-protection/aes256/… | 33.14 µs (7,953 c) | 51.36 µs (12 kc) | - | - | - |
| quic-header-protection/chacha20/… | 9.38 µs (2,250 c) | 10.85 µs (2,603 c) | - | - | - |

## Key agreement

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| x25519-ecdh/… | 28.34 ms (6,801 kc) | 372.33 ms (89,360 kc) | 28.23 ms (6,776 kc) | 1464.33 ms (351,438 kc) | - |
| p256-ecdh/… | 50.10 ms (12,023 kc) | 732.89 ms (175,894 kc) | 84.33 ms (20,239 kc) | 1645.86 ms (395,006 kc) | - |

## Signatures

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| ed25519-sign/… | 15.77 ms (3,784 kc) | 21.10 ms (5,065 kc) | 20.03 ms (4,807 kc) | 1078.27 ms (258,786 kc) | - |
| ed25519-verify/… | 25.11 ms (6,025 kc) | 342.50 ms (82,201 kc) | 25.31 ms (6,075 kc) | 1366.09 ms (327,863 kc) | - |
| p256-ecdsa-sign/… | 24.32 ms (5,837 kc) | 170.02 ms (40,804 kc) | 26.87 ms (6,449 kc) | 517.53 ms (124,207 kc) | - |
| p256-ecdsa-verify/… | 44.36 ms (10,647 kc) | 730.92 ms (175,421 kc) | 72.76 ms (17,463 kc) | 1366.99 ms (328,077 kc) | - |

## Hashes and KDFs

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| sha256/…/64B | 100.20 µs (375.7 c/B) | 53.97 µs (202.4 c/B) | 46.91 µs (175.9 c/B) | 43.83 µs (164.4 c/B) | 27.25 µs (102.2 c/B) |
| sha256/…/1350B | 567.64 µs (100.9 c/B) | 544.55 µs (96.8 c/B) | 490.77 µs (87.2 c/B) | 460.65 µs (81.9 c/B) | 42.95 µs (7.6 c/B) |
| sha256/…/16KB | 5.44 ms (79.6 c/B) | 6.31 ms (92.4 c/B) | 5.64 ms (82.6 c/B) | 5.33 ms (78.1 c/B) | 185.76 µs (2.7 c/B) |
| sha512/…/64B | 3.34 ms (12520.2 c/B) | 102.66 µs (385.0 c/B) | 3.38 ms (12667.4 c/B) | 3.30 ms (12393.5 c/B) | - |
| sha512/…/1350B | 35.09 ms (6238.5 c/B) | 1.06 ms (189.3 c/B) | 36.17 ms (6429.7 c/B) | 35.01 ms (6224.9 c/B) | - |
| sha512/…/16KB | 409.16 ms (5993.6 c/B) | 12.42 ms (181.9 c/B) | 422.75 ms (6192.7 c/B) | 409.10 ms (5992.6 c/B) | - |
| hmac-sha256/…/64B | 507.14 µs (1901.8 c/B) | 140.35 µs (526.3 c/B) | 218.60 µs (819.8 c/B) | 172.31 µs (646.1 c/B) | - |
| hmac-sha256/…/1350B | 947.07 µs (168.4 c/B) | 630.92 µs (112.2 c/B) | 678.35 µs (120.6 c/B) | 604.70 µs (107.5 c/B) | - |
| hkdf-sha256/extract-expand/… | 1.14 ms (273 kc) | 231.01 µs (55 kc) | 2.18 ms (523 kc) | 807.22 µs (194 kc) | - |
| hkdf-sha256/expand/… | 321.92 µs (77 kc) | 60.57 µs (15 kc) | 103.61 µs (25 kc) | 474.23 µs (114 kc) | - |

## ML-KEM

| Operation | rind | ring | rustcrypto | libcrux | esp32s3-hw |
|---|---|---|---|---|---|
| mlkem768-keygen/… | 11.80 ms (2,832 kc) | - | 13.82 ms (3,316 kc) | 14.30 ms (3,433 kc) | - |
| mlkem768-encaps/… | 12.49 ms (2,998 kc) | - | 16.05 ms (3,851 kc) | 21.23 ms (5,096 kc) | - |
| mlkem768-decaps/… | 6.03 ms (1,448 kc) | - | 18.40 ms (4,415 kc) | 28.27 ms (6,786 kc) | - |

## Known gaps

Every operation where another Rust library beats rind by more than 10% here, with the decision on it, as rind's `benchmarks/gaps.toml` requires; `tools/report.py` refuses a gap without an entry in `results/gaps.toml`, and an entry that covers no gap.

| Operations | rind's time over the fastest | Fastest | Decision |
|---|---|---|---|
| `sha512/…/1350B`, `sha512/…/16KB`, `sha512/…/64B` | 32.52× to 32.96× | ring | Not compute: rind's portable SHA-512 (libcrux-sha2, the same code as the libcrux row, which rind matches within 1%) and RustCrypto's are fully unrolled 64-bit code, about 22,000 Xtensa instructions per block, far past the 16 KB (or 32 KB) instruction cache, so every block streams its code from DIO flash: about 0.80 M cycles a block at the 80 MHz flash clock (1.54 M at 40 MHz, results/diag/flash-80mhz), which is the flash's bandwidth. ring's C is a small loop that stays cached. A 32 KB cache changed nothing. A fix would be a rolled compression loop on targets without 64-bit registers, or placing it in IRAM. |
| `sha256/…/1350B`, `sha256/…/64B` | 1.23× to 2.29× | libcrux | Instruction-cache misses: with ESP-IDF's default 16 KB instruction cache, rind's one-shot SHA-256 path (its wrapper over libcrux-sha2, plus padding and wiping) misses the cache on every call, a fixed ~56 µs here (100.2 µs against libcrux's 43.8 µs at 64B; ~250 µs at the 40 MHz flash clock). With a 32 KB cache (results/diag/sha2-icache32, 40 MHz) rind's 64B was 1.06x libcrux and 1350B 1.01x. The per-block cost matches libcrux's (16KB within 2%). How large the fixed cost is depends on layout: a relink halved it (results/diag/layout-relink-2026-11-12). |
| `hkdf-sha256/expand/…`, `hkdf-sha256/extract-expand/…`, `hmac-sha256/…/1350B`, `hmac-sha256/…/64B` | 1.57× to 5.31× | libcrux, ring | The SHA-256 instruction-cache misses above, paid on each of HMAC's inner and outer hashes and HKDF's blocks. With a 32 KB instruction cache (results/diag/hmac-icache32, hkdf-icache32, at the 40 MHz flash clock) rind was the fastest: hmac-sha256 64B 116 µs against ring's 140 µs, 1350B 529 µs against libcrux's 529 µs and ring's 631 µs, hkdf-sha256 expand 50 µs against RustCrypto's 49 µs and ring's 61 µs, extract-expand 193 µs against ring's 231 µs. |
| `chacha20poly1305/…/1350B`, `chacha20poly1305/…/16KB`, `chacha20poly1305/…/32B` | 1.26× to 1.47× | ring | Compute, in Poly1305: rind's portable backend uses libcrux's HACL Poly1305, whose 64-bit limbs and u64 multiplies are emulated on 32-bit Xtensa, while ring's C Poly1305 uses 32x32->64 products (mul32x32_64). A FreeRTOS tick PC profile of the 16KB seal (results/diag/esp32s3-chacha-profile, 400 samples each) puts 58% of rind's time in Poly1305 against 32% of ring's; rind's ChaCha20 is a little faster than ring's. Neither doubling the flash clock (16KB 6.72 ms against ring's 5.34 ms, the same as at 40 MHz) nor a 32 KB instruction cache (results/diag/chacha20poly1305-icache32) moves the 1350B and 16KB gaps. At 32B rind is 54.4 µs against ring's 37.1 µs (open 54.6 µs against 38.3 µs). A fix would be a 32-bit radix-2^26 Poly1305 on targets without 64-bit multiplies. |
| `xchacha20poly1305/…/1350B`, `xchacha20poly1305/…/16KB`, `xchacha20poly1305/…/32B` | 1.20× to 7.50× | rustcrypto | Instruction-cache misses: rind's and libcrux's XChaCha20-Poly1305 seal draws its nonce from the RNG and walks more code than ESP-IDF's default 16 KB instruction cache holds, a fixed ~1.0 ms a call here (1.20 ms against RustCrypto's 160 µs at 32B; ~2 ms at the 40 MHz flash clock). With a 32 KB cache (results/diag/xchacha20poly1305-icache32, 40 MHz) rind was 1.10x RustCrypto at 32B, 1.05x at 1350B and 1.04x at 16KB. Not the RNG: rng-diag counts about 27k cycles for rind::rand::fill(24 B) and 36k cycles of RNG inside a 600k-cycle seal (results/diag/esp32s3-xchacha-rngdiag), and a seeded SplitMix64 RNG gives the same overhead (results/diag/esp32s3-xchacha-seeded). The 16KB residue past the cache is the Poly1305 gap above. |

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
| rind | 341,386 |
| rustcrypto | 261,646 |
| ring | 143,860 |
| esp32s3-hw | 13,114 |

## Confidence

Each run is a fresh boot of the same image. With 3 runs the lowest and highest run medians bound the board's median with probability 75% (distribution-free: each run median falls on either side with probability one half); `ns_low` and `ns_high` in `esp32s3.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run noise only. Relinking an image moves its short-input rows far more than that: see `diag/layout-relink-2026-11-12/`, and read short-input ratios with the README's layout caveat.

The `legacy rind-bench` column and the `legacy_rind_bench` field are provenance from the private `steelhead99x/rind-bench` history that built these images. That repository was archived on 2026-11-12 and published as [`steelhead99x/rind-hardware`](https://github.com/steelhead99x/rind-hardware) (squash root `8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3`). Each image's `rind_hardware` field is the public commit when the trees match (the RP2040 set maps to `8d9c1bb`), or `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3` when the recorded tree is earlier in the squashed history. The SHA recorded at build time is in `legacy_rind_bench`. See `tools/legacy-rind-bench-map.json`. The Python tools in the commands below produced this set and have since been replaced by `cargo xtask`. To rebuild today, follow the current RESULTS.md using `rind-hardware`.

## Reproduce

```sh
git clone --recursive https://github.com/steelhead99x/rind-hardware   # then called rind-bench; public as rind-hardware
git clone https://github.com/steelhead99x/rind-results
rind-results/tools/setup-toolchain.sh rind-hardware   # espup, espflash, ldproxy, at these versions
cd rind-hardware && scripts/apply-patches.sh && scripts/vendor-ring.sh
(cd esp32 && ./build-images.sh)   # images/<bench>.elf; check sha256 against images.sha256
cd ../rind-results && PORT=/dev/serial/by-id/usb-Espressif_... tools/run-all.sh ../rind-hardware/esp32/images
python3 tools/report.py --raw results/raw --rind ../rind-hardware/rind --firmware ../rind-hardware/esp32 --sizes results/sizes.json --images results/images.json --meta results/meta-esp32s3.json --out results
python3 tools/verify.py   # recomputes every published number from raw
```

## SHA-3 (outside the report at this pin)

`admin/bench-report` at 99b509a6 has no SHA-3 operations, so these rows from the `sha3` image are only in `esp32s3-all.json` and get no gaps.toml entry; the gap is recorded here instead. At 16KB, cycles per byte: SHA3-256 rind 331.6, RustCrypto 273.4, libcrux 158.2; SHA3-384 431.9 / 355.8 / 223.2; SHA3-512 621.5 / 511.7 / 319.9. So rind takes 2.1x libcrux's time. **Decision: fixed upstream.** rind's Keccak rewrite at 88ff91fd brings SHA3-256 16KB to 151.0 c/B, 0.95x libcrux (results/diag/upstream-88ff91fd, measured at the 40 MHz flash clock; every SHA-3 row of all three libraries is identical at 40 and 80 MHz, so SHA-3 fits the cache and the comparison holds). For scale, CycloneCRYPTO publishes ~218 c/B (1.10 MB/s) for software SHA3-256 on this chip.

Generated by `tools/report.py` on 2026-11-12; every benchmark, with each run's statistics, is in `esp32s3-all.json`.
