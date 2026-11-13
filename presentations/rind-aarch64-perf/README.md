# rind aarch64 performance work: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries). It walks through the
rind aarch64 performance work from the starting point on 2026-11-11 to the state on 2026-11-13, one section per phase:

1. PR #33: narrow vs wide multiplier dispatch on the Cortex-A76 (Raspberry Pi 5)
2. The Pi 5 follow-up branches: narrow NIST, RSA bench sizes, cpu-detect, RSA-8n, mont-uninit, P-384 and P-521 w7,
   and the P-256 generator-table alignment fix that removed cpu-detect's layout-dependent loss
3. The Argon2id NEON fill (M1 Pro)
4. Argon2id SHA3 `umlal`
5. The Argon2id two-round interleave, with the PMU root cause, and the scalar rider rounds

It also covers verification status, what was dropped, the current state, next steps, a status slide that says which
sources are on rind-results main (all of them since rind-results PRs #4 and #5 merged on 2026-11-13, plus the
separate Jetson Orin Nano full-suite run),
and an appendix of branches and data files.

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.

## Updating it

Every number used by charts, tables and KPI cards is in the `DATA` block (`<script id="deck-data">`) at the
top of `index.html`. Edit the numbers there. The prose bullets on some slides quote the same numbers, so search for and update those too.
Each phase is a group of `<section class="slide" data-section="...">` elements. Slide numbers and footers are generated.

Pi 5 clock: the Pi 5 used here has no active cooling, so a boot-time setting deliberately holds its CPU at
1.5 GHz (nominal maximum 2.4 GHz). Short 2.4 GHz bursts would otherwise push the SoC into thermal throttling. All
Pi 5 figures are therefore at a fixed 1.5 GHz. Both sides of every A/B ran at the same clock, so the ratios hold,
but absolute times are longer than an actively cooled 2.4 GHz Pi 5 would show. Slide 5 explains this, and every Pi slide points to it.

## Sources

Only facts from these sources are used. Numbers with no source are left out or marked "pending".

All of these are on steelhead99x/rind-results `main` (PR #4 from `results/pi5-lane-2026-11-12` and PR #5 from
`results/argon2-a76-interleave`, both merged 2026-11-13):

- `results/pr33-pi5/README.md`, `raw.md`; `results/argon2/RESULTS.md` and `raw/`; `results/m1pro-8feee30e/NOTES.md`
- `results/argon2-interleave/RESULTS.md` and `raw/`; `scripts/argon2-pmu/`, `scripts/argon2-pmu-*.sh`, `scripts/argon2-pmu-sum.py`
- `results/argon2-scalar-rider/RESULTS.md` and `raw/`
- `hosts/pi5/NOTES.md`, `hosts/pi5/ab-narrow-mul/` (README.md, ab-narrow-mul, ab-rsa-8n, ab-cpu-detect, ab-p384-w7,
  ab-p521-w7, series2-layout/, table-align/), `hosts/m1pro/ab-narrow-mul/`
- `results/jetson-orin-nano/` (status slide only)
- Working status notes kept beside the checkouts (Argon2, Pi follow-ups, constant-time verification, static llvm-mca scout), 2026-11-12 and 2026-11-13
- PR state on GitHub: steelhead99x/rind #33 (closed unmerged), #79 and #80 (open); steelhead99x/rind-results #4 and #5 (merged)
- steelhead99x/rind commit messages: `cursor/aarch64-narrow-mul-132a` (PR #33), `perf/argon2-neon`,
  `perf/argon2-sha3-umlal`, `perf/argon2-interleave`, `perf/pi5-narrow-nist`, `perf/pi5-rsa-bench-sizes`,
  `perf/pi5-cpu-detect`, `perf/pi5-rsa-8n`, `perf/pi5-mont-uninit`, `perf/pi5-p384-verify-w7`,
  `perf/pi5-p521-verify-w7`, `fix/p256-table-align`, `perf/argon2-neon-scalar-rider`

A dated log of what each refresh changed is in `PROGRESS.md`.
