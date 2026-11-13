<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840 at rind 99b509a (superseded)

The set published until 2026-11-12, rind 99b509a6 with legacy rind-bench c778f64 (later squashed into rind-hardware `8d9c1bb`)'s patches, moved here unchanged when `results/nrf52840/` was rerun at rind 98568332 (legacy rind-bench 911ebd2 (later squashed into rind-hardware `8d9c1bb`)). `raw/` is its captures; the rest is its report as published.

What changed at 98568332: ML-KEM-768 keygen and encaps went from 1.74x and 1.83x libcrux's time to 0.96x and 0.95x, and decaps from 0.82x to 0.46x, closing the gap recorded in this set's gaps.toml (rind's Keccak rewrite, 88ff91fd and the commits around it; the gap's reason had pointed at the 64-bit-lane Keccak). `admin/bench-report` also gained SHA-3 operations, recorded in the new gaps.toml. rind's P-256 rows got 6-10% faster; other rows moved by 3% or less.
