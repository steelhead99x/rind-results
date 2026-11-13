<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 at rind 99b509a, flash at 80 MHz (superseded)

The set published until the 2026-11-12 rerun, moved here unchanged when `results/` was rerun at rind 98568332 (legacy rind-bench 911ebd2 (later squashed into rind-hardware `8d9c1bb`)), also at 80 MHz. `raw/` is its captures; the rest is its report and tables as published.

What changed at 98568332: SHA-3 is now in the report, and rind's Keccak rewrite takes it from 2.1x libcrux's time at 16KB to 0.95x (SHA3-256) and 0.88x (SHA3-512); ML-KEM-768 keygen and encaps drop from 0.85x and 0.78x RustCrypto's time to 0.51x and 0.47x. rind's linked code shrinks from 341,386 to 327,135 bytes (`sizes.json`). Two short-input rows moved further than this pin's relinks move them: HMAC-SHA256 64B from 3.61x to 2.20x ring (2.20x-2.35x over the three layouts in `../esp32s3-relink-98568332/`) and HKDF expand from 5.31x to 6.93x (6.93x-7.09x). That is the new code, or the whole image's different layout, which a relink can't separate; not attributed further.
