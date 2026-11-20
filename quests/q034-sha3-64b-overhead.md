---
id: Q034
title: "SHA-3 64 B per-call overhead is rechecked on M4/M33"
priority: P3
hardware: [nrf52840, efr32mg24]
size: S
status: open
claimable_by: maintainer
test_plan_item: 15
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] SHA-3 64 B per-call overhead is rechecked on M4/M33

**Priority:** P3 · **Size:** S · **Hardware:** nrf52840, efr32mg24 · **Status:** open (maintainer only)

## Why it matters

Test plan item 15: accepted as small; recheck only after Q010–Q012 change the hash plumbing.

## Evidence

- rind-results `results/nrf52840/gaps.toml`: about 2.0 k cycles more than libcrux per call (19.3–19.4 k vs 17.4 k); "Decision: accepted, small." Test plan: 1.11×–1.12× libcrux on nRF52840 and EFR32MG24.

## Acceptance criteria

- [ ] Re-measured after Q010–Q012 land; gaps.toml entry updated or dropped.

## Related PRs

- none open
