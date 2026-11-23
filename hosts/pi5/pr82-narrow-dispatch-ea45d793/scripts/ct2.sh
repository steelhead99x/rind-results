#!/bin/bash
cd ~/rind-pr82-hw
. ~/rind-pr82-hw-tools/env.sh
export RUSTUP_TOOLCHAIN=1.98.1 CARGO_TARGET_DIR=~/rind-pr82-hw/target-ct2 CARGO_BUILD_JOBS=2
export RIND_TEST_ARM_CRYPTO_EXT=1 RIND_TEST_ARM_SHA3=0 RIND_TEST_ARM_SVE2=0
export RUSTFLAGS="--cfg rind_cpu_disable" VALGRIND_BUG_494162=1 CARGO_PROFILE_RELEASE_DEBUG=line-tables-only
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="$PWD/admin/ctgrind --run"
L=~/rind-pr82-hw/logs
run() { n=$1; shift; s=$(date +%s); echo "START $(date -Is)" > $L/$n.log; env "$@" >> $L/$n.log 2>&1; rc=$?; echo "EXIT $rc DURATION $(( $(date +%s)-s ))s END $(date -Is)" >> $L/$n.log; }
run ctgrind-a64-native RIND_TEST_ARM_WIDE_MUL=0 nice cargo test -p rind --lib --features __ctgrind --release --locked --no-fail-fast -- low::aarch64::
run ctgrind-a64-forcewide RIND_CPU_FORCE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=1 nice cargo test -p rind --lib --features __ctgrind --release --locked --no-fail-fast -- low::aarch64::
echo ALLDONE > $L/ct2.done
