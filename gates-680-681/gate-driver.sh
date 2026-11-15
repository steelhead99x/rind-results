#!/usr/bin/env bash
# One detached supervisor for remaining Linux gates on
#   #689 f3ccebb7 (worktree r8-f3ccebb)
#   #694 322256db (worktree r3-322256d)
# Watches the already-running run-gates-current.sh jobs, appends each
# finished gate to ~/rind-results/gates-status.txt, and if a job dies
# before ALLDONE, runs only the remaining gates. Never redoes a finished
# gate. Does not push, amend, or comment.
set -u
. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh 2>/dev/null || true

STATUS=~/rind-results/gates-status.txt
G=~/rind-results/gates-680-681
LOCK=$G/logs/gate-driver.lock
mkdir -p "$G/logs" "$(dirname "$STATUS")"

# single instance
exec 9>"$LOCK"
if ! flock -n 9; then
  echo "$(date): another gate-driver already holds $LOCK; exiting"
  exit 0
fi
echo "$(date): gate-driver start pid $$"

GATES=(ci wasm ctgrind armv7 qemu-s390x qemu-riscv64 qemu-riscv64-musl qemu-loongarch64 qemu-armv6 qemu-armv6-softfloat qemu-powerpc64le qemu-powerpc64 qemu-powerpc qemu-i686)

append() {
  local line=$1
  mkdir -p "$(dirname "$STATUS")"
  touch "$STATUS"
  if grep -Fqx "$line" "$STATUS"; then
    return 0
  fi
  echo "$line" >> "$STATUS"
  echo "$(date): status: $line"
}

# latest PASS/FAIL/RUNNING for sha+gate in the status file
latest_result() {
  local sha=$1 gate=$2
  awk -v sha="$sha" -v gate="$gate" '
    $0 ~ sha && $0 ~ "  "gate"  " {
      for (i=1;i<=NF;i++) if ($i==sha) { s=i; break }
      if ($(s+1)==gate) r=$(s+2)
    }
    END { if (r!="") print r }
  ' "$STATUS" 2>/dev/null
}

sync_summary() {
  local dir=$1 sha=$2
  local sum=$G/logs/$dir/summary.txt
  [ -f "$sum" ] || return 0
  local gate rc rest endtime result log
  while IFS= read -r raw; do
    case "$raw" in
      "== "*)
        ;;
      *)
        continue
        ;;
    esac
    # == NAME rc=N end DATETIME / ...
    if [[ "$raw" =~ ^==[[:space:]]+([A-Za-z0-9_-]+)[[:space:]]+rc=([0-9]+)[[:space:]]+end[[:space:]]+(.*) ]]; then
      gate=${BASH_REMATCH[1]}
      rc=${BASH_REMATCH[2]}
      rest=${BASH_REMATCH[3]}
      endtime=${rest%% / *}
      if [ "$rc" = 0 ]; then result=PASS; else result=FAIL; fi
      log=$G/logs/$dir/$gate.log
      if [ "$(latest_result "$sha" "$gate")" != "$result" ]; then
        append "$endtime  $sha  $gate  $result  rc=$rc  $log"
      fi
    elif [[ "$raw" =~ ^==[[:space:]]+([A-Za-z0-9_-]+)[[:space:]]+start[[:space:]]+(.*) ]]; then
      gate=${BASH_REMATCH[1]}
      endtime=${BASH_REMATCH[2]}
      cur=$(latest_result "$sha" "$gate")
      if [ "$cur" != "PASS" ] && [ "$cur" != "FAIL" ] && [ "$cur" != "RUNNING" ]; then
        log=$G/logs/$dir/$gate.log
        append "$endtime  $sha  $gate  RUNNING  rc=-  $log"
      fi
    fi
  done < "$sum"
  if grep -qx ALLDONE "$sum" 2>/dev/null; then
    if ! grep -q "^# $sha ALLDONE" "$STATUS"; then
      append "# $sha ALLDONE ($dir)"
    fi
    return 0
  fi
  return 1
}

driver_alive() {
  local dir=$1
  pgrep -f "run-gates-current.sh $dir " >/dev/null 2>&1
}

suite_done() {
  local sha=$1 dir=$2
  grep -q "^# $sha ALLDONE" "$STATUS" 2>/dev/null && return 0
  [ -f "$G/logs/$dir/summary.txt" ] && grep -qx ALLDONE "$G/logs/$dir/summary.txt"
}

