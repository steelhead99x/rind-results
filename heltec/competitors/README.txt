Heltec ESP32-S3 (rev v0.2, /dev/ttyUSB0), 240 MHz, xtensa-esp32s3-espidf, ESP-IDF v5.5.1, esp rustc 1.97.0-nightly.
Harness: fw/src/main.rs, one image per library (images/*.elf): the combined image fails to link (build-combined-call8-overflow.log).
Timer esp_timer_get_time (us), warm-up iters/10, same iterations for every library. Inputs from rind-bench (zero key/nonce, 32 B AAD).
rind: pristine 99b509a6 + LABELED WORKAROUND -Zcrate-attr=feature(asm_experimental_arch) for the rind crate only (fw/Cargo.toml).
ring: uses ring's own feature less-safe-getrandom-espidf. Its ChaCha20-Poly1305 resets the chip (RTCWDT_RTC_RST), so it runs last.
Results: report-table.md (from report.py over logs/serial-<lib>-run{1,2}.log).
Build failures: build-probe-awslc.log, build-probe-openssl.log, build-probe-openssl-vendored.log, build-probe-graviola.log.
SHA-512: logs/serial-rustcrypto-sha2compact-run*.log, logs/serial-rind+rustcrypto-icache32-run*.log, sha512_update.disasm.
logs/discarded/README.txt explains the mislabeled and interrupted captures.
