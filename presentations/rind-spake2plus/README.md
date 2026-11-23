# SPAKE2+ comes to rind: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 4 slides at 1280×720.
Written 12 November 2026; the pull request slides were updated on 19 November after the merge. kixel-ai/rind
was named moq-dev/rind when the deck was written; links now point at kixel-ai/rind. Times are PST.

Cut on 23 November (rind-results #34) to sourced tables: the protocol explanation, the earlier-attempts history,
the copied Pi 5 performance slides (they are in rind-aarch64-perf) and the sources slide are gone.

1. Title
2. The RFC 9383 and Matter PASE suites in rind, and how the type system keeps them apart
3. kixel-ai/rind#689's Validation: merged 13 November 10:39 AM, `just ci` 1100 passed / 37 skipped on `c5da7311`,
   ctgrind 0 errors, 13 SPAKE2+ tests
4. The four fixes split out of #689 with authorship kept: #724 (13 Nov 11:26 AM), #722 (13 Nov 11:53 AM),
   #725 (14 Nov 12:51 PM), #723 (14 Nov 4:53 PM)

There is no SPAKE2+ speed chart: #689 says no benchmark applies, since ECDH and ECDSA never reach the changed
P-256 code. The #723 timing (about 9.6 µs per thread against about 19.6 µs to spawn one) is the maintainer's,
with no raw log in rind-results.

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

## PR links on the slides

Each slide tied to a PR has a muted `PRs:` line at the top of its footer. `rind #N` links to
https://github.com/kixel-ai/rind/pull/N (upstream), `fork #N` to https://github.com/steelhead99x/rind/pull/N,
and `rind-results #N` to https://github.com/steelhead99x/rind-results/pull/N. Fork and upstream numbers are
different PRs and are never mixed.

## Sources

- https://github.com/kixel-ai/rind/pull/689 (merged body and timeline) and the split-out PRs
  https://github.com/kixel-ai/rind/pull/722, /723, /724, /725; earlier attempts #616, #680, #685; closed #709
- steelhead99x/rind#67 and #68 (the fork PRs the split-out commits and the constant-time check came from)
- RFC 9383: https://www.rfc-editor.org/rfc/rfc9383
- draft-bar-cfrg-spake2plus-01: https://datatracker.ietf.org/doc/html/draft-bar-cfrg-spake2plus-01
- steelhead99x/rind-results branches `results/m1pro-pi5-argon2-2026-11-12`, `results/pi5-lane-2026-11-12`,
  `results/argon2-a76-interleave`, `main` (`hosts/pi5/NOTES.md`), and `docs/perf-phases-deck` (the companion deck)
