<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Uno R3 runs

Five primed length benches after a single known-answer prelude, 2026-11-13 21:54:14 to 21:54:48 UTC (1:54 PM PT, America/Los_Angeles). Source: [raw/capture.log](raw/capture.log). Timer1 no prescale, 16 MHz, µs = cycles / 16 exactly.

Known-answer prelude (once, before run 1): SHA-256 empty PASS, SHA-256 "abc" PASS, SHA-512 "abc" PASS, ChaCha20 RFC 8439 PASS, Poly1305 RFC 8439 PASS. `TIMER nop1000` = 7,012 cycles (sanity only).

Published cells are the median of the five runs. ChaCha20 pattern-bench rows are listed for the record and are **not** published as results (see [README.md](README.md)).

| operation | run 1 | run 2 | run 3 | run 4 | run 5 | median | spread |
|---|---:|---:|---:|---:|---:|---:|---:|
| `sha256/32B` | 61,048 | 61,048 | 61,048 | 61,048 | 61,048 | 61,048 | 0.00% |
| `sha256/1350B` | 1,267,012 | 1,266,969 | 1,266,969 | 1,266,969 | 1,266,969 | 1,266,969 | 0.00% |
| `sha256/16KB` | 14,771,379 | 14,771,336 | 14,771,336 | 14,771,336 | 14,771,336 | 14,771,336 | 0.00% |
| `sha512/32B` | 216,452 | 216,452 | 216,452 | 216,452 | 216,452 | 216,452 | 0.00% |
| `sha512/1350B` | 2,317,472 | 2,317,472 | 2,317,472 | 2,317,472 | 2,317,515 | 2,317,472 | 0.00% |
| `sha512/16KB` | 27,108,699 | 27,108,699 | 27,108,699 | 27,108,699 | 27,108,699 | 27,108,699 | 0.00% |
| `poly1305/16B` | 16,509 | 16,509 | 16,509 | 16,509 | 16,509 | 16,509 | 0.00% |
| `poly1305/1350B` | 1,249,725 | 1,249,725 | 1,249,725 | 1,249,725 | 1,249,725 | 1,249,725 | 0.00% |

Spread is (max − min) / median. The 0.00% rows are either identical or differ by a few tens of cycles on a multi-million-cycle measurement (sha256/1350B is 43 cycles on 1,266,969, or 0.003%).

Not published:

| operation | run 1 | run 2 | run 3 | run 4 | run 5 | why |
|---|---:|---:|---:|---:|---:|---|
| `chacha20/64B` | 34,911 | 4,294,936,671 | 4,294,936,671 | 34,911 | 34,911 | checksum not stable; two wraps near 2<sup>32</sup> |
| `chacha20/1350B` | 32,382 | 32,382 | 32,382 | 4,294,934,099 | 4,294,934,099 | checksum not stable; some wraps; 1,350 B non-wrap readings are lower than 64 B |
| `chacha20+poly1305/1350B` | 4,294,937,268 | 4,294,937,268 | 4,294,937,268 | 4,294,937,268 | 4,294,937,268 | not RFC 8439 AEAD; all five wrapped |

Skipped on the board: x25519, P-256, Ed25519, ML-KEM, AES-GCM, SHA-3 (do not fit).
