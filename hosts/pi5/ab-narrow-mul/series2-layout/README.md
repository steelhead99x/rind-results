# Series 2's P-256 delta: the generator table's cache-line offset (2026-11-12)

`ab-cpu-detect.*` (one directory up) showed `perf/pi5-cpu-detect` (series 2, lazy MIDR-first multiplier detection)
2.5% slower at P-256 ECDSA sign and 0.5% slower at ECDH than PR #33, with x25519, RSA and P-256 verify flat. This
directory traces that delta to where the linker put the P-256 generator table, and measures the fix now on
`perf/pi5-cpu-detect` (3d7dd094, "perf(p256): align the aarch64 generator table to a cache line").

Raspberry Pi 5 (Cortex-A76, cpufreq capped at 1.5 GHz by `pw-thermal.service`; see `../../NOTES.md`). Setup:
- `taskset -c 3`, criterion `--warm-up-time 1 --measurement-time 3`.
- Variants interleaved with the order rotated each round, bench lock held per round.
- Values are medians, with spread = max/min − 1.

## Evidence

1. **The detection isn't on the hot path.**
   - A standalone harness (`det-pmu/`, `harness-pmu-raw.log`, `s2-harness-pmu.md`) reads the core PMU through `perf_event_open`, 7 interleaved rounds × 9 × 2000 calls.
   - Series 2 retires +1 instruction per P-256 key generation, ECDH or sign on 86de4410, and +5 to +7 on `perf/pi5-narrow-nist`.
   - Branch misses and L1I refills are unchanged.
   - In the harness binary, series 2 is no slower: keygen 37.70 → 37.63 µs, sign 46.27 → 46.22 µs, ECDH 201.27 → 201.24 µs. The delta exists only in some binaries.
2. **All of the time is in one routine.** `perf record` (cycles:u) puts 97.6% of key generation in `p256_scalarmulbase_narrow` for both binaries.
3. **What differs is backend stalls, not misses.** `perf stat` (`s2-perf-stat.md`, 3 runs each):
   - Key generation IPC: 2.223 vs 2.157 (−3.0%).
   - Backend stall cycles per 1000 instructions: 124.8 vs 138.3.
   - L1D, L1I and branch misses per instruction are the same.
4. **Function alignment isn't it.** I rebuilt both bench binaries with `-Cllvm-args=-align-all-functions=6`, which puts every function at a 64-byte boundary (`ab-s2-det2.*`). The delta is unchanged: sign 1.025, keygen 1.031, ECDH 1.006.
5. **The table's address mod 64 predicts it every time.** `CURVE_GENERATOR_PRECOMP` (43 rows of 32 64-byte points) is read by `p256_scalarmulbase` with 16-byte `ldp`s. At an offset of 8 bytes mod 16, one load in four crosses a cache line.

| binary | table address | mod 64 | P-256 keygen / sign |
|---|---|---|---|
| criterion, #33 | 0x4750a0 | 32 | fast (36.4 / 45.1 µs) |
| criterion, #33 + series 2 | 0x477108 | 8 | slow (37.5 / 46.2 µs) |
| criterion, both built with function alignment 64 | 0x4990a0 / 0x499108 | 32 / 8 | fast / slow, same as above |
| harness, #33 and #33 + series 2 | 0x75a08 / 0x76048 | 8 / 8 | both slow (37.7 / 37.6 µs keygen) |
| criterion, narrow-nist / `perf/pi5-cpu-detect` before the fix | 0x47c260 / 0x47ce60 | 32 / 32 | both fast (36.5 / 45.2-45.3 µs) |

## Fix and A/B

The fix stores the non-x86_64 table in a `repr(C, align(64))` wrapper, the way the x86_64 rows already are. A test asserts the alignment, and `admin/p256-precomp` writes the new form.

| A/B (5 rounds) | P-256 keygen | ECDSA sign | ECDH | ECDSA verify |
|---|---|---|---|---|
| `ab-s2-deta`: #33 + series 2 (table +8) / #33 (+32) | 1.031 | 1.025 | 1.006 | 1.000 |
| `ab-s2-deta`: #33 + series 2 + fix (table +0) / #33 (+32) | 1.003 | 1.002 | 1.001 | 1.000 |
| `ab-s2-det3`: `perf/pi5-cpu-detect` + fix (3d7dd094) / narrow-nist | 0.999 | 1.000 | 0.999 | 1.001 |
| `ab-s2-det3`: `perf/pi5-cpu-detect` before the fix / narrow-nist | 0.999 | 1.001 | 0.999 | 1.001 |
| `ab-s2-nista`: narrow-nist + fix / narrow-nist | 1.001 | 1.001 | 1.000 | 1.000 |

All spreads are ≤ 0.2%.
- The fix removes the 2.5-3.1% loss in the binary where the table landed badly.
- Where the table already landed well, the fix is neutral.
- On narrow-nist alone it's also neutral, because that build's table happens to sit at +32. The fix makes the fast case certain rather than lucky.

## Thermal record

Since these runs I sample `vcgencmd measure_temp`, `get_throttled` and `scaling_cur_freq` before each variant and after each round (`run-s2-*.log`).
- `get_throttled` bits 0-3 were set only in round 1, right after the build that preceded it. That was every variant of round 1 in `ab-s2-det2`, and only the first variant in the other three A/Bs. That was 0x6 (capped + throttled) at 84.0-88.9 °C, or 0x8 (soft temperature limit) at 82.3-84.0 °C.
- Every later reading was 0xe0000 (history bits only) at 73.6-81.2 °C.
- Medians without round 1 match the medians with it to ±0.001 (`excl-r1` recomputations agree), so no conclusion depends on round 1.
- The earlier A/B runs in `..` (pk1, rsa1, det1, w7, p521) were taken before this logging existed, so their throttle state wasn't recorded. Their round-to-round spreads (≤ 1.2%, mostly ≤ 0.4%) show no throttling-sized excursions.
- From now on, wait for the SoC to drop below about 78 °C before round 1 after a build.

## Tests of 3d7dd094

- Pi 5, native: `cargo test -p rind --release` with detection asserted narrow and with wide forced. 691 lib tests plus wycheproof, cctv, KATs and the rest pass in both modes. Wipe tests (10 + 15) pass, and release clippy `--deny warnings` is clean.
- M1 Pro, native: default and `RIND_CPU_DISABLE_wide_mul=1`. 693 lib tests plus the rest pass, wipe tests pass, and clippy is clean.
- Not run here: `admin/prove lean`. `P256Table.lean`'s `tabulated` and `P256Ecdsa.lean` name `CURVE_GENERATOR_PRECOMP`, whose Aeneas type is now the wrapper struct. Those statements will likely need `.0` or a `Deref` lemma, and CI's `prove` job has to confirm.
