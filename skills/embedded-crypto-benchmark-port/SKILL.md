---
name: Embedded crypto benchmark port
description: >-
  When asked to benchmark a Rust crypto or other performance library on an
  attached microcontroller (ESP32 or similar), port an upstream project's bench
  suite to the board, and publish firmware and results to repos.
---
<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Embedded crypto benchmark port

## 1. Inspect the hardware (on the user's machine, since the board is plugged in there)
- List serial ports and use udevadm to tell boards apart by vendor/model. Never touch boards that aren't the target.
- A "BOOT" USB name doesn't mean the device is in its bootloader. Look for a mass-storage interface and check who holds the port (services such as bridges).
- Identify the chip with esptool (chip_id, flash_id): arch, cores, clock, flash, PSRAM.
- Check which toolchains exist (rustup, espup, espflash, ESP-IDF, PlatformIO) and install user-local only, no sudo.
- For a display, run an I2C scan and check existing projects for pin configs. Don't guess the hardware.

## 2. Back up before flashing any device you don't own the firmware of
- Export the app config and info, copy the full current firmware (UF2 CURRENT.UF2 or similar), fetch the matching official release as a second restore path, and checksum everything.
- Flash only the app region, and verify the SoftDevice and bootloader regions are byte-identical afterwards.
- Restore, diff the info before and after, and restart any services you stopped.

## 3. Decide what to measure
- Read the upstream project's own bench code, benchmark report, and any author blog post. Reuse its operations, sizes, and competitor set so results line up with its published numbers.
- Check whether the library has assembly for the target. If not, say the portable path is being measured.
- Try every competitor on the target. When one fails to build, record the concrete reason instead of dropping it silently.
- Add the hardware accelerators as a clearly labelled baseline when practical.

## 4. Benchmark honestly
- Correctness first: every candidate must pass known test vectors and agree with the others. Only race the ones that pass.
- Same work: same keys, nonces, sizes, core, and clock, plus the same seeded RNG when an op draws randomness. Radios and interrupts off, caches warmed up.
- Time with the cycle counter, take many samples, and report the median plus spread, ns/op, cycles/op, and cycles/byte. Include binary size.
- Run the full suite at least 3 times. Also rebuild once with a different layout: results that repeat exactly can still shift 0.6-1.6x on small inputs after a relink.
- Keep UI, display work and prints out of timed regions: anything between timed calls (even one print) can evict the instruction cache and inflate a measurement 10x. Time calls in a tight loop and print afterwards. Start runs on an explicit host go line, since ModemManager can swallow serial output.
- Host benches: the upstream tool may refuse a busy host. Disclose any relaxed limit and keep the noisy flag, and don't build while a run is going.

