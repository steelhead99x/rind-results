<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# aarch64 measurements, 11–14 November 2026

`index.html` is 8 slides at 1280×720. Open it in a browser. Arrow keys or `#N`.
Print from Chrome with margins none and background graphics on. Each slide is one page.

Pi 5 times are at a fixed 1.5 GHz (`hosts/pi5/ab-narrow-mul/README.md`).

1. Title.
2. Narrow versus wide, 5-round median, built on #33 at `86de4410`.
3. NIST, width-7, rsa-8n, and mont-uninit follow-ups. Ratios are the ones printed in the README.
4. Argon2id `umlal`, commit `a05b78e4`.
5. Two-round interleave, commit `d7b1f399`.
6. Scalar riders, commit `3025440d`.
7. #82 head `ece9afa7`. Narrow only for A72, A76, and Neoverse N1. The Pi 5 table was not re-run on that head.
8. #79 head `54d339e7`, not re-timed. The measured A/B is the earlier Pi 5 one.

## Sources

- `hosts/pi5/ab-narrow-mul/README.md`, `series2-layout/README.md`, `table-align/README.md`
- `results/pr33-pi5/README.md` (3-round set, cited, not reprinted as the main table)
- `results/argon2/RESULTS.md`, `results/argon2-interleave/RESULTS.md`, `results/argon2-scalar-rider/RESULTS.md`
- steelhead99x/rind#82 body and #79 body plus comment 6859469361
- `results/jetson-orin-nano/facts.txt` for the A78AE part id

A dated log is in `PROGRESS.md`.
