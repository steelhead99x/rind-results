<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840: relinks that really move the libraries, rind 98568332

legacy rind-bench 2f0c5e8 (later squashed into rind-hardware `8d9c1bb`) puts the layout padding in section `.Reset`, which cortex-m-rt's `link.x` places first in `.text`, so the padding sits ahead of every library. Before that, `--pad N` put a `.text.rind_bench_pad` static after the Rust code: the relink in `../nrf52840-relink-98568332/` moved rind, libcrux, ring's Rust and RustCrypto by only 8 bytes, and so it did not test layout.

Images: `cargo xtask build-images nrf chacha20poly1305 hkdf hmac sha2 --pad N --out images-fixpadN` for N = 0, 1031, 2053, at legacy rind-bench 2f0c5e8 (later squashed into rind-hardware `8d9c1bb`) (which includes f4c7802's watchdog feed, so pad0 is rebuilt here rather than reusing the published images). Hashes are in [images.sha256](images.sha256).

[shifts.md](shifts.md) gives each library's shift from pad0, the most common per-function shift from `arm-none-eabi-nm -n -C` (in parentheses, how many functions share it when not all do). Every library moved +1040 to +1048 B at pad1031 and +2064 to +2072 B at pad2053.

Runs: one T1000-E window, 2026-11-12 15:13:03 to 15:45:09 PST (bridge down to bridge connected), three runs of each bench per layout, all complete. Meshtastic 2.7.26.54e0d8d and the identical config were restored afterwards (empty export diff; MBR and SoftDevice sha256 [redacted] and INFO_UF2.TXT unchanged).

- [relink-short.md](relink-short.md): rind over the fastest other library per layout, short inputs. No ratio moves by more than 0.01; rind's own cycles spread 0.0 to 0.2% over layouts.
- [relink.md](relink.md): the same for every size.
- [compare-pad1031.md](compare-pad1031.md), [compare-pad2053.md](compare-pad2053.md): every row, pad0 against each relink. 0 of 264 rows moved by more than max(1%, run spread) in either.
- [compare-published-pad0.md](compare-published-pad0.md): the published runs (`results/raw-nrf52840/`, pre-f4c7802 firmware) against this pad0. 8 of 264 rows move by more than max(1%, run spread), all slower in the new firmware and by at most 6% (the largest is `sha256-updates/rind/1B`, 1.062×); they are firmware-to-firmware differences, not layout.
- `pad0/`, `pad1031/`, `pad2053/`: the raw runs.

So on this board the short-input ratios do not depend on layout, now tested with the libraries really moved.
