# rind on an nRF52840 (Seeed T1000-E)

rind-bench's own benches, run unchanged but for the aws-lc-rs rows, on the nRF52840 of a Seeed T1000-E at 64 MHz, timed with the Cortex-M4's DWT cycle counter. Every operation and ID is `admin/bench-report`'s.

- rind: upstream `99b509a680cf0ad0cd8b7d9c03f7360e7fd95cef`, plus the port's patches: `0001-fix-portable-build-MD5-on-Xtensa-whose-inline-assemb.patch`, `0002-feat-bench-leave-aws-lc-rs-rows-out-on-Xtensa-and-ba.patch`, `0003-feat-build-for-bare-metal-targets-without-std.patch`, `ring-0.17.14-thumbv7em.patch`
- Firmware images (one per bench file; `images.sha256` beside this file):

  | Image | sha256 | rind | rind-hardware | legacy rind-bench | Built |
  |---|---|---|---|---|---|
  | aes_gcm | `175f6220bf624eb2…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | chacha20poly1305 | `66c7228d51db4ac8…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | cipher | `0338fe1f95874f5f…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | ed25519 | `63005609c0cffae3…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | hkdf | `a31671c7eaa3029c…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | hmac | `ea09235adce44cec…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | mlkem768 | `a6442d42c7721568…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | p256 | `854f8cfb08835a7e…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | sha2 | `e75d1f6137a6935c…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | x25519 | `a206f54be8941dd6…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |
  | xchacha20poly1305 | `31a38e4c1b2dcd53…` | `99b509a6` | ancestor of `8d9c1bb` | the uncommitted tree that became c778f64 (2026-11-12 02:23 PST) | 2026-11-12 02:12-02:14 PST |

