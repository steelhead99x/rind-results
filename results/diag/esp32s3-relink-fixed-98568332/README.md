<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: relinks that really move the libraries, rind 98568332

legacy rind-bench 2f0c5e8 (later squashed into rind-hardware `8d9c1bb`) passes ld `--section-ordering-file` with `.flash.text : { *(.text.rind_bench_pad) }`, so `--pad N`'s static is the first thing in `.flash.text` (at `_stext`, 0x42000020) and every library after it moves. Before that, ESP-IDF's single `.flash.text` input rule put the static after the libraries: the relinks in `../esp32s3-relink-98568332/` moved the harness, and the libraries by only 12 bytes.

Images: `cargo xtask build-images esp aes_gcm chacha20poly1305 xchacha20poly1305 cipher sha2 sha3 hmac hkdf --pad N --out images-fixpadN` for N = 1031 and 2053, at legacy rind-bench 2f0c5e8 (later squashed into rind-hardware `8d9c1bb`). pad0 is the published set (`results/raw/`, images at 911ebd2); a pad0 rebuild at 2f0c5e8 (`images-fixpad0-check`) puts every function at the same address as the published image, and its sha2 rerun the same afternoon (`pad0-control/`) matched the published sha2 runs (0 of 74 rows moved by more than max(1%, run spread); SHA-256 64B 2.36x libcrux in both), so the published set is a same-layout baseline and the moves below are layout, not session. Hashes are in [images.sha256](images.sha256).

[shifts.md](shifts.md) gives each library's shift from pad0, the most common per-function shift from `xtensa-esp32s3-elf-nm -n` (in parentheses, how many functions share it when not all do). rind, libcrux, ring and RustCrypto moved +1032 to +1048 B at pad1031 and +2056 to +2072 B at pad2053, except RustCrypto's `aes` and `ghash` in the aes_gcm and cipher images, which ld placed differently (shifts of -7772 to +4380 B). pad1031 and pad2053 shift every library by amounts 1024 B apart (1044 and 2068 in most images), so the code keeps the same alignment to the 32-byte cache line and the 16-byte fetch; that is why they time the same (tested below with pad1039).

Three runs of each bench per layout, flash at 80 MHz, 2026-11-12 15:05-15:44 PST; the pad0 control at 15:50-15:54 PST.

- [relink-short.md](relink-short.md): each short-input row (under 1 KB, or no size), rind over the fastest other library in each of three distinct alignments (pad0, pad1031, pad1039), the range and the ratio from the medians over them. pad2053 has pad1031's alignment and is left out so it is not counted twice.
- [relink.md](relink.md): every operation of the eight benches, the same three layouts.
- [relink-short-all-runs.md](relink-short-all-runs.md), [relink-all-runs.md](relink-all-runs.md): every set, including pad2053 and the second pad0 control, as columns.
- [compare-pad1031.md](compare-pad1031.md), [compare-pad2053.md](compare-pad2053.md): every row, pad0 against each (123 and 122 of 558 rows moved by more than max(1%, run spread)).
- [compare-pad1039.md](compare-pad1039.md): every row, pad0 against pad1039 (119 of 558 rows moved by more than max(1%, run spread)); [compare-pad0-control-mod16.md](compare-pad0-control-mod16.md): published pad0 against the second pad0 control (0 of 558).
- `pad1031/`, `pad2053/`, `pad1039/`, `pad0-control/`, `pad0-control-mod16/`: the raw runs.

## pad1039: a different alignment

`cargo xtask build-images esp <the eight benches> --pad 1039 --out images-fixpad1039`, built in a separate worktree (legacy rind-bench 2f0c5e8 (later squashed into rind-hardware `8d9c1bb`) with rind 98568332 and patches 0001-0004 only; the shared checkout had picked up patch 0005, which changes rind's code). A pad0 rebuild there (`images-p0check`, sha2) puts every function at the published address. [shifts-pad1039.md](shifts-pad1039.md): the libraries move +1036 to +1072 B, which against pad1031's shift differs mod 32 in seven of the eight images (rind: sha2 and xchacha 20 -> 28, hmac 20 -> 12, hkdf 20 -> 16, sha3 20 -> 0, cipher 20 -> 12, chacha 8 -> 16, aes_gcm 24 -> 8). Three runs each at 80 MHz, 2026-11-12 17:14-17:33 PST, then the published pad0 images again (17:33-17:52 PST), which matched the published set row for row (0 of 558 moved).

pad1039 times differently from both others: SHA-256 64B 3.07x libcrux (2.36x at pad0, 2.80x at pad1031/2053), HMAC-SHA256 64B 2.07x ring (2.20x, 2.34x), XChaCha20-Poly1305 32B 7.59x RustCrypto (7.50x, 7.94x), AES-256-GCM 32B 0.56x ring (0.56x, 0.61x). So the identical pad1031 and pad2053 results fit their shared alignment: moving the code 1 KB without changing its alignment to the cache line changed nothing, while a few bytes of alignment changed these rows by up to 0.3 (which alignment bits matter, 16-byte fetch or 32-byte line, is not separated here).

## Short inputs over three alignments

From [relink-short.md](relink-short.md) (pad0, pad1031, pad1039): SHA-256 64B 2.36x to 3.07x libcrux, 2.79x from the medians (rind's cycles spread 26%); HMAC-SHA256 64B 2.07x to 2.34x ring, 2.20x; HKDF extract-expand 5.50x to 5.81x, 5.79x; HKDF expand 6.93x to 7.11x, 7.02x; XChaCha20-Poly1305 32B 7.50x to 7.94x RustCrypto, 7.59x; the AES-GCM open 32B rows 0.66x to 0.77x and 0.66x to 0.75x ring, 0.76x and 0.74x; AES-256-GCM 32B 0.56x to 0.61x, 0.56x. The other eight move by at most 0.05 (median range over all sixteen 0.05, largest 0.71). No verdict flips: every row on either side of 1.0x stays there.
