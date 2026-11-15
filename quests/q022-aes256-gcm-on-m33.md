---
id: Q022
title: "AES-256-GCM on the Cortex-M33 is profiled"
priority: P2
hardware: [efr32mg24]
size: S
status: open
claimable_by: maintainer
test_plan_item: 8
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] AES-256-GCM on the Cortex-M33 is profiled

**Priority:** P2 · **Size:** S · **Hardware:** efr32mg24 · **Status:** open (maintainer only)

## Why it matters

Test plan item 8. EFR32MG24 is 1.13×–1.17× RustCrypto on AES-256-GCM while nRF52840 shows the same rows at 1.03×–1.05×, so something is M33-specific.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): gaps.toml: "Not profiled… the extra four rounds cost rind more than RustCrypto".

## Acceptance criteria

- [ ] `aes_gcm` image AES-256 seal/open at 1350 B and 16 KB with a cycle profile, AES-128 as control, and the armv6m `0002` GHASH-by-`muls` patch as an A/B.
- [ ] Under 1.10× after a fix, or a profile that names the cause in gaps.toml.

## Related PRs

- none open
