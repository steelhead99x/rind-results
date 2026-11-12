#!/bin/bash
# pmu-ab.sh OUT ALTERNATIONS ROUNDS "VARIANTS" SIZES...: alternates pmu-<variant> binaries under the bench lock.
D=$HOME/rind-work; OUT=$1; A=$2; R=$3; V="$4"; shift 4; S="$*"
{ echo "# start $(date) load=$(uptime | sed 's/.*average: //') $(command -v vcgencmd >/dev/null && vcgencmd measure_temp)"
for a in $(seq 1 $A); do for v in $V; do $PIN $D/pmu-bins/pmu-$v $v $R $S | sed "s/^/a$a\t/"; done; done
echo "# end $(date) load=$(uptime | sed 's/.*average: //') $(command -v vcgencmd >/dev/null && vcgencmd measure_temp) $(command -v vcgencmd >/dev/null && vcgencmd get_throttled)"; } > $OUT
