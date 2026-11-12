<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: three layouts at rind 98568332

The published set (`results/raw/`, pad0) and two relinked builds of the same source, rind-bench's `cargo xtask build-images esp --pad 1031` and `--pad 2053` (a never-executed static of that many bytes in the firmware's own `.text`, which moves the code linked after it), for the seven benches whose short-input rows the 99b509a relink moved (`../layout-relink-2026-11-12/`). Three runs of each, flash at 80 MHz, 2026-11-12 12:01-13:06 PST, all in one sequence on the same board.

- [relink-short.md](relink-short.md): each short-input row (under 1 KB, or no size), rind over the fastest other library in each layout, the range, and the ratio from the medians over the three layouts. This is the multi-relink median for those rows.
- [relink.md](relink.md): the same for every operation of the seven benches.
- [compare-pad1031.md](compare-pad1031.md), [compare-pad2053.md](compare-pad2053.md): every row, pad0 against each.
- `pad1031/`, `pad2053/`: the raw runs.

Most short-input rows move by under 0.1. SHA-256 at 64B moves most (2.36x, 3.55x and 2.78x libcrux's time); the AES-GCM 32B rows are 0.53x-0.66x in the published layout and 0.58x-0.77x in both others. Numbers taken from one layout carry that much uncertainty on this board.
