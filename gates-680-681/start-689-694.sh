#!/usr/bin/env bash
# Fetch current heads of spake2plus-up-r8 and ed25519-cctv-wipe-canaries-r3,
# clone/checkout those exact SHAs, and start Linux gates under nohup.
# Does not push, amend, or comment.
set -euo pipefail
. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
D=~/rind-results/gates-680-681
LOG=$D/logs
mkdir -p "$LOG" "$D/tmp"
FETCH_LOG=$LOG/start-689-694.fetch.log
{
  echo "===== fetch $(date) ====="
  R8=$(git ls-remote https://github.com/steelhead99x/rind.git refs/heads/spake2plus-up-r8 | awk '{print $1}')
  R3=$(git ls-remote https://github.com/steelhead99x/rind.git refs/heads/ed25519-cctv-wipe-canaries-r3 | awk '{print $1}')
  MAIN=$(git ls-remote https://github.com/moq-dev/rind.git refs/heads/main | awk '{print $1}')
  echo "spake2plus-up-r8 $R8"
  echo "ed25519-cctv-wipe-canaries-r3 $R3"
  echo "upstream main $MAIN"
  if [ -z "$R8" ] || [ -z "$R3" ] || [ -z "$MAIN" ]; then
    echo "missing SHA" >&2
    exit 1
  fi

  clone_at() {
    local dir=$1 branch=$2 sha=$3
    if [ ! -d "$D/$dir/.git" ] && [ ! -f "$D/$dir/.git" ]; then
      local ref=()
      if [ -d "$D/r8-542264a/.git" ] || [ -f "$D/r8-542264a/.git" ]; then
        ref=(--reference-if-able "$D/r8-542264a")
      fi
      git clone -q --recurse-submodules "${ref[@]}" --branch "$branch" https://github.com/steelhead99x/rind "$D/$dir"
    fi
    git -C "$D/$dir" fetch -q origin "$branch"
    git -C "$D/$dir" checkout -q --detach "$sha"
    git -C "$D/$dir" submodule update --init --recursive
    local got
    got=$(git -C "$D/$dir" rev-parse HEAD)
    if [ "$got" != "$sha" ]; then
      echo "checkout mismatch $dir expected $sha got $got" >&2
      exit 1
    fi
    echo "verified $dir $got tree $(git -C "$D/$dir" rev-parse 'HEAD^{tree}')"
  }

  d8="r8-${R8:0:7}"
  d3="r3-${R3:0:7}"
  clone_at "$d8" spake2plus-up-r8 "$R8"
  clone_at "$d3" ed25519-cctv-wipe-canaries-r3 "$R3"

  nohup "$D/run-gates-current.sh" "$d8" "$MAIN" > "$LOG/$d8.driver.log" 2>&1 &
  echo "started #689 $d8 pid $! log $LOG/$d8.driver.log"
  nohup "$D/run-gates-current.sh" "$d3" "$MAIN" > "$LOG/$d3.driver.log" 2>&1 &
  echo "started #694 $d3 pid $! log $LOG/$d3.driver.log"
} >> "$FETCH_LOG" 2>&1
