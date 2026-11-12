# Argon2id: closing the Cortex-A76 gap to RustCrypto (2026-11-12)

rind branch `perf/argon2-interleave` (d7b1f399, on `perf/argon2-sha3-umlal` a05b78e4, on
`perf/argon2-neon` 4ad61ae3). Hosts: `pi5` (Raspberry Pi 5, Cortex-A76, no SHA3, Debian,
**`scaling_max_freq` capped at 1.5 GHz** by someone else; not changed) and `m1pro` (Apple M1 Pro, macOS).
Toolchain rustc 1.98.1. Builds use `--cfg rind_cpu_disable`; `neon` mode on the M1 sets
`RIND_CPU_DISABLE_sha3=1`. The Pi has no SHA3, so it always runs the NEON fill.

## Variants

| id | rind commit | change |
|---|---|---|
| a0-before | 4ad61ae3 | `perf/argon2-neon` (one round at a time) |
| e1-final | a05b78e4 | `perf/argon2-sha3-umlal` (umlal in the SHA3 fill only; NEON fill = a0's code) |
| f1 | e795b53a | two rounds at once, `round()` a function: LLVM left it out of line, 107 stack accesses in `compress` (built, not timed) |
| f2-x2inl | wip `wip/argon2-interleave2` | two rounds at once, `round!` macro, inlined, no spills |
| f3-x2umlal | wip `wip/argon2-x2-umlal-neon` | f2 + `umlal` in the NEON fill too |
| g1-final | d7b1f399 | f2 + docs (same opcode stream as f2 on the Pi, same binary as f2 on the M1) |

## Diagnosis: why RustCrypto won on the A76

RustCrypto argon2 0.6's `compress` is plain scalar Rust (no SIMD on aarch64): BlaMka is
`ubfiz` + `madd`, the rotations `eor` + `ror`. Disassembly (`raw/pi5-disasm-summary.txt`):
885 static instructions, 241 stack accesses. rind's NEON `compress`: 373 instructions, 3 stack accesses.

Core PMU (`perf_event_open`, user-only, A76 events CPU_CYCLES, INST_RETIRED, STALL_BACKEND, ...),
per 1 KiB block at 1 MiB / 3 passes, quiet run `raw/pi5-pmu-r4.tsv`, median of 8 alternations:

| | cycles/blk | inst/blk | stall_backend/blk | IPC | L1D refill/blk |
|---|---|---|---|---|---|
| rind a0 (before) | 2307 | 2816 | 1560 (68%) | 1.22 | 11.9 |
| RustCrypto 0.6 | 2048 | 4365 | 451 (22%) | 2.13 | 13.6 |
| **rind g1 (after)** | **1522** | 2664 | 756 (50%) | **1.75** | 17.9 |

rind ran 35% fewer instructions than RustCrypto but at little more than half the IPC: it was
latency-bound, not throughput-bound. One BLAKE2 round is a single dependency chain only two vectors
wide (`a0/a1`, ...), each step a 2-cycle ASIMD add or 4-cycle `umull`, and the eight rounds of a pass
ran back to back; the A76's 128-entry ROB cannot reach the next round (~150 instructions away).
The scalar code has four independent G chains per half-round across three integer ALUs. Front-end
stalls (~1/blk) and cache refills were negligible for both, so neither memory nor decode was the issue.

## Fix (kept): two rounds at once

Each pass runs rounds 2k and 2k+1 together, step by step (`g1`/`g2` take `[[uint64x2_t; 8]; 2]`),
16 state vectors, no spills (5 stack accesses, all outside the rounds). `round` is a macro: as a
function LLVM kept it out of line and passed the state through the stack (variant f1).

### Pi 5, criterion (`rind-bench` argon2 bench), 5 interleaved runs, `taskset -c 3`, 13:43-13:46 PST,
load 1.0, 82-84 C (`raw/pi5-criterion-r5.md`)

| size | a0 before | e1 | **g1 after** | g1/a0 | RustCrypto | g1/RustCrypto |
|---|---|---|---|---|---|---|
| 1 MiB, t=3 | 4.728 ms (0.1%) | 4.731 ms | **3.113 ms** (0.1%) | 0.658 | 4.202 ms | 0.741 |
| 16 MiB, t=1 | 28.116 ms (0.1%) | 28.116 ms | **19.720 ms** (0.3%) | 0.701 | 26.357 ms | 0.748 |
| 64 MiB, t=1 | 124.111 ms (0.3%) | 124.245 ms | **90.356 ms** (0.4%) | 0.728 | 117.839 ms | 0.767 |

(percentages: (max-min)/median over the 5 runs.) PMU harness, same session (`raw/pi5-pmu-r4.tsv`):
1 MiB 4.732 -> 3.123 ms, 16 MiB 28.079 -> 19.919 ms, 64 MiB 124.314 -> 90.550 ms, spreads <= 0.4%.
The first quiet PMU run (`raw/pi5-pmu-r1.tsv`, 13:20) gave the same numbers within 0.3%.
Loaded runs (another agent's tests on the Pi, 92 C, ~1 GHz; `*-LOADED*`) gave g1/a0 0.68 / 0.71 / 0.70.

### M1 Pro, criterion, 5 interleaved runs, 13:39-13:42 PST, load 43 -> 15 (Zoom, Chrome, Falcon running)
(`raw/m1pro-criterion-r6.md`)

| size | mode | e1 before | **g1 after** | ratio | RustCrypto |
|---|---|---|---|---|---|
| 1 MiB | sha3 | 624.95 us (20.9%) | **562.30 us** (2.1%) | 0.900 | 1.073 ms |
| 16 MiB | sha3 | 3.538 ms (5.7%) | **3.233 ms** (8.4%) | 0.914 | 5.882 ms |
| 64 MiB | sha3 | 22.141 ms (13.5%) | **20.496 ms** (10.1%) | 0.926 | 33.047 ms |
| 1 MiB | neon | 799.14 us (4.5%) | **726.69 us** (7.9%) | 0.909 | 1.069 ms |
| 16 MiB | neon | 4.436 ms (11.5%) | **4.078 ms** (2.8%) | 0.919 | 5.822 ms |
| 64 MiB | neon | 26.711 ms (11.7%) | **23.700 ms** (26.2%) | 0.887 | 32.480 ms |

Wall-clock harness rounds on the M1 (`raw/m1pro-*-wall-r*.tsv`, 8-10 alternations, load 45-100)
agree: sha3 0.88-0.97, neon 0.89-0.95. The M1 numbers are noisy in absolute terms; every run had
g1 faster, none had it slower.

## Tried and dropped

- **umlal in the NEON fill too (f3)**: with the rounds interleaved it helps the M1's NEON mode
  (0.783 / 0.823 / 0.864 vs e1, against f2's 0.885 / 0.914 / 0.935, loaded host) but costs the A76
  5.0% / 4.0% / 3.7% (quiet run `raw/pi5-pmu-r5-umlal.tsv`, 1598 vs 1522 cycles/blk). The M1 has
  SHA3 and runs the SHA3 fill, which already uses umlal, so the NEON fill keeps `umull`.
- **round() as a function (f1)**: out of line, state through the stack; superseded by the macro, not timed.
- From the earlier session (`results/argon2/`): a scalar fill on the A76 (4.967 ms, slower than both),
  prefetch (no gain on either machine).
- **Not tried**: a hybrid scalar + NEON fill (run a scalar round on the idle integer pipes alongside
  each NEON pair; the A76 now sits at IPC 1.75 of a 2-vector-pipe ceiling, with ~3 integer ALUs idle),
  and three rounds at once (24 state vectors + temporaries would spill). Both are follow-up candidates.

## Tests (g1, 2038051e; d7b1f399 differs only in comments and ALGORITHMS.md)

| check | M1 Pro | Pi 5 |
|---|---|---|
| `cargo test --release -p rind --lib` | 561 passed, 33 ignored | 558 passed, 33 ignored |
| `--test crosschecks` (incl. Argon2id vs RustCrypto) | 72 passed | 72 passed |
| debug `--lib argon2` (RFC 9106, reference KAT, `neon_matches_scalar` both fills) | 10 passed (re-run on d7b1f399) | 10 passed |
| `--features wipe` `zeroing` + `heap_scratch` | 11 + 15 passed | 11 + 15 passed |
| forced NEON (`--cfg rind_cpu_disable`, `RIND_CPU_DISABLE_sha3=1`) argon2 lib + crosscheck | 10 + 1 passed | NEON is the default |
| `low::abi` tests (no new `asm!`) | 109 passed (d7b1f399) | in lib run |
| `admin/ct-vartime aarch64-unknown-linux-gnu` | - | rc=0 |
| `cargo fmt --check`, `clippy --all-targets --features wipe -D warnings` | clean | - |

Not run: `admin/ct-qemu`, ctgrind, `just ci`. Constant time: the change reorders the same
fixed-latency vector instructions on registers; no branch or address depends on secret data.
Every run also asserts rind's and RustCrypto's tags agree (the PMU harness checks before timing).

## Method / scripts

- `scripts/argon2-pmu/`: harness; rind (path dep) and RustCrypto argon2 =0.6.0, interleaved per
  iteration, wall time plus a PMU group on Linux. `scripts/argon2-pmu-build{,-mac}.sh` builds it
  per variant tarball; `scripts/argon2-pmu-ab.sh` alternates variants under `~/rind-work/.bench.lock`;
  `scripts/argon2-pmu-sum.py` summarises (median of per-alternation medians, spread, ratio).
- Criterion runs: `scripts/argon2-build-variants.sh`, `scripts/argon2-run-ab.sh` (from `results/argon2/`),
  `scripts/pi5-argon2-g1.sh`, `scripts/pi5-argon2-g1-confirm.sh`.
