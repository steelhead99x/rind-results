<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# ESP32-S3: SHA-512 in IRAM (steelhead99x/rind#94) against main, 2026-11-17

Raw serial logs of an A/B on one ESP32-S3 board: the `sha2`, `hmac` and `hkdf` bench images built from steelhead99x/rind main `4fbb71883` and from the PR #94 commit `942bb48a0`, three fresh boots of each image, alternating main and PR. This is the raw data behind the board-run numbers in [steelhead99x/rind#94](https://github.com/steelhead99x/rind/pull/94). Nothing here edits a published number; it is a new diagnostic directory.

## What was measured

| | |
|---|---|
| Board | ESP32-S3 rev v0.2, 8 MB flash, USB-Serial/JTAG, 240 MHz CPU (`cpu_hz` 240000000 in every start record), DIO flash at 80 MHz (`Boot SPI Speed : 80MHz` in every log), 16 KiB instruction cache (the ESP-IDF default) |
| Run date | 2026-11-17, 20:05 to 20:35 PST, one board, one host (x86-host-c) |
| Firmware | rind-hardware `esp32` bench harness, ESP-IDF v5.5.1, esp rustc 1.97.0-nightly (1.97.0.0), `RIND_BENCH_PAD=0`, release profile, flashed with espflash 4.7.0 `--flash-freq 80mhz` |
| main | steelhead99x/rind `4fbb718839efe8c784ce27d74567a95b4419fb95` |
| PR | steelhead99x/rind PR #94 at `942bb48a04309023f35641b9188a8e8890b5c1a2` |
| Runs | 3 fresh boots per image (the board is reset by RTS before each capture), 9 per side, 18 in all, flash order main, PR, main, PR, ... within each round |
| Known-answer checks | `checks-done` with `failed: 0` in all 18 boots |

Each `run-N.log` is the whole serial output of one boot, one line per serial line with the seconds since the capture opened (lines that arrived together share a time). `run-N.jsonl` is the same boot's `RB:` records, one JSON object per line. `ns` is the median of the boot's samples. `ratios.tsv` has, for every row present in all six boots of both sides, the median of the three boots' `ns` and the PR over main ratio (`summary.json` has the same data). [tools/analyze.py](tools/analyze.py) makes both from the logs.

Layout:

```text
main-4fbb7188/{sha2,hmac,hkdf}/run-{1,2,3}.{log,jsonl}
pr94-942bb48a/{sha2,hmac,hkdf}/run-{1,2,3}.{log,jsonl}
images.sha256    sha256 of the six ELFs that were flashed
build-status.txt how the images were built
ratios.tsv  summary.json
tools/           the scripts used to build, flash, capture and summarise
```

## How the images differ from the rind-hardware runbook

rind-hardware's `esp32/pr69` runbook (draft rind-hardware#7) was written for older rind commits, and its patch series no longer applies as is. What was built, in order, on each commit:

1. `patches/0001` (MD5 on Xtensa): applies cleanly.
2. `patches/0002` (leave aws-lc-rs rows out on Xtensa): applies after dropping its `mlkem768.rs` hunk, because that bench file is `mlkem.rs` on both commits and neither image uses it ([tools/0002-without-mlkem768-hunk.patch](tools/0002-without-mlkem768-hunk.patch)).
3. `patches/0003` to `0005` (bare-metal `no_std`, sha3, `reuse_cache` without CAS) are skipped. They target bare-metal builds, and `0003` and `0005` no longer apply to either commit. The std ESP-IDF images build without them.
4. A bench-only commit removes the `verified-garbage` rows from `sha2.rs`, `hmac.rs` and `hkdf.rs` ([tools/dropvg.py](tools/dropvg.py), and the resulting commits as `tools/bench-only-dropvg-*.patch`). That crate has no Xtensa support (no 64-bit atomics, and its `hashes` module exists only on x86 and AArch64), so those rows cannot run on this board. The same edit is applied to both commits, and it touches no rind call and no other library's row. It also removes those rows' known-answer asserts against `verified-garbage`; the harness's own `check` records, which compare rind with ring, RustCrypto, libcrux and the ESP32-S3 hardware SHA block, are unchanged and all pass.

