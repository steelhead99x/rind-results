# RP2040

RP2040 rev B2, dual Cortex-M0+ at 125 MHz, 2048 KiB flash, 264 KiB SRAM. Stock firmware was an Arduino (Mbed OS) sketch, about 92 KiB.

rind does not build for `thumbv6m-none-eabi` (no `std`). Portable-backend dependencies do. The flash was never written.

## Backup

Full flash (`0x10000000`–`0x10200000`) was read three times with picotool 2.3.1; all three copies matched.

sha256 `a347fa9098c487728ca29d380311143f1849a97032759e9a9ff5b423a3515c86`

The image is not in this tree. To restore it later:

```
picotool load backup.bin -t bin -o 0x10000000 -v -x
```

BOOTSEL can be entered with a 1200-baud touch on the CDC-ACM port; exit with `picotool reboot`.

## What's here

- `builds/` — cargo workspace that compiles rind and comparison crates for this target and nearby ones. Run `./build-matrix.sh thumbv6m-none-eabi`.
- `COMPETITOR-BUILDS.md` — results (compile only, 2026-11-12, rustc 1.99.1).
- `TARGET-SURVEY.md` — other MCUs with the same limits, CI targets, and timing caveats.
