# start Thu Nov 12 13:56:41 PST 2026 load=13:56  up  5:12, 4 users, load averages: 6.55 10.00 23.32
# end Thu Nov 12 13:56:49 PST 2026 load=13:56  up  5:12, 4 users, load averages: 6.35 9.90 23.20

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=10 med=   0.575 ms min=   0.571 spread=100.4%
rind       h1-hybrid      n=10 med=   0.540 ms min=   0.535 spread= 72.7%
rustcrypto g1-final       n=10 med=   1.107 ms min=   1.094 spread=100.0%
rustcrypto h1-hybrid      n=10 med=   1.109 ms min=   1.085 spread= 75.5%

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=10 med=   3.024 ms min=   2.950 spread= 21.7%
rind       h1-hybrid      n=10 med=   2.752 ms min=   2.725 spread= 19.0%
rustcrypto g1-final       n=10 med=   5.745 ms min=   5.602 spread= 20.7%
rustcrypto h1-hybrid      n=10 med=   5.721 ms min=   5.625 spread= 15.2%

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=10 med=  18.509 ms min=  17.035 spread= 24.2%
rind       h1-hybrid      n=10 med=  18.828 ms min=  17.389 spread= 23.9%
rustcrypto g1-final       n=10 med=  29.132 ms min=  28.085 spread= 21.6%
rustcrypto h1-hybrid      n=10 med=  28.957 ms min=  28.028 spread= 11.7%
