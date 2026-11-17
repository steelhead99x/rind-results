<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Presentations

One tree on main, collected from the `docs/*-deck` branches. Most decks
are self-contained HTML; open `index.html` in a browser.
[rind-benchmarks](rind-benchmarks/) is a PowerPoint of the published host
and board results.

| Deck | What it covers | Source branch (newest files used) |
|---|---|---|
| [rind-benchmarks](rind-benchmarks/) | Published host and board results: how to read the speed chart, the Apple DIT tradeoff and `Dit::hold()`, later corrections, and the ESP32-S3 and nRF52840 | `docs/benchmarks-deck` |
| [rind-hardware-today](rind-hardware-today/) | 12 November board improvements: Poly1305-32, layout and cache, flash and watchdog, RP2040, the move to rind-hardware, and a timeline; plus an unpublished late-evening ESP32-S3 run that closed the SHA-512, HMAC/HKDF, ChaCha/XChaCha and SHA-256-spread I-cache gaps | `docs/esp32s3-gaps-deck` |
| [rind-rp2040](rind-rp2040/) | RP2040 port, SysTick fix, SHA-512/GHASH findings, no_std split | `docs/rp2040-deck-r3` |
| [rind-systick](rind-systick/) | SysTick and timing across the benched hardware, including the 12 November harness fixes | `docs/systick-deck` |
| [rind-aarch64-perf](rind-aarch64-perf/) | rind aarch64 work, phase by phase, including ChaCha20 NEON on the M1 Pro. Some slides cite data that still lives only on the `results/*` branches listed in that deck's README | `docs/aarch64-chacha20-neon` |
| [rind-spake2plus](rind-spake2plus/) | SPAKE2+ in rind (RFC 9383, Matter PASE) | `docs/spake2plus-deck` |

Older copies of the same decks on `docs/rp2040-deck`, `docs/rp2040-deck-r2`,
`docs/rp2040-deck-r3`, `docs/systick-deck`, `docs/perf-phases-deck` and
`docs/spake2plus-deck` are superseded by this tree for anything already on
main. Those branches also carry an older xtask serial-port change that main
already has, so they were not merged as branches.
