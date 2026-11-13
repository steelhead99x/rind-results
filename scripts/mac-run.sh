#!/bin/bash
# Runs inside rind-bench-run only. Crate cache and build dir stay inside it.
D="$HOME/rind-bench-run"; cd "$D" || exit 1
export PATH="/opt/homebrew/bin:$PATH" CARGO_HOME="$D/cargo" HOSTRUN_FREE_TARGET=1
free_gb() { df -g "$HOME" | awk 'NR==2{print $4}'; }
echo "start $(date) free=$(free_gb)G"
if [ "$(free_gb)" -lt 5 ]; then echo "ABORT: under 5 GiB free"; echo aborted > status; exit 1; fi
rm -rf src && mkdir src && tar xzf rind-src.tar.gz -C src
# Watchdog: stop and clean the build if free space drops under 1.5 GiB.
( while sleep 20; do
    f=$(df -m "$HOME" | awk 'NR==2{print $4}')
    if [ "$f" -lt 1500 ]; then echo "WATCHDOG: ${f}MB free, stopping"; pkill -f "host-run.py --host m1pro" ; pkill -f "rind-bench-run/target" ; rm -rf "$D/target" "$D/src/.tmp"; echo lowdisk > "$D/status"; exit; fi
    [ -f "$D/status" ] && exit
  done ) &
cd src && python3 host-run.py --host m1pro --commit b66514238d449fc211d75fd14554301bb690e421 --jobs 8 --target-dir ../target --out ../m1pro.json
rc=$?
cd "$D"; rm -rf target src/.tmp
[ -f status ] || echo "done rc=$rc" > status
echo "end $(date) rc=$rc free=$(free_gb)G"
