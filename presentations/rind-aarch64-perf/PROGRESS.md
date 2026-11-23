# Deck progress log

Each refresh reworks the whole deck into one story; this log records what changed in each refresh. Times are PST.

## 2026-11-17 20:30

- New slide 20: ML-DSA signing on the Jetson Orin Nano, from steelhead99x/rind#126 comment 6936142476 (base
  19f94a07, head bbae5dc0, 3 interleaved runs, `taskset -c 3`): ML-DSA-44 149.40 → 142.79 µs (−4.4%), -65
  227.97 → 210.36 µs (−7.7%), -87 247.27 → 229.81 µs (−7.1%). #126 was closed for kixel-ai/rind#863, which makes
  the same change and is open. The A/B is in a PR comment, not on rind-results main.
- Slide 21 ("where the numbers live") rewritten: every Pi 5 and M1 Pro source the deck uses is now on rind-results
  main (checked at 88cbf85), and the Orin has a published set (`results/jetson-orin-nano/`, rind fastest on 65 of 68).
- Slide 19: the "open PRs" item notes that the table alignment is upstream as kixel-ai/rind#790, ready for review.
- Title subtitle mentions the Orin update. `meta.asOf` stays 2026-11-12 20:15, since the phase data did not change.

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
