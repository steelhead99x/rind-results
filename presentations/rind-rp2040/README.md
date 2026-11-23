# rind on the RP2040: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 3 slides at 1280×720.
It covers:

1. Title
2. Where rind started: 1672 errors at a4759c48 (see the note on that label below), plus the 2026-11-12 upstream count (1,739 → 0)
3. Which other Rust crypto libraries compile for this and nearby targets (compile only)

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.
- The navigation hint sits below the slide; the slide is scaled to leave room for it.

## Updating it

Every number and table is in the `DATA` block (`<script id="deck-data">`) at the top of `index.html`. Each part of
`DATA` names the file it came from. The CSS and navigation script are reused from `presentations/rind-spake2plus`.

### Updating the live results (slide 10)

Edit only `DATA.live`:

1. Set `status` to `"partial"` while rounds are still running, or `"complete"` when all are in. With
   `"pending"` the slide shows the pending banner and no numbers. `title`, if set, replaces the generated slide title;
   update or remove it when the numbers change.
2. Fill `asOf` (time with zone), `source` (branch, file and commit in this repository where the raw results are
   published), and `placement` (`"XIP flash"` or `"SRAM"`, where the hot code ran).
3. For each row, set `op` (for example `"seal 1 KiB"`), `unit`, `rind`, `best`, `bestLib` and `rounds`:
   - `unit: "cycles"` for SysTick-timed rows (every row so far), `unit: "us"` for rows timed with the 1 MHz timer.
   - `rind` and `best` are the median of per-round medians from `summarize.py` (see slide 9). `best` is the fastest other
     library from the builds listed in `baremetal/rp2040/COMPETITOR-BUILDS.md`; ring is excluded.
   - `rounds` is "valid rounds / rounds counted" for that row.
   - Leave a value `null` and that cell shows "pending". Set `noOther: true` when no other library is in the image.
4. The table shows cycles (or µs), and the chart draws rind ÷ fastest other for every row that has both values.
5. Add the results file to `DATA.sources` so it appears on the sources slide.

Only publish numbers that passed the on-device known-answer checks and were measured as slide 10 describes.

### The 2026-11-12 slides (6–15, 18)

Their numbers are in the `DATA` blocks `snap`, `pico`, `systick`, `sha512`, `layout`, `ghash`, `m0`, `p256`, `nostd`,
`status` and `live`. They are a snapshot from 2026-11-12 18:35 PST, while rounds were still running: the aes_gcm, sha2, hmac, ed25519, hkdf
and sha3 rows have 3 of 5 planned rounds, and the other six rows have 2. Replace them
when the remaining rounds land, and say how many rounds each number is based on. Estimates (the SHA-512 refetch cost,
the X25519 lmul count) are labeled as estimates on the slides; keep them labeled.

## Sources

All in https://github.com/steelhead99x/rind-results, branch `main`:

- `baremetal/rp2040/README.md`, `baremetal/rp2040/COMPETITOR-BUILDS.md`, `baremetal/rp2040/TARGET-SURVEY.md`,
  `baremetal/rp2040/builds/README.md`, `baremetal/mg24/README.md`
- `hosts/rp2040/DEVICE.md`, `hosts/rp2040/FEASIBILITY.md`, `hosts/rp2040/NOTES.md`, `hosts/rp2040/images.json`
- commits 2cb88c3, f4e0b1d, cb264f8

The 2026-11-12 on-device work is not yet published in this repository. Slides 2 and 6–15 and 18 cite these working notes by name
(no local paths, board serials or hostnames are in the deck):

- RP2040 bench lane: STATUS notes, round logs, raw run JSON summarized with `summarize.py` (median of per-round
  medians; a round more than 3% off is marked invalid), and the drafted SHA-512 and GHASH commit messages
- M0+ static-analysis lane: `REPORT.md` and `STATUS.md` (llvm-nm / llvm-objdump on the SysTick-fixed images, rustc 1.98.1)
- no_std lane: `STATUS.md` and its gates log
- X25519 / Ed25519 / Poly1305 lane: `STATUS.md` (work in progress)

The fairness rules on slide 9 come from the firmware team's measurement plan as of 2026-11-12. The SysTick
extension to 32 bits and the 1 MHz timer as a startup cross-check come from `hosts/rp2040/NOTES.md` at 2cb88c3.
No pull request has been opened upstream; slide 18 lists the branches being prepared.

The a4759c48 label: `hosts/rp2040/FEASIBILITY.md` calls it a rind commit; `baremetal/rp2040/COMPETITOR-BUILDS.md` takes its
versions (including rind 0.3.2) from `rind-bench/Cargo.toml` at a4759c48. The deck cites both. 