- OS: none (bare metal, no_std); the board's UF2 bootloader and S140 SoftDevice stay in flash, unused
- Runs: 33 of 33 completed, one firmware image per rind-bench bench file
  - aes_gcm run 1: ok, 167 results, 2026-11-12T10:14:30+00:00 to 2026-11-12T10:16:34+00:00 (UTC)
  - aes_gcm run 2: ok, 167 results, 2026-11-12T10:16:34+00:00 to 2026-11-12T10:18:38+00:00 (UTC)
  - aes_gcm run 3: ok, 167 results, 2026-11-12T10:18:38+00:00 to 2026-11-12T10:20:42+00:00 (UTC)
  - chacha20poly1305 run 1: ok, 56 results, 2026-11-12T10:20:51+00:00 to 2026-11-12T10:21:30+00:00 (UTC)
  - chacha20poly1305 run 2: ok, 56 results, 2026-11-12T10:21:30+00:00 to 2026-11-12T10:22:09+00:00 (UTC)
  - chacha20poly1305 run 3: ok, 56 results, 2026-11-12T10:22:09+00:00 to 2026-11-12T10:22:48+00:00 (UTC)
  - cipher run 1: ok, 42 results, 2026-11-12T10:22:59+00:00 to 2026-11-12T10:23:49+00:00 (UTC)
  - cipher run 2: ok, 42 results, 2026-11-12T10:23:49+00:00 to 2026-11-12T10:24:39+00:00 (UTC)
  - cipher run 3: ok, 42 results, 2026-11-12T10:24:39+00:00 to 2026-11-12T10:25:30+00:00 (UTC)
  - ed25519 run 1: ok, 12 results, 2026-11-12T10:25:55+00:00 to 2026-11-12T10:26:29+00:00 (UTC)
  - ed25519 run 2: ok, 12 results, 2026-11-12T10:26:29+00:00 to 2026-11-12T10:27:03+00:00 (UTC)
  - ed25519 run 3: ok, 12 results, 2026-11-12T10:27:03+00:00 to 2026-11-12T10:27:36+00:00 (UTC)
  - hkdf run 1: ok, 38 results, 2026-11-12T10:27:52+00:00 to 2026-11-12T10:28:20+00:00 (UTC)
  - hkdf run 2: ok, 38 results, 2026-11-12T10:28:20+00:00 to 2026-11-12T10:28:49+00:00 (UTC)
  - hkdf run 3: ok, 38 results, 2026-11-12T10:28:49+00:00 to 2026-11-12T10:29:17+00:00 (UTC)
  - hmac run 1: ok, 102 results, 2026-11-12T10:29:28+00:00 to 2026-11-12T10:30:37+00:00 (UTC)
  - hmac run 2: ok, 102 results, 2026-11-12T10:30:37+00:00 to 2026-11-12T10:31:47+00:00 (UTC)
  - hmac run 3: ok, 102 results, 2026-11-12T10:31:47+00:00 to 2026-11-12T10:32:56+00:00 (UTC)
  - mlkem768 run 1: ok, 12 results, 2026-11-12T10:35:52+00:00 to 2026-11-12T10:36:32+00:00 (UTC)
  - mlkem768 run 2: ok, 12 results, 2026-11-12T10:36:32+00:00 to 2026-11-12T10:37:11+00:00 (UTC)
  - mlkem768 run 3: ok, 12 results, 2026-11-12T10:37:11+00:00 to 2026-11-12T10:37:51+00:00 (UTC)
  - p256 run 1: ok, 21 results, 2026-11-12T10:41:13+00:00 to 2026-11-12T10:42:18+00:00 (UTC)
  - p256 run 2: ok, 21 results, 2026-11-12T10:42:18+00:00 to 2026-11-12T10:43:24+00:00 (UTC)
  - p256 run 3: ok, 21 results, 2026-11-12T10:43:24+00:00 to 2026-11-12T10:44:30+00:00 (UTC)
  - sha2 run 1: ok, 68 results, 2026-11-12T10:44:42+00:00 to 2026-11-12T10:45:37+00:00 (UTC)
  - sha2 run 2: ok, 68 results, 2026-11-12T10:45:37+00:00 to 2026-11-12T10:46:32+00:00 (UTC)
  - sha2 run 3: ok, 68 results, 2026-11-12T10:46:32+00:00 to 2026-11-12T10:47:28+00:00 (UTC)
  - x25519 run 1: ok, 9 results, 2026-11-12T10:47:43+00:00 to 2026-11-12T10:48:15+00:00 (UTC)
  - x25519 run 2: ok, 9 results, 2026-11-12T10:48:15+00:00 to 2026-11-12T10:48:47+00:00 (UTC)
  - x25519 run 3: ok, 9 results, 2026-11-12T10:48:47+00:00 to 2026-11-12T10:49:19+00:00 (UTC)
  - xchacha20poly1305 run 1: ok, 21 results, 2026-11-12T10:49:28+00:00 to 2026-11-12T10:49:55+00:00 (UTC)
  - xchacha20poly1305 run 2: ok, 21 results, 2026-11-12T10:49:55+00:00 to 2026-11-12T10:50:21+00:00 (UTC)
  - xchacha20poly1305 run 3: ok, 21 results, 2026-11-12T10:50:21+00:00 to 2026-11-12T10:50:48+00:00 (UTC)

Each cell is the median over runs of each run's median time per operation, with cycles per byte for byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. rind runs its portable Rust backend here (its 32-bit Arm assembly needs ARM state and NEON, which an M-profile core lacks); ring keeps its Thumb-2 assembly for SHA-2, AES and GHASH. There is no hardware baseline: the chip's CryptoCell CC310 needs Nordic's closed library.

## Headline: rind against the fastest other Rust library

| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |
|---|---|---|---|---|
| aes128-gcm/…/32B | 284.24 µs (568.5 c/B) | rustcrypto: 304.07 µs (608.1 c/B) | 0.93× | 0.0% |
| aes128-gcm/…/1350B | 5.43 ms (257.5 c/B) | rustcrypto: 5.15 ms (244.3 c/B) | 1.05× | 0.0% |
| aes128-gcm/…/16KB | 63.21 ms (246.9 c/B) | rustcrypto: 58.89 ms (230.0 c/B) | 1.07× | 0.0% |
| aes128-gcm-open/…/32B | 287.52 µs (575.0 c/B) | rustcrypto: 313.99 µs (628.0 c/B) | 0.92× | 0.0% |
| aes128-gcm-open/…/1350B | 5.46 ms (259.0 c/B) | rustcrypto: 5.21 ms (246.8 c/B) | 1.05× | 0.0% |
| aes128-gcm-open/…/16KB | 63.58 ms (248.4 c/B) | rustcrypto: 59.45 ms (232.2 c/B) | 1.07× | 0.0% |
| aes256-gcm/…/32B | 338.78 µs (677.6 c/B) | rustcrypto: 373.13 µs (746.3 c/B) | 0.91× | 0.0% |
| aes256-gcm/…/1350B | 6.62 ms (313.7 c/B) | rustcrypto: 6.27 ms (297.1 c/B) | 1.06× | 0.0% |
| aes256-gcm/…/16KB | 77.03 ms (300.9 c/B) | rustcrypto: 71.50 ms (279.3 c/B) | 1.08× | 0.0% |
| aes256-gcm-open/…/32B | 342.02 µs (684.0 c/B) | rustcrypto: 376.98 µs (754.0 c/B) | 0.91× | 0.0% |
| aes256-gcm-open/…/1350B | 6.65 ms (315.3 c/B) | rustcrypto: 6.30 ms (298.7 c/B) | 1.06× | 0.0% |
| aes256-gcm-open/…/16KB | 77.40 ms (302.3 c/B) | rustcrypto: 71.87 ms (280.7 c/B) | 1.08× | 0.0% |
| chacha20poly1305/…/32B | 153.38 µs (306.8 c/B) | ring: 89.14 µs (178.3 c/B) | 1.72× | 0.0% |
| chacha20poly1305/…/1350B | 1.27 ms (60.4 c/B) | ring: 728.02 µs (34.5 c/B) | 1.75× | 0.0% |
| chacha20poly1305/…/16KB | 13.84 ms (54.1 c/B) | ring: 7.86 ms (30.7 c/B) | 1.76× | 0.0% |
| xchacha20poly1305/…/32B | 3.50 ms (6992.2 c/B) | rustcrypto: 3.45 ms (6893.2 c/B) | 1.01× | 1.8% |
| xchacha20poly1305/…/1350B | 4.65 ms (220.3 c/B) | rustcrypto: 4.38 ms (207.8 c/B) | 1.06× | 1.0% |
| xchacha20poly1305/…/16KB | 17.42 ms (68.0 c/B) | rustcrypto: 14.78 ms (57.7 c/B) | 1.18× | 0.7% |
| quic-header-protection/aes128/… | 74.67 µs (4,779 c) | ring: 101.54 µs (6,499 c) | 0.74× | 0.0% |
| quic-header-protection/aes256/… | 101.87 µs (6,520 c) | ring: 138.07 µs (8,836 c) | 0.74× | 0.0% |
| quic-header-protection/chacha20/… | 23.34 µs (1,494 c) | ring: 24.29 µs (1,554 c) | 0.96× | 0.0% |
| x25519-ecdh/… | 45.33 ms (2,901 kc) | rustcrypto: 45.29 ms (2,899 kc) | 1.00× | 0.2% |
| p256-ecdh/… | 81.32 ms (5,204 kc) | ring: 171.80 ms (10,995 kc) | 0.47× | 0.1% |
| ed25519-sign/… | 12.53 ms (802 kc) | rustcrypto: 12.77 ms (817 kc) | 0.98× | 0.0% |
| ed25519-verify/… | 30.66 ms (1,962 kc) | ring: 29.90 ms (1,914 kc) | 1.03× | 0.0% |
| p256-ecdsa-sign/… | 24.28 ms (1,554 kc) | ring: 57.48 ms (3,678 kc) | 0.42× | 0.3% |
| p256-ecdsa-verify/… | 72.91 ms (4,666 kc) | ring: 157.14 ms (10,057 kc) | 0.46× | 0.0% |
| sha256/…/64B | 152.93 µs (152.9 c/B) | ring: 106.16 µs (106.2 c/B) | 1.44× | 0.0% |
| sha256/…/1350B | 1.42 ms (67.2 c/B) | ring: 885.37 µs (42.0 c/B) | 1.60× | 0.0% |
| sha256/…/16KB | 16.28 ms (63.6 c/B) | ring: 10.02 ms (39.1 c/B) | 1.63× | 0.0% |
| sha512/…/64B | 242.71 µs (242.7 c/B) | ring: 205.84 µs (205.8 c/B) | 1.18× | 0.0% |
| sha512/…/1350B | 2.33 ms (110.7 c/B) | ring: 1.95 ms (92.6 c/B) | 1.19× | 0.0% |
| sha512/…/16KB | 26.98 ms (105.4 c/B) | ring: 22.54 ms (88.0 c/B) | 1.20× | 0.0% |
| hmac-sha256/…/64B | 359.53 µs (359.5 c/B) | ring: 293.17 µs (293.2 c/B) | 1.23× | 0.0% |
| hmac-sha256/…/1350B | 1.62 ms (76.8 c/B) | ring: 1.07 ms (50.9 c/B) | 1.51× | 0.0% |
| hkdf-sha256/extract-expand/… | 610.42 µs (39 kc) | ring: 504.58 µs (32 kc) | 1.21× | 0.0% |
| hkdf-sha256/expand/… | 172.80 µs (11 kc) | rustcrypto: 132.55 µs (8,483 c) | 1.30× | 0.0% |
| mlkem768-keygen/… | 45.60 ms (2,918 kc) | libcrux: 26.24 ms (1,680 kc) | 1.74× | 0.2% |
| mlkem768-encaps/… | 43.65 ms (2,794 kc) | libcrux: 23.81 ms (1,524 kc) | 1.83× | 0.3% |
| mlkem768-decaps/… | 18.36 ms (1,175 kc) | libcrux: 22.43 ms (1,435 kc) | 0.82× | 0.0% |

