#!/usr/bin/env bash
# usage: run-gates-current.sh WORKTREE_DIR CI_BASE
# CI_BASE is the upstream main SHA passed to `just ci` (the branch diff base).
set -u
. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
D=~/rind-results/gates-680-681
b=$1
CI_BASE=${2:?ci base sha required}
export TMPDIR=$D/tmp/$b
mkdir -p "$TMPDIR"
export CARGO_TARGET_DIR=$D/target-$b
export CARGO_BUILD_JOBS=6
export CARGO_TERM_COLOR=never
export VALGRIND_BUG_494162=1
cd "$D/$b"
L=$D/logs/$b
mkdir -p "$L"
{
  echo "head $(git rev-parse HEAD)"
  echo "tree $(git rev-parse 'HEAD^{tree}')"
  echo "ci_base $CI_BASE"
} > "$L/summary.txt"
run() {
  name=$1; shift
  echo "== $name start $(date)" >> "$L/summary.txt"
  ( "$@" ) > "$L/$name.log" 2>&1
  rc=$?
  echo "== $name rc=$rc end $(date) / $(df --output=avail -h / | tail -1) /home $(df --output=avail -h /home | tail -1)" >> "$L/summary.txt"
  return 0
}
run ci nix develop --command just ci "$CI_BASE"
run wasm just wasm
run ctgrind nix develop .#ctgrind --command admin/ctgrind --release
run armv7 just armv7
for s in s390x riscv64 riscv64-musl loongarch64 armv6 armv6-softfloat powerpc64le powerpc64 powerpc i686; do
  run qemu-$s just qemu "$s"
done
echo ALLDONE >> "$L/summary.txt"
