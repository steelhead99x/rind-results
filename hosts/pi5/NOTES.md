# `pi5`

- CPU: Raspberry Pi 5 Model B Rev 1.0 (aarch64, 4 logical CPUs)
- OS: Debian GNU/Linux 13 (trixie) (Linux 6.18.60+rpt-rpi-2712)
- Toolchain: rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05)
- Commit: b66514238d449fc211d75fd14554301bb690e421
- Started / finished (UTC): 2026-11-12T06:25:59+00:00 / 2026-11-12T07:21:11+00:00
- Busy CPUs (other work): before build 0.0, before run 0.01, during run 0.0 (harness limit 1.0)
- Noisy flag (harness): False
- Libraries refused for missing CPU features: none
- OpenSSL linked: OpenSSL 3.5.7 9 Jun 2026 (Library: OpenSSL 3.5.7 9 Jun 2026)
- Build cost: ok, cc (Debian 14.2.0-19) 14.2.0, 4 jobs
- Operations timed: 62; results: 372

## Notes

- Raspberry Pi 5 Model B (BCM2712, 4× Cortex-A76 at 2.4 GHz, 8 GB), Raspberry Pi OS (Debian 13), reached over ssh through the desktop. Idle during the run (0.0 other busy CPUs).
- rustc 1.98.1 from rustup, the same release the flake pins (not the Nix build of it). Run through `scripts/host-run.py`.
- The board has `libssl3` but no `libssl-dev`, and nothing was installed on it: `scripts/pi-run.sh` unpacked `libssl-dev` (3.5.9 headers) with `apt-get download` + `dpkg -x` into the work dir, linking against the system's libssl 3.5.7 at runtime. The OpenSSL column is the system's OpenSSL 3.5.7.
- rind-cpuid reports aes, pmull, sha2, neon, and no sha3 (the SHA-512/SHA-3 extension) or DIT on this Cortex-A76.
- Build cost measured by the harness's own `admin/build-cost` after the benchmarks, 4 jobs.
- The largest gaps of this run are here: public-key operations (X25519 2.32×, RSA-2048 about 1.65×, P-256/P-384/P-521 ECDH 1.5 to 1.6×, Ed25519 about 1.45×) trail aws-lc-rs on the Cortex-A76, while the same rind code leads on Apple M-series. A plausible cause, not checked here: on aarch64 rind runs s2n-bignum's `_alt` routines (as `benchmarks/verified.toml` records for X25519 and P-256), which are tuned for cores with fast high multiplies, while aws-lc picks per core.
