#!/bin/bash
set -euo pipefail
PID=1893469
RR=~/rind-results
SRC=~/tmp/bestprac-20261112/logs/pr51-ctgrind-portable.log
SNAP="$RR/pr51-7c29ef48-ctgrind-portable.log"
EXITF="$RR/pr51-7c29ef48-ctgrind-portable.exit"
# Wait for the already-detached portable ctgrind wrapper
while kill -0 "$PID" 2>/dev/null; do
  sleep 15
done
# Prefer the run script's END line; fall back to the wrapper's wait status if missing
if rc=$(rg -N "^===== END .* exit=([0-9]+) " -or '$1' "$SRC" | tail -1) && [ -n "$rc" ]; then
  :
else
  rc=99
fi
# Snapshot (replace the symlink with a regular copy once complete)
rm -f "$SNAP"
cp -a "$SRC" "$SNAP"
printf '%s\n' "$rc" > "$EXITF"
echo "portable ctgrind finished exit=$rc $(date -R)" >> "$RR/pr51-7c29ef48-ctgrind-portable-waiter.log"
