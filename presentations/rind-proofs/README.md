<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Keeping rind's proven code proven: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 8 slides at 1280×720,
with the CSS and navigation of `presentations/rind-hardware-today`. It covers the proof side of the November
2026 work, as of 17 November 2026, with PR status refreshed on 19 November. All times are America/Los_Angeles (PST).

1. Title
2. What "proven" means here: Lean over rind's Rust, HOL Light over imported assembly
3. P-256 on x86_64 (steelhead99x/rind#79): the point addition moves to s2n-bignum's routine, proven on every x86_64 OS
4. The Lean fix: 3,705 jobs build locally; CI's `lean`, HOL Light and `cosim` jobs pass on `c2b8f5ca`
5. Table alignment (kixel-ai/rind#790) and its proofs: the Pi 5 build, the fork CI run, and what is left open
6. Gained and lost: SHA-512 on newer Intel moves onto proven code (#821); SHA-256 on ESP-IDF Xtensa leaves it (#793)
7. Where each PR stands: #821 merged 19 Nov 2:48 PM, #793 merged 19 Nov 9:51 AM, #790 closed unmerged 18 Nov, fork #79 open
8. Sources

## Sources

- steelhead99x/rind#79 body, comments 6964936428, 6900182949 and 6859469361, and its CI checks on head `c2b8f5ca`
- rind-results `hosts/pi5/ab-narrow-mul/table-align/README.md` (Lean section, 3,612 jobs on the Pi 5)
- kixel-ai/rind#790 body; kixel-ai/rind#793 and #821 merged bodies. On 19 Nov the #821 cost on slide 6 was
  changed to the merged body's figures (maintainer, Core Ultra 7 270K Plus: 2.0× at 64 B, 4.2 to 4.6× from
  1200 B, 0.82 to 0.86× aws-lc-rs) in place of the earlier commit a674bc6e figures
- steelhead99x/rind#69 body

## Left out for lack of data

- No proof-job timing or count is given for kixel-ai/rind#790 itself: Lean was not run in the port's environment.
- The x86_64 P-256 proof's `generator_table` change in #790 is listed as not rechecked, not as passing.
- No proof plan exists yet to cite for the rolled loops; #793 says the maintainer plans a follow-up quest.

## PR links on the slides

Each slide tied to a PR has a muted `PRs:` line at the top of its footer. `rind #N` links to
https://github.com/kixel-ai/rind/pull/N (upstream), `fork #N` to https://github.com/steelhead99x/rind/pull/N,
and `rind-results #N` to https://github.com/steelhead99x/rind-results/pull/N. Fork and upstream numbers are
different PRs and are never mixed.


## Opening it

Open `index.html` in a browser. Navigate with ← / → or click either half of the slide. `#N` opens slide N.
