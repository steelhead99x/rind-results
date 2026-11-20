<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind benchmark results

A 14-slide PowerPoint of the published host and board results in this
repo, including what the red and orange cells on the Apple hosts mean
(DIT) and how `Dit::hold()` changes the comparison. Open
`rind-results-deck.pptx` in PowerPoint, Keynote, or LibreOffice.
JPEG renders of each slide are in `renders/`; `renders/contact.jpg`
is a contact sheet of all 14. Related pull requests are linked in
the slide footers.

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
12. The work so far
13. Benchmarking and verifying at scale
14. Open gaps and next steps

## Sources

Every number comes from files on `steelhead99x/rind-results` main. Each
slide's speaker notes name its source files. The DIT per-call cost is
given as 30 to 55 ns on the M4 and 45 to 60 ns on the M1 and M4, each
labelled with its source, because those figures differ across files.

## Rebuilding it

```
python3 build.py
```

Needs `python-pptx` and Pillow, and the PNGs in `img/`. The script
writes `rind-results-deck.pptx` beside itself. The JPEG renders are a
snapshot of that deck; rebuilding the `.pptx` does not refresh them.

`build.py` still builds the original 12-slide deck. Slides 12 and 13,
and the PR links in the footers, were edited directly in the `.pptx`
after `build.py` ran. Rerunning `build.py` drops those edits.
