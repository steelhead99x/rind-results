<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# iPad Pro (Apple M4) runs for rind#128, #188, upstream #823 and #884, 2026-11-19

Raw timings from one Apple M4 iPad (iPad16,6, iPadOS 27.2.1) for three comparisons. This is the Apple M4 row that [steelhead99x/rind#128](https://github.com/steelhead99x/rind/pull/128) and [#188](https://github.com/steelhead99x/rind/pull/188) list as waiting. The ChaCha20 and ML-KEM/ML-DSA sets have no fork PR; they are here so their numbers have a source.

## What was measured

| | |
|---|---|
| Device | iPad16,6, Apple M4, iPadOS 27.2.1, app in the foreground with the screen kept on |
| Runner | One iOS app that loads one prebuilt library per variant from its `Frameworks` directory and times the same `rind-bench` operations in each. Variant order alternates every round |
| Rounds | 3 runs of 7 rounds each, after one warm-up round per set. Each sample runs for at least 150 ms (`min_ms=150`) |
| Control | `ed25519-verify` of aws-lc-rs, built into every variant. A round is kept when its control agrees within 1.02 across the variants of that set. The tables take the median of the kept rounds |
| Not recorded | Thermal state, battery state and core placement. iPadOS gives no way to pin a core |

`run-N.txt` is one run as the runner printed it. The only edit is the first line, where the app's install directory (which holds a per-install identifier) became `<app bundle>`. Line format: `<set> <w|r> <round> <variant> op<N> <ns>`, with `w` the warm-up round. `tools/analyze.py` maps `op<N>` to the operation and prints the tables in `tables.txt`: `python3 tools/analyze.py run-1.txt`.

## Variants

| Set | Variant | rind commit | What it is |
|---|---|---|---|
| `ed` | `main` | `e97dc453` | main, the base |
| `ed` | `g824` | `b14b1ed7` | #128's head. #188 carries the same Ed25519 code (`rind/src/mid/ed25519.rs` is byte-identical) |
| `cc` | `cx4` | `6ecdfee9` | aarch64 ChaCha20 `core_x4`, the base |
| `cc` | `cx8` | `1e015593` | `core_x8`, eight blocks per pass |
| `cc` | `cx9` | `aa6670cf` | `core_x9`, the head of kixel-ai/rind#823 |
| `kk` | `k0` | `51808a98` | main before kixel-ai/rind#884 |
| `kk` | `k1` | `91d7dc0c` | kixel-ai/rind#884 |

## Ed25519 verify, #128 against main

Ratios of the median of 7 clean rounds, per run. Below 1 is faster than main.

| Row | main (run 1, 2, 3) | #128 (run 1, 2, 3) | #128 / main |
|---|---|---|---|
| `ed25519-verify` rind | 15.04, 15.08, 15.01 µs | 12.29, 12.30, 12.19 µs | 0.817, 0.816, 0.812 |
| `ed25519-sign` rind | 3.73, 3.73, 3.71 µs | 3.73, 3.74, 3.72 µs | 1.001, 1.004, 1.003 |
| `sha512/4096` rind | 4.13, 4.12, 4.09 µs | 4.13, 4.13, 4.10 µs | 0.998, 1.002, 1.002 |

The control agreed within 1.007 across the two variants in every round of all three runs. Sign and SHA-512 are the unchanged-code checks.

## ChaCha20, `core_x8` and `core_x9` against `core_x4`

Ratios to `core_x4`, per run (1, 2, 3). Below 1 is faster. Run 2's second round (control 1.022) is dropped, so it uses 6 rounds.

| Bytes | `core_x9` / `core_x4` | `core_x8` / `core_x4` |
|---:|---|---|
| 32 | 1.047, 1.032, 1.039 | 1.041, 1.039, 1.032 |
| 200 | 0.701, 0.701, 0.701 | 1.022, 1.015, 1.017 |
| 576 | 0.627, 0.615, 0.614 | 0.766, 0.757, 0.758 |
| 1200 | 0.627, 0.619, 0.621 | 0.776, 0.767, 0.768 |
| 1350 | 0.681, 0.675, 0.676 | 0.790, 0.784, 0.784 |
| 16384 | 0.617, 0.612, 0.612 | 0.702, 0.697, 0.698 |

`core_x4` takes 92.3 to 93.0 ns at 32 B and 8.56 µs at 16 KB. SHA-512 is within 1.006 of `core_x4` in every variant.

## ML-KEM and ML-DSA, kixel-ai/rind#884 against its parent

Ratios to `k0`, per run (1, 2, 3). Run 3 drops its last round (control 1.036), so it uses 6 rounds.

| Row | `k1` / `k0` |
|---|---|
| `mlkem768-keygen` | 0.774, 0.781, 0.779 |
| `mlkem768-encaps` | 0.895, 0.894, 0.892 |
| `mldsa65-sign` | 0.839, 0.831, 0.835 |
| `sha3-256/4096` | 0.998, 1.001, 1.000 |
| `sha512/4096` | 0.997, 1.000, 1.000 |

M1 Pro neutral, M4 improved; cpu classification on M4/iOS not verified.

## Not here

- The app, its signing and the prebuilt variant libraries are not committed.
- No other iPad, no thermal log and no power-state log.

(Written by Grok Bot)
