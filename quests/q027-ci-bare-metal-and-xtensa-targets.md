---
id: Q027
title: "CI compile-checks rind for the board targets"
priority: P2
hardware: [none]
size: M
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] CI compile-checks rind for the board targets

**Priority:** P2 · **Size:** M · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

Without a compile check, every upstream change can silently break the boards, and the break is found only when someone flashes one.

## Evidence

- rind-results `hosts/rp2040/QUEST-DRAFT.md` proposes compile checks for `thumbv6m-none-eabi`, `thumbv7em-none-eabihf`, `thumbv8m.main-none-eabihf`, `riscv32imc-unknown-none-elf`, `riscv32imac-unknown-none-elf`; upstream `no-std.md` asks for one bare-metal target per assembly arch.
- Fork PR #85 adds Xtensa linting; it is still a draft.

## Acceptance criteria

- [ ] One CI job (fork first, then proposed upstream with Q025) builds `rind` for thumbv6m, thumbv7em and xtensa-esp32s3; it fails on a regression.
- [ ] No new third-party actions beyond what rind's CI already uses.

## Related PRs

- https://github.com/steelhead99x/rind/pull/85 — ci: lint rind on the ESP32's Xtensa targets
- https://github.com/steelhead99x/rind-hardware/pull/5 — chore: add SECURITY.md, a host-only CI workflow, and pin its actions
- https://github.com/steelhead99x/rind-agent/pull/58 — build the no_std matrix with cargo xtask build-matrix
