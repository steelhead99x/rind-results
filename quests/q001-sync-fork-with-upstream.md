---
id: Q001
title: "The fork's main takes the 43 upstream commits it is missing"
priority: P0
hardware: [none]
size: M
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] The fork's main takes the 43 upstream commits it is missing

**Priority:** P0 · **Size:** M · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

The fork's `main` lags kixel-ai/rind by commits that fix constant-time and side-channel issues. Every benchmark, hardware patch and fork PR is built on a tree that lacks them, so results and reviews describe code that upstream has already changed.

## Evidence

- `git rev-list --left-right --count up/main...main` in steelhead99x/rind at `3fc24bd1` (2026-11-14 03:08 PST): **43 behind, 93 ahead**; merge base `381ba596`.
- Missing upstream commits include: https://github.com/kixel-ai/rind/pull/759 (RSA private keys load in constant time to the last byte), https://github.com/kixel-ai/rind/pull/766 (speculation barrier after setting DIT on Apple silicon), https://github.com/kixel-ai/rind/pull/760 (Intel MXCSR-dependent timing and DOITM), https://github.com/kixel-ai/rind/pull/696 (`feat!`: rind needs no runtime CPU check), https://github.com/kixel-ai/rind/pull/714 (vendored Wycheproof/CAVP/CCTV vectors), https://github.com/kixel-ai/rind/pull/748 (ML-DSA in rind-aws-lc-rs), https://github.com/kixel-ai/rind/pull/641 (SLH-DSA).
- `git merge-tree --write-tree --name-only main up/main` reports conflicts in 14 paths, including `rind/src/high/signature.rs`, `rind-aws-lc-rs/src/signature.rs`, `rind-aws-lc-rs/tests/ctgrind.rs`, `admin/bench-report`, `admin/adapter-api.drift`, `BENCHMARKS.md`, `TESTING.md`, and three `quest/` files.

## Acceptance criteria

- [ ] A sync PR (as `rind-agent/skills/rind-quest-run` "Sync the fork first" describes) merges `kixel-ai/rind` `main` into the fork; `git rev-list --count main..up/main` is 0 after it lands.
- [ ] Each conflict is resolved toward upstream unless a fork-only commit is still needed, and the PR body lists every fork-only commit kept and why (see Q003 for the ML-DSA adapter).
- [ ] `cargo test -p rind --release --locked`, clippy and fmt pass on the pinned toolchain (rustc 1.98.1 from `flake.nix`).
- [ ] Open fork PRs are rebased or re-checked against the synced main (see Q038).

## Related PRs

- https://github.com/kixel-ai/rind/pull/759 — RSA DER constant time
- https://github.com/kixel-ai/rind/pull/766 — DIT barrier
- https://github.com/kixel-ai/rind/pull/760 — MXCSR/DOITM
