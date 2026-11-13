<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# pi2b-bench: how this set was measured

The board is a Raspberry Pi 2 Model B Rev 1.1 (BCM2836, four Cortex-A7 cores at 900 MHz, NEON, no ARMv8 cryptography extensions, 920 MiB visible). The OS is Raspbian 13 (trixie), kernel 6.18.60+rpt-rpi-v7. See [facts.txt](facts.txt) for the rest of the hardware and toolchain.

This set was not produced by rind's `admin/bench-report run`. The rind-bench criterion harnesses were cross-compiled on a macOS host for `armv7-unknown-linux-gnueabihf` with rustc/cargo 1.98.1, Homebrew clang 22, the Pi's own sysroot, and rust-lld, then run on the board itself. `build_cost` was not measured.

## Correctness before timing

Validation ran on the board, from the same rind commit, using the `sh/armv7.sh` matrix (release tests with debug assertions and overflow checks). Doctests were not run (the tests were cross-compiled).

- NEON enabled: 1023 passed, 0 failed, 34 ignored.
- NEON denied (`RIND_CPU_DISABLE_neon`): 1023 passed, 0 failed, 34 ignored.
- Memory-wipe tests, both modes: 28 passed, 0 failed.
- `detects_the_expected_cpu` passed in both tiers, so the NEON path is the one that ran with NEON enabled and the portable path is the one that ran with it denied.

## Libraries

The libraries timed are rind, aws-lc-rs, ring, RustCrypto, libcrux, and the Pi's system OpenSSL 3.5.9. Graviola does not build on armv7 and has no rows.

rind-cpuid has no armv7 output. rind's NEON path was confirmed by the test above, not by a cpuid dump.

## Runs

Five full rounds, 2026-11-13 13:24:02 to 16:05:32 on the board clock (UTC-5; 10:24 AM to 1:05 PM America/Los_Angeles). Each harness was pinned to cpu3. The CPU governor was `performance` (900 MHz) for every round; lightdm and gpsd were stopped; the board was otherwise idle. Harness order rotated each round. Criterion used the usual report arguments plus `--sample-size 10` so five rounds would fit on this board.

Temperatures were 42.2–46.0 °C. `throttled=0x0` before and after every round. After the last round the governor was restored to `ondemand` and lightdm and gpsd were started again.

Each published time is the median of the five rounds' criterion estimates. [RUNS.md](RUNS.md) lists every run. The raw files are [raw/run-1.json](raw/run-1.json) through [raw/run-5.json](raw/run-5.json).

## One spike, left in the median

Round 5 of `mldsa65-sign` for OpenSSL was 20.46 ms. The other four rounds were 17.32, 18.01, 18.19 and 18.41 ms (median of those four: 18.10 ms). The published OpenSSL figure is the median of all five, 18.19 ms, because `cargo verify` checks that every operation and library appears in every run and that the host JSON matches the median of those runs. No other (operation, library) pair was treated as a spike. All five rounds are marked valid.

## Where rind is behind on armv7

The Cortex-A7 has NEON but no AES instructions. On AES-GCM (about 3.2–3.9×) and rustls bulk AES-GCM (3.71×), rind matches RustCrypto and is behind aws-lc-rs and ring, which use ARMv7 NEON bitsliced AES (bsaes) and NEON GHASH (`vmull.p8`). AES QUIC header protection is about 5.1–5.3× slower than aws-lc-rs: that path encrypts one block per packet and needs a fast single-block AES, separate from the bulk CTR path.

P-256 ECDH and ECDSA-verify are about 1.11–1.19× behind OpenSSL's 32-bit ARM `ecp_nistz256` assembly; rind is still faster than the other libraries on those rows. P-256 ECDSA-sign is 1.08× behind OpenSSL, under the 10% gap threshold.

ChaCha20-Poly1305 is 1.33× behind RustCrypto at 32 B (per-call setup) and 1.12× behind OpenSSL at 16 KB (OpenSSL's wider NEON interleave). XChaCha20-Poly1305 at 32 B is 1.26× behind RustCrypto for the same setup reason. Mid-size ChaCha rows are ties or within 2%.

RSA-2048 verify is 1.05× behind aws-lc-rs, and P-521 ECDSA-verify is 1.15× behind aws-lc-rs. Those last two are small; P-521 verify is a gap over 10% and has an entry in [gaps.toml](gaps.toml).

On the other operations rind is already the fastest or tied, including the rustls full and resumed handshakes, ML-KEM, ML-DSA, SHA-3, and P-384.
