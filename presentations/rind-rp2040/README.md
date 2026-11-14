<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# RP2040: images build, no published timing

`index.html` is 3 slides at 1280×720. Open it in a browser. Arrow keys or `#N`.

1. Title. Known-answer checks passed on a Pico on 12 November. There is no `results/rp2040/` tree.
2. What `hosts/rp2040/` records: 264 KB RAM, SysTick wrap 134.2 ms, 1672 errors at rind `a4759c48`, images built from rind `98568332`.
3. #84 head `4b4476e4`: host tests pass. Nothing new was timed on the Pico.

Cycle counts from the 12 November working notes are not in this repo, so they are not on the slides.

## Sources

`hosts/rp2040/NOTES.md`, `hosts/rp2040/FEASIBILITY.md`, `hosts/rp2040/images.json`.
#84 is the pull body.
