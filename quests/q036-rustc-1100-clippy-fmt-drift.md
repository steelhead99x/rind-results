---
id: Q036
title: "rind is clean on rustc 1.100's clippy and rustfmt"
priority: P3
hardware: [none]
size: S
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] rind is clean on rustc 1.100's clippy and rustfmt

**Priority:** P3 · **Size:** S · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

The box's `stable` moved to rustc/cargo 1.100.0 on 2026-11-15; rind pins 1.98.1, so CI is fine today, but the next toolchain bump will fail lint.

## Evidence

- `cargo clippy -p rind --locked --all-targets -- -D warnings` on cargo 1.100.0: `error: the loop variable i is used to index self.words` at `rind/src/low/posint.rs:118` (`needless_range_loop`).
- `cargo fmt --all --check` on 1.100.0 reports diffs in the `rsaz_routine!` invocations; on `+1.98.1` both clippy and fmt pass (exit 0).

## Acceptance criteria

- [ ] `posint.rs:118` rewritten or allowed with a reason (keeping constant-time behaviour), and rustfmt output stable across 1.98.1 and 1.100.0, proposed upstream.

## Related PRs

- none open
