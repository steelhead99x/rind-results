---
id: Q039
title: "The fork's Alert workflow stops failing for a missing secret"
priority: P3
hardware: [none]
size: S
status: open
claimable_by: maintainer
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [S] The fork's Alert workflow stops failing for a missing secret

**Priority:** P3 · **Size:** S · **Hardware:** none · **Status:** open (maintainer only)

## Why it matters

Alert fails on every triggering run, so a real nightly failure (like Q002) produces noise instead of a notification.

## Evidence

- https://github.com/steelhead99x/rind/actions/runs/43717105806: job `Discord`, step `Post to Discord`: `sh/alert.sh: line 25: DISCORD_WEBHOOK: DISCORD_WEBHOOK is required`.

## Acceptance criteria

- [ ] Either the fork sets `DISCORD_WEBHOOK` (maintainer decision) or the workflow skips cleanly when the secret is absent on a fork.

## Related PRs

- none open
