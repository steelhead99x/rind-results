# start Thu Nov 12 13:58:47 PST 2026 load=13:58  up  5:14, 4 users, load averages: 7.14 9.11 21.14
# end Thu Nov 12 13:59:04 PST 2026 load=13:59  up  5:14, 4 users, load averages: 7.01 8.99 20.89

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=20 med=   0.558 ms min=   0.549 spread=  9.9%
rind       h1-hybrid      n=20 med=   0.526 ms min=   0.510 spread= 17.1%
rustcrypto g1-final       n=20 med=   1.063 ms min=   1.051 spread=  9.8%
rustcrypto h1-hybrid      n=20 med=   1.065 ms min=   1.051 spread= 14.1%

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=20 med=   3.296 ms min=   2.978 spread= 17.0%
rind       h1-hybrid      n=20 med=   3.026 ms min=   2.915 spread= 18.6%
rustcrypto g1-final       n=20 med=   6.060 ms min=   5.834 spread=  8.4%
rustcrypto h1-hybrid      n=20 med=   6.069 ms min=   5.840 spread= 12.7%

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=20 med=  21.017 ms min=  19.299 spread= 20.4%
rind       h1-hybrid      n=20 med=  22.318 ms min=  21.585 spread= 10.1%
rustcrypto g1-final       n=20 med=  33.197 ms min=  31.588 spread=  7.1%
rustcrypto h1-hybrid      n=20 med=  33.490 ms min=  32.201 spread=  6.4%
