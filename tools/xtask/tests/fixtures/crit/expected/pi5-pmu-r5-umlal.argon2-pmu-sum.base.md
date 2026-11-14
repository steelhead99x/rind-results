# start Thu 12 Nov 13:48:53 PST 2026 load=0.89, 1.57, 2.32 temp=80.7'C
# end Thu 12 Nov 13:49:18 PST 2026 load=0.93, 1.51, 2.27 temp=81.2'C throttled=0xe0008

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=   3.123 ms min=   3.119 spread=  0.2% ratio=1.000 cycles=1522.00 inst=2664.40 stall_be=756.65 stall_fe=1.35 l1d_refill=17.90 ipc=1.75 ghz=1.50
rind       f3-x2umlal     n=8 med=   3.278 ms min=   3.276 spread=  0.1% ratio=1.050 cycles=1598.35 inst=2650.40 stall_be=771.05 stall_fe=1.10 l1d_refill=18.50 ipc=1.66 ghz=1.50
rustcrypto g1-final       n=8 med=   4.204 ms min=   4.197 spread=  0.5% ratio=1.000 cycles=2049.05 inst=4365.20 stall_be=452.05 stall_fe=0.95 l1d_refill=13.70 ipc=2.13 ghz=1.50
rustcrypto f3-x2umlal     n=8 med=   4.202 ms min=   4.199 spread=  0.2% ratio=0.999 cycles=2048.10 inst=4365.20 stall_be=450.80 stall_fe=1.05 l1d_refill=13.70 ipc=2.13 ghz=1.50

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=  19.918 ms min=  19.879 spread=  0.3% ratio=1.000 cycles=1816.85 inst=2607.60 stall_be=1058.95 stall_fe=1.00 l1d_refill=26.25 ipc=1.44 ghz=1.50
rind       f3-x2umlal     n=8 med=  20.724 ms min=  20.701 spread=  0.2% ratio=1.040 cycles=1890.80 inst=2593.50 stall_be=1069.10 stall_fe=1.00 l1d_refill=25.90 ipc=1.37 ghz=1.50
rustcrypto g1-final       n=8 med=  26.439 ms min=  26.267 spread=  1.1% ratio=1.000 cycles=2411.40 inst=4384.70 stall_be=790.20 stall_fe=1.20 l1d_refill=16.50 ipc=1.82 ghz=1.50
rustcrypto f3-x2umlal     n=8 med=  26.369 ms min=  26.262 spread=  1.1% ratio=0.997 cycles=2403.85 inst=4384.70 stall_be=783.90 stall_fe=1.25 l1d_refill=16.15 ipc=1.83 ghz=1.50

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=  90.267 ms min=  90.191 spread=  0.3% ratio=1.000 cycles=1750.90 inst=2599.00 stall_be=996.25 stall_fe=1.30 l1d_refill=27.00 ipc=1.48 ghz=1.27
rind       f3-x2umlal     n=8 med=  93.645 ms min=  93.342 spread=  0.5% ratio=1.037 cycles=1827.25 inst=2584.90 stall_be=1009.35 stall_fe=1.30 l1d_refill=26.40 ipc=1.42 ghz=1.28
rustcrypto g1-final       n=8 med= 118.281 ms min= 117.517 spread=  0.9% ratio=1.000 cycles=2393.95 inst=4379.30 stall_be=776.10 stall_fe=1.30 l1d_refill=18.70 ipc=1.83 ghz=1.32
rustcrypto f3-x2umlal     n=8 med= 117.615 ms min= 116.961 spread=  1.0% ratio=0.994 cycles=2375.70 inst=4379.30 stall_be=761.95 stall_fe=1.30 l1d_refill=18.50 ipc=1.84 ghz=1.32
