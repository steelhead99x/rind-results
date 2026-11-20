<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# iPad mini (Apple A17 Pro) runs for rind#128, #188, upstream #823 and #884, 2026-11-19

Raw timings from one Apple A17 Pro iPad mini (iPad16,2, iPadOS 27.0.1) for the same three comparisons as [the Apple M4 set](../ipad-m4-2026-11-19/README.md), with the same runner, variants and method. It fills the A17 Pro row for [steelhead99x/rind#128](https://github.com/steelhead99x/rind/pull/128) and [#188](https://github.com/steelhead99x/rind/pull/188). The ChaCha20 and ML-KEM/ML-DSA sets have no fork PR; they are here so their numbers have a source.

## What was measured

| | |
|---|---|
| Device | iPad16,2, Apple A17 Pro, iPadOS 27.0.1, app in the foreground with the screen kept on |
| Runner | One iOS app that loads one prebuilt library per variant from its `Frameworks` directory and times the same `rind-bench` operations in each. Variant order alternates every round |
| Rounds | 3 runs of 7 rounds each, after one warm-up round per set. Each sample runs for at least 150 ms (`min_ms=150`) |
| Control | `ed25519-verify` of aws-lc-rs, built into every variant. A round is kept when its control agrees within 1.02 across the variants of that set. The tables take the median of the kept rounds. When fewer than 5 rounds are kept, `tools/analyze.py` prints a warning and uses all 7 |
| Not recorded | Thermal state, battery state and core placement. iPadOS gives no way to pin a core |

Only the first three runs of the device are here. Later runs overlapped other work on the app and are left out. `run-N.txt` is one run as the runner printed it. The only edit is the first line, where the app's install directory (which holds a per-install identifier) became `<app bundle>`. Line format: `<set> <w|r> <round> <variant> op<N> <ns>`, with `w` the warm-up round. `tools/analyze.py` maps `op<N>` to the operation and prints the tables in `tables.txt`: `python3 tools/analyze.py run-1.txt`.

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

Ratios of the median of 7 clean rounds, per run (1, 2, 3). Below 1 is faster than main.

| Row | main | #128 | #128 / main |
|---|---|---|---|
| `ed25519-verify` rind | 20.01, 19.98, 19.99 µs | 15.99, 16.01, 16.00 µs | 0.799, 0.801, 0.800 |
| `ed25519-sign` rind | 4.91, 4.90, 4.90 µs | 4.91, 4.91, 4.90 µs | 1.001, 1.001, 1.001 |
| `sha512/4096` rind | 5.36, 5.36, 5.36 µs | 5.37, 5.36, 5.36 µs | 1.001, 1.000, 1.000 |

The control agreed within 1.008 across the two variants in every round of all three runs. Sign and SHA-512 are the unchanged-code checks.

## ChaCha20, `core_x8` and `core_x9` against `core_x4`

Ratios to `core_x4`, per run (1, 2, 3). Below 1 is faster. Run 3 keeps only 3 rounds within the control limit (the control was 1.023 to 1.038 in rounds 4 to 7), so its table uses all 7 rounds; its ratios match runs 1 and 2 within 0.007.

| Bytes | `core_x9` / `core_x4` | `core_x8` / `core_x4` |
|---:|---|---|
| 32 | 1.006, 1.006, 1.004 | 1.002, 1.001, 1.001 |
| 200 | 0.692, 0.696, 0.695 | 0.999, 0.999, 1.000 |
| 576 | 0.554, 0.554, 0.554 | 0.700, 0.702, 0.702 |
| 1200 | 0.562, 0.562, 0.563 | 0.706, 0.707, 0.707 |
| 1350 | 0.613, 0.613, 0.613 | 0.733, 0.734, 0.734 |
| 16384 | 0.540, 0.541, 0.547 | 0.597, 0.598, 0.598 |

`core_x4` takes 112.4 ns at 32 B and 10.64 µs at 16 KB. SHA-512 is within 1.001 of `core_x4` in every variant.

## ML-KEM and ML-DSA, kixel-ai/rind#884 against its parent

Ratios to `k0`, per run (1, 2, 3). Runs 1 and 3 drop the rounds whose control moved more than 1.02 (one and one), so they use 6 rounds.

| Row | `k1` / `k0` |
|---|---|
| `mlkem768-keygen` | 0.762, 0.781, 0.784 |
| `mlkem768-encaps` | 0.879, 0.891, 0.885 |
| `mldsa65-sign` | 0.827, 0.845, 0.840 |
| `sha3-256/4096` | 1.003, 1.003, 1.002 |
| `sha512/4096` | 1.000, 1.001, 0.992 |

kixel-ai/rind#884 gains on both the Apple M4 and the Apple A17 Pro, by a similar amount; whether rind classifies either chip as Apple was not verified.

## Not here

- The app, its signing and the prebuilt variant libraries are not committed.
- No thermal log and no power-state log.

(Written by Grok Bot)
