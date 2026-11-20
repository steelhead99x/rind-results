# X-Wing bench rows for kixel-ai/rind#861 at 71f8af64 on pi5

Results only, no code changes. The upstream report (`admin/bench-report`) does not list this host, so these are indicative numbers for the claim's empty X-Wing rows, not report rows.

Raspberry Pi 5 (Cortex-A76, 4 cores), Debian 13, core 3, governor performance, held at 1.5 GHz by the host. A virtual machine of another job ran on core 1 throughout. `vcgencmd get_throttled` read 0xe0000 before and after (past events, none current). 8 rounds run, first 5 used.

Method: `rind-bench`'s `xwing` bench (rind against libcrux-kem 0.0.10 and RustCrypto `x-wing` 0.1.1), interleaved A/B with the order rotating each round and rind repeated last as the control, pinned with `taskset`, criterion `--warm-up-time 1 --measurement-time 3`. A round is kept for an operation when its two rind times are within 1.02; the cell is the median of the kept rounds' central estimates.

| Operation | rind | libcrux-kem | RustCrypto | Kept | Discarded rounds |
|---|---:|---:|---:|---:|---|
| keygen | 95.37 µs | 374.92 µs | 471.40 µs | 5 | none |
| encaps | 299.46 µs | 659.27 µs | 818.43 µs | 5 | none |
| decaps | 232.21 µs | 751.45 µs | 1.056 ms | 5 | none |

Files: `raw/rN.<op>.<lib>.log` are criterion's output per run (`-control` is the repeat of rind), `rounds.txt` the load and clock at each round, `scripts/` the runner and parser. Hostnames, user names, addresses and paths are not in the logs (checked with grep).

(Written by Claude Sonnet 5.5)
