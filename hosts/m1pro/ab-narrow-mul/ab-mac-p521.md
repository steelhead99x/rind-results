# M1 Pro: perf/pi5-p384-verify-w7 vs + P-521 w7 generator NAF (7 interleaved rounds, noisy host)

Medians:

| op | w7 µs (n, spread) | p521 µs (n, spread) | p521/w7 | aws-lc-rs µs | w7/aws | p521/aws |
|---|---|---|---|---|---|---|
| p384-ecdsa-verify | 141.1 (7, 129.0%) | 141.0 (7, 14.2%) | 0.999 | 176.7 | 0.80 | 0.80 |
| p521-ecdh | 239.2 (7, 43.3%) | 263.9 (7, 128.6%) | 1.103 | 255.5 | 0.94 | 1.03 |
| p521-ecdsa-sign | 58.7 (7, 74.3%) | 60.3 (7, 93.5%) | 1.027 | 150.2 | 0.39 | 0.40 |
| p521-ecdsa-verify | 225.2 (7, 80.7%) | 208.2 (7, 153.4%) | 0.925 | 292.7 | 0.77 | 0.71 |

Min of rounds:

| op | w7 min µs (n) | p521 min µs (n) | p521/w7 (min) |
|---|---|---|---|
| p384-ecdsa-verify | 130.5 (7) | 134.0 (7) | 1.027 |
| p521-ecdh | 227.2 (7) | 230.8 (7) | 1.016 |
| p521-ecdsa-sign | 58.4 (7) | 57.5 (7) | 0.986 |
| p521-ecdsa-verify | 207.4 (7) | 200.9 (7) | 0.968 |
