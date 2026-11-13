#!/bin/sh
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# Installs, without root, what building and flashing rind-hardware's ESP32-S3
# images needs, at the versions the published results used:
#   rustup (stable), espup 0.19.0 -> Rust "esp" 1.97.0.0 + Xtensa GCC
#   esp-14.2.0_20241119, ldproxy 0.3.5, espflash 4.7.0.
# Usage: tools/setup-toolchain.sh RIND_HARDWARE_DIR
# It stays shell because it installs rustup itself; everything after it is
# `cargo xtask` (rind-hardware and rind-results).
# Tools land in RIND_HARDWARE_DIR/.tools; ESP-IDF v5.5.1 itself is fetched by
# the first firmware build into esp32/.embuild.
set -eu
bench=$(cd "$1" && pwd)
tools=$bench/.tools
mkdir -p "$tools"
arch=$(uname -m)

if ! command -v rustup >/dev/null 2>&1 && [ ! -x "$HOME/.cargo/bin/rustup" ]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
fi
export PATH=$HOME/.cargo/bin:$PATH
rustup toolchain install stable --profile minimal --component rustfmt,clippy

if [ ! -x "$tools/espup" ]; then
    curl -sSfL -o "$tools/espup" \
        "https://github.com/esp-rs/espup/releases/download/v0.19.0/espup-$arch-unknown-linux-gnu"
    chmod +x "$tools/espup"
fi
"$tools/espup" install --targets esp32s3 --toolchain-version 1.97.0.0 --export-file "$tools/export-esp.sh"

cargo install --locked ldproxy --version 0.3.5

if [ ! -x "$tools/espflash" ]; then
    curl -sSfL -o "$tools/espflash.zip" \
        "https://github.com/esp-rs/espflash/releases/download/v4.7.0/espflash-$arch-unknown-linux-musl.zip"
    (cd "$tools" && unzip -o -q espflash.zip && chmod +x espflash && rm espflash.zip)
fi

echo "done: in $bench, cargo xtask apply-patches && cargo xtask vendor-ring && cargo xtask build-images esp"
echo "the board's USB-Serial/JTAG port needs your user in the dialout (or uucp) group"
