<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Hosts and boards

Index of every machine this repository publishes numbers for. The four
`admin/bench-report` hosts live under `hosts/<name>/`. Later board and SBC
sets use rind-results' own `cargo xtask` / rind-bench host harness and live
under `results/`. New publishes use
[`results/<host>/<upstream-sha>/`](../results/README.md). The trees named
below stay where they are; they are the historical sets.

The on-board commands are `cargo xtask`, not the old shell wrappers:

| Was | Command |
|---|---|
| `orin-setup.sh`, `orin-env.sh` | `cargo xtask orin-setup` |
| `orin-run.sh`, `orin-faketime.sh` | `cargo xtask orin-run <n> [core]` (`FAKETIME_OFFSET` still selects libfaketime) |
| `orin-runs.sh` | `cargo xtask orin-runs` |
| `pixel-run.sh` | `cargo xtask pixel-run --serial <id>` |
| `pixel-run2.sh` | `cargo xtask pixel-run2 --serial <id>` |
| `mac-run.sh` | `cargo xtask mac-run` (default commit is the comparable pin `d6ca9a756d3d1e71489e61e44c04f33c3c893589`; the published `b6651423` numbers are `--commit b66514238d449fc211d75fd14554301bb690e421`) |
| `pi-run.sh` | `cargo xtask pi-run` (same commit default as `mac-run`) |

Each one calls rind's `admin/bench-report` and does not edit that file.

## `admin/bench-report` hosts (rind `b6651423`)

Raw JSON, a trimmed log, and notes are in each folder. The top-level
[README.md](../README.md) tables are rendered from these JSON files.

| Host | Hardware | Results |
|---|---|---|
| [xeon-vm](xeon-vm/NOTES.md) | Intel Xeon, x86_64 VM | [xeon-vm.json](xeon-vm/xeon-vm.json) |
| [m1pro](m1pro/NOTES.md) | Apple M1 Pro | [m1pro.json](m1pro/m1pro.json) |
| [pi5](pi5/NOTES.md) | Raspberry Pi 5 Model B, 1.5 GHz (no active cooling) | [pi5.json](pi5/pi5.json) |
| [pixel9pro](pixel9pro/NOTES.md) | Google Tensor G4 (Pixel 9 Pro) | [pixel9pro.json](pixel9pro/pixel9pro.json) |
| [rp2040](rp2040/NOTES.md) | Raspberry Pi Pico (Cortex-M0+, 125 MHz) | Firmware images and notes only; see below |

## Board and SBC sets (rind `98568332`, unless a diag folder says otherwise)

| Board or host | Hardware | Where the numbers are |
|---|---|---|
| ESP32-S3 (Seeed XIAO) | Xtensa LX7, 240 MHz, flash 80 MHz | [results/RESULTS.md](../results/RESULTS.md) |
| nRF52840 (Seeed T1000-E) | Cortex-M4F, 64 MHz, bare metal | [results/nrf52840/RESULTS.md](../results/nrf52840/RESULTS.md) |
| Jetson Orin Nano | Cortex-A78AE, up to 1.73 GHz, Ubuntu 24.04, MAXN_SUPER | [results/jetson-orin-nano/RESULTS.md](../results/jetson-orin-nano/RESULTS.md) |
| RP2040 (Raspberry Pi Pico) | Cortex-M0+, 125 MHz | Known-answer checks passed on a Pico (2026-11-12). No `results/rp2040/` tree is on main yet, so this repo does not publish timing numbers for it. |


## Later Linux host (rind `a6577cb4`)

New publishes use `results/<host>/<upstream-sha>/`. This is the first Linux host in that layout.

| Board or host | Hardware | Where the numbers are |
|---|---|---|
| [pi2b-bench](../results/pi2b-bench/a6577cb4a4b48af06385f8d748f3892c07cc229e/RESULTS.md) | Raspberry Pi 2 Model B Rev 1.1, BCM2836 Cortex-A7, 900 MHz, armv7 | [RESULTS.md](../results/pi2b-bench/a6577cb4a4b48af06385f8d748f3892c07cc229e/RESULTS.md) at rind `a6577cb4` |

## Comparable-set host (rind `d6ca9a75`)

New publishes use `results/<host>/<upstream-sha>/`. A host id is any filesystem-safe name; the recommended one is a slug of the CPU model.

| Board or host | Hardware | Where the numbers are |
|---|---|---|
| [intel-core-ultra-9-275hx](../results/intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RESULTS.md) | Intel Core Ultra 9 275HX laptop, x86_64, Oracle Linux Server 10.2 | [RESULTS.md](../results/intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RESULTS.md) at rind `d6ca9a75` |

The laptop was on battery (power-saving governor) for run 1 and part of run 2, then on AC. bench-report's noisy flag is false on every run. See [NOTES.md](../results/intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/NOTES.md).

## Later findings (2026-11-12, America/Los_Angeles)

These sit beside the published sets. None of them changes a gap verdict on the published 16 KB-cache ESP32-S3 tables.

| Finding | What it shows | Path |
|---|---|---|
| Poly1305-32 A/B | Optional 32-bit-limb Poly1305 is faster than rind's previous HACL* code at every size on the ESP32-S3 and the T1000-E (21/21 known-answer checks). Gain is against rind's previous code, not against ring. | [ESP32-S3](../results/esp32s3/diag/poly1305-32/README.md), [T1000-E](../results/nrf52840/diag/poly1305-32/README.md) |
| Three-alignment relinks | Moving code by 1 KB without changing alignment does nothing; a different alignment moves some short-input rows by up to 0.3×. SHA-256 64B is 2.36×–3.07× libcrux (2.79× from the medians). An earlier relink that only moved the harness is labelled as such. | [esp32s3-relink-fixed-98568332](../results/diag/esp32s3-relink-fixed-98568332/README.md) |
| 32 KB icache at 80 MHz | Confirms the 40 MHz diagnostic at the published pin and clock: the XChaCha gap is cache misses (7.50× → 1.10× RustCrypto at 32B). The ChaCha gap stays put (Poly1305 compute). | [esp32s3-icache32-80mhz-98568332](../results/diag/esp32s3-icache32-80mhz-98568332/README.md) |
| Legacy provenance map | Images built from the archived private `rind-bench` history record that SHA in `legacy_rind_bench`, and map it onto public `rind-hardware` (`8d9c1bb`, or `ancestor of 8d9c1bb…`). | [tools/legacy-rind-bench-map.json](../tools/legacy-rind-bench-map.json) |

## Presentations

HTML decks are in [presentations/](../presentations/README.md) on main.

## Branches not on main

Argon2 and later Pi 5 A/B work still live on `results/m1pro-pi5-argon2-2026-11-12`,
`results/argon2-a76-interleave`, and `results/pi5-lane-2026-11-12`. Those
branches are behind main and were left unmerged; the aarch64 deck lists them
as sources.
