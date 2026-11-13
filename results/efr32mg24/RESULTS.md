# rind on an EFR32MG24 (Seeed XIAO MG24)

rind-bench's own benches, run unchanged but for the aws-lc-rs rows, on the EFR32MG24 of a Seeed XIAO MG24 at 78 MHz, timed with the Cortex-M33's DWT cycle counter. Every operation and ID is `admin/bench-report`'s.

- rind: upstream `985683329fb98aebfaae6c493f205f20070a8fca`, plus the port's patches: `0001-fix-portable-build-MD5-on-Xtensa-whose-inline-assemb.patch`, `0002-feat-bench-leave-aws-lc-rs-rows-out-on-Xtensa-and-ba.patch`, `0003-feat-build-for-bare-metal-targets-without-std.patch`, `0004-feat-bench-leave-aws-lc-rs-rows-out-of-sha3-on-Xtensa.patch`, `0005-feat-reuse-cache-without-CAS-on-targets-that-lack-it.patch`, `ring-0.17.14-thumbv7em.patch`
- Firmware images (one per bench file; `images.sha256` beside this file):

  | Image | sha256 | rind | rind-hardware | legacy rind-bench | Built |
  |---|---|---|---|---|---|
  | aes_gcm | `5ae220c5a9408f23…` | `98568332` | `0777915` |  | 2026-11-12 20:39 PST |
  | chacha20poly1305 | `a36ae77355e5a799…` | `98568332` | `0777915` |  | 2026-11-12 20:39 PST |
  | cipher | `7d78fffc48630767…` | `98568332` | `0777915` |  | 2026-11-12 20:39 PST |
  | ed25519 | `81acc6434501781f…` | `98568332` | `0777915` |  | 2026-11-12 20:40 PST |
  | hkdf | `2328ba6e691b222a…` | `98568332` | `0777915` |  | 2026-11-12 20:41 PST |
  | hmac | `6d3db3426522d836…` | `98568332` | `0777915` |  | 2026-11-12 20:40 PST |
  | mlkem768 | `30d8e76b3a51eeb1…` | `98568332` | `0777915` |  | 2026-11-12 20:41 PST |
  | p256 | `e91c7a16aa8800d9…` | `98568332` | `0777915` |  | 2026-11-12 20:40 PST |
  | sha2 | `76a3b35ee70e0109…` | `98568332` | `0777915` |  | 2026-11-12 20:40 PST |
  | sha3 | `e0c5a1e11ff5c099…` | `98568332` | `0777915` |  | 2026-11-12 20:40 PST |
  | x25519 | `465ff14f82adfd1f…` | `98568332` | `0777915` |  | 2026-11-12 20:40 PST |
  | xchacha20poly1305 | `756ec02abe77151c…` | `98568332` | `0777915` |  | 2026-11-12 20:39 PST |

