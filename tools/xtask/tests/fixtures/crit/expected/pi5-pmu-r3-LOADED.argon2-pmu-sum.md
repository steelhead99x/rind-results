# start Thu 12 Nov 13:35:10 PST 2026 load=4.15, 3.95, 2.81 temp=92.2'C
# end Thu 12 Nov 13:36:13 PST 2026 load=3.83, 3.89, 2.87 temp=91.1'C throttled=0xe0006

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med=   7.244 ms min=   7.110 spread=  4.7% cycles=2334.90 inst=2816.10 stall_be=1585.80 stall_fe=3.10 l1d_refill=11.80 ipc=1.21 ghz=0.99
rind       e1-final       n=8 med=   7.368 ms min=   7.118 spread=  5.9% cycles=2374.50 inst=2816.10 stall_be=1624.65 stall_fe=3.30 l1d_refill=11.85 ipc=1.19 ghz=0.99
rind       g1-final       n=8 med=   4.948 ms min=   4.699 spread=  7.1% cycles=1600.30 inst=2664.40 stall_be=833.15 stall_fe=3.85 l1d_refill=17.70 ipc=1.67 ghz=0.99
rustcrypto a0-before      n=8 med=   6.617 ms min=   6.327 spread= 11.6% cycles=2133.85 inst=4365.20 stall_be=530.15 stall_fe=4.30 l1d_refill=13.85 ipc=2.05 ghz=0.99
rustcrypto e1-final       n=8 med=   6.859 ms min=   6.342 spread= 10.9% cycles=2215.70 inst=4365.20 stall_be=612.00 stall_fe=4.55 l1d_refill=14.25 ipc=1.97 ghz=0.99
rustcrypto g1-final       n=8 med=   6.829 ms min=   6.346 spread=  8.7% cycles=2200.45 inst=4365.20 stall_be=597.50 stall_fe=4.45 l1d_refill=14.40 ipc=1.98 ghz=0.99

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med=  42.556 ms min=  41.139 spread=  5.1% cycles=2581.75 inst=2762.80 stall_be=1844.00 stall_fe=0.70 l1d_refill=19.90 ipc=1.07 ghz=0.99
rind       e1-final       n=8 med=  42.941 ms min=  41.281 spread=  4.8% cycles=2604.60 inst=2762.80 stall_be=1866.30 stall_fe=0.85 l1d_refill=19.90 ipc=1.06 ghz=0.99
rind       g1-final       n=8 med=  30.151 ms min=  28.872 spread=  5.8% cycles=1828.45 inst=2607.60 stall_be=1073.15 stall_fe=0.90 l1d_refill=26.90 ipc=1.42 ghz=0.99
rustcrypto a0-before      n=8 med=  40.577 ms min=  38.489 spread=  6.4% cycles=2459.50 inst=4384.70 stall_be=838.10 stall_fe=1.45 l1d_refill=16.75 ipc=1.79 ghz=0.99
rustcrypto e1-final       n=8 med=  40.896 ms min=  38.818 spread=  6.3% cycles=2478.20 inst=4384.70 stall_be=856.75 stall_fe=1.50 l1d_refill=16.70 ipc=1.77 ghz=0.99
rustcrypto g1-final       n=8 med=  40.764 ms min=  38.380 spread=  7.6% cycles=2471.75 inst=4384.70 stall_be=849.60 stall_fe=1.10 l1d_refill=17.00 ipc=1.77 ghz=0.99

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med= 188.500 ms min= 181.376 spread=  4.7% cycles=2528.00 inst=2754.20 stall_be=1792.80 stall_fe=1.10 l1d_refill=19.10 ipc=1.09 ghz=0.88
rind       e1-final       n=8 med= 188.575 ms min= 180.661 spread=  5.1% cycles=2539.00 inst=2754.20 stall_be=1803.50 stall_fe=1.30 l1d_refill=19.05 ipc=1.08 ghz=0.89
rind       g1-final       n=8 med= 132.900 ms min= 129.666 spread=  6.2% cycles=1725.05 inst=2599.00 stall_be=969.15 stall_fe=1.30 l1d_refill=27.45 ipc=1.50 ghz=0.85
rustcrypto a0-before      n=8 med= 181.252 ms min= 172.705 spread=  6.8% cycles=2419.55 inst=4379.30 stall_be=803.15 stall_fe=1.70 l1d_refill=18.80 ipc=1.81 ghz=0.88
rustcrypto e1-final       n=8 med= 180.495 ms min= 171.155 spread=  7.6% cycles=2418.70 inst=4379.30 stall_be=803.80 stall_fe=1.70 l1d_refill=18.70 ipc=1.81 ghz=0.88
rustcrypto g1-final       n=8 med= 177.612 ms min= 170.807 spread=  6.1% cycles=2389.05 inst=4379.30 stall_be=771.70 stall_fe=1.30 l1d_refill=18.90 ipc=1.83 ghz=0.89
