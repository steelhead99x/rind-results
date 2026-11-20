---
id: Q021
title: "A rustls TLS 1.3 handshake is timed on the EFR32MG24"
priority: P2
hardware: [efr32mg24]
size: L
status: open
claimable_by: maintainer
test_plan_item: 7
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [L] A rustls TLS 1.3 handshake is timed on the EFR32MG24

**Priority:** P2 · **Size:** L · **Hardware:** efr32mg24 · **Status:** open (maintainer only)

## Why it matters

Test plan item 7. The handshake is Luke's headline row (a win on all six hosts) and what MoQ actually runs; no MCU has a handshake row.

## Evidence

- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): nixos full handshake X25519 227.82 µs vs aws-lc-rs 292.13 µs; X25519MLKEM768 260.78 µs. `baremetal/mg24/README.md`: the MG24 has CAS so "rustls (no provider) and rustls-rustcrypto build"; RP2040 cannot (no CAS).
- Blocker: rind must build `no_std` (upstream quest `quest/m2/adoption/no-std.md`; rind-hardware patch 0003 is the stopgap) — see Q025.

## Acceptance criteria

- [ ] Full handshake with X25519 and X25519MLKEM768, Ed25519 and P-256 certs, resumed handshake, and bulk ChaCha20-Poly1305 / AES-128-GCM, rind-rustls vs rustls-rustcrypto, 3 runs.
- [ ] rind-rustls within 1.10× of rustls-rustcrypto or faster, and the X25519MLKEM768/X25519 ratio reported.

## Related PRs

- https://github.com/steelhead99x/rind/pull/84 — feat: build rind without std on bare metal
