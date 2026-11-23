<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# RindBench All v14.1 (rind-devbench): iPad Pro M4 and iPad mini A17 Pro, one Full run each, 2026-11-22

These follow the Quick runs in [devbench-v14.1-ipad-android-2026-11-20](../devbench-v14.1-ipad-android-2026-11-20/README.md). The app is the same all-library device app, `rind-devbench`, from [steelhead99x/rind#258](https://github.com/steelhead99x/rind/pull/258). Each report is a bench-report host result (schema 1) with `device`, `timing` and `toolchain` blocks added.

Each device has **one Full run**: 3 rounds of 10 samples of 100 ms each, with a 300 ms warm-up per row. In each report, `ns` is the median of the per-round medians, and `ns_low` / `ns_high` are the min and max per-round median. Thermal state was not recorded, and iPadOS does not let the app pin a core.

The default report operations and the extra "More:" groups were all on (every library, and every encryption-related operation the app offers). That is 1,959 rows per device, against 598 in the Quick set, which left the extra groups off.

## What was measured

| | M4 | A17 Pro |
|---|---|---|
| Device | iPad16,6, Apple M4 | iPad16,2, Apple A17 Pro |
| OS | iPadOS 27.2.1 | iPadOS 27.0.1 |
| App | RindBench All 14.1 | RindBench All 14.1 |
| rind commit | `0d907ec7` (#258's merge into main) | `0d907ec7` |
| Started (UTC) | 2026-11-22 16:24:04 | 2026-11-22 16:17:59 |
| Finished (UTC) | 2026-11-22 18:47:12 | 2026-11-22 18:41:26 |
| Rows | 1,959, 0 errors | 1,959, 0 errors |
| Libraries | rind, rind (DIT held), aws-lc-rs, ring, RustCrypto, graviola, libcrux, OpenSSL (vendored), verified-garbage | same |
| Toolchain | rustc 1.98.1, `aarch64-apple-ios` | rustc 1.98.1, `aarch64-apple-ios` |

## Files

- `m4/`, `a17pro/`: one run each, as the app sent it. `<run_id>.json` is the bench-report document. `<run_id>.jsonl` is the full run log: a `start` line with config, crate versions and device, then every `row` with its ten samples, then `round`, `done` and the same `report`. No number was edited.
- `tools/headline.py`: prints rind and the other libraries for the headline operations, straight from each `.json`.
- `headline.txt`: its output for the two runs.

## Headline operations

`ns` as recorded in each report: rind, then the fastest other library on that device. `headline.txt` lists every library.

| Operation | M4 | A17 Pro |
|---|---|---|
| Ed25519 verify | rind 12.92 µs; graviola 15.88 µs | rind 18.35 µs; graviola 23.15 µs |
| Ed25519 sign | rind 3962.0 ns; graviola 4064.7 ns | rind 5376.0 ns; graviola 5506.7 ns |
| ChaCha20-Poly1305, 1350 B | rind 782.0 ns; verified-garbage 736.5 ns | rind 967.5 ns; verified-garbage 924.0 ns |
| ML-KEM-768 keygen | rind 10.38 µs; aws-lc-rs 6890.5 ns | rind 13.88 µs; aws-lc-rs 8756.4 ns |
| ML-KEM-768 encaps | rind 10.21 µs; aws-lc-rs 7346.1 ns | rind 13.77 µs; aws-lc-rs 9383.8 ns |
| ML-DSA-65 sign | rind 50.42 µs; aws-lc-rs 66.18 µs | rind 67.76 µs; aws-lc-rs 85.43 µs |
| SHA3-256, 1350 B | rind 1908.6 ns; rustcrypto 1267.1 ns | rind 2498.3 ns; rustcrypto 1565.6 ns |
| SHA-512, 1350 B | rind 1508.1 ns; rustcrypto 716.5 ns | rind 1878.6 ns; rustcrypto 835.3 ns |
| SPAKE2+ P-256 prover start | rind 26.34 µs; rustcrypto 272.11 µs | rind 36.10 µs; rustcrypto 338.82 µs |
| SPAKE2+ P-256 verifier start | rind 89.46 µs; rustcrypto 652.92 µs | rind 126.34 µs; rustcrypto 848.35 µs |
| SPAKE2+ P-256 prover finish | rind 63.17 µs; rustcrypto 388.87 µs | rind 85.76 µs; rustcrypto 501.85 µs |
| SPAKE2+ P-256 verifier finish | rind 55.7 ns; rustcrypto 33.0 ns | rind 64.4 ns; rustcrypto 37.6 ns |
| TLS 1.3 full handshake, X25519 | rind 129.31 µs; graviola 144.61 µs | rind 168.03 µs; graviola 192.78 µs |
| TLS 1.3 full handshake, X25519MLKEM768 | rind 158.92 µs; graviola 175.56 µs | rind 207.80 µs; graviola 233.73 µs |

These Full runs are the source for the follow-up comments on steelhead99x/rind#259 through #266. `verified-garbage` and `rind-dit-held` are never counted as "fastest" in those quest tables.

(Written by Grok Bot)
