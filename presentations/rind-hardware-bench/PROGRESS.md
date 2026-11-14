# Deck progress log

Times are PST.

## 2026-11-14

- Merged rind-results main `45c523e`. No number in the DATA block changed.
- Host paths match that main: Pi 2 is `results/pi2b-bench`, Uno R3 is `results/uno-r3`, and the comparable x86 host outside this deck is `results/intel-core-ultra-9-275hx`. The RP2040 deck is on main; there is still no `results/rp2040/` timing tree.
- Public text no longer names a GitHub account. The title slide says "rind fork · rind-results".

## 2026-11-13 14:10

- First version, 11 slides: devices, verification matrix, cross-device SHA-256 / SHA-512 / X25519 / P-256 sign,
  microcontrollers, Uno R3, application processors, Pi 5 narrow multiplier (PR #82), pending, sources.
- One DATA block, same look as `presentations/rind-aarch64-perf/`.
- Pending at this point: Pi 5 lane 2 timed rounds, Pi 5 lane 3 rounds 3–5, Nano 33 BLE (never enumerated),
  iPad (parked), Uno R3 ChaCha20 timings, Pico W re-verify, and merging the Pi 2, Uno R3 and RP2040 results to main.
