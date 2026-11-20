<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Fire TV Stick (AFTSSS): rind on 32-bit Cortex-A53

One bench run of rind, aws-lc-rs, ring and OpenSSL on an Amazon Fire TV Stick, model `AFTSSS`
(codename `sheldonp`), on 19 November 2026, 9:38 to 10:12 AM PST, plus the repeat rows and
component probes from the same day that the fork's handoff (fork #217) is built on.

This tree is not part of the comparable set at `d6ca9a75`: the run is a fork commit, it was
noisy (`noisy: true`), and it covers 14 of the bench binaries. Read it as a first look at the
`arm` backend on an in-order 32-bit core.

## Device

| | |
|---|---|
| Model | `AFTSSS` (`ro.product.model`), MediaTek MT8695 (`ro.board.platform=mt8695`) |
| CPU | 4x Cortex-A53 r0p4 (`CPU part 0xd03`), 1.69 GHz in every snapshot, `interactive` governor |
| Userland | 32-bit, `armeabi-v7a`, kernel 4.4.162+ `armv7l` |
| CPU features | `/proc/cpuinfo`: `neon vfpv4 idiva lpae aes pmull sha1 sha2 crc32` (full line in [raw/cpuinfo.txt](raw/cpuinfo.txt)) |
| auxv | `AT_HWCAP=0x3fb0d6` (4174038), `AT_HWCAP2=0x1f`, from `rind-cpuid` ([raw/tv/cpuid.json](raw/tv/cpuid.json)) |
| Memory | 921136 kB total, 50 to 110 MB free with swap in use |
| OS at the run | Fire OS 7.7.0.4, Android 9 (API 28), build `PS7704.5024N`, security patch 2025-06-01 |
| OS on 20 Nov, 12:48 AM PST | build `PS7717.5746N`, security patch 2026-03-01 (read-only `getprop`). The device updated after the run, so a rerun is on a newer Fire OS. |

## How it was run

- rind: steelhead99x/rind `ac0f2f936cb25ab2f737fca1adde5923f64fa60a` (the fork commit of the
  Galaxy A16 run), with a local `rind-cpuid` probe patch that became fork #207.
- Build: `cargo +1.98.1 bench -p rind-bench --features __bench_openssl,openssl-sys/vendored
  --target armv7-linux-androideabi --no-run`, Android NDK r28c (`armv7a-linux-androideabi28-clang`),
  cross-compiled on an x86_64 Linux host.
- Run: binaries pushed with `adb` to `/data/local/tmp`, one core (`taskset 8`, CPU 3),
  `--noplot --warm-up-time 1 --measurement-time 3`, filtered to rind, aws-lc-rs, ring and OpenSSL.
  [harness/run.sh](harness/run.sh) is the on-device loop; the host scripts are in fork #217
  (`admin/handoff-firetv-armv7/scripts/`).
- The on-device loop was cut off at about 17:47Z when the TV rebooted. `aes_gcm`,
  `chacha20poly1305` and `xchacha20poly1305` come from that pass; the other 11 binaries
  (with `cipher` from scratch) ran from the host, one `adb` call each.
- Same day, on fork main `847a3e3c`: a back-to-back repeat of the ChaCha20-Poly1305, AES-128-GCM
  and P-256 rows ([raw/handoff/bench-main-repeat.txt](raw/handoff/bench-main-repeat.txt)), the rows
  skipped by the first run, one binary at a time with peak RSS
  ([raw/handoff/lowmem-run.txt](raw/handoff/lowmem-run.txt),
  [raw/handoff/lowmem-rind-rows.txt](raw/handoff/lowmem-rind-rows.txt)), and a component probe
  ([raw/handoff/probe-components.txt](raw/handoff/probe-components.txt)).

## Files

| File | What it is |
|---|---|
| [firetv-aftsss.json](firetv-aftsss.json) | `admin/bench-report` schema 1, 164 results |
| [RESULTS.md](RESULTS.md) | rind against the other libraries, and against the Galaxy A16 and Pixel 9 Pro |
| [NOTES.md](NOTES.md) | the run summary written at the time |
| [raw/tv/](raw/tv/) | `run.log` (criterion output), `snap.log` (clock and memory per bench), `stat-before`, `stat-after`, `cpuid.json`, start and finish times |
| [raw/before.txt](raw/before.txt), [raw/during-freq.txt](raw/during-freq.txt), [raw/after.txt](raw/after.txt) | `top`, clock and memory snapshots |
| [raw/handoff/](raw/handoff/) | the repeat, low-memory and probe data behind fork #217 |

## Caveats

- `com.amazon.whad` used about one core while the TV was idle; thermals are not readable from the
  `shell` uid; `/proc/stat` went backwards across the run, so there is no busy share.
- The first run's AES-128-GCM open 16 KB row (37.8 µs) did not repeat on main (32.6 to 32.85 µs).
- `load.note` in the JSON says "1 of 8 CPUs" and names A78 and A55 cores. That text came from the
  Galaxy A16 run and is wrong for this 4-core A53 device; `noisy_reason` is correct. Left as recorded.
- `build_cost` is `failed`: the binaries were cross-compiled on another host.
- Not committed: `criterion.tar` (2.6 MB of criterion state; `run.log` has the same estimates), the
  `getprop` dump, and the read-only security notes, which carry device and network identifiers.

## Redactions

The adb serial, the `/proc/cpuinfo` `Serial`, the build host's name and home paths are removed.
No MAC address, IP address or account appears in these files.

## PRs

- fork #207, merged: `rind-cpuid` on 32-bit ARM Linux and Android.
- fork #217, merged: findings, scripts and fix checklist (`admin/handoff-firetv-armv7/`).
- fork #215 and #222, merged: P-256 on armv7 with OpenSSL's `ecp_nistz256-armv4` field arithmetic
  (measured on a Raspberry Pi 2 B, not yet on this device).
