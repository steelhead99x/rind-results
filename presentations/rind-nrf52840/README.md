<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind on the nRF52840: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 6 slides at 1280×720.
The board is the Seeed T1000-E (Cortex-M4F, 64 MHz).

1. Title
2. #87: 26-bit Poly1305, and the statement that the hardware ratios were not re-measured on head `6f2866d9`
3. The published compare #87 cites: ChaCha20-Poly1305 seal 16 KB is 0.698 of the HACL* image
4. #84, #69, #96, #98, #76, #82, and #79 are not measured on this board
5. What is still unmeasured
6. Sources

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End) or click the right
or left half of the slide. `#N` in the URL opens slide N.

## Sources

The 0.698 ratio and the "not re-measured" line are steelhead99x/rind#87. The other cycle rows are
`results/nrf52840/diag/poly1305-32/compare.md`, the file that pull cites. They are the 12 November images,
not a re-run of `6f2866d9`.