- OS: none (bare metal, no_std); output over SEGGER RTT through the board's CMSIS-DAP probe
- Runs: 60 of 60 completed, one firmware image per rind-bench bench file
  - aes_gcm run 1: ok, 167 results, 2026-11-13T04:42:19+00:00 to 2026-11-13T04:44:28+00:00 (UTC)
  - aes_gcm run 2: ok, 167 results, 2026-11-13T04:44:28+00:00 to 2026-11-13T04:46:37+00:00 (UTC)
  - aes_gcm run 3: ok, 167 results, 2026-11-13T04:46:37+00:00 to 2026-11-13T04:48:47+00:00 (UTC)
  - aes_gcm run 4: ok, 167 results, 2026-11-13T04:48:47+00:00 to 2026-11-13T04:50:56+00:00 (UTC)
  - aes_gcm run 5: ok, 167 results, 2026-11-13T04:50:56+00:00 to 2026-11-13T04:53:05+00:00 (UTC)
  - chacha20poly1305 run 1: ok, 56 results, 2026-11-13T04:54:40+00:00 to 2026-11-13T04:55:21+00:00 (UTC)
  - chacha20poly1305 run 2: ok, 56 results, 2026-11-13T04:55:21+00:00 to 2026-11-13T04:56:01+00:00 (UTC)
  - chacha20poly1305 run 3: ok, 56 results, 2026-11-13T04:56:01+00:00 to 2026-11-13T04:56:42+00:00 (UTC)
  - chacha20poly1305 run 4: ok, 56 results, 2026-11-13T04:56:42+00:00 to 2026-11-13T04:57:22+00:00 (UTC)
  - chacha20poly1305 run 5: ok, 56 results, 2026-11-13T04:57:22+00:00 to 2026-11-13T04:58:02+00:00 (UTC)
  - cipher run 1: ok, 42 results, 2026-11-13T05:02:41+00:00 to 2026-11-13T05:03:34+00:00 (UTC)
  - cipher run 2: ok, 42 results, 2026-11-13T05:03:34+00:00 to 2026-11-13T05:04:27+00:00 (UTC)
  - cipher run 3: ok, 42 results, 2026-11-13T05:04:27+00:00 to 2026-11-13T05:05:19+00:00 (UTC)
  - cipher run 4: ok, 42 results, 2026-11-13T05:05:19+00:00 to 2026-11-13T05:06:11+00:00 (UTC)
  - cipher run 5: ok, 42 results, 2026-11-13T05:06:11+00:00 to 2026-11-13T05:07:04+00:00 (UTC)
  - ed25519 run 1: ok, 12 results, 2026-11-13T05:28:16+00:00 to 2026-11-13T05:29:01+00:00 (UTC)
  - ed25519 run 2: ok, 12 results, 2026-11-13T05:29:01+00:00 to 2026-11-13T05:29:46+00:00 (UTC)
  - ed25519 run 3: ok, 12 results, 2026-11-13T05:29:46+00:00 to 2026-11-13T05:30:29+00:00 (UTC)
  - ed25519 run 4: ok, 12 results, 2026-11-13T05:30:29+00:00 to 2026-11-13T05:31:12+00:00 (UTC)
  - ed25519 run 5: ok, 12 results, 2026-11-13T05:31:12+00:00 to 2026-11-13T05:31:55+00:00 (UTC)
  - hkdf run 1: ok, 38 results, 2026-11-13T06:05:38+00:00 to 2026-11-13T06:06:13+00:00 (UTC)
  - hkdf run 2: ok, 38 results, 2026-11-13T06:06:13+00:00 to 2026-11-13T06:06:48+00:00 (UTC)
  - hkdf run 3: ok, 38 results, 2026-11-13T06:06:48+00:00 to 2026-11-13T06:07:24+00:00 (UTC)
  - hkdf run 4: ok, 38 results, 2026-11-13T06:07:24+00:00 to 2026-11-13T06:07:59+00:00 (UTC)
  - hkdf run 5: ok, 38 results, 2026-11-13T06:07:59+00:00 to 2026-11-13T06:08:34+00:00 (UTC)
  - hmac run 1: ok, 102 results, 2026-11-13T05:54:44+00:00 to 2026-11-13T05:55:57+00:00 (UTC)
  - hmac run 2: ok, 102 results, 2026-11-13T05:55:57+00:00 to 2026-11-13T05:57:10+00:00 (UTC)
  - hmac run 3: ok, 102 results, 2026-11-13T05:57:10+00:00 to 2026-11-13T05:58:23+00:00 (UTC)
  - hmac run 4: ok, 102 results, 2026-11-13T05:58:23+00:00 to 2026-11-13T05:59:37+00:00 (UTC)
  - hmac run 5: ok, 102 results, 2026-11-13T05:59:37+00:00 to 2026-11-13T06:00:50+00:00 (UTC)
  - mlkem768 run 1: ok, 12 results, 2026-11-13T06:10:40+00:00 to 2026-11-13T06:11:19+00:00 (UTC)
  - mlkem768 run 2: ok, 12 results, 2026-11-13T06:11:19+00:00 to 2026-11-13T06:11:59+00:00 (UTC)
  - mlkem768 run 3: ok, 12 results, 2026-11-13T06:11:59+00:00 to 2026-11-13T06:12:38+00:00 (UTC)
  - mlkem768 run 4: ok, 12 results, 2026-11-13T06:12:38+00:00 to 2026-11-13T06:13:19+00:00 (UTC)
  - mlkem768 run 5: ok, 12 results, 2026-11-13T06:13:19+00:00 to 2026-11-13T06:13:59+00:00 (UTC)
  - p256 run 1: ok, 21 results, 2026-11-13T05:16:47+00:00 to 2026-11-13T05:18:04+00:00 (UTC)
  - p256 run 2: ok, 21 results, 2026-11-13T05:18:04+00:00 to 2026-11-13T05:19:29+00:00 (UTC)
  - p256 run 3: ok, 21 results, 2026-11-13T05:19:29+00:00 to 2026-11-13T05:20:45+00:00 (UTC)
  - p256 run 4: ok, 21 results, 2026-11-13T05:20:45+00:00 to 2026-11-13T05:22:04+00:00 (UTC)
  - p256 run 5: ok, 21 results, 2026-11-13T05:22:05+00:00 to 2026-11-13T05:23:24+00:00 (UTC)
  - sha2 run 1: ok, 68 results, 2026-11-13T05:35:58+00:00 to 2026-11-13T05:36:56+00:00 (UTC)
  - sha2 run 2: ok, 68 results, 2026-11-13T05:36:56+00:00 to 2026-11-13T05:37:55+00:00 (UTC)
  - sha2 run 3: ok, 68 results, 2026-11-13T05:37:55+00:00 to 2026-11-13T05:38:53+00:00 (UTC)
  - sha2 run 4: ok, 68 results, 2026-11-13T05:38:54+00:00 to 2026-11-13T05:39:52+00:00 (UTC)
  - sha2 run 5: ok, 68 results, 2026-11-13T05:39:52+00:00 to 2026-11-13T05:40:51+00:00 (UTC)
  - sha3 run 1: ok, 54 results, 2026-11-13T05:45:51+00:00 to 2026-11-13T05:46:54+00:00 (UTC)
  - sha3 run 2: ok, 54 results, 2026-11-13T05:46:54+00:00 to 2026-11-13T05:47:58+00:00 (UTC)
  - sha3 run 3: ok, 54 results, 2026-11-13T05:47:58+00:00 to 2026-11-13T05:49:01+00:00 (UTC)
  - sha3 run 4: ok, 54 results, 2026-11-13T05:49:01+00:00 to 2026-11-13T05:50:05+00:00 (UTC)
  - sha3 run 5: ok, 54 results, 2026-11-13T05:50:05+00:00 to 2026-11-13T05:51:09+00:00 (UTC)
  - x25519 run 1: ok, 9 results, 2026-11-13T05:10:09+00:00 to 2026-11-13T05:10:42+00:00 (UTC)
  - x25519 run 2: ok, 9 results, 2026-11-13T05:10:42+00:00 to 2026-11-13T05:11:15+00:00 (UTC)
  - x25519 run 3: ok, 9 results, 2026-11-13T05:11:15+00:00 to 2026-11-13T05:11:48+00:00 (UTC)
  - x25519 run 4: ok, 9 results, 2026-11-13T05:11:48+00:00 to 2026-11-13T05:12:22+00:00 (UTC)
  - x25519 run 5: ok, 9 results, 2026-11-13T05:12:22+00:00 to 2026-11-13T05:12:55+00:00 (UTC)
  - xchacha20poly1305 run 1: ok, 21 results, 2026-11-13T04:59:37+00:00 to 2026-11-13T05:00:01+00:00 (UTC)
  - xchacha20poly1305 run 2: ok, 21 results, 2026-11-13T05:00:01+00:00 to 2026-11-13T05:00:25+00:00 (UTC)
  - xchacha20poly1305 run 3: ok, 21 results, 2026-11-13T05:00:25+00:00 to 2026-11-13T05:00:48+00:00 (UTC)
  - xchacha20poly1305 run 4: ok, 21 results, 2026-11-13T05:00:48+00:00 to 2026-11-13T05:01:12+00:00 (UTC)
  - xchacha20poly1305 run 5: ok, 21 results, 2026-11-13T05:01:12+00:00 to 2026-11-13T05:01:36+00:00 (UTC)

