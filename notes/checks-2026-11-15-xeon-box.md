<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Correctness checks on a Xeon cloud VM, 2026-11-15

These are correctness gates only. No timing was measured, and no board was flashed, reset or opened on serial. Times are America/Los_Angeles (PST).

Host: Intel(R) Xeon(R) Processor, 8 vCPUs, Debian GNU/Linux 13 (trixie), Linux 6.12.113+, shared with other agents. Toolchain: rustc 1.100.0 (0ef1bc386 2026-11-09), stable via rustup. OpenSSL came from the distribution's `libssl-dev` (`OPENSSL_LIB_DIR`/`OPENSSL_INCLUDE_DIR` set, since `pkg-config` was not installed).

| Check | Tree | Command | Result |
|---|---|---|---|
| rind-hardware host known-answer checks | rind-hardware `ec7743a8`, `rind` submodule at the upstream pin `d6ca9a75` (no patches) | `cd host-kat && cargo run --release` | `{"type":"host-kat-done","failed":0}`: 91 checks over 20 vectors, 0 failed (rind 17, libcrux 17, RustCrypto 16, ring 11, aws-lc-rs 9, graviola 9, OpenSSL 9, dalek 2, libcrux-ml-kem 1). Raw: [raw/host-kat-2026-11-15-xeon-box.ndjson](raw/host-kat-2026-11-15-xeon-box.ndjson) |
| rind library tests (vectors, differential, doctests) | steelhead99x/rind `3fc24bd1` (fork main), with the Wycheproof, CAVP and CCTV submodules | `cargo test -p rind --release --locked --no-fail-fast` | exit 0: 912 passed, 0 failed, 40 ignored (unit 767, crosschecks 78, wycheproof 21, aws_lc 9, debug 7, cpu 2, cctv 1, spake2plus_entry 1, doctests 24 + 2) |
| rind zeroing and heap-scratch tests | same | `cargo test -p rind --release --locked --features wipe --test zeroing --test heap_scratch` | exit 0: zeroing 18 passed, heap_scratch 12 passed |
| rind-hardware `cargo xtask apply-patches` | rind-hardware `ec7743a8` | as named | base series committed at `eb053e787735b1898fa64ebc91ae13b31db663ec`, the commit README.md gives for this pin |
| rind-hardware `cargo xtask check` | same, after apply-patches | as named | rustfmt ok, clippy xtask ok; stopped at `clippy aes_gcm`: the ESP toolchain (`.tools/export-esp.sh`) is not installed on this host. Firmware lints were not run |
| rind-results gate | rind-results `9037ce2d` | `cargo xtask ci` | exit 0: fmt, clippy, 115 tests passed (1 ignored), cargo deny ok, `cargo verify` ok on the published trees and on 12 `verify --dir` trees |
| Skills repository gate | a private skills repository | `cargo xtask ci` | exit 0: 18 tests passed, MSRV 1.74 check ok, checker ok |
| rind-agent-lite | rind-agent-lite `244b1ecc` | `cargo test --locked`, then `cargo run --locked --quiet` | exit 0 both: 11 tests passed; pin check `acc61b3892ceb6afecfa96c94132bf6e35bbf1fb` (kixel-ai/rind) ok |

Not run: SLOW_TESTS, ctgrind and the other constant-time harnesses, fuzzing, `just` recipes, and any firmware build, flash or board run.
