# rind on an nRF52840 (Seeed T1000-E)

rind-bench's own benches, run unchanged but for the aws-lc-rs rows, on the nRF52840 of a Seeed T1000-E at 64 MHz, timed with the Cortex-M4's DWT cycle counter. Every operation and ID is `admin/bench-report`'s.

- rind: upstream `985683329fb98aebfaae6c493f205f20070a8fca`, plus the port's patches: `0001-fix-portable-build-MD5-on-Xtensa-whose-inline-assemb.patch`, `0002-feat-bench-leave-aws-lc-rs-rows-out-on-Xtensa-and-ba.patch`, `0003-feat-build-for-bare-metal-targets-without-std.patch`, `0004-feat-bench-leave-aws-lc-rs-rows-out-of-sha3-on-Xtensa.patch`, `ring-0.17.14-thumbv7em.patch`
- Firmware images (one per bench file; `images.sha256` beside this file):

  | Image | sha256 | rind | rind-hardware | legacy rind-bench | Built |
  |---|---|---|---|---|---|
  | aes_gcm | `4e528f7b7c27be2f…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:45 PST |
  | chacha20poly1305 | `d52fffc3533b4604…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:45 PST |
  | cipher | `d49f5f7c9f743f4f…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:46 PST |
  | ed25519 | `38f288f83db3dff0…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:49 PST |
  | hkdf | `fbb3525101dea1f6…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:51 PST |
  | hmac | `fbfb179720451cc9…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:50 PST |
  | mlkem768 | `06cd2745444dc059…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:52 PST |
  | p256 | `abb01fbcfe4e95aa…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:47 PST |
  | sha2 | `c72876aead0abe21…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:49 PST |
  | sha3 | `c7b2e685dcff996b…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:49 PST |
  | x25519 | `3482ac5d727828f5…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:46 PST |
  | xchacha20poly1305 | `b2290ab79a7d7129…` | `98568332` | ancestor of `8d9c1bb` | 911ebd2 (2026-11-12 11:44 PST), clean tree | 2026-11-12 11:45 PST |

- OS: none (bare metal, no_std); the board's UF2 bootloader and S140 SoftDevice stay in flash, unused
- Runs: 36 of 36 completed, one firmware image per rind-bench bench file
  - aes_gcm run 1: ok, 167 results, 2026-11-12T20:47:57+00:00 to 2026-11-12T20:50:01+00:00 (UTC)
  - aes_gcm run 2: ok, 167 results, 2026-11-12T20:50:01+00:00 to 2026-11-12T20:52:04+00:00 (UTC)
  - aes_gcm run 3: ok, 167 results, 2026-11-12T20:52:04+00:00 to 2026-11-12T20:54:07+00:00 (UTC)
  - chacha20poly1305 run 1: ok, 56 results, 2026-11-12T20:08:36+00:00 to 2026-11-12T20:09:15+00:00 (UTC)
  - chacha20poly1305 run 2: ok, 56 results, 2026-11-12T20:09:15+00:00 to 2026-11-12T20:09:54+00:00 (UTC)
  - chacha20poly1305 run 3: ok, 56 results, 2026-11-12T20:09:54+00:00 to 2026-11-12T20:10:33+00:00 (UTC)
  - cipher run 1: ok, 42 results, 2026-11-12T20:10:43+00:00 to 2026-11-12T20:11:34+00:00 (UTC)
  - cipher run 2: ok, 42 results, 2026-11-12T20:11:34+00:00 to 2026-11-12T20:12:24+00:00 (UTC)
  - cipher run 3: ok, 42 results, 2026-11-12T20:12:24+00:00 to 2026-11-12T20:13:14+00:00 (UTC)
  - ed25519 run 1: ok, 12 results, 2026-11-12T20:13:40+00:00 to 2026-11-12T20:14:13+00:00 (UTC)
  - ed25519 run 2: ok, 12 results, 2026-11-12T20:14:13+00:00 to 2026-11-12T20:14:47+00:00 (UTC)
  - ed25519 run 3: ok, 12 results, 2026-11-12T20:14:47+00:00 to 2026-11-12T20:15:20+00:00 (UTC)
  - hkdf run 1: ok, 38 results, 2026-11-12T20:15:36+00:00 to 2026-11-12T20:16:04+00:00 (UTC)
  - hkdf run 2: ok, 38 results, 2026-11-12T20:16:04+00:00 to 2026-11-12T20:16:32+00:00 (UTC)
  - hkdf run 3: ok, 38 results, 2026-11-12T20:16:32+00:00 to 2026-11-12T20:17:00+00:00 (UTC)
  - hmac run 1: ok, 102 results, 2026-11-12T20:17:12+00:00 to 2026-11-12T20:18:21+00:00 (UTC)
  - hmac run 2: ok, 102 results, 2026-11-12T20:18:21+00:00 to 2026-11-12T20:19:29+00:00 (UTC)
  - hmac run 3: ok, 102 results, 2026-11-12T20:19:29+00:00 to 2026-11-12T20:20:38+00:00 (UTC)
  - mlkem768 run 1: ok, 12 results, 2026-11-12T20:20:55+00:00 to 2026-11-12T20:21:32+00:00 (UTC)
  - mlkem768 run 2: ok, 12 results, 2026-11-12T20:21:32+00:00 to 2026-11-12T20:22:10+00:00 (UTC)
  - mlkem768 run 3: ok, 12 results, 2026-11-12T20:22:10+00:00 to 2026-11-12T20:22:48+00:00 (UTC)
  - p256 run 1: ok, 21 results, 2026-11-12T20:23:17+00:00 to 2026-11-12T20:24:22+00:00 (UTC)
  - p256 run 2: ok, 21 results, 2026-11-12T20:24:22+00:00 to 2026-11-12T20:25:27+00:00 (UTC)
  - p256 run 3: ok, 21 results, 2026-11-12T20:25:27+00:00 to 2026-11-12T20:26:33+00:00 (UTC)
  - sha2 run 1: ok, 68 results, 2026-11-12T20:26:45+00:00 to 2026-11-12T20:27:40+00:00 (UTC)
  - sha2 run 2: ok, 68 results, 2026-11-12T20:27:40+00:00 to 2026-11-12T20:28:35+00:00 (UTC)
  - sha2 run 3: ok, 68 results, 2026-11-12T20:28:35+00:00 to 2026-11-12T20:29:30+00:00 (UTC)
  - sha3 run 1: ok, 54 results, 2026-11-12T20:29:39+00:00 to 2026-11-12T20:30:45+00:00 (UTC)
  - sha3 run 2: ok, 54 results, 2026-11-12T20:30:45+00:00 to 2026-11-12T20:31:51+00:00 (UTC)
  - sha3 run 3: ok, 54 results, 2026-11-12T20:31:51+00:00 to 2026-11-12T20:32:56+00:00 (UTC)
  - x25519 run 1: ok, 9 results, 2026-11-12T20:33:12+00:00 to 2026-11-12T20:33:43+00:00 (UTC)
  - x25519 run 2: ok, 9 results, 2026-11-12T20:33:43+00:00 to 2026-11-12T20:34:15+00:00 (UTC)
  - x25519 run 3: ok, 9 results, 2026-11-12T20:34:15+00:00 to 2026-11-12T20:34:47+00:00 (UTC)
  - xchacha20poly1305 run 1: ok, 21 results, 2026-11-12T20:34:56+00:00 to 2026-11-12T20:35:22+00:00 (UTC)
  - xchacha20poly1305 run 2: ok, 21 results, 2026-11-12T20:35:22+00:00 to 2026-11-12T20:35:49+00:00 (UTC)
  - xchacha20poly1305 run 3: ok, 21 results, 2026-11-12T20:35:49+00:00 to 2026-11-12T20:36:15+00:00 (UTC)

Each cell is the median over runs of each run's median time per operation, with cycles per byte for byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. rind runs its portable Rust backend here (its 32-bit Arm assembly needs ARM state and NEON, which an M-profile core lacks); ring keeps its Thumb-2 assembly for SHA-2, AES and GHASH. There is no hardware baseline: the chip's CryptoCell CC310 needs Nordic's closed library.

## Headline: rind against the fastest other Rust library

| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |
|---|---|---|---|---|
| aes128-gcm/…/32B | 276.64 µs (553.3 c/B) | rustcrypto: 304.18 µs (608.4 c/B) | 0.91× | 0.0% |
| aes128-gcm/…/1350B | 5.28 ms (250.4 c/B) | rustcrypto: 5.14 ms (243.7 c/B) | 1.03× | 0.0% |
| aes128-gcm/…/16KB | 61.46 ms (240.1 c/B) | rustcrypto: 58.73 ms (229.4 c/B) | 1.05× | 0.0% |
| aes128-gcm-open/…/32B | 279.40 µs (558.8 c/B) | rustcrypto: 313.55 µs (627.1 c/B) | 0.89× | 0.0% |
| aes128-gcm-open/…/1350B | 5.31 ms (251.8 c/B) | rustcrypto: 5.18 ms (245.5 c/B) | 1.03× | 0.0% |
| aes128-gcm-open/…/16KB | 61.82 ms (241.5 c/B) | rustcrypto: 59.09 ms (230.8 c/B) | 1.05× | 0.0% |
| aes256-gcm/…/32B | 330.71 µs (661.4 c/B) | rustcrypto: 372.69 µs (745.4 c/B) | 0.89× | 0.0% |
| aes256-gcm/…/1350B | 6.47 ms (306.6 c/B) | rustcrypto: 6.28 ms (297.6 c/B) | 1.03× | 0.0% |
| aes256-gcm/…/16KB | 75.28 ms (294.1 c/B) | rustcrypto: 71.62 ms (279.8 c/B) | 1.05× | 0.0% |
| aes256-gcm-open/…/32B | 333.62 µs (667.2 c/B) | rustcrypto: 376.52 µs (753.0 c/B) | 0.89× | 0.0% |
| aes256-gcm-open/…/1350B | 6.50 ms (308.0 c/B) | rustcrypto: 6.31 ms (299.1 c/B) | 1.03× | 0.0% |
| aes256-gcm-open/…/16KB | 75.64 ms (295.5 c/B) | rustcrypto: 71.99 ms (281.2 c/B) | 1.05× | 0.0% |
| chacha20poly1305/…/32B | 154.87 µs (309.7 c/B) | ring: 88.93 µs (177.9 c/B) | 1.74× | 0.0% |
| chacha20poly1305/…/1350B | 1.29 ms (61.3 c/B) | ring: 727.91 µs (34.5 c/B) | 1.78× | 0.0% |
| chacha20poly1305/…/16KB | 14.07 ms (55.0 c/B) | ring: 7.86 ms (30.7 c/B) | 1.79× | 0.0% |
| xchacha20poly1305/…/32B | 3.50 ms (6994.0 c/B) | rustcrypto: 3.41 ms (6820.3 c/B) | 1.03× | 1.5% |
| xchacha20poly1305/…/1350B | 4.61 ms (218.5 c/B) | rustcrypto: 4.35 ms (206.1 c/B) | 1.06× | 0.6% |
| xchacha20poly1305/…/16KB | 17.32 ms (67.7 c/B) | rustcrypto: 14.77 ms (57.7 c/B) | 1.17× | 0.3% |
| quic-header-protection/aes128/… | 74.63 µs (4,776 c) | ring: 101.61 µs (6,503 c) | 0.73× | 0.0% |
| quic-header-protection/aes256/… | 101.84 µs (6,518 c) | ring: 138.13 µs (8,841 c) | 0.74× | 0.0% |
| quic-header-protection/chacha20/… | 23.39 µs (1,497 c) | ring: 23.87 µs (1,528 c) | 0.98× | 0.0% |
| x25519-ecdh/… | 45.27 ms (2,898 kc) | rustcrypto: 45.25 ms (2,896 kc) | 1.00× | 0.1% |
| p256-ecdh/… | 73.81 ms (4,724 kc) | ring: 171.97 ms (11,006 kc) | 0.43× | 0.1% |
| ed25519-sign/… | 12.54 ms (802 kc) | rustcrypto: 12.78 ms (818 kc) | 0.98× | 0.0% |
| ed25519-verify/… | 30.75 ms (1,968 kc) | ring: 29.95 ms (1,917 kc) | 1.03× | 0.0% |
| p256-ecdsa-sign/… | 22.91 ms (1,466 kc) | ring: 57.48 ms (3,679 kc) | 0.40× | 0.2% |
| p256-ecdsa-verify/… | 66.46 ms (4,254 kc) | ring: 157.30 ms (10,068 kc) | 0.42× | 0.0% |
| sha256/…/64B | 152.30 µs (152.3 c/B) | ring: 106.20 µs (106.2 c/B) | 1.43× | 0.0% |
| sha256/…/1350B | 1.41 ms (67.0 c/B) | ring: 885.35 µs (42.0 c/B) | 1.60× | 0.0% |
| sha256/…/16KB | 16.23 ms (63.4 c/B) | ring: 10.02 ms (39.1 c/B) | 1.62× | 0.0% |
| sha512/…/64B | 242.86 µs (242.9 c/B) | ring: 206.30 µs (206.3 c/B) | 1.18× | 0.0% |
| sha512/…/1350B | 2.34 ms (110.9 c/B) | ring: 1.95 ms (92.6 c/B) | 1.20× | 0.0% |
| sha512/…/16KB | 27.02 ms (105.6 c/B) | ring: 22.54 ms (88.0 c/B) | 1.20× | 0.0% |
| sha3-256/…/64B | 303.25 µs (303.3 c/B) | libcrux: 271.99 µs (272.0 c/B) | 1.11× | 0.0% |
| sha3-256/…/1350B | 2.79 ms (132.2 c/B) | libcrux: 2.68 ms (127.2 c/B) | 1.04× | 0.0% |
| sha3-256/…/16KB | 33.29 ms (130.1 c/B) | libcrux: 32.33 ms (126.3 c/B) | 1.03× | 0.0% |
| sha3-512/…/64B | 302.82 µs (302.8 c/B) | libcrux: 272.00 µs (272.0 c/B) | 1.11× | 0.0% |
| sha3-512/…/1350B | 5.23 ms (248.0 c/B) | libcrux: 5.10 ms (242.0 c/B) | 1.02× | 0.0% |
| sha3-512/…/16KB | 62.37 ms (243.6 c/B) | libcrux: 61.15 ms (238.9 c/B) | 1.02× | 0.0% |
| hmac-sha256/…/64B | 359.54 µs (359.5 c/B) | ring: 294.65 µs (294.6 c/B) | 1.22× | 0.0% |
| hmac-sha256/…/1350B | 1.62 ms (76.9 c/B) | ring: 1.07 ms (50.9 c/B) | 1.51× | 0.0% |
| hkdf-sha256/extract-expand/… | 611.30 µs (39 kc) | ring: 507.47 µs (32 kc) | 1.20× | 0.0% |
| hkdf-sha256/expand/… | 173.01 µs (11 kc) | rustcrypto: 131.35 µs (8,407 c) | 1.32× | 0.0% |
| mlkem768-keygen/… | 25.27 ms (1,617 kc) | libcrux: 26.22 ms (1,678 kc) | 0.96× | 0.1% |
| mlkem768-encaps/… | 22.81 ms (1,460 kc) | libcrux: 23.89 ms (1,529 kc) | 0.95× | 1.5% |
| mlkem768-decaps/… | 10.25 ms (656 kc) | libcrux: 22.44 ms (1,436 kc) | 0.46× | 0.0% |

## AEADs

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| aes128-gcm/…/32B | 276.64 µs (553.3 c/B) | 333.84 µs (667.7 c/B) | 304.18 µs (608.4 c/B) | 1.11 ms (2210.7 c/B) |
| aes128-gcm/…/1350B | 5.28 ms (250.4 c/B) | 5.86 ms (277.6 c/B) | 5.14 ms (243.7 c/B) | 19.23 ms (911.6 c/B) |
| aes128-gcm/…/16KB | 61.46 ms (240.1 c/B) | 67.63 ms (264.2 c/B) | 58.73 ms (229.4 c/B) | 224.24 ms (875.9 c/B) |
| aes128-gcm-open/…/32B | 279.40 µs (558.8 c/B) | 338.17 µs (676.3 c/B) | 313.55 µs (627.1 c/B) | 1.10 ms (2204.0 c/B) |
| aes128-gcm-open/…/1350B | 5.31 ms (251.8 c/B) | 5.89 ms (279.2 c/B) | 5.18 ms (245.5 c/B) | 19.48 ms (923.4 c/B) |
| aes128-gcm-open/…/16KB | 61.82 ms (241.5 c/B) | 68.00 ms (265.6 c/B) | 59.09 ms (230.8 c/B) | 227.35 ms (888.1 c/B) |
| aes256-gcm/…/32B | 330.71 µs (661.4 c/B) | 406.45 µs (812.9 c/B) | 372.69 µs (745.4 c/B) | 1.29 ms (2585.8 c/B) |
| aes256-gcm/…/1350B | 6.47 ms (306.6 c/B) | 7.21 ms (341.9 c/B) | 6.28 ms (297.6 c/B) | 22.81 ms (1081.5 c/B) |
| aes256-gcm/…/16KB | 75.28 ms (294.1 c/B) | 83.29 ms (325.3 c/B) | 71.62 ms (279.8 c/B) | 266.26 ms (1040.1 c/B) |
| aes256-gcm-open/…/32B | 333.62 µs (667.2 c/B) | 410.70 µs (821.4 c/B) | 376.52 µs (753.0 c/B) | 1.28 ms (2562.2 c/B) |
| aes256-gcm-open/…/1350B | 6.50 ms (308.0 c/B) | 7.25 ms (343.5 c/B) | 6.31 ms (299.1 c/B) | 22.88 ms (1084.9 c/B) |
| aes256-gcm-open/…/16KB | 75.64 ms (295.5 c/B) | 83.66 ms (326.8 c/B) | 71.99 ms (281.2 c/B) | 267.24 ms (1043.9 c/B) |
| chacha20poly1305/…/32B | 154.87 µs (309.7 c/B) | 88.93 µs (177.9 c/B) | 90.13 µs (180.3 c/B) | 122.83 µs (245.7 c/B) |
| chacha20poly1305/…/1350B | 1.29 ms (61.3 c/B) | 727.91 µs (34.5 c/B) | 1.12 ms (53.1 c/B) | 1.34 ms (63.5 c/B) |
| chacha20poly1305/…/16KB | 14.07 ms (55.0 c/B) | 7.86 ms (30.7 c/B) | 12.62 ms (49.3 c/B) | 14.96 ms (58.4 c/B) |
| xchacha20poly1305/…/32B | 3.50 ms (6994.0 c/B) | - | 3.41 ms (6820.3 c/B) | 3.47 ms (6940.7 c/B) |
| xchacha20poly1305/…/1350B | 4.61 ms (218.5 c/B) | - | 4.35 ms (206.1 c/B) | 4.66 ms (220.7 c/B) |
| xchacha20poly1305/…/16KB | 17.32 ms (67.7 c/B) | - | 14.77 ms (57.7 c/B) | 18.25 ms (71.3 c/B) |

## QUIC header protection

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| quic-header-protection/aes128/… | 74.63 µs (4,776 c) | 101.61 µs (6,503 c) | - | - |
| quic-header-protection/aes256/… | 101.84 µs (6,518 c) | 138.13 µs (8,841 c) | - | - |
| quic-header-protection/chacha20/… | 23.39 µs (1,497 c) | 23.87 µs (1,528 c) | - | - |

## Key agreement

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| x25519-ecdh/… | 45.27 ms (2,898 kc) | 47.70 ms (3,052 kc) | 45.25 ms (2,896 kc) | 69.89 ms (4,473 kc) |
| p256-ecdh/… | 73.81 ms (4,724 kc) | 171.97 ms (11,006 kc) | 464.21 ms (29,709 kc) | 193.96 ms (12,413 kc) |

## Signatures

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| ed25519-sign/… | 12.54 ms (802 kc) | 15.54 ms (995 kc) | 12.78 ms (818 kc) | 45.02 ms (2,881 kc) |
| ed25519-verify/… | 30.75 ms (1,968 kc) | 29.95 ms (1,917 kc) | 30.80 ms (1,972 kc) | 47.58 ms (3,045 kc) |
| p256-ecdsa-sign/… | 22.91 ms (1,466 kc) | 57.48 ms (3,679 kc) | 236.06 ms (15,108 kc) | 70.06 ms (4,484 kc) |
| p256-ecdsa-verify/… | 66.46 ms (4,254 kc) | 157.30 ms (10,068 kc) | 254.31 ms (16,276 kc) | 161.73 ms (10,351 kc) |

## Hashes and KDFs

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| sha256/…/64B | 152.30 µs (152.3 c/B) | 106.20 µs (106.2 c/B) | 123.48 µs (123.5 c/B) | 132.95 µs (133.0 c/B) |
| sha256/…/1350B | 1.41 ms (67.0 c/B) | 885.35 µs (42.0 c/B) | 1.24 ms (58.7 c/B) | 1.39 ms (66.1 c/B) |
| sha256/…/16KB | 16.23 ms (63.4 c/B) | 10.02 ms (39.1 c/B) | 14.33 ms (56.0 c/B) | 16.19 ms (63.2 c/B) |
| sha512/…/64B | 242.86 µs (242.9 c/B) | 206.30 µs (206.3 c/B) | 235.58 µs (235.6 c/B) | 222.31 µs (222.3 c/B) |
| sha512/…/1350B | 2.34 ms (110.9 c/B) | 1.95 ms (92.6 c/B) | 2.39 ms (113.5 c/B) | 2.32 ms (109.8 c/B) |
| sha512/…/16KB | 27.02 ms (105.6 c/B) | 22.54 ms (88.0 c/B) | 27.83 ms (108.7 c/B) | 27.00 ms (105.5 c/B) |
| sha3-256/…/64B | 303.25 µs (303.3 c/B) | - | 676.88 µs (676.9 c/B) | 271.99 µs (272.0 c/B) |
| sha3-256/…/1350B | 2.79 ms (132.2 c/B) | - | 6.66 ms (315.7 c/B) | 2.68 ms (127.2 c/B) |
| sha3-256/…/16KB | 33.29 ms (130.1 c/B) | - | 80.34 ms (313.8 c/B) | 32.33 ms (126.3 c/B) |
| sha3-512/…/64B | 302.82 µs (302.8 c/B) | - | 675.15 µs (675.1 c/B) | 272.00 µs (272.0 c/B) |
| sha3-512/…/1350B | 5.23 ms (248.0 c/B) | - | 12.61 ms (597.8 c/B) | 5.10 ms (242.0 c/B) |
| sha3-512/…/16KB | 62.37 ms (243.6 c/B) | - | 151.18 ms (590.5 c/B) | 61.15 ms (238.9 c/B) |
| hmac-sha256/…/64B | 359.54 µs (359.5 c/B) | 294.65 µs (294.6 c/B) | 312.79 µs (312.8 c/B) | 347.85 µs (347.9 c/B) |
| hmac-sha256/…/1350B | 1.62 ms (76.9 c/B) | 1.07 ms (50.9 c/B) | 1.43 ms (67.6 c/B) | 1.61 ms (76.2 c/B) |
| hkdf-sha256/extract-expand/… | 611.30 µs (39 kc) | 507.47 µs (32 kc) | 508.93 µs (33 kc) | 573.28 µs (37 kc) |
| hkdf-sha256/expand/… | 173.01 µs (11 kc) | 146.66 µs (9,386 c) | 131.35 µs (8,407 c) | 294.67 µs (19 kc) |

## ML-KEM

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| mlkem768-keygen/… | 25.27 ms (1,617 kc) | - | 49.95 ms (3,197 kc) | 26.22 ms (1,678 kc) |
| mlkem768-encaps/… | 22.81 ms (1,460 kc) | - | 49.91 ms (3,194 kc) | 23.89 ms (1,529 kc) |
| mlkem768-decaps/… | 10.25 ms (656 kc) | - | 49.37 ms (3,160 kc) | 22.44 ms (1,436 kc) |

## Known gaps

Every operation where another Rust library beats rind by more than 10% here, with the decision on it, as rind's `benchmarks/gaps.toml` requires; `cargo xtask report` refuses a gap without an entry in `results/gaps.toml`, and an entry that covers no gap.

| Operations | rind's time over the fastest | Fastest | Decision |
|---|---|---|---|
| `chacha20poly1305/…/1350B`, `chacha20poly1305/…/16KB`, `chacha20poly1305/…/32B` | 1.74× to 1.79× | ring | Assembly against portable Rust: ring assembles OpenSSL's Thumb-2 ChaCha20 (chacha-armv4) here, while rind runs its portable backend, because its 32-bit Arm assembly is ARM-state NEON that an M-profile core lacks. rind's portable code is 0.93x libcrux's and 1.15x RustCrypto's at 1350B. A Thumb-2 (or DSP-extension) ChaCha20 for M-profile would close it. |
| `xchacha20poly1305/…/16KB` | 1.17× | rustcrypto | Portable against portable: ring has no XChaCha, and RustCrypto's soft ChaCha20 runs 16 KB in 0.85x rind's cycles (946 kc against 1,115 kc); rind is 0.95x libcrux. At 1350B and below the HChaCha20 key setup dominates and all three are within 6%. Same fix as chacha20poly1305: an M-profile ChaCha20. |
| `hkdf-sha256/extract-expand/…`, `hmac-sha256/…/1350B`, `hmac-sha256/…/64B`, `sha256/…/1350B`, `sha256/…/16KB`, `sha256/…/64B` | 1.20× to 1.62× | ring | Assembly against portable Rust: ring assembles OpenSSL's sha256-armv4 here (Thumb-2, no NEON needed), 0.62x rind's cycles at 16 KB. rind's portable SHA-256 is libcrux-sha2's code and matches libcrux within 1% on every length, and RustCrypto's soft SHA-256 is 0.88x. HMAC and HKDF inherit it. An M-profile SHA-256 block function would close it. |
| `hkdf-sha256/expand/…` | 1.32× | rustcrypto | Fixed per-call cost on a 64-byte output: rind's expand takes 11.1 kc against RustCrypto's 8.5 kc and ring's 9.3 kc, about 2.5 kc of setup over three SHA-256 blocks; with the same SHA-256 the long rows track libcrux. Not investigated further; the same pattern does not show on SHA-384/512, where rind is within 14% of ring and level with RustCrypto. |
| `sha512/…/1350B`, `sha512/…/16KB`, `sha512/…/64B` | 1.18× to 1.20× | ring | Assembly against portable Rust: ring assembles OpenSSL's sha512-armv4 (Thumb-2), 0.84x rind's cycles. rind's portable SHA-512 (libcrux-sha2's code) matches libcrux within 1% and beats RustCrypto's soft SHA-512 (1.03x theirs). |
| `sha3-256/…/64B`, `sha3-512/…/64B` | 1.11× | libcrux | Per-call overhead, not the permutation: rind takes about 2.0 k cycles more than libcrux on every call whatever the digest (sha3-256, -384 and -512 at 32B and 64B are all 19.3-19.4 k against 17.4 k), while each further block costs it about 3% more (16KB: 2.13 M against 2.07 M cycles for SHA3-256, 3.99 M against 3.91 M for SHA3-512). At one block the fixed cost is 11%. Not profiled; rind is 2.2x faster than RustCrypto on every SHA-3 row. Decision: accepted, small. |

## Known-answer checks

Run on the board before any timing, every run. A library failing an operation's check is not raced there.

| Check | Library | Result |
|---|---|---|
| aes128-gcm/1350B-agrees-with-rind | libcrux | pass (6 runs) |
| aes128-gcm/1350B-agrees-with-rind | ring | pass (6 runs) |
| aes128-gcm/1350B-agrees-with-rind | rustcrypto | pass (6 runs) |
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
| sha256/16KB-agrees-with-rind | libcrux | pass (3 runs) |
| sha256/16KB-agrees-with-rind | ring | pass (3 runs) |
| sha256/16KB-agrees-with-rind | rustcrypto | pass (3 runs) |
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

Each run is a fresh boot of the same image. With 3 runs the lowest and highest run medians bound the board's median with probability 75% (distribution-free: each run median falls on either side with probability one half); `ns_low` and `ns_high` in `nrf52840.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run noise only. Relinked builds that move every library by about 1 KB and about 2 KB (rind-hardware's `--pad 1031` and `--pad 2053`, padding ahead of the libraries since 2026-11-12, checked with `nm -n`) moved no short-input ratio by more than 0.01 (`diag/nrf52840-relink-fixed-98568332/relink-short.md`: chacha20poly1305, hkdf, hmac and sha2). An earlier relink of all twelve benches (`diag/nrf52840-relink-98568332/`) moved the Rust code only 8 bytes, so it did not test layout. Unlike the ESP32-S3's, these rows hardly depend on layout (why is not tested).

The `legacy rind-bench` column and the `legacy_rind_bench` field are provenance from the private `steelhead99x/rind-bench` history that built these images. That repository was archived on 2026-11-12 and published as [`steelhead99x/rind-hardware`](https://github.com/steelhead99x/rind-hardware) (squash root `8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3`). Each image's `rind_hardware` field is the public commit when the trees match (the RP2040 set maps to `8d9c1bb`), or `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3` when the recorded tree is earlier in the squashed history. The SHA recorded at build time is in `legacy_rind_bench`. See `tools/legacy-rind-bench-map.json`. Clone `rind-hardware` to rebuild.

## Reproduce

```sh
git clone --recursive https://github.com/steelhead99x/rind-hardware && cd rind-hardware
cargo xtask apply-patches && cargo xtask vendor-ring && cargo xtask build-images nrf
cd ../rind-results && cargo xtask run-all-nrf ../rind-hardware/nrf52840/images   # double-tap reset for the UF2 drive; back up the board's firmware first
cargo xtask report --board nrf52840 --raw results/raw-nrf52840 --rind ../rind-hardware/rind --firmware ../rind-hardware/nrf52840 --gaps results/nrf52840/gaps.toml --images results/nrf52840/images.json --out results/nrf52840
cargo verify
```

The published images were built from `steelhead99x/rind-bench` (private, now archived). The firmware repo is now the public [steelhead99x/rind-hardware](https://github.com/steelhead99x/rind-hardware); its first commit (`8d9c1bb`) is a squash of that history. Each image's `rind_hardware` field is the public commit when the trees match (the RP2040 set maps to `8d9c1bb`), or `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3` when the recorded tree is earlier in the squashed history. The SHA recorded at build time is in `legacy_rind_bench`. See `tools/legacy-rind-bench-map.json`. Build new images from rind-hardware.

Generated by `cargo xtask report` on 2026-11-12; every benchmark, with each run's statistics, is in `nrf52840-all.json`.
