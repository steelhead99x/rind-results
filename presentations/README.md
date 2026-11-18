<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Presentations

One tree on main, collected from the `docs/*-deck` branches. Most decks
are self-contained HTML; open `index.html` in a browser.
[rind-benchmarks](rind-benchmarks/) is a PowerPoint of the published host
and board results.

| Deck | What it covers | Source branch (newest files used) |
|---|---|---|
| [rind-benchmarks](rind-benchmarks/) | Published host and board results: how to read the speed chart, the Apple DIT tradeoff and `Dit::hold()`, later corrections, and the ESP32-S3 and nRF52840. Updated 17 Nov: SHA-512 and Intel's timing list (kixel-ai/rind#821), and the sets published on main since | `docs/benchmarks-deck` |
| [rind-hardware-today](rind-hardware-today/) | 12 November board improvements: Poly1305-32, layout and cache, flash and watchdog, RP2040, the move to rind-hardware, and a timeline; plus an unpublished late-evening ESP32-S3 run that closed the SHA-512, HMAC/HKDF, ChaCha/XChaCha and SHA-256-spread I-cache gaps. Updated 17 Nov: the fix that merged on the fork and its board run, the Xtensa CI lint, and the Pi 5 lane now on main | `docs/esp32s3-gaps-deck` |
| [rind-rp2040](rind-rp2040/) | RP2040 port, SysTick fix, SHA-512/GHASH findings, no_std split | `docs/rp2040-deck-r3` |
| [rind-systick](rind-systick/) | SysTick and timing across the benched hardware, including the 12 November harness fixes | `docs/systick-deck` |
| [rind-aarch64-perf](rind-aarch64-perf/) | rind aarch64 work, phase by phase. Updated 17 Nov: every source it uses is now on main, and a new slide has the ML-DSA signing A/B on the Jetson Orin Nano | `docs/perf-phases-deck` |
| [rind-spake2plus](rind-spake2plus/) | SPAKE2+ in rind (RFC 9383, Matter PASE) | `docs/spake2plus-deck` |
| [rind-host-speed](rind-host-speed/) | Four speed changes in review: ML-DSA signing (x86_64 and Orin Nano), P-256 on x86_64, the portable big-number multiply, and generator-table alignment | `decks/refresh-2026-11-17` |
| [rind-proofs](rind-proofs/) | Proofs: P-256 on x86_64 moves to a routine proven on every OS, the Lean fix (3,705 jobs), table-alignment proofs, and the SHA-512 and ESP32 SHA-256 trade-offs | `decks/refresh-2026-11-17` |
| [rind-testing](rind-testing/) | Tests and checks: wasm and Ed25519 CCTV verdicts, adapter mutants, hash-to-curve and HPKE edge cases, constant-time checks, CI alerts, the fork sync, and the Xeon VM run | `decks/refresh-2026-11-17` |

Each deck's README lists its sources and anything left out for lack of data. Figures that live only in a
PR body or PR comment are labelled that way on the slide.

Older copies of the same decks on `docs/rp2040-deck`, `docs/rp2040-deck-r2`,
`docs/rp2040-deck-r3`, `docs/systick-deck`, `docs/perf-phases-deck` and
`docs/spake2plus-deck` are superseded by this tree for anything already on
main. Those branches also carry an older xtask serial-port change that main
already has, so they were not merged as branches.
