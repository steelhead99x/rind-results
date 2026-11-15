<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Presentations

Each deck is self-contained HTML at 1280×720. Open `index.html` in a browser.
Arrow keys move between slides. `#N` opens slide N.

| Deck | Slides | What it shows |
|---|---|---|
| [rind-hardware-today](rind-hardware-today/) | 12 | ESP32-S3 and nRF52840 measurements, including the 14 November XIAO runs of #69, #96, #97, and #98 |
| [rind-aarch64-perf](rind-aarch64-perf/) | 8 | Pi 5 and M1 Pro tables: narrow schedules, Argon2id, #82, and #79 |
| [rind-jetson](rind-jetson/) | 3 | Jetson Orin Nano published set at rind `98568332`. #82 and #79 are not measured there |
| [rind-nrf52840](rind-nrf52840/) | 2 | nRF52840 Poly1305-32 compare. Not re-measured on #87 |
| [rind-rp2040](rind-rp2040/) | 3 | What `hosts/rp2040/` records. No published Pico timing |
| [rind-systick](rind-systick/) | 4 | Which counter each measured board used |
| [rind-spake2plus](rind-spake2plus/) | 2 | No SPAKE2+ timing on 14 November. #64's Ed25519 CCTV count |

A number on a slide comes from a file in this repo or from a linked
[steelhead99x/rind](https://github.com/steelhead99x/rind) pull comment.
The 14 November XIAO benches are comments on #96, #98, and #97. They are
not a new `results/` publish. Pico cycle counts and the 12 November evening
I-cache run are not in this repo, so they are not on the slides.
