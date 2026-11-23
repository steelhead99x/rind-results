# X-Wing bench rows for kixel-ai/rind#861 at 71f8af64 on pi-host-a

Results only, no code changes. The upstream report (`admin/bench-report`) does not list this host, so these are indicative numbers for the claim's empty X-Wing rows, not report rows.

Raspberry Pi 2 B (BCM2836, Cortex-A7, 4 cores), 32-bit armv7, core 3, governor set to performance for the run (900 MHz) and restored to ondemand (`post.txt`), under the shared `bench.lock`. Binary cross-built with the repo flake `armv7` shell, interpreter patched to `/lib/ld-linux-armhf.so.3`. 8 rounds run, first 5 used.

Method: `rind-bench`'s `xwing` bench (rind against libcrux-kem 0.0.10 and RustCrypto `x-wing` 0.1.1), interleaved A/B with the order rotating each round and rind repeated last as the control, pinned with `taskset`, criterion `--warm-up-time 1 --measurement-time 3`. A round is kept for an operation when its two rind times are within 1.02; the cell is the median of the kept rounds' central estimates.

| Operation | rind | libcrux-kem | RustCrypto | Kept | Discarded rounds |
|---|---:|---:|---:|---:|---|
| keygen | 1.258 ms | 3.168 ms | 2.786 ms | 3 | 1, 2 |
| encaps | 2.369 ms | 5.357 ms | 4.878 ms | 4 | 3 |
| decaps | 1.560 ms | 6.906 ms | 7.319 ms | 5 | none |

Files: `raw/rN.<op>.<lib>.log` are criterion's output per run (`-control` is the repeat of rind), `rounds.txt` the load and clock at each round, `scripts/` the runner and parser. Hostnames, user names, addresses and paths are not in the logs (checked with grep).

(Written by Claude Sonnet 5.5)
