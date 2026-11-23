#!/bin/bash
cd ~/rind-pr82-hw
source ~/.cargo/env
export CARGO_BUILD_JOBS=3 CARGO_TARGET_DIR=~/rind-pr82-hw/target
L=~/rind-pr82-hw/logs
cargo +1.98.1 --version > $L/toolchain.txt; rustc +1.98.1 -vV >> $L/toolchain.txt
run() { n=$1; shift; s=$(date +%s); echo "START $(date -Is)" > $L/$n.log; env "$@" >> $L/$n.log 2>&1; rc=$?; echo "EXIT $rc DURATION $(( $(date +%s)-s ))s END $(date -Is)" >> $L/$n.log; }
run run1 nice cargo +1.98.1 test -p rind --lib --locked -- low::aarch64::
run run2 RIND_CPU_FORCE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=1 nice cargo +1.98.1 test -p rind --lib --locked -- low::aarch64::cpu::tests::detects_the_expected_cpu low::aarch64::schedules low::aarch64::mont
run run3 RIND_CPU_DISABLE_wide_mul=1 RIND_TEST_ARM_WIDE_MUL=0 nice cargo +1.98.1 test -p rind --lib --locked -- low::aarch64::
echo ALLDONE > $L/runs.done
