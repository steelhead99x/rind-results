#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# Build the ESP32-S3 sha2, hmac, and hkdf images at two rind commits.
# Does not flash. See esp32/pr69/README.md.

set -euo pipefail

# Another agent may hold the shared rind index.lock for a moment; retry.
git() { local i rc=0; for i in 1 2 3 4 5 6 7 8; do command git "$@" && return 0; rc=$?; sleep 4; done; return $rc; }

root=$HOME/tmp/tt/a-brief/hw2
rind=$root/rind
esp=$root/esp32
out=$HOME/tmp/tt/a-brief/images
status=$out/patch-status.txt
fork_url=https://github.com/steelhead99x/rind.git

# Fork main, then the PR head. Short names are the output directories.


mkdir -p "$out" "$root/.tools"
: >"$status"

say() { printf '%s\n' "$*" | tee -a "$status"; }

if [[ ! -e $rind/.git ]]; then
  git -C "$root" submodule update --init -- rind
fi
if ! git -C "$rind" remote get-url fork >/dev/null 2>&1; then
  git -C "$rind" remote add fork "$fork_url"
fi
git -C "$rind" fetch --no-tags fork main "+refs/pull/94/head:refs/remotes/fork/pr94" "+refs/pull/87/head:refs/remotes/fork/pr87" "+refs/pull/85/head:refs/remotes/fork/pr85"

rind_orig=$(git -C "$rind" rev-parse HEAD)
restore_rind() {
  git -C "$rind" checkout --detach --quiet "$rind_orig" || true
  git -C "$rind" reset --hard --quiet "$rind_orig" || true
  git -C "$rind" clean -fdq || true
}
trap restore_rind EXIT

# Same unsigned identity cargo xtask apply-patches uses, so the fork commit
# does not depend on who runs the script or on commit.gpgsign.
git_am() {
  local patch=$1
  shift
  env -u GIT_AUTHOR_NAME -u GIT_AUTHOR_EMAIL -u GIT_AUTHOR_DATE \
    -u GIT_COMMITTER_NAME -u GIT_COMMITTER_EMAIL -u GIT_COMMITTER_DATE \
    -u GIT_CONFIG_COUNT -u GIT_CONFIG_PARAMETERS \
    GIT_EDITOR=true \
    git -C "$rind" \
      -c user.name=steelhead99x \
      -c user.email=steelhead99x@users.noreply.github.com \
      -c commit.gpgsign=false \
      -c tag.gpgsign=false \
      -c i18n.commitEncoding=utf-8 \
      -c am.signoff=false \
      -c format.signOff=false \
      -c am.messageid=false \
      am --no-gpg-sign --no-signoff --no-verify --whitespace=nowarn \
      --committer-date-is-author-date "$@" "$patch"
}

apply_series() {
  local sha=$1
  say "commit $sha"
  git -C "$rind" reset --hard --quiet
  git -C "$rind" checkout --detach --quiet "$sha"
  git -C "$rind" reset --hard --quiet "$sha"
  git -C "$rind" clean -fdq
  if [[ ${NOPATCH:-0} == 1 ]]; then say "  no bench patches (head already carries them)"; return 0; fi
  local patch base plain three
  local patches=("$root"/patches/000[1-2]-*.patch)
  if [[ ${NOPATCH:-0} == 2 ]]; then patches=(); say "  bench patches 0001/0002 skipped (head carries them); dropvg only"; fi
  for patch in "${patches[@]}"; do
    base=$(basename "$patch")
    plain=$(mktemp)
    three=$(mktemp)
    if git -C "$rind" apply --check "$patch" >"$plain" 2>&1; then
      git_am "$patch"
      say "  $base clean $(git -C "$rind" rev-parse HEAD)"
    elif git -C "$rind" apply --3way --check "$patch" >"$three" 2>&1; then
      # Plain `git am` is what xtask runs. On these two commits 0003's
      # cpu.rs hunk does not match: upstream inserted MissingFeature::name
      # between the Display impl and the Error impl. The blob in the patch
      # is still in this history, and `git am -3` auto-merges it.
      git_am "$patch" -3
      say "  $base plain-apply-failed 3way-merged $(git -C "$rind" rev-parse HEAD)"
      say "  $base plain apply said:"
      grep -E 'error:|patch failed' "$plain" | sed 's/^/    /' | tee -a "$status" || true
    else
      say "  $base CONFLICT"
      cat "$plain" "$three" | tee -a "$status"
      rm -f "$plain" "$three"
      git -C "$rind" am --abort >/dev/null 2>&1 || true
      exit 1
    fi
    rm -f "$plain" "$three"
  done
  python3 "$root/esp32/pr94/dropvg.py" "$rind"
  git -C "$rind" -c user.name=steelhead99x -c user.email=steelhead99x@users.noreply.github.com -c commit.gpgsign=false commit -qam "bench-only: drop verified-garbage rows (no Xtensa support)"
  git -C "$rind" format-patch -1 --stdout > "$out/bench-only-dropvg-head-${sha:0:8}.patch"
  say "  dropvg $(git -C "$rind" rev-parse HEAD)"
  say "  fork_commit $(git -C "$rind" rev-parse HEAD)"
}

