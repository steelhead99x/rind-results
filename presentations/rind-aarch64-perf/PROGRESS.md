# Deck progress log

Each refresh reworks the whole deck into one story; this log records what changed in each refresh. Times are PST.

## 2026-11-16 23:51

- New Phase 6: ChaCha20 NEON on an Apple M1 Pro. Kept r2 (`core_x8`, `1e015593`) and r5
  (`core_x9` plus a ≤128 B fast path, `f011025b`) on `quest/m2/aarch64-chacha20-neon`.
  Headline, from the median of three session medians: 1.89× at 16 KB (13.07 → 6.93 µs),
  1.81× at 1200 B, 1.69× at 1350 B, unchanged at 32 B. The run is an unpinned 3×5
  interleaved A/B, not a published set. Neoverse is not measured.
- Dropped rounds R1, R3, R4 and R6 are on their own slide, with the reason each lost.
- Timeline is now six phases on three tracks. The status, next-steps, sources and
  appendix slides cite kixel-ai/rind#823 (draft) and the DATA block in this file.
- `asOf` is 2026-11-16 23:51 PST.

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
