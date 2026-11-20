---
id: Q020
title: "P-384, P-521 and RSA-2048 verify are timed on 32-bit limbs"
priority: P2
hardware: [efr32mg24, nrf52840, esp32-s3]
size: M
status: open
claimable_by: maintainer
test_plan_item: 6
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] P-384, P-521 and RSA-2048 verify are timed on 32-bit limbs

**Priority:** P2 · **Size:** M · **Hardware:** efr32mg24, nrf52840, esp32-s3 · **Status:** open (maintainer only)

## Why it matters

Test plan item 6. These are the tier chart's portable losses (P-521 2.79, P-256 1.41, P-384 1.16, RSA signatures 4.36), yet MCU P-256 is rind's biggest board win (0.40×–0.46× ring on Cortex-M). Nobody has measured which way the 32-bit-limb P-384/P-521/RSA code goes.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): only 12 images are built (aes_gcm, chacha20poly1305, xchacha20poly1305, cipher, x25519, p256, ed25519, sha2, sha3, hmac, hkdf, mlkem768); RSA, P-384, P-521 are "Not measured on any microcontroller" although the bench files exist in `rind-bench/benches/`.

## Acceptance criteria

- [ ] New `p384`, `p521` (ECDH, ECDSA sign/verify) and `rsa_pub` (verify) images, KATs passing, 3 runs each on EFR32MG24 and nRF52840, then ESP32-S3.
- [ ] rind at or under 1.10× of RustCrypto's p384/p521 and ring (P-384), or a gaps.toml entry naming 32-bit limbs.

## Related PRs

- https://github.com/steelhead99x/rind/pull/76 — perf(mont): issue four portable row products before the carry