So these images share a patch set, but it is not the set rind-hardware's runbook names. Whoever measured the numbers in the PR body did not leave a record of theirs; this directory is a rerun, not that run's logs.

## Result: SHA-512 family

PR over main, median of the three boots' `ns` per side, rind rows. The spread is the largest max-minus-min over the six boots of that image, as a share of the median: at most 0.005% on any rind row.

| Row | main (ns) | PR (ns) | PR / main |
|---|---:|---:|---:|
| `sha512/rind/32B` | 111,405 | 102,266 | 0.918 |
| `sha512/rind/1200B` | 1,051,296 | 957,802 | 0.911 |
| `sha512/rind/16KB` | 13,477,592 | 12,281,133 | 0.911 |
| `sha512-256/rind/16KB` | 13,477,733 | 12,283,075 | 0.911 |
| `hmac-sha384/rind/32B` | 447,947 | 411,518 | 0.919 |
| `hmac-sha384/rind/16KB` | 13,813,662 | 12,590,608 | 0.911 |
| `hkdf-sha384/extract-expand/rind` | 897,033 | 823,721 | 0.918 |
| `hkdf-sha512/extract-expand/rind` | 896,496 | 818,850 | 0.913 |

Ranges over every row of the family: SHA-512 and SHA-512/256 at 32 B to 16 KB (12 rows) 0.911 to 0.918; HMAC-SHA384 (12 rows, including reused-key) 0.911 to 0.919; HKDF-SHA384 and SHA512 `extract-expand` and `expand` (4 rows) 0.913 to 0.918, and with the two `expand-aes-256-gcm-key` rows 0.913 to 0.938.

## Result: rows the PR does not target

- rind SHA-256 and SHA-256 update rows (8 rows): 0.9998 to 1.0000.
- HMAC-SHA256 with a reused key (6 rows): within 0.003% of main. With a new key every row is 525 ns slower (425 ns at 16 KB), 1.006 at 32 B and 1.001 at 1200 B.
- HKDF-SHA256 `extract-expand`: 1.006 (179,811 against 178,759 ns). Its `expand` rows are unchanged.
- HKDF-SHA1 `extract-expand`: 1.026 (184,481 against 179,819 ns). HMAC-SHA1 rows are unchanged. The PR does not change SHA-1, so this reads as layout movement.
- Other libraries' rows moved too: libcrux (HACL*) SHA-256 1.002 to 1.010, RustCrypto SHA-256 1.017 to 1.936 (the 1.936 is 32 B: 41,674 against 80,673 ns).
- Free heap at boot (`free_heap` in the start record): the sha2 image 153,072 B on main and 134,000 B on the PR (19,072 B less), the hmac and hkdf images 153,632 B and 134,816 B (18,816 B less).

## Against the numbers in the PR body

The body's SHA-512 and SHA-512/256 range (0.911 to 0.918), the 16 KB times (12.28 ms against 13.48 ms), HMAC-SHA384 (0.911 to 0.919), the unchanged SHA-256 rows, the HMAC-SHA256 525 ns and 1.006, the HKDF-SHA256 1.006, the 1% HACL* and 1.9x RustCrypto moves, the 0.006% repeat spread and the heap losses all reproduce here.

Two figures differ:

- HKDF-SHA1 `extract-expand`: the body has 1.102 (198 against 180 µs). This run has 1.026 (184 against 180 µs). The main side matches.
- HKDF-SHA384 and SHA512 range: the body has 0.913 to 0.935. This run has 0.913 to 0.938 over the six rows above (0.913 to 0.918 for the four `extract-expand` and `expand` rows).

## Not covered

- Only `942bb48a` was measured, not the PR's current head `48c611199`. Between the two, `rind/src/low/generic/sha512.rs` changed only inside its `tests` module and `rind/build.rs` did not change, so the compressor source is the same; the head's images were not built or run.
- One board, one flash size and clock. The 16 KiB instruction cache makes short-input rows layout-sensitive (see [../layout-relink-2026-11-12/](../layout-relink-2026-11-12/README.md)), so the unchanged and slower rows above are not evidence about the PR's code.
- The `verified-garbage` rows, and every row outside `sha2`, `hmac` and `hkdf`.