## AEADs

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| aes128-gcm/…/32B | 284.24 µs (568.5 c/B) | 334.95 µs (669.9 c/B) | 304.07 µs (608.1 c/B) | 1.11 ms (2225.4 c/B) |
| aes128-gcm/…/1350B | 5.43 ms (257.5 c/B) | 5.87 ms (278.5 c/B) | 5.15 ms (244.3 c/B) | 19.26 ms (913.0 c/B) |
| aes128-gcm/…/16KB | 63.21 ms (246.9 c/B) | 67.85 ms (265.0 c/B) | 58.89 ms (230.0 c/B) | 224.48 ms (876.9 c/B) |
| aes128-gcm-open/…/32B | 287.52 µs (575.0 c/B) | 339.76 µs (679.5 c/B) | 313.99 µs (628.0 c/B) | 1.10 ms (2194.3 c/B) |
| aes128-gcm-open/…/1350B | 5.46 ms (259.0 c/B) | 5.91 ms (280.1 c/B) | 5.21 ms (246.8 c/B) | 19.49 ms (923.9 c/B) |
| aes128-gcm-open/…/16KB | 63.58 ms (248.4 c/B) | 68.22 ms (266.5 c/B) | 59.45 ms (232.2 c/B) | 227.50 ms (888.7 c/B) |
| aes256-gcm/…/32B | 338.78 µs (677.6 c/B) | 407.61 µs (815.2 c/B) | 373.13 µs (746.3 c/B) | 1.29 ms (2587.0 c/B) |
| aes256-gcm/…/1350B | 6.62 ms (313.7 c/B) | 7.23 ms (342.8 c/B) | 6.27 ms (297.1 c/B) | 22.82 ms (1081.7 c/B) |
| aes256-gcm/…/16KB | 77.03 ms (300.9 c/B) | 83.50 ms (326.2 c/B) | 71.50 ms (279.3 c/B) | 266.33 ms (1040.3 c/B) |
| aes256-gcm-open/…/32B | 342.02 µs (684.0 c/B) | 412.28 µs (824.6 c/B) | 376.98 µs (754.0 c/B) | 1.29 ms (2576.2 c/B) |
| aes256-gcm-open/…/1350B | 6.65 ms (315.3 c/B) | 7.27 ms (344.4 c/B) | 6.30 ms (298.7 c/B) | 22.90 ms (1085.7 c/B) |
| aes256-gcm-open/…/16KB | 77.40 ms (302.3 c/B) | 83.88 ms (327.6 c/B) | 71.87 ms (280.7 c/B) | 267.39 ms (1044.5 c/B) |
| chacha20poly1305/…/32B | 153.38 µs (306.8 c/B) | 89.14 µs (178.3 c/B) | 90.12 µs (180.2 c/B) | 122.42 µs (244.8 c/B) |
| chacha20poly1305/…/1350B | 1.27 ms (60.4 c/B) | 728.02 µs (34.5 c/B) | 1.10 ms (52.3 c/B) | 1.33 ms (63.2 c/B) |
| chacha20poly1305/…/16KB | 13.84 ms (54.1 c/B) | 7.86 ms (30.7 c/B) | 12.41 ms (48.5 c/B) | 14.89 ms (58.2 c/B) |
| xchacha20poly1305/…/32B | 3.50 ms (6992.2 c/B) | - | 3.45 ms (6893.2 c/B) | 3.50 ms (6994.3 c/B) |
| xchacha20poly1305/…/1350B | 4.65 ms (220.3 c/B) | - | 4.38 ms (207.8 c/B) | 4.69 ms (222.2 c/B) |
| xchacha20poly1305/…/16KB | 17.42 ms (68.0 c/B) | - | 14.78 ms (57.7 c/B) | 18.35 ms (71.7 c/B) |

