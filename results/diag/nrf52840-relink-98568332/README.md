<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840: relinked build at rind 98568332

The same source as the published set (`results/raw-nrf52840/`), rebuilt with rind-bench's `cargo xtask build-images nrf --pad 1031 --out images-pad1031`, which links a never-executed 1031-byte static into the firmware's `.text` and so shifts the code after it. Three runs each of aes_gcm, chacha20poly1305 and sha2 (the other nine pad1031 images were built but deferred to keep the board's Meshtastic downtime short), in the same window as the published runs, 2026-11-12 12:36-12:47 PST.

- [relink.md](relink.md): rind over the fastest other library in each layout (`cargo xtask relink`); no ratio moves by more than 0.02.
- [compare.md](compare.md): every row of both sets (`cargo xtask compare`).
- `pad1031/`: the raw runs.

So on this board, with its flash cache, layout moves these rows by a percent or two, unlike the ESP32-S3 (see `../layout-relink-2026-11-12/`).
