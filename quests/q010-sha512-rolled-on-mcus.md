---
id: Q010
title: "SHA-512 runs a rolled loop on ESP32-S3, then on Cortex-M4/M33"
priority: P1
hardware: [esp32-s3, efr32mg24, nrf52840]
size: M
status: open
claimable_by: maintainer
test_plan_item: 1
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# [M] SHA-512 runs a rolled loop on ESP32-S3, then on Cortex-M4/M33

**Priority:** P1 · **Size:** M · **Hardware:** esp32-s3, efr32mg24, nrf52840 · **Status:** open (maintainer only)

## Why it matters

Test plan item 1 ("Do first"). This is the largest measured gap anywhere: on ESP32-S3 rind's SHA-512 is 32.56×–32.96× ring's time. Ed25519 hashes with SHA-512, so it also taxes every signature. Luke's CPU-tier chart cannot show it (SHA-384/512: portable 1.04, armv7 1.00), so only hardware can drive the fix.

## Evidence

- rind-results `results/gaps.toml` (sha512 on esp32s3): "about 22,000 Xtensa instructions per block, far past the 16 KB (or 32 KB) instruction cache, so every block streams its code from DIO flash ... A fix would be a rolled compression loop on targets without 64-bit registers, or placing it in IRAM."
- Test plan ([`quests/hardware-test-plan.md`](hardware-test-plan.md)): ESP32-S3 sha512 64 B 3.34 ms vs ring 102.66 µs; EFR32MG24 1.74×–1.78× ring; nRF52840 1.18×–1.20× ring (rind-results `results/nrf52840/gaps.toml`: ring's sha512-armv4 is 0.84× rind's cycles).
- rind-hardware `patches/armv6m-sha512-ghash/0001` (rolled SHA-512) is gated to ARMv6-M today; the plan quotes sha512/16KB at 0.095× on the Pico.

## Acceptance criteria

- [ ] `sha2` and `ed25519` images, base vs. rolled SHA-512 (cfg widened to Xtensa, then thumbv7em/thumbv8m), 3 fresh-boot runs each with KATs passing first, published under `results/<board>/<sha>-<label>/` and `cargo verify` clean.
- [ ] ESP32-S3 sha512 within 1.10× of ring on all 3 runs (ring: 102.66 µs at 64 B, 12.42 ms at 16 KB), with flash clock and icache size recorded.
- [ ] On M4/M33 an A/B ratio under 0.90 with every patched run beating every base run; if it regresses there, the cfg stays Xtensa/ARMv6-M only and the result is recorded.

## Related PRs

- https://github.com/steelhead99x/rind/pull/69 — perf(portable): pure-Rust ESP32-S3 SHA-256 in IRAM, rolled SHA-512 and HChaCha
- https://github.com/steelhead99x/rind/pull/94 — perf(portable): place ESP32-S3 rolled SHA-512 in IRAM
- https://github.com/steelhead99x/rind-hardware/pull/7 — docs(esp32): build both rind#69 images and a board runbook
- https://github.com/steelhead99x/rind/pull/102 — feat: drop libcrux from the rind crate — overlaps: the portable SHA-2 is libcrux-sha2's code (gaps.toml)
