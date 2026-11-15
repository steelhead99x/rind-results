---
id: Q030
title: "ML-DSA-65 is timed on an MCU"
priority: P3
hardware: [esp32-s3, efr32mg24]
size: M
status: open
claimable_by: maintainer
test_plan_item: 11
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] ML-DSA-65 is timed on an MCU

**Priority:** P3 · **Size:** M · **Hardware:** esp32-s3, efr32mg24 · **Status:** open (maintainer only)

## Why it matters

Test plan item 11: Luke's report never times ML-DSA ("Not timed" on all six hosts); rind-results has it only on x86/aarch64.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): Jetson ML-DSA 0.84×–0.91× aws-lc-rs.

## Acceptance criteria

- [ ] keygen/sign/verify vs libcrux's and RustCrypto's ML-DSA on ESP32-S3 (PSRAM), then EFR32MG24 if RAM allows, plus peak stack/heap.

## Related PRs

- none open
