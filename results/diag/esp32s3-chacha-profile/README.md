<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3 ChaCha20-Poly1305 16KB seal PC profile

OpenOCD over the built-in USB-JTAG needs a root udev rule, so the profile came
from inside the firmware. With the `rng-diag` feature and `RIND_BENCH_SPIN`
set to `chacha-16k-rind` or `chacha-16k-ring`, the firmware seals 16 KB in a
loop on core 0. A priority-24 task on core 1 wakes every FreeRTOS tick
(100 Hz) and reads the preempted task's saved PC and a0 from `pxTopOfStack`
(words 1 and 3), then emits `{"type":"sample","pc":..,"a0":..}`.

`*.functions.txt` is `xtensa-esp32s3-elf-addr2line -fC` of each sampled PC
against the spin image (built from rind-hardware `esp32`, not committed: no
binaries). Classification for 400 samples:

| | Poly1305 | ChaCha20 | Total time | Poly1305 time |
|---|---|---|---|---|
| rind | 232 (58%): libcrux HACL poly1305, u64 add/mul/shr, u64 loads | 160 (40%) | 6.77 ms | ~3.9 ms |
| ring | 129 (32%): poly1305_update, mul32x32_64 | 227+ (57%) | 5.34 ms | ~1.7 ms |
