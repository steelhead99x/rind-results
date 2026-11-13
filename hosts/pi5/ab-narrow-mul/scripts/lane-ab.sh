#!/bin/bash
# lane-ab.sh OUT ROUNDS CORE FILTER SPEC... ; SPEC = label:bindir:ENV (ENV is VAR=val or -).
# Per round (bench lock held only for that round) runs every binary in each spec's bindir; spec order
# rotates each round. CRITERION_HOME=OUT/<label>-r<N>.
D=$HOME/rind-work; OUT=$1; R=$2; C=$3; F=$4; shift 4; S=("$@"); N=${#S[@]}
mkdir -p $OUT
for r in $(seq 1 $R); do
  until mkdir $D/.bench.lock 2>/dev/null; do sleep 30; done
  echo "pi5-lane $(date)" > $D/.bench.lock/owner
  for i in $(seq 0 $((N-1))); do sp=${S[$(( (i + r) % N ))]}
    l=${sp%%:*}; rest=${sp#*:}; b=${rest%%:*}; e=${rest#*:}
    if [ "$e" = - ]; then E=env; else E="env $e"; fi
    echo "r$r $l $(date +%T) load: $(cat /proc/loadavg)"
    for f in $b/*; do
      CRITERION_HOME=$OUT/$l-r$r $E taskset -c $C $f --bench --noplot --warm-up-time 1 --measurement-time 3 "$F" > /dev/null 2>>$OUT/stderr.log
    done
  done
  rm -rf $D/.bench.lock
  echo "r$r done $(date +%T) load: $(cat /proc/loadavg)"
  sleep 20
done
echo "end $(date)"
