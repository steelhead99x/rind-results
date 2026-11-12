# EFR32MG24

EFR32MG24B220F1536IM48 (Secure Vault High), Cortex-M33F at 78 MHz, 1536 KiB flash at `0x08000000`, 256 KiB RAM. Onboard CMSIS-DAP, reached with probe-rs 0.32.0. Debug was not locked; no erase or unlock was used.

An SSD1306 128x64 sits on I2C1 (PC4 SDA, PC5 SCL, address 0x3C).

rind 0.3.2 does not build for `thumbv8m.main-none-eabihf` (`can't find crate for std`, after getrandom's custom-backend cfg). Portable-backend dependencies do. Nothing rind-related was flashed. See `../rp2040/COMPETITOR-BUILDS.md`.

Unlike the RP2040, this target has CAS (`target_has_atomic` 8/16/32/ptr), so rustls (no provider) and rustls-rustcrypto build. It also has a TRNG in the Secure Engine. `umull` is present (rustc 1.99.1 lowers a u32×u32→u64 product to one `umull`); Arm publishes no M33 cycle table and names early termination only for the divider, so constant time is unverified until measured. The Secure Engine does AES-GCM, ChaCha20-Poly1305, SHA-2, ECDSA/ECDH, X25519, and Ed25519, which would give a software-vs-engine row once rind is `no_std`.

The OLED demo in `oled-demo/` is display-only.

## Backup and restore

Main flash `0x08000000`–`0x08180000` was read three times; all three copies matched.

sha256 `53b5c80d8bc284a281922a8ab8c937791fa8f5b62f4e210b7e57ac8317261b1b`

USER DATA page `0x0FE00000` (1 KiB) was read twice, hash `5f4ecdb7b71c3e403983fe405cddcdc2f2576b655fdb3e80d94a6f7c32e58bc2`, and was not written.

The images are not in this tree. The OLED demo wrote only the app region (`0x08006000`–`0x080E98A0`); the bootloader and last pages were left alone.

To restore the board's existing firmware, put `backup.bin` next to `restore.sh` and run it. The script checks the hash, writes the image at `0x08000000` with `--verify`, reads it back, checks the hash again, and resets.
