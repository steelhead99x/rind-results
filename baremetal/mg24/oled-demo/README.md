# EFR32MG24 OLED demo

Cycles three 128x64 frames on the SSD1306 (I2C 0x3C, I2C1 on PC4/PC5), 4 s each:

1. moq.dev — MoQ mark plus "moq.dev" / "Media over" / "QUIC"
2. rind — rind wordmark plus "pure-Rust crypto"
3. TLS 1.3 full handshake, X25519, rind vs aws-lc-rs at rind b6651423: Xeon 228 µs vs 299 µs, M1 Pro 284 µs vs 425 µs, Pi 5 1.57 ms vs 1.33 ms

The sketch probes 0x3C then 0x3D. Serial at 115200 prints `{"app":"oled-demo","oled":"0x3C","frame":N}`, or `"oled":"absent"` and blinks the LED.

Logo files are not in this tree (see SOURCES.md). Generate `oled_demo/frames.h` and the preview PNGs before compiling:

```
python3 tools/gen_frames.py
```

Needs cairosvg, Pillow, and DejaVu fonts. The script fetches the SVGs into a local `assets/` directory (gitignored) on first run.

Build and flash with arduino-cli (Silicon Labs core 5.0.0) and probe-rs 0.32.0. Use the EFR32MG24 board FQBN from that core; this writes only the app region, leaving the bootloader and last pages alone:

```
arduino-cli compile --fqbn <efr32mg24-board> --output-dir build oled_demo
probe-rs download --chip EFR32MG24B220F1536IM48 --binary-format hex --verify build/oled_demo.ino.hex
probe-rs reset --chip EFR32MG24B220F1536IM48
```

Restore the board's existing firmware with `../restore.sh` when the demo is done.
