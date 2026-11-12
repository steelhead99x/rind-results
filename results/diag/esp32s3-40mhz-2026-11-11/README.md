<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: the first published set, at a 40 MHz flash clock (superseded)

The ESP32-S3 results published until 2026-11-12 11:15 PST: the eleven images built 2026-11-11/12
(`images.json`), three runs each. espflash flashed them without `--flash-freq`, so the flash ran at
40 MHz, not the 80 MHz the build configures (see `../flash-80mhz/`). They were superseded by the
80 MHz rerun in `../../RESULTS.md`: twelve images (with `sha3`) built in one pass from rind-bench
0463305, same rind pin and patches.

`RESULTS.md`, `RUNS.md` and the JSON here are regenerated from this raw with the corrected
provenance (the old RESULTS named rind-bench `a26b1a3`, HEAD at report time, not the source the
images came from). `tools/verify.py` checked the old tables against this raw: 0 mismatches.

At 80 MHz no gap verdict changes at the 1.1x line (40 headline operations), but cache-missing
rows run faster: `sha512/…` from 63x ring to 33x, `sha256/…/64B` from 6.65x to 2.29x libcrux,
`hkdf-sha256/expand` from 10.5x to 5.3x ring (cached ring rows do not move), and ring's
and libcrux's curve rows halve (`x25519-ecdh/ring` 167 M to 89 M cycles).

| Operation | 40 MHz: rind/fastest | 80 MHz: rind/fastest | verdict |
| aes128-gcm-open/…/1350B | 0.84x (ring) | 0.86x (ring) | ok |
| aes128-gcm-open/…/16KB | 0.86x (ring) | 0.87x (rustcrypto) | ok |
| aes128-gcm-open/…/32B | 0.74x (ring) | 0.77x (ring) | ok |
| aes128-gcm/…/1350B | 0.84x (ring) | 0.84x (ring) | ok |
| aes128-gcm/…/16KB | 0.86x (rustcrypto) | 0.87x (rustcrypto) | ok |
| aes128-gcm/…/32B | 0.68x (ring) | 0.62x (ring) | ok |
| aes256-gcm-open/…/1350B | 0.82x (ring) | 0.84x (ring) | ok |
| aes256-gcm-open/…/16KB | 0.89x (rustcrypto) | 0.91x (rustcrypto) | ok |
| aes256-gcm-open/…/32B | 0.72x (ring) | 0.74x (ring) | ok |
| aes256-gcm/…/1350B | 0.83x (ring) | 0.82x (rustcrypto) | ok |
| aes256-gcm/…/16KB | 0.89x (rustcrypto) | 0.91x (rustcrypto) | ok |
| aes256-gcm/…/32B | 0.59x (ring) | 0.67x (ring) | ok |
| chacha20poly1305/…/1350B | 1.31x (ring) | 1.28x (ring) | gap |
| chacha20poly1305/…/16KB | 1.27x (ring) | 1.26x (ring) | gap |
| chacha20poly1305/…/32B | 1.49x (ring) | 1.47x (ring) | gap |
| ed25519-sign/… | 0.88x (ring) | 0.79x (rustcrypto) | ok |
| ed25519-verify/… | 0.99x (rustcrypto) | 0.99x (rustcrypto) | ok |
| hkdf-sha256/expand/… | 10.51x (ring) | 5.31x (ring) | gap |
| hkdf-sha256/extract-expand/… | 8.52x (ring) | 4.92x (ring) | gap |
| hmac-sha256/…/1350B | 2.29x (libcrux) | 1.57x (libcrux) | gap |
| hmac-sha256/…/64B | 6.61x (ring) | 3.61x (ring) | gap |
| mlkem768-decaps/… | 0.31x (rustcrypto) | 0.33x (rustcrypto) | ok |
| mlkem768-encaps/… | 0.79x (rustcrypto) | 0.78x (rustcrypto) | ok |
| mlkem768-keygen/… | 0.88x (rustcrypto) | 0.85x (rustcrypto) | ok |
| p256-ecdh/… | 0.70x (rustcrypto) | 0.59x (rustcrypto) | ok |
| p256-ecdsa-sign/… | 1.08x (rustcrypto) | 0.91x (rustcrypto) | ok |
| p256-ecdsa-verify/… | 0.69x (rustcrypto) | 0.61x (rustcrypto) | ok |
| quic-header-protection/aes128/… | 0.65x (ring) | 0.65x (ring) | ok |
| quic-header-protection/aes256/… | 0.65x (ring) | 0.65x (ring) | ok |
| quic-header-protection/chacha20/… | 0.87x (ring) | 0.86x (ring) | ok |
| sha256/…/1350B | 1.74x (libcrux) | 1.23x (libcrux) | gap |
| sha256/…/16KB | 1.08x (libcrux) | 1.02x (libcrux) | ok |
| sha256/…/64B | 6.65x (libcrux) | 2.29x (libcrux) | gap |
| sha512/…/1350B | 63.36x (ring) | 32.96x (ring) | gap |
| sha512/…/16KB | 63.34x (ring) | 32.95x (ring) | gap |
| sha512/…/64B | 62.62x (ring) | 32.52x (ring) | gap |
| x25519-ecdh/… | 1.01x (rustcrypto) | 1.00x (rustcrypto) | ok |
| xchacha20poly1305/…/1350B | 4.11x (rustcrypto) | 2.63x (rustcrypto) | gap |
| xchacha20poly1305/…/16KB | 1.35x (rustcrypto) | 1.20x (rustcrypto) | gap |
| xchacha20poly1305/…/32B | 13.06x (rustcrypto) | 7.50x (rustcrypto) | gap |

40 operations, 0 change verdict at the 1.1x line
