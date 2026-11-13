# SysTick and timing across the hardware we bench on: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 14 slides at 1280×720.

1. Title
2. The eight targets rind is benched on and what times each one's published runs
3. How SysTick works: the 24-bit down counter, SYST_CSR / RVR / CVR / CALIB, CLKSOURCE, COUNTFLAG
4. Wrap time: 2^bits ÷ clock (134.2 ms at 125 MHz), with a log-scale chart for each counter
5. Extending SysTick to 32 or 64 bits in software, as the RP2040 images do, and the COUNTFLAG wrap-count bug found on the first Pico run (rind-hardware a9d057a)
6. Pitfalls: torn reads, interrupts, flash stalls, clock changes
7. Counters on the microcontrollers (RP2040, EFR32MG24, nRF52840, ESP32-S3)
8. Counters on the application processors (Raspberry Pi 5, Apple M1, Pixel 9 Pro, Xeon VM)
9. A safe read sequence for each counter (sketches)
10. Recommendations per chip
11. Recommendations for every chip (methodology)
12. Values that still have to be read on the boards
13. and 14. Sources

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.

## Updating it

Every sourced value is in the `DATA` block (`<script id="deck-data">`) at the top of `index.html`: the target list,
the SysTick register fields, counter widths and clocks, the recommendations, the open items, and the source list.
Resolution and wrap time are not stored: the render script computes them from each row's `bits` and `mhz`, and the
slides mark them "computed". When a value in "Not yet known" (slide 12) is read on a board, add it to the matching row
in `DATA.app` or `DATA.mcu`, cite where it is recorded in `DATA.sources`, and remove it from `DATA.open`.

The CSS and navigation script are reused from `presentations/rind-rp2040`.

## What is a recommendation

Slides 5 (the extension method), 9 (read sequences), 10 and 11 are recommendations, labelled as such. Everything else
states what a source says. The methodology on slide 11 follows the measurement plan of 2026-11-12
and the harness checks in the published ESP32-S3 and nRF52840 results, plus the RP2040 boot cross-check after the wrap-count fix.

## Sources

- RP2040 datasheet (build 2025-02-20): https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf
- Arm ARMv6-M ARM DDI 0419E, ARMv7-M ARM DDI 0403D, Cortex-M33 TRM (100230), A-profile ARM DDI 0487 (D12, D13)
- Silicon Labs EFR32xG24 Reference Manual rev 1.1 and AN0004.2. The reference manual blocked automated download, so
  its sections (3.1, 3.2, 6.6.6.6) were read from an extracted-text copy.
- ESP-IDF v5.5.1 `esp_cpu.h`
- Hetterich and Schwarz, "Branch Different: Spectre Attacks on Apple Silicon", DIMVA 2022 (M1 counter frequency and PMU access)
- Rust `std::time::Instant` and criterion `WallTime` documentation
- steelhead99x/rind-results: `hosts/rp2040/NOTES.md` (2cb88c3), `baremetal/rp2040/builds/README.md`,
  `baremetal/rp2040/TARGET-SURVEY.md`, `baremetal/mg24/README.md`, `results/RESULTS.md`, `results/nrf52840/RESULTS.md`,
  `tools/xtask/src/report.rs`, `hosts/{pi5,m1pro,pixel9pro,xeon-vm}/NOTES.md` (main), and
  `hosts/pi5/ab-narrow-mul/README.md` (branch `results/pi5-lane-2026-11-12`)
- steelhead99x/rind-hardware `rp2040/systick-wrap-fix` (a9d057a): COUNTFLAG double-count on the first Pico run, the fix, and the 300 ms boot cross-check (37,500,815 cycles against 300,021 µs, 125.0 MHz)
