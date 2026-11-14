<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840 Poly1305-32

`index.html` is 2 slides at 1280×720. The board is the Seeed T1000-E (Cortex-M4F, 64 MHz, DWT).
Open it in a browser. Arrow keys or `#N`.

1. Title. #87 head `6f2866d9` was not re-measured.
2. The published compare. ChaCha seal 16 KB is 0.698×. XChaCha 32 B is 0.987×.

#69, #96, #97, #98, #76, and #84 have no nRF timing. The ESP32-S3 table is on the boards deck.

## Sources

`results/nrf52840/diag/poly1305-32/compare.md` and `README.md`.
The not-remeasured line is the body of steelhead99x/rind#87.
