<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind host speed-ups: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 10 slides at 1280×720,
with the CSS and navigation of `presentations/rind-hardware-today`. It covers four speed changes that
steelhead99x sent for review, as of 17 November 2026. PR status refreshed on 19 November: fork #126 closed for the
maintainer's kixel-ai/rind#863 (same reorder, merged 17 Nov 9:40 PM; the maintainer's PR, not this fork's),
kixel-ai/rind#780 and #790 closed without a merge on 18 Nov 10:07 PM, fork #79 still open. All times are America/Los_Angeles (PST).

1. Title
2. Summary: what each change measured, where, and its PR state
3. ML-DSA signing: the rejection-order idea and the x86_64 paired timing
4. ML-DSA signing on the NVIDIA Jetson Orin Nano: −4.4%, −7.7%, −7.1%
5. P-256 on x86_64: where a key multiply spends its time, before and after s2n-bignum's addition
6. P-256 on x86_64: end-to-end criterion run, and the noise floor
7. Portable big-number multiply: four products before the carry
8. Generator tables on a 64-byte line: neutral or a removed loss, by where the linker put them
9. What is not measured or not published yet
10. Sources

## Sources

Only the Pi 5 alignment runs are a rind-results set (`hosts/pi5/ab-narrow-mul/table-align/`, `series2-layout/`).
Every other figure comes from a PR body or PR comment, named in each slide's footer:

- steelhead99x/rind#126 body and comment 6936142476 (Orin A/B), closed in favor of kixel-ai/rind#863
- steelhead99x/rind#79 body (Profile, Benchmarks run 1, Waiting)
- kixel-ai/rind#780 body (multiply medians from steelhead99x/rind#76 head 7ebf1a90)
- kixel-ai/rind#790 body

## Left out for lack of data

- The P-256 swap on AMD Zen: not measured (#79 Waiting).
- End-to-end RSA time for the #780 multiply: not measured.
- #79's run 2 (P-256 only): shown only as "overstates the gain", since its base drifted with host load.
- #790's Criterion table: it compares rind with aws-lc-rs at one head, not the change with its parent, so it is not used as a speed claim.

## PR links on the slides

Each slide tied to a PR has a muted `PRs:` line at the top of its footer. `rind #N` links to
https://github.com/kixel-ai/rind/pull/N (upstream), `fork #N` to https://github.com/steelhead99x/rind/pull/N,
and `rind-results #N` to https://github.com/steelhead99x/rind-results/pull/N. Fork and upstream numbers are
different PRs and are never mixed.

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End) or click the right
or left half of the slide. `#N` in the URL opens slide N. To print, use Chrome's "Save as PDF" with margins
"None" and background graphics on.
