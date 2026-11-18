<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Keeping rind's proven code proven: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 8 slides at 1280×720,
with the CSS and navigation of `presentations/rind-hardware-today`. It covers the proof side of the November
2026 work, as of 17 November 2026. All times are America/Los_Angeles (PST).

1. Title
2. What "proven" means here: Lean over rind's Rust, HOL Light over imported assembly
3. P-256 on x86_64 (steelhead99x/rind#79): the point addition moves to s2n-bignum's routine, proven on every x86_64 OS
4. The Lean fix: 3,705 jobs build locally; CI's `lean`, HOL Light and `cosim` jobs pass on `c2b8f5ca`
5. Table alignment (kixel-ai/rind#790) and its proofs: the Pi 5 build, the fork CI run, and what is left open
6. Gained and lost: SHA-512 on newer Intel moves onto proven code (#821); SHA-256 on ESP-IDF Xtensa leaves it (#793)
7. Where each PR stands
8. Sources

## Sources

- steelhead99x/rind#79 body, comments 6964936428, 6900182949 and 6859469361, and its CI checks on head `c2b8f5ca`
- rind-results `hosts/pi5/ab-narrow-mul/table-align/README.md` (Lean section, 3,612 jobs on the Pi 5)
- kixel-ai/rind#790 and #793 bodies; kixel-ai/rind#821 commits 628fdb69, 067bdc17, a674bc6e;
  `quest/m2/confidence/intel-vsha512.md` on kixel-ai/rind main
- steelhead99x/rind#69 body

## Left out for lack of data

- No proof-job timing or count is given for kixel-ai/rind#790 itself: Lean was not run in the port's environment.
- The x86_64 P-256 proof's `generator_table` change in #790 is listed as not rechecked, not as passing.
- No proof plan exists yet to cite for the ESP-IDF Xtensa SHA-256 body; the deck says only that the PR lists one as a follow-up.

## Opening it

Open `index.html` in a browser. Navigate with ← / → or click either half of the slide. `#N` opens slide N.
