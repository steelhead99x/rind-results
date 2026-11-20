---
id: Q012
title: "SHA-256, HMAC and HKDF short inputs close the gap on MCUs (TLS key schedule)"
priority: P1
hardware: [esp32-s3, nrf52840, efr32mg24]
size: M
status: open
claimable_by: maintainer
test_plan_item: 3
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] SHA-256, HMAC and HKDF short inputs close the gap on MCUs (TLS key schedule)

**Priority:** P1 · **Size:** M · **Hardware:** esp32-s3, nrf52840, efr32mg24 · **Status:** open (maintainer only)

## Why it matters

Test plan item 3 ("Do first"). HKDF and HMAC run on every TLS handshake (feeds Q021). Luke's tiers show SHA-256 portable 0.88 and HKDF 0.98, so his charts cannot see these board losses.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): ESP32-S3 HKDF 5.50×–6.93×, SHA-256 64 B 2.36×, HMAC 64 B 2.20×; nRF/EFR SHA-256 1.43×–1.62×, HMAC up to 1.52×, HKDF up to 1.42×.
- rind-results `results/gaps.toml` (esp32s3) quoted by the plan: with a 32 KB icache, "hmac-sha256 64B 115.6 µs against ring's 140.3 µs".
- rind-results `results/nrf52840/gaps.toml`: ring's sha256-armv4 is 0.62× rind's cycles at 16 KB; `hkdf-sha256/expand` has ~2.5 kc fixed per-call setup (11.1 kc vs RustCrypto 8.5 kc, ring 9.3 kc), "Not investigated further".
- rind-hardware `patches/armv6m-sha512-ghash/0004`: SHA-256 below 2 KB by 8-round loops in `.text.rind_sha256_hot`, ARMv6-M only today.

## Acceptance criteria

- [ ] `sha2`, `hmac`, `hkdf` images base vs. the widened short path, 3 runs each, short rows held across the three-alignment relink (`--pad 1031`, `--pad 1039`).
- [ ] ESP32-S3 HKDF and HMAC within 1.10× at the default 16 KB icache, or a published 32 KB-icache variant with numbers and a documented recommendation.
- [ ] On M4/M33 the remaining gap is either closed by a Thumb-2 SHA-256 or recorded as an accepted gaps.toml entry naming sha256-armv4.
- [ ] The HKDF-expand fixed cost on nRF52840 is profiled and explained in gaps.toml.

## Related PRs

- https://github.com/steelhead99x/rind/pull/69 — ESP32-S3 SHA-256 in IRAM
- https://github.com/steelhead99x/rind/pull/100 — docs(portable): Xtensa SHA-256 IRAM stays on ssai/src asm
- https://github.com/steelhead99x/rind-hardware/pull/7 — build both rind#69 images and a board runbook
- https://github.com/steelhead99x/rind-agent/pull/65 — docs(skills): record the XIAO SHA-256 IRAM path
- https://github.com/steelhead99x/rind/pull/102 — feat: drop libcrux from the rind crate — overlaps the portable SHA-256 path
