# rind on the boards, 12 November 2026: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 19 slides at 1280×720,
with the CSS of `presentations/rind-rp2040`. It covers what changed on the microcontroller benchmarks on
12 November 2026 and a timeline built from git history. All times are America/Los_Angeles (PST).

Slides after the published Poly1305 A/B and the original timeline add an unpublished
ESP32-S3 perf-branch run at steelhead99x/rind `6aba8055` plus five patches: the I-cache
root cause, SHA-512, HMAC/HKDF and the SHA-256 64 B spread, ChaCha20-Poly1305 and
XChaCha, and a late-evening timeline. Those figures are not on rind-results main.

1. Title
2. Summary of today's changes and their status, including a row for the unpublished evening I-cache run
3. Poly1305 32-bit patch A/B on the ESP32-S3 and nRF52840, with the libcrux layout-shift flag (published, rind 98568332)
4. ESP32-S3 relinks at three alignments (pad0, pad1031, pad1039) and their controls
5. The 32 KB instruction-cache diagnostics at 80 MHz (a diagnostic build, not the published setup)
6. The flash-clock fix (40 → 80 MHz) and the nRF52840 watchdog fix
7. The RP2040 port and the move from rind-bench to rind-hardware
8. Hosts: Jetson Orin Nano results are on main; today's Pi 5 lane is still on branches (not on main)
9. Timeline, overnight to midday
10. Timeline, afternoon to evening
11. Root cause of the remaining ESP32-S3 gaps: I-cache thrash from unrolled HACL* code (unpublished)
12. SHA-512: 32.95× ring → 1.08× after a rolled 1,290-byte compress (unpublished)
13. HMAC-SHA256, HKDF-SHA256, and the SHA-256 64 B 25.7% spread (unpublished)
14. ChaCha20-Poly1305 and XChaCha: 32-bit-limb Poly1305 and rolled HChaCha (unpublished)
15. Timeline, late evening on the perf branch
16. Update, 17 Nov: the fix that merged on the fork (steelhead99x/rind#69) and its board run against upstream main
17. Update, 17 Nov, status refreshed 19 Nov: where the ESP32 and Xtensa work stands (#69 merged on the fork,
    kixel-ai/rind#793 merged upstream 19 Nov, steelhead99x/rind#85 closed, #94 open with raw logs on main from rind-results #79)
18. Caveats: noise, layout, and work that is not on main, including these evening figures
19. Sources, with a separate row for the unpublished raw jsonl and one for the 17 Nov update

## Sources

Published figures (slides 2–10) are from a file on `steelhead99x/rind-results` main at `95300f2` or
`steelhead99x/rind-hardware` main at `7a14b38`; each of those slides names its files in the footer.
The Jetson Orin Nano set is on rind-results main. The evening I-cache figures (slides 11–15) are an
unpublished perf-branch run on the ESP32-S3 at steelhead99x/rind `6aba8055` plus five patches
(`61570282`, `9dbb1ed2`, `e54b7894`, `a60ee4b8`, `fa0ab77c`). They are labelled as unpublished
on the slides and are not on rind-results main. Other work that is only on branches (the RP2040
SysTick and SHA-512/GHASH fixes on rind-hardware, the Pi 5 lane on rind-results) is named but
its unpublished numbers are left out.

## 17 November update

Slides 2, 8 and 18 now say the 12 Nov Pi 5 lane is on rind-results main (`hosts/pi5/ab-narrow-mul/`, and a full
1.5 GHz set at rind a6577cb4 in `results/pi5-nofan-1p5ghz/`, rind fastest on 45 of 68 shared operations). Slides 16
and 17 are new. Their ESP32-S3 figures come from steelhead99x/rind#69 comment 6936378505, the maintainer's board run
of #69 head 504049d4 against upstream main 19f94a07 (3 runs per image, 0 known-answer failures). That run is in a
PR comment, not on rind-results main, and the slides say so.

Left out for lack of data: steelhead99x/rind#94's SHA-512-in-IRAM figures (0.911× to 0.918× main) have no raw
logs, so slide 17 lists the PR as "not verified" and uses none of its numbers.

## 19 November status refresh

Slide 17 only: kixel-ai/rind#793 merged upstream on 19 Nov at 9:51 AM PST (its board numbers are still the fork's;
the merged head was not flashed), steelhead99x/rind#85 is closed, and #94's raw board logs are now on rind-results
main in `results/diag/esp32s3-pr94-pr87-pr85-2026-11-18/` (rind-results #79), so its row now cites them. Slides 16
and 17 carry a `PRs:` footer line (`fork #N` is steelhead99x/rind, `rind #N` is kixel-ai/rind, `rind-results #N`
is steelhead99x/rind-results). The new rind-esp32-sha2-cache deck covers #793 in full.

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End) or click the right
or left half of the slide. `#N` in the URL opens slide N.

## Updating it

When the Pi 5 lane results reach main, replace the remaining in-progress row on slide 8 and add those commits to the timeline.
