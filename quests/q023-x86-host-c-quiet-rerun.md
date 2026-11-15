---
id: Q023
title: "x86-host-c (Ryzen 5 7535HS) gets a quiet re-run"
priority: P2
hardware: [host:x86-host-c]
size: S
status: open
claimable_by: maintainer
test_plan_item: 10
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] x86-host-c (Ryzen 5 7535HS) gets a quiet re-run

**Priority:** P2 · **Size:** S · **Hardware:** host:x86-host-c · **Status:** open (maintainer only)

## Why it matters

Test plan item 10. The published run is flagged noisy and shows five over-10% losses no upstream host shows.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): aes256-gcm-open/1350B 1.40× ring, p256-ecdsa-verify 1.22× graviola, rsa2048-pss-sign 1.19×, p521-ecdsa-verify 1.14×, rustls-bulk ChaCha 1.12×; Luke's nixos (Zen 3) P-256 ECDSA verify 39.25 µs vs graviola 41.92 µs.

## Acceptance criteria

- [ ] Full `admin/bench-report run` on AC power and idle, 3 runs.
- [ ] Each loss either vanishes (recorded as noise) or repeats (gaps.toml entry, upstream report if warranted).

## Related PRs

- none open
