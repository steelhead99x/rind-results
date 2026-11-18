<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# x86-host-b-zen5: how this set was measured

A mini PC with an AMD Ryzen 7 255 (Zen 5, x86_64, 16 logical CPUs, AVX-512 reported by rind-cpuid), running Oracle Linux Server 10.1. See [facts.txt](facts.txt).

Three runs of rind's `admin/bench-report run` at `d6ca9a756d3d1e71489e61e44c04f33c3c893589` (kixel-ai/rind, unpatched), driven by `cargo xtask bench`, pinned to CPUs 2 and 10, on 2026-11-18 (07:23:55Z to 09:55:03Z). Criterion arguments were `--noplot --warm-up-time 1 --measurement-time 3`. The CPU governor was `performance`. The bench-report noisy flag is false on every run. The timing rows are the criterion estimates. The `build_cost` step of run 3 failed with `host is busy (0.5 of 2 CPUs busy)`; the timing rows of all three runs are status ok.

The published host id is `x86-host-b-zen5`.

SPHINCS+ (SLH-DSA) PR #143 AVX-512 data measured on the same host afterwards is in [SUMMARY.md](SUMMARY.md). It is a separate diagnostic and not part of the median JSON.
