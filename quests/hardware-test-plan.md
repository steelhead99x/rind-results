<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# rind hardware test plan: what to test, in what order

Written 2026-11-15 (America/Los_Angeles) from a read of Luke Curley's upstream report
(`rind/BENCHMARKS.md`, the `benchmarks/*.svg` charts (rendered to PNG for reading),
`benchmarks/gaps.toml`, `PLATFORMS.md`, `rind-bench/benches/`, `quest/`) against what
`rind-results` (README, `hosts/README.md`, `results/*/RESULTS.md`, `results/*/gaps.toml`,
`media/charts/*-rind-vs-fastest.png`) and `steelhead99x/rind-hardware` already measure.
All numbers below are quoted from those files. Nothing here is a new measurement.

## What Luke's charts say, in short

- **Overview** (6 hosts, 7 libraries): rind is "the fastest, or within 10% of it, in 331 of 354 times".
  rind wins every rustls handshake row on every host (nixos: full handshake X25519 227.82 µs against
  aws-lc-rs 292.13 µs and ring 328.46 µs; X25519MLKEM768 260.78 µs against aws-lc-rs 331.87 µs).
- **Where rind isn't green on the overview:**
  - `kixtop` (Apple M4): the per-call DIT toggle. AES-128 header protection is 34 ns, or 3 ns with
    DIT held, and SHA-256 64 B is 90 ns, or 37 ns held. gaps.toml puts the cost at about 30 to 55 ns a call
    and covers it with `Dit::hold()`. Held, everything is within 10% except HMAC-SHA256 64 B at 1.10×.
  - `ovh-iad` and `ovh-sea` (Haswell VPS with ADX hidden): Ed25519 verify at 1.29× to 1.30× RustCrypto, closed by #415.
  - SHA-256 64 B at 1.17× on nixos, narrowed by #352 and closed by #394.
- **Rows Luke never times:** ML-DSA-65 and SHA3 are "Not timed" on every one of his six hosts.
- **CPU-tier chart** (rind over the fastest of aws-lc-rs and ring, nightly on GitHub runners):
  - Level or ahead on every native tier, including `armv7` and `no crypto`. The worst armv7 cell is P-521 at 1.08.
  - Clearly behind in one column only, `portable`: AES-GCM 7.26, AES-CTR 9.20, AES-CBC 9.38,
    rind-rustls 1 MB transfer 7.16, RSA signatures 4.36, RSA-OAEP 3.18, JWT 3.48, rind-rustls handshakes
    3.61, P-521 2.79, P-256 1.41, P-384 1.16.
  - Ahead in that same portable column: ChaCha20-Poly1305 0.90, X25519 0.80, Ed25519 0.69,
    ML-KEM-768 0.62, SHA-256 0.88, HKDF 0.98.
- **Why the tier chart doesn't predict microcontrollers:**
  1. The portable column runs on a 64-bit x86 host against aws-lc's C. PLATFORMS.md: "RSA and the
     NIST curves run on 64-bit limbs on 64-bit targets … and on 32-bit limbs elsewhere" (#614).
     #621's bitsliced AES is aws-lc's `aes_nohw.c` "in its 64-bit configuration".
  2. The armv7 column runs aws-lc's ARM-state NEON assembly, which no Cortex-M core can execute
     (nRF52840 and EFR32MG24 gaps.toml: "its 32-bit Arm assembly is ARM-state NEON that an M-profile core lacks").
  3. aws-lc and OpenSSL can't be built for Xtensa or bare metal, so on a board the competition is ring and RustCrypto, not aws-lc.

  So the boards are the only data on 32-bit-limb, M-profile, and Xtensa code, and they disagree
  with the tier chart in both directions:
  - **AES-GCM** looks worst on the tier chart (7.26) but is a win or a tie on boards
    (ESP32-S3 0.53× to 0.91×; nRF52840 0.89× to 1.05×; EFR32MG24 0.84× to 1.17×).
  - **ChaCha20-Poly1305 and SHA-2** look fine on the tier chart but are the main board losses.

## What the boards already show (rind's time over the fastest other Rust library)

