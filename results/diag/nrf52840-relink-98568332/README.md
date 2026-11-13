<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840: relinked build at rind 98568332

> **Correction (2026-11-12, 15:50 PST).** This relink did not move the libraries: `--pad` then put its static after the Rust code, so rind, libcrux, ring's Rust and RustCrypto moved 8 bytes (only ring's assembly and compiler_builtins moved about 1 KB; see `arm-none-eabi-nm -n`). The relinks in [`../nrf52840-relink-fixed-98568332/`](../nrf52840-relink-fixed-98568332/) move every library by about 1 KB and 2 KB and reach the same conclusion for the short inputs.

The same source as the published set (`results/raw-nrf52840/`), rebuilt with rind-hardware's `cargo xtask build-images nrf --pad 1031 --out images-pad1031`, which links a never-executed 1031-byte static into the firmware's `.text` and so shifts the code after it.

- aes_gcm, chacha20poly1305 and sha2 ran in the published window, 2026-11-12 12:36-12:47 PST.
- The other nine benches (cipher, ed25519, hkdf, hmac, mlkem768, p256, sha3, x25519, xchacha20poly1305) ran in a later window, 2026-11-12 14:00-14:23 PST, after a first-image watchdog probe had let the leftover Meshtastic watchdog time out and clear. Three runs each, all complete.

- [relink.md](relink.md): rind over the fastest other library in each layout (`cargo xtask relink`); 46 operations, no ratio moves by more than 0.03 (the largest is QUIC ChaCha header protection, 0.95× to 0.98× ring; the median range over layouts is 0.01).
- [compare.md](compare.md): every row of both sets (`cargo xtask compare`).
- `pad1031/`: the raw runs (the twelve benches only; a `zz-aes_gcm-pad0` retry from the first window is kept out so it does not double-count aes_gcm rows).

So on this board, with its flash cache, layout moves these rows by a percent or two, unlike the ESP32-S3 (see `../layout-relink-2026-11-12/` and `../esp32s3-relink-98568332/`).