## QUIC header protection

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| quic-header-protection/aes128/… | 74.67 µs (4,779 c) | 101.54 µs (6,499 c) | - | - |
| quic-header-protection/aes256/… | 101.87 µs (6,520 c) | 138.07 µs (8,836 c) | - | - |
| quic-header-protection/chacha20/… | 23.34 µs (1,494 c) | 24.29 µs (1,554 c) | - | - |

## Key agreement

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| x25519-ecdh/… | 45.33 ms (2,901 kc) | 47.74 ms (3,055 kc) | 45.29 ms (2,899 kc) | 70.11 ms (4,487 kc) |
| p256-ecdh/… | 81.32 ms (5,204 kc) | 171.80 ms (10,995 kc) | 462.32 ms (29,588 kc) | 193.17 ms (12,363 kc) |

## Signatures

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| ed25519-sign/… | 12.53 ms (802 kc) | 15.61 ms (999 kc) | 12.77 ms (817 kc) | 45.16 ms (2,890 kc) |
| ed25519-verify/… | 30.66 ms (1,962 kc) | 29.90 ms (1,914 kc) | 30.72 ms (1,966 kc) | 47.70 ms (3,053 kc) |
| p256-ecdsa-sign/… | 24.28 ms (1,554 kc) | 57.48 ms (3,678 kc) | 235.04 ms (15,042 kc) | 69.81 ms (4,468 kc) |
| p256-ecdsa-verify/… | 72.91 ms (4,666 kc) | 157.14 ms (10,057 kc) | 253.17 ms (16,203 kc) | 161.13 ms (10,312 kc) |

