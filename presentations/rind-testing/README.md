<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Tests that catch more: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 11 slides at 1280×720,
with the CSS and navigation of `presentations/rind-hardware-today`. It covers the test and CI changes from the
fork that were merged into kixel-ai/rind between 14 and 19 November 2026. Written 17 November, refreshed
19 November from the merged PR bodies. All times are America/Los_Angeles (PST).

1. Title
2. Summary: seven merged upstream PRs with merge times, plus the fork sync
3. wasm and Ed25519 (kixel-ai/rind#797, merged 18 Nov 9:45 PM): 914 CCTV verdicts, wipe checks on wasm
4. Adapter mutants (kixel-ai/rind#792, merged 19 Nov 6:49 AM): mutant tallies per crate group
5. Edge cases: hash-to-curve sums (#791, merged 19 Nov 8:35 AM) and HPKE edge vectors (#798, merged 19 Nov 8:07 AM)
6. Constant-time checks: ctgrind on fork #126 and #79, and `ct-vartime` refusing VSHA512 (#821, merged 19 Nov 2:48 PM)
7. CI alerts (#796, merged 18 Nov 8:55 PM) and dev-shell cargo shims (#876, merged 19 Nov 5:52 AM)
8. The mutation tool fix (#725, merged 14 Nov 12:51 PM) and the fork sync (steelhead99x/rind#134)
9. The 15 November Xeon VM correctness run (rind-results PR #36), a "before" for these PRs
10. What was and was not run
11. Sources

## Changes on 19 November

- Status: every upstream PR changed from "ready for review" to merged, with its merge time.
- Corrected from the merged bodies:
  - #791: 52 tests passed (the 17 Nov deck said 9).
  - #797: wasm `heap_scratch` 15 passed (the deck said 14). The `just ci` table from the PR as opened
    (1,753 workspace tests on `1a798d6a`) is replaced by the merged body's Validation on `d00812be5`, and the
    slide now says the merge gate only clippy-checks the wasm tests; they run nightly.
  - #792: the per-crate test counts from the PR as opened are replaced by the merged body's mutant tallies
    (499 mutants, 364 caught, 135 unviable, 0 missed; 128, 109 caught, 1 timeout, 17 unviable, 1 missed).
  - #796: `sh/alert.sh check` covers 19 workflows (the deck said 18), and the alert rule is the merged one
    (post only on the time-limit annotation).
- Added: #876 and #725.

## Sources

- kixel-ai/rind#725, #791, #792, #796, #797, #798, #876 merged bodies; #821 merged body (Approach)
- steelhead99x/rind#126 and #79 bodies (ctgrind)
- steelhead99x/rind#134 body, commits 3d5520b9 and f1c59f5e, and comment 6964107098 (Dart verification)
- rind-results `notes/checks-2026-11-15-xeon-box.md` and `notes/raw/host-kat-2026-11-15-xeon-box.ndjson`

All counts are as stated in the PR bodies, not rind-results raw logs, except the Xeon VM slide.

## Left out for lack of data

- A before-and-after mutation score for the whole workspace: none exists.
- A run of #876 at the merged head: rind-results has only open PR #76's `just ci` log at fork head `bdae2ad6`.
- `admin/mutants --pins` for #792 and the rest of rind-rcgen's mutants (40 of 104 ran).

## PR links on the slides

Each slide tied to a PR has a muted `PRs:` line at the top of its footer. `rind #N` links to
https://github.com/kixel-ai/rind/pull/N (upstream), `fork #N` to https://github.com/steelhead99x/rind/pull/N,
and `rind-results #N` to https://github.com/steelhead99x/rind-results/pull/N. Fork and upstream numbers are
different PRs and are never mixed.

## Opening it

Open `index.html` in a browser. Navigate with ← / → or click either half of the slide. `#N` opens slide N.
