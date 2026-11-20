---
id: Q035
title: "HPKE, VRF and blind RSA rows on one MCU"
priority: P3
hardware: [esp32-s3]
size: S
status: open
claimable_by: maintainer
test_plan_item: 16
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] HPKE, VRF and blind RSA rows on one MCU

**Priority:** P3 · **Size:** S · **Hardware:** esp32-s3 · **Status:** open (maintainer only)

## Why it matters

Test plan item 16, once upstream quest `quest/m2/bench/protocol-rows.md` lands host rows to compare against.

## Evidence

- Upstream `quest/m2/bench/protocol-rows.md` ("[S] The protocol primitives have benchmark rows"); upstream https://github.com/kixel-ai/rind/pull/753 added HPKE, blind RSA, OPRF, VRF, SPAKE2+ rows (not on the fork, Q001).

## Acceptance criteria

- [ ] Images for the protocol bench files on one board, 3 runs, published beside the host rows.

## Related PRs

- none open