vendor_ring() {
  local dest=$root/.tools/ring-esp
  if [[ -f $dest/Cargo.toml && -f $dest/crypto/poly1305/poly1305.c ]]; then
    if grep -q POLY1305_STATE_ALIGN "$dest/crypto/poly1305/poly1305.c"; then
      say "ring 0.17.14 already patched in .tools/ring-esp"
      return
    fi
  fi
  local tmp
  tmp=$(mktemp -d)
  curl -fsSL -o "$tmp/ring.crate" https://static.crates.io/crates/ring/ring-0.17.14.crate
  tar -xzf "$tmp/ring.crate" -C "$tmp"
  rm -rf "$dest"
  mv "$tmp/ring-0.17.14" "$dest"
  rm -rf "$tmp"
  # Same patch `cargo xtask vendor-ring` applies. The file has a note before
  # the diff; `patch` skips that preamble.
  patch -d "$dest" -p1 <"$root/patches/ring-0.17.14-xtensa-poly1305-align.patch"
  say "ring 0.17.14 patched into .tools/ring-esp"
}

# esp-idf-sys installs the ESP-IDF toolchain from its build script. ring's C
# files need xtensa-esp32s3-elf-gcc. Building this crate first, into the same
# target directory and .embuild tree the firmware uses, finishes that install
# before ring's build script runs.
build_esp_idf_sys() {
  local helper=$root/.tools/idf-first
  mkdir -p "$helper/src" "$helper/.cargo"
  cat >"$helper/Cargo.toml" <<'EOF'
[package]
name = "idf-first"
version = "0.0.0"
edition = "2021"
publish = false

[dependencies]
esp-idf-sys = { version = "=0.38.1", default-features = false, features = ["std", "binstart", "native"] }
EOF
  printf '// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0\n' >"$helper/src/lib.rs"
  cat >"$helper/.cargo/config.toml" <<EOF
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
[build]
target = "xtensa-esp32s3-espidf"

[target.xtensa-esp32s3-espidf]
linker = "ldproxy"
rustflags = ["--cfg", "espidf_time64"]

[unstable]
build-std = ["std", "panic_abort"]

[env]
MCU = "esp32s3"
ESP_IDF_VERSION = "v5.5.1"
ESP_IDF_SDKCONFIG_DEFAULTS = "sdkconfig.defaults"
ESP_IDF_TOOLS_INSTALL_DIR = "workspace"
CC_xtensa_esp32s3_espidf = "xtensa-esp32s3-elf-gcc"
AR_xtensa_esp32s3_espidf = "xtensa-esp32s3-elf-ar"
CARGO_WORKSPACE_DIR = { value = "$esp", force = true }
EOF
  say "building esp-idf-sys before ring"
  (
    cd "$helper"
    export CARGO_WORKSPACE_DIR=$esp
    export CARGO_TARGET_DIR=$esp/target
    unset IDF_PATH || true
    cargo build --release
  )
}



build_one() { # label bench extra-feature
  local label=$1 bench=$2 extra=${3:-}
  local dest=$out/$label; mkdir -p "$dest"
  local feats=$bench
  cp "$esp/Cargo.toml" "$esp/Cargo.toml.bak"
  if [[ -n $extra ]]; then
    sed -i 's/^sha2 = \[\]$/sha2 = []\nsha512-flash = ["rind\/esp32s3-sha512-flash"]/' "$esp/Cargo.toml"
    feats="$bench,$extra"
  fi
  say "build $label features=$feats"
  ( cd "$esp"; export RIND_BENCH_PAD=0; cargo build --release --features "$feats" ) || { cp "$esp/Cargo.toml.bak" "$esp/Cargo.toml"; say "BUILD FAIL $label"; return 1; }
  cp "$esp/Cargo.toml.bak" "$esp/Cargo.toml"
  local elf=$esp/target/xtensa-esp32s3-espidf/release/rind-bench-esp32
  cp "$elf" "$dest/$bench.elf"
  say "  file $(stat -c '%s' "$dest/$bench.elf") bytes  sha256 $(sha256sum "$dest/$bench.elf" | awk '{print $1}')"
  xtensa-esp32s3-elf-size "$dest/$bench.elf" | tee -a "$status"
}

say "a-brief ESP32-S3 images; this script does not flash"
vendor_ring
set +u; . "$esp/env.sh"; set -u
rustc +esp --version | tee -a "$status"
build_esp_idf_sys

# usage: build.sh   (all variants)
for spec in "$@"; do
  IFS=: read -r label sha benches extra nopatch <<<"$spec"
  NOPATCH=${nopatch:-0} apply_series "$sha"
  if [[ $benches == *poly1305* ]]; then bash "$esp/poly-expose.patch.sh" "$rind"; say "  poly1305 shim applied (working tree only)"; fi
  for b in ${benches//,/ }; do build_one "$label" "$b" "$extra" || true; done
  say "  rind_HEAD $(git -C "$rind" rev-parse HEAD)"
done
say "done."