Each cell is the median over runs of each run's median time per operation, with cycles per byte for byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. rind runs its portable Rust backend here (its 32-bit Arm assembly needs ARM state and NEON, which an M-profile core lacks); ring keeps its Thumb-2 assembly for SHA-2, AES and GHASH. There is no hardware baseline: the chip's AES and SHA engines sit behind the Secure Engine. getrandom is a seeded SplitMix64 (the TRNG is behind the Secure Engine too), the same for every library.

## Headline: rind against the fastest other Rust library

| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |
|---|---|---|---|---|
| aes128-gcm/…/32B | 212.15 µs (517.1 c/B) | rustcrypto: 243.83 µs (594.3 c/B) | 0.87× | 0.0% |
| aes128-gcm/…/1350B | 4.03 ms (232.7 c/B) | rustcrypto: 3.78 ms (218.6 c/B) | 1.06× | 0.0% |
| aes128-gcm/…/16KB | 46.79 ms (222.8 c/B) | rustcrypto: 42.93 ms (204.4 c/B) | 1.09× | 0.0% |
| aes128-gcm-open/…/32B | 215.30 µs (524.8 c/B) | rustcrypto: 255.13 µs (621.9 c/B) | 0.84× | 0.0% |
| aes128-gcm-open/…/1350B | 4.04 ms (233.7 c/B) | rustcrypto: 3.81 ms (220.1 c/B) | 1.06× | 0.0% |
| aes128-gcm-open/…/16KB | 46.99 ms (223.7 c/B) | rustcrypto: 43.13 ms (205.3 c/B) | 1.09× | 0.0% |
| aes256-gcm/…/32B | 256.05 µs (624.1 c/B) | rustcrypto: 299.37 µs (729.7 c/B) | 0.86× | 0.0% |
| aes256-gcm/…/1350B | 4.97 ms (287.4 c/B) | rustcrypto: 4.39 ms (253.5 c/B) | 1.13× | 0.0% |
| aes256-gcm/…/16KB | 57.81 ms (275.2 c/B) | rustcrypto: 49.49 ms (235.6 c/B) | 1.17× | 0.0% |
| aes256-gcm-open/…/32B | 259.17 µs (631.7 c/B) | rustcrypto: 300.11 µs (731.5 c/B) | 0.86× | 0.0% |
| aes256-gcm-open/…/1350B | 4.99 ms (288.4 c/B) | rustcrypto: 4.40 ms (254.4 c/B) | 1.13× | 0.0% |
| aes256-gcm-open/…/16KB | 58.01 ms (276.2 c/B) | rustcrypto: 49.68 ms (236.5 c/B) | 1.17× | 0.0% |
| chacha20poly1305/…/32B | 108.51 µs (264.5 c/B) | ring: 67.26 µs (163.9 c/B) | 1.61× | 0.0% |
| chacha20poly1305/…/1350B | 898.13 µs (51.9 c/B) | ring: 576.35 µs (33.3 c/B) | 1.56× | 0.0% |
| chacha20poly1305/…/16KB | 9.74 ms (46.4 c/B) | ring: 6.26 ms (29.8 c/B) | 1.55× | 0.0% |
| xchacha20poly1305/…/32B | 136.49 µs (332.7 c/B) | rustcrypto: 86.48 µs (210.8 c/B) | 1.58× | 0.0% |
| xchacha20poly1305/…/1350B | 925.87 µs (53.5 c/B) | rustcrypto: 820.52 µs (47.4 c/B) | 1.13× | 0.0% |
| xchacha20poly1305/…/16KB | 9.77 ms (46.5 c/B) | rustcrypto: 8.98 ms (42.8 c/B) | 1.09× | 0.0% |
| quic-header-protection/aes128/… | 57.37 µs (4,475 c) | ring: 76.45 µs (5,964 c) | 0.75× | 0.0% |
| quic-header-protection/aes256/… | 79.27 µs (6,183 c) | ring: 105.07 µs (8,195 c) | 0.75× | 0.0% |
| quic-header-protection/chacha20/… | 16.86 µs (1,315 c) | ring: 19.18 µs (1,496 c) | 0.88× | 0.0% |
| x25519-ecdh/… | 30.28 ms (2,362 kc) | rustcrypto: 30.26 ms (2,360 kc) | 1.00× | 0.0% |
| p256-ecdh/… | 57.02 ms (4,447 kc) | ring: 123.87 ms (9,662 kc) | 0.46× | 0.0% |
| ed25519-sign/… | 9.87 ms (770 kc) | rustcrypto: 10.14 ms (791 kc) | 0.97× | 0.0% |
| ed25519-verify/… | 23.66 ms (1,845 kc) | ring: 23.47 ms (1,831 kc) | 1.01× | 0.0% |
| p256-ecdsa-sign/… | 17.13 ms (1,336 kc) | ring: 38.81 ms (3,027 kc) | 0.44× | 0.0% |
| p256-ecdsa-verify/… | 54.03 ms (4,214 kc) | ring: 119.09 ms (9,289 kc) | 0.45× | 0.0% |
| sha256/…/64B | 112.60 µs (137.2 c/B) | ring: 70.29 µs (85.7 c/B) | 1.60× | 0.0% |
| sha256/…/1350B | 946.12 µs (54.7 c/B) | ring: 597.11 µs (34.5 c/B) | 1.58× | 0.0% |
| sha256/…/16KB | 10.65 ms (50.7 c/B) | ring: 6.78 ms (32.3 c/B) | 1.57× | 0.0% |
| sha512/…/64B | 246.00 µs (299.8 c/B) | ring: 141.08 µs (171.9 c/B) | 1.74× | 0.0% |
| sha512/…/1350B | 2.44 ms (140.8 c/B) | ring: 1.37 ms (79.1 c/B) | 1.78× | 0.0% |
| sha512/…/16KB | 28.25 ms (134.5 c/B) | ring: 15.84 ms (75.4 c/B) | 1.78× | 0.0% |
| sha3-256/…/64B | 224.45 µs (273.5 c/B) | libcrux: 209.46 µs (255.3 c/B) | 1.07× | 0.0% |
| sha3-256/…/1350B | 2.10 ms (121.3 c/B) | libcrux: 2.03 ms (117.6 c/B) | 1.03× | 0.0% |
| sha3-256/…/16KB | 25.16 ms (119.8 c/B) | libcrux: 24.52 ms (116.7 c/B) | 1.03× | 0.0% |
| sha3-512/…/64B | 224.08 µs (273.1 c/B) | libcrux: 200.75 µs (244.7 c/B) | 1.12× | 0.0% |
| sha3-512/…/1350B | 3.95 ms (228.2 c/B) | libcrux: 3.83 ms (221.3 c/B) | 1.03× | 0.0% |
| sha3-512/…/16KB | 47.16 ms (224.5 c/B) | libcrux: 45.96 ms (218.8 c/B) | 1.03× | 0.0% |
| hmac-sha256/…/64B | 266.76 µs (325.1 c/B) | ring: 195.78 µs (238.6 c/B) | 1.36× | 0.0% |
| hmac-sha256/…/1350B | 1.10 ms (63.6 c/B) | ring: 722.96 µs (41.8 c/B) | 1.52× | 0.0% |
| hkdf-sha256/extract-expand/… | 467.68 µs (36 kc) | ring: 342.39 µs (27 kc) | 1.37× | 0.0% |
| hkdf-sha256/expand/… | 138.79 µs (11 kc) | rustcrypto: 97.54 µs (7,608 c) | 1.42× | 0.0% |
| mlkem768-keygen/… | 12.88 ms (1,005 kc) | libcrux: 13.62 ms (1,063 kc) | 0.95× | 0.0% |
| mlkem768-encaps/… | 14.44 ms (1,126 kc) | libcrux: 15.48 ms (1,207 kc) | 0.93× | 0.0% |
| mlkem768-decaps/… | 8.53 ms (666 kc) | libcrux: 18.02 ms (1,405 kc) | 0.47× | 0.0% |

