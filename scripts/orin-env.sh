#!/bin/bash
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# Sourced on the Jetson: everything lives in ~/rind-bench-orin, nothing is
# installed system-wide and no sudo is used. The board is offline, so rustc is
# the standalone 1.98.1 release, crates come from a `cargo vendor` copy, and
# OpenSSL's headers come from the matching libssl-dev .deb unpacked locally
# (the binaries link the system's libssl3 at runtime).
D="$HOME/rind-bench-orin"
export PATH="$D/rust/bin:$PATH" CARGO_HOME="$D/cargo-home" CARGO_NET_OFFLINE=true
ML=aarch64-linux-gnu
export OPENSSL_INCLUDE_DIR="$D/sysroot/usr/include" OPENSSL_LIB_DIR="$D/sysroot/lib" OPENSSL_NO_PKG_CONFIG=1
export IN_RIND_DEV_SHELL=1 CARGO_TARGET_DIR="$D/target"
