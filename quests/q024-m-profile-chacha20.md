---
id: Q024
title: "An M-profile (Thumb-2) ChaCha20 closes the remaining ChaCha gap"
priority: P2
hardware: [nrf52840, efr32mg24]
size: L
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [L] An M-profile (Thumb-2) ChaCha20 closes the remaining ChaCha gap

**Priority:** P2 · **Size:** L · **Hardware:** nrf52840, efr32mg24 · **Status:** open (maintainer only)

## Why it matters

Follow-up named by test plan item 2: what stays above 1.10× after Poly1305-32 is ChaCha20 itself, where ring assembles OpenSSL's Thumb-2 chacha-armv4.

## Evidence

- rind-results `results/nrf52840/gaps.toml`: "rind runs its portable backend, because its 32-bit Arm assembly is ARM-state NEON that an M-profile core lacks ... A Thumb-2 (or DSP-extension) ChaCha20 for M-profile would close it." XChaCha 16 KB: RustCrypto 946 kc vs rind 1,115 kc.

## Acceptance criteria

- [ ] Starts only after Q011 publishes; scope = the residual gap it reports.
- [ ] A constant-time M-profile ChaCha20 (Rust or proven assembly) with KATs, A/B under 0.90 on nRF52840 and EFR32MG24, and a Q015-style timing check.

## Related PRs

- https://github.com/steelhead99x/rind/pull/87 — Poly1305-32 (prerequisite)
