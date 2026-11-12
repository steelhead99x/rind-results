#!/bin/bash
# g1-pi.sh: build g1-final (criterion + pmu), criterion A/B e1-final vs g1-final under the lock, then native tests.
D=$HOME/rind-work; cd $D
export PATH=$HOME/.cargo/bin:$PATH OPENSSL_INCLUDE_DIR=$D/sysroot/usr/include OPENSSL_LIB_DIR=$D/sysroot/lib OPENSSL_NO_PKG_CONFIG=1
VARIANTS="h2-hybrid-neon" bash build-argon2.sh $D 3
VARIANTS="h2-hybrid-neon" ./pmu-build.sh $D 3
objdump -d --no-show-raw-insn bins/argon2-h2-hybrid-neon | awk '/argon24neon8compress>:/{p=1} p&&/^$/{exit} p' > /tmp/g1.dis
echo "h2 neon::compress lines=$(wc -l </tmp/g1.dis) sp=$(grep -cE '\[sp' /tmp/g1.dis) umull=$(grep -cE 'umull' /tmp/g1.dis) umlal=$(grep -c umlal /tmp/g1.dis)"
until mkdir $D/.bench.lock 2>/dev/null; do sleep 30; done; echo "argon2-agent (argon2 h2 criterion A/B pi5) $(date)" > $D/.bench.lock/owner
echo "start $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
PIN="taskset -c 3" VARIANTS="g1-final h2-hybrid-neon" ./run-argon2.sh $D/bins $D/out-argon2-pi6 5 neon
echo "mid $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
PIN="taskset -c 3" ./pmu-ab.sh.nolock $D/pmu-pi-h2.tsv 8 5 "g1-final h2-hybrid-neon" 1 16 64
echo "end $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
rm -rf $D/.bench.lock
python3 crit-parse.py $D/out-argon2-pi6 > $D/argon2-pi6-results.md
export CARGO_TARGET_DIR=$D/target-test
while [ -d $D/.bench.lock ]; do sleep 30; done
rm -rf $D/test-src && mkdir $D/test-src && tar xmf /tmp/t-h2.tar -C $D/test-src && tar xzmf /tmp/thirdparty-vectors.tgz -C $D/test-src 2>/dev/null
cd $D/test-src && echo "== tests h2 $(date)"
nice -n 10 cargo test --jobs 3 --release -p rind --lib 2>&1 | grep -E "test result|FAILED|panicked"
nice -n 10 cargo test --jobs 3 --release -p rind --test crosschecks 2>&1 | grep -E "test result|FAILED|panicked"
nice -n 10 cargo test --jobs 3 -p rind --lib argon2 2>&1 | grep -E "test result|FAILED"
nice -n 10 cargo test --jobs 3 -p rind --features wipe --test zeroing --test heap_scratch 2>&1 | grep -E "test result|FAILED"
nice -n 10 python3 admin/ct-vartime aarch64-unknown-linux-gnu > $D/ct-vartime-h2.log 2>&1; echo "ct-vartime rc=$?"; tail -3 $D/ct-vartime-h2.log
echo "end tests $(date)"
