# Argon2id: scalar rider rounds beside the NEON pairs (2026-11-12)

rind branch `perf/argon2-neon-scalar-rider` (3025440d) on `perf/argon2-interleave` (d7b1f399, see
`results/argon2-interleave/`). Same hosts, toolchain (rustc 1.98.1), harnesses and lock protocol as there.
Pi 5 still capped at 1.5 GHz (`scaling_max_freq`, not changed).

## Idea

After the two-round interleave the A76 NEON fill ran at IPC 1.75, close to the limit of its two
vector pipes, with the integer ALUs idle. Each pass has eight independent rounds: the NEON fill now
runs vector pairs (0,1) and (2,3) each with one scalar round (6, 7) interleaved step by step
(`round_with_scalar!`), then pair (4,5) alone. Scalar BlaMka is `ubfiz` + `madd`, rotations `eor` + `ror`.

## Variants

| id | rind commit | change |
|---|---|---|
| g1-final | d7b1f399 | `perf/argon2-interleave` (baseline here) |
| h1-hybrid | `wip/argon2-hybrid` @ "wip: range loops" | scalar riders in both fills (NEON and SHA3) |
| h2-hybrid-neon | `wip/argon2-hybrid` @ 529de010 | scalar riders in the plain NEON fill only; = 3025440d minus docs |

Codegen (M1 and Pi): the scalar rounds stay scalar (64 `madd`, 64 `ror`, not SLP-vectorized), no
bounds checks (an early draft looped over `[0, 2, 4]` and got 10 `panic_bounds_check` calls; a
`step_by` range removed them), ~no spills (21-23 stack accesses, mostly callee-saved prologue). In
h2 the SHA3 `compress` opcode stream is identical to g1's (588 instructions on the M1).

## Pi 5 (NEON fill; the Pi has no SHA3)

PMU harness, 8 alternations x 5 iterations, `taskset -c 3`, 14:04-14:05 PST, load ~2 (`raw/pi5-pmu-h2.tsv`):

| size | g1 | **h2** | ratio | cycles/blk g1 -> h2 | IPC g1 -> h2 | back-end stall/blk | RustCrypto |
|---|---|---|---|---|---|---|---|
| 1 MiB, t=3 | 3.123 ms (0.2%) | **2.582 ms** (3.5%) | 0.827 | 1522 -> 1259 | 1.75 -> 2.62 | 756 -> 345 | 4.202 ms |
| 16 MiB | 19.905 ms (0.4%) | **18.056 ms** (6.2%) | 0.907 | 1817 -> 1645 | 1.44 -> 1.95 | 1059 -> 744 | 26.437 ms |
| 64 MiB | 91.470 ms (0.3%) | **83.781 ms** (1.1%) | 0.916 | 1758 -> 1584 | 1.48 -> 2.02 | 1002 -> 686 | 119.263 ms |

Criterion, 5 interleaved runs, two sets. The first round of each set started under load (another
job at 14:02; my own Pi tests had just finished at 14:11), which shows in max/spread, not in medians:

| size | set | a0 (original) | g1 | **h2** | h2/g1 | RustCrypto (h2 run) | h2/RustCrypto |
|---|---|---|---|---|---|---|---|
| 1 MiB | r6 14:02-14:04 | - | 3.119 ms | **2.578 ms** | 0.827 | 4.218 ms | 0.611 |
| 16 MiB | r6 | - | 19.994 ms | **17.722 ms** | 0.886 | 26.486 ms | 0.669 |
| 64 MiB | r6 | - | 92.168 ms | **84.629 ms** | 0.918 | 119.071 ms | 0.711 |
| 1 MiB | r7 14:11-14:14 | 5.027 ms* | 3.122 ms | **2.588 ms** | 0.829 | 4.201 ms | 0.616 |
| 16 MiB | r7 | 28.236 ms | 19.863 ms | **18.139 ms** | 0.913 | 26.704 ms | 0.679 |
| 64 MiB | r7 | 126.073 ms | 94.341 ms | **85.753 ms** | 0.909 | 120.253 ms | 0.713 |

(*a0 1 MiB median pulled up by the loaded first round; min 4.728 ms matches the quiet runs.)
Against the original `perf/argon2-neon` fill (a0, 4.728 / 28.116 / 124.111 ms in the quiet
`results/argon2-interleave` run) the Pi NEON fill is now 0.55 / 0.64 / 0.68 of the time.
An early h1 run (`raw/pi5-pmu-h1-LOADED.tsv`, load 3-4) gave the same cycle counts (1259 / 1628 / 1580).

## M1 Pro

- **h1 (riders in the SHA3 fill too), dropped for SHA3**: wall harness, 20 alternations, load ~7
  (`raw/m1pro-sha3-wall-h1-20alt.tsv`): 1 MiB 0.944, 16 MiB 0.918, **64 MiB 1.062** (21.017 -> 22.318 ms,
  min 19.299 -> 21.585 ms). A 10-alternation run gave 0.939 / 0.910 / 1.017. Regression at 64 MiB, so h2
  keeps the SHA3 fill unchanged.
- **h2, default SHA3 path**: identical code; criterion r7 (14:03-14:06, load 32 -> 10) gives
  0.997 / 0.975 / 0.990, i.e. noise.
- **h2, forced NEON fill** (not the M1's default; M-series CPUs have SHA3): wall harness 0.864 / 0.863 /
  0.973 (10 alternations); criterion r7 0.866 / 0.946 / 1.038 with 64 MiB spreads of 24% (g1) and 84% (h2).

## Tests (h2 = 529de010; 3025440d differs in comments and ALGORITHMS.md only, argon2 KATs + clippy re-run on it)

| check | M1 Pro | Pi 5 |
|---|---|---|
| `cargo test --release -p rind --lib` | 561 passed, 33 ignored | 558 passed, 33 ignored |
| `--test crosschecks` (incl. Argon2id vs RustCrypto) | 72 passed | 72 passed |
| debug `--lib argon2` (RFC 9106, reference KAT, `neon_matches_scalar` both fills) | 10 passed | 10 passed |
| `--features wipe` `zeroing` + `heap_scratch` | 11 + 15 passed | 11 + 15 passed |
| forced NEON argon2 lib + crosscheck | 10 + 1 passed | NEON is the default |
| `admin/ct-vartime aarch64-unknown-linux-gnu` | - | rc=0 |
| fmt, `clippy --all-targets --features wipe -D warnings` | clean | - |

Not run: `admin/ct-qemu`, ctgrind, `just ci`. Constant time: the scalar rounds are fixed-latency
`madd`/`add`/`eor`/`ror` on registers with addresses fixed by loop indices; no data-dependent branch
or address. `ct-vartime` (which flags variable-time instructions such as division) passes.

## Not tried

A different split (e.g. two scalar riders per pair, or riders in the row pass only): the A76 is now
at IPC 2.62 at 1 MiB, with front-end stalls still ~1/blk, so remaining headroom is unclear.
