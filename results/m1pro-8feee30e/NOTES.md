# `m1pro` at `8feee30e` (steelhead99x/rind main), 2026-11-12

- CPU: Apple M1 Pro (8P + 2E, arm64, 10 logical CPUs), 32 GB; macOS 14.0 (23.0.0)
- Toolchain: rustc 1.98.1 (48a229cea 2026-09-01) from rustup, the release the flake pins (not Homebrew's 1.98.0 used for `hosts/m1pro/`)
- Commit: [`8feee30e`](https://github.com/steelhead99x/rind/commit/8feee30ed443e3329e246e85840a7b9dc07f0ab6), fork main, = upstream moq-dev/rind `88ff91fd` merged
- Started / finished: 2026-11-12 09:17 / 09:56 PST (17:17 / 17:56 UTC)
- Run: `scripts/host-run.py --host m1pro --jobs 8` from a `git archive` of the commit, wrapped by `scripts/m1pro-run-8feee30e.sh`; build cost skipped (`HOSTRUN_SKIP_BUILD_COST=1`)
- Busy CPUs (other work), as the harness measured: before build 9.68, before run 8.34, during run 4.76 (harness limit 1.0). **Very noisy.**
- rind-cpuid: aes, dit, neon, pmull, sha2, sha3
- OpenSSL: Homebrew openssl@3
- Operations timed: 68 (now including SHA3-256/512); results: 424

## Notes

- Not a clean run. Another agent was building and testing rind on this Mac for the whole run (cargo/clippy in
  `~/rind-upstream-check`), and this agent compiled the Argon2 tests for ~2 minutes at the start of the
  benchmark phase (before the shared bench lock existed). Libraries are timed one after another, so a burst of
  other work lands on whichever library was being timed: e.g. `p256-ecdsa-verify` (1.80x the earlier run),
  `p384-ecdh` (2.36x) and `mldsa65-verify` (4.13x) are noise, not regressions. Read `compare-b6651423.md`
  with that in mind; use it for large effects only.
- `sha256/16KB` measured 6.86 µs here against 10.94 µs in `hosts/m1pro/` (b6651423, also noisy), level with
  ring's 6.74 µs. rind runs aws-lc's `sha256_block_data_order_hw`, the same routine as ring and aws-lc-rs, so
  the earlier 1.55x gap was noise, not a code gap.
- `quic-header-protection/aes128` and `aes256` stay about 11-13x aws-lc-rs (53 ns vs 4 ns): rind's
  per-call DIT toggle, the decided trade-off `benchmarks/gaps.toml` records for `kixtop`; the DIT-held column
  closes it.
- For clean numbers on this machine, rerun under the bench lock with nothing else running.