| Board | Wins | Losses (over 1.10×) |
|---|---|---|
| ESP32-S3 (Xtensa LX7, 240 MHz) | AES-GCM 0.53×–0.91×, ML-KEM 0.28×–0.51×, P-256 ECDH 0.59×, ECDSA verify 0.60×, QUIC HP 0.65×–0.86× | **SHA-512 32.56×–32.96× ring** (64 B: 3.34 ms vs 102.66 µs), HKDF 5.50×–6.93×, XChaCha 32 B 7.50×, SHA-256 64 B 2.36×, HMAC 64 B 2.20×, ChaCha20-Poly1305 1.26×–1.45× |
| nRF52840 (Cortex-M4F, 64 MHz) | P-256 0.40×–0.43× ring, ML-KEM decaps 0.46×, QUIC HP AES 0.73× | ChaCha20-Poly1305 1.74×–1.79× ring, SHA-256 1.43×–1.62×, HMAC 1.22×–1.51×, HKDF 1.20×–1.32×, SHA-512 1.18×–1.20×, XChaCha 16 KB 1.17×, SHA3 64 B 1.11× |
| EFR32MG24 (Cortex-M33F, 78 MHz) | P-256 0.44×–0.46× ring, ML-KEM decaps 0.47× | SHA-512 1.74×–1.78× ring, ChaCha20-Poly1305 1.55×–1.61×, SHA-256 1.57×–1.60×, XChaCha 32 B 1.58×, HKDF 1.37×–1.42×, HMAC 1.36×–1.52×, AES-256-GCM 1.13×–1.17× RustCrypto, SHA3-512 64 B 1.12× |
| Jetson Orin Nano (A78AE) | everything ≤1.00× except one row; P-521 ECDSA sign 0.45×, P-384 ECDSA sign 0.53× | sha256/64B 1.07× RustCrypto (under the 10% line) |
| x86-host-c (Ryzen 5 7535HS, **noisy run**) | P-384/P-521 0.45×–0.60× | aes256-gcm-open/1350B 1.40× ring, p256-ecdsa-verify 1.22× graviola, rsa2048-pss-sign 1.19× aws-lc-rs, p521-ecdsa-verify 1.14×, rustls-bulk ChaCha 1.12× |

**Not measured on any microcontroller:**
- rustls handshake and bulk, RSA, P-384, P-521, ML-DSA, HPKE/VRF/blind RSA. Those bench files exist
  in `rind-bench/benches/`, but only 12 images are built: aes_gcm, chacha20poly1305, xchacha20poly1305,
  cipher, x25519, p256, ed25519, sha2, sha3, hmac, hkdf, mlkem768.
- Any RP2040 timing. It builds and its KATs pass, but no timing is published.
- The poly1305-32 and armv6m-sha512-ghash patch sets on the boards they weren't written for.

The microcontroller sets ran at rind `98568332` (2026-11-12). Upstream has moved on since:
#700 P-521, #702 portable RSA Montgomery, and 18 commits under `rind/src` in total.

## Rules for every run (what counts as a result)

- Same protocol as the published sets:
  - Known-answer checks pass on the board before any timing.
  - 3 fresh-boot runs per image, and the medians published with run-to-run spread.
  - `cargo verify` recomputes every number from raw.
- **Gap line = 10%**, as upstream gaps.toml and `results/gaps.toml` use it. A row only "closes" a gap
  if rind is within 1.10× of the fastest other library on all 3 runs.
  An improvement counts if the A/B ratio (patched over base, same board, same pin) is under 0.90 and every
  run of the patched image beats every run of the base image.
- **Short inputs (32/64 B):** hosts/README.md shows a relink at a different alignment moves some short
  rows by up to 0.3×, so any short-input win or loss must hold across the three-alignment relink
  (`--pad 1031`, `--pad 1039`).
- **ESP32-S3:** record flash clock (80 MHz) and icache size (16 KB default vs 32 KB) on every run. The 40 MHz
  slip slowed rows by up to 1.9×.

---

## 1) Do first

### 1. SHA-512 rolled loop on ESP32-S3, then on M4/M33
- **Test:** `sha2` image (sha512 64 B/1350 B/16 KB), plus `ed25519` because Ed25519 hashes with SHA-512.
  - Base: current portable SHA-512.
  - Patched: the rolled SHA-512 from rind-hardware `patches/armv6m-sha512-ghash/0001`, which is
    gated to ARMv6-M today, so widen the cfg to Xtensa, then to thumbv7em/thumbv8m.
