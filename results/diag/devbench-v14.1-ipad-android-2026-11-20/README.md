<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# RindBench All v14.1 (rind-devbench): iPad Pro M4, iPad mini A17 Pro and Galaxy A16, one Quick run each, 2026-11-20

These are the first runs from the all-library device app, `rind-devbench`, which [steelhead99x/rind#258](https://github.com/steelhead99x/rind/pull/258) added to rind. The app builds rind-bench's own bench files unchanged and times every library on every operation that `admin/bench-report` has, using the same row IDs. Its report is a bench-report host result (schema 1) with `device`, `timing` and `toolchain` blocks added.

Each device has **one Quick run**: 1 round of 5 samples of 50 ms each, with a 150 ms warm-up per row. That is fewer than criterion's defaults, and it is not the three runs that a `results/<host>/<sha>/` publish needs, so this set sits under `diag/`. In each report, `ns` is the per-round median. With only one round, `ns_low` and `ns_high` equal `ns` and say nothing about spread. Thermal state was not recorded, and neither OS lets the app pin a core.

## What was measured

| | M4 | A17 Pro | Galaxy A16 |
|---|---|---|---|
| Device | iPad16,6, Apple M4 | iPad16,2, Apple A17 Pro | Samsung SM-A166U1, Exynos s5e8535 |
| OS | iPadOS 27.2.1 | iPadOS 27.0.1 | Android 16 (SDK 36) |
| App | RindBench All 14.1 | RindBench All 14.1 | RindBench All 14.1 |
| rind commit | `0d907ec7` (#258's merge into main) | `cd2ea6a0`, the same code as `0d907ec7` (see below) | `0d907ec7` |
| Started (UTC) | 2026-11-20 09:09:21 | 2026-11-20 08:51:48 | 2026-11-20 09:14:49 |
| Rows | 598, 0 errors | 598, 0 errors | 570, 0 errors |
| Libraries | rind, rind (DIT held), aws-lc-rs, ring, RustCrypto, graviola, libcrux, OpenSSL (vendored), verified-garbage | same | same, without rind (DIT held) |
| Toolchain | rustc 1.98.1, `aarch64-apple-ios` | rustc 1.98.1, `aarch64-apple-ios` | rustc 1.98.1, `aarch64-linux-android` |

The default selection was used: every library, and every operation in the report. The extra "More:" groups were left off.

The **A17 Pro commit `cd2ea6a0`** is a local commit from before #258's commits had their author line rewritten for pushing. It is not on GitHub. Its tree (`07134452`) is identical to `c59bfae9` on rind main. The only commits between `c59bfae9` and `0d907ec7` are the iOS app's version number and the app's README. So `rind/`, `rind-bench/`, `rind-rustls/`, `rind-jsonwebtoken/`, the criterion shim and the app core are the same code as `0d907ec7`. The A17 Pro's later run on the `0d907ec7` build was cut short when iOS paused it behind the lock screen (43 of 598 rows), so it is not included.

## Files

- `m4/`, `a17pro/`, `a16/`: one run each, as the device saved it. `<run_id>.json` is the bench-report document. `<run_id>.jsonl` is the full run log: a `start` line with config, crate versions and device, then every `row` with its five samples, then `round`, `done` and the same `report`. No file was edited.
- `tools/headline.py`: prints rind and the other libraries for the headline operations, straight from each `.json`.
- `headline.txt`: its output for the three runs.

## Headline operations

`ns` as recorded in each report: rind, then the fastest other library on that device. `headline.txt` lists every library.

| Operation | M4 | A17 Pro | Galaxy A16 |
|---|---|---|---|
| Ed25519 verify | rind 12.84 µs; graviola 15.73 µs | rind 16.08 µs; graviola 20.05 µs | rind 39.73 µs; graviola 51.59 µs |
| Ed25519 sign | rind 3906.3 ns; graviola 4023.0 ns | rind 4899.0 ns; graviola 5009.1 ns | rind 11.01 µs; graviola 11.26 µs |
| ChaCha20-Poly1305, 1350 B | rind 769.3 ns; verified-garbage 729.7 ns | rind 896.5 ns; verified-garbage 831.2 ns | rind 2024.7 ns; verified-garbage 1976.9 ns |
| ML-KEM-768 keygen | rind 10.23 µs; aws-lc-rs 6823.8 ns | rind 13.16 µs; aws-lc-rs 8276.0 ns | rind 21.45 µs; graviola 21.23 µs |
| ML-KEM-768 encaps | rind 10.04 µs; aws-lc-rs 7260.4 ns | rind 13.70 µs; aws-lc-rs 8876.9 ns | rind 21.92 µs; verified-garbage 22.08 µs |
| ML-DSA-65 sign | rind 50.33 µs; aws-lc-rs 65.88 µs | rind 60.91 µs; aws-lc-rs 75.97 µs | rind 137.84 µs; aws-lc-rs 241.56 µs |
| SHA3-256, 1350 B | rind 1888.9 ns; RustCrypto 1255.6 ns | rind 2399.4 ns; RustCrypto 1352.6 ns | rind 3153.9 ns; verified-garbage 3102.2 ns |
| SHA-512, 1350 B | rind 1494.3 ns; RustCrypto 716.3 ns | rind 1851.0 ns; ring 836.7 ns | rind 2858.7 ns; ring 2883.1 ns |
| SPAKE2+ P-256 prover start | rind 26.12 µs; RustCrypto 258.31 µs | rind 32.22 µs; RustCrypto 302.12 µs | rind 82.21 µs; RustCrypto 688.49 µs |
| SPAKE2+ P-256 verifier start | rind 88.99 µs; RustCrypto 647.88 µs | rind 111.09 µs; RustCrypto 772.46 µs | rind 280.79 µs; RustCrypto 1729.80 µs |
| SPAKE2+ P-256 prover finish | rind 62.82 µs; RustCrypto 374.97 µs | rind 77.49 µs; RustCrypto 463.92 µs | rind 199.94 µs; RustCrypto 1041.15 µs |
| SPAKE2+ P-256 verifier finish | rind 55.7 ns; RustCrypto 32.7 ns | rind 59.2 ns; RustCrypto 34.9 ns | rind 69.0 ns; RustCrypto 100.8 ns |
| TLS 1.3 full handshake, X25519 | rind 128.64 µs; graviola 143.08 µs | rind 165.39 µs; graviola 191.56 µs | rind 409.68 µs; graviola 468.46 µs |
| TLS 1.3 full handshake, X25519MLKEM768 | rind 156.51 µs; graviola 173.68 µs | rind 202.69 µs; graviola 227.33 µs | rind 495.40 µs; graviola 580.65 µs |

These are single Quick runs. Do not read a small gap off them without a longer, repeated run.
