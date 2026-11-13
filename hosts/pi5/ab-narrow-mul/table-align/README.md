# fix/p256-table-align on its own (2026-11-12)

`fix/p256-table-align` (8e6f1670) is the P-256 generator-table alignment fix from `../series2-layout/` as a single commit on `perf/pi5-narrow-nist` (529a9d64). It includes the Lean proof updates. The commit applies cleanly to PR #33's tip 603b7120, to `main` 6aba8055, and to 86de4410.

## A/B vs 529a9d64 (`ab-p256-table-align.*`, `run-align.log`)

Raspberry Pi 5, Cortex-A76, cpufreq capped at 1.5 GHz by `pw-thermal.service` (see `../../NOTES.md`). Setup:
- `taskset -c 3`, criterion `--warm-up-time 1 --measurement-time 3`.
- 5 rounds, variants interleaved with the order rotated, bench lock held per round.
- After the build, I waited for the SoC to cool below 78 °C (77.9 °C) before round 1.
- Thermal samples were taken before each variant and after each round. Every one read `get_throttled=0xe0000`, with bits 0-3 clear, so there was no current throttling, cap or soft limit. Readings were 74.1-79.6 °C at 1.5 GHz.

Table address (`nm`): 529a9d64 0x47c260 (mod 64 = 32); this branch 0x47c440 (mod 64 = 0).

| op | narrow-nist µs (spread) | + fix µs (spread) | ratio |
|---|---|---|---|
| p256-keygen | 36.5 (0.0%) | 36.5 (0.0%) | 1.000 |
| p256-ecdsa-sign | 45.2 (0.0%) | 45.2 (0.0%) | 1.000 |
| p256-ecdh | 200.0 (0.0%) | 199.9 (0.0%) | 1.000 |
| p256-ecdsa-verify | 192.6 (0.0%) | 192.6 (0.0%) | 1.000 |

The result is neutral, as expected: the base's table already sat at +32. The binary where the table landed at +8 is in `../series2-layout/`, and there the same change removes a 2.5-3.1% loss.

## Lean (`lean-build.log`, `translate-tail.log`, `scripts/`)

`admin/prove lean` was run on the Pi 5 outside Nix, since the flake's `prove` shell is Linux-only and the Mac can't run it. Toolchain:
- Aeneas/Charon `nightly-2026.11.03-ab34bd2` aarch64, with the hash matching `flake.nix`.
- Rust nightly-2026-09-17 with Charon's miri sysroots, built the way the flake builds them.
- Lean v4.35.0 via elan, with Mathlib's cache.

Steps and results:
1. `PROVE_TRANSLATE_ONLY=1 admin/prove lean` passed (quote checks plus all translations).
2. `lake build` reported "Build completed successfully (3612 jobs)".
3. The verification lane's `wide_mux` FunsExternal fix was applied locally, because every PR #33-based branch needs it. It is not part of the branch.
4. The x86-only `SimdVectors.lean` guard step was skipped.

## Tests

Pi 5, native (`test-align.log`): `cargo test -p rind --locked --release` with detection asserted narrow (`RIND_TEST_ARM_WIDE_MUL=0`) and with wide forced (`RIND_CPU_FORCE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=1`). Both modes: 686 lib tests passed, 38 ignored, including `generator_table_is_line_aligned`; wycheproof, cctv, KATs and the other integration tests passed; wipe tests 10 + 15 passed. Release clippy `--all-targets --deny warnings` was clean. Finished at 18:32:18 PST.

M1 Pro, native: 689 lib tests (including the alignment test) plus the same integration and wipe tests, and clippy, passed in both modes (see the PR draft).
