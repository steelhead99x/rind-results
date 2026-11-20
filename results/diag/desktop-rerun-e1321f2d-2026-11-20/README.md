<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Desktop rerun of rind `e1321f2d` on Ultra 9 275HX, M1 Pro, and Ryzen 7 255, 2026-11-20

These are the 20 November 2026 desktop reruns of the full `admin/bench-report` suite at rind [`e1321f2de2d0306beb557116ed64c479c606fd00`](https://github.com/steelhead99x/rind/commit/e1321f2de2d0306beb557116ed64c479c606fd00), using rind-results harness [`417c35513303991f73a7e3f43eb68fcd98ef4ae2`](https://github.com/steelhead99x/rind-results/commit/417c35513303991f73a7e3f43eb68fcd98ef4ae2). The overnight 19–20 November runs on these hosts produced no results. This set used no-code workarounds so the same suite could finish; the report step then stopped at the 10% gap gate, so there is no published `<host>.json` and the files sit under `diag/` rather than `results/<host>/<sha>/`.

Times below are Pacific Time (America/Los_Angeles, PST, UTC-8).

## Why the overnight runs failed, and the workarounds used today

The failures were in the rind-results harness, not in rind's crypto:

1. **CPU pin vs rustls assertion.** By default the harness pins the process to logical CPUs 2,3 (`--cpus` default). A rustls check in rind (`rind-bench/benches/rustls.rs`, around lines 315–321) asserts that the process may use every logical CPU on the host. On the Ultra 9 275HX (24 CPUs) and the Ryzen 7 255 (16 CPUs) the suite panicked after `rustls-resumed-handshake/x25519`. Today's reruns passed `--cpus 0-23` and `--cpus 0-15` respectively, i.e. every logical CPU. Single-thread rows were therefore unpinned and noisier, and these runs are not comparable with older pinned runs.
2. **macOS has no `taskset`.** The Mac overnight run waited, then `cargo xtask bench` failed immediately with `python3: No such file or directory`. That message is mislabelled: the missing program is `taskset` (see `tools/xtask/src/upstream_run.rs`, around line 201), which macOS does not provide. Today's rerun used a passthrough `taskset` shim, so the harness recorded `cpus=2,3` in `run.log` but no CPU affinity was applied.
3. **`drive.sh` omitted `report --rind`.** The driver invoked the report step without the `--rind` path the report command expects. Today's reruns invoked `cargo xtask report --rind <rind-src> --dir <run-dir>`.

No harness or rind code was changed for these reruns.

## What was measured

| | Ultra 9 275HX | M1 Pro | Ryzen 7 255 |
|---|---|---|---|
| Board id | `intel-core-ultra-9-275hx` | `apple-m1-pro` | `amd-ryzen-7-255` |
| CPU | Intel Core Ultra 9 275HX, 24 logical / 24 physical, x86_64 | Apple M1 Pro, 10 logical / 10 physical, arm64 | AMD Ryzen 7 255, 16 logical / 8 physical (2 threads per core), x86_64 |
| OS | Oracle Linux Server 10.2 | macOS 14.0 (23.0.0) | Oracle Linux Server 10.1 |
| rind | `e1321f2de2d0306beb557116ed64c479c606fd00` | same | same |
| Toolchain | rustc 1.98.1, cargo 1.98.1 | rustc 1.98.1, cargo 1.98.1 | rustc 1.98.1, cargo 1.98.1 |
| Runs | 3, all `status: ok`, 570 results, 0 errors | 3, all `status: ok`, 598 results, 0 errors | 3 completed; raw JSON later overwritten (see below) |
| Window (PT) | 9:03 AM – 11:36 AM | 11:11 AM – 1:38 PM | 9:08 AM – 11:25 AM |
| `bench-rc` / `report-rc` / `verify-rc` | 0 / 1 / 1 | 0 / 1 / 1 | 0 / 1 / 1 |
| CPU flag | `--cpus 0-23` | `run.log` records `cpus=2,3`; the `taskset` shim was a passthrough, so nothing was pinned | `--cpus 0-15`, from the completed run's bench command |
| Busy | `--record-busy`. Run 1 `noisy: false`. Runs 2 and 3 `noisy: true`. See the load table. | `--record-busy`. All three runs `noisy: true`. See the load table. | Raw JSON (and therefore the `load` block) was not kept |

Criterion args, from each `meta.json`: `--noplot --warm-up-time 1 --measurement-time 3`. Relaxations, from each `relaxations.json`: `check_idle` was "record and continue" instead of refusing when busy; `build_cost` was skipped. `meta.relaxed` is `true`. `spread_is` is `runs`. `metric_units` is `ns`.

`report-rc=1` is the 10% gap gate: rind was more than 10% slower than the named library on the operations listed below, and no decision had been recorded for those gaps. `verify-rc=1` is the expected follow-on: the report step did not write a schema-1 `<host>.json`, so `cargo verify` on the run directory reported two mismatches. Those mismatches are in each host's `report-excerpt.txt`.

An earlier Mac attempt the same morning lost its Nix-managed Rust toolchain mid-run (likely a garbage collection) and is not included. A later Ryzen 7 255 rerun was still in progress when this directory was written and is not included.

### Load, as recorded in each run JSON

`limit_cpus`, `busy_cpus_before_build`, `busy_cpus_before_run`, and `other_busy_cpus_during_run` are the harness `load` fields, copied as stored.

**intel-core-ultra-9-275hx** (`limit_cpus` 2.4):

| Run | Started (PT) | Finished (PT) | before_build | before_run | during_run | noisy |
|---|---|---|---:|---:|---:|---|
| 1 | 9:03:56 AM | 9:50:48 AM | 0.53 | 0.35 | 0.43 | false |
| 2 | 9:50:48 AM | 10:45:18 AM | 0.89 | 0.79 | 5.69 | true |
| 3 | 10:45:18 AM | 11:36:54 AM | 14.71 | 4.07 | 4.82 | true |

**apple-m1-pro** (`limit_cpus` 1.0, of 10 logical CPUs):

| Run | Started (PT) | Finished (PT) | before_build | before_run | during_run | noisy |
|---|---|---|---:|---:|---:|---|
| 1 | 11:11:14 AM | 12:03:50 PM | 3.64 | 4.49 | 1.78 | true |
| 2 | 12:03:50 PM | 12:51:11 PM | 1.92 | 2.25 | 2.23 | true |
| 3 | 12:51:11 PM | 1:38:43 PM | 4.18 | 2.99 | 2.81 | true |

## Files

- `intel-core-ultra-9-275hx/` and `apple-m1-pro/`: `run-1.json`, `run-2.json`, `run-3.json` are the harness's `raw/run-*.json` documents (schema 1), copied without a `raw/` directory so `cargo verify --dir results` does not treat this as a published host tree. `meta.json`, `relaxations.json`, and `run.log` are the harness sidecars, unedited. `report-excerpt.txt` is a trimmed report log: return codes, the gaps line verbatim, and the verify mismatch lines. Full `bench.log` files are not included (they contain local paths).
- `amd-ryzen-7-255/report-excerpt.txt`: the gaps line and return codes from the 9:08–11:25 AM PT run only. That run's raw JSON was overwritten by a later rerun and is not included.

The JSON and sidecars were grepped for home paths, usernames, hostnames, LAN addresses, serials and UDIDs; none were found. Excerpts had local paths removed; the gaps lines were left as printed.

## Gaps over 10% without a decision

These lines are copied verbatim from each host's report log, including the `{lib}` placeholders.

**intel-core-ultra-9-275hx:**

```
  gaps over 10% without a decision: aes128-gcm/{lib}/16KB (1.17x verified-garbage), aes256-gcm-open/{lib}/16KB (1.21x verified-garbage), chacha20poly1305/{lib}/1350B (1.11x verified-garbage), chacha20poly1305/{lib}/16KB (1.12x verified-garbage), ed25519-sign/{lib} (1.11x graviola), mlkem768-encaps/{lib} (1.29x verified-garbage), mlkem768-keygen/{lib} (1.38x verified-garbage), p384-ecdsa-sign/{lib} (1.55x verified-garbage), p384-ecdsa-verify/{lib} (1.23x verified-garbage), rsa2048-pss-sha256-verify/{lib} (1.10x verified-garbage), sha3-256/{lib}/1350B (1.22x verified-garbage), sha3-256/{lib}/64B (1.13x verified-garbage), sha512/{lib}/1350B (3.89x verified-garbage), sha512/{lib}/16KB (4.18x verified-garbage), sha512/{lib}/64B (1.73x verified-garbage)
```

**apple-m1-pro:**

```
  gaps over 10% without a decision: aes128-gcm-open/{lib}/32B (1.55x rustcrypto), aes128-gcm/{lib}/1350B (1.11x aws-lc-rs), aes128-gcm/{lib}/32B (1.82x rustcrypto), aes256-gcm-open/{lib}/32B (1.56x rustcrypto), aes256-gcm/{lib}/32B (1.96x rustcrypto), chacha20poly1305/{lib}/16KB (1.24x verified-garbage), chacha20poly1305/{lib}/32B (1.25x ring), hkdf-sha256/expand/{lib} (1.42x rustcrypto), hkdf-sha256/extract-expand/{lib} (1.40x rustcrypto), hmac-sha256/{lib}/64B (1.71x graviola), md5/{lib}/64B (1.37x rustcrypto), mldsa65-keygen/{lib} (1.11x verified-garbage), mlkem768-keygen/{lib} (1.21x verified-garbage), p384-ecdsa-sign/{lib} (1.42x verified-garbage), quic-header-protection/aes128/{lib} (13.49x aws-lc-rs), quic-header-protection/aes256/{lib} (12.21x aws-lc-rs), quic-header-protection/chacha20/{lib} (1.37x ring), rsa2048-pkcs1-sha256-verify/{lib} (1.15x verified-garbage), rsa2048-pss-sha256-verify/{lib} (1.10x verified-garbage), sha1/{lib}/1350B (1.21x verified-garbage), sha1/{lib}/64B (1.88x rustcrypto), sha256/{lib}/1350B (1.22x ring), sha256/{lib}/64B (2.07x verified-garbage), sha3-256/{lib}/1350B (1.14x verified-garbage), sha3-256/{lib}/64B (1.40x rustcrypto), sha3-512/{lib}/1350B (1.12x verified-garbage), sha3-512/{lib}/64B (1.31x rustcrypto), sha512/{lib}/1350B (1.28x verified-garbage), sha512/{lib}/16KB (1.17x verified-garbage), sha512/{lib}/64B (1.43x verified-garbage), xchacha20poly1305/{lib}/32B (1.13x aws-lc-rs)
```

**amd-ryzen-7-255** (from the completed run whose raw data was later lost):

```
  gaps over 10% without a decision: aes128-gcm/{lib}/1350B (1.17x verified-garbage), aes256-gcm/{lib}/1350B (1.14x verified-garbage), chacha20poly1305/{lib}/1350B (1.10x verified-garbage), oprf-ristretto255-blind/{lib} (1.25x rustcrypto), oprf-ristretto255-evaluate/{lib} (1.34x rustcrypto), p384-ecdsa-sign/{lib} (1.41x verified-garbage), p384-ecdsa-verify/{lib} (1.12x verified-garbage), rsa2048-pkcs1-sha256-verify/{lib} (1.27x verified-garbage), rsa2048-pss-sha256-verify/{lib} (1.16x verified-garbage)
```

Because several runs are marked `noisy: true`, and because the Linux reruns were unpinned, these ratios should not be read as a quiet, pinned comparison against older published sets.
