---
id: Q003
title: "Retire or upstream the fork-only ML-DSA adapter in rind-aws-lc-rs"
priority: P0
hardware: [none]
size: S
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] Retire or upstream the fork-only ML-DSA adapter in rind-aws-lc-rs

**Priority:** P0 · **Size:** S · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

The fork carries its own ML-DSA support in rind-aws-lc-rs, while upstream merged a different one. Two diverging implementations of a published adapter's signature API is exactly the "published adapter that diverges" case rind's m0 milestone treats as urgent, and it blocks Q001.

## Evidence

- Fork-only commits since merge base `381ba596`: `1ce66279 feat(aws-lc-rs): offer ML-DSA in rind-aws-lc-rs`, `f95af317 Restore the rind-aws-lc-rs ML-DSA adapter dropped by f522a7d5`, `296443f7 fix(aws-lc-rs): honor the omitted expanded ML-DSA key`, and seven more `fix/test(aws-lc-rs)` commits; `git diff --stat up/main...main` shows +1011 lines in `rind-aws-lc-rs/tests/keys.rs`.
- Upstream https://github.com/kixel-ai/rind/pull/748 `feat(rind-aws-lc-rs): ML-DSA` and https://github.com/kixel-ai/rind/pull/783 (match each adapter's API to its upstream's) are not on the fork; `rind-aws-lc-rs/src/signature.rs` conflicts in the merge-tree check (Q001).
- `rind/quest/m0/README.md`: m0 holds "a published adapter that diverges from its upstream's API".

## Acceptance criteria

- [ ] For each fork-only ML-DSA commit, the PR states: already covered upstream (drop it), or not covered (keep it and offer it upstream as its own PR per `rind-agent/skills/fork-to-upstream`).
- [ ] After the change the fork's `rind-aws-lc-rs` ML-DSA API matches upstream's (`admin/adapter-api` clean against upstream's expectation).
- [ ] Any extra tests the fork wrote (e.g. `rind-aws-lc-rs/tests/keys.rs` PKCS#8 refusals) that upstream lacks are proposed upstream rather than silently dropped.

## Related PRs

- none open
