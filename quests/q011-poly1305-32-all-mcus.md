---
id: Q011
title: "Poly1305 in 26-bit u32 limbs is A/B'd on all four MCUs and offered upstream"
priority: P1
hardware: [esp32-s3, nrf52840, efr32mg24, rp2040]
size: M
status: open
claimable_by: maintainer
test_plan_item: 2
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] Poly1305 in 26-bit u32 limbs is A/B'd on all four MCUs and offered upstream

**Priority:** P1 · **Size:** M · **Hardware:** esp32-s3, nrf52840, efr32mg24, rp2040 · **Status:** open (maintainer only)

## Why it matters

Test plan item 2 ("Do first"). ChaCha20-Poly1305 is the second-biggest steady loss on every board and is the AEAD rind-rustls prefers wherever `has_aes_hardware()` is false, which includes every MCU (PLATFORMS.md). Luke's tier chart shows it winning (portable 0.90, armv7 0.99), so it hides this.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): ChaCha20-Poly1305 vs ring 1.74×–1.79× nRF52840, 1.55×–1.61× EFR32MG24, 1.26×–1.45× ESP32-S3.
- rind-hardware README "The upstream PR" item 5: hardware A/B (2026-11-12, 21/21 KATs) ChaCha20-Poly1305 seal 16 KB 0.735× on ESP32-S3 and 0.698× on T1000-E; XChaCha 16 KB 0.719× and 0.763×. "No upstream PR or issue has been opened." The A/B is only in `results/*/diag/poly1305-32`, not the published tables.
- Not yet A/B'd on EFR32MG24 or RP2040 (test plan).

## Acceptance criteria

- [ ] `chacha20poly1305` and `xchacha20poly1305` images, base vs. `--set poly1305-32`, every size row, 3 runs each on all four boards at the Q014 pin, published with per-board ratio against ring.
- [ ] RP2040 result states whether 32-bit limbs still help without `umull`.
- [ ] Whatever stays above 1.10× is attributed to ChaCha20 itself and handed to Q024.
- [ ] The patch is offered to kixel-ai/rind as its own PR (per rind-agent `fork-to-upstream`), linking the published A/B.

## Related PRs

- https://github.com/steelhead99x/rind/pull/87 — feat(portable): Poly1305 in 26-bit limbs on 32-bit targets
- https://github.com/steelhead99x/rind/pull/102 — feat: drop libcrux from the rind crate — may overlap the portable Poly1305
