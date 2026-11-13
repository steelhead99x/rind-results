# M1 Pro: perf/pi5-narrow-nist vs + P-384 w7 generator NAF (7 interleaved rounds, noisy host)

Medians:

| op | nist µs (n, spread) | w7 µs (n, spread) | w7/nist | aws-lc-rs µs | nist/aws | w7/aws |
|---|---|---|---|---|---|---|
| p256-ecdsa-verify | 40.4 (7, 3.4%) | 40.5 (7, 8.1%) | 1.002 | 45.5 | 0.89 | 0.89 |
| p384-ecdh | 154.1 (7, 24.9%) | 152.6 (7, 6.6%) | 0.990 | 168.7 | 0.91 | 0.90 |
| p384-ecdsa-sign | 36.6 (7, 14.3%) | 35.8 (7, 6.2%) | 0.976 | 71.6 | 0.51 | 0.50 |
| p384-ecdsa-verify | 140.4 (7, 5.5%) | 133.9 (7, 8.3%) | 0.954 | 175.7 | 0.80 | 0.76 |
| p521-ecdsa-verify | 209.7 (7, 6.0%) | 209.5 (7, 2.0%) | 0.999 | 289.5 | 0.72 | 0.72 |

Min of rounds:

| op | nist min µs (n) | w7 min µs (n) | w7/nist (min) |
|---|---|---|---|
| p256-ecdsa-verify | 39.5 (7) | 39.5 (7) | 1.001 |
| p384-ecdh | 148.6 (7) | 148.8 (7) | 1.001 |
| p384-ecdsa-sign | 35.0 (7) | 35.2 (7) | 1.005 |
| p384-ecdsa-verify | 136.7 (7) | 131.1 (7) | 0.959 |
| p521-ecdsa-verify | 206.3 (7) | 207.8 (7) | 1.007 |
