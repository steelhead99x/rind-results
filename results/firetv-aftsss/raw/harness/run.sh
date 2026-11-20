#!/system/bin/sh
# Run the pushed rind-bench binaries one core at a time on the Fire TV Stick, then write done.
# Single-threaded (criterion benches are single-threaded) and pinned to CPU $PIN.
P=/data/local/tmp/rind-firetv
cd "$P" || exit 1
PIN=3
PINMASK=8
BENCHES=`cat "$P/benches.txt"`
FILTER=`cat "$P/operation-regex.txt"`
iso_now() { date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date; }
mkdir -p "$P/target/criterion"
snap() { # $1 label
  { echo "== $1 $(iso_now)"; for c in 0 1 2 3; do echo "cpu$c cur=$(cat /sys/devices/system/cpu/cpu$c/cpufreq/scaling_cur_freq 2>&1)"; done
    grep -E "MemFree|MemAvailable|SwapFree" /proc/meminfo; } >> "$P/snap.log"
}
echo "run.sh start $(iso_now) pin=$PIN" >> "$P/run.log"
iso_now > "$P/started"
cat /proc/stat > "$P/stat-before"
snap start
taskset "$PINMASK" "$P/bin/rind-cpuid" > "$P/cpuid.json" 2> "$P/cpuid.err"
echo "rind-cpuid exit=$?" >> "$P/run.log"
fail=0
for name in $BENCHES; do
  b="$P/bin/$name"
  [ -x "$b" ] || { echo "SKIP $name" >> "$P/run.log"; continue; }
  echo "[$(iso_now)] BEGIN $name" >> "$P/run.log"
  snap "before-$name"
  taskset "$PINMASK" "$b" --bench --noplot --warm-up-time 1 --measurement-time 3 "$FILTER" >> "$P/run.log" 2>&1
  rc=$?
  snap "after-$name"
  echo "[$(iso_now)] END $name exit=$rc" >> "$P/run.log"
  [ "$rc" -ne 0 ] && fail=$((fail+1))
done
cat /proc/stat > "$P/stat-after"
iso_now > "$P/finished"
tar cf "$P/criterion.tar" -C "$P/target" criterion 2>/dev/null
echo "SUITE_FAILS=$fail" >> "$P/run.log"
touch "$P/done"
