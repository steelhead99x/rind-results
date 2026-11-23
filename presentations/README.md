<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Presentations

One tree on main, collected from the `docs/*-deck` branches. Most decks
are self-contained HTML; open `index.html` in a browser.
[rind-benchmarks](rind-benchmarks/) is a PowerPoint of the published host
and board results.

| Deck | What it covers | Source branch (newest files used) |
|---|---|---|
| [rind-benchmarks](rind-benchmarks/) | Published host and board results: how to read the speed chart, the Apple DIT tradeoff and `Dit::hold()`, later corrections, and the ESP32-S3 and nRF52840. Updated 19 Nov (rind-results #85): work-so-far and scale slides. For SHA-512 and Intel's timing list (kixel-ai/rind#821), see rind-speed-vs-guarantees | `docs/benchmarks-deck` |
| [rind-hardware-today](rind-hardware-today/) | 12 November board improvements: Poly1305-32, layout and cache, flash and watchdog, RP2040, the move to rind-hardware, and a timeline; plus an unpublished late-evening ESP32-S3 run that closed the SHA-512, HMAC/HKDF, ChaCha/XChaCha and SHA-256-spread I-cache gaps. Updated 17 Nov: the fix that merged on the fork and its board run, the Xtensa CI lint, and the Pi 5 lane now on main | `docs/esp32s3-gaps-deck` |
| [rind-rp2040](rind-rp2040/) | RP2040 port, SysTick fix, SHA-512/GHASH findings, no_std split | `docs/rp2040-deck-r3` |
| [rind-systick](rind-systick/) | SysTick and timing across the benched hardware, including the 12 November harness fixes | `docs/systick-deck` |
| [rind-aarch64-perf](rind-aarch64-perf/) | rind aarch64 work, phase by phase. Updated 17 Nov: every source it uses is now on main, and a new slide has the ML-DSA signing A/B on the Jetson Orin Nano | `docs/perf-phases-deck` |
| [rind-spake2plus](rind-spake2plus/) | SPAKE2+ in rind (RFC 9383, Matter PASE). Updated 19 Nov: kixel-ai/rind#689 merged 13 Nov, and the four fixes split out of it (#722 to #725) | `docs/spake2plus-deck` |
| [rind-host-speed](rind-host-speed/) | Four speed changes and where they stand (status refreshed 19 Nov): ML-DSA signing (x86_64 and Orin Nano), P-256 on x86_64, the portable big-number multiply, and generator-table alignment | `decks/refresh-2026-11-17` |
| [rind-proofs](rind-proofs/) | Proofs (status refreshed 19 Nov: #821 and #793 merged, #790 closed): P-256 on x86_64 moves to a routine proven on every OS, the Lean fix (3,705 jobs), table-alignment proofs, and the SHA-512 and ESP32 SHA-256 trade-offs | `decks/refresh-2026-11-17` |
| [rind-testing](rind-testing/) | Tests that catch more: seven test and CI PRs merged upstream 14 to 19 Nov (#797, #792, #791, #798, #796, #876, #725), with numbers refreshed from the merged bodies, plus constant-time checks, the fork sync and the Xeon VM run | `decks/refresh-2026-11-17` |
| [rind-esp32-sha2-cache](rind-esp32-sha2-cache/) | kixel-ai/rind#793, merged 19 Nov: why SHA-512 was 33× slower than ring on the ESP32-S3, the rolled loops and IRAM SHA-256, the i686 check, and the IRAM, heap and proof costs. Board numbers are fork runs and labelled so | `decks/refresh-2026-11-17` |
| [rind-speed-vs-guarantees](rind-speed-vs-guarantees/) | kixel-ai/rind#821 and #793, merged 19 Nov: SHA-512 on Intel gives up speed for a timing guarantee and a proof, small chips give up a proof for speed; the 275HX "after" is pending a rerun | `decks/refresh-2026-11-17` |
| [rind-firetv](rind-firetv/) | rind on an Amazon Fire TV Stick (AFTSSS, 4x Cortex-A53, 32-bit Android 9), run 19 Nov: rind faster in 88 of 118 comparisons, the ChaCha20-Poly1305 and P-256 gaps and their measured causes, the same-day repeat, and the fork's status (#207, #217, #215, #222 merged). Raw data in [`results/firetv-aftsss/`](../results/firetv-aftsss/README.md) | `docs/firetv-deck` |

Each deck's README lists its sources and anything left out for lack of data. Slides tied to a PR carry a
muted `PRs:` footer line: `rind #N` is kixel-ai/rind, `fork #N` is steelhead99x/rind, and `rind-results #N` is
this repository. Figures that live only in a
PR body or PR comment are labelled that way on the slide.

Older copies of the same decks on `docs/rp2040-deck`, `docs/rp2040-deck-r2`,
`docs/rp2040-deck-r3`, `docs/systick-deck`, `docs/perf-phases-deck` and
`docs/spake2plus-deck` are superseded by this tree for anything already on
main. Those branches also carry an older xtask serial-port change that main
already has, so they were not merged as branches.
