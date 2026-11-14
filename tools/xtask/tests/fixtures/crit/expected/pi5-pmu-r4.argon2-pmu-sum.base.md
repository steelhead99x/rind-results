# start Thu 12 Nov 13:45:47 PST 2026 load=1.10, 2.18, 2.65 temp=82.9'C
# end Thu 12 Nov 13:46:29 PST 2026 load=1.04, 2.01, 2.57 temp=83.4'C throttled=0xe0008

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med=   4.732 ms min=   4.730 spread=  0.1% ratio=1.000 cycles=2307.25 inst=2816.10 stall_be=1560.45 stall_fe=1.00 l1d_refill=11.90 ipc=1.22 ghz=1.50
rind       e1-final       n=8 med=   4.735 ms min=   4.732 spread=  0.1% ratio=1.001 cycles=2308.50 inst=2816.10 stall_be=1561.25 stall_fe=1.10 l1d_refill=11.65 ipc=1.22 ghz=1.50
rind       g1-final       n=8 med=   3.123 ms min=   3.120 spread=  0.2% ratio=0.660 cycles=1521.95 inst=2664.40 stall_be=756.30 stall_fe=1.35 l1d_refill=17.90 ipc=1.75 ghz=1.50
rustcrypto a0-before      n=8 med=   4.201 ms min=   4.197 spread=  0.4% ratio=1.000 cycles=2048.40 inst=4365.20 stall_be=450.95 stall_fe=0.95 l1d_refill=13.60 ipc=2.13 ghz=1.50
rustcrypto e1-final       n=8 med=   4.204 ms min=   4.199 spread=  0.2% ratio=1.001 cycles=2049.05 inst=4365.15 stall_be=450.00 stall_fe=0.95 l1d_refill=13.60 ipc=2.13 ghz=1.50
rustcrypto g1-final       n=8 med=   4.203 ms min=   4.197 spread=  0.4% ratio=1.000 cycles=2048.55 inst=4365.20 stall_be=450.60 stall_fe=1.10 l1d_refill=13.75 ipc=2.13 ghz=1.50

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med=  28.079 ms min=  28.054 spread=  0.3% ratio=1.000 cycles=2562.30 inst=2762.80 stall_be=1823.85 stall_fe=0.85 l1d_refill=20.35 ipc=1.08 ghz=1.50
rind       e1-final       n=8 med=  28.093 ms min=  28.078 spread=  0.2% ratio=1.000 cycles=2564.60 inst=2762.80 stall_be=1825.95 stall_fe=0.90 l1d_refill=19.80 ipc=1.08 ghz=1.50
rind       g1-final       n=8 med=  19.919 ms min=  19.896 spread=  0.2% ratio=0.709 cycles=1817.05 inst=2607.60 stall_be=1059.25 stall_fe=1.00 l1d_refill=26.20 ipc=1.44 ghz=1.50
rustcrypto a0-before      n=8 med=  26.344 ms min=  26.268 spread=  0.7% ratio=1.000 cycles=2404.00 inst=4384.70 stall_be=783.95 stall_fe=1.55 l1d_refill=16.20 ipc=1.82 ghz=1.50
rustcrypto e1-final       n=8 med=  26.387 ms min=  26.342 spread=  0.5% ratio=1.002 cycles=2407.50 inst=4384.70 stall_be=787.35 stall_fe=1.55 l1d_refill=16.20 ipc=1.82 ghz=1.50
rustcrypto g1-final       n=8 med=  26.389 ms min=  26.268 spread=  0.8% ratio=1.002 cycles=2407.60 inst=4384.70 stall_be=785.00 stall_fe=1.20 l1d_refill=16.55 ipc=1.82 ghz=1.50

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med= 124.314 ms min= 123.853 spread=  0.4% ratio=1.000 cycles=2523.20 inst=2754.20 stall_be=1787.50 stall_fe=1.20 l1d_refill=19.10 ipc=1.09 ghz=1.33
rind       e1-final       n=8 med= 124.340 ms min= 124.181 spread=  0.2% ratio=1.000 cycles=2524.60 inst=2754.20 stall_be=1788.75 stall_fe=1.20 l1d_refill=18.80 ipc=1.09 ghz=1.33
rind       g1-final       n=8 med=  90.550 ms min=  90.248 spread=  0.4% ratio=0.728 cycles=1752.65 inst=2599.00 stall_be=997.55 stall_fe=1.30 l1d_refill=27.10 ipc=1.48 ghz=1.27
rustcrypto a0-before      n=8 med= 117.577 ms min= 117.216 spread=  1.0% ratio=1.000 cycles=2370.50 inst=4379.30 stall_be=755.30 stall_fe=1.70 l1d_refill=18.40 ipc=1.85 ghz=1.32
rustcrypto e1-final       n=8 med= 118.002 ms min= 117.365 spread=  1.0% ratio=1.004 cycles=2379.65 inst=4379.30 stall_be=765.30 stall_fe=1.65 l1d_refill=18.50 ipc=1.84 ghz=1.32
rustcrypto g1-final       n=8 med= 118.543 ms min= 117.726 spread=  1.1% ratio=1.008 cycles=2390.10 inst=4379.30 stall_be=770.65 stall_fe=1.30 l1d_refill=18.80 ipc=1.83 ghz=1.32
