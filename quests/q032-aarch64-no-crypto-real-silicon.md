---
id: Q032
title: "aarch64 without the crypto extension is checked on a real Pi 4 / A53"
priority: P3
hardware: [host:pi4, host:pi-zero-2w]
size: M
status: open
claimable_by: maintainer
test_plan_item: 13
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] aarch64 without the crypto extension is checked on a real Pi 4 / A53

**Priority:** P3 · **Size:** M · **Hardware:** host:pi4, host:pi-zero-2w · **Status:** open (maintainer only)

## Why it matters

Test plan item 13: Luke's "no crypto" tier is a feature-denied EPYC/Neoverse runner, not real silicon.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): PLATFORMS.md flags the Cortex-A53 64-bit multiply as variable-time (Zero 2 W, Pi 3).

## Acceptance criteria

- [ ] Tier numbers confirmed on a Pi 4 / CM4 (`host:pi4`, Cortex-A72).
- [ ] On a Cortex-A53 (`host:pi-zero-2w`; a Pi 3 also counts), fixed-vs-random dudect-style runs on X25519, P-256 ECDH and ECDSA sign, and ML-KEM decaps, at least 10^6 traces per function, with |t| under 4.5 or a documented leak. This is Q015's method; Q015 itself covers only the four MCU boards.

## Related PRs

- https://github.com/steelhead99x/rind/pull/82 — perf(aarch64): run s2n-bignum's narrow-multiplier schedules on narrow cores
