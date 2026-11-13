<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind generic primitives on an Arduino Uno R3

A `no_std` port of rind's portable/generic SHA-256, rolled SHA-512, ChaCha20, and 64-bit-limb Poly1305 from `c08c91ea`, run on an Arduino Uno R3 (ATmega328P at 16 MHz) and timed with the 328P's Timer1 (no prescale, 1 tick = 1 CPU cycle). This is not the full rind crate, and it is not the comparable set at `d6ca9a75`. See [README.md](README.md).

- rind source: `c08c91ea` (`origin/rp2040/armv6m-no-std`), generic primitives only. On the AVR, SHA-512 compression uses 32-bit limbs of that rolled loop.
- OS: none (bare metal, `no_std`); output over the 16U2 USB-serial at 115200 8N1.
- Image: 26,880 / 32,256 flash bytes, 1,013 / 2,048 SRAM (data+bss).
- Runs: 5 of 5 completed, 2026-11-13 21:54:14 to 21:54:48 UTC (1:54 PM PT, America/Los_Angeles). Known-answer tests passed on every run: SHA-256 empty and "abc", SHA-512 "abc", ChaCha20 RFC 8439, Poly1305 RFC 8439.
- Each length is primed once, then timed. A published cell is the median of the five timed runs. µs = cycles / 16 exactly.
- Raw serial: [raw/capture.log](raw/capture.log). Per-run cycles: [RUNS.md](RUNS.md). JSON: [medians.json](medians.json).

## Headline medians

| Operation | Uno R3 cycles (n=5) | range | Uno R3 µs |
|---|---:|---:|---:|
| sha256/32B | 61,048 | 61,048–61,048 | 3,815 |
| sha256/1350B | 1,266,969 | 1,266,969–1,267,012 | 79,185 |
| sha256/16KB | 14,771,336 | 14,771,336–14,771,379 | 923,208 |
| sha512/32B | 216,452 | 216,452–216,452 | 13,528 |
| sha512/1350B | 2,317,472 | 2,317,472–2,317,515 | 144,842 |
| sha512/16KB | 27,108,699 | 27,108,699–27,108,699 | 1,694,293 |
| poly1305/16B | 16,509 | 16,509–16,509 | 1,031 |
| poly1305/1350B | 1,249,725 | 1,249,725–1,249,725 | 78,107 |

SHA-256, SHA-512, and Poly1305 digests matched a host hashlib / RFC 8439 Poly1305 check on every run. ChaCha20 pattern-bench timings are omitted; see [README.md](README.md). X25519, P-256, Ed25519, ML-KEM, AES-GCM, and SHA-3 do not fit and were skipped.

A Timer1 sanity check (`nop1000`) measured 7,012 cycles (loop overhead around a thousand `nop`s, not a 1,000-cycle walk). That check is not a benchmark row.

## Comparison with published EFR32MG24 and ESP32-S3 rind rows

Only sizes this repository already publishes are compared. Dashes are sizes the other trees do not publish (those boards report 64 B, not 32 B). RP2040 columns are omitted: this repository does not yet have a `results/rp2040/` timing tree on main.

Sources:

- EFR32MG24, Seeed XIAO MG24, Cortex-M33F at 78 MHz: [`results/efr32mg24/RUNS.md`](../efr32mg24/RUNS.md), rind `98568332`.
- ESP32-S3 at 240 MHz, flash 80 MHz, comparable set: [`results/esp32s3/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RUNS.md`](../esp32s3/d6ca9a756d3d1e71489e61e44c04f33c3c893589/RUNS.md), rind `d6ca9a75`.

| operation | Uno R3 @16 MHz cycles | EFR32MG24 rind @78 MHz cycles | ESP32-S3 rind @240 MHz cycles |
|---|---:|---:|---:|
| sha256/32B | 61,048 | — | — |
| sha256/1350B | 1,266,969 | 73,798 | 127,866 |
| sha256/16KB | 14,771,336 | 830,578 | 1,305,093 |
| sha512/32B | 216,452 | — | — |
| sha512/1350B | 2,317,472 | 190,122 | 8,421,487 |
| sha512/16KB | 27,108,699 | 2,203,643 | 98,198,722 |
| poly1305/16B | 16,509 | — | — |
| poly1305/1350B | 1,249,725 | — | — |

Wall time at each board's clock (same cycle counts; µs = cycles / MHz):

| operation | Uno R3 @16 MHz | EFR32MG24 @78 MHz | ESP32-S3 @240 MHz |
|---|---:|---:|---:|
| sha256/32B | 3,815 µs | — | — |
| sha256/1350B | 79,185 µs | 945.9 µs | 532.8 µs |
| sha256/16KB | 923,208 µs | 10,648.4 µs | 5,437.9 µs |
| sha512/32B | 13,528 µs | — | — |
| sha512/1350B | 144,842 µs | 2,437.5 µs | 35,089.5 µs |
| sha512/16KB | 1,694,293 µs | 28,251.8 µs | 409,161.3 µs |
| poly1305/16B | 1,031 µs | — | — |
| poly1305/1350B | 78,107 µs | — | — |

## Caveat on the cross-board SHA-512 rows

Do not read the SHA-512 cycle or wall-time ratios as "the same rind SHA-512 on three chips."

- The Uno R3 numbers are a 32-bit-limb port of rind's rolled generic SHA-512 loop from `c08c91ea`, written that way so the 80-round body fits in 2 KB SRAM.
- The EFR32MG24 numbers are full rind-bench `sha512/rind/…` medians at `98568332`.
- The ESP32-S3 numbers are full rind-bench `sha512/rind/…` medians at the comparable set `d6ca9a75`.

The three commits and the two implementation shapes are not the same. In particular, the published ESP32-S3 rind SHA-512/1350B median is 8,421,487 cycles (35,089.5 µs at 240 MHz), far above the EFR32MG24's 190,122 cycles (2,437.5 µs at 78 MHz). That ESP32-S3 figure is the published median in the comparable set, not a typo: [results/RESULTS.md](../RESULTS.md) already explains that rind's portable SHA-512 on the S3 is fully unrolled 64-bit code that streams from flash past the instruction cache (about 0.80 M cycles a block at the 80 MHz flash clock), while a small cached loop does not. The Uno's rolled 32-bit-limb port is a different path again. Cycle ratios across those rows mix code, commit, cache, and flash behaviour. The SHA-256 rows are a closer, but still not identical, comparison (different commits; the Uno is the generic port, the other two are full rind-bench).
