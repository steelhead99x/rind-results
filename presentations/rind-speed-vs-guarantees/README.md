<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Fast is not the only goal: speed versus guarantees

`index.html` is a self-contained HTML deck (no network access, no external libraries), 9 slides at 1280×720,
with the CSS and navigation of `presentations/rind-testing`. It covers two PRs merged into kixel-ai/rind on
19 November 2026 that each trade one thing for another and say so: #821 (SHA-512 on x86_64 runs no VSHA512
instruction, merged 2:48 PM PST; fork mirror #131) and #793 (SHA-2 on 32-bit microcontrollers, merged 9:51 AM PST).
It extends the "Gained and lost" slide of the rind-proofs deck. All times are America/Los_Angeles (PST).

1. Title
2. Intel's fixed-timing instruction list: SHA-1 and SHA-256 instructions yes, VSHA512 no
3. What #821 does: AVX2 routine, `ct-vartime` refuses `vsha512*`, the AVX2 routine is proven
4. The price, measured by the maintainer on a Core Ultra 7 270K Plus: 2.0× at 64 B, 4.2× to 4.6× from 1200 B, 0.82× to 0.86× aws-lc-rs
5. Kyle's Core Ultra 9 275HX: the published "before" at rind d6ca9a75 ran the SHA512 path; the "after" is pending a rerun
6. The OS can hide a CPU feature: Oracle Linux's `/proc/cpuinfo` lists `sha_ni` but not `sha512` on this chip,
   while rind's CPUID detection found `sha512`
7. #821 and #793 side by side: what each gives up and gains
8. What the fork tried and dropped (fork #131, #136) and the fork-safe DRBG self-test (#723)
9. Gaps and sources

## Which numbers come from where

- #821 merged body: Intel's list, the change, and the speed table (the maintainer's `desktop`, one CPU, `taskset -c 2`).
- rind-results main: `results/intel-core-ultra-9-275hx/d6ca9a75.../RESULTS.md` (sha512 64 B 62 ns, 1350 B 263 ns,
  16 KB 2.78 µs, 3 runs, 13 Nov).
- #821 commit a674bc6e message: this CPU at 2.0× to 2.2× (32 and 64 B) and 4.4× to 4.7× (from 1200 B) its old
  time, 0.81× to 0.86× aws-lc-rs. No raw log is in rind-results, so slide 5 gives only these ratios, and the
  "after" row is marked pending.
- #793 merged body: the proof tradeoff and its costs; its board numbers are the fork's.

## The 275HX has SHA512; the OS does not list it

Kyle confirmed on 19 Nov 2026 (11:20 PM PST) that the Core Ultra 9 275HX has the SHA512 instructions. A read-only
check on the Alienware that night: Oracle Linux Server 10.2, kernel `7.2.13-1.el10.elrepo.x86_64`, Intel Core
Ultra 9 275HX; `/proc/cpuinfo` flags list `sha_ni` but no `sha512`. rind's runtime CPUID detection found `sha512`
in the 13 Nov run (`results/intel-core-ultra-9-275hx/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RESULTS.md`), and
#821 commit a674bc6e timed this chip 2.0× to 2.2× and 4.4× to 4.7× slower after the change, which only happens if
the SHA512 path was in use. Slide 6 does not say why the kernel omits the flag; that is not verified. Takeaway:
trust CPUID detection over `/proc/cpuinfo`, and record both in every results run.

## Left out for lack of data

- An "after" chart on the 275HX: needs a rerun at the merged commit, saving both CPUID detection and `/proc/cpuinfo` flags.
- CPUs without the extension: only fork #136's percentages (1.8% slower verify, within-noise gain).

## PR links on the slides

Each slide tied to a PR has a muted `PRs:` line at the top of its footer. `rind #N` links to
https://github.com/kixel-ai/rind/pull/N (upstream), `fork #N` to https://github.com/steelhead99x/rind/pull/N,
and `rind-results #N` to https://github.com/steelhead99x/rind-results/pull/N. Fork and upstream numbers are
different PRs and are never mixed.

## Opening it

Open `index.html` in a browser. Navigate with ← / → or click either half of the slide. `#N` opens slide N.
