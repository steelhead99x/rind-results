<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# amd-ryzen-9-3900x: how this set was measured

The machine is a desktop (gaming-002) with an AMD Ryzen 9 3900X (x86_64, 12 cores / 24 threads, Zen 2), running Oracle Linux Server 10.3. This CPU is new to both kixel-ai/rind's and this repo's results.

Three runs of `cargo xtask bench` at steelhead99x/rind fork main `0a768eb702b8bc7cf84ce2286b64e64489979d56` (unpatched; `meta.json` records it as `upstream_commit`), no CPU pinning, boost on, on 2026-11-26, against every library in the README: aws-lc-rs, ring, RustCrypto + dalek, graviola, libcrux, OpenSSL and pyca/verified-garbage, plus build cost. Criterion arguments were `--noplot --warm-up-time 1 --measurement-time 3`. Every run ended `ok` (627 results each). No relaxations ([relaxations.json](relaxations.json)).

## Gaps

The report step refuses to write a summary while a gap over 10% has no decision. [gaps.toml](gaps.toml) records the one gap on this host, ML-KEM-768 keygen at 1.56x pyca/verified-garbage, carrying over upstream's decision in kixel-ai/rind#812. That decision was measured on Zen 3 and later hosts at 1.17-1.25x, so the larger Zen 2 margin should be rechecked against decapsulation on this CPU. `cargo xtask report` and `cargo verify --dir` then passed with 0 mismatches; no benchmark was rerun.
