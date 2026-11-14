<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# intel-core-ultra-9-275hx: how this set was measured

The machine is a laptop with an Intel Core Ultra 9 275HX (x86_64, 24 logical CPUs), running Oracle Linux Server 10.2 (Linux 7.2.13-1.el10.elrepo.x86_64). See [facts.txt](facts.txt) for the toolchain and the load.

Three runs of rind's `admin/bench-report run` at `d6ca9a756d3d1e71489e61e44c04f33c3c893589` (kixel-ai/rind, unpatched), pinned to CPUs 2 and 3, on 2026-11-13 (17:56:59Z to 19:45:45Z). Criterion arguments were `--noplot --warm-up-time 1 --measurement-time 3`. The bench-report noisy flag is false on every run. `build_cost` failed on each run (run 1: the host was busy; runs 2 and 3: the build waited on other jobs). The timing rows are the criterion estimates.

The published host id is the CPU slug `intel-core-ultra-9-275hx`.

## Power

The laptop was on battery (power-saving governor) for run 1 and part of run 2, then on AC. The published time is the median of all three runs: `cargo verify` checks that median against the raw files.
