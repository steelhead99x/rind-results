---
id: Q031
title: "EFR32MG24 Secure Engine and nRF52840 CryptoCell baseline rows"
priority: P3
hardware: [efr32mg24, nrf52840]
size: M
status: open
claimable_by: maintainer
test_plan_item: 12
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] EFR32MG24 Secure Engine and nRF52840 CryptoCell baseline rows

**Priority:** P3 · **Size:** M · **Hardware:** efr32mg24, nrf52840 · **Status:** open (maintainer only)

## Why it matters

Test plan item 12: gives users a software-vs-engine number, as `esp32s3-hw` does.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): on ESP32-S3 the engine does AES-128-GCM 16 KB in 4.43 ms vs rind's 19.96 ms, SHA-256 16 KB 185.76 µs vs 5.42 ms.

## Acceptance criteria

- [ ] Non-competing baseline rows (never counted as competitors) for AES-GCM, ChaCha20-Poly1305, SHA-2, ECDSA/ECDH, X25519, Ed25519 where the engine supports them.

## Related PRs

- none open
