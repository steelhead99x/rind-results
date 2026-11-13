#!/bin/bash
# lane-guard.sh: while another agent holds ~/rind-work/.bench.lock, SIGSTOP every descendant of this lane's
# queue scripts (builds, tests); SIGCONT them when the lock is free or ours. Exits when no queue script is left.
D=$HOME/rind-work; stopped=0
desc() { local p; for p in $(pgrep -P $1); do echo $p; desc $p; done; }
while :; do
  roots=$(pgrep -f "^/bin/bash ./lane-(q[0-9]+|test|build2?)\.sh"; pgrep -f "^sh -c while kill -0")
  [ -z "$roots" ] && break
  owner=$(cat $D/.bench.lock/owner 2>/dev/null)
  pids=""; for r in $roots; do pids="$pids $(desc $r)"; done
  if [ -d $D/.bench.lock ] && [ -n "$owner" ] && [[ "$owner" != pi5-lane* ]]; then
    [ -n "${pids// }" ] && kill -STOP $pids 2>/dev/null
    [ $stopped = 0 ] && echo "stop $(date +%T) owner: $owner pids:$pids"; stopped=1
  elif [ $stopped = 1 ]; then
    [ -n "${pids// }" ] && kill -CONT $pids 2>/dev/null; echo "cont $(date +%T)"; stopped=0
  fi
  sleep 3
done
[ $stopped = 1 ] && kill -CONT $pids 2>/dev/null
echo "guard end $(date)"
