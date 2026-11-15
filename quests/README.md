<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# Quests

Prioritized work items from a review of [steelhead99x/rind](https://github.com/steelhead99x/rind) at `3fc24bd1` (2026-11-14), its open PRs, rind-hardware, rind-agent, and the [hardware test plan](hardware-test-plan.md), done on 2026-11-15.

**These are public, but only the maintainer claims them for now** (`claimable_by: maintainer`). Comments and result PRs are welcome.

These are proposals for this fork, not quests in rind's own `quest/` tree. rind's quests are written by the maintainer upstream (rind-agent `rind-quest-run`: "Never invent, write, or add a quest file"), so anything here meant for upstream goes there as a question or a `/quest-plan`, not as a copied file. Where an upstream quest already exists, the quest file names it.

## Priorities

- **P0** — security, correctness, or divergence that blocks other work.
- **P1** — performance and hardware-benchmarking gaps; the test plan's "Do first" items (1–5), plus its constant-time item (9), raised because it is a security question.
- **P2** — the plan's "Next" items, portability / no_std / embedded blockers, and CI for board targets.
- **P3** — the plan's "Later" items, docs, tests, toolchain drift, and cleanup.

Hardware quests keep the plan's order (`test_plan_item`). Every number in a quest is quoted from a file, a command, or a CI log it names; nothing here is a new measurement.

## Format

One markdown file per quest, `qNNN-<slug>.md`, with YAML frontmatter:

```yaml
id: Q010
title: "..."
priority: P1          # P0..P3
hardware: [esp32-s3]  # esp32-s3, nrf52840, rp2040, efr32mg24, host:<name>, or none
size: M               # S, M, L
status: open          # open, claimed, done, dropped
claimable_by: maintainer
test_plan_item: 1     # optional: item number in hardware-test-plan.md
```

The body follows rind's quest style (`# [size] title`) with **Why it matters**, **Evidence**, **Acceptance criteria**, and **Related PRs**.

## Index

3 P0 · 6 P1 · 9 P2 · 10 P3

| Priority | ID | Quest | Size | Hardware | Plan item |
|---|---|---|---|---|---|
| P0 | Q001 | [The fork's main takes the 43 upstream commits it is missing](q001-sync-fork-with-upstream.md) | M | none | — |
| P0 | Q002 | [Triage the dudect LEAK verdicts in the fork's nightly timing run](q002-dudect-leaks-on-fork-main.md) | M | none | — |
| P0 | Q003 | [Retire or upstream the fork-only ML-DSA adapter in rind-aws-lc-rs](q003-fork-mldsa-adapter-vs-upstream.md) | S | none | — |
| P1 | Q010 | [SHA-512 runs a rolled loop on ESP32-S3, then on Cortex-M4/M33](q010-sha512-rolled-on-mcus.md) | M | esp32-s3, efr32mg24, nrf52840 | 1 |
| P1 | Q011 | [Poly1305 in 26-bit u32 limbs is A/B'd on all four MCUs and offered upstream](q011-poly1305-32-all-mcus.md) | M | esp32-s3, nrf52840, efr32mg24, rp2040 | 2 |
| P1 | Q012 | [SHA-256, HMAC and HKDF short inputs close the gap on MCUs (TLS key schedule)](q012-sha256-hmac-hkdf-short-path.md) | M | esp32-s3, nrf52840, efr32mg24 | 3 |
| P1 | Q013 | [RP2040 (Cortex-M0+) gets its first published timing set](q013-rp2040-first-timing-set.md) | M | rp2040 | 4 |
| P1 | Q014 | [Re-baseline every MCU at the current rind pin](q014-rebaseline-mcus-current-pin.md) | M | esp32-s3, nrf52840, efr32mg24 | 5 |
| P1 | Q015 | [Constant time is measured on the MCU multipliers (M33, M0+, Xtensa, M4)](q015-constant-time-on-mcu-multipliers.md) | L | efr32mg24, rp2040, esp32-s3, nrf52840 | 9 |
| P2 | Q020 | [P-384, P-521 and RSA-2048 verify are timed on 32-bit limbs](q020-p384-p521-rsa-verify-on-mcus.md) | M | efr32mg24, nrf52840, esp32-s3 | 6 |
| P2 | Q021 | [A rustls TLS 1.3 handshake is timed on the EFR32MG24](q021-rustls-handshake-on-mcu.md) | L | efr32mg24 | 7 |
| P2 | Q022 | [AES-256-GCM on the Cortex-M33 is profiled](q022-aes256-gcm-on-m33.md) | S | efr32mg24 | 8 |
| P2 | Q023 | [x86-host-c (Ryzen 5 7535HS) gets a quiet re-run](q023-x86-host-c-quiet-rerun.md) | S | host:x86-host-c | 10 |
| P2 | Q024 | [An M-profile (Thumb-2) ChaCha20 closes the remaining ChaCha gap](q024-m-profile-chacha20.md) | L | nrf52840, efr32mg24 | — |
| P2 | Q025 | [Reconcile the fork's no_std work with upstream's no-std quest](q025-no-std-reconcile-with-upstream-quest.md) | L | rp2040 | — |
| P2 | Q026 | [rind builds for Xtensa without the MD5 asm patch](q026-xtensa-md5-build.md) | S | esp32-s3 | — |
| P2 | Q027 | [CI compile-checks rind for the board targets](q027-ci-bare-metal-and-xtensa-targets.md) | M | none | — |
| P2 | Q028 | [rind-bench leaves aws-lc-rs out on Xtensa upstream](q028-bench-aws-lc-rows-off-xtensa.md) | S | none | — |
| P3 | Q030 | [ML-DSA-65 is timed on an MCU](q030-ml-dsa-on-mcus.md) | M | esp32-s3, efr32mg24 | 11 |
| P3 | Q031 | [EFR32MG24 Secure Engine and nRF52840 CryptoCell baseline rows](q031-hardware-engine-baselines.md) | M | efr32mg24, nrf52840 | 12 |
| P3 | Q032 | [aarch64 without the crypto extension is checked on a real Pi 4 / A53](q032-aarch64-no-crypto-real-silicon.md) | M | host:pi4 | 13 |
| P3 | Q033 | [Apple DIT is re-checked on the M1 Pro in a quiet run](q033-m1-dit-quiet-run.md) | S | host:m1pro | 14 |
| P3 | Q034 | [SHA-3 64 B per-call overhead is rechecked on M4/M33](q034-sha3-64b-overhead.md) | S | nrf52840, efr32mg24 | 15 |
| P3 | Q035 | [HPKE, VRF and blind RSA rows on one MCU](q035-protocol-rows-on-mcu.md) | S | esp32-s3 | 16 |
| P3 | Q036 | [rind is clean on rustc 1.100's clippy and rustfmt](q036-rustc-1100-clippy-fmt-drift.md) | S | none | — |
| P3 | Q037 | [A fresh fork checkout runs the vector tests without hidden setup](q037-test-vectors-need-submodules.md) | S | none | — |
| P3 | Q038 | [Triage the fork's open PRs against upstream](q038-fork-pr-triage.md) | S | none | — |
| P3 | Q039 | [The fork's Alert workflow stops failing for a missing secret](q039-alert-workflow-webhook.md) | S | none | — |
