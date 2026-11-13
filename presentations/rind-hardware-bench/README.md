# rind on real hardware: benchmarks and verification (slide deck)

`index.html` is a self-contained HTML deck (no network access, no external libraries) covering every device rind has
been benchmarked on in this round of work, and what has been verified on each. 11 slides:

1. Title
2. The devices: core, clock, memory, what ran
3. Verification matrix: build, tests or KATs, timed medians, public-key ops, whether the data is in rind-results
4. SHA-256 and SHA-512 (16 KB) across devices, in cycles and µs, log-scale charts, with the SHA-512 caveat
5. X25519 and P-256 ECDSA sign across devices
6. Microcontrollers: XIAO MG24, ESP32-S3, RP2040 Pico and Pico W, in cycles
7. Arduino Uno R3: what fits in 2 KB SRAM
8. Application processors: Apple M1 Pro, Raspberry Pi 5, Raspberry Pi 2, in µs
9. Raspberry Pi 5: the narrow-multiplier path (rind PR #82)
10. What is still pending
11. Where the numbers live

Not to be confused with `presentations/rind-hardware-today/` (a separate deck about the 12 November board work).

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click the right or
left half of the slide. `#N` in the URL opens slide N. To print, use Chrome or Edge "Save as PDF", margins "None",
background graphics on; each slide is one 1280×720 page.

## Updating it

Every number is in the `DATA` block (`<script id="deck-data">`) at the top of `index.html`. Tables, charts and the
matrix render from it. Derived values are computed by the render script and marked with a superscript "c" on the
slides:

- µs for microcontrollers = published cycles ÷ clock in MHz (78, 240, 125, 16).
- cycles for the Pi 2 = published ns × 0.9 GHz (900 MHz was recorded before and after every round).
- cycles for the Pi 5 public-key rows = published µs × 1.5 GHz (the A/B runs recorded the 1.5 GHz clock).
- No cycles are computed for the M1 Pro (clock not fixed) or for the Pi 5 SHA rows (that run's clock was not checked).
- narrow / wide ratios on slide 9 = narrow median ÷ wide median, as shown.

Style and navigation are the same as `presentations/rind-aarch64-perf/`.

## Run counts and conditions

All figures are rind's own rows. Times are medians unless stated.

| Device | What | Runs | Conditions | Source |
|---|---|---|---|---|
| Apple M1 Pro | criterion estimates (slides 4, 5, 8) | 1 run | rind `b6651423`, `admin/bench-report` via `cargo xtask mac-run`, Homebrew rustc 1.98.0; harness flagged the host busy (2.7 other CPUs busy): treat differences under 10–15% as noise. A later run at `8feee30e` (also busy) measured SHA-256 16 KB at 6.86 µs against 10.94 µs here; its notes call the earlier gap noise. | `hosts/m1pro/m1pro.json`, `hosts/m1pro/NOTES.md`, `results/m1pro-8feee30e/NOTES.md` |
| Apple M1 Pro | tests | — | 705 lib tests passed in forced-wide and narrow modes, plus integration and wipe tests, on the PR #82 branch (rustc 1.98.1) | rind PR #82 |
| Pi 5, lane 1 | SHA and host-table rows (slides 4, 8) | 1 run | rind `b6651423`, 0.0 other busy CPUs; the board is capped at 1.5 GHz by a boot-time service and this run probably ran capped, but that was not checked then. Public-key rows on slide 8 are the wide-multiplier build. | `hosts/pi5/pi5.json`, `hosts/pi5/NOTES.md` |
| Pi 5, lane 1 | PR #82 A/B (slides 5, 9) | 5 interleaved runs | narrow (default detection) vs wide (forced), pinned to core 3, 1.5 GHz, 64.8–71.9 °C (X25519, RSA) and 68.6–71.9 °C (P-256), throttle history bits only, no live throttling | rind PR #82; earlier PR #33 A/B in `hosts/pi5/ab-narrow-mul/` |
| Pi 5, lane 1 | tests | — | 703 lib tests passed (37 ignored) in default, wide and narrow modes, plus integration and wipe tests; clippy clean | rind PR #82 |
| Pi 5, lane 2 | tests | — | full test tiers with and without the crypto extensions: 1,269 passed, 33 ignored, 0 failed each; wipe, debug and slow CCTV tiers pass. Timed rounds pending. | lane log, not yet in rind-results |
| Pi 5, lane 3 | tests | — | same tiers: 1,239 passed, 32 ignored, 0 failed each (doctests not run). Fixed 1.5 GHz. Timed rounds: 2 of 5 complete at the time of writing. | lane log, not yet in rind-results |
| Pi 2 Model B | criterion estimates (slides 4, 5, 8) | median of 5 valid rounds | rind `a6577cb4`, armv7 cross-build, 900 MHz in every round, 42–46 °C, no throttling, core 3, performance governor; 345 results | `results/pi2b-bench/a6577cb4…/` on branch `results/pi2b-bench-armv7` |
| Pi 2 Model B | tests | — | 52 test binaries, 1,023 passed, 34 ignored, 0 failed, with NEON and with NEON denied; wipe tests 28 + 28 | same |
| XIAO MG24 | cycles (slides 4–6) | median of 5 run medians | EFR32MG24 Cortex-M33F at 78 MHz (clock checked against the crystal), 12 images, 0 KAT failures in every run | `results/efr32mg24/` |
| ESP32-S3 | cycles (slides 4–6) | median of 5 run medians per board | Xtensa LX7 at 240 MHz, flash 80 MHz, 2 boards (board 1 shown; boards agree within 0.3%), 0 KAT failures | `results/diag/esp32s3-second-host-2026-11-12/` |
| RP2040 Pico, Pico W | cycles (slides 4–6) | median of 5 rounds per board | Cortex-M0+ at 125 MHz, base build (upstream `c333c59e` + rind-hardware `a87d3a1f`, SysTick wrap fix), 207 rows 5/5 valid, 0 KAT failures, boards agree within 0.10%. A later re-verify on the Pico alone passed 66/66 KATs. | `presentations/rind-rp2040/` on branch `docs/rp2040-deck-r3`; `hosts/rp2040/` |
| Arduino Uno R3 | cycles (slides 4, 7) | median of 5 runs, after one priming call | ATmega328P at 16 MHz, Timer1 with no prescale (1 tick = 1 cycle). `no_std` port of rind's generic SHA-256, the rolled SHA-512 loop and Poly1305 from rind `c08c91ea`; not the full crate. 5/5 KATs pass; every timed digest matched a host check. ChaCha20 timings omitted. | `results/uno-r3/` on branch `results/uno-r3-2026-11-13` |

### SHA-512 caveat

The ESP32-S3 (98,199,751 cycles) and RP2040 (36,057,712) publish more cycles for SHA-512 16 KB than the 8-bit Uno
(27,108,699). They are shown as published. The Uno port uses the rolled SHA-512 loop from the RP2040 branch
(`c08c91ea`), while the ESP32-S3 and RP2040 rows are the base build. On the RP2040 that rolled loop measured about
3.42 M cycles in one run (n=1); that is not the 5-run baseline and is not used in the deck.

## Devices not covered

Also published on rind-results main but outside this deck: nRF52840 (Seeed T1000-E), Jetson Orin Nano, Pixel 9 Pro,
and x86_64 hosts.
