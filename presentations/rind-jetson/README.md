<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind on the Jetson Orin Nano: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 8 slides at 1280×720.
It covers the NVIDIA Jetson Orin Nano Engineering Reference Developer Kit Super (6× Cortex-A78AE, Tegra).

1. Title
2. The board: part 0xd42, schedutil 729.6–1728 MHz, the three pinned runs
3. Published baseline at rind `98568332`: fastest on 65 of 68 shared operations, and the rows #82 would have touched
4. #82 does not select narrow schedules on Cortex-A78AE. The Pi 5 table is labelled as a Pi 5 result. This board is not measured yet
5. #79's 64-byte P-256 table applies here and is not measured. #76 does not change the aarch64 assembly path
6. ESP32 IRAM, Poly1305, and bare metal are other machines
7. What is still unmeasured on this board
8. Sources

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End) or click the right
or left half of the slide. `#N` in the URL opens slide N.

## Sources

Published times are `results/jetson-orin-nano/RESULTS.md`, `NOTES.md`, and `facts.txt` (rind `98568332`).
14 November statements are the bodies and comments of steelhead99x/rind#82, #79, #76, #69, #96, #98, #87, and #84.
A pull with no Jetson number is labelled not measured yet.
