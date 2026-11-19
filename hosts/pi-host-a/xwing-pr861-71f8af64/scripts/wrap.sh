#!/bin/bash
# usage: wrap.sh BIN CORE OUT ROUNDS  (Pi 2B: lock + governor performance, restored after)
LOCK=$HOME/bench.lock
while ! mkdir $LOCK 2>/dev/null; do sleep 30; done
echo "xwb x-wing bench $(date -Is) pid $$" > $LOCK/owner
OLD=$(cat /sys/devices/system/cpu/cpufreq/policy0/scaling_governor)
echo performance | sudo tee /sys/devices/system/cpu/cpufreq/policy0/scaling_governor >/dev/null
trap 'echo $OLD | sudo tee /sys/devices/system/cpu/cpufreq/policy0/scaling_governor >/dev/null; rm -rf $LOCK' EXIT
{ date -Is; vcgencmd measure_temp; vcgencmd get_throttled; vcgencmd measure_clock arm; } > $3.pre 2>&1
MAXLOAD=${MAXLOAD:-1.5} $HOME/w-xwb/run.sh "$1" "$2" "$3" "${4:-5}" 1 3
{ date -Is; vcgencmd measure_temp; vcgencmd get_throttled; vcgencmd measure_clock arm; echo old-governor=$OLD; } > $3.post 2>&1
