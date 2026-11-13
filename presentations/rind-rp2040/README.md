# rind on the RP2040: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 12 slides at 1280×720.
It covers:

1. Title
2. The board (RP2040 rev B2, Cortex-M0+ at 125 MHz), the November 12 backup, and the first flash
3. The five limits: no OS, no compare-and-swap (CAS), a 32×32→32 multiplier, no hardware RNG, 264 KiB of SRAM
4. Where rind started: 1672 errors at a4759c48 (see the note on that label below)
5. Patches 0003 (no_std) and 0005 (ReuseCache without CAS), and the placeholder getrandom backend
6. The current state: 12 UF2 bench images, first on-device run on 12 November 2026, known-answer checks passed
7. Which other Rust crypto libraries compile for this and nearby targets (compile only)
8. Other chips with the same limits, including the EFR32MG24
9. What is done on the board, and what is still to publish or send upstream
10. How results are measured, including the SysTick wrap-count bug found on the first run and the 300 ms cross-check
11. Live results: partial before/after numbers for SHA-512 and AES-GCM (and related rows); other benches and competitor times pending
12. Sources

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.
- The navigation hint sits below the slide; the slide is scaled to leave room for it.

## Updating it

Every number and table is in the `DATA` block (`<script id="deck-data">`) at the top of `index.html`. Each part of
`DATA` names the file or commit it came from. The CSS and navigation script are reused from `presentations/rind-spake2plus`.

### Updating the live results (slide 11)

Edit `DATA.live`:

1. Set `status` to `"partial"` while some rows are still missing, or `"complete"` when all are in. With
   `"pending"` the slide shows the pending banner and no numbers.
2. Fill `asOf` (time with zone), `source` (repository, branch and commit where the raw results are recorded), and `placement` (`"XIP flash"` or `"SRAM"`, where the hot code ran).
3. For a before/after patch comparison, set `compare` to `"before-after"` and for each row set `op`, `before`, `rind` (the after value), `beforeText`, `afterText`, and `ratio`, using the figures as the source wrote them.
4. For rind versus another library, leave `compare` unset. For each row, set `op`, `unit`, `rind`, `best`, and `bestLib`:
   - `unit: "cycles"` for SysTick-timed rows, `unit: "us"` for rows timed with the 1 MHz timer.
   - `rind` and `best` are medians of at least 5 samples, in that unit (see slide 10 for how samples are taken). `best` is the fastest other library
     from the builds listed in `baremetal/rp2040/COMPETITOR-BUILDS.md`; ring is excluded.
   - Leave a value `null` and that cell shows "pending".
5. The table and chart follow `compare`. Add the results file or commit to `DATA.sources` so it appears on the sources slide.

Only publish numbers that passed the on-device known-answer checks and were measured as slide 10 describes.
If a firmware README points at `rind-results/results/rp2040/` but that path is not in the repository, say so and cite the commit that actually has the figures.

## Sources

- https://github.com/steelhead99x/rind-results, branch `main`: `baremetal/rp2040/*`, `hosts/rp2040/*`, commits 2cb88c3 and 876586ce (firmware links now point at rind-hardware; `steelhead99x/rind-bench` is archived).
- https://github.com/steelhead99x/rind-hardware: `main` (8d9c1bb), `rp2040/systick-wrap-fix` (a9d057a), `rp2040/armv6m-sha512-ghash` (6abf7fd, including `rp2040/README.md`).
- https://github.com/steelhead99x/rind: `rp2040/armv6m-no-std` (c08c91ea), `no-std/thumbv6m-alloc` (3dc5634).
- The start token on the USB serial port is still `rind-bench:go`.

The measurement method on slide 10 started as the plan of 2026-11-12. The SysTick wrap-count bug, the fix, and the 300 ms cross-check (37,500,815 cycles against 300,021 µs, 125.0 MHz) come from rind-hardware a9d057a. The before/after cycle counts come from rind-hardware 6abf7fd and steelhead99x/rind c08c91ea (125 MHz, XIP cache on, interleaved rounds, median of per-round medians). Those numbers are not yet in `rind-results/results/rp2040/`.

The a4759c48 label: `hosts/rp2040/FEASIBILITY.md` calls it a rind commit; `baremetal/rp2040/COMPETITOR-BUILDS.md` takes its
versions (including rind 0.3.2) from `rind-bench/Cargo.toml` at a4759c48. The deck cites both.

No upstream pull request on kixel-ai/rind (the current home of moq-dev/rind) has been opened for this RP2040 work.
