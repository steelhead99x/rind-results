#!/bin/bash
D=$HOME/rind-work; cd $D
for b in f2-x2inl g1-final; do objdump -d --no-show-raw-insn pmu-bins/pmu-$b | awk '/argon24neon8compress>:/{p=1} p&&/^$/{exit} p' | awk '{print $2}' > /tmp/$b.ops; done
echo "pmu f2 vs g1 neon opcode stream: $(cmp -s /tmp/f2-x2inl.ops /tmp/g1-final.ops && echo SAME || echo differ)"
until mkdir $D/.bench.lock 2>/dev/null; do sleep 30; done; echo "argon2-agent (argon2 g1 confirm pi5) $(date)" > $D/.bench.lock/owner
echo "start $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
PIN="taskset -c 3" VARIANTS="a0-before e1-final g1-final" ./run-argon2.sh $D/bins $D/out-argon2-pi5 5 neon
echo "mid $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
PIN="taskset -c 3" ./pmu-ab.sh.nolock $D/pmu-pi-r4.tsv 8 5 "a0-before e1-final g1-final" 1 16 64
echo "end $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
rm -rf $D/.bench.lock
python3 crit-parse.py $D/out-argon2-pi5 > $D/argon2-pi5-results.md
