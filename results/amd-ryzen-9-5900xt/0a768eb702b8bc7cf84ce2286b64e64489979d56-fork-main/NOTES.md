<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# amd-ryzen-9-5900xt: how this set was measured

The machine is a desktop (gaming-001) with an AMD Ryzen 9 5900XT (x86_64, 16 cores / 32 threads, Zen 3), running Oracle Linux Server 10.3 (Linux 6.12.0-208.112.5.3.el10uek.x86_64).

Three runs of `cargo xtask bench` at steelhead99x/rind fork main `0a768eb702b8bc7cf84ce2286b64e64489979d56` (unpatched; `meta.json` records it as `upstream_commit`), no CPU pinning, on 2026-11-26, against every library in the README: aws-lc-rs, ring, RustCrypto + dalek, graviola, libcrux, OpenSSL and pyca/verified-garbage. Criterion arguments were `--noplot --warm-up-time 1 --measurement-time 3`. Every run ended `ok` (627 results each).

## Load and relaxations

The host shared its CPUs with a vLLM job (about 3 busy CPUs), so `BUSY_SHARE` was relaxed from 0.10 to 0.15, and `build_cost` was skipped because upstream already has this CPU (`nixos`); see [relaxations.json](relaxations.json). Other busy CPUs per run: 3.27, 0.47, 0.39.

## Boost was off

CPU boost was disabled on this host (`cpufreq/boost` = 0, 3.5 GHz), and the governor and boost were left as found. Every library is about 1.40x slower than upstream's `nixos` (same CPU, boost on): the aws-lc-rs control's ratio is 1.417 and rind's is 1.395 over the shared rows. Ratios between libraries are valid; absolute times are not comparable with `nixos`.

## Gaps

The report step refuses to write a summary while a gap over 10% has no decision. [gaps.toml](gaps.toml) records the three gaps on this host (ML-KEM-768 keygen 1.25x verified-garbage, P-384 ECDSA sign 1.11x verified-garbage, SHA-1 at 64 B 1.18x RustCrypto), each carrying over the decision upstream kixel-ai/rind already made (kixel-ai/rind#812 for ML-KEM-768 keygen). `cargo xtask report` and `cargo verify --dir` then passed with 0 mismatches; no benchmark was rerun.