## AEADs

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| aes128-gcm/…/32B | 212.15 µs (517.1 c/B) | 250.99 µs (611.8 c/B) | 243.83 µs (594.3 c/B) | 842.70 µs (2054.1 c/B) |
| aes128-gcm/…/1350B | 4.03 ms (232.7 c/B) | 4.41 ms (254.6 c/B) | 3.78 ms (218.6 c/B) | 14.63 ms (845.5 c/B) |
| aes128-gcm/…/16KB | 46.79 ms (222.8 c/B) | 50.86 ms (242.1 c/B) | 42.93 ms (204.4 c/B) | 170.52 ms (811.8 c/B) |
| aes128-gcm-open/…/32B | 215.30 µs (524.8 c/B) | 255.40 µs (622.5 c/B) | 255.13 µs (621.9 c/B) | 842.58 µs (2053.8 c/B) |
| aes128-gcm-open/…/1350B | 4.04 ms (233.7 c/B) | 4.42 ms (255.6 c/B) | 3.81 ms (220.1 c/B) | 14.97 ms (864.9 c/B) |
| aes128-gcm-open/…/16KB | 46.99 ms (223.7 c/B) | 51.06 ms (243.1 c/B) | 43.13 ms (205.3 c/B) | 174.70 ms (831.7 c/B) |
| aes256-gcm/…/32B | 256.05 µs (624.1 c/B) | 308.27 µs (751.4 c/B) | 299.37 µs (729.7 c/B) | 991.96 µs (2417.9 c/B) |
| aes256-gcm/…/1350B | 4.97 ms (287.4 c/B) | 5.49 ms (317.1 c/B) | 4.39 ms (253.5 c/B) | 17.42 ms (1006.7 c/B) |
| aes256-gcm/…/16KB | 57.81 ms (275.2 c/B) | 63.36 ms (301.6 c/B) | 49.49 ms (235.6 c/B) | 203.21 ms (967.4 c/B) |
| aes256-gcm-open/…/32B | 259.17 µs (631.7 c/B) | 312.69 µs (762.2 c/B) | 300.11 µs (731.5 c/B) | 1.00 ms (2449.1 c/B) |
| aes256-gcm-open/…/1350B | 4.99 ms (288.4 c/B) | 5.51 ms (318.1 c/B) | 4.40 ms (254.4 c/B) | 17.77 ms (1026.8 c/B) |
| aes256-gcm-open/…/16KB | 58.01 ms (276.2 c/B) | 63.56 ms (302.6 c/B) | 49.68 ms (236.5 c/B) | 207.40 ms (987.4 c/B) |
| chacha20poly1305/…/32B | 108.51 µs (264.5 c/B) | 67.26 µs (163.9 c/B) | 68.78 µs (167.7 c/B) | 84.50 µs (206.0 c/B) |
| chacha20poly1305/…/1350B | 898.13 µs (51.9 c/B) | 576.35 µs (33.3 c/B) | 871.61 µs (50.4 c/B) | 916.64 µs (53.0 c/B) |
| chacha20poly1305/…/16KB | 9.74 ms (46.4 c/B) | 6.26 ms (29.8 c/B) | 9.85 ms (46.9 c/B) | 10.25 ms (48.8 c/B) |
| xchacha20poly1305/…/32B | 136.49 µs (332.7 c/B) | - | 86.48 µs (210.8 c/B) | 124.82 µs (304.2 c/B) |
| xchacha20poly1305/…/1350B | 925.87 µs (53.5 c/B) | - | 820.52 µs (47.4 c/B) | 961.62 µs (55.6 c/B) |
| xchacha20poly1305/…/16KB | 9.77 ms (46.5 c/B) | - | 8.98 ms (42.8 c/B) | 10.29 ms (49.0 c/B) |

