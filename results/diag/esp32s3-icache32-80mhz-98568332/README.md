<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: the 32 KB instruction-cache diagnostics at 80 MHz, rind 98568332

The earlier 32 KB-cache runs (`../curves-icache32/`, `../xchacha20poly1305-icache32/`, `../chacha20poly1305-icache32/`) ran at rind 99b509a with a 40 MHz flash clock. These are the same four benches at the published pin and clock: rind 98568332 with rind-bench's patches 0001-0004, built from legacy rind-bench 2f0c5e8 (later squashed into rind-hardware `8d9c1bb`) with `ESP_IDF_SDKCONFIG_DEFAULTS='sdkconfig.defaults;sdkconfig.icache32'` (32 KB instruction cache, 32 B lines; a diagnostic, not the published configuration), flashed with `--flash-freq 80mhz`, three runs each, 2026-11-12 17:56-18:03 PST. Hashes in [images.sha256](images.sha256). Compared against the published set (`results/raw/`, 16 KB cache, 80 MHz).

- [ratios.md](ratios.md): rind over the fastest other library in each configuration.
- `compare-<bench>.md`: every row, published against 32 KB (cycles, B/A).

Findings, the same as at 40 MHz:

- **XChaCha20-Poly1305 is the cache.** rind 1.10x RustCrypto at 32B (7.50x with 16 KB), 1.05x at 1350B (2.72x), 1.04x at 16KB (1.21x); rind's 32B seal goes from 288,632 to 42,247 cycles (1.20 ms to 176 µs), libcrux's from 302,388 to 41,023, RustCrypto's is unchanged.
- **ChaCha20-Poly1305 is not.** rind, ring and RustCrypto are unchanged (1.000x), so rind stays 1.45x, 1.27x and 1.26x ring at 32B, 1350B and 16KB: the Poly1305 compute gap. libcrux's open path gains up to 0.36x.
- **Curves: ring's and libcrux's X25519 and ring's P-256 are the cache**: `x25519-ecdh/libcrux` 0.061x (350.7 M to 21.3 M cycles), `x25519-ecdh/ring` 0.105x, ring's P-256 0.085x-0.132x. libcrux's P-256 (0.96x) is not. rind and dalek gain 2-17%. X25519 ECDH stays a tie between rind and dalek (1.00x); with 32 KB ring becomes rind's nearest P-256 rival and rind stays ahead (ECDH 0.70x ring against 0.59x RustCrypto at 16 KB, sign 0.96x, verify 0.66x). No verdict flips.

So at 80 MHz, as at 40 MHz, the published curve rows for ring and libcrux's X25519, and the XChaCha gap, measure the 16 KB cache.
