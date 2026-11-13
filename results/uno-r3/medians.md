<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Uno R3 medians (2026-11-13, 1:54 PM PT, America/Los_Angeles)

Board: Arduino Uno R3, ATmega328P @ 16 MHz, 32 KB flash (32,256-byte app region after the 512-byte bootloader), 2 KB SRAM. This is not an Uno R4.

Firmware is a `no_std` port of rind's portable/generic SHA-256, rolled SHA-512, ChaCha20, and 64-bit-limb Poly1305 from `c08c91ea` (`origin/rp2040/armv6m-no-std`). It is **not** the full rind crate: rind's TARGET-SURVEY lists AVR as out of scope (8-bit, 16-bit pointers, too little RAM). On the AVR, SHA-512 compression uses 32-bit limbs of that same rolled loop so the 80-round body fits in SRAM; host known-answer tests and on-device hashlib digests agree with the 64-bit reference.

Image: 26,880 / 32,256 flash (83.3%), 1,013 / 2,048 SRAM (data+bss), 1,035 bytes of stack room. Timed with Timer1, no prescale (1 tick = 1 cycle). Each length is primed once, then timed; figures are medians of 5 runs. µs = cycles / 16 exactly. Raw serial: [raw/capture.log](raw/capture.log). See [README.md](README.md) for what this is, and [RESULTS.md](RESULTS.md) for the comparison and caveats.

All published SHA-256, SHA-512, and Poly1305 digests matched a host hashlib / RFC 8439 Poly1305 check. RFC 8439 ChaCha20 and Poly1305 known-answer tests also passed on the board.

## Published medians

| operation | Uno R3 cycles (n=5) | range | Uno R3 µs |
|---|---:|---:|---:|
| sha256/32B | 61,048 | 61,048–61,048 | 3,815 |
| sha256/1350B | 1,266,969 | 1,266,969–1,267,012 | 79,185 |
| sha256/16KB | 14,771,336 | 14,771,336–14,771,379 | 923,208 |
| sha512/32B | 216,452 | 216,452–216,452 | 13,528 |
| sha512/1350B | 2,317,472 | 2,317,472–2,317,515 | 144,842 |
| sha512/16KB | 27,108,699 | 27,108,699–27,108,699 | 1,694,293 |
| poly1305/16B | 16,509 | 16,509–16,509 | 1,031 |
| poly1305/1350B | 1,249,725 | 1,249,725–1,249,725 | 78,107 |

ChaCha20 pattern-bench rows are omitted (see [README.md](README.md)). JSON: [medians.json](medians.json).