## QUIC header protection

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| quic-header-protection/aes128/… | 57.37 µs (4,475 c) | 76.45 µs (5,964 c) | - | - |
| quic-header-protection/aes256/… | 79.27 µs (6,183 c) | 105.07 µs (8,195 c) | - | - |
| quic-header-protection/chacha20/… | 16.86 µs (1,315 c) | 19.18 µs (1,496 c) | - | - |

## Key agreement

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| x25519-ecdh/… | 30.28 ms (2,362 kc) | 31.75 ms (2,477 kc) | 30.26 ms (2,360 kc) | 60.43 ms (4,714 kc) |
| p256-ecdh/… | 57.02 ms (4,447 kc) | 123.87 ms (9,662 kc) | 336.71 ms (26,263 kc) | 165.81 ms (12,933 kc) |

## Signatures

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| ed25519-sign/… | 9.87 ms (770 kc) | 11.59 ms (904 kc) | 10.14 ms (791 kc) | 51.63 ms (4,027 kc) |
| ed25519-verify/… | 23.66 ms (1,845 kc) | 23.47 ms (1,831 kc) | 23.70 ms (1,848 kc) | 55.44 ms (4,324 kc) |
| p256-ecdsa-sign/… | 17.13 ms (1,336 kc) | 38.81 ms (3,027 kc) | 172.55 ms (13,459 kc) | 56.03 ms (4,370 kc) |
| p256-ecdsa-verify/… | 54.03 ms (4,214 kc) | 119.09 ms (9,289 kc) | 184.86 ms (14,419 kc) | 140.57 ms (10,965 kc) |

