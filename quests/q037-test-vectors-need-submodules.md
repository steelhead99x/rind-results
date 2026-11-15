---
id: Q037
title: "A fresh fork checkout runs the vector tests without hidden setup"
priority: P3
hardware: [none]
size: S
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] A fresh fork checkout runs the vector tests without hidden setup

**Priority:** P3 · **Size:** S · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

27 of rind's library tests fail on a checkout without the vector submodules, which reads as a real failure to anyone (or any agent) testing the fork.

## Evidence

- `cargo test -p rind --locked --release` on a fresh clone of the fork: `740 passed; 27 failed; 39 ignored`, every failure `cannot open .../thirdparty/{cavp,cctv,wycheproof}/...` (`rind/src/test.rs:39`, `:98`, `:199`). After `git submodule update --init thirdparty/cavp thirdparty/cctv thirdparty/wycheproof`: `767 passed; 0 failed; 39 ignored`.
- Upstream https://github.com/kixel-ai/rind/pull/714 vendors the vectors (Q001 brings it).

## Acceptance criteria

- [ ] After Q001, a fresh clone passes `cargo test -p rind` with no submodule step; until then TESTING.md on the fork names the step.

## Related PRs

- https://github.com/steelhead99x/rind-results/pull/36 — notes: correctness checks on a Xeon cloud VM, 2026-11-15
