---
id: Q032
title: "aarch64 without the crypto extension is checked on a real Pi 4 / A53"
priority: P3
hardware: [host:pi4]
size: M
status: open
claimable_by: maintainer
test_plan_item: 13
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] aarch64 without the crypto extension is checked on a real Pi 4 / A53

**Priority:** P3 · **Size:** M · **Hardware:** host:pi4 · **Status:** open (maintainer only)

## Why it matters

Test plan item 13: Luke's "no crypto" tier is a feature-denied EPYC/Neoverse runner, not real silicon.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): PLATFORMS.md flags the Cortex-A53 64-bit multiply as variable-time (Zero 2 W, Pi 3).

## Acceptance criteria

- [ ] Tier numbers confirmed on a Pi 4 / CM4, and Q015's CT test run on an A53.

## Related PRs

- https://github.com/steelhead99x/rind/pull/82 — perf(aarch64): run s2n-bignum's narrow-multiplier schedules on narrow cores