## Hashes and KDFs

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| sha256/…/64B | 112.60 µs (137.2 c/B) | 70.29 µs (85.7 c/B) | 88.39 µs (107.7 c/B) | 93.49 µs (113.9 c/B) |
| sha256/…/1350B | 946.12 µs (54.7 c/B) | 597.11 µs (34.5 c/B) | 825.97 µs (47.7 c/B) | 922.04 µs (53.3 c/B) |
| sha256/…/16KB | 10.65 ms (50.7 c/B) | 6.78 ms (32.3 c/B) | 9.44 ms (45.0 c/B) | 10.62 ms (50.5 c/B) |
| sha512/…/64B | 246.00 µs (299.8 c/B) | 141.08 µs (171.9 c/B) | 244.57 µs (298.1 c/B) | 229.34 µs (279.5 c/B) |
| sha512/…/1350B | 2.44 ms (140.8 c/B) | 1.37 ms (79.1 c/B) | 2.53 ms (146.3 c/B) | 2.42 ms (139.8 c/B) |
| sha512/…/16KB | 28.25 ms (134.5 c/B) | 15.84 ms (75.4 c/B) | 29.49 ms (140.4 c/B) | 28.24 ms (134.4 c/B) |
| sha3-256/…/64B | 224.45 µs (273.5 c/B) | - | 510.71 µs (622.4 c/B) | 209.46 µs (255.3 c/B) |
| sha3-256/…/1350B | 2.10 ms (121.3 c/B) | - | 5.05 ms (291.7 c/B) | 2.03 ms (117.6 c/B) |
| sha3-256/…/16KB | 25.16 ms (119.8 c/B) | - | 60.92 ms (290.0 c/B) | 24.52 ms (116.7 c/B) |
| sha3-512/…/64B | 224.08 µs (273.1 c/B) | - | 510.04 µs (621.6 c/B) | 200.75 µs (244.7 c/B) |
| sha3-512/…/1350B | 3.95 ms (228.2 c/B) | - | 9.55 ms (552.0 c/B) | 3.83 ms (221.3 c/B) |
| sha3-512/…/16KB | 47.16 ms (224.5 c/B) | - | 114.56 ms (545.4 c/B) | 45.96 ms (218.8 c/B) |
| hmac-sha256/…/64B | 266.76 µs (325.1 c/B) | 195.78 µs (238.6 c/B) | 237.68 µs (289.7 c/B) | 249.22 µs (303.7 c/B) |
| hmac-sha256/…/1350B | 1.10 ms (63.6 c/B) | 722.96 µs (41.8 c/B) | 982.62 µs (56.8 c/B) | 1.08 ms (62.1 c/B) |
| hkdf-sha256/extract-expand/… | 467.68 µs (36 kc) | 342.39 µs (27 kc) | 373.23 µs (29 kc) | 418.57 µs (33 kc) |
| hkdf-sha256/expand/… | 138.79 µs (11 kc) | 98.53 µs (7,686 c) | 97.54 µs (7,608 c) | 216.16 µs (17 kc) |

## ML-KEM

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| mlkem768-keygen/… | 12.88 ms (1,005 kc) | - | 31.21 ms (2,434 kc) | 13.62 ms (1,063 kc) |
| mlkem768-encaps/… | 14.44 ms (1,126 kc) | - | 35.35 ms (2,757 kc) | 15.48 ms (1,207 kc) |
| mlkem768-decaps/… | 8.53 ms (666 kc) | - | 39.20 ms (3,057 kc) | 18.02 ms (1,405 kc) |

## Known gaps

Every operation where another Rust library beats rind by more than 10% here, with the decision on it, as rind's `benchmarks/gaps.toml` requires; `cargo xtask report` refuses a gap without an entry in `results/gaps.toml`, and an entry that covers no gap.

