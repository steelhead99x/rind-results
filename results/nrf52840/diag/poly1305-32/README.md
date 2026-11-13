<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840 (T1000-E) poly1305-32 A/B (2026-11-12, America/Los_Angeles)

The same optional rind-hardware `patches/poly1305-32` set as
[results/esp32s3/diag/poly1305-32/](../../esp32s3/diag/poly1305-32/README.md),
on the Seeed T1000-E (Cortex-M4F, 64 MHz, DWT_CYCCNT). Image sha256 matched
[SHA256SUMS](https://github.com/steelhead99x/rind-hardware/blob/7a14b38fd124786b9115b2b985c9eeed994dca1d/patches/poly1305-32/SHA256SUMS)
on rind-hardware `7a14b38`; they were not rebuilt.

The Meshtastic node was backed up first (`~/t1000e-backups/20261112-182202`):
config export, `CURRENT.UF2`, SoftDevice `0x1000`-`0x27000` sha256
`[redacted]` (unchanged
after the run). The node's background services
were stopped for the window. The firmware reported
`wdt_runstatus=1`, `wdt_crv=2949120` (90 s), `wdt_rren=0x1` in every start
record and fed the leftover watchdog; all 12 runs completed. After the run the
Meshtastic app-region UF2 was restored, the SoftDevice region and bootloader
`INFO_UF2.TXT` were unchanged, the exported config matched the pre-run export,
and the bridge logged its `connected:` line again.

- 3 runs each of `chacha20poly1305` and `xchacha20poly1305`, base then
  poly1305-32, `cargo xtask run-all-nrf`.
- Known-answer checks: 21/21 on both images for both benches. No failed runs.
- Compare: [compare.md](compare.md). Raw logs: [raw-base/](raw-base/), [raw/](raw/).

Rind is faster at every size that the benches report. 16 KiB first:

| Row | hacl cycles | poly1305-32 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305/rind/16KB` | 895,189 | 624,562 | 0.698 |
| `chacha20poly1305-open/rind/16KB` | 918,865 | 648,242 | 0.705 |
| `chacha20poly1305/rind/1350B` | 82,315 | 57,410 | 0.697 |
| `chacha20poly1305-open/rind/1350B` | 84,417 | 59,478 | 0.705 |
| `chacha20poly1305/rind/1200B` | 72,818 | 50,334 | 0.691 |
| `chacha20poly1305-open/rind/1200B` | 74,667 | 52,180 | 0.699 |
| `chacha20poly1305/rind/quic-512B` | 34,440 | 23,920 | 0.695 |
| `chacha20poly1305-open/rind/quic-512B` | 35,433 | 24,776 | 0.699 |
| `chacha20poly1305/rind/quic-256B` | 20,561 | 14,229 | 0.692 |
| `chacha20poly1305-open/rind/quic-256B` | 21,186 | 14,716 | 0.695 |
| `chacha20poly1305/rind/quic-128B` | 13,622 | 9,384 | 0.689 |
| `chacha20poly1305-open/rind/quic-128B` | 14,063 | 9,686 | 0.689 |
| `chacha20poly1305/rind/32B` | 9,877 | 6,503 | 0.658 |
| `chacha20poly1305-open/rind/32B` | 10,046 | 6,667 | 0.664 |
| `xchacha20poly1305/rind/16KB` | 1,098,677 | 838,153 | 0.763 |
| `xchacha20poly1305/rind/1350B` | 297,393 | 270,641 | 0.910 |
| `xchacha20poly1305/rind/1200B` | 284,169 | 264,697 | 0.931 |
| `xchacha20poly1305/rind/quic-512B` | 244,513 | 235,321 | 0.962 |
| `xchacha20poly1305/rind/quic-256B` | 233,989 | 226,181 | 0.967 |
| `xchacha20poly1305/rind/quic-128B` | 227,485 | 221,701 | 0.975 |
| `xchacha20poly1305/rind/32B` | 222,925 | 219,949 | 0.987 |

libcrux, ring and rustcrypto stayed at 1.00 (within about 1%). Unlike the
ESP32-S3, this board's other libraries did not move with the patch. The
smallest gain is XChaCha20-Poly1305 at 32 B (0.987).

Together with the ESP32-S3 set, rind's seal and open (and the XChaCha rows,
which have no open) are faster at every size on both boards, so the patch
meets the bar for an upstream PR. No upstream PR or issue was opened.
