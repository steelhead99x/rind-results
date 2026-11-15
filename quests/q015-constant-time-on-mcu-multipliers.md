---
id: Q015
title: "Constant time is measured on the MCU multipliers (M33, M0+, Xtensa, M4)"
priority: P1
hardware: [efr32mg24, rp2040, esp32-s3, nrf52840]
size: L
status: open
claimable_by: maintainer
test_plan_item: 9
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [L] Constant time is measured on the MCU multipliers (M33, M0+, Xtensa, M4)

**Priority:** P1 · **Size:** L · **Hardware:** efr32mg24, rp2040, esp32-s3, nrf52840 · **Status:** open (maintainer only)

## Why it matters

Test plan item 9, raised to P1 because it is a security question: speed numbers on these boards are only worth publishing if the code is constant time there, and PLATFORMS.md lists variable-time cores but says nothing about Cortex-M or Xtensa.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): `baremetal/mg24/README.md`: "Arm publishes no M33 cycle table … constant time is unverified until measured"; the poly1305-32 README claims constant time on ESP32-S3 and M4, "not Cortex-M3".
- rind-results `hosts/rp2040/QUEST-DRAFT.md`: Cortex-M3 `umull` finishes early on small operands; Espressif RISC-V and Xtensa multiply timing undocumented; ARMv6-M 32×32→64 goes through `__aeabi_lmul`, which on rustc 1.99.1 is branch-free, and proposes adding `thumbv6m-none-eabi` to `admin/ct-vartime`.

## Acceptance criteria

- [ ] Fixed-vs-random dudect-style runs on X25519, P-256 ECDH and ECDSA sign, Poly1305-32 and ML-KEM decaps, at least 10^6 traces per function per board, with |t| under 4.5 or a documented leak.
- [ ] `umull`/`mull` timed with small vs large operands on each core and the result added to PLATFORMS.md's multiplier list.
- [ ] `admin/ct-vartime` release builds cover `thumbv6m-none-eabi` and `thumbv7em-none-eabihf` (proposed upstream).

## Related PRs

- https://github.com/steelhead99x/rind/pull/87 — Poly1305-32 (constant-time claim to verify)
