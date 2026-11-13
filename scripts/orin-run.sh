#!/bin/bash
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# One timed run of rind's own `admin/bench-report run` on the Jetson, through
# scripts/host-run.py (bench-report unchanged; a busy host is recorded rather
# than refused, and build-cost is skipped so the live box does no cold builds).
# Usage: orin-run.sh <n> [core]. Pins the whole run to one core: the given one,
# or the least busy of cpu1..cpu5 over 10 s (cpu0 takes most interrupts).
# Writes raw/run-<n>.json and raw/run-<n>.load.txt under ~/rind-bench-orin.
# With FAKETIME_OFFSET=<seconds> set (the board's clock is unset and it is
# offline), cargo runs each built binary through orin-faketime.sh, which
# moves only the wall clock so the rustls benches' certificates are valid.
set -u
n=$1
. "$HOME/rind-bench-orin/orin-env.sh"
D="$HOME/rind-bench-orin"; mkdir -p "$D/raw"; cd "$D/rind" || exit 1
L="$D/raw/run-$n.load.txt"
percore() { # busy share of each cpu over $1 seconds, from /proc/stat
  local a b; a=$(grep '^cpu[0-9]' /proc/stat); sleep "$1"; b=$(grep '^cpu[0-9]' /proc/stat)
  paste <(echo "$a") <(echo "$b") | awk '{n=NF/2; t0=0;t1=0; for(i=2;i<=9;i++){t0+=$i;t1+=$(i+n)}; i0=$5+$6; i1=$(5+n)+$(6+n); d=t1-t0; printf "%s %.3f\n", $1, (d>0 ? (d-(i1-i0))/d : 0)}'
}
snap() {
  echo "## $1"
  echo "loadavg: $(cat /proc/loadavg)"
  echo "per-cpu busy over 10 s:"; percore 10 | sed 's/^/  /'
  for c in /sys/devices/system/cpu/cpu[0-9]*/cpufreq; do echo "  $(basename "$(dirname "$c")") $(cat "$c/scaling_governor") cur=$(cat "$c/scaling_cur_freq") max=$(cat "$c/scaling_max_freq")"; done
  echo "top cpu users:"; ps -eo pcpu,comm --sort=-pcpu | sed -n 2,7p | sed 's/^/  /'
  for t in /sys/class/thermal/thermal_zone*/; do echo "  thermal $(cat "$t/type") $(cat "$t/temp")"; done 2>/dev/null | head -9
}
{
  snap "before run $n"
} > "$L"
core=${2:-$(awk '/^  cpu[1-5] [0-9.]+$/{print $1, $2}' "$L" | sort -k2 -n | head -1 | sed 's/cpu//;s/ .*//')}
echo "pinned to cpu$core" >> "$L"
if [ -n "${FAKETIME_OFFSET:-}" ]; then
  export FAKETIME_OFFSET CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="$D/orin-faketime.sh"
  echo "libfaketime runner: wall clock +${FAKETIME_OFFSET}s for the bench binaries, monotonic untouched" >> "$L"
fi
HOSTRUN_SKIP_BUILD_COST=1 RIND_ROOT="$D/rind" taskset -c "$core" python3 "$D/host-run.py" \
  --host jetson-orin-nano --commit "$(cat .commit)" --jobs 2 --target-dir "$CARGO_TARGET_DIR" \
  --out "$D/raw/run-$n.json" > "$D/raw/run-$n.log" 2>&1
rc=$?
{ echo "exit: $rc"; snap "after run $n"; } >> "$L"
echo "run $n rc=$rc core=$core"
