---
id: Q014
title: "Re-baseline every MCU at the current rind pin"
priority: P1
hardware: [esp32-s3, nrf52840, efr32mg24]
size: M
status: open
claimable_by: maintainer
test_plan_item: 5
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] Re-baseline every MCU at the current rind pin

**Priority:** P1 · **Size:** M · **Hardware:** esp32-s3, nrf52840, efr32mg24 · **Status:** open (maintainer only)

## Why it matters

Test plan item 5. Every published MCU set was taken at rind `98568332` (2026-11-12). Items Q010–Q013 need a base at the same pin as their patches, or the A/B ratios mix two variables. Run alongside them.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): "The microcontroller sets ran at rind `98568332` (2026-11-12). Upstream has moved on since: #700 P-521, #702 portable RSA Montgomery, and 18 commits under `rind/src` in total."
- rind-hardware README: submodule "pinned at `d6ca9a75` (OPRF, 2026-11-13)"; upstream main has 43 commits the fork lacks (Q001).

## Acceptance criteria

- [ ] Full 12-image set, 3 runs each, on ESP32-S3, nRF52840, EFR32MG24 at one stated pin, published as new `results/<board>/<sha>/` trees (old trees untouched).
- [ ] No row moves more than run spread versus `98568332` unless a named commit explains it; any new over-10% row gets a gaps.toml entry.
- [ ] ESP32-S3 runs record flash clock (80 MHz) and icache size.

## Related PRs

- https://github.com/steelhead99x/rind-results/pull/35 — verify: recompute cycle medians from the serial capture
- https://github.com/steelhead99x/rind-hardware/pull/8 — lint RB captures against the timing contract
- https://github.com/steelhead99x/rind-agent/pull/56 — docs(skills): add a multi-agent hardware benchmarking skill
- https://github.com/steelhead99x/rind-agent/pull/59 — docs(bench-shared-hardware): rules for benchmarking on shared boards
