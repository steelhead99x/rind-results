# rind on the RP2040: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 12 slides at 1280×720.
It covers:

1. Title
2. The board (RP2040 rev B2, Cortex-M0+ at 125 MHz) and how its original firmware was backed up
3. The five limits: no OS, no compare-and-swap (CAS), a 32×32→32 multiplier, no hardware RNG, 264 KiB of SRAM
4. Where rind started: 1672 errors at a4759c48 (see the note on that label below)
5. Patches 0003 (no_std) and 0005 (ReuseCache without CAS), and the placeholder getrandom backend
6. The current state: 12 UF2 bench images, built but not yet run on the chip
7. Which other Rust crypto libraries compile for this and nearby targets (compile only)
8. Other chips with the same limits, including the EFR32MG24
9. The plan for on-device runs
10. How results will be measured (the rind-bench team's plan, not results)
11. Live results: pending until on-device runs land
12. Sources

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.
- The navigation hint sits below the slide; the slide is scaled to leave room for it.

## Updating it

Every number and table is in the `DATA` block (`<script id="deck-data">`) at the top of `index.html`. Each part of
`DATA` names the file it came from. The CSS and navigation script are reused from `presentations/rind-spake2plus`.

### Updating the live results (slide 11)

Edit only `DATA.live`:

1. Set `status` to `"partial"` while some rows are still missing, or `"complete"` when all are in. With
   `"pending"` the slide shows the pending banner and no numbers.
2. Fill `asOf` (time with zone), `source` (branch, file and commit in this repository where the raw results are
   published), and `placement` (`"XIP flash"` or `"SRAM"`, where the hot code ran).
3. For each row, set `op` (for example `"seal 1 KiB"`), `unit`, `rind`, `best`, and `bestLib`:
   - `unit: "cycles"` for SysTick-timed rows, `unit: "us"` for rows timed with the 1 MHz timer.
   - `rind` and `best` are medians of at least 5 samples, in that unit (see slide 10 for how samples are taken). `best` is the fastest other library
     from the builds listed in `baremetal/rp2040/COMPETITOR-BUILDS.md`; ring is excluded.
   - Leave a value `null` and that cell shows "pending".
4. The table shows both units (cycles = µs × 125 at 125 MHz), and the chart draws rind ÷ fastest other for every row
   that has both values. The slide title changes from "pending" to a count of rows in once `status` is not `"pending"`.
5. Add the results file to `DATA.sources` so it appears on the sources slide.

Only publish numbers that passed the on-device known-answer checks and were measured as slide 10 describes.

## Sources

All in https://github.com/steelhead99x/rind-results, branch `main`:

- `baremetal/rp2040/README.md`, `baremetal/rp2040/COMPETITOR-BUILDS.md`, `baremetal/rp2040/TARGET-SURVEY.md`,
  `baremetal/rp2040/builds/README.md`, `baremetal/mg24/README.md`
- `hosts/rp2040/DEVICE.md`, `hosts/rp2040/FEASIBILITY.md`, `hosts/rp2040/NOTES.md`, `hosts/rp2040/images.json`
- commits 2cb88c3, f4e0b1d, cb264f8

The measurement method on slide 10 is the rind-bench team's plan as of 2026-11-12. It describes how results will be
taken; it is not a result. The note on that slide about the images built so far (SysTick extended in software to
32 bits, the 1 MHz timer as a startup cross-check) comes from `hosts/rp2040/NOTES.md` at 2cb88c3.

The a4759c48 label: `hosts/rp2040/FEASIBILITY.md` calls it a rind commit; `baremetal/rp2040/COMPETITOR-BUILDS.md` takes its
versions (including rind 0.3.2) from `rind-bench/Cargo.toml` at a4759c48. The deck cites both. No other branch had RP2040 data newer than `main` at 2cb88c3 when this deck was made.
