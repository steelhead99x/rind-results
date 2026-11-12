#!/bin/bash
D=$HOME/rind-work; cd $D
sleep 60
while pgrep -f "rind-timing|dudect|run-pr33.sh|build-argon2.sh" >/dev/null; do sleep 30; done
until mkdir $D/.bench.lock 2>/dev/null; do sleep 30; done; echo "argon2-agent (argon2 A/B pi5) $(date)" > $D/.bench.lock/owner; trap "rm -rf $D/.bench.lock" EXIT
echo "start $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
PIN="taskset -c 3" VARIANTS="a0-before c1-asm1 c2-asm1prefetch" ./run-argon2.sh $D/bins $D/out-argon2-pi 5 neon
echo "end $(date) $(cat /proc/loadavg) temp=$(cat /sys/class/thermal/thermal_zone0/temp)"
python3 crit-parse.py $D/out-argon2-pi > $D/argon2-pi-results.md
