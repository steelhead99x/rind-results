---
id: Q038
title: "Triage the fork's open PRs against upstream"
priority: P3
hardware: [none]
size: S
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] Triage the fork's open PRs against upstream

**Priority:** P3 · **Size:** S · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

14 open PRs on the fork; some overlap upstream work or upstream quests, which wastes review and confuses agents picking work.

## Evidence

- `gh pr list -R steelhead99x/rind --state open` on 2026-11-15 listed 16 PRs; #95 (superseded) and #101 (landed upstream as https://github.com/kixel-ai/rind/pull/750) have since closed.
- #23 overlaps upstream quest `quest/m2/confidence/mutants-adapters.md`; #39 is a fix for an upstream issue (branch `cursor/fix-upstream-632`) and belongs upstream.

## Acceptance criteria

- [ ] Each open PR is labelled keep / close / send upstream with a one-line reason after Q001, and duplicates are closed by the maintainer.

## Related PRs

- https://github.com/steelhead99x/rind/pull/23
- https://github.com/steelhead99x/rind/pull/32
- https://github.com/steelhead99x/rind/pull/39
- https://github.com/steelhead99x/rind/pull/64
- https://github.com/steelhead99x/rind/pull/79