## Hashes and KDFs

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| sha256/…/64B | 152.93 µs (152.9 c/B) | 106.16 µs (106.2 c/B) | 123.29 µs (123.3 c/B) | 133.44 µs (133.4 c/B) |
| sha256/…/1350B | 1.42 ms (67.2 c/B) | 885.37 µs (42.0 c/B) | 1.24 ms (58.7 c/B) | 1.40 ms (66.3 c/B) |
| sha256/…/16KB | 16.28 ms (63.6 c/B) | 10.02 ms (39.1 c/B) | 14.31 ms (55.9 c/B) | 16.25 ms (63.5 c/B) |
| sha512/…/64B | 242.71 µs (242.7 c/B) | 205.84 µs (205.8 c/B) | 235.14 µs (235.1 c/B) | 222.13 µs (222.1 c/B) |
| sha512/…/1350B | 2.33 ms (110.7 c/B) | 1.95 ms (92.6 c/B) | 2.39 ms (113.3 c/B) | 2.31 ms (109.6 c/B) |
| sha512/…/16KB | 26.98 ms (105.4 c/B) | 22.54 ms (88.0 c/B) | 27.78 ms (108.5 c/B) | 26.95 ms (105.3 c/B) |
| hmac-sha256/…/64B | 359.53 µs (359.5 c/B) | 293.17 µs (293.2 c/B) | 314.50 µs (314.5 c/B) | 346.87 µs (346.9 c/B) |
| hmac-sha256/…/1350B | 1.62 ms (76.8 c/B) | 1.07 ms (50.9 c/B) | 1.43 ms (68.0 c/B) | 1.60 ms (76.0 c/B) |
| hkdf-sha256/extract-expand/… | 610.42 µs (39 kc) | 504.58 µs (32 kc) | 513.81 µs (33 kc) | 570.57 µs (37 kc) |
| hkdf-sha256/expand/… | 172.80 µs (11 kc) | 145.22 µs (9,294 c) | 132.55 µs (8,483 c) | 293.36 µs (19 kc) |

## ML-KEM

| Operation | rind | ring | rustcrypto | libcrux |
|---|---|---|---|---|
| mlkem768-keygen/… | 45.60 ms (2,918 kc) | - | 49.49 ms (3,167 kc) | 26.24 ms (1,680 kc) |
| mlkem768-encaps/… | 43.65 ms (2,794 kc) | - | 49.38 ms (3,161 kc) | 23.81 ms (1,524 kc) |
| mlkem768-decaps/… | 18.36 ms (1,175 kc) | - | 48.84 ms (3,126 kc) | 22.43 ms (1,435 kc) |

## Known gaps

Every operation where another Rust library beats rind by more than 10% here, with the decision on it, as rind's `benchmarks/gaps.toml` requires; `tools/report.py` refuses a gap without an entry in `results/gaps.toml`, and an entry that covers no gap.

