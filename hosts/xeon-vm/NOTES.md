# `xeon-vm`

- CPU: Intel(R) Xeon(R) Processor (x86_64, 8 logical CPUs)
- OS: Debian GNU/Linux 13 (trixie) (Linux 6.12.113+)
- Toolchain: rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05)
- Commit: b66514238d449fc211d75fd14554301bb690e421
- Started / finished (UTC): 2026-11-12T06:24:55+00:00 / 2026-11-12T06:56:04+00:00
- Busy CPUs (other work): before build 1.14, before run 1.17, during run 0.93 (harness limit 1.0)
- Noisy flag (harness): False
- Libraries refused for missing CPU features: none
- OpenSSL linked: OpenSSL 3.5.9 29 Sep 2026 (Library: OpenSSL 3.5.9 29 Sep 2026)
- Build cost: ok, gcc (GCC) 16.2.0, 8 jobs
- Operations timed: 62; results: 372

## Notes

- Cloud VM, 8 vCPUs shared with other agents. CPU family 6 model 207 (a 5th-gen Xeon class part) exposing AVX-512 (F/DQ/BW/VL/IFMA/VBMI/VBMI2), VAES, VPCLMULQDQ, ADX, and SHA-NI.
- Built and run in the repo's Nix dev shell with the pinned rustc 1.98.1, through `cargo xtask host-run`, which records instead of refusing the harness's idle check (2.3 of 8 CPUs were busy at the first attempt, which the unmodified harness refused).
- `admin/build-cost` refused right after the benchmarks ("host is busy (7.9 of 8 CPUs busy)"), so it was rerun on its own in the dev shell with `CARGO_BUILD_JOBS=8` and merged into the JSON as `build_cost`; the first attempt is kept as `build_cost_first_attempt`.
- The ChaCha20-Poly1305 gap (16 KB and 1 MB transfer) is against OpenSSL, which has an AVX-512 ChaCha20 path; rind, aws-lc-rs, and ring all ran within 2% of each other there.
