# SPAKE2+ comes to rind: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 16 slides at 1280×720.
It covers:

1. What SPAKE2+ (RFC 9383) does, with a one-page flow of prover and verifier
2. How rind implements it: the RFC 9383 and Matter PASE suites, and the constant-time identity check
3. The pull request moq-dev/rind#689: timeline, tests, and its status at 4:22 PM PST on 2026-11-12
4. A summary of the aarch64 performance work from 2026-11-11 to 2026-11-12, and the Raspberry Pi 5 follow-up branches

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.

## Updating it

The benchmark numbers are in the `DATA` block (`<script id="deck-data">`) at the top of `index.html`. It is a copy of the
DATA block in `presentations/rind-aarch64-perf/index.html`; if that deck's numbers change, copy them over and update the
prose that quotes them. The CSS and navigation script are reused from the same deck.

Pi 5 clock: the board has no fan, so its CPU is held at a fixed 1.5 GHz (nominal 2.4 GHz). Both sides of every A/B ran at
that clock, so ratios hold, but absolute times are longer than a cooled 2.4 GHz Pi 5 would show.

## Sources

- https://github.com/moq-dev/rind/pull/689 and https://github.com/moq-dev/rind/pull/709
- RFC 9383: https://www.rfc-editor.org/rfc/rfc9383
- draft-bar-cfrg-spake2plus-01: https://datatracker.ietf.org/doc/html/draft-bar-cfrg-spake2plus-01
- steelhead99x/rind-results branches `results/m1pro-pi5-argon2-2026-11-12`, `results/pi5-lane-2026-11-12`,
  `results/argon2-a76-interleave`, `main` (`hosts/pi5/NOTES.md`), and `docs/perf-phases-deck` (the companion deck)
