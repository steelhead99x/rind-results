# rind aarch64 performance work: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries). It walks through the
rind aarch64 performance work from the starting point on 2026-11-11 to the state on 2026-11-12, one section per phase:

1. PR #33: narrow vs wide multiplier dispatch on the Cortex-A76 (Raspberry Pi 5)
2. The Pi 5 follow-up branches: narrow NIST, RSA bench sizes, cpu-detect, RSA-8n, mont-uninit, P-384 w7
3. The Argon2id NEON fill (M1 Pro)
4. Argon2id SHA3 `umlal`
5. The Argon2id two-round interleave, with the PMU root cause

It also covers verification status, what was dropped, the current state, next steps, and an appendix of branches and data files.

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.

## Updating it

Every number used by charts, tables and KPI cards is in the `DATA` block (`<script id="deck-data">`) at the
top of `index.html`. Edit the numbers there. The prose bullets on some slides quote the same numbers, so search for and update those too.
Each phase is a group of `<section class="slide" data-section="...">` elements. Slide numbers and footers are generated.

Pi 5 caveat: the board's `scaling_max_freq` was found capped at 1.5 GHz during the Argon2 session. Every Pi
slide carries that note; compare ratios within a run, not absolute times across runs.

## Sources

Only facts from these sources are used. Numbers with no source are left out or marked "pending".

- steelhead99x/rind-results `results/m1pro-pi5-argon2-2026-11-12`: `results/pr33-pi5/README.md`, `raw.md`;
  `results/argon2/RESULTS.md` and `raw/`; `results/m1pro-8feee30e/NOTES.md`
- steelhead99x/rind-results `results/argon2-a76-interleave`: `results/argon2-interleave/RESULTS.md` and `raw/`;
  `scripts/argon2-pmu/`, `scripts/argon2-pmu-*.sh`, `scripts/argon2-pmu-sum.py`
- steelhead99x/rind-results `main`: `hosts/pi5/NOTES.md`
- The Pi lane's rind-results working tree (not yet committed when this deck was made):
  `hosts/pi5/ab-narrow-mul/{README.md,ab-narrow-mul.md,ab-rsa-8n.md}`, `hosts/m1pro/ab-narrow-mul/`; the lane's PR drafts
  (`lane-pi5/pr/*.md`) and `lane-pi5/STATUS.md`
- `ARGON2-STATUS.md` (Argon2 agent status, 2026-11-12)
- The rind-verify and rind-scout status notes (`rind-verify/STATUS.md`, `rind-scout/FINDINGS.md`)
- steelhead99x/rind commit messages: `cursor/aarch64-narrow-mul-132a` (PR #33), `perf/argon2-neon`,
  `perf/argon2-sha3-umlal`, `perf/argon2-interleave`, `perf/pi5-narrow-nist`, `perf/pi5-rsa-bench-sizes`,
  `perf/pi5-cpu-detect`, `perf/pi5-rsa-8n`, `perf/pi5-mont-uninit`, `perf/pi5-p384-verify-w7`
