# start Thu Nov 12 13:25:06 PST 2026 load=13:25  up  4:40, 4 users, load averages: 60.05 61.20 60.24
# end Thu Nov 12 13:25:20 PST 2026 load=13:25  up  4:40, 4 users, load averages: 49.21 58.74 59.38

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=10 med=   0.820 ms min=   0.796 spread= 12.3% ratio=1.000
rind       f2-x2inl       n=10 med=   0.726 ms min=   0.712 spread=  8.7% ratio=0.885
rind       f3-x2umlal     n=10 med=   0.642 ms min=   0.617 spread= 25.4% ratio=0.783
rustcrypto e1-final       n=10 med=   1.073 ms min=   1.056 spread=  8.2% ratio=1.000
rustcrypto f2-x2inl       n=10 med=   1.075 ms min=   1.052 spread=  6.1% ratio=1.002
rustcrypto f3-x2umlal     n=10 med=   1.101 ms min=   1.051 spread= 31.2% ratio=1.027

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=10 med=   4.506 ms min=   4.412 spread=  9.6% ratio=1.000
rind       f2-x2inl       n=10 med=   4.119 ms min=   3.997 spread=  8.7% ratio=0.914
rind       f3-x2umlal     n=10 med=   3.707 ms min=   3.465 spread= 11.3% ratio=0.823
rustcrypto e1-final       n=10 med=   6.252 ms min=   5.851 spread=  9.2% ratio=1.000
rustcrypto f2-x2inl       n=10 med=   6.237 ms min=   5.783 spread= 10.8% ratio=0.998
rustcrypto f3-x2umlal     n=10 med=   6.079 ms min=   5.845 spread=  7.6% ratio=0.972

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=10 med=  27.099 ms min=  25.606 spread= 10.2% ratio=1.000
rind       f2-x2inl       n=10 med=  25.343 ms min=  24.823 spread=  6.6% ratio=0.935
rind       f3-x2umlal     n=10 med=  23.416 ms min=  22.668 spread=  7.0% ratio=0.864
rustcrypto e1-final       n=10 med=  34.269 ms min=  31.829 spread=  9.7% ratio=1.000
rustcrypto f2-x2inl       n=10 med=  34.583 ms min=  31.501 spread= 10.6% ratio=1.009
rustcrypto f3-x2umlal     n=10 med=  33.748 ms min=  33.100 spread=  7.0% ratio=0.985
