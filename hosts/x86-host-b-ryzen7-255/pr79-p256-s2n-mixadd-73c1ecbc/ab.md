| Benchmark | n | base (median) | branch (median) | Δ medians | paired Δ, 95% CI | branch faster in | spread base / branch | read |
|---|---:|---:|---:|---:|---|---:|---|---|
| `p256-ecdh-stored/aws-lc-rs` | 10 | 33.66 µs | 33.72 µs | +0.2% | +0.2% [-0.1, +0.4] | 2/10 | 0.7% / 0.7% | no difference beyond noise |
| `p256-ecdh-stored/rind` | 10 | 32.23 µs | 32.25 µs | +0.0% | -0.0% [-0.1, +0.2] | 5/10 | 0.9% / 0.4% | no difference beyond noise |
| `p256-ecdh/aws-lc-rs` | 10 | 40.74 µs | 40.73 µs | -0.0% | +0.1% [-0.2, +0.2] | 4/10 | 0.7% / 0.9% | no difference beyond noise |
| `p256-ecdh/graviola` | 10 | 41.96 µs | 41.91 µs | -0.1% | -0.0% [-0.3, +0.1] | 6/10 | 0.8% / 0.5% | no difference beyond noise |
| `p256-ecdh/rind` | 10 | 38.55 µs | 39.00 µs | +1.2% | +1.1% [+0.9, +1.2] | 0/10 | 0.7% / 0.5% | slower |
| `p256-ecdsa-keygen/aws-lc-rs` | 10 | 6.92 µs | 6.93 µs | +0.2% | +0.2% [-0.0, +0.4] | 3/10 | 0.6% / 1.1% | no difference beyond noise |
| `p256-ecdsa-keygen/rind` | 10 | 6.55 µs | 7.00 µs | +6.9% | +6.7% [+6.2, +7.2] | 0/10 | 2.2% / 0.7% | slower |
| `p256-ecdsa-sign-digest/aws-lc-rs` | 10 | 11.74 µs | 11.75 µs | +0.1% | +0.0% [-0.2, +0.3] | 5/10 | 0.4% / 0.7% | no difference beyond noise |
| `p256-ecdsa-sign-digest/rind` | 10 | 8.62 µs | 9.06 µs | +5.1% | +5.1% [+4.9, +5.3] | 0/10 | 1.5% / 0.5% | slower |
| `p256-ecdsa-sign/aws-lc-rs` | 10 | 11.89 µs | 11.91 µs | +0.2% | +0.1% [-0.1, +0.3] | 3/10 | 1.0% / 0.7% | no difference beyond noise |
| `p256-ecdsa-sign/graviola` | 10 | 10.81 µs | 10.78 µs | -0.2% | -0.3% [-0.8, +0.0] | 7/10 | 1.9% / 0.5% | no difference beyond noise |
| `p256-ecdsa-sign/rind` | 10 | 8.64 µs | 9.10 µs | +5.3% | +5.2% [+4.9, +5.7] | 0/10 | 1.6% / 0.7% | slower |
| `p256-ecdsa-verify-digest/aws-lc-rs` | 10 | 37.22 µs | 37.25 µs | +0.1% | +0.0% [-0.2, +0.2] | 4/10 | 0.6% / 0.6% | no difference beyond noise |
| `p256-ecdsa-verify-digest/rind` | 10 | 34.70 µs | 34.67 µs | -0.1% | -0.1% [-0.2, +0.0] | 8/10 | 1.0% / 0.7% | no difference beyond noise |
| `p256-ecdsa-verify-parsed/aws-lc-rs` | 10 | 37.42 µs | 37.43 µs | +0.0% | +0.0% [-0.5, +0.2] | 5/10 | 1.3% / 0.8% | no difference beyond noise |
| `p256-ecdsa-verify-parsed/rind` | 10 | 34.72 µs | 34.68 µs | -0.1% | -0.1% [-0.2, +0.1] | 5/10 | 0.5% / 0.4% | no difference beyond noise |
| `p256-ecdsa-verify/aws-lc-rs` | 10 | 38.42 µs | 38.32 µs | -0.3% | -0.1% [-0.6, +0.0] | 7/10 | 1.7% / 0.8% | no difference beyond noise |
| `p256-ecdsa-verify/graviola` | 10 | 41.04 µs | 41.00 µs | -0.1% | -0.2% [-0.4, +0.1] | 7/10 | 0.7% / 1.1% | no difference beyond noise |
| `p256-ecdsa-verify/rind` | 10 | 38.91 µs | 38.88 µs | -0.1% | -0.0% [-0.4, +0.2] | 5/10 | 0.7% / 0.7% | no difference beyond noise |
| `p256-keygen/aws-lc-rs` | 10 | 6.92 µs | 6.92 µs | -0.0% | -0.1% [-0.2, +0.2] | 6/10 | 0.9% / 0.4% | no difference beyond noise |
| `p256-keygen/graviola` | 10 | 7.81 µs | 7.76 µs | -0.7% | -0.7% [-0.9, -0.4] | 10/10 | 1.1% / 0.9% | faster |
| `p256-keygen/rind` | 10 | 6.25 µs | 6.69 µs | +7.1% | +7.1% [+6.8, +7.2] | 0/10 | 2.6% / 0.5% | slower |
