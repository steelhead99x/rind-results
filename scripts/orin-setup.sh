#!/bin/bash
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# One-time setup on the Jetson, inside ~/rind-bench-orin only, at nice 19.
# Expects rind-<commit>.tar.gz, vendor/, the rust-1.98.1 aarch64 tarball and
# libssl-dev_<ver>_arm64.deb copied there by the controlling PC.
set -e
D="$HOME/rind-bench-orin"; cd "$D"
# Extract with -m: the board's clock may be unset, and source mtimes in its
# future would make cargo rebuild the workspace on every invocation.
rm -rf rind && mkdir rind && tar xzmf rind-*.tar.gz -C rind
if [ ! -x rust/bin/rustc ]; then
  rm -rf rust-inst && mkdir rust-inst && tar xJmf rust-1.98.1-aarch64-unknown-linux-gnu.tar.xz -C rust-inst
  nice -n 19 rust-inst/rust-1.98.1-aarch64-unknown-linux-gnu/install.sh --prefix="$D/rust" --components=rustc,cargo,rust-std-aarch64-unknown-linux-gnu --disable-ldconfig >/dev/null
  rm -rf rust-inst
fi
rm -rf sysroot && mkdir -p sysroot/lib && dpkg -x libssl-dev_*_arm64.deb sysroot
cp -n sysroot/usr/include/aarch64-linux-gnu/openssl/*.h sysroot/usr/include/openssl/
ln -sf /usr/lib/aarch64-linux-gnu/libssl.so.3 sysroot/lib/libssl.so
ln -sf /usr/lib/aarch64-linux-gnu/libcrypto.so.3 sysroot/lib/libcrypto.so
mkdir -p cargo-home
printf '[source.crates-io]\nreplace-with = "vendored-sources"\n\n[source.vendored-sources]\ndirectory = "%s/vendor"\n' "$D" > cargo-home/config.toml
. "$D/orin-env.sh"
rustc -V; cargo -V
