<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: a second host and two more boards (2026-11-12)

The twelve rind-bench images rebuilt on a second build host from
[rind-hardware](https://github.com/steelhead99x/rind-hardware) main `a87d3a1` (rind `98568332`,
`cargo xtask build-images esp`; ESP-IDF v5.5.1, esp toolchain 1.97.0-nightly, GCC esp-14.2.0_20241119)
and run 5 times each on two further ESP32-S3 boards (chip revision v0.2, 8 MB flash, 240 MHz),
`board-1` and `board-2`, with `cargo xtask run-all` and espflash 4.7.0 at an 80 MHz flash clock,
2026-11-12 20:42-22:12 PST (120 of 120 runs ok). The ELFs' sha256 are in `images.sha256` and `images.json`. The question:
do the published medians repeat on other boards and on another build of the same source?

**The boards agree with each other exactly; the build differs from the published one in its short-input
rows.**

- Every known-answer check passes on every run of both boards (45/45, 35/35, 80/80, 20/20, 20/20,
  20/20, 15/15, 20/20, 65/65, 75/75, 20/20, 35/35 per bench file over 5 runs).
- Run-to-run: the widest spread of run medians is 2.4% (`aes256-gcm-open/rustcrypto/quic-64B`, on
  both boards); every other row is under 0.6%.
- board-1 against board-2: 0 of 612 rows move by more than max(1%, run spread)
  ([compare-board-1-board-2.md](compare-board-1-board-2.md)).
- Against the published set (`../../raw`): cipher, ed25519, mlkem768, sha3 and x25519 repeat
  (0 rows moved). The rows that move are short-input rows of SHA-2, HMAC, HKDF, AES-GCM, ChaCha20-Poly1305
  and XChaCha20-Poly1305 ([compare-published-board-1.md](compare-published-board-1.md),
  [compare-published-board-2.md](compare-published-board-2.md)): for example rind's sha2 short rows range
  0.64x to 1.64x of published, while every library's median B/A stays within 0.95-1.05. This is
  the layout sensitivity documented in `../esp32s3-relink-fixed-98568332/` and
  `../esp32s3-hmac-hkdf-repin-2026-11-12/`: these images are a different link of different code,
  since they carry rind-hardware `patches/0005` (ReuseCache without CAS), which the published images
  predate, and were linked on another host. No conclusion about rind's speed changes here; read
  short-input ratios with the README's layout caveat.

Both boards ran at the same time as a third (EFR32MG24) board on the same USB host. Every number is
the board's own cycle counter, so host load only affects transfer between benchmarks; the
spreads above show no effect.

`board-1/` and `board-2/` hold each bench's `run-N.log`, `run-N.jsonl` and `runs.json` as capture
writes them. Compare tables made with:

```sh
cargo xtask compare results/raw results/diag/esp32s3-second-host-2026-11-12/board-1 aes_gcm chacha20poly1305 cipher ed25519 hkdf hmac mlkem768 p256 sha2 sha3 x25519 xchacha20poly1305 --label-a published --label-b board-1 --out results/diag/esp32s3-second-host-2026-11-12/compare-published-board-1.md
```

(the same with `board-2`, and with `board-1` against `board-2`).
