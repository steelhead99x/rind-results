---
id: Q025
title: "Reconcile the fork's no_std work with upstream's no-std quest"
priority: P2
hardware: [rp2040]
size: L
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [L] Reconcile the fork's no_std work with upstream's no-std quest

**Priority:** P2 · **Size:** L · **Hardware:** rp2040 · **Status:** open (maintainer only)

## Why it matters

All hardware work rides on rind-hardware's patch stack. Upstream already has a maintainer-written quest for no_std whose design differs from the fork's, so the fork's PR cannot go upstream as-is.

## Evidence

- Upstream `quest/m2/adoption/no-std.md` ("[M] rind builds without std") recommends "No `std` feature"; rind-results `hosts/rp2040/QUEST-DRAFT.md` (historical) proposed a default-on `std` feature.
- rind-hardware `patches/0003` (no_std on `target_os = "none"`) and `0005` (ReuseCache without CAS); on fork main `rind/src/low/generic/reuse_cache.rs:44,49,73` still use `fetch_max`/`compare_exchange`, and `std::thread_local!` remains at `rind/src/low/cpu.rs:129`, `rind/src/low/entry.rs:8`, `rind/src/fips_rng.rs:38`.
- rind-agent `rind-quest-run`: work only quests that exist upstream; claim with `quest start` or a fork draft naming the quest path.

## Acceptance criteria

- [ ] Fork PR #84 is rebased on synced main (Q001), names `quest/m2/adoption/no-std.md`, and follows its Plan (no `std` feature) or records each disagreement as a maintainer question.
- [ ] rind-hardware's 0003/0005 shrink to nothing once it lands; `cargo build -p rind --target thumbv6m-none-eabi` and `thumbv7em-none-eabihf` succeed on the fork's main.

## Related PRs

- https://github.com/steelhead99x/rind/pull/84 — feat: build rind without std on bare metal
- https://github.com/steelhead99x/rind-agent/pull/58 — chore(skills): build the no_std matrix with cargo xtask build-matrix
