# start Thu 12 Nov 13:56:10 PST 2026 load=4.11, 2.51, 2.35 temp=89.5'C
# end Thu 12 Nov 13:56:36 PST 2026 load=3.05, 2.39, 2.32 temp=86.7'C throttled=0xe0006

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=   3.122 ms min=   3.119 spread= 58.3% cycles=1522.00 inst=2664.40 stall_be=756.40 stall_fe=1.25 l1d_refill=17.90 ipc=1.75 ghz=1.50
rind       h1-hybrid      n=8 med=   2.579 ms min=   2.566 spread=203.8% cycles=1259.10 inst=3301.70 stall_be=347.65 stall_fe=4.45 l1d_refill=20.90 ipc=2.62 ghz=1.50
rustcrypto g1-final       n=8 med=   4.204 ms min=   4.195 spread= 63.3% cycles=2047.50 inst=4365.20 stall_be=450.50 stall_fe=1.05 l1d_refill=13.70 ipc=2.13 ghz=1.50
rustcrypto h1-hybrid      n=8 med=   4.294 ms min=   4.203 spread= 64.4% cycles=2065.05 inst=4365.15 stall_be=462.25 stall_fe=3.50 l1d_refill=13.45 ipc=2.12 ghz=1.50

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=  19.928 ms min=  19.887 spread= 50.1% cycles=1817.00 inst=2607.60 stall_be=1058.45 stall_fe=0.95 l1d_refill=26.20 ipc=1.44 ghz=1.50
rind       h1-hybrid      n=8 med=  17.859 ms min=  17.750 spread= 39.1% cycles=1628.05 inst=3215.20 stall_be=726.40 stall_fe=1.40 l1d_refill=28.55 ipc=1.97 ghz=1.49
rustcrypto g1-final       n=8 med=  26.452 ms min=  26.425 spread= 51.5% cycles=2412.80 inst=4384.70 stall_be=793.85 stall_fe=1.20 l1d_refill=16.60 ipc=1.82 ghz=1.49
rustcrypto h1-hybrid      n=8 med=  26.430 ms min=  26.307 spread= 44.9% cycles=2408.00 inst=4384.70 stall_be=787.50 stall_fe=1.15 l1d_refill=16.20 ipc=1.82 ghz=1.49

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=  91.390 ms min=  91.103 spread= 49.2% cycles=1755.10 inst=2599.00 stall_be=1000.05 stall_fe=1.30 l1d_refill=27.00 ipc=1.48 ghz=1.26
rind       h1-hybrid      n=8 med=  83.756 ms min=  83.178 spread=  1.4% cycles=1580.10 inst=3206.60 stall_be=681.10 stall_fe=1.55 l1d_refill=28.50 ipc=2.03 ghz=1.24
rustcrypto g1-final       n=8 med= 119.166 ms min= 118.803 spread= 51.1% cycles=2397.70 inst=4379.30 stall_be=785.25 stall_fe=1.30 l1d_refill=18.85 ipc=1.83 ghz=1.31
rustcrypto h1-hybrid      n=8 med= 118.800 ms min= 118.312 spread=  0.6% cycles=2384.60 inst=4379.30 stall_be=769.45 stall_fe=1.30 l1d_refill=18.50 ipc=1.84 ghz=1.31
