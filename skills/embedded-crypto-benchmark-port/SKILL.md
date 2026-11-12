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
- Identify the chip with esptool (chip_id, flash_id): arch, cores, clock, flash, PSRAM.
- Check which toolchains exist (rustup, espup, espflash, ESP-IDF, PlatformIO) and install user-local only, no sudo.
- For a display, run an I2C scan and check existing projects for pin configs. Don't guess the hardware.

## 2. Decide what to measure
- Read the upstream project's own bench code, benchmark report, and any author blog post. Reuse its operations, sizes, and competitor set so results line up with its published numbers.
- Check whether the library has assembly for the target. If not, say the portable path is being measured.
- Try every competitor on the target. When one fails to build, record the concrete reason instead of dropping it silently.
- Add the hardware accelerators as a clearly labelled baseline when practical.

## 3. Benchmark honestly
- Correctness first: every candidate must pass known test vectors and agree with the others. Only race the ones that pass.
- Same work: same keys, nonces, sizes, core, and clock. Radios and interrupts off, caches warmed up.
- Time with the cycle counter, take many samples, and report the median plus spread, ns/op, cycles/op, and cycles/byte. Include binary size.
- Run the full suite at least 3 times to show run-to-run variance, and disclose what the numbers don't prove.
- Keep any UI or display work out of timed regions.

## 4. Structure for upstream
- Follow the upstream project's contribution rules: license, fmt, clippy, tests, commit style, unsafe/asm policy, and report rules.
- Keep custom glue and patches small and in a clearly separate place, and note what an upstream PR would contain. Never open upstream PRs or issues without the user's go-ahead.
- Firmware port and results/tools can live in separate repos that link to each other. Match upstream's license and keep attribution for anything copied.
- Sync the latest upstream before final runs, and push progress as you go.

## 5. Report
Give the repo URLs, what ran vs. what failed (with reasons), a headline table comparing the target library with the best competitor per operation, surprises, and open issues.

## Lessons learned (ESP32-S3, Rust, ESP-IDF std)

Toolchain
- espup's "esp" Rust toolchain plus `xtensa-esp32s3-espidf`, `build-std = ["std", "panic_abort"]`, ldproxy as linker, embuild fetching ESP-IDF into `.embuild/` (`ESP_IDF_TOOLS_INSTALL_DIR=workspace`). Use std/ESP-IDF when the library under test is std-only.
- Long first builds (ESP-IDF fetch) die with the shell session: run them under `nohup` and poll the log.
- Crates with C (ring) need `CC_xtensa_esp32s3_espidf=xtensa-esp32s3-elf-gcc` and `AR_...`: the generic `xtensa-esp-elf-gcc` cc picks emits big-endian objects, and the GCC dir must be on PATH.
- ring on espidf needs feature `less-safe-getrandom-espidf` for `SystemRandom`.
- `asm!` is unstable on Xtensa (`asm_experimental_arch`): any `asm!` barrier in the library needs a `black_box` fallback.
- aws-lc-sys: `#error "Unknown target CPU"`. openssl-src: "don't know how to configure OpenSSL for xtensa-esp32s3-espidf". graviola off x86_64/aarch64 builds but exports only SHA-2/HMAC.
- `esp_clk_cpu_freq`/`xPortGetCoreID` are not in esp-idf-sys bindings: use `esp_rom_get_cpu_ticks_per_us()` and `xTaskGetCoreID(xTaskGetCurrentTaskHandle())`. Bindgen unions: e.g. `i2c_master_bus_config_t.__bindgen_anon_1.clk_source`.

LLVM Xtensa backend
- It aborts with "Unable to properly handle scavenged register for indirect jump" on big generated crypto (libcrux-hacl-rs at O3; whole crate under thin LTO): use `lto = false` and a per-package `opt-level = 2`.
- Link errors: `call8` out of range when everything is in one image (more than 512 KB of text): build one image per bench file via cargo features. `l32r` literal out of range for a huge function: per-package `opt-level = "s"`. `--mtext-section-literals` crashed LLVM.
- Disclose every per-package opt-level change: it changes whose code is being measured.

