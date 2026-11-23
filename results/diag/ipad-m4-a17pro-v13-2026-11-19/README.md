<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# iPad Pro (Apple M4) and iPad mini (Apple A17 Pro), one more run each of the built-in variants, 2026-11-19

One more run on each of the two iPads that already have sets under [`../ipad-m4-2026-11-19/`](../ipad-m4-2026-11-19/README.md) and [`../ipad-a17pro-2026-11-19/`](../ipad-a17pro-2026-11-19/README.md). It was taken in the evening with the RindBench v13 app. It is a diagnostic of the rind variants that are built into the app, not the full library comparison: no RustCrypto, OpenSSL, ring or other library is timed here, and the only non-rind row is the aws-lc-rs Ed25519 verify control. The variants are the same as in the two earlier sets, so the variant table is not repeated here beyond the labels.

## What was measured

| | M4 | A17 Pro |
|---|---|---|
| Device | iPad16,6, Apple M4 | iPad16,2, Apple A17 Pro |
| OS | iPadOS 27.2.1 (24B156) | iPadOS 27.0.1 (24A446) |
| App | RindBench v13 | RindBench v13 |
| Saved | 2026-11-19 11:33 PM PT | 2026-11-19 11:47 PM PT |
| Run | 1 run of 7 rounds, after one warm-up round per set | 1 run of 7 rounds, after one warm-up round per set |
| Time | 87 s | 89 s |

Same method as the earlier sets: one iOS app loads one prebuilt library per variant from its `Frameworks` directory and times the same `rind-bench` operations, variant order alternating every round, each sample at least 150 ms (`min_ms=150`). The `ed25519-verify` of aws-lc-rs is built into every variant and is the control. Thermal state, battery state and core placement were not recorded, and iPadOS cannot pin a core. The app was started by a person tapping through its consent screen and "Start test".

Each device has one run, not the three runs that a `results/<host>/<sha>/` publish needs, which is why this sits under `diag/`.

## Files

- `m4-run-1.txt`, `a17pro-run-1.txt`: the runs as the app sent them. Line format: `<set> <w|r> <round> <variant> op<N> <ns>`, `w` the warm-up round, `r` rounds 1 to 7. The only edit is the first line, where the app's install directory (which holds a per-install identifier) became `<app bundle>`.
- `tools/analyze.py`: the same script as in the earlier sets (identical apart from a comment). `python3 tools/analyze.py m4-run-1.txt`.
- `tables.txt`: its output for both runs. It keeps only the rounds whose control agrees within 1.02 across the variants of a set. All 7 rounds were kept except A17 Pro `kk`, which dropped round 1 (control 1.021) and uses 6.

Units are the app's raw units, which `analyze.py` prints as ns per operation. `op<N>` maps to: 0 `ed25519-verify/rind`, 1 `ed25519-verify/aws-lc-rs` (control), 2 `ed25519-sign/rind`, 3 to 8 `chacha20` at 32, 200, 576, 1200, 1350, 16384 bytes, 9 `mlkem768-keygen`, 10 `mlkem768-encaps`, 11 `mldsa65-sign`, 12 `sha3-256/4096`, 13 `sha512/4096`. Each set only reports its own ops.

## Variants

Labels as `analyze.py` and the earlier sets give them.

| Set | Variants | Source |
|---|---|---|
| `ed` | `main` (base), `g824` | main `e97dc453`; `g824` is the head `b14b1ed7` of [steelhead99x/rind#128](https://github.com/steelhead99x/rind/pull/128) |
| `cc` | `cx4` (base), `cx8`, `cx9` | aarch64 ChaCha20 `core_x4` `6ecdfee9`, `core_x8` `1e015593`, `core_x9` `aa6670cf` (head of kixel-ai/rind#823) |
| `kk` | `k0` (base), `k1` | `51808a98` (main before kixel-ai/rind#884), `91d7dc0c` (kixel-ai/rind#884) |

The commits come from the labels in `tools/analyze.py` and the earlier READMEs. That the v13 app embeds those same commits was not re-checked against the app's build inputs; the ratios agree with the earlier sets (for example `g824` verify 0.817 on the M4 in both).

## Medians

Median of the 7 measured rounds of one run, ns, in the app's raw units. Ratio is variant over base; below 1 is faster. These use all 7 rounds; `tables.txt` has the clean-round medians (they differ only for A17 Pro `kk`).

### Ed25519, `g824` against `main`

| Row | M4 main | M4 g824 | M4 ratio | A17 main | A17 g824 | A17 ratio |
|---|---:|---:|---:|---:|---:|---:|
| `ed25519-verify` rind (op0) | 14899 | 12175 | 0.817 | 19989 | 15987 | 0.800 |
| `ed25519-sign` rind (op2) | 3708 | 3694 | 0.996 | 4906 | 4908 | 1.000 |
| `sha512/4096` (op13) | 4097 | 4094 | 0.999 | 5360 | 5366 | 1.001 |

Sign and SHA-512 are the unchanged-code checks.

### ChaCha20, `cx8` and `cx9` against `cx4`

| Bytes (op) | M4 cx4 | M4 cx8 | M4 cx9 | M4 cx8/cx4 | M4 cx9/cx4 | A17 cx4 | A17 cx8 | A17 cx9 | A17 cx8/cx4 | A17 cx9/cx4 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32 (op3) | 92 | 96 | 97 | 1.038 | 1.047 | 112 | 112 | 113 | 1.000 | 1.004 |
| 200 (op4) | 259 | 263 | 183 | 1.014 | 0.707 | 317 | 317 | 221 | 0.999 | 0.697 |
| 576 (op5) | 353 | 268 | 216 | 0.758 | 0.611 | 439 | 309 | 243 | 0.703 | 0.554 |
| 1200 (op6) | 728 | 556 | 447 | 0.764 | 0.614 | 911 | 644 | 512 | 0.707 | 0.562 |
| 1350 (op7) | 803 | 624 | 539 | 0.776 | 0.671 | 1003 | 736 | 614 | 0.734 | 0.613 |
| 16384 (op8) | 8566 | 5895 | 5193 | 0.688 | 0.606 | 10639 | 6362 | 5750 | 0.598 | 0.540 |

### ML-KEM-768, ML-DSA-65 and SHA-3, `k1` against `k0`

| Row (op) | M4 k0 | M4 k1 | M4 ratio | A17 k0 | A17 k1 | A17 ratio |
|---|---:|---:|---:|---:|---:|---:|
| `mlkem768-keygen` (op9) | 10002 | 7777 | 0.778 | 12837 | 10019 | 0.780 |
| `mlkem768-encaps` (op10) | 2876 | 2570 | 0.894 | 3567 | 3154 | 0.884 |
| `mldsa65-sign` (op11) | 48851 | 40352 | 0.826 | 61263 | 52295 | 0.854 |
| `sha3-256/4096` (op12) | 5507 | 5508 | 1.000 | 7187 | 7200 | 1.002 |
| `sha512/4096` (op13) | 4205 | 4204 | 1.000 | 5360 | 5361 | 1.000 |

`sha3-256` and `sha512` are the unchanged-code checks here; the `kk` set has its own SHA-512 timing, which is why it differs from the `ed` set's.

## Not here

- No library other than rind and the aws-lc-rs control. The full library comparison is not part of this app run.
- One run per device. The earlier sets have three.
- No thermal log, no power-state log, no pinned cores.
- The app, its signing and the prebuilt variant libraries are not committed.

(Written by Grok Bot)