- **Boards:** ESP32-S3 first, then EFR32MG24 and nRF52840.
- **Why first:**
  - This is the largest measured gap anywhere: 32.56×–32.96× ring on ESP32-S3.
  - gaps.toml blames code size ("about 22,000 Xtensa instructions per block, far past the 16 KB (or 32 KB)
    instruction cache"), not compute. The same patch measured sha512/16KB at 0.095× on the Pico.
  - EFR32MG24 also trails ring 1.74×–1.78× and nRF52840 1.18×–1.20×.
  - Luke's tiers never show it (SHA-384/512: portable 1.04, armv7 1.00), so only hardware can drive this fix.
- **Meaningful:**
  - ESP32-S3 sha512 within 1.10× of ring (ring: 102.66 µs at 64 B, 12.42 ms at 16 KB).
  - On M4/M33, an A/B ratio under 0.90. If it is a regression there, keep the cfg Xtensa/ARMv6-M only.

### 2. Poly1305-32 across all four MCUs (ChaCha20-Poly1305, XChaCha20-Poly1305)
- **Test:** `chacha20poly1305` and `xchacha20poly1305` images, base against `patches/poly1305-32`, every size row.
- **Boards:**
  - EFR32MG24 and RP2040: not yet A/B'd.
  - nRF52840 and ESP32-S3: repeat at the new pin, since their A/B is in `diag/` only and not in the published tables.
- **Why:**
  - ChaCha20-Poly1305 is the second-biggest steady loss on every board: 1.74×–1.79× nRF, 1.55×–1.61× EFR,
    1.26×–1.45× ESP versus ring.
  - It is the AEAD rind-rustls prefers wherever `has_aes_hardware()` is false, which includes every MCU (PLATFORMS.md).
  - Luke's tier chart shows ChaCha20-Poly1305 *winning* in portable (0.90) and armv7 (0.99), so his charts hide this.
  - The nRF A/B already shows 0.698 at 16 KB (895,189 to 624,562 cycles).
- **Meaningful:**
  - Published per-board ratio against ring after the patch.
  - Whatever remains above 1.10× is ChaCha20 itself: ring runs OpenSSL's Thumb-2 chacha-armv4, as the
    nRF/EFR gap notes say. That becomes the "M-profile ChaCha20" quest.
  - On RP2040 (no `umull`), record whether 32-bit limbs still help.

### 3. SHA-256, HMAC, HKDF short paths (TLS key schedule)
- **Test:**
  - `sha2`, `hmac`, `hkdf` images, base against `armv6m-sha512-ghash/0004` (SHA-256 below 2 KB by
    8-round loops in `.text.rind_sha256_hot`), widened beyond ARMv6-M.
  - On ESP32-S3, also run the 32 KB-icache configuration as a published variant, not only as a diag.
- **Boards:** ESP32-S3, nRF52840, EFR32MG24.
- **Why:**
  - ESP32-S3 losses: HKDF 5.50×–6.93×, SHA-256 64 B 2.36×, HMAC 64 B 2.20×.
  - nRF and EFR: SHA-256 1.43×–1.62×, HMAC up to 1.52×, HKDF up to 1.42×.
  - Luke's tiers show SHA-256 portable 0.88 and HKDF 0.98, so his charts can't see this.
  - On ESP32-S3, the gap is instruction-cache misses. gaps.toml: with 32 KB icache, "hmac-sha256 64B 115.6 µs
    against ring's 140.3 µs".
  - HKDF runs on every TLS handshake, so this feeds item 7.
- **Meaningful:**
  - ESP32-S3 HKDF and HMAC within 1.10× at default 16 KB icache, or a documented "use 32 KB icache" recommendation with numbers.
  - On M4/M33, the remaining gap is ring's Thumb-2 sha256-armv4 (0.62×–0.64× rind's cycles). Close it only
    if a Thumb-2 SHA-256 lands; otherwise record it as an accepted gap with that reason.

### 4. RP2040 (Cortex-M0+) first published timing set
- **Test:** all 12 images, three runs each, base series and then `--set armv6m-sha512-ghash`. Publish to
  `results/rp2040/<sha>/`.