Board and timing
- If quad PSRAM init fails ("PSRAM chip is not connected") the board likely has octal PSRAM: turn PSRAM off and keep buffers in internal SRAM, which is also fairer.
- Size the bench thread's stack from the disassembly, not by guess: on Xtensa, `entry a1, N` gives each frame. ML-KEM put a 27 KB frame (rind `DecapKey::generate`) under two ~31 KB bench frames, overflowing 96 KB into the heap; the symptom was a crash on core 0 ("Unhandled debug exception", corrupted backtrace) or a later `realloc` fault. 160 KB fixed it.
- A crash that resets the chip (rst `USB_UART_CHIP_RESET`) drops the USB-Serial/JTAG port every few seconds: capture tools must reopen the port and keep logging, or the evidence is lost.
- Pin the bench thread to core 1 at high priority with a big stack (96 KB for ML-KEM/curves); watchdogs off; no WiFi; check 240 MHz at boot and print it.
- CCOUNT through `xthal_get_ccount()`; it wraps at 2^32 (about 17.9 s at 240 MHz), so keep samples short (2 ms) and use wrapping subtraction.
- Cycle counts that repeat to 0.0% are not robust: relinking the same benches (adding display code) moved competitors' 32-64 B rows by 0.6-1.6x on a 16 KB flash-backed instruction cache. Rebuild with a changed layout once and report the shift as the small-input margin.
- Try a bigger instruction cache as a diagnostic (`CONFIG_ESP32S3_INSTRUCTION_CACHE_32KB`): it separates cache misses from compute, and turned rind's 6.6x SHA-256 gap into 1.06x. Publish at the default and say so.
- espflash flashes its own bundled bootloader unless you pass the build's: record which one was used.

Shared machines and other boards
- rind's `admin/bench-report run` refuses a host where other work keeps over 10% of the CPUs busy. If a resident load (someone's stream) may not be stopped, relax `BUSY_SHARE` in a reverted local edit, pin with `taskset` to one physical core's SMT pair, and record governor, boost, EPP and load; it still marks the run `noisy`. Outside Nix, `RUSTUP_TOOLCHAIN=<flake.nix's version> IN_RIND_DEV_SHELL=1` stands in for the dev shell.
- Don't run builds on the same machine while a host benchmark runs.
- Check what else uses a serial port before touching it (`/proc/*/fd`, systemd user units): a "BOOT" USB product string (Seeed's T1000-E reports `T1000-E-BOOT` even running Meshtastic) does not mean the board is in its bootloader; look for a mass-storage interface (class 08) instead.
- Another agent's `espflash board-info` on the bench port mid-run left the capture waiting forever: give captures a per-run timeout well under the default, and coordinate port ownership.

Porting a criterion suite
- A tiny criterion API shim renamed `criterion` lets the upstream bench files compile unchanged via `#[path]` modules; print one JSON line per result with upstream's IDs so upstream's report tool can read them.
- Cross-check every AEAD on the bench's own input (e.g. 1350 B), not only the RFC vector, and run KATs with heap and stack guards: a heap corruption showed up as a crash in a `Vec` realloc right after a library's seal, before any timing.
- Capture script: reset via RTS, read until a `done` record, treat a panic or reboot banner as a failed run, and keep the raw log next to the JSONL.

OLED demo (SSD1306 128x64 on I2C)
- Keep the UI as a `no_std` embedded-graphics crate drawing onto any `DrawTarget`, plus a host `Frame` target so every screen renders to PBM/PNG/Markdown text without the board.
- Draw only from an observer called between benchmarks (before setup and warm-up, after the last sample); the panel self-refreshes, so a shown frame costs nothing. Count benchmarks with a dry run for the progress bar. Publish numbers from images built without the display feature.
- Layout that reads at 128x64: inverted 9 px title bar (5x8 font), 5 rows of 9 px with a 4-char library tag, bar scaled to the slowest (target filled, rivals outlined), value right-aligned in 5x8, 4x6 footer with "X 1.19x faster than Y". 4x6 is too small for numbers.
- Don't commit rendered PNGs when the upstream rules forbid binaries or build artifacts; commit the renderer.
