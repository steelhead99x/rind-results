<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-C6 upstream refresh A/B at kixel-ai/rind `8a5977ad`, 2026-11-23

Recorded 2026-11-23, 20:52 to 22:31 UTC, on one ESP32-C6. This is a new A/B directory. No published tree or number is touched.

## What this is

An interleaved A/B of five steelhead99x/rind draft branches against upstream main, on the same board, flashed in the order `ctl zero ctl aes ctl p256 ctl cp ctl all ctl` (11 flashes).

- `commit` (upstream, the control): [kixel-ai/rind `8a5977adb9c32ea2ed010c400f5ffbdcdec14b47`](https://github.com/kixel-ai/rind/commit/8a5977adb9c32ea2ed010c400f5ffbdcdec14b47). Upstream main has since moved one commit, to `f4ab024e` (quest v0.24.0 and `flake.lock` only, no rind code).
- Candidates. The commit is the one each image was built from:

| Image | rind commit | Fork PR | What it adds to the control |
|---|---|---|---|
| `ctl` | `8a5977adb9c32ea2ed010c400f5ffbdcdec14b47` | none | upstream main as it is |
| `zero` | `617cf6a133020d67c8cae51daad3bfdd639e5c55` | [#307](https://github.com/steelhead99x/rind/pull/307) | the key wipe as one memset on riscv32 |
| `aes` | `a612f8051a62e5be22c65e784b652f6aa5b05528` | [#308](https://github.com/steelhead99x/rind/pull/308) | fixsliced AES and a mulhu-free GHASH on 32-bit portable |
| `p256` | `8e2128d4ba186669837f94ef4c3bf8a6dec68457` | [#309](https://github.com/steelhead99x/rind/pull/309) | a 4-bit P-256 generator table on 32-bit portable |
| `cp` | `dacc7cc01df515584ead22ab43935965db914927` | [#310](https://github.com/steelhead99x/rind/pull/310) | 26-bit Poly1305 and word-XOR ChaCha20 on riscv32 |
| `all` | `3cffff4d5018882b082688cd4ce5de5d5f72bc1c` | [#311](https://github.com/steelhead99x/rind/pull/311) | `zero`, `aes`, `p256` and `cp` together, on #297 |

On 2026-11-23 each of #307 to #311 still had the commit above as its head.

## Run counts and the three-run rule

- The control `ctl` has **six runs** (flashes 1, 3, 5, 7, 9, 11). `esp32c6.json`, RUNS.md and RESULTS.md are those six runs.
- Each candidate was flashed **once**, so each has **one run**. It does not meet the three-run rule. [ab.md](ab.md) shows `n` = 1 and a spread of 0 for each candidate. That 0 comes from having one run. It does not show that the candidate repeats. This directory is a diagnostic A/B. It is not a comparable-set publish of any candidate.
- Within a flash the firmware repeats every row once per round, 22 to 29 complete rounds per flash. Those rounds are not counted as runs.
- The six control runs agree closely. Every control row has a run spread of 0.00% except `p256_verify`, at 0.32% (RUNS.md). Each candidate flash sits between two control flashes.
- To meet the rule, each of `zero`, `aes`, `p256`, `cp` and `all` needs **two more flashes** on this board, interleaved with `ctl` as before (for example `ctl zero ctl aes ctl p256 ctl cp ctl all ctl`, twice). That is 10 more candidate flashes and up to 12 control flashes. These were run the same evening as two more full passes, [pass 2](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-pass2-2026-11-23/NOTES.md) and [pass 3](../8a5977adb9c32ea2ed010c400f5ffbdcdec14b47-refresh-ab-pass3-2026-11-23/NOTES.md). Pass 3's `ab-all-passes.md` pools all three passes, and there every candidate has three runs.

## Board and harness

- ESP32-C6 rev v0.2 (`Chip type: esp32c6 (revision v0.2)` in every flash), 40 MHz crystal, 4 MB flash, on USB-Serial/JTAG. It is the board whose MAC ends `2C:48`. The full MAC and the host's serial device names are not published.
- CPU at 160 MHz (`CpuClock::max` in the harness). Times are the firmware's wall-clock medians in ns (`median_ns`). No cycle counter is printed.
- Firmware: a private WS-BENCH harness, built against each rind worktree by path. It is not published here. It times rind and RustCrypto (ed25519-dalek for Ed25519) ChaCha20-Poly1305 seal and open (16, 51, 243, 255 and 1500 B) and Ed25519 sign and verify. It also times rind-only AES-128-GCM seal and open (16, 243 and 1500 B) and P-256 sign and verify. A row is the median of 32 calls, or 8 calls for Ed25519 and P-256. Open decrypts a scratch copy of the sealed message. The RustCrypto rows are the same code in every image, so they act as a layout control.
- Known-answer checks, every round: RFC 8439 ChaCha20-Poly1305 and RFC 8032 Ed25519 for both libraries, and the McGrew-Viega GCM test case 3 and RFC 6979 P-256 checks for rind. There were 1,634 checks over the 11 boots and 0 failures. No `FAILED` line appears in any log.
- Built with nixpkgs rustc 1.98.1 for `riscv32imac-unknown-none-elf`, opt-level 3, fat LTO, one codegen unit. The images are merged (`espflash save-image --merge`, then re-merged and checked by the harness repo's `merge-image` and `verify-merged`). They flash at 0x0. `images.sha256` lists the sha256 of every `.bin` and `.elf`. The images are not committed.

## How it was run

Each flash was `espflash write-bin --chip esp32c6 --after watchdog-reset 0x0 c6-<image>.bin` (espflash 4.7.0), followed by a 120 s raw capture at 115200 baud. [run.log](run.log) has each flash's espflash output and the time each capture was written. The serial device paths and the MAC are removed from it.

- For every flash, espflash printed `The esp32c6 does not support watchdog reset` after it connected. It did not print a `Binary successfully written` line. The capture then had no rows, so the script ran `espflash reset` (the `reset-N` lines) and captured again. That capture is the published log. The logs carry no image hash. Two things show which image was running: every `ctl` flash prints the same rows, and each candidate's rows differ from the control flashes on either side of it.
- The run paused for 52 minutes between flash 4 (`aes`, captured 21:09 UTC) and flash 5 (`ctl`, flashed 22:01 UTC). The host's USB serial devices were re-created during the pause. The control rows after the pause match the control rows before it (RUNS.md).

## How the numbers are made

[tools/ws_bench_import.py](tools/ws_bench_import.py) with [tools/import.json](tools/import.json) writes `raw/run-*.json`, `esp32c6.json`, RUNS.md, RESULTS.md, `ab.json`, `ab.md` and `meta.json`. It reads only the logs in `raw/`. To reproduce, run `python3 -I tools/ws_bench_import.py tools/import.json` from this directory.

- A log is one flash. The importer drops the rows before the first `# round complete`, because the capture opens part way into a round. A row's value for the flash is the median over the remaining rounds of the printed `median_ns`. Each round's values are kept in `rounds_ns` (run JSON) and `rounds` (ab.json).
- A run is one control flash. The control's published time is the median of its six runs. Its spread is (slowest − fastest run) / median.
- `cargo verify --dir` recomputes `esp32c6.json`, RESULTS.md and RUNS.md from `raw/run-*.json`, and every `ab.md` cell from `ab.json`.

## Raw logs

`raw/b-<flash>-<image>.log` are the captures as written, byte for byte. Each one holds `WS-BENCH`, `WS-KAT` and `# round complete` lines, plus one partial ESP-IDF bootloader line at the start (an `esp_image` segment or `Loaded app` line). Nothing was removed. No MAC, network name or key is in them.

## What to read off it

Use [ab.md](ab.md). Each candidate is one flash, so these are one-run readings:

- `cp` and `all` are the ChaCha20-Poly1305 changes. `all` seals 1500 B in 0.537 of the control's time and opens it in 0.544. RustCrypto stays faster than the control at every length. `all` is faster than RustCrypto at 243, 255 and 1500 B, and still slower at 16 and 51 B.
- `aes` is AES-128-GCM at 0.58 to 0.71 of the control. On its own, `aes` makes ChaCha20-Poly1305 open slower: 1.10 at 16 B, 1.08 at 51 B. `zero` and `all` do not show that.
- `p256` signs in 0.857 of the control's time, and `all` in 0.864. `p256_verify` is 1.000 to 1.011 in every image.
- The Ed25519 rows move by at most 1.1% in any image.
