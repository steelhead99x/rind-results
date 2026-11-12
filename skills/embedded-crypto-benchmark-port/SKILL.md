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

## 6. Verify before you publish
- Recompute every published number from raw with a separate script that shares no code with the report generator (rind-results' `tools/verify.py`): table cells, chart labels, display pages, the fastest-library call, ratios and spreads. It found two real errors the generator could not see in itself: truncated OLED headings that merged three pages, and provenance taken from HEAD at report time.
- Record provenance when you build, not when you report: per image, its sha256, the source commits, the patches applied, and the build time (`images.json`). A tree that has moved on since will name the wrong commit.
- Recheck by reflashing the published images, verified by hash, and rerunning them. Medians should land within max(1%, run spread). If they do, the remaining uncertainty is in the build, not in the noise.
- Build the benchmark's own checks into the firmware:
  - the cycle counter against an independent timer at every boot
  - each benchmark's wall time against its cycles. Wall time can only be larger, because setup runs between samples, so treat it as an upper bound.
  - an empty `iter` and `iter_batched` as the harness's own cost. Report it next to the fastest row; don't subtract it.
- Check that time grows with input size within one message shape. A row that gets faster as its input grows may have been optimised out. Equal-block sizes such as SHA-512 at 32 B and 64 B are the exception.
- Compare the board to public figures for the same chip. Look for the vendor's minimum hardware throughput and for third-party software numbers.
- Gaps outside the upstream report's operation list (a bench file the pinned report doesn't know, such as SHA-3 at 99b509a) still need a recorded reason. The report generator won't ask for one, so check by hand.
- Re-pull upstream before you finish. Check which commits touch code on your target and rerun those benches as a separate, labelled set rather than mixing them into the pinned one.

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
- Cycle counts that repeat to 0.0% are not robust. On a 16 KB flash-backed instruction cache, relinking the same benches moves short-input rows a lot:
  - an added display feature moved competitors' 32-64 B rows by 0.6-1.6x
  - an added harness function moved `sha256/rustcrypto/32B` by 0.12x, and also rind's own rows (by 0.48x)

  ring's small C loops and the 16 KB rows barely move. Relink at least once, publish the per-row shift with a check on whether any gap verdict flips (none did, though magnitudes halved), and never claim one library is layout-immune from a single relink.
- **espflash sets the SPI flash clock in the image header when it flashes, and defaults to 40 MHz whatever `sdkconfig` says.** The app keeps the bootloader's clock. Always pass `--flash-freq 80mhz`, and confirm the bootloader's `Boot SPI Speed` line in every run log (the report prints it, and the verifier fails a set that mixes clocks). The first published set ran at 40 MHz, which made cache-missing rows (SHA-512, short SHA-256) 1.92x slow and biased them against cached code such as ring.
- Before you blame compute for a 30-150x gap between boards, rebuild with a 32 KB instruction cache. ring's and libcrux's X25519/P-256 dropped 13-23x (cache thrash), but libcrux's P-256 dropped only 4% (opt-level `s`), so one diagnostic build sorts them.
- Try a bigger instruction cache as a diagnostic (`CONFIG_ESP32S3_INSTRUCTION_CACHE_32KB`): it separates cache misses from compute, and turned rind's 6.6x SHA-256 gap into 1.06x (both at the 40 MHz flash clock; 2.29x at 80 MHz with 16 KB). Publish at the default and say so.
- espflash flashes its own bundled bootloader unless you pass the build's: record which one was used.
- Display pages: keep every heading unique and fitting at the panel's font. Fail generation on duplicates or overlap rather than truncating: truncated headings merged three QUIC pages into one and turned "1350B" into "135".
- `git submodule update --init` (inside a patch script) resets a submodule you checked out at another commit by hand. Restore the pin with `checkout <pin>` before you rerun the script, and regenerate pinned reports only from the pinned tree.

