---
id: Q033
title: "Apple DIT is re-checked on the M1 Pro in a quiet run"
priority: P3
hardware: [host:m1pro]
size: S
status: open
claimable_by: maintainer
test_plan_item: 14
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] Apple DIT is re-checked on the M1 Pro in a quiet run

**Priority:** P3 · **Size:** S · **Hardware:** host:m1pro · **Status:** open (maintainer only)

## Why it matters

Test plan item 14: the rind-results m1pro set is flagged noisy; Luke's kixtop shows the per-call DIT cost.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): kixtop AES-128 header protection 34 ns, or 3 ns with DIT held; SHA-256 64 B 90 ns, or 37 ns held.
- rind-results `hosts/m1pro/NOTES.md` "DIT and the gaps table (draft decision)": holding DIT closes 14 of 17 gaps; `hmac-sha256/1350B` 1.10×.
- Upstream https://github.com/kixel-ai/rind/pull/766 (DIT speculation barrier) is not on the fork yet (Q001).

## Acceptance criteria

- [ ] A quiet 3-run set after Q001 confirming whether "DIT held" brings every short row within 10%.

## Related PRs

- none open
