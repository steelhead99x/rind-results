#!/bin/bash
# lane-test.sh SRC: native rind tests on the Pi, in both multiplier modes. Waits for no bench lock before each step.
D=$HOME/rind-work; S=$1
export PATH=$HOME/.cargo/bin:$PATH OPENSSL_INCLUDE_DIR=$D/sysroot/usr/include OPENSSL_LIB_DIR=$D/sysroot/lib OPENSSL_NO_PKG_CONFIG=1 RUSTFLAGS="--cfg rind_cpu_disable" CARGO_TARGET_DIR=$D/lane-pi5/target-$S
cd $D/lane-pi5/$S || exit 1
w() { while [ -d $D/.bench.lock ]; do sleep 30; done; }
w; echo "build $(date) load: $(cat /proc/loadavg)"
nice -n 19 cargo test -p rind --locked --release --jobs 3 --no-run 2>&1 | grep -v "^ *Compiling" | tail -5
nice -n 19 cargo test -p rind --features wipe --test zeroing --test heap_scratch --locked --release --jobs 3 --no-run 2>&1 | tail -2
for mode in narrow wide; do
  if [ $mode = wide ]; then E="env RIND_CPU_FORCE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=1"; else E="env RIND_TEST_ARM_WIDE_MUL=0"; fi
  w; echo "test $mode $(date) load: $(cat /proc/loadavg)"
  $E nice -n 19 cargo test -p rind --locked --release --jobs 3 -- --test-threads 3 > test-$mode.log 2>&1; echo "  rc=$?"
  grep -E "^test result|FAILED|panicked" test-$mode.log | sort | uniq -c | sort -rn | head -8
  $E nice -n 19 cargo test -p rind --features wipe --test zeroing --test heap_scratch --locked --release --jobs 3 > test-wipe-$mode.log 2>&1; echo "  wipe rc=$?"
  grep -E "^test result" test-wipe-$mode.log
done
echo "end $(date)"
w; echo "clippy $(date)"
nice -n 19 cargo clippy -p rind --all-targets --locked --release --jobs 3 -- --deny warnings 2>&1 | grep -E "^(warning|error)|Finished" | head -20
echo "clippy end $(date)"
