---
id: Q013
title: "RP2040 (Cortex-M0+) gets its first published timing set"
priority: P1
hardware: [rp2040]
size: M
status: open
claimable_by: maintainer
test_plan_item: 4
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] RP2040 (Cortex-M0+) gets its first published timing set

**Priority:** P1 · **Size:** M · **Hardware:** rp2040 · **Status:** open (maintainer only)

## Why it matters

Test plan item 4 ("Do first"). The Pico is the worst case for 32-bit limbs (no `umull`, no CAS) and no tier on Luke's chart approximates it. It builds and passes KATs, but rind-results has zero RP2040 numbers.

## Evidence

- rind-results `hosts/rp2040/NOTES.md`: "Known-answer checks later passed on a Raspberry Pi Pico (2026-11-12). Published timing results are not in this repository yet (there is no `results/rp2040/` tree on main)".
- rind-hardware README board table: RP2040 "builds; known-answer checks passed on a Pico (2026-11-12); published timing results are not yet in rind-results".
- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): the patch set claims sha512/16KB 0.095× and AES-128-GCM/16KB 0.479× on two Picos, "only in a README, not in rind-results".

## Acceptance criteria

- [ ] All 12 images, 3 fresh-boot runs each, base series then `--set armv6m-sha512-ghash`, published to `results/rp2040/<sha>/` with schema-1 JSON, `RUNS.md`, `RESULTS.md`, and `cargo verify` clean.
- [ ] A `results/rp2040/gaps.toml` entry for every row over 1.10×.
- [ ] The two README A/B ratios (0.095×, 0.479×) reproduced within run spread, or the discrepancy recorded.

## Related PRs

- https://github.com/steelhead99x/rind-agent/pull/55 — docs(skills): add RP2040 bench and ARMv6-M skills
- https://github.com/steelhead99x/rind-hardware/tree/main/rp2040 — rind-hardware rp2040 firmware
- https://github.com/steelhead99x/rind-hardware/pull/8 — feat(xtask): lint RB captures against the timing contract
- https://github.com/steelhead99x/rind-results/pull/35 — verify: recompute cycle medians from the serial capture
