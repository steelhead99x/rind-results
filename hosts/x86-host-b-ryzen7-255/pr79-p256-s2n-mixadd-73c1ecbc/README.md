<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# AMD Ryzen 7 255: P-256 timing for rind #79 (s2n-bignum `p256_montjmixadd`), 2026-11-18

[steelhead99x/rind#79](https://github.com/steelhead99x/rind/pull/79) at `73c1ecbc0a958c0b437189b9b3043af44caefff7`
(`fix/p256-table-align`) against its base, kixel-ai/rind-fork `main` at `d721a8e3d0c930b9df16ee3825c30250dbb6c50e`
(also the merge-base). The PR asked for AMD timing: its x86_64 P-256 generator windows now add with s2n-bignum's
`p256_montjmixadd` instead of aws-lc's `ecp_nistz256_point_add_affinex`, and
[kixel-ai/rind#234](https://github.com/kixel-ai/rind/pull/234) had found aws-lc's routine faster on GitHub's runners.
The PR's own measurement was on a Sapphire Rapids VM only.

## Result

**Slower here.** On this CPU the PR makes P-256 key generation about 7% slower and ECDSA signing about 5% slower. The
branch was slower in every one of the 10 paired rounds. The decision rule in the PR (keep the swap if P-256 sign and
keygen are no slower on Zen) is **not met**.

| operation (rind) | base | #79 | paired Δ (median of 10 rounds), 95% CI | #79 faster in |
|---|---:|---:|---|---:|
| `p256-keygen` | 6.25 µs | 6.69 µs | **+7.1%** [+6.8, +7.2] | 0/10 |
| `p256-ecdsa-keygen` | 6.55 µs | 7.00 µs | **+6.7%** [+6.2, +7.2] | 0/10 |
| `p256-ecdsa-sign` | 8.64 µs | 9.10 µs | **+5.2%** [+4.9, +5.7] | 0/10 |
| `p256-ecdsa-sign-digest` | 8.62 µs | 9.06 µs | **+5.1%** [+4.9, +5.3] | 0/10 |
| `p256-ecdh` | 38.55 µs | 39.00 µs | +1.1% [+0.9, +1.2] | 0/10 |
| `p256-ecdh-stored` | 32.23 µs | 32.25 µs | 0.0% [-0.1, +0.2] | 5/10 |
| `p256-ecdsa-verify` | 38.91 µs | 38.88 µs | -0.0% [-0.4, +0.2] | 5/10 |
| `p256-ecdsa-verify-parsed` | 34.72 µs | 34.68 µs | -0.1% [-0.2, +0.1] | 5/10 |
| `p256-ecdsa-verify-digest` | 34.70 µs | 34.67 µs | -0.1% [-0.2, +0.0] | 8/10 |

Verification and stored-key ECDH do not use the generator windows, and they did not move, as expected. The unchanged
libraries in the same binaries (aws-lc-rs and graviola, rows in [ab.md](ab.md)) moved by -0.7% to +0.2% (aws-lc-rs: all
within ±0.3%), so the 5 to 7% is not drift in the host. After the change rind's key generation is still 3% faster than
aws-lc-rs (6.69 vs 6.92 µs), where before it was 10% faster, and signing is still 24% faster (9.10 vs 11.91 µs).

What the numbers do not separate: the head also carries the table alignment (item 1) and the deletion of aws-lc's
mixed addition (item 3), so the slowdown is attributed to item 2 (the s2n-bignum addition) by the code path
(only the generator-window addition changed arithmetic), not by a three-way measurement. Splitting the PR, as its own
text suggests, would measure it directly.

## Host

AMD Ryzen 7 255 (family 19h model 75h, i.e. Zen 4, not Zen 5), 8 cores / 16 threads, 8 MiB L2, 16 MiB L3, AVX2, ADX,
BMI2, AVX-512 (F, VL, IFMA, BF16), SHA-NI, VAES. Oracle Linux Server 10.1, Linux 6.12 (UEK), 60 GiB RAM, rustc and cargo
1.98.1 from rustup (rind's pinned version; no Nix), `Cargo.lock` honoured (`--locked`). `amd-pstate-epp`, governor
`performance`, EPP `performance`, boost on (left on). ADX is present, so the non-ADX `_alt` addition did not run, and
this run says nothing about it.

## Method

- Same harness on both sides: `cargo bench --locked -p rind-bench --bench p256`, rind-bench from each checkout
  (rind-bench itself is unchanged by the PR), criterion `--noplot --warm-up-time 1 --measurement-time 3` as
  `admin/bench-report` uses, one row filter `^p256-.*/(rind|aws-lc-rs|graviola)$` (22 rows).
- Separate target directories and detached worktrees for base and head, built once with the same toolchain.
- 10 rounds, base and head alternating, with the order swapped every round (base first in odd rounds), pinned with
  `taskset` to one physical core's two threads.
- The time of a row is criterion's slope estimate (mean when there is no slope), as `admin/bench-report` records.
  "Δ medians" in [ab.md](ab.md) compares the medians of the 10 rounds; "paired Δ" is the median of the 10
  head/base ratios from adjacent runs, with a 4000-resample bootstrap 95% CI. "spread" is (max-min)/median of the 10 rounds.
- Raw per-run estimates (with CI bounds): [raw/](raw/). Runner and analysis: [scripts/](scripts/). Log with the load
  average before each run: [run.log](run.log).

## Noise

The box was not idle. A resident video encode (about a third of one core) and a few web services run on it all the
time, and other jobs started now and then. The one-minute load average was 1.0 to 1.9 before most runs and reached 3.1
to 3.9 before four of them (base in round 1, head in round 3, base in round 6, head in round 10), see [run.log](run.log). The measured work was pinned to one core
and the base/head runs were interleaved, so such bursts hit both sides. The direction held in all 10 rounds, and the
per-side spread of the changed rows is 0.5% to 2.6%, well below the 5 to 7% difference.
This is not a quiet-machine run in rind's `bench-report` sense (that tool refuses a host that is busy), and it is not in the comparable set.

## Not measured

Whole-suite `admin/bench-report run` (the PR's other suggestion), Zen 3, a CPU without ADX, and the split of the three changes.
