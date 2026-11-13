#!/bin/bash
# q14: charon sysroots, clone fix/p256-table-align, translate-only (with wide_mux patch locally)
D=$HOME/rind-work; L=$D/lane-pi5; P=$L/prove-tools
w() { while [ -e $D/.bench.lock ] && ! grep -q pi5-lane $D/.bench.lock 2>/dev/null; do sleep 20; done; }
set -x
NB=$HOME/.rustup/toolchains/nightly-2026-09-17-aarch64-unknown-linux-gnu/bin
if [ ! -d $P/charon-sysroots ]; then
  w
  export HOME2=$HOME; T=$P/sysbuild; mkdir -p $T/cargo
  SRC=$($NB/rustc --print sysroot)/lib/rustlib/src/rust/library/vendor
  printf "[source.crates-io]\nreplace-with = \"vendored-sources\"\n[source.vendored-sources]\ndirectory = \"%s\"\n" "$SRC" > $T/cargo/config.toml
  for t in x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu; do
    sr=$(env PATH=$NB:$PATH CARGO_HOME=$T/cargo CARGO_NET_OFFLINE=true MIRI_SYSROOT=$T/sysroot nice $NB/cargo miri setup --target=$t --print-sysroot) || exit 1
  done
  mkdir -p $P/charon-sysroots && cp -a $sr/. $P/charon-sysroots/
fi
cd $P && rm -rf align && git clone -q align.bundle -b fix/p256-table-align align && cd align && git am -q ../wide-mux-0001.patch && git log --oneline -3
. $L/prove-env.sh
w
PROVE_TRANSLATE_ONLY=1 nice admin/prove lean; echo "translate rc=$?"
