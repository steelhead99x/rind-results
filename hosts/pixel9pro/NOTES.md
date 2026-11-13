# `pixel9pro`

- CPU: Google Tensor G4 (Pixel 9 Pro) (aarch64, 8 logical CPUs)
- OS: Android 17 (API 37, build CP3A.261005.005)
- Toolchain: rustc 1.98.1 (48a229cea 2026-09-01); cargo 1.98.1 (797e8a9bc 2026-08-05)
- Commit: b66514238d449fc211d75fd14554301bb690e421
- Started / finished (UTC): 2026-11-12T08:13:03+00:00 / 2026-11-12T08:51:00+00:00
- Busy CPUs (other work): before build None, before run None, during run 0.72 (harness limit 1.0)
- Noisy flag (harness): True
- Libraries refused for missing CPU features: none
- OpenSSL linked: vendored openssl-src ? (static, built by openssl-sys's vendored feature)
- Build cost: failed (not measured: the binaries were cross-compiled on another host)
- Operations timed: 62; results: 372

## Notes

- Pixel 9 Pro, Google Tensor G4 (1× Cortex-X4, 3× Cortex-A720, 4× Cortex-A520), Android 17 (API 37), on AC power the whole run (battery 90% → 89%).
- Cross-compiled on the x86_64 box: `cargo +1.98.1 bench -p rind-bench --features __bench_openssl,openssl-sys/vendored --target aarch64-linux-android --no-run` with Android NDK r28c (API 31 clang). The benches were stripped, pushed with adb to `/data/local/tmp`, and each bench binary ran with `--bench --noplot --warm-up-time 1 --measurement-time 3 <admin/bench-report's operation regex>`. `cargo xtask android-convert` parsed criterion's output the way `admin/bench-report run` does. The adb push and pull is `cargo xtask pixel-run`. `/data/local/tmp` was emptied afterward (checked).
- OpenSSL: Android ships no system OpenSSL, so the OpenSSL column is a statically linked, vendored OpenSSL built by `openssl-src`. The `openssl-sys/vendored` feature comes from the cargo command line; nothing in the repo changed.
- rind-cpuid on the phone: aes, pmull, sha2, sha3 (SHA-512/SHA-3), neon, dit all present.
- First pass: the `rustls` bench panicked. It reads `rind-rustls/tests/keys` at runtime from the build machine's absolute `CARGO_MANIFEST_DIR`. It was rebuilt from a source copy at `/data/local/tmp/rind/src` on the box, the keys were pushed to the same path on the phone, and it was rerun alone (`second_pass` in the JSON). The other 16 bench binaries come from the first pass.
- Noise and thermals: whole-system busy share 0.215 during the main pass (about 0.7 CPUs besides the benchmark) and 0.177 during the rerun. Battery temperature went 24.7 → 37.7 °C (main pass) and 32.5 → 36.9 °C (rerun). The run wasn't pinned to a core, so a benchmark could land on an X4, an A720, or an A520 core. Marked noisy by hand; per-core results would need taskset/root.
- The `rind (DIT held)` column is compiled only for Apple targets in rind-bench (`target_vendor = "apple"`), so it isn't timed here.
- No build cost: nothing was built on the phone.
