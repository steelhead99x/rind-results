---
id: Q028
title: "rind-bench leaves aws-lc-rs out on Xtensa upstream"
priority: P2
hardware: [none]
size: S
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] rind-bench leaves aws-lc-rs out on Xtensa upstream

**Priority:** P2 · **Size:** S · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

The bench files cannot compile for Xtensa while they reference aws-lc-rs, so the firmware carries two patches for it.

## Evidence

- rind-hardware patches `0002` and `0004` gate aws-lc-rs rows on `not(target_arch = "xtensa")`; README item 3: move `aws-lc-rs`/`aws-lc-sys` in `rind-bench/Cargo.toml` under a target-specific dev-dependency table, as graviola's already are ("Not yet a patch").

## Acceptance criteria

- [ ] One upstream PR with the gate and the Cargo.toml move; rind-hardware drops 0002 and 0004 and the images are unchanged otherwise.

## Related PRs

- none open
