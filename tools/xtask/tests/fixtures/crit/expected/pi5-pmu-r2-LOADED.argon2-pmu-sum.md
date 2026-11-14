# start Thu 12 Nov 13:23:56 PST 2026 load=3.74, 1.97, 1.55 temp=88.4'C
# end Thu 12 Nov 13:24:45 PST 2026 load=4.04, 2.32, 1.69 temp=89.5'C throttled=0xe0006

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=   5.005 ms min=   4.901 spread=119.7% cycles=2414.40 inst=2816.10 stall_be=1663.40 stall_fe=4.85 l1d_refill=11.75 ipc=1.17 ghz=1.48
rind       f2-x2inl       n=8 med=   3.528 ms min=   3.267 spread= 49.9% cycles=1675.85 inst=2664.40 stall_be=904.75 stall_fe=5.55 l1d_refill=17.40 ipc=1.59 ghz=1.48
rind       f3-x2umlal     n=8 med=   3.702 ms min=   3.430 spread= 50.7% cycles=1750.65 inst=2650.40 stall_be=917.00 stall_fe=5.10 l1d_refill=18.30 ipc=1.51 ghz=1.49
rustcrypto e1-final       n=8 med=   4.699 ms min=   4.543 spread= 67.7% cycles=2271.40 inst=4365.20 stall_be=667.65 stall_fe=6.80 l1d_refill=14.30 ipc=1.92 ghz=1.49
rustcrypto f2-x2inl       n=8 med=   5.005 ms min=   4.526 spread= 47.5% cycles=2341.55 inst=4365.20 stall_be=735.30 stall_fe=7.55 l1d_refill=14.50 ipc=1.86 ghz=1.48
rustcrypto f3-x2umlal     n=8 med=   4.939 ms min=   4.490 spread= 50.4% cycles=2318.05 inst=4365.20 stall_be=710.80 stall_fe=6.65 l1d_refill=14.40 ipc=1.89 ghz=1.49

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med=  30.404 ms min=  29.312 spread= 12.1% cycles=2757.90 inst=2762.80 stall_be=2019.40 stall_fe=1.10 l1d_refill=19.70 ipc=1.00 ghz=1.49
rind       f2-x2inl       n=8 med=  23.710 ms min=  21.714 spread= 35.4% cycles=2000.00 inst=2607.60 stall_be=1240.35 stall_fe=1.20 l1d_refill=26.05 ipc=1.31 ghz=1.48
rind       f3-x2umlal     n=8 med=  24.897 ms min=  22.558 spread= 25.1% cycles=2115.20 inst=2593.50 stall_be=1300.50 stall_fe=1.15 l1d_refill=25.75 ipc=1.23 ghz=1.49
rustcrypto e1-final       n=8 med=  29.320 ms min=  27.946 spread= 11.0% cycles=2649.90 inst=4384.70 stall_be=1029.60 stall_fe=2.05 l1d_refill=16.65 ipc=1.65 ghz=1.49
rustcrypto f2-x2inl       n=8 med=  31.383 ms min=  28.457 spread= 42.6% cycles=2640.00 inst=4384.70 stall_be=1018.15 stall_fe=1.40 l1d_refill=17.10 ipc=1.66 ghz=1.49
rustcrypto f3-x2umlal     n=8 med=  30.107 ms min=  28.427 spread= 29.8% cycles=2661.90 inst=4384.70 stall_be=1047.85 stall_fe=1.40 l1d_refill=16.75 ipc=1.65 ghz=1.49

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       e1-final       n=8 med= 138.129 ms min= 132.263 spread= 12.4% cycles=2680.15 inst=2754.20 stall_be=1944.30 stall_fe=1.50 l1d_refill=18.80 ipc=1.03 ghz=1.27
rind       f2-x2inl       n=8 med= 109.356 ms min=  98.560 spread= 27.5% cycles=1895.55 inst=2599.00 stall_be=1141.00 stall_fe=1.45 l1d_refill=27.10 ipc=1.38 ghz=1.16
rind       f3-x2umlal     n=8 med= 110.201 ms min= 106.610 spread= 23.2% cycles=2006.50 inst=2584.90 stall_be=1188.05 stall_fe=1.50 l1d_refill=26.20 ipc=1.29 ghz=1.21
rustcrypto e1-final       n=8 med= 150.899 ms min= 127.787 spread= 24.4% cycles=2531.85 inst=4379.30 stall_be=916.25 stall_fe=1.95 l1d_refill=18.70 ipc=1.73 ghz=1.17
rustcrypto f2-x2inl       n=8 med= 144.976 ms min= 126.948 spread= 24.2% cycles=2541.45 inst=4379.30 stall_be=926.50 stall_fe=1.45 l1d_refill=18.95 ipc=1.72 ghz=1.23
rustcrypto f3-x2umlal     n=8 med= 146.447 ms min= 132.644 spread= 28.1% cycles=2554.15 inst=4379.30 stall_be=939.10 stall_fe=1.45 l1d_refill=18.80 ipc=1.71 ghz=1.15
