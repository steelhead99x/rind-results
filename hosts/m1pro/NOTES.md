# `m1pro`

- CPU: Apple M1 Pro (arm64, 10 logical CPUs)
- OS: macOS 14.0 (23.0.0)
- Toolchain: rustc 1.98.0 (88d9e12ae 2026-08-18) (Homebrew); cargo 1.98.0 (797e8a9bc 2026-08-05) (Homebrew)
- Commit: b66514238d449fc211d75fd14554301bb690e421
- Started / finished (UTC): 2026-11-12T07:33:51+00:00 / 2026-11-12T08:11:50+00:00
- Busy CPUs (other work): before build 3.6, before run 3.86, during run 2.69 (harness limit 1.0)
- Noisy flag (harness): True
- Libraries refused for missing CPU features: none
- OpenSSL linked: OpenSSL 3.5.9 29 Sep 2026 (Library: OpenSSL 3.5.9 29 Sep 2026)
- Build cost: failed (error: host is busy (10.1 of 10 CPUs busy))
- Operations timed: 62; results: 400

## Notes

- MacBook Pro, Apple M1 Pro (8 performance + 2 efficiency cores), macOS 14.0, in daily use. The harness flags it noisy: other work kept 2.7 CPUs busy during the benchmarks, 3.6 to 3.9 before. Treat differences under about 10 to 15% as noise.
- Built with Homebrew's rustc 1.98.0, not the flake's pinned 1.98.1 (the Nix dev shell would have needed GBs of downloads on a nearly full disk). Run through `cargo xtask mac-run --commit b66514238d449fc211d75fd14554301bb690e421` with the crate cache and build dir inside its own work dir, and a free-space watchdog. That commit is the published host set. The command's default commit is the later comparable pin `d6ca9a756d3d1e71489e61e44c04f33c3c893589`.
- Two earlier attempts were stopped by that watchdog when free space fell under 1.5 GB (the disk was 99% full); this third run started with 35 GB free after space was freed.
- `admin/build-cost` refused after the benchmarks: "host is busy (10.1 of 10 CPUs busy)". No build cost for this host.
- The M1 Pro has DIT (rind-cpuid: dit=1), so the `rind (DIT held)` column is timed, as on the upstream `kixtop` (M4). rind sets DIT around each call that handles a secret and the other libraries don't, which costs it on small inputs; see the gaps table.
- OpenSSL is Homebrew's 3.5.9.
