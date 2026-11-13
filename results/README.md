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