| Operations | rind's time over the fastest | Fastest | Decision |
|---|---|---|---|
| `chacha20poly1305/…/1350B`, `chacha20poly1305/…/16KB`, `chacha20poly1305/…/32B` | 1.55× to 1.61× | ring | Assembly against portable Rust: ring links OpenSSL's Thumb-2 ChaCha20 (ChaCha20_ctr32_nohw, from chacha-armv4) here, while rind runs its portable backend because its 32-bit Arm assembly is ARM-state NEON that an M-profile core lacks. 16KB: rind 760 kc against ring's 489 kc; rind is level with RustCrypto (768 kc) and libcrux (799 kc). At 32B rind also pays about 3 kc more per call than RustCrypto (8.5 kc against 5.4 kc). Same gap as on the nRF52840; an M-profile ChaCha20 would close it. |
| `xchacha20poly1305/…/1350B`, `xchacha20poly1305/…/32B` | 1.13× to 1.58× | rustcrypto | Portable against portable: ring has no XChaCha, and RustCrypto's soft ChaCha20 is faster here: 1350B 64.0 kc against rind's 72.2 kc, 32B 6.7 kc against 10.6 kc, where rind's per-call cost (HChaCha20 setup plus the fixed cost seen in chacha20poly1305/32B) dominates. rind beats libcrux at 1350B (75.0 kc) and is within 9% at 16KB (762 kc against RustCrypto's 701 kc). Not profiled. |
| `aes256-gcm-open/…/1350B`, `aes256-gcm-open/…/16KB`, `aes256-gcm/…/1350B`, `aes256-gcm/…/16KB` | 1.13× to 1.17× | rustcrypto | Portable against portable: RustCrypto's soft AES-256-GCM runs 16KB in 3.86 M cycles against rind's 4.51 M (1.17x); rind beats ring (4.94 M) and libcrux (15.9 M). AES-128-GCM is within 10% of RustCrypto (16KB 3.65 M against 3.35 M), so the extra four rounds cost rind more than RustCrypto. Not profiled on this board. |
| `hkdf-sha256/extract-expand/…`, `hmac-sha256/…/1350B`, `hmac-sha256/…/64B`, `sha256/…/1350B`, `sha256/…/16KB`, `sha256/…/64B` | 1.36× to 1.60× | ring | Assembly against portable Rust: ring links OpenSSL's sha256-armv4 (sha256_block_data_order_nohw, Thumb-2), 0.64x rind's cycles at 16KB (529 kc against 831 kc). rind's portable SHA-256 is libcrux-sha2's code and matches libcrux within 1% at 16KB (828 kc); RustCrypto's soft SHA-256 is 0.89x. HMAC and HKDF inherit it. Same gap as on the nRF52840; an M-profile SHA-256 block function would close it. |
| `hkdf-sha256/expand/…` | 1.42× | rustcrypto | Fixed per-call cost on a short output: rind's expand takes 10.8 kc against RustCrypto's 7.6 kc and ring's 7.7 kc (libcrux 16.9 kc), the same pattern as on the nRF52840. Not investigated further. |
| `sha512/…/1350B`, `sha512/…/16KB`, `sha512/…/64B` | 1.74× to 1.78× | ring | Assembly against portable Rust: ring links OpenSSL's sha512-armv4 (sha512_block_data_order_nohw, Thumb-2), 0.56x rind's cycles at 16KB (1.24 M against 2.20 M). rind's portable SHA-512 (libcrux-sha2's code) matches libcrux within 0.1% at 16KB and is 0.96x RustCrypto's. |
| `sha3-512/…/64B` | 1.12× | libcrux | Per-call overhead, not the permutation: rind takes 17.5 kc on every one-block SHA-3 call against libcrux's 15.7-16.3 kc, while at 16KB it is within 3% of libcrux (SHA3-512 3.68 M against 3.58 M). rind is 2.3x faster than RustCrypto on every SHA-3 row. Same pattern as on the nRF52840. Decision: accepted, small. |

## Known-answer checks

Run on the board before any timing, every run. A library failing an operation's check is not raced there.

