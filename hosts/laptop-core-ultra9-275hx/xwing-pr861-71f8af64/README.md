# X-Wing bench rows for kixel-ai/rind#861 at 71f8af64 on laptop-core-ultra9-275hx

Results only, no code changes. The upstream report (`admin/bench-report`) does not list this host, so these are indicative numbers for the claim's empty X-Wing rows, not report rows.

Intel Core Ultra 9 275HX laptop, core 11 (an E-core), shared and at a load average of 40 to 60 on 24 CPUs throughout. PROVISIONAL: 3 rounds of criterion with 2 s of measurement; only one round survived for keygen and encaps, none for decaps.

Method: `rind-bench`'s `xwing` bench (rind against libcrux-kem 0.0.10 and RustCrypto `x-wing` 0.1.1), interleaved A/B with the order rotating each round and rind repeated last as the control, pinned with `taskset`, criterion `--warm-up-time 1 --measurement-time 3`. A round is kept for an operation when its two rind times are within 1.02; the cell is the median of the kept rounds' central estimates.

| Operation | rind | libcrux-kem | RustCrypto | Kept | Discarded rounds |
|---|---:|---:|---:|---:|---|
| keygen | 24.92 µs | 98.88 µs | 109.03 µs | 1 | 1, 2 |
| encaps | 59.55 µs | 166.05 µs | 209.30 µs | 1 | 1, 2 |
| decaps | no clean round | no clean round | no clean round | 0 | 1, 2, 3 |

Files: `raw/rN.<op>.<lib>.log` are criterion's output per run (`-control` is the repeat of rind), `rounds.txt` the load and clock at each round, `scripts/` the runner and parser. Hostnames, user names, addresses and paths are not in the logs (checked with grep).

(Written by Claude Sonnet 5.5)
