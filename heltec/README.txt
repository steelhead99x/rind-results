Heltec (ESP32-S3 rev v0.2, 8MB embedded flash, MAC 48:ca:43:3c:cc:60) on /dev/ttyUSB0 (CP2102), 2026-11-11 PT.
rind: pristine `git archive 99b509a6` copy in rind-99b509a6/ (NOT ~/rind-bench/rind, whose tree carries local patches).
Target xtensa-esp32s3-espidf, esp toolchain rustc 1.97.0-nightly (esp), ESP-IDF v5.5.1, espflash 4.7.0 (~/.local/bin).
build-pristine.log : plain build fails, E0658 asm! in rind/src/low/generic/md5.rs:23.
build-gateflag.log : builds with fw/Cargo.toml [profile.release.package.rind] rustflags -Zcrate-attr=feature(asm_experimental_arch); rind source unchanged.
serial-run1.log / serial-run2.log : device output. Timer: esp_timer_get_time (us); cycles column = us x 240.
heltec-flash-backup-before-rind.bin : full 8MB dump of the original (Meshtastic) firmware.
Restore: espflash write-bin --port /dev/ttyUSB0 0x0 heltec-flash-backup-before-rind.bin
