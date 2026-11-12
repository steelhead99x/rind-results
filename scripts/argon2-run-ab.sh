#!/bin/bash
# run-argon2.sh BINDIR OUT ROUNDS MODES...: interleaved rounds of each variant and fill.
B=$1; OUT=$2; R=$3; shift 3; MODES="$*"
rm -rf $OUT; mkdir -p $OUT
for r in $(seq 1 $R); do for v in ${VARIANTS:-a0-before a1-umlal a2-prefetch}; do for m in $MODES; do
  if [ $m = neon ]; then E="env RIND_CPU_DISABLE_sha3=1"; else E=env; fi
  echo "r$r $v $m $(date +%T) $(uptime | sed 's/.*load/load/')"
  CRITERION_HOME=$OUT/$v-$m-r$r $E $PIN $B/argon2-$v --bench --noplot > /dev/null 2>>$OUT/stderr.log
done; done; done
echo "end $(date)"
