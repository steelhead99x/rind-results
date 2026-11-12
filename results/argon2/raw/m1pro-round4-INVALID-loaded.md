| id | mode | runs | median | min | max | spread |
|---|---|---|---|---|---|---|
| 16MiB-1/rind | a0-before-neon | 3 | 7.732 ms | 6.326 ms | 9.479 ms | 49.9% |
| 16MiB-1/rind | c1-asm1-neon | 3 | 4.255 ms | 3.945 ms | 5.529 ms | 40.2% |
| 16MiB-1/rind | d1-scalar-neon | 3 | 7.629 ms | 7.355 ms | 9.894 ms | 34.5% |
| 16MiB-1/rustcrypto | a0-before-neon | 3 | 11.674 ms | 6.931 ms | 12.594 ms | 81.7% |
| 16MiB-1/rustcrypto | c1-asm1-neon | 3 | 8.132 ms | 6.233 ms | 8.888 ms | 42.6% |
| 16MiB-1/rustcrypto | d1-scalar-neon | 3 | 8.730 ms | 6.812 ms | 10.055 ms | 47.6% |
| 1MiB-3/rind | a0-before-neon | 3 | 1.583 ms | 1.059 ms | 1.919 ms | 81.1% |
| 1MiB-3/rind | c1-asm1-neon | 3 | 1.016 ms | 796.67 us | 1.095 ms | 37.4% |
| 1MiB-3/rind | d1-scalar-neon | 3 | 1.402 ms | 1.267 ms | 1.974 ms | 55.8% |
| 1MiB-3/rustcrypto | a0-before-neon | 3 | 1.782 ms | 1.095 ms | 2.078 ms | 89.8% |
| 1MiB-3/rustcrypto | c1-asm1-neon | 3 | 1.662 ms | 1.097 ms | 1.693 ms | 54.3% |
| 1MiB-3/rustcrypto | d1-scalar-neon | 3 | 1.254 ms | 1.151 ms | 1.743 ms | 51.5% |
| 64MiB-1/rind | a0-before-neon | 3 | 50.322 ms | 34.798 ms | 51.582 ms | 48.2% |
| 64MiB-1/rind | c1-asm1-neon | 3 | 30.232 ms | 23.766 ms | 32.371 ms | 36.2% |
| 64MiB-1/rind | d1-scalar-neon | 3 | 45.535 ms | 45.307 ms | 50.130 ms | 10.6% |
| 64MiB-1/rustcrypto | a0-before-neon | 3 | 55.256 ms | 38.718 ms | 62.526 ms | 61.5% |
| 64MiB-1/rustcrypto | c1-asm1-neon | 3 | 42.488 ms | 35.051 ms | 49.670 ms | 41.7% |
| 64MiB-1/rustcrypto | d1-scalar-neon | 3 | 41.818 ms | 37.550 ms | 56.864 ms | 51.4% |
