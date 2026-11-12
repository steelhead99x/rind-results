<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: relinks that really move the libraries, rind 98568332

rind-bench 2f0c5e8 passes ld `--section-ordering-file` with `.flash.text : { *(.text.rind_bench_pad) }`, so `--pad N`'s static is the first thing in `.flash.text` (at `_stext`, 0x42000020) and every library after it moves. Before that, ESP-IDF's single `.flash.text` input rule put the static after the libraries: the relinks in `../esp32s3-relink-98568332/` moved the harness, and the libraries by only 12 bytes.

Images: `cargo xtask build-images esp aes_gcm chacha20poly1305 xchacha20poly1305 cipher sha2 sha3 hmac hkdf --pad N --out images-fixpadN` for N = 1031 and 2053, at rind-bench 2f0c5e8. pad0 is the published set (`results/raw/`, images at 911ebd2); a pad0 rebuild at 2f0c5e8 (`images-fixpad0-check`) puts every function at the same address as the published image, and its sha2 rerun the same afternoon (`pad0-control/`) matched the published sha2 runs (0 of 74 rows moved by more than max(1%, run spread); SHA-256 64B 2.36x libcrux in both), so the published set is a same-layout baseline and the moves below are layout, not session. Hashes are in [images.sha256](images.sha256).

[shifts.md](shifts.md) gives each library's shift from pad0, the most common per-function shift from `xtensa-esp32s3-elf-nm -n` (in parentheses, how many functions share it when not all do). rind, libcrux, ring and RustCrypto moved +1032 to +1048 B at pad1031 and +2056 to +2072 B at pad2053, except RustCrypto's `aes` and `ghash` in the aes_gcm and cipher images, which ld placed differently (shifts of -7772 to +4380 B). Both relinks shift by 4 mod 16 (1044 and 2068 in most images), which may be why they time the same; that is not tested.

Three runs of each bench per layout, flash at 80 MHz, 2026-11-12 15:05-15:44 PST; the pad0 control at 15:50-15:54 PST.

- [relink-short.md](relink-short.md): each short-input row (under 1 KB, or no size), rind over the fastest other library in each layout, the range and the ratio from the medians over layouts.
- [relink.md](relink.md): every operation of the eight benches.
- [compare-pad1031.md](compare-pad1031.md), [compare-pad2053.md](compare-pad2053.md): every row, pad0 against each (123 and 122 of 558 rows moved by more than max(1%, run spread)).
- `pad1031/`, `pad2053/`, `pad0-control/`: the raw runs.

Short inputs: SHA-256 64B moves most, 2.36x libcrux at pad0 and 2.80x in both relinks (rind's cycles spread 15% over layouts); HMAC-SHA256 64B 2.20x to 2.34x ring, HKDF extract-expand 5.50x to 5.81x, HKDF expand 6.93x to 7.02x, XChaCha20-Poly1305 32B 7.50x to 7.94x RustCrypto, and the AES-GCM open 32B rows 0.66x to 0.76x and 0.74x ring (ring's time moved, rind's by 0.7%). The other nine short-input operations move by at most 0.05 (median 0.05 over all sixteen; largest 0.44). No verdict flips: every row on either side of 1.0x stays there. The medians over three layouts weigh the two relinks, which share an alignment, twice.