Shared machines and other boards
- rind's `admin/bench-report run` refuses a host where other work keeps over 10% of the CPUs busy. If a resident load (someone's stream) may not be stopped, relax `BUSY_SHARE` in a reverted local edit, pin with `taskset` to one physical core's SMT pair, and record governor, boost, EPP and load; it still marks the run `noisy`. Outside Nix, `RUSTUP_TOOLCHAIN=<flake.nix's version> IN_RIND_DEV_SHELL=1` stands in for the dev shell.
- Don't run builds on the same machine while a host benchmark runs.
- `taskset` shrinks `os.cpu_count()`'s view for tools that use the affinity mask: rind's `build_cost` step then judged 1.4 busy CPUs against a 2-CPU limit and refused. Pin only the timed step, or expect build cost to be skipped.
- A noisy host run still shows its noise in place: one row at 3.8x its twin (aes256-gcm-open/1350B vs aes128) is the tell. Flag such rows rather than publish them as gaps.
- Check what else uses a serial port before touching it (`/proc/*/fd`, systemd user units): a "BOOT" USB product string (Seeed's T1000-E reports `T1000-E-BOOT` even running Meshtastic) does not mean the board is in its bootloader; look for a mass-storage interface (class 08) instead.
- Another agent's `espflash board-info` on the bench port mid-run left the capture waiting forever: give captures a per-run timeout well under the default, and coordinate port ownership.

Porting a criterion suite
- A tiny criterion API shim renamed `criterion` lets the upstream bench files compile unchanged via `#[path]` modules; print one JSON line per result with upstream's IDs so upstream's report tool can read them.
- Cross-check every AEAD on the bench's own input (e.g. 1350 B), not only the RFC vector, and run KATs with heap and stack guards: a heap corruption showed up as a crash in a `Vec` realloc right after a library's seal, before any timing.
- Capture script: reset via RTS, read until a `done` record, treat a panic or reboot banner as a failed run, and keep the raw log next to the JSONL.

Media for sharing
- Record with `script -q -e --log-timing T -c CMD LOG` (asciinema is rarely installed), then convert to an asciinema v2 cast and a plain log while replacing the home directory, user and host names; keep the raw log and timing out of git (byte offsets break if you edit the log itself).
- Charts: a dependency-free SVG writer (log-scale ratio bars, green faster, red slower) plus `rsvg-convert` PNGs; screenshots: Markdown to HTML (python-markdown) in headless Chrome, shot tall and autocropped with Pillow.
- Render every display page on the host at native resolution and 4x from the same drawing code the firmware uses; that beats photographing a 0.96" panel.
- Re-run one image while recording: it doubles as a reproducibility check (a rebuilt x25519 image matched the published run within 1.1%).
- Scan for home paths before publishing: `sha256sum` with absolute paths leaked the home directory into a results file.

OLED demo (SSD1306 128x64 on I2C)
- Keep the UI as a `no_std` embedded-graphics crate drawing onto any `DrawTarget`, plus a host `Frame` target so every screen renders to PBM/PNG/Markdown text without the board.
- Draw only from an observer called between benchmarks (before setup and warm-up, after the last sample); the panel self-refreshes, so a shown frame costs nothing. Count benchmarks with a dry run for the progress bar. Publish numbers from images built without the display feature.
- Layout that reads at 128x64: inverted 9 px title bar (5x8 font), 5 rows of 9 px with a 4-char library tag, bar scaled to the slowest (target filled, rivals outlined), value right-aligned in 5x8, 4x6 footer with "X 1.19x faster than Y". 4x6 is too small for numbers.
- Don't commit rendered PNGs when the upstream rules forbid binaries or build artifacts; commit the renderer.

- **Crashes inside a library's safe Rust on Xtensa: suspect stack realignment first.** LLVM-Xtensa realigns SP with a plain `add` after `entry` for any local aligned past 32 bytes (espressif/llvm-project#140). Window spills then corrupt live locals, and the crash shows up later in the caller, often as heap corruption. Scan the linked ELF: `objdump -d` for an `and aX, a1, …` within the first instructions of a function lists every affected function. ring's Poly1305 state (`align(64)`) was the only hit; rind-bench patches it to 16.
- **Use Espressif's QEMU when the board misbehaves.** `idf_tools.py install qemu-xtensa` fails its self-test without the system SDL2 and libslirp. Extract the tarball instead, and put `libSDL2-2.0.so.0` from the `pysdl2-dll` wheel and `libslirp` from `dnf download` into a local `LD_LIBRARY_PATH`; no root is needed.
  - Set the console to UART for that build.
  - Skip `adc_hw_calibration` (it spins in QEMU) with a gdb `break` / `return`.
  - Run QEMU under host gdb to read the guest PC and registers at a QEMU crash (`((XtensaCPU*)current_cpu)->env`), and break on `helper_exception_cause` to log the first guest exception.
- **Layered sdkconfig files need absolute paths** once `CARGO_TARGET_DIR` points outside the crate. embuild resolves them against the target dir's parent and silently skips any that are missing.
- **A reset-looping image can get the USB port disabled** by the host hub ("disabled by hub (EMI?)", "unable to enumerate"). Re-enabling it needs root or a physical replug. Build debug images with print-halt, and never leave a boot-looping image on the board.
- **Clear exported env between steps** (`CARGO_TARGET_DIR`, `RUSTFLAGS`): a stale target dir silently copies the wrong ELF.
- **A large fixed per-call cost that the 16 KB-or-bigger rows hide is the instruction cache, until shown otherwise.** XChaCha20-Poly1305 cost rind and libcrux ~2 ms a call (13x RustCrypto at 32 B); a 32 KB cache build brought it to 1.10x. Rebuild with `CONFIG_ESP32S3_INSTRUCTION_CACHE_32KB` before chasing RNG or algorithm theories, publish at the default, and keep the 32 KB runs as a diagnostic.
- **Anything between timed calls evicts the cache.** A `println!` between two measurements of the same seal moved it from 20k to ~220k cycles. Time a call in a loop with nothing else in it, and print after.
- **To rule the RNG in or out, count it.** getrandom 0.4's custom backend (`__getrandom_v03_custom`) can wrap `esp_fill_random` with call, byte and cycle counters, and a seeded SplitMix64 behind a feature gives a deterministic second build. Enable it with `--config 'target.xtensa-esp32s3-espidf.rustflags=["--cfg","getrandom_backend=\"custom\""]'`, because cargo merges `--config` arrays with `.cargo/config.toml`'s rustflags, while `RUSTFLAGS` would replace them.
- **Profile without JTAG.** OpenOCD over the built-in USB-JTAG needs a root udev rule (`LIBUSB_ERROR_ACCESS`). A priority-24 FreeRTOS task on the other core can wake every tick and read the preempted task's saved PC and a0 from `pxTopOfStack` (words 1 and 3). Spin the operation on core 0, take 400 samples, then run `addr2line -fC` on them and classify by function. That showed rind's 1.27x ChaCha20-Poly1305 gap to ring is Poly1305 (libcrux HACL's 64-bit limbs, emulated on 32-bit Xtensa, at 58% of samples), not ChaCha20.
- **Check the gap before deciding a fix "verified".** report.py refusing undocumented gaps is the point: every new row over 10% needs measured evidence (cache diagnostic, RNG count, profile), not a guess.
- **A display build that embeds the published table goes stale whenever results change.** Regenerate the table (`demo_tsv.py`), rebuild the showcase, and only then flash it.

## Lessons learned (nRF52840 bare metal, a board that belongs to something else)

- Borrowed board (a Meshtastic node): stop its user service, export config and `--info`, enter the UF2 bootloader and copy `CURRENT.UF2` + `INFO_UF2.TXT`, and check what `CURRENT.UF2` covers (here `0x1000`-`0xEA000`, family `0x28860057`, so it cannot be copied back as is). Compare its application range with the official release block for block, then build a family-`0xADA52840` restore UF2 of the exact app region from it. Checksum all of it before flashing anything.
- Keep the image inside the application region (`0x27000`-`0xED000` with S140 7.3.0 and Adafruit's bootloader); have the UF2 writer refuse anything outside. Never call `embassy_nrf::init`, because it writes UICR. Afterwards, re-read `CURRENT.UF2` to prove the SoftDevice is byte-identical. For the restore, diff `--export-config` (identical means no re-import is needed) and the `--info` sections other than the node list.
- Every exit should lead to the bootloader: a panic, a fault, a lockup, or a 1200-baud touch. Use a 300-baud touch to restart for the next run. Give UsageFault, BusFault and MemManage a priority below USB so they can report pc/cfsr before resetting.
- ModemManager opens new CDC-ACM ports and eats output or drops the run. Start runs only on an explicit host token (`rind-bench:go`) sent after a settle delay, and fail any run whose first record isn't `start`. Use `sync -f <drive>`, not a global `sync`, after copying a UF2, since a busy disk elsewhere stalls the global one for minutes.
- rind on `thumbv7em`: patch it to build `no_std` (patches/0003), with the portable backend for M-profile. Getting ring 0.17 onto bare metal takes three things:
  - an `AsmTarget` for os `none`, minus the NEON-in-ARM-state `x25519-asm-arm.S` and `poly1305_arm_asm.S`, with the NEON Poly1305 cfg'd out
  - `less-safe-getrandom-custom-or-rdrand`
  - a getrandom 0.2 custom backend
- getrandom 0.4's custom backend symbol is still `__getrandom_v03_custom`.
- The p256 image (with libcrux-ecdh pulling in X25519) overflowed 792 KB at codegen-units 16. Fat LTO fixed it; apply it to every image on that board so they're built alike, and disclose it.
- The nRF RNG with bias correction takes about 170 µs per byte, so rows that draw randomness inside the timed loop (ML-KEM keygen/encaps) can be RNG-bound. Say so in the gap reason until it's measured.
