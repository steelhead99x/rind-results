<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Board measurements, 12–14 November 2026

`index.html` is 12 slides at 1280×720. Open it in a browser. Arrow keys or `#N`.

1. Title. Published compares are rind `98568332`. The 14 November XIAO runs are pull comments.
2. Poly1305-32 cycles, ESP32-S3 and nRF52840.
3. ESP32-S3 relink at three alignments.
4. 32 KB I-cache diagnostic.
5. Flash 40→80 MHz, the nRF watchdog, and the ESP32 cycle-counter check.
6. #69 head `ef58672f`. SHA-512 holds. The SHA-256 hold fails. Head `a01b2b9e` is not timed.
7. #96 head `966efe08`. Passes the hold. IRAM +12,596 B (12.3 KiB).
8. #98 head `4e551a93`. Pure Rust, +1,484 B IRAM. Fails the hold on long inputs. The owner's end goal.
9. #95 and #97. Both fail. #97's #96 column is head `28837e94`.
10. #76 on an unnamed host, and #84 host tests. #87 was not re-measured.
11. #23, #32, #39 head `19644395`, and #64.
12. Sources.

## Sources

Published rows: `results/esp32s3/diag/poly1305-32/`, `results/nrf52840/diag/poly1305-32/`,
`results/diag/esp32s3-relink-fixed-98568332/`, `results/diag/esp32s3-icache32-80mhz-98568332/`,
`results/diag/flash-80mhz/`, `results/diag/nrf52840-watchdog-2026-11-12/`, `results/RESULTS.md`,
`results/nrf52840/RESULTS.md`.

14 November rows: steelhead99x/rind comments 6875774312 (#69), 6882029555 (#96),
6882515749 (#98), 6881716592 (#97), 6880930228 and 6881390638 (#95), 6874901551 (#76),
6882545398 (#39). #84, #87, #23, #32, and #64 are the pull bodies.
