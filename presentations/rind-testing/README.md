<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind tests and checks: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 9 slides at 1280×720,
with the CSS and navigation of `presentations/rind-hardware-today`. It covers the test, constant-time and CI
work of November 2026, as of 17 November 2026. All times are America/Los_Angeles (PST).

1. Title
2. Summary: five upstream PRs ready for review, one fork sync merged, one VM check run on main
3. wasm and Ed25519 (kixel-ai/rind#797): 914 CCTV verdicts, wipe checks on wasm, the `just ci` counts
4. Adapter mutants (kixel-ai/rind#792): test counts per adapter crate
5. Edge cases: hash-to-curve sums (#791) and the HPKE revision's edge vectors (#798)
6. Constant-time checks: ctgrind on the ML-DSA and P-256 changes, and `ct-vartime` refusing VSHA512 (#821)
7. CI alerts (#796) and the fork sync (steelhead99x/rind#134)
8. The 15 November Xeon VM correctness run (rind-results PR #36)
9. What is left out, and sources

## Sources

- kixel-ai/rind#791, #792, #796, #797 bodies; #798 commits d21a82c3 and 4aad0a63 and its quest file
- steelhead99x/rind#126 and #79 bodies (ctgrind); kixel-ai/rind#821 commit 4342478f
- steelhead99x/rind#134 body, commits 3d5520b9 and f1c59f5e, and comment 6964107098 (Dart verification)
- rind-results `notes/checks-2026-11-15-xeon-box.md` and `notes/raw/host-kat-2026-11-15-xeon-box.ndjson`

## Left out for lack of data

- A "488 wasm32 tests passed" figure for #797: no PR body, comment, CI log or file found states it. The deck uses
  #797's own counts instead (wasm `cctv` 1 passed, `heap_scratch` 14 passed, workspace 1,753 passed).
- `just ci` on the upstream ports #791, #792 and #793: not run there (no Nix on that host), so no gate result is shown for them.
- `admin/mutants --pins` for #792: not run (cargo-mutants not installed).

## Opening it

Open `index.html` in a browser. Navigate with ← / → or click either half of the slide. `#N` opens slide N.
