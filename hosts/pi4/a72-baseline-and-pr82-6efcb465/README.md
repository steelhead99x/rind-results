# Raspberry Pi 4 B (Cortex-A72): baseline, #82 narrow against wide, and #163 Argon2

Results only, no code changes. Raw criterion logs from one Pi 4 Model B Rev 1.5 (4 x Cortex-A72, MIDR part 0xd08, no AES, PMULL, SHA, or SHA-3 extension, Debian 13, kernel 6.18, aarch64, governor ondemand at 1.8 GHz). Write-up and tables: steelhead99x/rind `benchmarks/pi4.md` (the pull request this set accompanies).

- `baseline/r1.txt` to `r5.txt`: ring, aws-lc-rs, and rind in the same binary, upstream `main` at `6efcb465`, core 3, 5 rounds, criterion 1 s warm-up and 2 s measurement. `meta.txt` has the temperature, `vcgencmd get_throttled`, governor, frequency, and load at each round start and end. Round 1 started with the first SHA-2 rows at about half speed and round 5 doubled several late rows (see the write-up); the tables use the median of 5. `0x80000` at the end of round 5 is the sticky "soft temperature limit has occurred" bit, not a current throttle.
- `ab/n82-r<n>-{default,narrow,wide}.txt`: #82 head `51fd07cf` built with `--cfg rind_cpu_disable`, default against `RIND_CPU_DISABLE_wide_mul=1` against `RIND_CPU_FORCE_wide_mul=1`, order rotated each round, X25519, Ed25519, and P-256 against aws-lc-rs.
- `ab/argon-r<n>-{main,163}.txt`: Argon2id t=3 p=1 at 8 MiB and 64 MiB, `main` against #163 head `fc4d7df0`, median of 9 per line, RustCrypto `argon2` as the control.
- `ab/sweep.txt`: AES-GCM and ChaCha20-Poly1305 sweep from 16 B to 16 KB against aws-lc-rs and ring.
- `scripts/run2.sh`: the board-side script that produced all of it.

An earlier partial set at 1.8 GHz ondemand (rounds 1 to 3, taken before the board lost power) is superseded by this one and not published.

Scrubbed: home paths, user names, host names, addresses. The logs held none of them; the script had its paths rewritten to `~`.
