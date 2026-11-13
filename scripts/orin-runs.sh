#!/bin/bash
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# The Jetson's three timed runs, back to back, each through orin-run.sh.
# FAKETIME_OFFSET (seconds from the board's clock to the PC's, measured over
# ssh before the runs) also dates each run in raw/run-<n>.pc.txt, since the
# board's own clock, which bench-report's JSON keeps, was unset. Run 1 picks
# the least busy core (or CORE); runs 2 and 3 reuse it.
D="$HOME/rind-bench-orin"
pc() { date -u -d "@$(( $(date +%s) + ${FAKETIME_OFFSET:-0} ))" +%FT%T+00:00; }
for n in 1 2 3; do
  echo "pc_started: $(pc)" > "$D/raw/run-$n.pc.txt"
  "$D/orin-run.sh" "$n" ${CORE:-}
  echo "pc_finished: $(pc)" >> "$D/raw/run-$n.pc.txt"
  # Runs 2 and 3 stay on run 1's core, so the three time the same core.
  [ -z "${CORE:-}" ] && CORE=$(sed -n 's/^pinned to cpu//p' "$D/raw/run-1.load.txt")
done
echo runs-done
