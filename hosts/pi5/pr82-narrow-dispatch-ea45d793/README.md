<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Pi 5: hardware checks for rind #82 (aarch64 narrow-multiplier dispatch) at `ea45d793` (2026-11-17)

Correctness checks on the Raspberry Pi 5 (`pi5`: BCM2712, 4x Cortex-A76 r4p1, capped at 1.5 GHz with no fan, 8 GB,
Debian 13, Linux 6.18.60+rpt-rpi-2712, rustc 1.98.1 from rustup, valgrind 3.24.0) for
[steelhead99x/rind#82](https://github.com/steelhead99x/rind/pull/82) at
`perf/aarch64-narrow-dispatch` = `ea45d7936f3d46b9e3e6ec7b3fe246e86c590881` (kixel-ai/rind `main` merged in). The
upstream branch `up/aarch64-narrow-dispatch` (`580a3dbf`) carries the same aarch64 sources.

These are correctness runs, not timings. Durations include building on the Pi (`nice`, 3 jobs).

## Schedule selected

`MIDR_EL1` = `0x414fd0b1` (implementer `0x41`, part `0xd0b`, Cortex-A76 r4p1), and `HWCAP_CPUID` is set, so
`wide_multiplier()` is **false**: run natively, the Pi takes the **narrow** schedules. `run-detect.log` confirms it.
`detects_the_expected_cpu` passes with `RIND_TEST_ARM_WIDE_MUL=0`. With `RIND_TEST_ARM_WIDE_MUL=1` and no force it
fails (`left: false, right: true`), and that second run is a deliberate negative control.

## Results

| run | command / env | result | tests | duration |
|---|---|---|---|---|
| 1 | `cargo test -p rind --lib --locked -- low::aarch64::` (native detection: narrow) | pass | 172 passed, 11 ignored (`ct_qemu_*`) | 281 s (incl. cold build) |
| 2 | `RIND_CPU_FORCE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=1` … `cpu::tests::detects_the_expected_cpu low::aarch64::schedules low::aarch64::mont` | pass | 5 passed, 6 ignored (`ct_qemu_*`) | 1 s |
| 3 | `RIND_CPU_DISABLE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=0` … `low::aarch64::` | pass | 172 passed, 11 ignored | 5 s |
| detect | native `detects_the_expected_cpu` + `detected_value_matches_midr` with `RIND_TEST_ARM_WIDE_MUL=0`, then `=1` as a negative control | narrow confirmed | 2 passed; control failed as expected | <2 s |
| 4 | `admin/ctgrind --release --locked --no-fail-fast` (`RIND_TEST_ARM_CRYPTO_EXT=1 RIND_TEST_ARM_SHA3=0 RIND_TEST_ARM_SVE2=0`, as CI sets them) | **pass** (EXIT 0) | pass 1: 921 passed (unit 801, wycheproof 21, adapters 99), 43 ignored; narrow pass (`RIND_CPU_DISABLE_wide_mul=1`): 926 passed (unit 806, wycheproof 21, adapters 99), 43 ignored; 52 memcheck summaries, all 0 errors | 9182 s |
| 4b | `admin/ctgrind fips --release --locked --no-fail-fast` | _pending_ | | |
| 4c | extra: `rind` unit suite under memcheck with `RIND_CPU_FORCE_wide_mul=1` (`--cfg rind_cpu_disable`) | _pending_ | | |
| 5 | `admin/prove bytes` | **pass** (EXIT 0) | 189 aarch64 + 155 x86_64 routines compared, none absent | 98 s |
| ct2-native | extra: `low::aarch64::` under memcheck, `--cfg rind_cpu_disable`, `RIND_TEST_ARM_WIDE_MUL=0`, no toggle | 1 failure (see below) | 171 passed, 1 failed, 11 ignored; 0 memcheck errors | 547 s (incl. build) |
| ct2-forcewide | extra: same with `RIND_CPU_FORCE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=1` | pass | 172 passed, 11 ignored; 0 memcheck errors | 71 s |

The `ct_qemu_*` tests are `#[ignore]`d on purpose, because `admin/ct-qemu` runs them under qemu. Nobody ran them here.

## Finding: valgrind hides `HWCAP_CPUID`, so memcheck sees a wide core

The one failure, ct2-native, is `low::aarch64::cpu::tests::detects_the_expected_cpu` at `rind/src/low/aarch64/cpu.rs:589`
(`left: true, right: false`). Under valgrind, `wide_multiplier()` returned true. The probe in
[`scripts/midrprobe.c`](scripts/midrprobe.c) ([output](logs/midrprobe.log)) shows why:

| | `AT_HWCAP` | `HWCAP_CPUID` | `MIDR_EL1` |
|---|---|---|---|
| native | `0x119fff` | 1 | `0x414fd0b1` (part `0xd0b`) |
| valgrind 3.24.0 | `0x1001fb` | 0 | not read |

Valgrind clears `HWCAP_CPUID` in the auxv it hands the guest. `read_midr()` gates on that bit, as its doc comment says
("valgrind cannot decode the register without it"), so it returns `None`, and detection falls back to `_alt` by design.
Valgrind isn't emulating the register with a different value: the read never happens.

What this means:

- On any aarch64 host, `admin/ctgrind`'s first pass checks the `_alt` (wide) schedules, and its second pass
  (`RIND_CPU_DISABLE_wide_mul=1`) checks the narrow ones. That holds on the wide arm64 CI runners and on narrow
  hardware like this Pi alike, so CI covers both schedules either way. The run 4 numbers above match this.
- The failure comes from this report's own ct2 harness. It set `RIND_TEST_ARM_WIDE_MUL=0` without
  `RIND_CPU_DISABLE_wide_mul`, which asserts native detection under valgrind. `admin/ctgrind` never does that: its first
  pass leaves `RIND_TEST_ARM_WIDE_MUL` unset, and its narrow pass sets the variable together with the denial. This is the
  likely explanation for the handoff [steelhead99x/rind#150](https://github.com/steelhead99x/rind/pull/150). The 4c
  pending row is also redundant here, since on this host pass 1 already ran `_alt`.
- A possible hardening: document that the variable is meaningless under valgrind without a toggle, like
  `RIND_TEST_ARM_SHA3=0`, which CI sets because valgrind hides SHA3. Alternatively, have `detects_the_expected_cpu` skip
  the `WIDE_MUL` assertion when `HWCAP_CPUID` is absent and no toggle is set.

## Setup

- A fresh clone at `ea45d793`. The repo has no `rust-toolchain` file, and `flake.nix` pins `rust-bin.stable."1.98.1"`, which
  is the Pi's rustup default. `llvm-tools` and the `x86_64-unknown-linux-gnu` target were added for `prove bytes`.
- No Nix and no `sudo` on the board. The flake shells' extra tools came from Debian 13 `.deb`s unpacked with `dpkg -x` into a
  user directory ([`scripts/tools-env.sh`](scripts/tools-env.sh)): `libclang1-19` and `libclang-common-19-dev` for crabgrind's
  bindgen (the `.#ctgrind` shell's `LIBCLANG_PATH`), `libssl-dev` for `pkg-config openssl`, and `binutils-x86-64-linux-gnu`,
  `cpp-x86-64-linux-gnu`, and `gawk` for `prove bytes`. `prove bytes` needs gawk's `strtonum`, which Debian's default
  `mawk` lacks.
- The driver scripts are in [`scripts/`](scripts/): `runs.sh` covers runs 1-3, `ct.sh` covers 4, 4b, and 4c, `ct2.sh`
  covers the ct2 rows, and `prove.sh` covers 5.
- [`logs/`](logs/) has the trimmed logs: start and exit lines, test-result lines, failures with their context, and the count
  of memcheck `ERROR SUMMARY` lines. A summary with any errors would be kept in full. `prove-bytes.log` is complete.
  The logs also include `cpuinfo.txt` (model, part, and features), `uname.txt`, `toolchain.txt`, and `valgrind-version.txt`.
