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

## DIT and the gaps table (draft decision)

rind's `benchmarks/gaps.toml` explains the M4's (`kixtop`) small-input gaps with one DIT entry. rind sets DIT around every call that handles a secret, and the other libraries never set it; holding `rind::cpu::Dit::hold()` across a batch pays that cost once. The M1 Pro has DIT (`rind-cpuid: dit=1`), so the same entry applies here. The table below applies it to this host's measured data, which is the run above. Nothing was re-timed, and the run was flagged noisy, so treat differences under about 10 to 15% as noise.

Each gap here is where another library is more than 10% faster than rind, compared with the `rind (DIT held)` column:

| operation | in kixtop's DIT entry | rind / fastest | fastest | rind (DIT held) / fastest | closed by holding DIT |
|---|---|---|---|---|---|
| `quic-header-protection/aes128` | yes | 11.98× | aws-lc-rs 4.1 ns | 1.15× | no |
| `quic-header-protection/aes256` | yes | 10.97× | aws-lc-rs 5.1 ns | 1.00× | yes |
| `sha256/64B` | yes | 2.11× | graviola 55.3 ns | 1.17× | no |
| `aes256-gcm/32B` | yes | 1.82× | rustcrypto 42.2 ns | 0.46× | yes |
| `aes128-gcm/32B` | yes | 1.72× | rustcrypto 41.5 ns | 0.40× | yes |
| `hmac-sha256/64B` | yes | 1.67× | rustcrypto 144.4 ns | 0.98× | yes |
| `hkdf-sha256/expand` | yes | 1.58× | rustcrypto 85.6 ns | 1.05× | yes |
| `sha256/16KB` | no | 1.56× | aws-lc-rs 7,035.1 ns | 1.61× | no |
| `sha512/64B` | yes | 1.47× | ring 160.0 ns | 0.67× | yes |
| `hkdf-sha256/extract-expand` | yes | 1.46× | rustcrypto 285.9 ns | 0.97× | yes |
| `aes256-gcm-open/32B` | yes | 1.43× | rustcrypto 49.4 ns | 0.43× | yes |
| `aes128-gcm-open/32B` | yes | 1.39× | rustcrypto 46.5 ns | 0.43× | yes |
| `quic-header-protection/chacha20` | yes | 1.35× | ring 95.6 ns | 0.97× | yes |
| `rustls-bulk/chacha20poly1305` | no | 1.28× | openssl 1,437,563.7 ns | not timed | no |
| `sha512/1350B` | yes | 1.26× | ring 1,074.4 ns | 1.40× | no |
| `chacha20poly1305/32B` | yes | 1.22× | ring 209.5 ns | 1.01× | yes |
| `rustls-bulk/aes256-gcm` | no | 1.21× | aws-lc-rs 428,061.7 ns | not timed | no |
| `aes128-gcm/1350B` | yes | 1.19× | aws-lc-rs 226.6 ns | 0.95× | yes |
| `sha256/1350B` | yes | 1.17× | ring 605.8 ns | 0.98× | yes |
| `aes256-gcm/1350B` | yes | 1.14× | aws-lc-rs 269.2 ns | 0.93× | yes |
| `rustls-full-handshake-group/x25519` | no | 1.13× | graviola 251,900.2 ns | not timed | no |
| `chacha20poly1305/16KB` | no | 1.12× | openssl 9,385.8 ns | 1.11× | no |
| `hmac-sha256/1350B` | yes | 1.10× | ring 704.6 ns | 0.95× | yes |

**Decision (draft): the DIT entry applies to `m1pro`.** Of the 18 operations that entry lists, 17 are gaps here, and holding DIT closes 14 of them. `hmac-sha256/1350B` measured 1.10×, which is not a gap.

Three gaps of operations in that entry stay open even with DIT held, so DIT alone does not explain them:

- `quic-header-protection/aes128` at 1.15×. The aes256 form closes to 1.00×, so this is likely noise at about 4 ns a call.
- `sha256/64B` at 1.17× graviola's time.
- `sha512/1350B` at 1.40× ring's time. rind with DIT held measured slower than rind without it (1,505.6 ns against 1,356.5 ns), which looks like noise in this run.

These operations are gaps here but are outside the DIT entry. They are per-byte, not per-call, so DIT does not explain them:

- `sha256/16KB`: 1.56×, and 1.61× held, against aws-lc-rs.
- `chacha20poly1305/16KB`: 1.12× against OpenSSL.
- The rustls bulk and X25519 handshake rows.

The SHA-256 16 KB gap is the largest of these. It needs a profile on this core before it gets a reason or a quest.

rind's `benchmarks/gaps.toml` cannot list `m1pro` until `admin/bench-report`'s `HOSTS` has it: `load_gaps` fails with "no host or tier m1pro". Its data would also have to be in `benchmarks/data` at the report's commit. Until then, this note is the decision record for this host.
