# rind on the RP2040: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 25 slides at 1280×720.
It covers:

1. Title
2. The boards: an earlier RP2040 left alone after a backup; the Raspberry Pi Pico and Pico W backed up before any write; the CatWAN USBStick2040 still waiting on a fresh backup (it needs BOOTSEL)
3. The five limits: no OS, no compare-and-swap (CAS), a 32×32→32 multiplier, no hardware RNG, 264 KiB of SRAM
4. Where rind started: 1672 errors at a4759c48 (see the note on that label below), plus the 2026-11-12 upstream count (1,739 → 0)
5. Patches 0003 (no_std) and 0005 (ReuseCache without CAS), and the placeholder getrandom backend
6. Where it stands: the 12 SysTick-fixed images run on the Pico, 0 known-answer failures, old-base variants, and the new stack on upstream main, whose 5-round medians are on slide 11
7. The on-device run procedure (backup, flash, check, time, rounds, restore) and the run setup
8. The SysTick wrap bug the first run found (COUNTFLAG never cleared, +2^24 cycles) and its fix; rind-hardware main still lacks that fix
9. How it was measured: SysTick cycles, median of per-round medians, fairness rules
10. Live results: baseline for all 12 benches from the SysTick-fixed images, labeled **old base**, 2 or 3 of 5 planned rounds (snapshot 2026-11-12 18:35 PST)
11. New base, **measured, 5 rounds**: unmodified rind no-std/thumbv6m-alloc @ 3dc5634b, 22 rind rows; Pico and Pico W agree within 0.1%; all known-answer tests pass
12. Finding 1, SHA-512: ~2,200 cycles/byte in every library (unrolled code > 16 KB XIP cache) and the rolled variant; the code-placement side effect, with 4-round old-base medians
13. Finding 3, XIP placement: why HACL `sha256_update` (9,940 B) conflicts in the 16 KiB 2-way cache, the `sram-hot` model, and SRAM cost
14. Finding 3, fixed: fork branch rp2040/armv6m-no-std @ bcce5f60 (pushed; 9f0e1e75 fixes the bare-metal debug build, bcce5f60 puts SHA-256's short-message path in one flash section). **Measured, 2–3 interleaved rounds**: sha256/32B 0.737×, HMAC/32B 0.876×, HKDF expand 0.814×, plus branch-wide ratios; before the fix 1.39× / 1.57× / 1.44× slower (**1 round**)
15. Finding 2, AES-GCM: GHASH's `__aeabi_lmul` calls and the muls-only GHASH (**old base, measured**, 3 rounds)
16. Finding 4, fixsliced AES: pure Rust (A) and generated Thumb-1 assembly (B). B **measured, 1 round**: AES-128-GCM 16 KiB 6.33M cycles (0.383× base) vs RustCrypto 7.16M; known-answer tests pass on both boards; local only. CTR table and A remain model estimates
17. Static scan of the M0+ images: code size and `__aeabi_lmul` counts per primitive, constant-time findings, ranked candidates
18. The constant-time P-256 stack on 3dc5634b / c333c59e: 0 rind lmul; patch 0008 alone is not constant-time (`cmp`/`blo` carries) and 178f7966 fixes that
19. P-256 emulator estimates: scalar inverse 2,108,875 → 203,983 cycles, field inverse 897,579 → ~187,000, point operations 18 to 23% faster. Not hardware timings
20. The P-256 constant-time lesson: on the board (**1 round**) ECDH 21.80M → 33.16M and verify 20.16M → 30.24M, sign 8.58M → 6.17M, because hot code grew from ~9.3 KB to ~15.3 KB and conflicts in the XIP cache, which the emulator does not model. Hot code in SRAM (**1 round**): 16.24M / 13.95M / 4.35M. Flash-pack 0d143bcf (local): **model estimate only**, not timed
21. The upstreamable std / alloc feature split for no_std (thumbv6m 1,739 → 0 errors, identical desktop codegen)
22. Which other Rust crypto libraries compile for this and nearby targets (compile only)
23. Other chips with the same limits, including the EFR32MG24
24. Status and sign-off (2026-11-13): branch status, what has not run on boards yet, and process lessons. No upstream pull requests yet; draft fork PRs #84 and #87 are on steelhead99x/rind only
25. Sources

Every number is labeled as measured on the board (with its round count) or as a model or emulator estimate. Old-base measured rows are labeled "old base". One-round board figures are labeled "1 round".

## Opening it

- Open `index.html` in any modern browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End), or click
  the right or left half of the slide. `#N` in the URL opens slide N.
- PDF: print from Chrome or Edge with "Save as PDF", margins "None", and background graphics on. Each slide prints as one 1280×720 page.
- The navigation hint sits below the slide; the slide is scaled to leave room for it.

## Updating it

Every number and table is in the `DATA` block (`<script id="deck-data">`) at the top of `index.html`. Each part of
`DATA` names the file it came from. The CSS and navigation script are reused from `presentations/rind-spake2plus`.

### Updating the live results (slide 10)

Edit only `DATA.live`:

