# start Thu 12 Nov 13:20:55 PST 2026 load=0.92, 1.00, 1.24 temp=77.4'C
# end Thu 12 Nov 13:21:37 PST 2026 load=0.96, 1.00, 1.23 temp=79.0'C throttled=0xe0000

## 1MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med=   4.732 ms min=   4.730 spread=  0.1% cycles=2307.45 inst=2816.10 stall_be=1560.50 stall_fe=1.00 l1d_refill=11.80 ipc=1.22 ghz=1.50
rind       e1-final       n=8 med=   4.735 ms min=   4.732 spread=  0.2% cycles=2308.65 inst=2816.10 stall_be=1561.10 stall_fe=1.10 l1d_refill=11.80 ipc=1.22 ghz=1.50
rind       f2-x2inl       n=8 med=   3.122 ms min=   3.120 spread=  0.1% cycles=1522.25 inst=2664.40 stall_be=756.70 stall_fe=1.20 l1d_refill=17.90 ipc=1.75 ghz=1.50
rustcrypto a0-before      n=8 med=   4.200 ms min=   4.197 spread=  0.3% cycles=2047.75 inst=4365.20 stall_be=450.40 stall_fe=1.00 l1d_refill=13.55 ipc=2.13 ghz=1.50
rustcrypto e1-final       n=8 med=   4.204 ms min=   4.201 spread=  0.3% cycles=2049.80 inst=4365.20 stall_be=450.90 stall_fe=1.00 l1d_refill=13.50 ipc=2.13 ghz=1.50
rustcrypto f2-x2inl       n=8 med=   4.204 ms min=   4.197 spread=  0.3% cycles=2048.40 inst=4365.20 stall_be=452.80 stall_fe=0.95 l1d_refill=13.80 ipc=2.13 ghz=1.50

## 16MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med=  28.090 ms min=  28.042 spread=  0.2% cycles=2563.90 inst=2762.80 stall_be=1825.45 stall_fe=0.80 l1d_refill=20.15 ipc=1.08 ghz=1.50
rind       e1-final       n=8 med=  28.123 ms min=  28.060 spread=  1.9% cycles=2565.05 inst=2762.80 stall_be=1826.70 stall_fe=0.90 l1d_refill=20.00 ipc=1.08 ghz=1.50
rind       f2-x2inl       n=8 med=  19.916 ms min=  19.885 spread=  0.3% cycles=1816.60 inst=2607.60 stall_be=1058.80 stall_fe=1.00 l1d_refill=26.20 ipc=1.44 ghz=1.50
rustcrypto a0-before      n=8 med=  26.367 ms min=  26.312 spread=  0.5% cycles=2406.85 inst=4384.70 stall_be=787.05 stall_fe=1.55 l1d_refill=16.20 ipc=1.82 ghz=1.50
rustcrypto e1-final       n=8 med=  26.404 ms min=  26.307 spread=  3.2% cycles=2408.70 inst=4384.70 stall_be=786.70 stall_fe=1.55 l1d_refill=16.15 ipc=1.82 ghz=1.50
rustcrypto f2-x2inl       n=8 med=  26.422 ms min=  26.279 spread=  0.9% cycles=2409.65 inst=4384.70 stall_be=790.40 stall_fe=1.10 l1d_refill=16.50 ipc=1.82 ghz=1.50

## 64MiB  (n = alternations; spread = (max-min)/median of the per-alternation medians)
rind       a0-before      n=8 med= 124.078 ms min= 123.882 spread=  0.3% cycles=2523.00 inst=2754.20 stall_be=1787.30 stall_fe=1.20 l1d_refill=19.10 ipc=1.09 ghz=1.33
rind       e1-final       n=8 med= 124.081 ms min= 123.366 spread=  1.1% cycles=2523.75 inst=2754.20 stall_be=1787.90 stall_fe=1.20 l1d_refill=18.95 ipc=1.09 ghz=1.33
rind       f2-x2inl       n=8 med=  90.262 ms min=  90.151 spread=  0.3% cycles=1752.30 inst=2599.00 stall_be=997.50 stall_fe=1.30 l1d_refill=27.10 ipc=1.48 ghz=1.27
rustcrypto a0-before      n=8 med= 117.475 ms min= 116.941 spread=  1.2% cycles=2375.15 inst=4379.30 stall_be=760.90 stall_fe=1.70 l1d_refill=18.50 ipc=1.85 ghz=1.32
rustcrypto e1-final       n=8 med= 117.518 ms min= 117.113 spread=  2.0% cycles=2373.25 inst=4379.30 stall_be=758.40 stall_fe=1.70 l1d_refill=18.40 ipc=1.85 ghz=1.32
rustcrypto f2-x2inl       n=8 med= 117.920 ms min= 117.294 spread=  0.9% cycles=2382.60 inst=4379.30 stall_be=769.05 stall_fe=1.30 l1d_refill=18.70 ipc=1.84 ghz=1.32
