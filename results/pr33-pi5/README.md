# PR #33 on a Raspberry Pi 5: narrow-multiplier schedules, measured

[steelhead99x/rind#33](https://github.com/steelhead99x/rind/pull/33) (`cursor/aarch64-narrow-mul-132a`, head
`86de4410`, = fork main `8feee30e` + the PR) selects s2n-bignum's non-`_alt` routines and the Karatsuba
RSA products on Cortex-A72/A76/Neoverse N1. Its results table said "not measured". This is that measurement.

- Host: `pi5` — Raspberry Pi 5 Model B (BCM2712, 4× Cortex-A76 @ 2.4 GHz, 8 GB), Debian 13 (Linux 6.18.60+rpt-rpi-2712),
  reached over SSH from the M1 Pro.
- Toolchain: rustc 1.98.1 (rustup). OpenSSL headers unpacked from `libssl-dev` into the work dir (nothing installed), as `scripts/pi-run.sh` did.
- One set of release bench binaries (`x25519 p256 p384 p521 ed25519 rsa_priv rsa_pub`) built with
  `RUSTFLAGS=--cfg rind_cpu_disable` (`scripts/pi5-pr33-build.sh`). Each run is either **narrow** (default detection,
  which picks the narrow schedules on this A76) or **wide** (`RIND_CPU_FORCE_wide_mul=1`, the `_alt` code fork main runs
  on every aarch64 core). Same binary, so only the schedule differs.
- `admin/bench-report`'s criterion arguments (`--warm-up-time 1 --measurement-time 3`), the report's operation names,
  rind and aws-lc-rs only, `taskset -c 3`, 3 interleaved rounds (narrow, wide, narrow, wide, …), estimate = slope or
  mean as the harness parses it (`scripts/pi5-pr33-ab.sh`, `scripts/crit-parse.py`).
- 2026-11-12 11:39–11:58 PST, under the shared bench lock, after another agent's dudect run on core 1 finished.
  Load average ~1.0 (the benchmark alone) except at the start of round 1 (3.5), which produced the few
  13–25% spreads in round-1 `narrow` rows; every other row's spread is ≤0.4%.

| Operation | rind wide (`_alt`, main) | rind narrow (PR #33) | narrow/wide | aws-lc-rs | rind narrow / aws-lc-rs | rind wide / aws-lc-rs |
|---|---|---|---|---|---|---|
| ed25519-sign | 51.48 µs | 34.55 µs | 0.67 | 35.42 µs | 0.98 | 1.45 |
| ed25519-verify | 263.02 µs | 169.74 µs | 0.65 | 182.37 µs | 0.93 | 1.44 |
| p256-ecdh | 326.62 µs | 200.03 µs | 0.61 | 213.54 µs | 0.94 | 1.53 |
| p256-ecdsa-sign | 60.72 µs | 45.10 µs | 0.74 | 75.91 µs | 0.59 | 0.80 |
| p256-ecdsa-verify | 303.22 µs | 273.88 µs | 0.90 | 256.66 µs | 1.07 | 1.18 |
| p384-ecdh | 1.116 ms | 662.02 µs | 0.59 | 699.32 µs | 0.95 | 1.60 |
| p384-ecdsa-sign | 248.36 µs | 175.02 µs | 0.70 | 350.03 µs | 0.50 | 0.71 |
| p384-ecdsa-verify | 1.021 ms | 896.78 µs | 0.88 | 792.49 µs | 1.13 | 1.29 |
| p521-ecdh | 2.386 ms | 1.448 ms | 0.61 | 1.472 ms | 0.98 | 1.62 |
| p521-ecdsa-sign | 522.08 µs | 366.32 µs | 0.70 | 850.01 µs | 0.43 | 0.61 |
| p521-ecdsa-verify | 2.116 ms | 1.382 ms | 0.65 | 1.741 ms | 0.79 | 1.22 |
| rsa2048-pkcs1-sha256-sign | 5.209 ms | 3.071 ms | 0.59 | 3.134 ms | 0.98 | 1.66 |
| rsa2048-pkcs1-sha256-verify | 151.73 µs | 85.92 µs | 0.57 | 90.58 µs | 0.95 | 1.68 |
| rsa2048-pss-sha256-sign | 5.211 ms | 3.072 ms | 0.59 | 3.136 ms | 0.98 | 1.66 |
| rsa2048-pss-sha256-verify | 153.35 µs | 87.57 µs | 0.57 | 92.15 µs | 0.95 | 1.66 |
| x25519-ecdh | 252.77 µs | 107.61 µs | 0.43 | 108.84 µs | 0.99 | 2.32 |

Medians of 3. `raw.md` has min, max and spread per row; aws-lc-rs was timed in both modes as a control
(narrow and wide medians agree within 0.2%).

## Reading it

- PR #33 closes the Pi 5 gaps that `hosts/pi5/NOTES.md` recorded at b6651423 (X25519 2.32×, RSA-2048 ~1.65×,
  ECDH 1.5–1.6×, Ed25519 ~1.45×): with the narrow schedules rind is at or ahead of aws-lc-rs on 14 of 16 operations.
  X25519 goes from 252.8 µs to 107.6 µs (0.43×), RSA-2048 signing from 5.21 ms to 3.07 ms.
- Still behind aws-lc-rs: **P-256 ECDSA verify 1.07×** and **P-384 ECDSA verify 1.13×** (both improved, by 10% and
  12%). These are the remaining Pi 5 public-key gaps.
- The wide column reproduces the b6651423 report's ratios (X25519 2.32×, RSA ~1.66×), so the earlier diagnosis
  (s2n-bignum `_alt` on a core with a slow `umulh`) is confirmed.
- The other PR #33 checks on this board (tests in both modes, ctgrind, dudect) were run by another agent in
  `~/rind-pr33-latest` on the Pi; their logs are there, not here.
