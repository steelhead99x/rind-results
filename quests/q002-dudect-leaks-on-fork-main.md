---
id: Q002
title: "Triage the dudect LEAK verdicts in the fork's nightly timing run"
priority: P0
hardware: [none]
size: M
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] Triage the dudect LEAK verdicts in the fork's nightly timing run

**Priority:** P0 · **Size:** M · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

rind's `timing` workflow is the constant-time backstop. Its first scheduled run on the fork's main after the 2026-11-14 sync failed with LEAK verdicts on secret-dependent operations. Until each is shown to be runner noise or fixed, any "constant time" claim for these rows is unsupported.

## Evidence

- https://github.com/steelhead99x/rind/actions/runs/43710298683 (scheduled `timing`, 2026-11-15 03:43 PST): conclusion `failure`. The 4 previous `timing` runs on `main` (2026-11-12) succeeded.
- `dudect (ubuntu-latest, westmere)`: `hmac/sha512/sign` t = -19.72 LEAK; `pbkdf2/sha256/derive` t = -13.41 LEAK; job ends `rind-timing: failed: hmac/sha512/sign, pbkdf2/sha256/derive`. Threshold printed by the tool: "a case leaks when |t| > 10".
- `dudect (ubuntu-latest, native)`: `pbkdf2/sha256/derive` t = -220.67 LEAK; `aead/aes-192-gcm/open-ciphertext` t = +15.35 LEAK.
- Upstream fixes not yet on the fork (Q001) touch nearby code: https://github.com/kixel-ai/rind/pull/764 (SHA-2 contexts stop stalling on store-to-load forwarding), https://github.com/kixel-ai/rind/pull/760 (MXCSR-dependent timing).

## Acceptance criteria

- [ ] Each of the three operations is re-run with `admin/timing dudect` on the same runner class at least twice, and the verdicts are recorded in the PR or a rind-results note.
- [ ] A verdict that repeats is either fixed (with the dudect run attached) or reported upstream to kixel-ai/rind with the reproducer; a verdict that does not repeat is recorded as noise with both runs linked.
- [ ] The next scheduled `timing` run on the fork's main passes, or its remaining failures are each linked to an open upstream report.

## Related PRs

- none open
