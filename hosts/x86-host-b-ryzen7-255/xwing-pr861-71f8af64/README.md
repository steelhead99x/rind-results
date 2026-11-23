# X-Wing bench rows for kixel-ai/rind#861 at 71f8af64 on x86-host-b-ryzen7-255

Results only, no code changes. The upstream report (`admin/bench-report`) does not list this host, so these are indicative numbers for the claim's empty X-Wing rows, not report rows.

AMD Ryzen 7 255 (Zen 4, 16 logical CPUs), Linux, core 5, governor performance. Built as the full `rind-bench --bench xwing` binary. 10 rounds were run; the table uses the first 5, and `raw/` holds all 10. A first run of 5 rounds while other jobs were building on the host is in `raw-first-run-disturbed/`: it was discarded whole (load average 8 to 9, rind controls disagreed by up to 40 percent).

Method: `rind-bench`'s `xwing` bench (rind against libcrux-kem 0.0.10 and RustCrypto `x-wing` 0.1.1), interleaved A/B with the order rotating each round and rind repeated last as the control, pinned with `taskset`, criterion `--warm-up-time 1 --measurement-time 3`. A round is kept for an operation when its two rind times are within 1.02; the cell is the median of the kept rounds' central estimates.

| Operation | rind | libcrux-kem | RustCrypto | Kept | Discarded rounds |
|---|---:|---:|---:|---:|---|
| keygen | 9.08 µs | 48.72 µs | 64.82 µs | 5 | none |
| encaps | 17.88 µs | 78.84 µs | 111.11 µs | 5 | none |
| decaps | 14.69 µs | 95.27 µs | 178.03 µs | 5 | none |

Files: `raw/rN.<op>.<lib>.log` are criterion's output per run (`-control` is the repeat of rind), `rounds.txt` the load and clock at each round, `scripts/` the runner and parser. Hostnames, user names, addresses and paths are not in the logs (checked with grep).

(Written by Claude Sonnet 5.5)