- **Board:** Raspberry Pi Pico (RP2040).
- **Why:**
  - It builds and KATs pass, but there are zero published numbers.
  - The Pico is the worst case for 32-bit limbs (no `umull`, no CAS), which no tier on Luke's chart
    approximates (#614 helps only 64-bit targets).
  - The patch set already claims sha512/16KB 0.095× and AES-128-GCM/16KB 0.479× on two Picos, but only in a
    README, not in rind-results.
- **Meaningful:**
  - A full table with gaps.toml entries for every row over 1.10×.
  - Those two A/B ratios reproduced within run spread.

## 2) Next

### 5. Re-baseline every MCU at the current upstream pin
- **Test:** the full 12-image set on ESP32-S3, nRF52840, EFR32MG24 at the current rind pin
  (rind-hardware's submodule is at `d6ca9a75`; upstream main is later).
- **Why:** every published MCU set is `98568332`. Items 1–4 need a clean "base" at the same pin as their
  patches, or the A/B ratios mix two variables.
- **Meaningful:** no row moves more than run spread versus `98568332` unless a commit explains it. Any new
  over-10% row gets a gaps.toml entry.
- Run this alongside 1–4. It is the base half of each of their A/Bs, which is why it isn't listed first.

### 6. P-384 and P-521 (and RSA-2048 verify) on 32-bit limbs
- **Test:**
  - Add `p384` and `p521` images (ECDH, ECDSA sign/verify) from the existing bench files.
  - Add `rsa_pub` (verify only; signing is likely too slow and RAM-heavy to be useful first).
- **Boards:** EFR32MG24 and nRF52840 (256 KiB RAM), then ESP32-S3.
- **Why:**
  - These rows are where the tier chart's portable column loses: P-521 2.79, P-256 1.41, P-384 1.16, RSA signatures 4.36.
  - Yet MCU P-256 is rind's biggest board win: 0.40×–0.46× ring on Cortex-M; ESP32-S3 ECDH 0.59× and verify 0.60× RustCrypto.
  - Nobody has measured whether the 32-bit-limb P-384/P-521/RSA code follows P-256 or the tier chart.
  - Jetson shows P-384/P-521 as rind's strongest aarch64 wins (P-521 ECDSA sign 0.45×, P-384 sign 0.53×).
- **Meaningful:** rind at or under 1.10× of RustCrypto's p384/p521 (the competitor likely to build bare
  metal) and ring (P-384 only). Any loss gets a gaps.toml entry naming 32-bit limbs.

### 7. rustls TLS 1.3 handshake on EFR32MG24
- **Test:**
  - rustls full handshake with X25519 and X25519MLKEM768, Ed25519 and P-256 certs, the resumed handshake,
    and bulk transfer with ChaCha20-Poly1305 and AES-128-GCM.
  - rind-rustls against rustls-rustcrypto.
- **Board:** EFR32MG24 (`baremetal/mg24/README.md`: it has CAS, so "rustls (no provider) and
  rustls-rustcrypto build"). The RP2040 cannot run this (no CAS).
- **Why:**
  - The handshake is Luke's headline row, a win on all six hosts and on Jetson (≤1.00× graviola).
  - It is what MoQ actually runs, and no MCU has a handshake row.
  - The board data suggests the post-quantum hybrid should cost little on MCUs, but that is an inference
    that needs measuring. On ESP32-S3, ML-KEM-768 keygen is 7.00 ms and decaps 5.20 ms against
    X25519 28.29 ms. On nixos the hybrid adds 260.78 to 227.82 µs.
- **Blocker:** needs `rind` to build `no_std` (quest `m2/adoption/no-std.md`); rind-hardware's patch 0003 is the stopgap.
- **Meaningful:**
  - rind-rustls full handshake within 1.10× of rustls-rustcrypto, or faster.
  - The X25519MLKEM768 over X25519 handshake ratio reported per board.

### 8. AES-256-GCM on Cortex-M33
- **Test:** `aes_gcm` image, AES-256 seal/open at 1350 B and 16 KB, with a cycle profile. Also run AES-128
  as control and the armv6m `0002` GHASH-by-`muls` patch as an A/B.
- **Board:** EFR32MG24 (compare nRF52840).
- **Why:** EFR is at 1.13×–1.17× RustCrypto. gaps.toml: "Not profiled… the extra four rounds cost rind more
  than RustCrypto". nRF52840 shows the same rows at only 1.03×–1.05×, so something is M33-specific.
- **Meaningful:** under 1.10× after a fix, or a profile that names the cause.

### 9. Constant-time checks on the MCU multipliers
- **Test:**
  - Timing-variance runs (fixed vs random secret inputs, dudect-style) on X25519, P-256 ECDH/ECDSA sign,
    Poly1305-32 and ML-KEM decaps.
  - Also time `umull` with small vs large operands.
- **Boards:** EFR32MG24 (Cortex-M33), RP2040 (M0+), ESP32-S3, nRF52840.
- **Why:**
  - PLATFORMS.md lists cores where rind is *not* constant-time (A53, some PowerPC, C906, XScale…) but says
    nothing about Cortex-M or Xtensa.
  - `baremetal/mg24/README.md`: "Arm publishes no M33 cycle table … constant time is unverified until measured".
  - The poly1305-32 README claims constant time on ESP32-S3 and M4 "not Cortex-M3".
  - Speed numbers on these boards are only worth publishing if the code is constant-time there.
- **Meaningful:** a t-statistic under the usual 4.5 threshold over at least 10^6 traces per function, or a
  documented early-terminating multiply to add to PLATFORMS.md.

### 10. Quiet re-run on x86-host-c
- **Test:** full `admin/bench-report run` on AC power and idle.
- **Why:**
  - The published run is flagged noisy and shows five over-10% losses that no upstream host shows:
    aes256-gcm-open/1350B 1.40× ring, p256-ecdsa-verify 1.22× graviola, rsa2048-pss-sign 1.19×,
    p521-ecdsa-verify 1.14×, rustls-bulk ChaCha 1.12×.
  - Luke's nixos (Zen 3) has P-256 ECDSA verify at 39.25 µs against graviola 41.92 µs.
- **Meaningful:** each loss either vanishes (record it as noise) or repeats in 3 quiet runs (a real Zen 3+
  gap, which needs a gaps.toml entry and possibly an upstream report).

## 3) Later

11. **ML-DSA-65 on MCUs.**
    - Board: ESP32-S3 (PSRAM), then EFR32MG24 if RAM allows.
    - Luke's report never times ML-DSA ("Not timed" on all six hosts). rind-results has it only on
      x86/aarch64 (Jetson ML-DSA 0.84×–0.91× aws-lc-rs).
    - Meaningful: keygen/sign/verify against libcrux's and RustCrypto's ML-DSA, plus peak stack/heap.
12. **Hardware-engine baselines.**
    - Boards: EFR32MG24 Secure Engine (AES-GCM, ChaCha20-Poly1305, SHA-2, ECDSA/ECDH, X25519, Ed25519)
      and nRF52840 CryptoCell, as a non-competing baseline row like `esp32s3-hw`.
    - On ESP32-S3 the engine is far ahead: AES-128-GCM 16 KB 4.43 ms against rind's 19.96 ms; SHA-256 16 KB
      185.76 µs against 5.42 ms.
    - Gives users a "software vs engine" number.
13. **aarch64 without the crypto extension on real silicon** (Pi 4 / CM4 / Zero 2 W).
    - Luke's "no crypto" tier is level or better on every row, but it's a feature-denied EPYC/Neoverse runner.
    - PLATFORMS.md flags the Cortex-A53 64-bit multiply as variable-time (Zero 2 W, Pi 3).
    - Meaningful: confirm the tier numbers on a real Pi 4, and run item 9's CT test on an A53.
14. **Apple DIT on M1 Pro, quiet run.**
    - The rind-results m1pro set is flagged noisy.
    - Luke's kixtop shows the per-call DIT cost; check that "DIT held" brings every short row within 10% on M1 too.
15. **SHA3 64 B per-call overhead** on nRF52840 and EFR32MG24 (1.11×–1.12× libcrux; about 2.0 kc a call
    more). Accepted as small; recheck only after items 1–3 change the hash plumbing.
16. **Protocol rows (HPKE, VRF, blind RSA)** on one MCU, once upstream quest `m2/bench/protocol-rows.md`
    lands host rows to compare against.

## Order at a glance

| # | Test | Board(s) | Driver |
|---|---|---|---|
| 1 | SHA-512 rolled | ESP32-S3, then EFR32MG24, nRF52840 | 32.56×–32.96× ring on ESP32-S3; invisible on Luke's tiers |
| 2 | Poly1305-32 / ChaCha20-Poly1305 | all four MCUs | 1.26×–1.79× ring; rind-rustls' MCU default AEAD |
| 3 | SHA-256/HMAC/HKDF short path, 32 KB icache | ESP32-S3, nRF52840, EFR32MG24 | HKDF up to 6.93×; TLS key schedule |
| 4 | First RP2040 timing set | Pico | builds and passes KATs, nothing published; 32-bit-limb worst case |
| 5 | Re-baseline at current pin | ESP32-S3, nRF52840, EFR32MG24 | published sets are `98568332` |
| 6 | P-384 / P-521 / RSA verify | EFR32MG24, nRF52840, ESP32-S3 | tier chart's portable losses vs. MCU P-256 wins |
| 7 | rustls handshake (X25519, X25519MLKEM768) | EFR32MG24 | Luke's headline row; no MCU data |
| 8 | AES-256-GCM profile | EFR32MG24 | 1.13×–1.17× vs 1.03×–1.05× on M4 |
| 9 | Constant-time on MCU multipliers | EFR32MG24, RP2040, ESP32-S3, nRF52840 | PLATFORMS.md silent on Cortex-M/Xtensa |
| 10 | Quiet x86-host-c re-run | x86-host-c | five losses only on a noisy run |
| 11–16 | ML-DSA, HW engines, Pi 4/A53, M1 DIT, SHA3 64 B, protocol rows | various | coverage |