## 5. Explain every gap with evidence
- The upstream report tool refusing undocumented gaps is the point: each row over its threshold needs a measured reason, not a guess.
- A large fixed per-call cost hidden at big sizes is instruction-cache misses until shown otherwise. Rebuild with a doubled instruction cache as a diagnostic; publish at the default config and keep the bigger-cache runs as diagnostics.
- To rule the RNG in or out, count it: wrap the platform RNG backend (e.g. getrandom's custom backend) with call/byte/cycle counters, and add a seeded deterministic RNG behind a feature for a second build. When adding cfg rustflags, pass them via cargo `--config` (which merges with config.toml) rather than RUSTFLAGS (which replaces them).
- Profile without JTAG when debug access needs root: a high-priority RTOS task on the other core can sample the preempted task's saved PC each tick; take a few hundred samples, run addr2line, and classify by function. 64-bit-limb code on 32-bit cores is a common culprit.

## 6. Verify before you publish
- Recompute every published number from raw with a separate script that shares no code with the report generator (in rind-results, `cargo verify` beside `cargo xtask`): table cells, chart labels, display pages, the fastest-library call, ratios and spreads. It has already found real errors the generator could not see in itself: truncated OLED headings that merged three pages, and provenance taken from HEAD at report time.
- Record provenance at build time, not report time: per image sha256, source commits, patches, build time.
- Recheck by reflashing the published images (verified by hash) and rerunning; medians should land within max(1%, run spread).
- Build self-checks into the firmware: cycle counter vs an independent timer at boot, wall time vs cycles per benchmark, and the empty-call harness cost (report it next to the fastest row; don't subtract).
- Check time grows with input size; a row that gets faster with more input may be optimised out or a layout effect.
- Compare against public figures for the same chip (vendor minimum hardware throughput, third-party software numbers).
- Gaps in bench files the pinned report doesn't know still need a recorded reason.
- Re-pull upstream before finishing; rerun affected benches as a separate labelled set rather than mixing into the pinned set.
- ESP32: espflash writes the flash clock into the image header and defaults to 40 MHz regardless of sdkconfig. Always pass `--flash-freq 80mhz` and confirm the bootloader's `Boot SPI Speed` line in every run log; a wrong clock makes cache-missing rows up to ~2x slow.
- Relink at least once and publish per-row shifts; on small instruction caches even the target library's short-input rows move a lot, so never claim layout immunity from one relink.
- Display pages: keep headings unique and fitting; fail generation instead of truncating.
- A patch script running `git submodule update --init` resets a hand-checked-out submodule; restore the pin before rerunning.

- Shared devices: keep the window short (trim to the essential runs), and measure downtime from the service's own "connected" log line, not from `systemctl is-active` (a service can auto-restart and show active while it can't reach the device). The first image flashed over a running vendor firmware may reset early, possibly from a hardware watchdog it left armed; record a run that fails to start rather than silently retrying.
- When the pin moves, check whether the upstream report generator gained operations, and measure layout sensitivity with padded relinks (e.g. shifts of 0, ~1 KB, ~2 KB) and publish per-layout ratios plus the median.

- nRF52 boards coming off a vendor firmware/bootloader may have the hardware watchdog running (it cannot be stopped short of a watchdog, pin or power-on reset). Read WDT RUNSTATUS/CRV/RREN at boot, log them, and reload every enabled RR register between timed regions and while idle. Also stop any companion timer/watcher that restarts the device's bridge service during the window, and don't chmod/copy-metadata onto a UF2 drive (it unmounts after the write).

- Check with `nm -n` what a padded relink actually moved before describing it: padding can land after the libraries, moving only the harness. Put padding ahead of the code you want to shift, and record each library's shift.
- Before calling a post-repin shift a code change, diff the upstream source for that path, the lock file and function sizes (`nm -S`). If all match, compare relative placement and rerun with a cache large enough to hold the path; identical cycles there mean layout.

- Rerun the unpadded build in the same session as a control, and include a padding that differs mod 16 (paddings 1 KB apart keep the same alignment and may time identically). Label any relink that moved libraries only a few bytes as such.

## 6b. Rewriting tooling in another language
- Prove parity before deleting the old tools: regenerate every artifact with both and diff; record byte-identical vs wording-only vs pixel-identical differences in a diag folder.
- Byte-matching JSON needs the same key order, separators and float formatting; Rust's `{:.2}` rounds like Python's `.2f`. PNG bytes only match with the same zlib, so compare pixels.
- Keep the verifier independent of the report generator (no shared computation code) and test it with a deliberate mutation.
- Copy instead of rename when writing outputs that may cross filesystems.

## 7. Debugging crashes
- Run the image in the vendor's QEMU to get clean faults when hardware reset-loops.
- On Xtensa, over-aligned locals (align > 16) can make LLVM realign the stack without movsp, which breaks the register windows: interrupt saves then corrupt locals. Search the disassembly for functions that realign the stack; a small patch lowering the alignment on that target fixes it. Verify on hardware before calling it fixed.
- Work out stack size from the disassembled frame sizes. ESP-IDF sdkconfig paths need to be absolute when the target dir is elsewhere.
- A reset-looping board can get its USB port disabled by the hub, and recovering it then needs a physical replug. Disable reboot-on-panic for debug builds to avoid this.

## 8. Structure for upstream
- Follow the upstream project's contribution rules: license, fmt, clippy, tests, commit style, unsafe/asm policy, and report rules such as gap reasons.
- Keep custom glue and patches small and in a clearly separate place, with minimal repros. Never open upstream PRs or issues without the user's go-ahead.
- Firmware port and results/tools can live in separate repos that link to each other. Match upstream's license and keep attribution.
- For this work the firmware repo is [steelhead99x/rind-hardware](https://github.com/steelhead99x/rind-hardware) (public; formerly the private `steelhead99x/rind-bench`, archived). Recorded provenance SHAs live in `legacy_rind_bench`; `rind_hardware` maps them onto the public history (`tools/legacy-rind-bench-map.json` in rind-results). Map by tree, or treat squash root `8d9c1bb` as a match when the only differences are the publish-time rename and secret scrub (RP2040 tip `f656f95`). Earlier SHAs are ancestors: write `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3` and keep the recorded string in `legacy_rind_bench`. Leave crate, binary and protocol names (`rind-bench` crate, `rind-bench:go`, `rind-bench-esp32`, `.text.rind_bench_pad`) alone.
- Scan for home paths before publishing. Other agents may push to the same repo, so fetch and rebase before pushing, and commit often.
- Media: render display pages on the PC at native and 4x size, and record terminals with script and asciinema with names and paths scrubbed.
- A display/demo build that embeds the published results goes stale whenever results change: regenerate the embedded table and rebuild before flashing.

- When a results checkout has diverged after a history rewrite, back up first (`git bundle --all`, diffs, a tar of small untracked files and a manifest of huge ones). Compare local-only commits to origin by tree and patch-id; do not cherry-pick content that is already on the rewritten history or that would re-introduce scrubbed secrets. Reset to origin/main and re-apply uncommitted edits with `git apply --3way`; leave a conflicting patch and report it. Never stash or discard another agent's work. Commit in a clean worktree, rebase, and do not force-push if that would drop someone else's commits.
- A first rewrite can miss home-relative paths in rustc backtraces and cargo compile lines (`~/rind-bench/...`, `~/.cargo/...`). Scan for `~/` and `/home/<user>` as well as ports and MACs. A `by-id` path built from the firmware's own USB product string is a firmware identifier, not a host-specific serial.

## 9. Report
Give the repo URLs, what ran vs. what failed (with reasons), a headline table comparing the target library with the best competitor per operation, surprises, and open issues.

## Lessons learned (target-specific)

The checklist above is the procedure. The notes below keep board- and host-specific details that are easy to lose when the checklist is shortened.

### ESP32-S3 (ESP-IDF std, Xtensa)

Toolchain
- espup's "esp" Rust toolchain plus `xtensa-esp32s3-espidf`, `build-std = ["std", "panic_abort"]`, ldproxy as linker, embuild fetching ESP-IDF into `.embuild/` (`ESP_IDF_TOOLS_INSTALL_DIR=workspace`). Use std/ESP-IDF when the library under test is std-only.
- Long first builds (ESP-IDF fetch) die with the shell session: run them under `nohup` and poll the log.
- Crates with C (ring) need `CC_xtensa_esp32s3_espidf=xtensa-esp32s3-elf-gcc` and `AR_...`: the generic `xtensa-esp-elf-gcc` cc picks emits big-endian objects, and the GCC dir must be on PATH.
- ring on espidf needs feature `less-safe-getrandom-espidf` for `SystemRandom`.
- `asm!` is unstable on Xtensa (`asm_experimental_arch`): any `asm!` barrier in the library needs a `black_box` fallback.
- aws-lc-sys: `#error "Unknown target CPU"`. openssl-src: "don't know how to configure OpenSSL for xtensa-esp32s3-espidf". graviola off x86_64/aarch64 builds but exports only SHA-2/HMAC.
- `esp_clk_cpu_freq`/`xPortGetCoreID` are not in esp-idf-sys bindings: use `esp_rom_get_cpu_ticks_per_us()` and `xTaskGetCoreID(xTaskGetCurrentTaskHandle())`. Bindgen unions: e.g. `i2c_master_bus_config_t.__bindgen_anon_1.clk_source`.

LLVM and link
- The Xtensa backend can abort with "Unable to properly handle scavenged register for indirect jump" on big generated crypto (libcrux-hacl-rs at O3; whole crate under thin LTO): use `lto = false` and a per-package `opt-level = 2`.
- `call8` is out of range when everything is in one image (more than 512 KB of text): build one image per bench file via cargo features. `l32r` literal out of range for a huge function: per-package `opt-level = "s"`. `--mtext-section-literals` crashed LLVM.
- Disclose every per-package opt-level change: it changes whose code is being measured.

Board and timing
- If quad PSRAM init fails ("PSRAM chip is not connected") the board likely has octal PSRAM: turn PSRAM off and keep buffers in internal SRAM, which is also fairer.
- Size the bench thread's stack from the disassembly, not by guess: on Xtensa, `entry a1, N` gives each frame. ML-KEM put a 27 KB frame (rind `DecapKey::generate`) under two ~31 KB bench frames, overflowing 96 KB into the heap; 160 KB fixed it. The symptom was a crash on core 0 or a later `realloc` fault.
- A crash that resets the chip (rst `USB_UART_CHIP_RESET`) drops the USB-Serial/JTAG port every few seconds: capture tools must reopen the port and keep logging.
- Pin the bench thread to core 1 at high priority with a big stack (96 KB for ML-KEM/curves); watchdogs off; no WiFi; check 240 MHz at boot and print it.
- CCOUNT through `xthal_get_ccount()` wraps at 2^32 (about 17.9 s at 240 MHz), so keep samples short (2 ms) and use wrapping subtraction.
- Relink paddings: ESP-IDF's one `.flash.text` rule takes all `.text.*` and can put a pad static after the libraries even when its object is linked first. Have build.rs pass `-Wl,--section-ordering-file=<file>` with `.flash.text : { *(.text.rind_bench_pad) }` (binutils 2.43+) so the static sits at `_stext`. Paddings 1 KB apart (pad1031 and pad2053) kept the same alignment and timed identically; pad1039, a different alignment, moved SHA-256 64B from 2.36× to 3.07× libcrux (2.79× from the medians over three alignments).

### nRF52840 (borrowed Meshtastic node, bare metal)

- Stop the user service, export config and `--info`, copy `CURRENT.UF2` + `INFO_UF2.TXT`, and check what `CURRENT.UF2` covers (here `0x1000`-`0xEA000`, family `0x28860057`, so it cannot be copied back as is). Compare its application range with the official release block for block, then build a family-`0xADA52840` restore UF2 of the exact app region. Checksum everything before flashing.
- Keep the image inside the application region (`0x27000`-`0xED000` with S140 7.3.0 and Adafruit's bootloader); have the UF2 writer refuse anything outside. Never call `embassy_nrf::init`, because it writes UICR. Afterwards, re-read `CURRENT.UF2` to prove the SoftDevice is byte-identical.
- Every exit should lead to the bootloader: a panic, a fault, a lockup, or a 1200-baud touch. Use a 300-baud touch to restart for the next run. Give UsageFault, BusFault and MemManage a priority below USB so they can report pc/cfsr before resetting.
- rind on `thumbv7em` needs `patches/0003` (`no_std`, portable M-profile backend). ring 0.17 needs an `AsmTarget` for os `none` (minus NEON-in-ARM-state assembly), `less-safe-getrandom-custom-or-rdrand`, and a getrandom 0.2 custom backend. getrandom 0.4's custom backend symbol is still `__getrandom_v03_custom`.
- The p256 image overflowed 792 KB at codegen-units 16. Fat LTO fixed it; apply it to every image on that board so they're built alike, and disclose it.
- The nRF RNG with bias correction takes about 170 µs per byte, so rows that draw randomness inside the timed loop (ML-KEM keygen/encaps) can be RNG-bound until measured.
- On cortex-m-rt, give the relink pad `#[unsafe(link_section = ".Reset")]`; `link.x` places `*(.Reset)` first in `.text`.

### RP2040 (Cortex-M0+ / no CAS)

- `patches/0005` makes `ReuseCache` work without CAS, via a `Cell` on targets that lack `target_has_atomic = "ptr"`. ring 0.17 is left out (its Arm assembly uses NEON).
- Timing is SysTick at the processor clock (24-bit hardware, software-extended). A set COUNTFLAG must be consumed in the wrap handler; otherwise `cycles()` can count the same wrap twice (2^24 cycles extra). The boot cross-check should run long enough to cover at least two wraps.
- The RP2040 has no TRNG. A seeded SplitMix64 is fine for "same work" but is not entropy; record `rng` in the start record.
- Known-answer checks have passed on a Pico (2026-11-12). Published timing results are not in rind-results until a `results/rp2040/` tree lands on main; do not claim they are there.

### Publishing a squash and a rewritten results history

- Map recorded provenance SHAs with `tools/legacy-rind-bench-map.json`. The public rind-hardware root is `8d9c1bb`; earlier private trees are `ancestor of 8d9c1bb5e76e5ae8eb2d481535dc63ccd6a8f5f3`. The RP2040 tip `f656f95` maps to `8d9c1bb` when the only differences are the publish-time rename and secret scrub. Leave crate, binary and protocol names (`rind-bench` crate, `rind-bench:go`, `rind-bench-esp32`, `.text.rind_bench_pad`) alone. `cargo verify` and `cargo xtask report` both read the map.
- Never `git rebase --autostash` (or stash) where others have uncommitted edits. If the remote history was rewritten, cherry-pick only your new commits onto the new remote head in a clean worktree, check them for anything the rewrite scrubbed, and push from there.

### Linux SBC hosts (Jetson Orin Nano that runs live services)

- Do not stop the box's own services to free a core. Pick the core from a per-CPU busy sample (`/proc/stat` over 10 s), skip cpu0 (interrupts), and record per-CPU load, frequencies, thermals and the top CPU users before and after every run.
- Record the PC's UTC start and finish when the board's clock is wrong, and publish those.
- `pkill -f <name>` over ssh matches the ssh command line itself and kills your own session; kill by pid or `pgrep -x`.
- Build natively at `nice -n 19` with `-j 2` when the PC lacks a cross gcc; prebuild the bench binaries and rind-cpuid in the same target dir so bench-report's own build step is a no-op, and run nothing else while it times.
- On aarch64 the host's rind-cpuid line says which paths run: Cortex-A78AE has aes, pmull, sha2 and NEON but not the SHA-512/SHA-3 extension or DIT. libcrux's NEON code sits behind its `simd128` feature, which rind-bench does not enable, so libcrux runs its portable code; say so beside its rows.
- Run the boards' known-answer checks on the host too (rind-hardware `host-kat/`), then rind's own test suite with its vector submodules, then every bench once in criterion's `--test` mode, before any timed run.
- Check the core-picking output before you trust a pin. In awk, `printf "%s", x, d>0 ? a : b` parses `>` as an output redirect, writes a stray file named `0...` and prints nothing. Parenthesize ternaries inside `printf`, and stop and restart a run whose `pinned to` line doesn't follow the busy sample.
- Write a short PROGRESS.md beside the staging files before each long step, and land tooling commits as soon as they pass verify, so an interrupted session resumes without redoing work.
