# Deck progress log

Each refresh reworks the whole deck into one story; this log records what changed in each refresh. Times are PST.

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
