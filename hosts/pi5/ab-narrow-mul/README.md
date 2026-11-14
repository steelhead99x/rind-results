# Pi 5: narrow-multiplier schedules, A/B (2026-11-12)

Branch-level A/B runs on the Raspberry Pi 5 (`pi5`: BCM2712, 4x Cortex-A76 rated 2.4 GHz but capped at 1.5 GHz, see below; 8 GB, Debian 13,
Linux 6.18.60+rpt-rpi-2712, rustc 1.98.1 from rustup), for the stack on top of
[`cursor/aarch64-narrow-mul-132a`](https://github.com/steelhead99x/rind/pull/33) @ 86de4410:

| branch (steelhead99x/rind) | what |
|---|---|
| `cursor/aarch64-narrow-mul-132a` (PR #33) | narrow X25519 / Ed25519 / P-curve scalar-mul / RSA-2048 schedules on A72/A76/N1 |
| `perf/pi5-narrow-nist` | + the ten P-256/P-384/P-521 field and point routines proven by equivalence (ECDSA verify) |
| `perf/pi5-cpu-detect` | lazy, MIDR-first detection; unreadable cpuinfo keeps `_alt` |
| `perf/pi5-rsa-8n` | s2n-bignum generic product + 8n reduction at 24/48/64 words (RSA-3072, RSA-4096 verify) |
| `perf/pi5-mont-uninit` | Karatsuba scratch left uninitialised |
| `perf/pi5-p384-verify-w7` | P-384 ECDSA verify: generator terms from a width-7 NAF over G..63G (on `perf/pi5-narrow-nist`) |
| `perf/pi5-p521-verify-w7` | the same for P-521 (on `perf/pi5-p384-verify-w7`) |
| `perf/pi5-rsa-bench-sizes` | `rsa_sizes` bench: RSA-2048/3072/4096 sign + verify, rind vs aws-lc-rs |
| `fix/p256-table-align` | P-256 generator table aligned to 64 bytes (on `perf/pi5-narrow-nist`) |

## Clock: 1.5 GHz cap

The cap comes from `pw-thermal.service` (no fan; see `../NOTES.md`). No 2.4 GHz data exists: lifting the cap needs `sudo`, which asks for a password on this board, and the cap was left in place on purpose.

`/sys/devices/system/cpu/cpu*/cpufreq/scaling_max_freq` read 1500000 on 2026-11-12 (`cpuinfo_max_freq` 2400000,
governor `ondemand`). The file's mtime is the boot time (2026-11-08 12:21), and `stats/time_in_state` shows the
cores at 1.5 GHz for all but a few seconds since boot, so **every run here was at 1.5 GHz, not 2.4 GHz**.
`vcgencmd get_throttled` = 0xe0000 (frequency capping, throttling and soft temperature limit have occurred since
boot; none active at the time of reading). Each A/B compares variants at the same clock, so the ratios hold. The
absolute µs are 1.5 GHz figures. The "wide" rows reproduce the published `hosts/pi5/pi5.json` within 0.1%, which
suggests that report ran under the same cap. That isn't verified here.

## Method

- Bench binaries built natively on the Pi (`cargo bench --no-run`, `RUSTFLAGS=--cfg rind_cpu_disable` so the
  `RIND_CPU_FORCE_wide_mul` / `RIND_CPU_DISABLE_wide_mul` toggles work in release), copied out, then run with
  criterion `--warm-up-time 1 --measurement-time 3` (the report's `CRITERION_ARGS`), `taskset -c 3`.
- Every round runs every variant; the variant order rotates each round (`scripts/lane-ab.sh`). A shared lock
  (`~/rind-work/.bench.lock`) is held per round so no other agent's timed run or build overlaps; load is logged
  per variant (`run-*.log`). Load was ~1.0 before each round: another agent's single-threaded job on core 1.
- "wide" = the PR #33 binary with `RIND_CPU_FORCE_wide_mul=1`, i.e. the `_alt` code main runs. It reproduces the
  published `hosts/pi5/pi5.json` main numbers within 0.1% (x25519 252.7 vs 252.9 µs, p256-ecdh 326.5 vs 326.6 µs).
- Tables: median of the rounds, spread = max/min - 1 (`cargo xtask ab-cmp`, formerly `scripts/cmp.py`; `cargo xtask ab-min` is the min-of-rounds table). JSON: one row per id, variant and round
  with criterion's slope (or mean) and its 95% interval (`cargo xtask ab-collect`, formerly `scripts/collect.py`, schema `rind-results/ab-v1`).

## Results

### `ab-narrow-mul.*`: PR #33 and the NIST series (5 rounds)

| operation | wide (main) µs | #33 narrow µs | + NIST µs | #33 / wide | NIST / wide | aws-lc-rs µs | wide / aws | #33 / aws | NIST / aws |
|---|---|---|---|---|---|---|---|---|---|
| x25519-ecdh | 252.7 | 107.6 | – | 0.426 | – | 108.8 | 2.32 | 0.99 | – |
| ed25519-sign | 51.5 | 34.6 | – | 0.672 | – | 35.4 | 1.45 | 0.98 | – |
| ed25519-verify | 263.0 | 169.7 | – | 0.645 | – | 182.4 | 1.44 | 0.93 | – |
| p256-ecdh | 326.5 | 200.0 | 200.0 | 0.612 | 0.613 | 213.6 | 1.53 | 0.94 | 0.94 |
| p256-ecdsa-sign | 60.7 | 45.1 | 45.2 | 0.742 | 0.745 | 75.9 | 0.80 | 0.59 | 0.60 |
| p256-ecdsa-verify | 303.2 | 273.8 | 192.5 | 0.903 | 0.635 | 256.6 | 1.18 | 1.07 | 0.75 |
| p384-ecdh | 1115.8 | 661.3 | 650.2 | 0.593 | 0.583 | 699.1 | 1.60 | 0.95 | 0.93 |
| p384-ecdsa-sign | 248.2 | 175.0 | 164.7 | 0.705 | 0.663 | 350.0 | 0.71 | 0.50 | 0.47 |
| p384-ecdsa-verify | 1021.4 | 896.6 | 609.6 | 0.878 | 0.597 | 791.6 | 1.29 | 1.13 | 0.77 |
| p521-ecdh | 2386.2 | 1447.6 | 1447.5 | 0.607 | 0.607 | 1472.6 | 1.62 | 0.98 | 0.98 |
| p521-ecdsa-sign | 522.2 | 365.9 | 367.2 | 0.701 | 0.703 | 850.3 | 0.61 | 0.43 | 0.43 |
| p521-ecdsa-verify | 2115.2 | 1381.4 | 1324.8 | 0.653 | 0.626 | 1740.3 | 1.22 | 0.79 | 0.76 |
| rsa2048-pkcs1-sha256-sign | 5208.5 | 3069.9 | – | 0.589 | – | 3132.9 | 1.66 | 0.98 | – |
| rsa2048-pkcs1-sha256-verify | 151.7 | 86.0 | – | 0.567 | – | 90.5 | 1.68 | 0.95 | – |
| rsa2048-pss-sha256-sign | 5210.1 | 3072.5 | – | 0.590 | – | 3135.6 | 1.66 | 0.98 | – |
| rsa2048-pss-sha256-verify | 153.3 | 87.6 | – | 0.571 | – | 92.1 | 1.67 | 0.95 | – |

All spreads ≤ 0.4%. Every public-key gap to aws-lc-rs on this host (up to 2.32x) closes with #33; with the NIST
series ECDSA verify goes from trailing (1.07-1.22x) to leading (0.75-0.77x). The NIST binary was the prototype
c044b3ec; its `rind/src` equals `perf/pi5-narrow-nist` except one test-only line.

### `ab-rsa-8n.*`: RSA sizes, series 3 and 4 (5 rounds)

| operation | wide (main) µs | #33 µs | + rsa-8n µs | + mont-uninit µs | rsa-8n / #33 | uninit / #33 | aws-lc-rs µs | #33 / aws | rsa-8n / aws |
|---|---|---|---|---|---|---|---|---|---|
| rsa2048 sign | 5211.6 | 3069.9 | 3070.4 | 3039.0 | 1.000 | 0.990 | 3135.2 | 0.98 | 0.98 |
| rsa2048 verify | 138.1 | 72.2 | 72.4 | 72.1 | 1.002 | 0.998 | 90.8 | 0.80 | 0.80 |
| rsa3072 sign | 17084.4 | 17084.3 | 12868.7 | 17088.4 | 0.753 | 1.000 | 13431.0 | 1.27 | 0.96 |
| rsa3072 verify | 307.1 | 307.1 | 219.9 | 307.2 | 0.716 | 1.000 | 253.3 | 1.21 | 0.87 |
| rsa4096 sign | 39356.4 | 20624.8 | 20487.9 | 20586.7 | 0.993 | 0.998 | 21698.5 | 0.95 | 0.94 |
| rsa4096 verify | 543.2 | 543.2 | 385.5 | 543.3 | 0.710 | 1.000 | 439.7 | 1.24 | 0.88 |

(`rsa_sizes` bench, PKCS#1 v1.5 SHA-256; verify uses a pre-parsed key, so its 2048 row differs from the report's
`rsa2048-pkcs1-sha256-verify`, which is also in the JSON.) Spreads ≤ 1.2%. The rsa-8n binary was the drafted
commit (every multiple of 8 up to 128); the pushed branch restricts dispatch to 24/48/64 words, the widths these
rows exercise, so the rows apply unchanged. mont-uninit was measured on its own on 86de4410.

### `ab-cpu-detect.*`: series 2 (lazy MIDR-first detection) overhead (3 rounds)

| operation | #33 (narrow) µs | + series 2 µs | ratio | aws-lc-rs µs |
|---|---|---|---|---|
| p256-ecdh | 200.0 | 201.1 | 1.005 | 213.8 |
| p256-ecdsa-sign | 45.1 | 46.2 | 1.025 | 76.0 |
| p256-ecdsa-verify | 273.9 | 273.7 | 1.000 | 256.9 |
| rsa2048-pkcs1-sha256-verify | 86.0 | 86.0 | 1.000 | 90.6 |
| x25519-ecdh | 107.6 | 107.6 | 1.000 | 108.8 |

Spreads ≤ 0.1%. Round 1 started at load ~3.3; rounds 2-3 at 0.7-1.3 agree with it. The cached check is one relaxed
load, and x25519, RSA and P-256 verify are flat. Still, P-256 sign and ECDH are about 1.1 µs slower in every round.
**Resolved:** the delta came from the P-256 generator table's cache-line offset, which series 2 only moved. `perf/pi5-cpu-detect` now aligns the table to 64 bytes (3d7dd094), which removes the delta. See [`series2-layout/`](series2-layout/README.md). The branch is still kept off the RSA and uninit perf stack.

### `ab-p384-w7.*`: P-384 verify, width-7 generator NAF (5 rounds)

| operation | narrow-nist µs | + w7 µs | ratio | aws-lc-rs µs |
|---|---|---|---|---|
| p384-ecdsa-verify | 609.7 | 582.1 | 0.955 | 792.0 |
| p384-ecdsa-sign | 164.8 | 164.9 | 1.001 | 350.2 |
| p384-ecdh | 650.4 | 650.6 | 1.000 | 699.3 |
| p256-ecdsa-verify (control) | 192.6 | 192.6 | 1.000 | 256.6 |
| p521-ecdsa-verify (control) | 1324.8 | 1324.9 | 1.000 | 1740.8 |

Spreads ≤ 0.1%.

### `ab-p521-w7.*`: P-521 verify, width-7 generator NAF (7 rounds)

| operation | p384-w7 µs | + p521 w7 µs | ratio | aws-lc-rs µs |
|---|---|---|---|---|
| p521-ecdsa-verify | 1325.0 | 1267.3 | 0.956 | 1740.8 |
| p521-ecdsa-sign | 366.2 | 365.9 | 0.999 | 850.1 |
| p521-ecdh | 1446.4 | 1447.1 | 1.000 | 1472.6 |
| p384-ecdsa-verify (control) | 582.0 | 582.1 | 1.000 | 792.1 |

Spreads ≤ 0.4%. The scout's llvm-mca estimate for P-521 was −4.0%; the measured result is −4.4%.


### `table-align/`: P-256 generator table alignment, standalone (5 rounds)

`fix/p256-table-align` (8e6f1670) on `perf/pi5-narrow-nist` (529a9d64). See [`table-align/`](table-align/README.md).

| operation | narrow-nist µs | + align µs | ratio | aws-lc-rs µs |
|---|---|---|---|---|
| p256-keygen | 36.5 | 36.5 | 1.000 | 43.1 |
| p256-ecdsa-sign | 45.2 | 45.2 | 1.000 | 75.9 |
| p256-ecdh | 200.0 | 199.9 | 1.000 | 213.6 |
| p256-ecdsa-verify | 192.6 | 192.6 | 1.000 | 256.7 |

Spreads 0.0%. Round 1 started at 77.9 °C after a cool-down; readings 74.1-79.6 °C, `get_throttled=0xe0000`, clock 1.5 GHz. The base's table already sat at +32, so the result is neutral. The +8 case and the same fix's recovery are in [`series2-layout/`](series2-layout/README.md).

### `../../m1pro/ab-narrow-mul/`: Apple M1 Pro (noisy)

The M1 Pro was shared with the owner's desktop (60-80 Chrome processes) and other agents' builds: load 4-214, spreads
up to 330%. Meaningful: forcing narrow on the M1 is 1.17-1.85x slower on every public-key row (round 1, load 4-9),
so Apple must keep `_alt`; and the stack tip's default (wide) path is within 0.96-1.03x of main by min-of-5 on every
row (`ab-mac-stack.*`). The P-384 w7 run gave p384 verify 0.954x by median and 0.959x by min-of-7, with the other rows at 0.98-1.01x (`ab-mac-w7.*`, load 8-50). The P-521 w7 run gave p521 verify 0.968x by min-of-7, with the other rows at 0.99-1.03x by min (`ab-mac-p521.*`, load 8-36, spreads up to 153%). Medians there are not reliable.

## Tests (native)

See each branch's PR draft. Pi 5: `cargo test -p rind --locked --release` with detection (narrow) and forced wide,
`--features wipe --test zeroing --test heap_scratch`, and `cargo clippy -p rind --all-targets --release -- --deny warnings`,
all with `--cfg rind_cpu_disable`. M1 Pro: the same with default (wide) and narrow forced.