| Check | Library | Result |
|---|---|---|
| aes128-gcm/1350B-agrees-with-rind | libcrux | pass (10 runs) |
| aes128-gcm/1350B-agrees-with-rind | ring | pass (10 runs) |
| aes128-gcm/1350B-agrees-with-rind | rustcrypto | pass (10 runs) |
| aes128-gcm/nist-gcm-tc2 | libcrux | pass (10 runs) |
| aes128-gcm/nist-gcm-tc2 | rind | pass (10 runs) |
| aes128-gcm/nist-gcm-tc2 | ring | pass (10 runs) |
| aes128-gcm/nist-gcm-tc2 | rustcrypto | pass (10 runs) |
| chacha20poly1305/1350B-agrees-with-rind | libcrux | pass (15 runs) |
| chacha20poly1305/1350B-agrees-with-rind | ring | pass (15 runs) |
| chacha20poly1305/1350B-agrees-with-rind | rustcrypto | pass (15 runs) |
| chacha20poly1305/rfc8439-2.8.2 | libcrux | pass (15 runs) |
| chacha20poly1305/rfc8439-2.8.2 | rind | pass (15 runs) |
| chacha20poly1305/rfc8439-2.8.2 | ring | pass (15 runs) |
| chacha20poly1305/rfc8439-2.8.2 | rustcrypto | pass (15 runs) |
| ed25519/rfc8032-test1 | dalek | pass (5 runs) |
| ed25519/rfc8032-test1 | libcrux | pass (5 runs) |
| ed25519/rfc8032-test1 | rind | pass (5 runs) |
| ed25519/rfc8032-test1 | ring | pass (5 runs) |
| hmac-sha256/rfc4231-tc2 | libcrux | pass (10 runs) |
| hmac-sha256/rfc4231-tc2 | rind | pass (10 runs) |
| hmac-sha256/rfc4231-tc2 | ring | pass (10 runs) |
| hmac-sha256/rfc4231-tc2 | rustcrypto | pass (10 runs) |
| mlkem768/round-trip | libcrux-ml-kem | pass (5 runs) |
| mlkem768/round-trip | rind | pass (5 runs) |
| mlkem768/round-trip | rustcrypto | pass (5 runs) |
| p256-ecdh/agrees-with-rind | ring | pass (5 runs) |
| p256-ecdh/rfc5903-8.1 | libcrux | pass (5 runs) |
| p256-ecdh/rfc5903-8.1 | rind | pass (5 runs) |
| p256-ecdh/rfc5903-8.1 | rustcrypto | pass (5 runs) |
| sha256/16KB-agrees-with-rind | libcrux | pass (5 runs) |
| sha256/16KB-agrees-with-rind | ring | pass (5 runs) |
| sha256/16KB-agrees-with-rind | rustcrypto | pass (5 runs) |
| sha256/fips180-abc | libcrux | pass (5 runs) |
| sha256/fips180-abc | rind | pass (5 runs) |
| sha256/fips180-abc | ring | pass (5 runs) |
| sha256/fips180-abc | rustcrypto | pass (5 runs) |
| sha3-256/16KB-agrees-with-rind | libcrux | pass (5 runs) |
| sha3-256/16KB-agrees-with-rind | rustcrypto | pass (5 runs) |
| sha3-256/fips202-abc | libcrux | pass (5 runs) |
| sha3-256/fips202-abc | rind | pass (5 runs) |
| sha3-256/fips202-abc | rustcrypto | pass (5 runs) |
| sha3-384/16KB-agrees-with-rind | libcrux | pass (5 runs) |
| sha3-384/16KB-agrees-with-rind | rustcrypto | pass (5 runs) |
| sha3-384/fips202-abc | libcrux | pass (5 runs) |
| sha3-384/fips202-abc | rind | pass (5 runs) |
| sha3-384/fips202-abc | rustcrypto | pass (5 runs) |
| sha3-512/16KB-agrees-with-rind | libcrux | pass (5 runs) |
| sha3-512/16KB-agrees-with-rind | rustcrypto | pass (5 runs) |
| sha3-512/fips202-abc | libcrux | pass (5 runs) |
| sha3-512/fips202-abc | rind | pass (5 runs) |
| sha3-512/fips202-abc | rustcrypto | pass (5 runs) |
| sha512/fips180-abc | libcrux | pass (5 runs) |
| sha512/fips180-abc | rind | pass (5 runs) |
| sha512/fips180-abc | ring | pass (5 runs) |
| sha512/fips180-abc | rustcrypto | pass (5 runs) |
| x25519/agrees-with-rind | ring | pass (5 runs) |
| x25519/rfc7748-6.1 | dalek | pass (5 runs) |
| x25519/rfc7748-6.1 | libcrux | pass (5 runs) |
| x25519/rfc7748-6.1 | rind | pass (5 runs) |

## Confidence

Each run is a fresh boot of the same image. With 5 runs the lowest and highest run medians bound the board's median with probability 94% (distribution-free: each run median falls on either side with probability one half); `ns_low` and `ns_high` in `efr32mg24.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run noise only. No relinked build has run on this board yet; read short-input ratios with the README's layout caveat.

## Reproduce

```sh
git clone --recursive https://github.com/steelhead99x/rind-hardware && cd rind-hardware
cargo xtask apply-patches && cargo xtask vendor-ring && cargo xtask build-images mg24
probe-rs run --chip EFR32MG24B220F1536IM48 efr32mg24/images/<bench>.elf   # once per run; back up the board's flash first (efr32mg24/README.md)
cargo xtask report --board efr32mg24 --raw results/raw-efr32mg24 --rind ../rind-hardware/rind --firmware ../rind-hardware/efr32mg24 --gaps results/efr32mg24/gaps.toml --images results/efr32mg24/images.json --out results/efr32mg24
```

Generated by `cargo xtask report` on 2026-11-12; every benchmark, with each run's statistics, is in `efr32mg24-all.json`.
