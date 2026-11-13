#!/bin/bash
# lane-build.sh SRC JOBS BENCH...: build bench binaries on the Pi, waiting for no bench lock.
D=$HOME/rind-work; S=$1; J=$2; shift 2
while [ -d $D/.bench.lock ]; do sleep 30; done
cd $D/lane-pi5/$S/rind-bench || exit 1
export PATH=$HOME/.cargo/bin:$PATH OPENSSL_INCLUDE_DIR=$D/sysroot/usr/include OPENSSL_LIB_DIR=$D/sysroot/lib OPENSSL_NO_PKG_CONFIG=1 RUSTFLAGS="--cfg rind_cpu_disable" CARGO_TARGET_DIR=$D/lane-pi5/target-$S
echo "start $(date) load: $(cat /proc/loadavg)"
B=""; for b in "$@"; do B="$B --bench $b"; done
nice -n 19 cargo bench --jobs $J --no-run $B 2>&1 | grep -v "^ *Compiling"
echo "end $(date) rc=${PIPESTATUS[0]}"
