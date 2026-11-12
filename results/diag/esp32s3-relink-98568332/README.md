<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: three layouts at rind 98568332

The published set (`results/raw/`, pad0) and two relinked builds of the same source, rind-bench's `cargo xtask build-images esp --pad 1031` and `--pad 2053` (a never-executed static of that many bytes in the firmware's own `.text`, which moves the code linked after it: the linker puts it after the libraries, so it moves the firmware's bench loop and harness by that much and the libraries by 12 bytes; see `../esp32s3-hmac-hkdf-repin-2026-11-12/`), for all twelve benches. Three runs of each, flash at 80 MHz, on the same board: the seven benches whose short-input rows the 99b509a relink moved (`../layout-relink-2026-11-12/`) on 2026-11-12 12:01-13:06 PST in one sequence with the published set, and ed25519, mlkem768, p256, sha3 and x25519 at 13:48-14:14 PST (built from the same tree, 911ebd2). `images.sha256` is every relinked image's hash.

- [relink-short.md](relink-short.md): each short-input row (under 1 KB, or no size), rind over the fastest other library in each layout, the range, and the ratio from the medians over the three layouts. This is the multi-relink median for those rows.
- [relink.md](relink.md): the same for every operation of the twelve benches.
- [compare-pad1031.md](compare-pad1031.md), [compare-pad2053.md](compare-pad2053.md): every row, pad0 against each.
- `pad1031/`, `pad2053/`: the raw runs.

Most rows move by under 0.1 (the median move is 0.00; the five benches added later move by at most 0.04, P-256 ECDSA sign). SHA-256 at 64B moves most (2.36x, 3.55x and 2.78x libcrux's time); the AES-GCM 32B rows are 0.53x-0.66x in the published layout and 0.58x-0.77x in both others. Numbers taken from one layout carry that much uncertainty on this board. These layouts are placements of the harness against the libraries, not of the libraries themselves, so a repin, which moves library code by kilobytes, can move a row further than any of them (HMAC-SHA256 64B and HKDF expand did).
