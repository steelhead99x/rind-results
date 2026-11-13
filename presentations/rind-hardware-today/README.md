# rind on the boards, 12 November 2026: slide deck

`index.html` is a self-contained HTML deck (no network access, no external libraries), 12 slides at 1280×720,
with the CSS of `presentations/rind-rp2040`. It covers what changed on the microcontroller benchmarks on
12 November 2026 and a timeline built from git history. All times are America/Los_Angeles (PST).

1. Title
2. Summary of today's changes and their status
3. Poly1305 32-bit patch A/B on the ESP32-S3 and nRF52840, with the libcrux layout-shift flag
4. ESP32-S3 relinks at three alignments (pad0, pad1031, pad1039) and their controls
5. The 32 KB instruction-cache diagnostics at 80 MHz (a diagnostic build, not the published setup)
6. The flash-clock fix (40 → 80 MHz) and the nRF52840 watchdog fix
7. The RP2040 port and the move from rind-bench to rind-hardware
8. Hosts: Orin and today's Pi 5 lane are in progress (not on main)
9. Timeline, overnight to midday
10. Timeline, afternoon to evening
11. Caveats: noise, layout, and what is not on main
12. Sources

## Sources

Every number is from a file on `steelhead99x/rind-results` main at `95300f2` or `steelhead99x/rind-hardware`
main at `7a14b38`; each slide names its files in the footer. Work that is only on branches (the RP2040
first-run fixes on rind-hardware, the Pi 5 lane on rind-results) is named but its numbers are left out.

## Opening it

Open `index.html` in a browser. Navigate with ← / → (also PageUp/PageDown, Space, Home/End) or click the right
or left half of the slide. `#N` in the URL opens slide N.

## Updating it

When the Orin or Pi 5 lane results reach main, replace slide 8 and add their commits to the timeline.
