# start Thu Nov 12 13:12:44 PST 2026 load=13:12  up  4:28, 4 users, load averages: 58.01 66.26 62.58
# end Thu Nov 12 13:12:52 PST 2026 load=13:12  up  4:28, 4 users, load averages: 51.40 64.58 62.02

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=   0.647 ms min=   0.625 spread=124.8% ratio=1.000
rind       f2-x2inl       n=8 med=   0.573 ms min=   0.556 spread= 45.8% ratio=0.886
rustcrypto e1-final       n=8 med=   1.091 ms min=   1.065 spread=102.9% ratio=1.000
rustcrypto f2-x2inl       n=8 med=   1.095 ms min=   1.058 spread= 21.2% ratio=1.004

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=   3.512 ms min=   3.373 spread= 16.9% ratio=1.000
rind       f2-x2inl       n=8 med=   3.254 ms min=   3.078 spread= 33.6% ratio=0.927
rustcrypto e1-final       n=8 med=   6.306 ms min=   5.752 spread= 16.2% ratio=1.000
rustcrypto f2-x2inl       n=8 med=   6.284 ms min=   5.824 spread= 30.2% ratio=0.997

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=  23.287 ms min=  22.663 spread=  8.6% ratio=1.000
rind       f2-x2inl       n=8 med=  22.049 ms min=  20.812 spread= 21.6% ratio=0.947
rustcrypto e1-final       n=8 med=  33.351 ms min=  32.488 spread=  9.2% ratio=1.000
rustcrypto f2-x2inl       n=8 med=  34.084 ms min=  32.751 spread= 14.1% ratio=1.022
