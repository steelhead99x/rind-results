<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Presentations

One tree on main, collected from the `docs/*-deck` branches. Each deck is
self-contained HTML. Open `index.html` in a browser.

| Deck | What it covers | Source branch (newest files used) |
|---|---|---|
| [rind-hardware-today](rind-hardware-today/) | 12 November board improvements: Poly1305-32, layout and cache, flash and watchdog, RP2040, the move to rind-hardware, and a timeline; plus an unpublished late-evening ESP32-S3 run that closed the SHA-512, HMAC/HKDF, ChaCha/XChaCha and SHA-256-spread I-cache gaps | `docs/esp32s3-gaps-deck` |
| [rind-rp2040](rind-rp2040/) | RP2040 port, SysTick fix, SHA-512/GHASH findings, no_std split | `docs/rp2040-deck-r3` |
| [rind-systick](rind-systick/) | SysTick and timing across the benched hardware, including the 12 November harness fixes | `docs/systick-deck` |
| [rind-aarch64-perf](rind-aarch64-perf/) | rind aarch64 work, phase by phase. Some slides cite data that still lives only on the `results/*` branches listed in that deck's README | `docs/perf-phases-deck` |
| [rind-spake2plus](rind-spake2plus/) | SPAKE2+ in rind (RFC 9383, Matter PASE). A 14 November slide records that SPAKE2+ itself was unchanged | `docs/spake2plus-deck` |
| [rind-jetson](rind-jetson/) | Jetson Orin Nano (aarch64 Tegra): the published rind `98568332` set, and what 14 November's aarch64 pulls do and do not measure on that Cortex-A78AE | new, 14 November 2026 |
| [rind-nrf52840](rind-nrf52840/) | nRF52840 (Seeed T1000-E): the Poly1305-32 compare #87 cites, and what is not measured on that board | new, 14 November 2026 |

Older copies of the same decks on `docs/rp2040-deck`, `docs/rp2040-deck-r2`,
`docs/rp2040-deck-r3`, `docs/systick-deck`, `docs/perf-phases-deck` and
`docs/spake2plus-deck` are superseded by this tree for anything already on
main. Those branches also carry an older xtask serial-port change that main
already has, so they were not merged as branches.

The 14 November 2026 slides in the decks above quote
[steelhead99x/rind](https://github.com/steelhead99x/rind) pull requests #69, #76, #79,
#82, #84, #87, #95, #96, #97, and #98, plus correctness pulls #23, #32, #39, and #64.
A host with no measurement is labelled not measured yet. The Jetson baseline is the
published set in `results/jetson-orin-nano/` at rind `98568332`, not a remeasure of
those pulls.
