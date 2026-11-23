#!/bin/bash
cd ~/rind-pr82-hw
. ~/rind-pr82-hw-tools/env.sh
export RUSTUP_TOOLCHAIN=1.98.1
L=~/rind-pr82-hw/logs
s=$(date +%s); echo "START $(date -Is)" > $L/prove-bytes.log
nice admin/prove bytes >> $L/prove-bytes.log 2>&1; rc=$?
echo "EXIT $rc DURATION $(( $(date +%s)-s ))s END $(date -Is)" >> $L/prove-bytes.log
echo ALLDONE > $L/prove.done
