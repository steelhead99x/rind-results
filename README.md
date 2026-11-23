<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind-results

Benchmark results for [rind](https://github.com/kixel-ai/rind), a cryptography library written in pure Rust.

Each result runs rind and other Rust crypto libraries (aws-lc-rs, ring, RustCrypto, graviola, libcrux, OpenSSL) on the same machine, doing the same job, so you can see which one is fastest there.

## How to read a result

- Every time is for one operation, such as one signature or one 16 KB encryption. **Lower is faster.**
- A ratio like **1.25×** is a library's time divided by the fastest library's time. **1.00×** means it is the fastest.
- A **✓** means the code is formally verified: a published proof shows it computes the right answer.
- Every result names the **rind commit** it measured. Compare results from the same commit only. rind gets faster over time, so an older result can look slower than rind is today.
- A result marked **noisy** ran while the machine was busy. Don't read small differences from it.

## Latest results

Open the `RESULTS.md` link for each machine's full table.

### Computers and single-board computers

| Machine | CPU | rind commit | Results |
|---|---|---|---|
| Intel Core Ultra 9 275HX laptop | x86_64 | `d6ca9a75` | [RESULTS.md](results/intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RESULTS.md) |
| Jetson Orin Nano | Arm Cortex-A78AE | `98568332` | [RESULTS.md](results/jetson-orin-nano/RESULTS.md) |
| Raspberry Pi 5 (no fan, 1.5 GHz) | Arm Cortex-A76 | `a6577cb4` | [RESULTS.md](results/pi5-nofan-1p5ghz/a6577cb4a4b48af06385f8d748f3892c07cc229e/RESULTS.md) |
| Raspberry Pi 2 | Arm Cortex-A7, 32-bit | `a6577cb4` | [RESULTS.md](results/pi-host-a-bench/a6577cb4a4b48af06385f8d748f3892c07cc229e/RESULTS.md) |
| Xeon cloud VM, Apple M1 Pro, Raspberry Pi 5, Pixel 9 Pro | mixed | `b6651423` | [REPORT-2026-11-12.md](REPORT-2026-11-12.md) |
| x86-host-c laptop (noisy) | x86_64 | `99b509a` | [README.md](results/host/README.md) |

### Microcontrollers

| Board | Chip | rind commit | Results |
|---|---|---|---|
| Seeed XIAO ESP32-S3 | Xtensa LX7, 240 MHz | `98568332` | [RESULTS.md](results/RESULTS.md) |
| Seeed T1000-E | nRF52840, Arm Cortex-M4F, 64 MHz | `98568332` | [RESULTS.md](results/nrf52840/RESULTS.md) |
| Seeed XIAO MG24 | EFR32MG24, Arm Cortex-M33, 78 MHz | `98568332` | [RESULTS.md](results/efr32mg24/RESULTS.md) |
| Arduino Uno R3 | ATmega328P, 16 MHz | `c08c91ea` (partial port, not full rind) | [RESULTS.md](results/uno-r3/RESULTS.md) |

### Experiments

These test one change at a time, such as a new code path, a memory layout, or a CPU setting. Each folder's README says what it changed.

- [results/diag/](results/diag/): microcontroller layout, cache and clock tests.
- [results/argon2/](results/argon2/RESULTS.md), [argon2-interleave/](results/argon2-interleave/RESULTS.md), [argon2-scalar-rider/](results/argon2-scalar-rider/RESULTS.md): Argon2id on Arm.
- [results/pr33-pi5/](results/pr33-pi5/README.md): the narrow-multiplier code on a Raspberry Pi 5.

### Not merged yet

These results are in open pull requests or branches. Read them there until they land:

- Ryzen 7 255 (Minisforum, AVX-512) at `d6ca9a75`: branch [`bench/minisforum-zen5-avx512`](https://github.com/steelhead99x/rind-results/tree/bench/minisforum-zen5-avx512). [#78](https://github.com/steelhead99x/rind-results/pull/78) corrects its label: the CPU is Zen 4, not Zen 5.
- [#77](https://github.com/steelhead99x/rind-results/pull/77) Ryzen 7 255: P-256 timing for rind#79.
- [#84](https://github.com/steelhead99x/rind-results/pull/84) Raspberry Pi 4 (Cortex-A72) baseline.
- [#80](https://github.com/steelhead99x/rind-results/pull/80) X-Wing on five machines.
- [#76](https://github.com/steelhead99x/rind-results/pull/76) Raspberry Pi 5 narrow-multiplier checks.
- [#74](https://github.com/steelhead99x/rind-results/pull/74) ESP32-S3 SHA-512 in IRAM.

## What's in each folder

| Folder | What it holds |
|---|---|
| [`results/`](results/README.md) | Every result, one folder per machine. Start with its `RESULTS.md`. |
| [`hosts/`](hosts/README.md) | Raw data for the four machines in the 2026-11-12 report, and an index of every machine. |
| [`benchmarks/`](benchmarks/) | Charts (SVG) for the 2026-11-12 report. |
| [`media/`](media/README.md) | Photos, screenshots, and serial and OLED captures from the boards. |
| [`presentations/`](presentations/README.md) | Slide decks. |
| [`tools/`](tools/) | `cargo xtask` runs and renders benchmarks. `cargo verify` re-checks every published number against the raw runs. |
| `baremetal/`, `notes/`, `skills/`, `scripts/` | Notes and helpers for the microcontroller work. |
| `upstream/` | A copy of rind's own benchmark data, for comparison. |

## Add a new result

1. On the machine, run `cargo xtask bench`. It writes `results/<machine>/<rind-commit>/` with three runs.
2. Check it: `cargo verify --dir results/<machine>/<rind-commit>`.
3. Run `cargo xtask ci`, then open a pull request.

[results/README.md](results/README.md) explains the folder layout and what each file holds.

## Rules

- **Never change a published number.** If a run was wrong, publish a new run and explain why in its `NOTES.md`.
- Every number must come from raw runs that are in this repository, so `cargo verify` can recompute it.
- `cargo xtask ci` must pass before you push. See [CONTRIBUTING.md](CONTRIBUTING.md).
