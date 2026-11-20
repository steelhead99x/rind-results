<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind on a Fire TV Stick (AFTSSS): slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 9 slides at
1280×720, with the CSS and navigation of `presentations/rind-speed-vs-guarantees`. It covers the bench
run of rind on an Amazon Fire TV Stick, model `AFTSSS` (MediaTek MT8695, 4x Cortex-A53, 32-bit
userland), on 19 November 2026, 9:38 to 10:12 AM PST, the same-day repeats, and where the fork's
fixes stand. All times are America/Los_Angeles (PST).

1. Title: rind faster in 88 of 118 comparisons with other libraries, 17 even, 13 slower (5% band)
2. Device and setup: hardware, Fire OS at the run and now, CPU features, build target, commits, how it ran over adb
3. Results, symmetric: AES-GCM, SHA and HMAC level or faster; ChaCha20-Poly1305 0.82× to 0.90× of the fastest other
4. Results, public key: chart of the fastest other library's time over rind's; P-256 0.83× to 0.91× of OpenSSL
5. First run against the repeat on fork main `847a3e3c`: the AES-GCM open gap did not repeat
6. Rows the first run skipped, rind only, with peak RSS (9.1 to 70.3 MB)
7. What it shows: the measured causes of the ChaCha20-Poly1305, small-message and P-256 gaps
8. Status: done, pending, failing
9. Next steps

## Which numbers come from where

Everything is in [`results/firetv-aftsss/`](../../results/firetv-aftsss/README.md):

- Slides 1, 3, 4: `firetv-aftsss.json` and `RESULTS.md` (run at steelhead99x/rind `ac0f2f93`); pair counts from `NOTES.md`.
- Slide 2: `README.md`, `raw/cpuinfo.txt`, `raw/tv/cpuid.json`, `harness/run.sh`. The current Fire OS build
  (`PS7717.5746N`, patch 2026-03-01) is a read-only `getprop` on 20 Nov, 12:48 AM PST.
- Slide 5: `raw/handoff/bench-main-repeat.txt` against `RESULTS.md`.
- Slide 6: `raw/handoff/lowmem-rind-rows.txt` and `raw/handoff/lowmem-run.txt`.
- Slide 7: `raw/handoff/probe-components.txt` and the fork #217 handoff README (areas 2 to 4).
- Slide 8: fork PR states from `gh` on 20 Nov, 12:50 AM PST; the Pi 2 B figure is from the fork #222 body.
- Slide 9: the fork #217 handoff (areas 2, 4, 5, 6). The adb listener on TCP 5555 is from a read-only
  device check on 19 Nov that is not committed, because it carries network identifiers.

## Left out for lack of data

- A Fire TV run of fork #222 (P-256 on `ecp_nistz256-armv4`): it was measured only on a Raspberry Pi 2 B.
- rustls handshake rows: the bench panics on the device.
- Thermals and CPU busy share: not readable from the `shell` uid.

## PR links on the slides

Each slide tied to a PR has a muted `PRs:` line at the top of its footer. `fork #N` links to
https://github.com/steelhead99x/rind/pull/N. No upstream kixel-ai/rind PR is involved. Data links
point at `results/firetv-aftsss/` on this repository's main branch.

## Opening it

Open `index.html` in a browser. Navigate with ← / → or click either half of the slide. `#N` opens slide N.
`screenshots/` holds a contact sheet and slides 1, 4 and 8 at 1440 px wide.
