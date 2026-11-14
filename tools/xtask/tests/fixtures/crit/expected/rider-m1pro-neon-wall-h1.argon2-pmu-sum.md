# start Thu Nov 12 13:56:49 PST 2026 load=13:56  up  5:12, 4 users, load averages: 6.35 9.90 23.20
# end Thu Nov 12 13:56:57 PST 2026 load=13:56  up  5:12, 4 users, load averages: 6.19 9.75 23.00

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=10 med=   0.743 ms min=   0.710 spread= 11.2%
rind       h1-hybrid      n=10 med=   0.642 ms min=   0.638 spread=  3.9%
rustcrypto g1-final       n=10 med=   1.082 ms min=   1.049 spread= 10.6%
rustcrypto h1-hybrid      n=10 med=   1.107 ms min=   1.089 spread=  9.6%

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=10 med=   3.866 ms min=   3.817 spread=  3.4%
rind       h1-hybrid      n=10 med=   3.335 ms min=   3.278 spread=  3.1%
rustcrypto g1-final       n=10 med=   5.711 ms min=   5.645 spread=  2.8%
rustcrypto h1-hybrid      n=10 med=   5.726 ms min=   5.676 spread=  2.4%

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=10 med=  21.157 ms min=  20.866 spread= 17.7%
rind       h1-hybrid      n=10 med=  20.576 ms min=  20.038 spread= 10.8%
rustcrypto g1-final       n=10 med=  29.205 ms min=  28.638 spread= 12.0%
rustcrypto h1-hybrid      n=10 med=  29.333 ms min=  28.525 spread= 16.9%
