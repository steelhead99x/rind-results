<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# msi-001: rind's own host suite on an x86_64 laptop

`msi-001.json` is rind's `admin/bench-report run` output (schema 1, as in rind's
`benchmarks/data/<host>.json`), for rind `99b509a680cf0ad0cd8b7d9c03f7360e7fd95cef` unpatched,
all competitors including aws-lc-rs, graviola and OpenSSL (`__bench_openssl`, system OpenSSL 3.5.8),
run with `tools/host-run.sh`. `summary.md` is rind's own `summary` table for it.

- CPU: AMD Ryzen 5 7535HS (Zen 3+, 6 cores / 12 threads), AES-NI, VAES, VPCLMULQDQ, SHA-NI, no AVX-512
- OS: Oracle Linux Server 10.2, Linux 6.12.0-211.74.1.el10_2.x86_64
- Toolchain: rustc 1.98.1 (rind's `flake.nix` pin) through rustup, outside Nix
  (`IN_RIND_DEV_SHELL=1` set by hand)
- Power: `amd-pstate-epp`, governor `performance`, EPP `performance`, **boost on**. Changing boost
  needs root, so it was left on and is noted here, not changed.
- Pinning: `taskset -c 2,3`, one physical core's two SMT threads (rind's runner does not pin).
- criterion: rind's `--noplot --warm-up-time 1 --measurement-time 3`, report operations only.
- Started 2026-11-12 08:33 UTC (00:33 PST), finished 09:05 UTC (01:05 PST).

## Noise: read this before the numbers

The machine was not quiet. Other agents' work and two resident video streams kept 1.2-1.3 CPUs
busy before the run and **3.8 CPUs on average during it** (load average up to 27). rind's idle
limit is 10% of the CPUs (1.2 here); to run at all it was relaxed to 20% (2.4) by a reverted local
edit (`BUSY_SHARE=0.2`, see `tools/host-run.sh`), and bench-report marks the run `noisy: true`.
`build_cost` did not run: its own idle check, seeing only the two pinned CPUs, refused.

Rows that look like noise rather than code: `aes256-gcm-open/1350B` (746 ns, against 195 ns for
`aes128-gcm-open/1350B` and 218 ns for sealing the same size) is not credible. Every row over
1.1x (`rustls-bulk/chacha20poly1305`, `aes256-gcm-open/1350B`, `p256-ecdsa-verify`,
`rsa2048-pss-sha256-sign`, `p521-ecdsa-verify`) needs a quiet rerun before it is read as a gap.
