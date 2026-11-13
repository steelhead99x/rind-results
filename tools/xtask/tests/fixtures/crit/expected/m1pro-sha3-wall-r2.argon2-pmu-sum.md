# start Thu Nov 12 13:24:53 PST 2026 load=13:24  up  4:40, 4 users, load averages: 69.50 62.96 60.83
# end Thu Nov 12 13:25:06 PST 2026 load=13:25  up  4:40, 4 users, load averages: 60.05 61.20 60.24

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=10 med=   0.629 ms min=   0.622 spread=  4.6%
rind       f2-x2inl       n=10 med=   0.556 ms min=   0.548 spread=  4.3%
rind       f3-x2umlal     n=10 med=   0.555 ms min=   0.548 spread= 31.4%
rustcrypto e1-final       n=10 med=   1.071 ms min=   1.051 spread=  5.8%
rustcrypto f2-x2inl       n=10 med=   1.069 ms min=   1.048 spread=  4.4%
rustcrypto f3-x2umlal     n=10 med=   1.065 ms min=   1.052 spread= 21.4%

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=10 med=   3.716 ms min=   3.427 spread= 11.2%
rind       f2-x2inl       n=10 med=   3.377 ms min=   3.197 spread=  9.9%
rind       f3-x2umlal     n=10 med=   3.372 ms min=   3.163 spread= 18.7%
rustcrypto e1-final       n=10 med=   6.175 ms min=   5.909 spread=  9.3%
rustcrypto f2-x2inl       n=10 med=   5.983 ms min=   5.847 spread=  8.4%
rustcrypto f3-x2umlal     n=10 med=   6.051 ms min=   5.831 spread= 23.9%

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=10 med=  23.796 ms min=  22.552 spread=  7.5%
rind       f2-x2inl       n=10 med=  22.993 ms min=  21.942 spread=  8.1%
rind       f3-x2umlal     n=10 med=  22.200 ms min=  21.709 spread=  7.8%
rustcrypto e1-final       n=10 med=  34.682 ms min=  33.830 spread=  6.4%
rustcrypto f2-x2inl       n=10 med=  34.734 ms min=  33.754 spread=  5.5%
rustcrypto f3-x2umlal     n=10 med=  34.520 ms min=  33.040 spread=  7.4%
