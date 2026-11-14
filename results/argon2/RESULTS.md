# Argon2id NEON fill: `umlal` BlaMka in the SHA3 build (and what did not help)

Branch work on steelhead99x/rind `perf/argon2-neon` (Argon2id, upstream [moq-dev/rind#634](https://github.com/moq-dev/rind/pull/634)),
continued from `4ad61ae3` (`perf(argon2): rotate with SHA3's xar on aarch64`). Measured 2026-11-12 (PST).

## Variants

| label | commit | change |
|---|---|---|
| `a0-before` | `4ad61ae3` | branch head before this work (NEON fill, SHA3 `xar`) |
| `a1-umlal` | `af4a8cfc` (dropped) | BlaMka written as two `vmlal_u32` intrinsics. **LLVM folds them back into `umull` + adds**: the SHA3 `compress` body is instruction-for-instruction identical to `a0` (96 `add.2d`, 16+16 `umull`/`umull2`, 0 `umlal`). Kept only as a noise control. |
| `c1-asm1` | `b373b262` | BlaMka as `(x + x·y) + (y + x·y)`: one `asm!` block of `umlal`/`umlal2` into each input, then one add. SHA3 `compress`: 32 `add.2d` + 32 `umlal` + 32 `umlal2` (was 96 `add.2d` + 32 `umull*`), 24 register `mov`s for the inputs still live. |
| `c2-asm1prefetch` | `78d88485` | `c1` + `prfm pldl1keep` of the next block's reference in Argon2id's data-independent half (address known one block ahead). |
| `c3-chain` | wip (dropped) | `umlal` twice into `x + y` (no `mov`s, but the two `umlal`s are serial): slower than `c1`. |
| `b1-asmumlal` | wip (dropped) | `c1` with four separate `asm!` statements: LLVM stopped inlining `round()` and passed the eight vectors through memory. Not timed; fixed by using one `asm!` block. |

## Method

- rind-bench `benches/argon2.rs` (Argon2id, 1 lane; `1MiB-3` = 1 MiB × 3 passes, `16MiB-1`, `64MiB-1`), rind and RustCrypto `argon2` 0.6 in the same binary; criterion settings are the bench's own (10 samples, 200 ms warm-up, 1 s measurement); the estimate is criterion's slope (or mean), as `admin/bench-report` parses it.
- One release binary per variant, all built with `RUSTFLAGS=--cfg rind_cpu_disable` so `RIND_CPU_DISABLE_sha3=1` times the plain-NEON fill (`neon`) next to the SHA3 one (`sha3`) on the M1 Pro. The Pi 5 has no SHA3, so it only runs `neon`.
- Interleaved: each round runs every variant × fill once; 5 rounds; table cells are the median of 5, with min–max spread. RustCrypto in the same binary is the drift control.
- Mac: rustc 1.98.1 (rustup), macOS 14.0, not pinned (macOS has no affinity). Pi 5: rustc 1.98.1, `taskset -c 3`.
- All timed runs under the shared `~/rind-work/.bench.lock`, except Mac round 1 (lock introduced mid-run). Scripts: `scripts/argon2-build-variants.sh`, `scripts/argon2-run-ab.sh`, `scripts/crit-parse.py` (now `cargo xtask crit-parse`). Raw per-variant tables: `raw/`.

## Apple M1 Pro (`m1pro`)

Round 2 (`raw/m1pro-round2.md`, 10:06–10:10 PST), median of 5:

| size | fill | before `a0` | `umlal` `c1` | c1/a0 | `c1`+prefetch `c2` | c2/c1 | RustCrypto |
|---|---|---|---|---|---|---|---|
| 1 MiB, t=3 | SHA3 | 720.6 µs | **623.2 µs** | 0.865 | 623.8 µs | 1.001 | 1.055 ms |
| 1 MiB, t=3 | NEON | 800.1 µs | **693.7 µs** | 0.867 | 697.6 µs | 1.006 | 1.053 ms |
| 16 MiB, t=1 | SHA3 | 3.888 ms | **3.367 ms** | 0.866 | 3.372 ms | 1.001 | 5.85 ms |
| 16 MiB, t=1 | NEON | 4.367 ms | **3.824 ms** | 0.876 | 3.808 ms | 0.996 | 5.70 ms |
| 64 MiB, t=1 | SHA3 | 21.02 ms | **19.74 ms** | 0.939 | 19.42 ms | 0.984 | 30.1 ms |
| 64 MiB, t=1 | NEON | 24.13 ms | **21.01 ms** | 0.871 | 20.53 ms | 0.977 | 29.9 ms |

Spreads (max/min − 1 over 5 runs): 1 MiB 0.8–6%, 16 MiB 2.8–7.4%, 64 MiB 7.8–19.6% (the 64 MiB rows are memory- and page-fault-bound and noisy on this laptop).

Round 3 (`raw/m1pro-round3.md`, same binaries plus `c3-chain`, 10:16–10:21 PST) reproduces it: `c1`/`a0` = 0.865 (1 MiB SHA3), 0.871 (1 MiB NEON), 0.865 / 0.849 (16 MiB), 0.963 / 0.907 (64 MiB); `c3-chain` was 1.011× `c1` (SHA3) and 1.078× (NEON) at 1 MiB, so the parallel form was kept.

Round 1 (`raw/m1pro-round1.md`): `a1-umlal` (intrinsics, identical fill code) measured within ±1.5% of `a0` at 1 and 16 MiB, which is this setup's noise floor; `a2-prefetch` (prefetch alone) was also within noise.

**Verdict on the M1 Pro:** `umlal` BlaMka: −13 to −15% at 1 and 16 MiB on both fills, −6 to −13% at 64 MiB. Prefetch: no change beyond noise (≤2.3% at 64 MiB with 8–19% spread).

## Raspberry Pi 5 (`pi5`, Cortex-A76, no SHA3)

Round 1 (`raw/pi5-round1.md`, 11:58–12:01 PST, `taskset -c 3`, load ≈ 1.0 = the benchmark alone, 73–76 °C), median of 5:

| size | before `a0` | `umlal` `c1` | c1/a0 | `c1`+prefetch `c2` | c2/c1 | RustCrypto |
|---|---|---|---|---|---|---|
| 1 MiB, t=3 | 4.729 ms | 4.847 ms | **1.025** | 4.857 ms | 1.002 | 4.201 ms |
| 16 MiB, t=1 | 28.11 ms | 28.71 ms | **1.021** | 28.59 ms | 0.996 | 26.35 ms |
| 64 MiB, t=1 | 124.4 ms | 126.8 ms | **1.019** | 125.5 ms | 0.990 | 118.2 ms |

Spreads 0.0–0.3% (rind) and 0.1–1.2% (RustCrypto): the 2% regression is real on this core.

Round 2 (`raw/pi5-round2-scalar-probe.md`, 12:12–12:15 PST) added `d1-scalar`, a probe (branch
`wip/argon2-scalar-probe`, not for merge) that runs rind's portable scalar permutation on aarch64. Its medians:
`a0` 4.729 ms, `c1` 4.847 ms (both reproduced to 4 digits), `d1-scalar` 4.967 ms at 1 MiB; 28.6 / 29.0 / 30.0 ms
at 16 MiB; 126.3 / 127.8 / 133.2 ms at 64 MiB. Another job ran during the first round (max spreads up to 50%), but
the medians are stable. So rind's own scalar code is slower still, and **RustCrypto's portable fill (4.20 ms) beats
every rind fill on the Cortex-A76 by about 12%**: an open gap, not addressed here.

## Decision and final commit

- `umlal`: faster on the M1 Pro (SHA3 and NEON builds), slower on the A76 (NEON build). The final commit uses it only
  in the SHA3 build (SHA3 cores: Apple M-series, Neoverse V1/V2/N2, …; the A76 and N1 have none) and keeps the `umull`
  form in the plain NEON build. On the M1 Pro the shipped SHA3 path keeps the −13.5% (1 MiB); the M1's NEON-only
  path (only reachable with `RIND_CPU_DISABLE_sha3`) gives up its −13%; the Pi is unchanged.
- Prefetch (`78d88485`): no gain beyond noise on either machine (M1: 0.98–1.01×, Pi: 0.99–1.00×). **Dropped**; kept only
  on the local branch `perf/argon2-neon-prefetch` for reference, not pushed.
- `vmlal_u32` intrinsics: identical code to before (LLVM folds them). Dropped.
- Serial `umlal` chain (`c3`): slower than the parallel form. Dropped.

Final: steelhead99x/rind `perf/argon2-sha3-umlal` = `4ad61ae3` + one commit, `perf(argon2): accumulate BlaMka's product
with umlal in the SHA3 fill`.

Codegen check of the final binary (`e1-final`), opcode stream of each `compress` (objdump):

- M1 Pro: `neon::compress` identical to `a0`'s (358 instructions, 0 `umlal`); `sha3::compress` identical to `c1`'s
  (322 instructions, 32 `umlal` + 32 `umlal2`).
- Pi 5: `neon::compress` identical to `a0`'s (374 lines).

So the shipped timings are the `c1` SHA3 numbers above on the M1 and the `a0` numbers on the Pi.

Confirmation runs of `e1-final` against `a0`:

- Pi 5 (`raw/pi5-round3-final.md`, 12:22–12:24 PST): `e1`/`a0` = 1.001 (1 MiB), 1.000 (16 MiB), 1.003 (64 MiB) — as expected
  for identical code. Absolute times are ~55% above round 1: other jobs kept the load at 4.6–5.7 and the SoC at
  89.8–91.4 °C (thermal throttling), so use these only as an A/B.
- M1 Pro (`raw/m1pro-round5-INVALID-loaded.md`) and the M1 scalar probe (`raw/m1pro-round4-INVALID-loaded.md`): **invalid**.
  Load average 50–105 from Spotlight (`mds_stores`) and CrowdStrike Falcon (`FXPredictService`) scanning the freshly
  built binaries; spreads 20–168%. Not used.

## Tests and checks (final commit)

| check | M1 Pro (macOS, aarch64-apple-darwin) | Pi 5 (Debian, aarch64-unknown-linux-gnu) |
|---|---|---|
| `cargo test --release -p rind --lib` (vector submodules checked out) | 561 passed, 33 ignored | 558 passed, 33 ignored |
| `cargo test --release -p rind --test crosschecks` (incl. Argon2id vs RustCrypto) | 72 passed | 72 passed |
| `cargo test -p rind --lib argon2` (debug: RFC 9106 vector, reference KAT, `neon_matches_scalar` for both builds) | 10 passed | 10 passed |
| `--features wipe` `zeroing`, `heap_scratch` | 11 + 15 passed | 11 + 15 passed |
| `admin/ct-vartime aarch64-unknown-linux-gnu` | – | rc=0 |
| `cargo fmt --check`, `cargo clippy -p rind --all-targets --features wipe -D warnings` | clean | – |
| `RIND_CPU_DISABLE_sha3=1` (NEON build) `--release --lib argon2`, `--cfg rind_cpu_disable` | 10 passed (re-run at a05b78e4) | NEON is the default there |

Not run: `admin/ct-qemu` (`ct_qemu_argon2`, needs the `.#ct-qemu-aarch64` Nix shell on Linux), ctgrind (Linux x86
only), `just ci`. `umlal`/`umlal2` are fixed-latency register multiply-accumulates; the change adds no branch or
memory access, so the constant-time argument of the fill is unchanged.
