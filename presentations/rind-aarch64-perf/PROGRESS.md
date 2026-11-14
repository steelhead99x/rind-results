# Deck progress log

Each refresh reworks the whole deck into one story; this log records what changed in each refresh. Times are PST.

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

## 2026-11-14

- New slides for steelhead99x/rind#82 (Pi 5 narrow-schedule table from the PR body; A72, N1, and Jetson Cortex-A78AE not measured; A78AE keeps `_alt`) and #79 (64-byte P-256 table, 88,064 bytes; the 2.5–3% A76 effect is the earlier measurement; head `54d339e7` was not re-timed).
- The 12 November status slide still records that evening's check. A following slide says the Orin set is now on main at `results/jetson-orin-nano` (rind `98568332`) and is not a remeasure of #79 or #82. Deck: `presentations/rind-jetson/`.

## Earlier

- 2026-11-12 14:05: first version, phase by phase.
- 2026-11-12 16:05: the Pi 5 1.5 GHz clock slide; the w7 and scalar-rider results.
- 2026-11-12 18:44: a slide on which sources are on rind-results main.