| Operations | rind's time over the fastest | Fastest | Decision |
|---|---|---|---|
| `chacha20poly1305/…/1350B`, `chacha20poly1305/…/16KB`, `chacha20poly1305/…/32B` | 1.72× to 1.76× | ring | Assembly against portable Rust: ring assembles OpenSSL's Thumb-2 ChaCha20 (chacha-armv4) here, while rind runs its portable backend, because its 32-bit Arm assembly is ARM-state NEON that an M-profile core lacks. rind's portable code is 0.93x libcrux's and 1.15x RustCrypto's at 1350B. A Thumb-2 (or DSP-extension) ChaCha20 for M-profile would close it. |
| `xchacha20poly1305/…/16KB` | 1.18× | rustcrypto | Portable against portable: ring has no XChaCha, and RustCrypto's soft ChaCha20 runs 16 KB in 0.85x rind's cycles (946 kc against 1,115 kc); rind is 0.95x libcrux. At 1350B and below the HChaCha20 key setup dominates and all three are within 6%. Same fix as chacha20poly1305: an M-profile ChaCha20. |
| `hkdf-sha256/extract-expand/…`, `hmac-sha256/…/1350B`, `hmac-sha256/…/64B`, `sha256/…/1350B`, `sha256/…/16KB`, `sha256/…/64B` | 1.21× to 1.63× | ring | Assembly against portable Rust: ring assembles OpenSSL's sha256-armv4 here (Thumb-2, no NEON needed), 0.62x rind's cycles at 16 KB. rind's portable SHA-256 is libcrux-sha2's code and matches libcrux within 1% on every length, and RustCrypto's soft SHA-256 is 0.88x. HMAC and HKDF inherit it. An M-profile SHA-256 block function would close it. |
| `hkdf-sha256/expand/…` | 1.30× | rustcrypto | Fixed per-call cost on a 64-byte output: rind's expand takes 11.1 kc against RustCrypto's 8.5 kc and ring's 9.3 kc, about 2.5 kc of setup over three SHA-256 blocks; with the same SHA-256 the long rows track libcrux. Not investigated further; the same pattern does not show on SHA-384/512, where rind is within 14% of ring and level with RustCrypto. |
| `sha512/…/1350B`, `sha512/…/16KB`, `sha512/…/64B` | 1.18× to 1.20× | ring | Assembly against portable Rust: ring assembles OpenSSL's sha512-armv4 (Thumb-2), 0.84x rind's cycles. rind's portable SHA-512 (libcrux-sha2's code) matches libcrux within 1% and beats RustCrypto's soft SHA-512 (1.03x theirs). |
| `mlkem768-encaps/…`, `mlkem768-keygen/…` | 1.74× to 1.83× | libcrux | Compute, not randomness (measured, results/diag/nrf52840-mlkem-*): every library draws the same 64 bytes per keygen and 32 per encaps through getrandom, about 8.7 kc a byte from the RNG peripheral (0.56 M and 0.28 M cycles), so the RNG adds the same to each. With a seeded SplitMix64 behind getrandom for all three, rind's keygen is 2.42 M cycles against libcrux's 1.13 M (2.15x) and RustCrypto's 2.65 M, and encaps 2.56 M against 1.25 M (2.05x) and 2.92 M. rind's decaps stays faster than libcrux's (1.19 M against 1.44 M). Keygen and encaps both expand the matrix A from SHAKE128 while rind's decaps can reuse its key's, so rind's 64-bit-lane Keccak on this 32-bit core is the likely cost; not yet profiled. |

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
| sha512/fips180-abc | libcrux | pass (3 runs) |
| sha512/fips180-abc | rind | pass (3 runs) |
| sha512/fips180-abc | ring | pass (3 runs) |
| sha512/fips180-abc | rustcrypto | pass (3 runs) |
| x25519/agrees-with-rind | ring | pass (3 runs) |
| x25519/rfc7748-6.1 | dalek | pass (3 runs) |
| x25519/rfc7748-6.1 | libcrux | pass (3 runs) |
| x25519/rfc7748-6.1 | rind | pass (3 runs) |

## Confidence

Each run is a fresh boot of the same image. With 3 runs the lowest and highest run medians bound the board's median with probability 75% (distribution-free: each run median falls on either side with probability one half); `ns_low` and `ns_high` in `nrf52840.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run noise only. No relinked build has been run on this board, so layout sensitivity here is unmeasured.

The `legacy rind-bench` column and the `legacy_rind_bench` field are provenance from the private `steelhead99x/rind-bench` history that built these images. That repository was archived on 2026-11-12 and published as [`steelhead99x/rind-hardware`](https://github.com/steelhead99x/rind-hardware) (squash root `8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3`). Each image's `rind_hardware` field is the public commit when the trees match (the RP2040 set maps to `8d9c1bb`), or `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3` when the recorded tree is earlier in the squashed history. The SHA recorded at build time is in `legacy_rind_bench`. See `tools/legacy-rind-bench-map.json`. The Python tools in the commands below produced this set and have since been replaced by `cargo xtask`. To rebuild today, follow the current RESULTS.md using `rind-hardware`.

## Reproduce

```sh
git clone --recursive https://github.com/steelhead99x/rind-hardware && cd rind-hardware
scripts/apply-patches.sh && scripts/vendor-ring.sh && (cd nrf52840 && ./build-images.sh)
cd ../rind-results && tools/run-all-nrf.sh ../rind-hardware/nrf52840/images   # double-tap reset for the UF2 drive; back up the board's firmware first
python3 tools/report.py --board nrf52840 --raw results/raw-nrf52840 --firmware ../rind-hardware/nrf52840 --gaps results/nrf52840/gaps.toml --images results/nrf52840/images.json --out results/nrf52840
python3 tools/verify.py
```

Generated by `tools/report.py` on 2026-11-12; every benchmark, with each run's statistics, is in `nrf52840-all.json`.
