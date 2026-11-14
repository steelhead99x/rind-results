<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Result trees

A new publish is one directory:

```text
results/<host>/<upstream-sha>/
    <host>.json    schema 1, median of the runs
    meta.json      provenance (upstream sha, patches, runs)
    NOTES.md
    run.log
    raw/           one file per run
    RESULTS.md
```

`upstream-sha` is the upstream rind commit that was measured, 7 to 40 lower-case hex digits (the full sha, or the prefix recorded for that set). An A/B or a diagnostic is the same directory with a suffix:

```text
results/<host>/<upstream-sha>-<label>/
```

`cargo xtask bench` writes that directory. `cargo verify --dir` checks it before it is published. Nothing here moves a number that was already published.

Published in this layout:

- [`pi2b-bench` at `a6577cb4`](pi2b-bench/a6577cb4a4b48af06385f8d748f3892c07cc229e/RESULTS.md) (Raspberry Pi 2 Model B, armv7, five rounds).
- [`pi5-nofan-1p5ghz` at `a6577cb4`](pi5-nofan-1p5ghz/a6577cb4a4b48af06385f8d748f3892c07cc229e/RESULTS.md) (Raspberry Pi 5 Model B Rev 1.0, BCM2712 Cortex-A76, no fan, held at 1.5 GHz, aarch64, five rounds).
- [`intel-core-ultra-9-275hx` at `d6ca9a75`](intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RESULTS.md) (Intel Core Ultra 9 275HX laptop, x86_64, three runs). The laptop was on battery (power-saving governor) for run 1 and part of run 2, then on AC. See [NOTES.md](intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/NOTES.md).

## Historical trees

These stay in the layout they were published in. They are not the new per-commit directories, and they are not one comparable set.

| Tree | What it is |
|---|---|
| [`hosts/`](../hosts/README.md) | The four `admin/bench-report` hosts at rind `b6651423` (`xeon-vm`, `m1pro`, `pi5`, `pixel9pro`). |
| [`RESULTS.md`](RESULTS.md) and [`raw/`](raw/) | The published ESP32-S3 set, flash at 80 MHz, rind `98568332`. |
| [`diag/esp32s3-40mhz-2026-11-11/`](diag/esp32s3-40mhz-2026-11-11/README.md) | The first ESP32-S3 set. Its flash ran at 40 MHz, so its times are not comparable with the 80 MHz set. |
| [`nrf52840/`](nrf52840/RESULTS.md) | nRF52840 at rind `98568332`. |
| [`jetson-orin-nano/`](jetson-orin-nano/RESULTS.md) | Jetson Orin Nano at rind `98568332`. A commit-suffixed directory such as `results/jetson-orin-nano-<sha>` is this older shape, not `results/jetson-orin-nano/<sha>/`. |
| [`host/`](host/README.md) | msi-001 at rind `99b509a`. The machine was noisy (bench-report `noisy: true`); do not read a gap off it without a quiet rerun. |
| [`diag/`](diag/) | Diagnostics and relinks. Each directory says what it varied. |

`cargo verify` with no arguments still checks the published ESP32-S3 set, the nRF52840 set, msi-001's summary, and `results/jetson-orin-nano/`. `cargo verify --dir <tree>` checks any other directory, including one of these.

## Comparable set

Compare hosts at upstream [kixel-ai/rind](https://github.com/kixel-ai/rind) `d6ca9a75` (`d6ca9a756d3d1e71489e61e44c04f33c3c893589`, [OPRF #705](https://github.com/kixel-ai/rind/commit/d6ca9a756d3d1e71489e61e44c04f33c3c893589)). The host set at this pin is [`intel-core-ultra-9-275hx`](intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RESULTS.md). The laptop was on battery (power-saving governor) for run 1 and part of run 2, then on AC. In a result, `commit` is that upstream SHA. A patched checkout also records `fork_commit` and `patches` as `{name, sha256}` objects. An optional set is named `patches/<set>/0001-....patch` (the ring vendor patch is included the same way). `images.*.rind` on those documents is the fork commit, not the pin.

`0b1f1469` (`0b1f1469c8e17e361e3353432bc7fceee9e04214`) was the previous comparable set. Its `results/<host>/0b1f1469c8e17e361e3353432bc7fceee9e04214/` directories are history. Do not delete them, overwrite them, or mix their numbers into `d6ca9a75` tables. A host that has both sets is tabulated from `d6ca9a75` only; the older set is listed as history.

Schema 1 may also carry `upstream_commit`, `hardware_commit`, `harness_commit`, `firmware_sha256`, `runs`, `warmup`, `iterations`, `metric_units`, `cycles`, `cycles_per_byte`, `run_spread_pct`, `runs_ns`, and `spread_is`. `spread_is` is `"runs"` when `ns_low` and `ns_high` are the minimum and maximum of the run estimates and `ns` is their median.

These are not that set:

- msi-001 at rind `99b509a` ([`host/`](host/README.md)) was noisy (`noisy: true`). Do not read a gap off it without a quiet rerun.
- The ESP32-S3 set at 40 MHz ([`diag/esp32s3-40mhz-2026-11-11/`](diag/esp32s3-40mhz-2026-11-11/README.md)) is not comparable with the 80 MHz set. Cache-missing rows slow down with the flash clock.