# Run one remaining gate in a worktree, appending to both summary and status.
run_one() {
  local dir=$1 sha=$2 ci_base=$3 name=$4
  shift 4
  local L=$G/logs/$dir
  mkdir -p "$L" "$G/tmp/$dir"
  export TMPDIR=$G/tmp/$dir
  export CARGO_TARGET_DIR=$G/target-$dir
  export CARGO_BUILD_JOBS=6
  export CARGO_TERM_COLOR=never
  export VALGRIND_BUG_494162=1
  cd "$G/$dir" || return 1
  echo "== $name start $(date)" >> "$L/summary.txt"
  append "$(date)  $sha  $name  RUNNING  rc=-  $L/$name.log"
  ( "$@" ) > "$L/$name.log" 2>&1
  local rc=$?
  echo "== $name rc=$rc end $(date) / $(df --output=avail -h / | tail -1) /home $(df --output=avail -h /home | tail -1)" >> "$L/summary.txt"
  local result=PASS
  [ "$rc" = 0 ] || result=FAIL
  append "$(date)  $sha  $name  $result  rc=$rc  $L/$name.log"
  return 0
}

gate_finished() {
  local sha=$1 gate=$2
  local r
  r=$(latest_result "$sha" "$gate")
  [ "$r" = PASS ] || [ "$r" = FAIL ]
}

# If the original driver is gone and the suite is incomplete, finish it here.
finish_remaining() {
  local dir=$1 sha=$2 ci_base=$3
  local L=$G/logs/$dir
  mkdir -p "$L"
  [ -f "$L/summary.txt" ] || {
    echo "head $sha" > "$L/summary.txt"
    echo "ci_base $ci_base" >> "$L/summary.txt"
  }
  echo "$(date): resuming remaining gates for $dir $sha"
  append "# $(date): supervisor resuming remaining gates for $sha ($dir)"
  local g
  for g in "${GATES[@]}"; do
    if gate_finished "$sha" "$g"; then
      echo "$(date): skip finished $sha $g"
      continue
    fi
    case "$g" in
      ci)      run_one "$dir" "$sha" "$ci_base" ci nix develop --command just ci "$ci_base" ;;
      wasm)    run_one "$dir" "$sha" "$ci_base" wasm just wasm ;;
      ctgrind) run_one "$dir" "$sha" "$ci_base" ctgrind nix develop .#ctgrind --command admin/ctgrind --release ;;
      armv7)   run_one "$dir" "$sha" "$ci_base" armv7 just armv7 ;;
      qemu-*)  run_one "$dir" "$sha" "$ci_base" "$g" just qemu "${g#qemu-}" ;;
    esac
  done
  echo ALLDONE >> "$L/summary.txt"
  append "# $sha ALLDONE ($dir)"
}

CI_BASE_689=da1faa639d8e456909e84b3721b992c9a00c3fcf
CI_BASE_694=da1faa639d8e456909e84b3721b992c9a00c3fcf
SHA689=f3ccebb7ba585c6311f2001e9b3ce2215b6687f6
SHA694=322256db26222c88c45de621bfc9e21401474356
DIR689=r8-f3ccebb
DIR694=r3-322256d

# Confirm worktrees still match the intended SHAs before resuming anything.
check_head() {
  local dir=$1 want=$2
  local got
  got=$(git -C "$G/$dir" rev-parse HEAD 2>/dev/null || echo missing)
  if [ "$got" != "$want" ]; then
    append "# ERROR $dir HEAD is $got, expected $want; will not start new gates there"
    return 1
  fi
  return 0
}

while true; do
  done689=0
  done694=0

  sync_summary "$DIR689" "$SHA689" && done689=1
  sync_summary "$DIR694" "$SHA694" && done694=1

  if [ "$done689" -eq 0 ]; then
    if ! driver_alive "$DIR689"; then
      if check_head "$DIR689" "$SHA689"; then
        finish_remaining "$DIR689" "$SHA689" "$CI_BASE_689"
        done689=1
      else
        echo "$(date): refusing to resume $DIR689 (HEAD mismatch)"
        done689=1
      fi
    fi
  fi

  if [ "$done694" -eq 0 ]; then
    if ! driver_alive "$DIR694"; then
      if check_head "$DIR694" "$SHA694"; then
        finish_remaining "$DIR694" "$SHA694" "$CI_BASE_694"
        done694=1
      else
        echo "$(date): refusing to resume $DIR694 (HEAD mismatch)"
        done694=1
      fi
    fi
  fi

  if [ "$done689" -eq 1 ] && [ "$done694" -eq 1 ]; then
    append "# $(date): gate-driver finished both current heads"
    echo "$(date): both suites done; exiting"
    exit 0
  fi
  sleep 30
done
