# start Thu Nov 12 13:12:52 PST 2026 load=13:12  up  4:28, 4 users, load averages: 51.40 64.58 62.02
# end Thu Nov 12 13:12:59 PST 2026 load=13:12  up  4:28, 4 users, load averages: 45.65 62.91 61.46

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=   0.820 ms min=   0.796 spread=  8.4% ratio=1.000
rind       f2-x2inl       n=8 med=   0.734 ms min=   0.719 spread=  6.9% ratio=0.896
rustcrypto e1-final       n=8 med=   1.075 ms min=   1.052 spread=  8.8% ratio=1.000
rustcrypto f2-x2inl       n=8 med=   1.071 ms min=   1.058 spread=  3.5% ratio=0.997

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=   4.439 ms min=   4.313 spread=  9.1% ratio=1.000
rind       f2-x2inl       n=8 med=   3.982 ms min=   3.845 spread=  9.7% ratio=0.897
rustcrypto e1-final       n=8 med=   6.136 ms min=   5.961 spread=  5.8% ratio=1.000
rustcrypto f2-x2inl       n=8 med=   6.156 ms min=   5.858 spread=  7.0% ratio=1.003

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=  27.037 ms min=  26.185 spread=  7.5% ratio=1.000
rind       f2-x2inl       n=8 med=  25.752 ms min=  24.755 spread=  7.2% ratio=0.952
rustcrypto e1-final       n=8 med=  33.508 ms min=  32.891 spread=  5.8% ratio=1.000
rustcrypto f2-x2inl       n=8 med=  33.814 ms min=  32.875 spread=  5.2% ratio=1.009