1. Set `status` to `"partial"` while rounds are still running, or `"complete"` when all are in. With
   `"pending"` the slide shows the pending banner and no numbers. `title`, if set, replaces the generated slide title;
   update or remove it when the numbers change. `base` / `baseNote` say whether the rows are old-base or new-base.
2. Fill `asOf` (time with zone), `source` (branch, file and commit in this repository where the raw results are
   published), and `placement` (`"XIP flash"` or `"SRAM"`, where the hot code ran).
3. For each row, set `op` (for example `"seal 1 KiB"`), `unit`, `rind`, `best`, `bestLib` and `rounds`:
   - `unit: "cycles"` for SysTick-timed rows (every row so far), `unit: "us"` for rows timed with the 1 MHz timer.
   - `rind` and `best` are the median of per-round medians from `summarize.py` (see slide 9). `best` is the fastest other
     library from the builds listed in `baremetal/rp2040/COMPETITOR-BUILDS.md`; ring is excluded.
   - `rounds` is "valid rounds / rounds counted" for that row.
   - Leave a value `null` and that cell shows "pending". Set `noOther: true` when no other library is in the image.
4. The table shows cycles (or µs), and the chart draws rind ÷ fastest other for every row that has both values.
5. Add the results file to `DATA.sources` so it appears on the sources slide.

Only publish numbers that passed the on-device known-answer checks and were measured as slide 10 describes.
Do not publish a median until its planned rounds are complete. Label old-base rows as old base.

### The 2026-11-12 and 2026-11-13 slides (6–21, 24)

Their numbers are in the `DATA` blocks `snap`, `pico`, `picoW`, `systick`, `sha512`, `layout`, `ghash`, `m0`, `p256`,
`place`, `aes`, `nostd`, `status` and `live`.

- Slide 10 is a snapshot from 2026-11-12 18:35 PST, **old base**, while rounds were still running: the aes_gcm, sha2, hmac, ed25519, hkdf
  and sha3 rows have 3 of 5 planned rounds, and the other six rows have 2.
- The layout table (slides 12–13) uses 4-round old-base medians from the bench lane STATUS at 20:30 PST.
- Slides 13, 16 (A and the CTR table), 19 and the flash-pack column of slide 20 are model or emulator estimates. Slides 14, 16 (B) and 20 carry 2026-11-13 board figures, each labeled with its round count.
  Replace them when completed board medians land, and keep the estimate figures labeled as estimates until then.

Estimates (the SHA-512 refetch cost, the X25519 lmul count, the sram-hot model, the fixsliced AES model, the P-256 Unicorn
cycle counts) are labeled as estimates on the slides; keep them labeled.

## Sources

All in https://github.com/steelhead99x/rind-results, branch `main`:

- `baremetal/rp2040/README.md`, `baremetal/rp2040/COMPETITOR-BUILDS.md`, `baremetal/rp2040/TARGET-SURVEY.md`,
  `baremetal/rp2040/builds/README.md`, `baremetal/mg24/README.md`
- `hosts/rp2040/DEVICE.md`, `hosts/rp2040/FEASIBILITY.md`, `hosts/rp2040/NOTES.md`, `hosts/rp2040/images.json`
- commits 2cb88c3, f4e0b1d, cb264f8

The 2026-11-12 on-device work is not yet published in this repository. Slides 2, 6–20 and 24 cite these working notes by name
(no local paths, board serials or hostnames are in the deck):

- RP2040 bench lane: STATUS notes, round logs, raw run JSON summarized with `summarize.py` (median of per-round
  medians; a round more than 3% off is marked invalid), and the drafted SHA-512 and GHASH commit messages. Only completed medians are used.
- Placement lane: `REPORT.md` and `BUILD.md`
- AES lane: `BUILD.md` and `STATUS.md`
- P-256 lane: `REPORT.md`, `BUILD-P256CT.md` and `STATUS.md`
- M0+ static-analysis lane: `REPORT.md` and `STATUS.md` (llvm-nm / llvm-objdump on the SysTick-fixed images, rustc 1.98.1)
- no_std lane: `STATUS.md` and its gates log
- X25519 / Ed25519 / Poly1305 lane: `STATUS.md` (work in progress)

The fairness rules on slide 9 come from the rind-bench team's measurement plan as of 2026-11-12. The SysTick
extension to 32 bits and the 1 MHz timer as a startup cross-check come from `hosts/rp2040/NOTES.md` at 2cb88c3.
Draft fork PRs steelhead99x/rind#84 (no_std on bare metal) and #87 (32-bit Poly1305, including thumbv6m mul16)
were opened on 13 November 2026; neither has been sent to upstream kixel-ai/rind (formerly moq-dev/rind).
Slide 24 lists those drafts and the unpushed fixsliced AES, P-256 CT and flash-pack work. #87 has been
measured on other boards; RP2040 board timing for it is still pending.

The a4759c48 label: `hosts/rp2040/FEASIBILITY.md` calls it a rind commit; `baremetal/rp2040/COMPETITOR-BUILDS.md` takes its
versions (including rind 0.3.2) from `rind-bench/Cargo.toml` at a4759c48. The deck cites both.
