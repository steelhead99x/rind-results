<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# rind-results

Results, capture and report tools, and run logs for
[rind](https://github.com/moq-dev/rind)'s own benchmark suite, run off its usual hosts:

| Host | What | Results |
|---|---|---|
| `esp32s3` | ESP32-S3 at 240 MHz, rind's portable backend, firmware from [steelhead99x/rind-bench](https://github.com/steelhead99x/rind-bench) | [results/RESULTS.md](results/RESULTS.md), `results/esp32s3.json` |
| `msi-001` | x86_64 laptop (AMD Ryzen 5 7535HS), rind's own `admin/bench-report run` | [results/host/](results/host/) |

Both JSON files are in rind's `benchmarks/data/<host>.json` schema 1, so they line up with
rind's [BENCHMARKS.md](upstream/rind/BENCHMARKS.md) and `admin/bench-report summary` can read them.

![rind's time over the fastest other Rust library on the ESP32-S3](media/charts/esp32s3-rind-vs-fastest.svg)

Charts, screenshots, the OLED pages rendered at native and 4x, terminal recordings of the checks,
builds, flashes and runs, and the serial logs: [media/](media/README.md).

## Layout

| Path | What |
|---|---|
| `results/RESULTS.md` | the ESP32-S3 report: headline, every operation, known gaps, known-answer checks, code size |
| `results/esp32s3.json` | report operations, schema 1; `esp32s3-all.json` has every benchmark |
| `results/raw/<bench>/run-N.{log,jsonl}` | each run's full serial log and its JSON records; `runs.json` says what ran and how it ended |
| `results/diag/` | diagnostic builds, never in the report: 32 KB instruction cache, the display build, the AEAD guard probe, a single ML-KEM run |
| `results/gaps.toml` | the decision on every gap over 10%, in rind's `benchmarks/gaps.toml` format |
| `results/sizes.json`, `meta-esp32s3.json`, `images.sha256` | code size by crate group, toolchain and board facts, the images measured |
| `results/demo.tsv` | the table the OLED demo cycles |
| `results/host/` | the x86_64 host run: `msi-001.json` and its power and load facts |
| `media/` | charts, screenshots, rendered OLED pages, terminal recordings ([index](media/README.md)) |
| `tools/` | `charts.py`, `md2png.py`, `script2cast.py`, `setup-toolchain.sh`, `run-all.sh` (flash and capture every image 3 times), `capture.py`, `report.py`, `sizes.py`, `demo_tsv.py`, `host-run.sh` |
| `upstream/rind/` | rind's agent guidance, quests, bench report tool, benchmark data and bench CI, verbatim, with attribution |
| `skills/embedded-crypto-benchmark-port/` | a reusable agent skill for this kind of port, with lessons learned |

## ESP32-S3 flow

```sh
tools/setup-toolchain.sh                       # rustup, espup (esp 1.97.0.0), ldproxy, espflash, pyserial venv
git clone --recursive https://github.com/steelhead99x/rind-bench ../rind-bench
(cd ../rind-bench && ./scripts/apply-patches.sh && cd esp32 && ./build-images.sh)
tools/run-all.sh ../rind-bench/esp32/images <PORT> 3   # 3 runs per image, raw logs to results/raw
tools/report.py --gaps results/gaps.toml       # RESULTS.md, esp32s3.json; fails on an undecided gap
tools/sizes.py && tools/demo_tsv.py
```

`report.py` takes each run's per-benchmark median and reports the median over runs, the
run-to-run spread, cycles per byte or per operation, and rind's time over the fastest other Rust
library. It refuses a gap of more than 10% without a `gaps.toml` entry, as rind does.

## What was found on the ESP32-S3

- Nine of eleven images ran, three times each, cycle counts repeating to 0.0%. The
  `chacha20poly1305` and `xchacha20poly1305` images crash in their known-answer checks (open; see
  rind-bench's README). aws-lc-rs and OpenSSL do not build for Xtensa; graviola has nothing to race
  there.
- rind is the fastest Rust library on AES-GCM (0.59-0.89x the next), QUIC header protection,
  P-256 ECDH and verify, Ed25519 and ML-KEM, and level on X25519.
- Its SHA-256, HMAC and HKDF gaps are instruction-cache misses: with ESP-IDF's default 16 KB
  cache it is 6.6x libcrux at 64 B; with a 32 KB cache (results/diag) it is 1.06x and fastest on
  HMAC and HKDF.
- SHA-512 is 62x ring's: the unrolled 64-bit code is ~22,000 instructions a block and streams from
  flash at any cache size. The same code is the libcrux row.
- Layout matters at small sizes: relinking the same benches moved ring's, libcrux's and
  RustCrypto's 32-64 B rows by 0.6-1.6x (rind's by under 2%). Read small-input ratios with that
  margin.
- The hardware baseline (`esp32s3-hw`, mbedTLS on the accelerators) is 4x rind on AES-GCM 1350 B
  and 19-31x on SHA-256 at 1350 B-16 KB; it is not a Rust library and is never counted as a competitor.

## What was found on msi-001 (x86_64)

rind's own suite with every competitor (aws-lc-rs, ring, RustCrypto, graviola, libcrux, OpenSSL
3.5.8), see [results/host/](results/host/README.md). The machine was busy (3.8 CPUs of other work
on average), so bench-report marks the run noisy. rind is fastest or within 10% on 57 of 62 report
operations; the five over 1.1x (`rustls-bulk/chacha20poly1305` 1.12x, `aes256-gcm-open/1350B`
1.40x, which is not credible beside its 195 ns AES-128 twin, `p256-ecdsa-verify` 1.22x graviola,
`rsa2048-pss-sha256-sign` 1.19x aws-lc-rs, `p521-ecdsa-verify` 1.14x aws-lc-rs) need a quiet rerun.

## Disclosures

- Firmware runs ESP-IDF v5.5.1, but espflash writes its own bundled second-stage bootloader.
- `libcrux-hacl-rs` is built at opt-level 2 and `libcrux-p256` at `s`, with `lto = false`, to
  get past LLVM Xtensa backend aborts and literal ranges. rind's portable backend shares
  `libcrux-hacl-rs`, so this touches rind and libcrux.
- Code sizes are attributed by symbol, a floor, and crates are shared between rind's portable
  backend and its competitors.
- The second board asked for, a Seeed T1000-E (nRF52840), was not used: it is a live Meshtastic
  node held open by a bridge service and not in its UF2 bootloader, and no trustworthy full flash
  backup (application, LittleFS config and keys, SoftDevice, bootloader) could be taken without
  stopping that service, so nothing was flashed.

## License

Apache-2.0 OR ISC OR MIT-0, as rind; see [LICENSE](LICENSE). `upstream/rind/` is the rind
authors' work under the same licenses.
