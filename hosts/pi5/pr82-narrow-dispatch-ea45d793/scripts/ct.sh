#!/bin/bash
cd ~/rind-pr82-hw
. ~/rind-pr82-hw-tools/env.sh
export RUSTUP_TOOLCHAIN=1.98.1 CARGO_TARGET_DIR=~/rind-pr82-hw/target
export RIND_TEST_ARM_CRYPTO_EXT=1 RIND_TEST_ARM_SHA3=0 RIND_TEST_ARM_SVE2=0
L=~/rind-pr82-hw/logs
run() { n=$1; shift; s=$(date +%s); echo "START $(date -Is)" > $L/$n.log; env "$@" >> $L/$n.log 2>&1; rc=$?; echo "EXIT $rc DURATION $(( $(date +%s)-s ))s END $(date -Is)" >> $L/$n.log; }
run ctgrind nice admin/ctgrind --release --locked --no-fail-fast
run ctgrind-fips nice admin/ctgrind fips --release --locked --no-fail-fast
# Extra: rind unit suite under memcheck with the wide (_alt) schedules forced, since this core is narrow.
run ctgrind-forced-wide RUSTFLAGS="--cfg rind_cpu_disable" RIND_CPU_FORCE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=1 VALGRIND_BUG_494162=1 CARGO_PROFILE_RELEASE_DEBUG=line-tables-only CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="$PWD/admin/ctgrind --run" nice cargo test -p rind --lib --features __ctgrind --release --locked --no-fail-fast
echo ALLDONE > $L/ct.done
