<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Jetson Orin Nano: how this set was measured

The board is a live service box (orin-web, ollama, whisper-server and others ran throughout). Nothing
was stopped, restarted or reconfigured, nvpmodel and clocks were left as found (MAXN_SUPER,
`schedutil`), and nothing ran as root. See [facts.txt](facts.txt) for the hardware and software.

## Build

- The board has no internet (its only link is USB device mode) and its clock reads 1970. The Rust
  1.98.1 aarch64 toolchain, `cargo vendor` output for rind `98568332`, and the `libssl-dev` headers
  matching the installed OpenSSL 3.0.13 were staged from a PC and unpacked under one directory
  (`cargo xtask orin-setup`, `tar -m` so mtimes don't confuse cargo).
- Built natively on the board (`-j2`, `nice -n 19`), so the services kept their CPU.
- `build_cost` was skipped (`HOSTRUN_SKIP_BUILD_COST=1`): it would have run repeated cold builds
  on the live box. The JSON has no `build_cost`.

## Correctness before timing

All in [checks/](checks/):

- `host-kat`: rind-hardware's board known-answer checks (NIST, RFC and Wycheproof vectors) built
  for the host: 91 of 91 pass across all seven libraries.
- rind's own test suite at `98568332`: 778 passed, 0 failed, 33 ignored.
- `cargo bench -- --test` for every bench: all pass. Without a current wall clock the rustls benches
  panic (`NotValidYet`), which is why the next section exists.

## libfaketime for the wall clock

The rustls handshake benches validate certificates against the wall clock. Setting the board's
clock needs root and would touch its services, so the bench binaries (only them, through cargo's
`target.aarch64-unknown-linux-gnu.runner`, never cargo or rustc) run under libfaketime 0.9.10 with
the wall clock shifted to the PC's date and `FAKETIME_DONT_FAKE_MONOTONIC=1`, so criterion's
monotonic timer is untouched. A control (plain, faketime, plain on sha256, aes128-gcm and x25519
for rind, aws-lc-rs and OpenSSL) moved rind and aws-lc-rs by at most 0.15% and OpenSSL by up to
1.2%, the same size as OpenSSL's plain-to-plain change: [checks/faketime-control.txt](checks/faketime-control.txt).

The raw JSON's `started` and `finished` come from the board clock (1970); the published times
come from the PC (`raw/run-<n>.pc.txt`).

## Libraries and the CPU's crypto extensions

The A78AE has AES, PMULL, SHA-1 and SHA-256 instructions and NEON, but no SHA-3 or SHA-512
instructions and no DIT.

- rind: aarch64 assembly for AES-GCM, GHASH (PMULL), SHA-256/SHA-1 (SHA2 extension), SHA-512,
  ChaCha20 (NEON), Keccak (scalar/NEON here, no SHA-3 extension), the s2n-bignum
  curves, and NEON ML-KEM/ML-DSA.
- aws-lc-rs, ring, OpenSSL: their upstream ARMv8 assembly (AES, PMULL, SHA2, NEON ChaCha20; known
  from upstream, not re-read for this set).
- graviola: its aarch64 assembly (likewise from upstream).
- RustCrypto: `aes` uses the AES extension and `polyval` PMULL through `cpufeatures`, `sha2` the
  SHA2 extension, `chacha20` NEON; `keccak` would use SHA-3 instructions, absent here.
- libcrux: its NEON code is behind its `simd128` feature, which rind-bench does not enable, so
  libcrux runs its portable code on this host.

## Runs

Three full runs, each pinned with `taskset` to the core that was least busy over 10 s before run 1
(runs 2 and 3 reuse it), with load, per-core busy share, frequencies and thermals recorded before
and after each run (`raw/run-<n>.load.txt`). The tables are each row's median of the three runs;
[RUNS.md](RUNS.md) has every run.
All three runs used cpu5 (cluster 1), measured 2026-11-13 03:18-05:05 UTC (2026-11-12 19:18-21:05
PST).

## Noise

- bench-report flagged no run as noisy. The other work during each run (orin-web, ollama,
  whisper-server, jtop and the rest, none stopped) kept about 0.53 CPUs busy, against the harness's
  limit of 1.0. A process named `cron` showed about 46% of a CPU (ps's lifetime average) in every
  snapshot and is likely most of that; it was left alone. The before-build samples of runs 1 and 3
  saw about 1.04 busy CPUs, and the before-timing samples 0.08-1.08.
- Run-to-run spread is small: 384 of 388 rows moved less than 2% across the three runs; the
  largest are OpenSSL's hmac-sha256/64B (6.3%) and hkdf extract-expand (3.5%), ring's
  hmac-sha256/64B (3.0%) and OpenSSL's sha256/64B (2.5%). rind's largest is 1.7%
  (quic-header-protection/aes128, about 11 ns).
- The board's CPU model is not in /proc/cpuinfo on aarch64, so the harness's JSON reads `aarch64`
  for the model; facts.txt has the part (Cortex-A78AE).
