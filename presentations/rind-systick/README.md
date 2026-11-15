<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Counters used on the measured boards

`index.html` is 4 slides at 1280×720. Open it in a browser. Arrow keys or `#N`.

1. Title.
2. The counter and clock each published set used. Times are not comparable across boards.
3. The three published harness findings: flash 40→80 MHz, the nRF watchdog, and ESP32-S3 alignment.
4. Which 14 November run used which counter. #96 and #98 were run (3 runs). #97 was 2 runs against #96 at `28837e94`. #69 head `a01b2b9e` is not timed.

## Sources

`results/RESULTS.md`, `results/nrf52840/diag/poly1305-32/README.md`, `hosts/rp2040/NOTES.md`,
`hosts/pi5/ab-narrow-mul/README.md`, `results/jetson-orin-nano/facts.txt`,
`results/diag/flash-80mhz/README.md`, `results/diag/nrf52840-watchdog-2026-11-12/README.md`,
`results/diag/esp32s3-relink-fixed-98568332/README.md`.
14 November: comments 6882029555, 6882515749, 6881716592, 6875774312, 6874901551, and the #82 body.
