---
id: Q026
title: "rind builds for Xtensa without the MD5 asm patch"
priority: P2
hardware: [esp32-s3]
size: S
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] rind builds for Xtensa without the MD5 asm patch

**Priority:** P2 · **Size:** S · **Hardware:** esp32-s3 · **Status:** open (maintainer only)

## Why it matters

rind-hardware calls this "the only thing stopping rind itself from building for `xtensa-esp32s3-espidf`".

## Evidence

- `rind/src/low/generic/md5.rs:23`: `core::arch::asm!("/* {0} */", ...)` as a reassociation barrier, which needs `asm_experimental_arch` on Xtensa.
- rind-hardware `patches/0001-fix-portable-build-MD5-on-Xtensa-...patch`: take `black_box` there, as on wasm and Miri. README: "None has been opened" upstream.

## Acceptance criteria

- [ ] The fix is offered upstream as a single small PR; after it lands, rind-hardware drops 0001 and the 12 ESP32-S3 images still pass KATs.

## Related PRs

- https://github.com/steelhead99x/rind/pull/85 — ci: lint rind on the ESP32's Xtensa targets
