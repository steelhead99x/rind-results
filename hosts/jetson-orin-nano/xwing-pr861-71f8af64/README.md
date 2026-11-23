# X-Wing bench rows for kixel-ai/rind#861 at 71f8af64 on jetson-orin-nano

Results only, no code changes. The upstream report (`admin/bench-report`) does not list this host, so these are indicative numbers for the claim's empty X-Wing rows, not report rows.

Jetson Orin Nano (Cortex-A78AE, 6 cores), Linux 6.8 tegra, 15 W mode, core 3, governor schedutil (sudo needs a password, so it stayed), 1.1 GHz in rounds 2 to 8. The binary is the Pi 5 build, copied, since the host has no network. 8 rounds run, first 5 used.

Method: `rind-bench`'s `xwing` bench (rind against libcrux-kem 0.0.10 and RustCrypto `x-wing` 0.1.1), interleaved A/B with the order rotating each round and rind repeated last as the control, pinned with `taskset`, criterion `--warm-up-time 1 --measurement-time 3`. A round is kept for an operation when its two rind times are within 1.02; the cell is the median of the kept rounds' central estimates.

| Operation | rind | libcrux-kem | RustCrypto | Kept | Discarded rounds |
|---|---:|---:|---:|---:|---|
| keygen | 52.58 µs | 174.10 µs | 213.29 µs | 5 | none |
| encaps | 112.67 µs | 259.78 µs | 328.83 µs | 5 | none |
| decaps | 80.93 µs | 338.39 µs | 468.52 µs | 5 | none |

Files: `raw/rN.<op>.<lib>.log` are criterion's output per run (`-control` is the repeat of rind), `rounds.txt` the load and clock at each round, `scripts/` the runner and parser. Hostnames, user names, addresses and paths are not in the logs (checked with grep).

(Written by Claude Sonnet 5.5)
