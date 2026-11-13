#!/bin/bash
D=$HOME/rind-work; cd $D
export PATH=$HOME/.cargo/bin:$PATH OPENSSL_INCLUDE_DIR=$D/sysroot/usr/include OPENSSL_LIB_DIR=$D/sysroot/lib OPENSSL_NO_PKG_CONFIG=1
while [ -d $D/.bench.lock ]; do sleep 30; done
VARIANTS="e1-final" bash build-argon2.sh $D 3
for v in a0-before e1-final; do objdump -d --no-show-raw-insn bins/argon2-$v | awk '/argon24neon8compress>:/{p=1} p&&/^$/{exit} p' | awk '{print $3}' > /tmp/$v-neon.ops; done
echo "neon opcode stream e1 vs a0: $(cmp -s /tmp/e1-final-neon.ops /tmp/a0-before-neon.ops && echo SAME || echo differ) ($(wc -l < /tmp/e1-final-neon.ops) lines)"
until mkdir $D/.bench.lock 2>/dev/null; do sleep 30; done; echo "argon2-agent (argon2 final confirm pi5) $(date)" > $D/.bench.lock/owner
echo "start $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
PIN="taskset -c 3" VARIANTS="a0-before e1-final" ./run-argon2.sh $D/bins $D/out-argon2-pi3 5 neon
echo "end $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
rm -rf $D/.bench.lock
python3 crit-parse.py $D/out-argon2-pi3 > $D/argon2-pi3-results.md
export CARGO_TARGET_DIR=$D/target-test
while [ -d $D/.bench.lock ]; do sleep 30; done
rm -rf $D/test-src && mkdir $D/test-src && tar xmf /tmp/t-final.tar -C $D/test-src && tar xzmf /tmp/thirdparty-vectors.tgz -C $D/test-src 2>/dev/null
cd $D/test-src && echo "== tests final $(date)"
nice -n 10 cargo test --jobs 4 --release -p rind --lib 2>&1 | grep -E "test result|FAILED|panicked"
nice -n 10 cargo test --jobs 4 --release -p rind --test crosschecks 2>&1 | grep -E "test result|FAILED|panicked"
nice -n 10 cargo test --jobs 4 -p rind --lib argon2 2>&1 | grep -E "test result|FAILED"
nice -n 10 cargo test --jobs 4 -p rind --features wipe --test zeroing --test heap_scratch 2>&1 | grep -E "test result|FAILED"
nice -n 10 python3 admin/ct-vartime aarch64-unknown-linux-gnu > $D/ct-vartime.log 2>&1; echo "ct-vartime rc=$?"; tail -3 $D/ct-vartime.log
echo "end tests $(date)"
