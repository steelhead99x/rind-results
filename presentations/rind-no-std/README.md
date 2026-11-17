# Building rind without std: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 12 slides at 1280×720,
with the CSS and navigation script of `presentations/rind-hardware-today` (itself from `presentations/rind-rp2040`).
It covers the upstream quest to make rind build without `std`, the fork pull request that does it, and the board
work that shows why it matters. All times are America/Los_Angeles (PST).

1. Title
2. Why no_std: the quest's goal, the 2026-11-12 ecosystem bet, and who depends on it
3. What blocks it today: the quest's list of `std` uses, and the build problems #84 found
4. The approach in #84: `no_std` on `target_os = "none"` with no `std` feature, backend choice, shared state
5. `x86_64-unknown-none`: portable by default, the SSE2 opt-in, and the AVX decision that differs from the quest
6. What changes and how it is split: the diff against the merge base by area, and the PR's own commits
7. The CI targets: the quest's three and the eight that `sh/bare-metal.sh` lints, with backend and reuse cache per target
8. The boards: ESP32-S3, nRF52840, EFR32MG24, RP2040 (known-answer checks passed, no timing tree), Uno R3, Pi 2B
9. From the rind-hardware base patches 0001–0005 to #84, and where they differ
10. The guard on hosted speed: SHA-256 64 B and AES-GCM 128 B against aws-lc-rs, before and after
11. Open questions and next steps
12. Sources

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End) or click the right
or left half of the slide. `#N` in the URL opens slide N. To make a PDF, print from Chrome or Edge with margins
"None" and background graphics on; each slide prints as one 1280×720 page.

## Sources

- The upstream quest [`quest/m2/adoption/no-std.md`](https://github.com/kixel-ai/rind/blob/main/quest/m2/adoption/no-std.md)
  on kixel-ai/rind main (slides 2, 3, 5, 7, 10).
- [steelhead99x/rind#84](https://github.com/steelhead99x/rind/pull/84), "feat: build rind without std on bare metal",
  branch `cursor/no-std-bare-metal-336d`, head `5821c3c5`, open and marked conflicting with fork main when checked on
  16 November 2026: its description, its commit list, and its diff against the merge base `19f94a07`
  (`sh/bare-metal.sh`, `sh/dispatch.sh`, `justfile`, `flake.nix`, `PLATFORMS.md`, `UPSTREAM.md`, `rind/src/lib.rs`,
  `rind/build.rs`, `rind/src/low/generic/reuse_cache_without_cas.rs`, `quest/m2/no-cas-reuse-cache.md`,
  `quest/m2/84-bare-metal-no-std.md`) (slides 3–7, 9–11). The file counts on slide 6 are from
  `git diff --numstat 19f94a07 5821c3c5`; GitHub lists more files because the branch carries merges.
- [steelhead99x/rind#85](https://github.com/steelhead99x/rind/pull/85), stacked on #84 (slide 11).
- rind-results main `88cbf85`: `results/esp32s3/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RESULTS.md`,
  `results/nrf52840/RESULTS.md`, `results/efr32mg24/RESULTS.md`, `results/uno-r3/README.md`,
  `results/pi-host-a-bench/a6577cb4a4b48af06385f8d748f3892c07cc229e/RESULTS.md`, `hosts/rp2040/NOTES.md`,
  `baremetal/mg24/README.md`, `baremetal/rp2040/TARGET-SURVEY.md`, and slide 15 of `presentations/rind-rp2040`
  (slides 8, 9).
- [steelhead99x/rind-hardware](https://github.com/steelhead99x/rind-hardware) main `8dc9811`: `README.md`,
  `patches/README.md`, and the commit messages of `patches/0001`–`0005` (slides 8, 9).

The hosted speed figures on slide 10 are the PR's own: median of five rounds, fork main `f6d54b18` against the branch
at `ea548396`. The change column and the rind ÷ aws-lc-rs ratios are computed from those medians. The per-call SHA
path pick came later and was measured on its own, without aws-lc-rs rows.

## Updating it

When #84 is merged or ported upstream, update slides 6, 7 and 11 with the merged commit, and replace the hosted
speed table on slide 10 if a quiet rerun is published. When an RP2040 timing tree reaches main, update its row on
slide 8.
