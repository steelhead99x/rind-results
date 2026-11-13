# `rp2040`: builds and is ready for hardware

> **Update (2026-11-12):** rind-hardware (then rind-bench) now builds every bench image for
> `thumbv6m-none-eabi`. `patches/0003` gives rind `no_std` on `target_os =
> "none"` with the M-profile portable backend. `patches/0005` makes
> `ReuseCache` work without CAS: on targets that do not set
> `target_has_atomic = "ptr"`, the same state machine runs in a `Cell`,
> with no new dependency, matching rind's existing "one execution context
> on bare metal" assumption. A critical-section or portable-atomic crate
> would have added a dependency without changing that assumption, so it
> was not used. The images have not been run on a board (none is
> attached); known-answer checks are in the firmware and will run on the
> first flash.

- Board needed: a Raspberry Pi Pico, Pico W, or another RP2040 board with
  a 12 MHz crystal (the official Pico default) and W25Q080 or compatible
  QSPI flash (embassy-rp `boot2-w25q080`). Hold BOOTSEL, plug in over USB,
  copy `rp2040/images/<bench>.uf2`, open the CDC-ACM port, and send
  `rind-bench:go\n`. See [rind-hardware/rp2040/README.md](https://github.com/steelhead99x/rind-hardware/blob/main/rp2040/README.md)
  (private repo).
- Chip: RP2040, Cortex-M0+ (`thumbv6m-none-eabi`), 125 MHz, 264 KB RAM
  (256 KB main SRAM), 2 MiB XIP flash.
- Timing: SysTick at the processor clock, 1 count = 1 cycle. The hardware
  counter is 24 bits (wraps every 134.2 ms at 125 MHz); a software
  extension gives a 32-bit count wrapping every 34.4 s. The 1 MHz TIMER is
  an independent boot cross-check only (1 µs is too coarse for short rows).
  Cortex-M0+ has no DWT cycle counter.
- Libraries in the images: rind, RustCrypto, dalek, libcrux. **ring 0.17.14
  is left out**: its Arm assembly uses NEON (`vpush`) and does not assemble
  for ARMv6-M. aws-lc-rs needs an OS.
- RNG: the RP2040 has no TRNG. `getrandom` is a seeded SplitMix64, not a
  source of entropy.
- Image hashes and source: [images.json](images.json) and
  [images.sha256](images.sha256) next to this note. Built from rind-bench
  `f656f95` (2026-11-12 16:35 PST; pre-public history, published squashed as rind-hardware), rind `98568332`, patches 0001–0005.
  The images have not been flashed or run.
- There is no official QEMU machine for the RP2040 with enough RAM to run
  these images, so only a build is published.
