<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: why the ChaCha20-Poly1305 and XChaCha20-Poly1305 rows crashed

**Status: fixed and verified on hardware, 2026-11-12 09:09-09:11 PST.** The patched
images (`chacha20poly1305.elf` `b5a7e512…`, `xchacha20poly1305.elf` `9e07e2b5…`, in
[../images.sha256](../images.sha256)) passed every known-answer check and finished 3 of 3
runs each (56 and 21 results a run) with no fault: [raw/chacha20poly1305](../raw/chacha20poly1305/),
[raw/xchacha20poly1305](../raw/xchacha20poly1305/). Their rows are in
[RESULTS.md](../RESULTS.md), with the gaps they show explained in [gaps.toml](../gaps.toml).

## What happened

- Both images failed 3 of 3 runs, with `StoreProhibited` during the first
  known-answer check: [esp32s3-chacha-crash-raw](esp32s3-chacha-crash-raw/) (the unpatched
  images' runs, kept for the record).
- **The fault was in ring, not rind.** ring's fallback Poly1305 state is
  `#[repr(C, align(64))]`. For any function holding such a local,
  `ring::aead::chacha20_poly1305::{seal,open}` included, LLVM-Xtensa realigns SP
  after `entry` with a plain `add` instead of `movsp`. This is
  [espressif/llvm-project#140](https://github.com/espressif/llvm-project/issues/140).
  Register-window spills, which happen on every interrupt under ESP-IDF, then
  overwrite live locals.
- In Espressif's QEMU, the unpatched image faulted in ring's `ChaCha20_ctr32`. It
  was handed an `Overlapping` whose `len` held a spilled return address,
  `0x8200d6e3`.
- Ruled out:
  - **rind.** No rind function realigns the stack, and rind's ChaCha passes its
    checks here and on the nRF52840.
  - **Stack overflow.** Frames on the path are under 2 KB, against a 96 KB or
    160 KB task stack.
  - **Unaligned loads.** No fault had cause 9.

## Fix

- rind-hardware carries `patches/ring-0.17.14-xtensa-poly1305-align.patch`.
  It keeps the state at 16 bytes on Xtensa, and nothing in ring's C code needs 64.
- The patched images contain no realigning function. In QEMU they finish all 56
  ChaCha20-Poly1305 results.
- The write-up and a 20-line repro are in rind-hardware at `repro/xtensa-overalign/`.
  The repro fails at every opt-level and with fat LTO, on the esp toolchain
  1.97.0.0.
- This is a ring-side workaround for a compiler bug. ring's ChaCha20-Poly1305 numbers
  on Xtensa come from the patched build. The other published ESP32-S3 rows
  use unpatched ring, which never realigns outside `chacha20_poly1305::{seal,open}`.
