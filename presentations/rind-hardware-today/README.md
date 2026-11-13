# rind on the boards, 12 November 2026: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 17 slides at 1280×720,
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
16. Caveats: noise, layout, and work that is not on main, including these evening figures
17. Sources, with a separate row for the unpublished raw jsonl

## Sources

Published figures (slides 2–10) are from a file on `steelhead99x/rind-results` main at `95300f2` or
`steelhead99x/rind-hardware` main at `7a14b38`; each of those slides names its files in the footer.
The Jetson Orin Nano set is on rind-results main. The evening I-cache figures (slides 11–15) are an
unpublished perf-branch run on the ESP32-S3 at steelhead99x/rind `6aba8055` plus five patches
(`61570282`, `9dbb1ed2`, `e54b7894`, `a60ee4b8`, `fa0ab77c`). They are labelled as unpublished
on the slides and are not on rind-results main. Other work that is only on branches (the RP2040
SysTick and SHA-512/GHASH fixes on rind-hardware, the Pi 5 lane on rind-results) is named but
its unpublished numbers are left out.

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End) or click the right
or left half of the slide. `#N` in the URL opens slide N.

## Updating it

When the Pi 5 lane results reach main, replace the remaining in-progress row on slide 8 and add those commits to the timeline.
