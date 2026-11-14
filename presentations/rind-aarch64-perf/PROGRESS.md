# Deck progress log

Each refresh reworks the whole deck into one story; this log records what changed in each refresh. Times are PST.

## 2026-11-14

- Merged rind-results main `45c523e`. No number in the DATA block changed.
- Host paths match that main: Pi 2 is `results/pi2b-bench` (PR #24), the Uno R3 set is `results/uno-r3` (PR #25), and the comparable x86 host is `results/intel-core-ultra-9-275hx` (PR #29). The Nano 33 BLE still has no published numbers. The Nov 13 Pi 5 full suites and the PR #82 A/B are still not on main.
- rind-results #18–#23 are on main. Appendix citations of `crit-parse.py` and `argon2-pmu-sum.py` now name `cargo xtask crit-parse` and `cargo xtask argon2-pmu-sum`.

## 2026-11-13 16:05

- New Phase 6 slide (17) from new `DATA.pi5full` and `DATA.pi5fullMeta` blocks: the full report suite on stock
  upstream main (a6577cb4) on the fanless Pi 5 at 1.5 GHz, 11:47–15:27 PST. 396 benchmark IDs, 68 operations,
  7 libraries, median of 5 valid rounds; 174 test binaries passed first with 0 failures. rind is fastest on 45 of 68
  (aws-lc-rs 19, RustCrypto 3, ring 1); all 19 operations where rind trails by more than 5% are public-key operations or
  the TLS handshakes that use them. Gaps to aws-lc-rs: X25519 2.32×, RSA-2048 sign 1.69×, P-256 ECDH 1.54×, Ed25519 sign
  1.46×, P-256 verify 1.19×. rind's times match the wide rows of the PR #82 A/B within 0.3% (X25519 252.76 vs 252.8 µs).
  Round-to-round spread median 0.2%, max 8.9%; up to 75.2 °C, no live throttle bits, 1.5 GHz in every timed sample.
  A power loss stopped round 3; it was rerun in full and the partial round is excluded.
- New `DATA.pi5at24` block, marked provisional: the second Pi 5 (16 GB) at its stock 2.4 GHz, same commit and
  binaries, 3 of 5 rounds finished at 16:00. Times are 1.60× shorter (median 1.601 over all 396 IDs, range 1.54–1.77),
  so every rind-vs-library ratio is unchanged. Quoted on the new slide, the clock slide and the next-steps slide.
  Replace it with the 5-round medians when the set finishes.
- Timeline: three new entries (the full suite, the 2.4 GHz rounds, rind-results #24 with the Pi 2 armv7 set merged at
  15:14). The 07:23 entry no longer calls its Pi 5 "a third Pi 5": the board with the 1239-test matrix is the same
  fanless 8 GB Pi 5 as the A/Bs (same boot-time 1.5 GHz cap, and it shared the bench lock with the PR #82 A/B that day).
- Summary, next steps and "where the numbers live" updated to match. Every earlier number is unchanged.

## 2026-11-13 13:40

- New: two timeline slides (19 and 20) rendered from a new `DATA.timeline` block: 25 dated entries from 2026-11-11 to
  13:40 PST on 11-13, each with what changed, host or board, measured effect and state. Unfinished items are yellow.
- New Phase 6 slide (16) from a new `DATA.pr82` block: draft rind PR #82 re-proposes the narrow dispatch on current
  fork main. Pi 5 A/B, 5 interleaved rounds at 1.5 GHz: X25519 252.8 → 107.7 µs, RSA-2048 sign 5209.2 → 3071.6 µs
  (PR #33 107.6 / 3069.8 in the same run); P-256 ECDH / sign / verify / keygen 0.613 / 0.745 / 0.902 / 0.703 of wide.
  Pi tests pass at 92cbbf21; still draft while CI regenerate on b6f71072 is pending.
- Story refactor: title, summary lede and first KPI, the phase map (now six phases), status, next steps and
  "where the numbers live" now end on PR #82 and the board benches. 25 slides (was 22).
- Boards in the timeline: RP2040 (first bench, SysTick fix, ARMv6-M stack, SHA-256 short-message fix), XIAO MG24,
  ESP32-S3 on a second host, Pi 2, a second and a third Pi 5; Uno R3 and Nano 33 BLE pending. rind-results #2, #3
  and #17 merged on 11-13; rind-agent #49 closed unmerged.
- Public text scrubbed of the bench-lock path, account names in the title and appendix, and service names.
- Every number in the Nov 11–12 DATA blocks is unchanged.

## 2026-11-13 08:30

- No new measurements. Status changes only; every number in the DATA block is unchanged.
- All results are on rind-results main: PR #4 (Pi 5 follow-up A/B, incl. `series2-layout/` and `table-align/`) merged at
  07:49 and PR #5 (PR #33 A/B, Argon2 umlal, interleave and scalar riders) at 07:52. The "where the numbers live" slide
  now says so, and branch-only footers and appendix rows point at main. The merge corrected one M1 spread in
  `results/argon2-interleave/RESULTS.md` (64 MiB NEON before: 11.7% → 10.7%); the deck does not quote it.
- PR #33 (603b7120) was closed unmerged on 2026-11-12 at 18:52. Its close is recorded as superseded by upstream #672,
  which only removes upstream's aarch64 X25519 quest note and carries none of the narrow schedules. The summary, status
  and next-steps slides now end on that open decision: reopen #33 or re-propose the narrow dispatch on current main.
- `fix/p256-table-align` (now c1b2e883, measured at 8e6f1670) and `perf/pi5-rsa-bench-sizes` (now bba85832, was 41f45409)
  were moved onto fork main as single commits and opened as rind PRs #79 and #80.
- Jetson Orin Nano: full-suite results are on main (`results/jetson-orin-nano/`, 3 pinned runs at another rind commit).
  They are listed on the status slide only, since they are not an A/B of this work.

## 2026-11-12 20:15

- Phase 2 now has eight branches. `perf/pi5-cpu-detect` (3d7dd094) moves from "held back" to "neutral, kept":
  its 2.5–3.1% P-256 loss came from the generator table landing at +8 mod 64, and the 64-byte alignment fix brings
  it to 1.003× / 1.002× / 1.001× (keygen / sign / ECDH) of PR #33 (`hosts/pi5/ab-narrow-mul/series2-layout/`).
- New row: `fix/p256-table-align` (8e6f1670), the same fix alone on narrow-nist with Lean proof updates. Pi 5 A/B
  vs narrow-nist: all four P-256 ops 1.000×, since that build's table already sat at +32 (`table-align/`). Tests pass
  on both machines, the Lean build passes on the Pi, and ct-qemu, qemu tests, ct-vartime and ctgrind pass in both modes.
- cpu-detect leaves the "Dropped" slide; the Status slide shows it as pushed; the Next slide drops the pending
  alignment item and adds the table fix to the PR #33 cherry-pick list.
- Branch heads refreshed after the rind-results history rewrite (results branches 14c89e4, 94b8b1a, 3a481c4;
  main edcd461). Orin: bench scripts are on main, results still pending.

## Earlier

- 2026-11-12 14:05: first version, phase by phase.
- 2026-11-12 16:05: the Pi 5 1.5 GHz clock slide; the w7 and scalar-rider results.
- 2026-11-12 18:44: a slide on which sources are on rind-results main.
