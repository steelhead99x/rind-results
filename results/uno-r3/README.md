<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Arduino Uno R3 (ATmega328P @ 16 MHz)

A `no_std` bench of rind's portable/generic SHA-256, rolled SHA-512, ChaCha20, and 64-bit-limb Poly1305, taken from [`c08c91ea`](https://github.com/steelhead99x/rind/commit/c08c91ea) (`origin/rp2040/armv6m-no-std`), run on an Arduino Uno R3 (ATmega328P at 16 MHz, 32 KB flash, 2 KB SRAM). Captured 2026-11-13, 1:54 PM PT (America/Los_Angeles).

This is **not** the full rind crate, and it is **not** the comparable set at `d6ca9a75`. rind's TARGET-SURVEY lists AVR as out of scope: 8-bit, 16-bit pointers, and too little RAM. The firmware in [firmware/](firmware/) is a close honest equivalent of those generic primitives, not a third-party clone. On the AVR, SHA-512 compression uses 32-bit limbs of rind's rolled loop so the 80-round body fits in SRAM. Host known-answer tests and the on-device hashlib digests agree with the 64-bit reference.

Numbers: [RESULTS.md](RESULTS.md), [medians.md](medians.md), [medians.json](medians.json). The five runs: [RUNS.md](RUNS.md). Raw serial: [raw/capture.log](raw/capture.log).

## What does not fit

X25519, P-256, Ed25519, ML-KEM, AES-GCM, and SHA-3 were not run. They do not fit in 2 KB SRAM and 32 KB flash (the application region is 32,256 bytes after the 512-byte bootloader). The image that did run is 26,880 bytes of flash and 1,013 bytes of SRAM (data+bss), leaving 1,035 bytes of stack.

## Why the ChaCha20 timings were dropped

The RFC 8439 114-byte ChaCha20 known-answer test passed on the board. The length-bench path (a streaming checksum of pattern bytes) did not produce a stable checksum across the five runs, and some cycle counts wrapped near 2<sup>32</sup>. The non-wrap 1,350 B readings were also *lower* than the 64 B ones, which is not a believable length scaling. Those timed rows are omitted, not guessed. A composed "ChaCha20 then Poly1305" 1,350 B row is also omitted: it is not the RFC 8439 AEAD, and all five of its cycle counts wrapped. The raw lines stay in [raw/capture.log](raw/capture.log).

Poly1305 length benches are published: the 16 B and 1,350 B tags matched an RFC 8439 host check, and the five cycle counts were identical.

## Build and run

The published MG24 and ESP32-S3 sets keep their harness in [rind-hardware](https://github.com/steelhead99x/rind-hardware). This AVR port is not in that tree, so the firmware and a small capture script live here.

Needs a nightly Rust toolchain with `rust-src` (see [firmware/rust-toolchain.toml](firmware/rust-toolchain.toml)), and `avr-gcc` that can target ATmega328P. `avrdude` flashes the image. The Arduino AVR core's `uno` board recipe is a typical source of that toolchain.

```text
cd firmware
export AVR_GCC=avr-gcc          # or the full path to an avr-gcc that supports atmega328p
cargo +nightly build --release -Z build-std=core --target avr-none
avr-objcopy -O ihex -R .eeprom target/avr-none/release/rind-uno-r3-bench.elf rind-uno-r3-bench.hex
avrdude -c arduino -p atmega328p -b 115200 -P <serial-device> -U flash:w:rind-uno-r3-bench.hex:i
```

Then capture, with the serial device passed explicitly (nothing here hard-codes one):

```text
python3 host/capture.py --port <serial-device> --seconds 900 --out-dir .
```

The board speaks 115200 8N1. Opening the port with DTR pulses the 328P into a reset, which starts a clean run. Five primed length benches plus known-answer tests finish in under a minute. `host/host_kat.py` prints the expected SHA-256 and SHA-512 pattern digests.

Timer1 runs with no prescale (1 tick = 1 cycle at 16 MHz). µs = cycles / 16 exactly.

## Comparison

[RESULTS.md](RESULTS.md) compares only sizes this repository already publishes for the EFR32MG24 (`results/efr32mg24/`, rind `98568332`, 78 MHz) and the ESP32-S3 comparable set (`results/esp32s3/d6ca9a756d3d1e71489e61e44c04f33c3c893589/`, rind `d6ca9a75`, 240 MHz). This repository does not yet have a `results/rp2040/` timing tree on main, so RP2040 numbers are not included. Read the SHA-512 caveat in RESULTS.md before treating those rows as the same code on different chips.
