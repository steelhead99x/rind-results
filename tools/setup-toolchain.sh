#!/bin/sh
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# Installs, without root, what building and flashing rind-hardware's ESP32-S3
# images needs, at the versions the published results used:
#   rustup-init 1.29.3, then the stable toolchain
#   espup 0.19.0 -> Rust "esp" 1.97.0.0 + Xtensa GCC esp-14.2.0_20241119
#   ldproxy 0.3.5, espflash 4.7.0
# Usage: tools/setup-toolchain.sh RIND_HARDWARE_DIR
# It stays shell because it installs rustup itself; everything after it is
# `cargo xtask` (rind-hardware and rind-results).
# Tools land in RIND_HARDWARE_DIR/.tools; ESP-IDF v5.5.1 itself is fetched by
# the first firmware build into esp32/.embuild.
# Every downloaded binary is checked against tools/checksums.sha256 before it
# is executed or marked executable. A mismatch deletes the file and exits.
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: tools/setup-toolchain.sh RIND_HARDWARE_DIR" >&2
    exit 1
fi

script_dir=$(CDPATH='' cd -- "$(dirname "$0")" && pwd)
sums=$script_dir/checksums.sha256
bench=$(cd "$1" && pwd)
tools=$bench/.tools
mkdir -p "$tools"
arch=$(uname -m)

case "$arch" in
    x86_64|aarch64) ;;
    *)
        echo "setup-toolchain: no pinned checksum for arch $arch" >&2
        exit 1
        ;;
esac

if ! command -v sha256sum >/dev/null 2>&1; then
    echo "setup-toolchain: sha256sum is required" >&2
    exit 1
fi

# verify_sha256 BASENAME CHECKSUM_NAME
# BASENAME is the file already in $tools. CHECKSUM_NAME is the second field
# in tools/checksums.sha256.
verify_sha256() {
    base=$1
    name=$2
    hash=$(awk -v n="$name" '$2 == n { print $1; exit }' "$sums")
    if [ -z "$hash" ]; then
        echo "setup-toolchain: no checksum recorded for $name" >&2
        rm -f "$tools/$base"
        exit 1
    fi
    if ! (cd "$tools" && printf '%s  %s\n' "$hash" "$base" | sha256sum -c -); then
        rm -f "$tools/$base"
        exit 1
    fi
}

rustup_triple=${arch}-unknown-linux-gnu
rustup_version=1.29.3

if ! command -v rustup >/dev/null 2>&1 && [ ! -x "$HOME/.cargo/bin/rustup" ]; then
    curl --proto '=https' --tlsv1.2 -fsSL \
        -o "$tools/rustup-init" \
        "https://static.rust-lang.org/rustup/archive/${rustup_version}/${rustup_triple}/rustup-init"
    verify_sha256 rustup-init "rustup-init-${rustup_version}-${rustup_triple}"
    chmod +x "$tools/rustup-init"
    "$tools/rustup-init" -y --profile minimal
fi
export PATH="$HOME/.cargo/bin:$PATH"
rustup toolchain install stable --profile minimal --component rustfmt,clippy

if [ ! -x "$tools/espup" ]; then
    espup_name=espup-${arch}-unknown-linux-gnu
    curl --proto '=https' --tlsv1.2 -fsSL \
        -o "$tools/espup" \
        "https://github.com/esp-rs/espup/releases/download/v0.19.0/${espup_name}"
    verify_sha256 espup "espup-0.19.0-${arch}-unknown-linux-gnu"
    chmod +x "$tools/espup"
fi
"$tools/espup" install --targets esp32s3 --toolchain-version 1.97.0.0 --export-file "$tools/export-esp.sh"

cargo install --locked ldproxy --version 0.3.5

if [ ! -x "$tools/espflash" ]; then
    espflash_name=espflash-${arch}-unknown-linux-musl.zip
    curl --proto '=https' --tlsv1.2 -fsSL \
        -o "$tools/espflash.zip" \
        "https://github.com/esp-rs/espflash/releases/download/v4.7.0/${espflash_name}"
    verify_sha256 espflash.zip "espflash-4.7.0-${arch}-unknown-linux-musl.zip"
    (cd "$tools" && unzip -o -q espflash.zip && rm espflash.zip)
    chmod +x "$tools/espflash"
fi

echo "done: in $bench, cargo xtask apply-patches && cargo xtask vendor-ring && cargo xtask build-images esp"
echo "the board's USB-Serial/JTAG port needs your user in the dialout (or uucp) group"
