#!/bin/bash
# q15: lake build of the translated proofs (no SimdVectors: x86-only)
D=$HOME/rind-work; L=$D/lane-pi5
w() { while [ -e $D/.bench.lock ] && ! grep -q pi5-lane $D/.bench.lock 2>/dev/null; do sleep 20; done; }
. $L/prove-env.sh
cd $L/prove-tools/align/proofs && cat lean-toolchain
:
w; date; nice lake build ${TARGETS:-} 2>&1 | grep -v "^✔\|^⣿\|Replayed\|^info: \[" | tail -${TAILN:-200}; echo "build rc=${PIPESTATUS[0]}"; date
