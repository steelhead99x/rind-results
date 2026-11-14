# start Thu 12 Nov 14:04:41 PST 2026 load=1.94, 2.92, 2.65 temp=87.8'C
# end Thu 12 Nov 14:05:06 PST 2026 load=1.62, 2.77, 2.61 temp=86.2'C throttled=0xe0006

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=   3.123 ms min=   3.121 spread=  0.2% ratio=1.000 cycles=1521.85 inst=2664.40 stall_be=756.05 stall_fe=1.20 l1d_refill=17.90 ipc=1.75 ghz=1.50
rind       h2-hybrid-neon n=8 med=   2.582 ms min=   2.578 spread=  3.5% ratio=0.827 cycles=1258.85 inst=3300.90 stall_be=345.05 stall_fe=1.55 l1d_refill=21.30 ipc=2.62 ghz=1.50
rustcrypto g1-final       n=8 med=   4.202 ms min=   4.197 spread=  0.4% ratio=1.000 cycles=2048.55 inst=4365.20 stall_be=451.15 stall_fe=1.00 l1d_refill=13.70 ipc=2.13 ghz=1.50
rustcrypto h2-hybrid-neon n=8 med=   4.207 ms min=   4.198 spread=  3.5% ratio=1.001 cycles=2048.75 inst=4365.20 stall_be=451.10 stall_fe=1.25 l1d_refill=13.60 ipc=2.13 ghz=1.50

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=  19.905 ms min=  19.884 spread=  0.4% ratio=1.000 cycles=1817.00 inst=2607.60 stall_be=1058.70 stall_fe=1.00 l1d_refill=26.20 ipc=1.44 ghz=1.50
rind       h2-hybrid-neon n=8 med=  18.056 ms min=  17.833 spread=  6.2% ratio=0.907 cycles=1645.35 inst=3214.20 stall_be=744.40 stall_fe=1.30 l1d_refill=28.55 ipc=1.95 ghz=1.49
rustcrypto g1-final       n=8 med=  26.437 ms min=  26.329 spread=  0.7% ratio=1.000 cycles=2410.85 inst=4384.70 stall_be=791.35 stall_fe=1.20 l1d_refill=16.55 ipc=1.82 ghz=1.50
rustcrypto h2-hybrid-neon n=8 med=  26.447 ms min=  26.307 spread=  4.6% ratio=1.000 cycles=2412.10 inst=4384.70 stall_be=800.45 stall_fe=1.45 l1d_refill=16.30 ipc=1.82 ghz=1.49

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       g1-final       n=8 med=  91.470 ms min=  91.368 spread=  0.3% ratio=1.000 cycles=1757.70 inst=2599.00 stall_be=1002.45 stall_fe=1.30 l1d_refill=27.00 ipc=1.48 ghz=1.26
rind       h2-hybrid-neon n=8 med=  83.781 ms min=  83.153 spread=  1.1% ratio=0.916 cycles=1584.05 inst=3205.70 stall_be=685.80 stall_fe=1.40 l1d_refill=28.40 ipc=2.02 ghz=1.24
rustcrypto g1-final       n=8 med= 119.263 ms min= 118.377 spread=  1.1% ratio=1.000 cycles=2390.85 inst=4379.30 stall_be=778.40 stall_fe=1.35 l1d_refill=18.70 ipc=1.83 ghz=1.31
rustcrypto h2-hybrid-neon n=8 med= 118.763 ms min= 117.958 spread=  1.2% ratio=0.996 cycles=2380.90 inst=4379.30 stall_be=767.30 stall_fe=1.55 l1d_refill=18.50 ipc=1.84 ghz=1.31
