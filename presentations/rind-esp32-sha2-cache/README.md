<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind on the ESP32-S3: fitting SHA-2 into a 16 KiB cache

`index.html` is a self-contained HTML deck (no network access, no external libraries), 9 slides at 1280×720,
with the CSS and navigation of `presentations/rind-testing`. It covers kixel-ai/rind#793, merged
19 November 2026 at 9:51 AM PST, a port of steelhead99x/rind#69. All times are America/Los_Angeles (PST).

1. Title
2. The problem in numbers: about 47 KiB unrolled SHA-512, about 12 KiB SHA-256, a 16 KiB cache, about 33× slower than ring
3. Evidence that it is the cache: the 32 KB cache check (XChaCha20-Poly1305 32 B, 7.50× to 1.10× RustCrypto)
4. The fix: rolled SHA-512 and HChaCha20 on 32-bit microcontrollers, SHA-256 in IRAM on ESP-IDF Xtensa, and the layout CI check
5. Board result, fork runs: SHA-512 0.033×, SHA-256 0.315× (32 B) to 0.741× (16 KB) of main
6. Side check on i686: main, PR as opened, merged head (why the rolled SHA-512 is limited to microcontrollers)
7. Raw board data on rind-results main for fork #94 (SHA-512 in IRAM, 0.911× to 0.918× of fork main), a different design
8. What it costs: +4,516 B IRAM, −4,608 B heap, no formal proof on these targets; what is next
9. Sources

## Which numbers come from where

- PR body (kixel-ai/rind#793 merged body): the problem sizes, the i686 table, the IRAM and heap costs (the body
  says these are the fork's measurement), and the proof tradeoff.
- Fork runs, not the merged head: the board ratios 0.315× to 0.741× and 0.033× (fork #98 comment 6883757723, image
  `3aaa3c6a`, cited by #793), and the 12 Nov run at fork branch `6aba8055` (3.34 ms to 111.15 µs at 64 B), which
  is in the rind-hardware-today deck and not on rind-results main. #793's body says the ESP toolchain and the
  board were not run for the upstream PR. The slides label these as fork runs.
- rind-results main, raw logs: `results/diag/esp32s3-icache32-80mhz-98568332/` and
  `results/diag/esp32s3-pr94-pr87-pr85-2026-11-18/` (fork #94, added by rind-results #79). Fork #94 is open and is
  not the #793 design; the slide says so.

## Left out for lack of data

- A board run of the merged #793 head. A chart of upstream #793 against main needs one, built with the ESP
  toolchain at the merged commit; rind-results has no `results/esp32s3` tree at a post-#793 rind commit.
- The fork #69 board run in the rind-hardware-today deck reports +4,808 B IRAM; this deck uses #793's stated +4,516 B.

## PR links on the slides

Each slide tied to a PR has a muted `PRs:` line at the top of its footer. `rind #N` links to
https://github.com/kixel-ai/rind/pull/N (upstream), `fork #N` to https://github.com/steelhead99x/rind/pull/N,
and `rind-results #N` to https://github.com/steelhead99x/rind-results/pull/N. Fork and upstream numbers are
different PRs and are never mixed.

## Opening it

Open `index.html` in a browser. Navigate with ← / → or click either half of the slide. `#N` opens slide N.
