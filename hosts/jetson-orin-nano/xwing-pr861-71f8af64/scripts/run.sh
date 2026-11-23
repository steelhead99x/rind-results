#!/bin/bash
# usage: run.sh BIN CORE OUTDIR ROUNDS WARM MEAS
BIN=$1; CORE=$2; OUT=$3; ROUNDS=${4:-5}; WARM=${5:-1}; MEAS=${6:-3}
mkdir -p "$OUT"
libs=(rind libcrux-kem rustcrypto)
for r in $(seq 1 $ROUNDS); do
  # gate: wait up to 15 min for the 1-minute load to fall under MAXLOAD
  for w in $(seq 1 90); do l=$(cut -d' ' -f1 /proc/loadavg); awk -v l=$l -v m=${MAXLOAD:-1.5} 'BEGIN{exit !(l<m)}' && break; sleep 10; done
  echo "round $r load $(cut -d' ' -f1-3 /proc/loadavg) freq $(cat /sys/devices/system/cpu/cpu$CORE/cpufreq/scaling_cur_freq 2>/dev/null)" >> "$OUT/rounds.txt"
  # rotate order per round; rind control last
  order=("${libs[@]}")
  for k in $(seq 1 $((r-1))); do order=("${order[@]:1}" "${order[0]}"); done
  order+=(rind-control)
  for op in keygen encaps decaps; do
    for l in "${order[@]}"; do
      lib=${l%-control}
      taskset -c $CORE $BIN --bench --noplot --warm-up-time $WARM --measurement-time $MEAS "^xwing-$op/$lib\$" > "$OUT/r$r.$op.$l.log" 2>&1
    done
  done
done
echo done >> "$OUT/rounds.txt"
