<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 poly1305-32 A/B (2026-11-12, America/Los_Angeles)

Optional rind-hardware patch set `patches/poly1305-32` (0006: portable Poly1305
in five 26-bit `u32` limbs on 32-bit targets) against the published HACL*
baseline (`-base`). Images are the pre-built set whose sha256 is recorded in
[patches/poly1305-32/SHA256SUMS](https://github.com/steelhead99x/rind-hardware/blob/7a14b38fd124786b9115b2b985c9eeed994dca1d/patches/poly1305-32/SHA256SUMS)
on rind-hardware `7a14b38`. `sha256sum -c` matched; they were not rebuilt.

- Board: ESP32-S3, one core at 240 MHz, CCOUNT. Every run log has
  `Boot SPI Speed : 80MHz` (12/12).
- 3 runs each of `chacha20poly1305` and `xchacha20poly1305`, base then
  poly1305-32, `cargo xtask run-all --flash-freq 80mhz`.
- Known-answer checks: 21/21 on both images for both benches. No failed runs.
- Compare: [compare.md](compare.md). Raw logs: [raw-base/](raw-base/), [raw/](raw/).

Rind is faster at every size that the benches report (ChaCha20-Poly1305 seal
and open; XChaCha20-Poly1305, which has no open rows). 16 KiB first:

| Row | hacl cycles | poly1305-32 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305/rind/16KB` | 1,610,381 | 1,184,190 | 0.735 |
| `chacha20poly1305-open/rind/16KB` | 1,621,040 | 1,194,335 | 0.737 |
| `chacha20poly1305/rind/1350B` | 143,125 | 105,056 | 0.734 |
| `chacha20poly1305-open/rind/1350B` | 144,083 | 105,963 | 0.735 |
| `chacha20poly1305/rind/1200B` | 126,525 | 92,387 | 0.730 |
| `chacha20poly1305-open/rind/1200B` | 127,379 | 93,173 | 0.731 |
| `chacha20poly1305/rind/quic-512B` | 57,721 | 42,051 | 0.729 |
| `chacha20poly1305-open/rind/quic-512B` | 58,139 | 42,477 | 0.731 |
| `chacha20poly1305/rind/quic-256B` | 32,711 | 23,669 | 0.724 |
| `chacha20poly1305-open/rind/quic-256B` | 32,966 | 23,938 | 0.726 |
| `chacha20poly1305/rind/quic-128B` | 20,216 | 14,475 | 0.716 |
| `chacha20poly1305-open/rind/quic-128B` | 20,397 | 14,649 | 0.718 |
| `chacha20poly1305/rind/32B` | 12,879 | 8,962 | 0.696 |
| `chacha20poly1305-open/rind/32B` | 12,984 | 9,076 | 0.699 |
| `xchacha20poly1305/rind/16KB` | 1,909,178 | 1,372,709 | 0.719 |
| `xchacha20poly1305/rind/1350B` | 445,856 | 295,542 | 0.663 |
| `xchacha20poly1305/rind/1200B` | 425,009 | 273,878 | 0.644 |
| `xchacha20poly1305/rind/quic-512B` | 354,764 | 222,301 | 0.627 |
| `xchacha20poly1305/rind/quic-256B` | 329,744 | 203,898 | 0.618 |
| `xchacha20poly1305/rind/quic-128B` | 317,231 | 194,695 | 0.614 |
| `xchacha20poly1305/rind/32B` | 288,956 | 141,457 | 0.490 |

ring and rustcrypto stayed at 1.000. libcrux moved as well (ChaCha20-Poly1305
32 B open 0.497, seal 1.321): that is a 16 KiB-cache layout shift, not a
change to libcrux's code. The patch only replaces rind's portable Poly1305.

This set, with the nRF52840 run in [results/nrf52840/diag/poly1305-32/](../../nrf52840/diag/poly1305-32/README.md),
meets the "faster at every size on both boards" bar for an upstream PR.
No upstream PR or issue was opened.
