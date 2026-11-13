#!/bin/bash
cd $HOME/rind-work/lane-pi5
L=$HOME/rind-work/.bench.lock
PERF="$PWD/perfdeb/root/usr/bin/perf"; export LD_LIBRARY_PATH=$PWD/perfdeb/root/usr/lib/aarch64-linux-gnu
mkdir -p perf10
EV="cycles:u,instructions:u,branch-misses:u,L1-icache-load-misses:u,L1-dcache-load-misses:u,dTLB-load-misses:u,iTLB-load-misses:u,stalled-cycles-frontend:u,stalled-cycles-backend:u"
for r in 1 2 3; do for v in narrow detect; do
  until mkdir $L 2>/dev/null; do sleep 30; done; echo "pi5-lane (series 2 perf $v r$r) $(date)" > $L/owner
  echo "# $v r$r $(date +%T) $(cut -d" " -f1-3 /proc/loadavg) $(vcgencmd measure_temp) $(vcgencmd get_throttled)"
  for op in p256-keygen p256-ecdsa-sign; do
    $PERF stat -x, -e $EV -o perf10/stat-$v-$op-r$r.csv taskset -c 3 bins/p256-$v/p256 --bench --profile-time 6 "^$op/rind\$" > /dev/null 2>&1
    [ $r = 1 ] && $PERF record -q -F 3999 -e cycles:u -o perf10/rec-$v-$op.data taskset -c 3 bins/p256-$v/p256 --bench --profile-time 6 "^$op/rind\$" > /dev/null 2>&1
  done
  rm -rf $L; sleep 10
done; done
echo "q10 end $(date)"
