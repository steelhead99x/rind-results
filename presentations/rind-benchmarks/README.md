<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind benchmark results

A 14-slide PowerPoint of the published host and board results in this
repo, including what the red and orange cells on the Apple hosts mean
(DIT) and how `Dit::hold()` changes the comparison. Open
`rind-results-deck.pptx` in PowerPoint, Keynote, or LibreOffice.
JPEG renders of each slide are in `renders/`; `renders/contact.jpg`
is a contact sheet of all 14.

## Slides

1. Title
2. What rind is, and what was measured
3. Hosts and method
4. How to read the speed chart
5. Headline wins on xeon-vm
6. What DIT is, and why it matters
7. What rind does with DIT, and what it costs
8. The Apple DIT tradeoff on the chart
9. Microcontrollers
10. Corrections along the way
11. Build cost
12. Open gaps and next steps
13. Since this deck: SHA-512 and Intel's timing list (added 2026-11-17)
14. Newer sets on main since these four hosts (added 2026-11-17)

## Sources

Every number comes from files on `steelhead99x/rind-results` main. Each
slide's speaker notes name its source files. The DIT per-call cost is
given as 30 to 55 ns on the M4 and 45 to 60 ns on the M1 and M4, each
labelled with its source, because those figures differ across files.

## 17 November update

- Slide 9's ESP32-S3 gap line adds that a fork fix (steelhead99x/rind#69, merged 16 Nov) cuts rind's SHA-512
  time to 0.033× of before, from the board run in #69 comment 6936378505. It is marked as not published yet.
- Slide 13 is new: kixel-ai/rind#821 stops using SHA-512 instructions that Intel's data-independent-timing list
  leaves out. Its cost on a Core Ultra 9 275HX (commit a674bc6e): 2.0 to 2.2× the old SHA-512 time at 32 and 64 B,
  4.4 to 4.7× from 1200 B, still 0.81 to 0.86× aws-lc-rs's time. #821 is open.
- Slide 14 is new: the sets published on main after the four b6651423 hosts (Jetson Orin Nano, Core Ultra 9
  275HX, Pi 5 at 1.5 GHz, Pi 2, the ESP32-S3 d6ca9a75 set, and the Uno R3), each with the "rind fastest on N of 68"
  count from its own RESULTS.md.
- No published number on slides 1 to 12 changed. Those slides describe the four b6651423 hosts, and those files
  did not change.

## Rebuilding it

```
python3 build.py
```

Needs `python-pptx` and Pillow, and the PNGs in `img/`. The script
writes `rind-results-deck.pptx` beside itself. The JPEG renders are a
snapshot of that deck; rebuilding the `.pptx` does not refresh them.
