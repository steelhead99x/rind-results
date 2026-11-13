#!/bin/bash
# pmu-build.sh WORKDIR JOBS: builds argon2-pmu against each variant tarball in /tmp (VARIANTS env).
export PATH=$HOME/.cargo/bin:$PATH
D=$1; J=$2; mkdir -p $D/pmu-bins
while [ -d $HOME/rind-work/.bench.lock ]; do sleep 30; done
export CARGO_TARGET_DIR=$D/target-pmu RUSTFLAGS="${PMU_RUSTFLAGS:---cfg rind_cpu_disable}"
for v in $VARIANTS; do
  rm -rf $D/argon2-src && mkdir -p $D/argon2-src && tar xzmf /tmp/$v.tar.gz -C $D/argon2-src
  touch $D/argon2-pmu/src/main.rs
  (cd $D/argon2-pmu && nice -n 10 cargo build --offline --release --jobs $J 2>&1 | grep -E "^(error|warning: unused)|Finished" ) || exit 1
  cp $CARGO_TARGET_DIR/release/argon2-pmu $D/pmu-bins/pmu-$v
done
md5sum $D/pmu-bins/*
