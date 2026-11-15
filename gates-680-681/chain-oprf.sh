#!/usr/bin/env bash
D=~/rind-results/gates-680-681
sleep 60
while pgrep -f 'run-gates-r8.sh|run-gates-692.sh|chain-692.sh|run-gates.sh ed25519' >/dev/null; do sleep 30; done
. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
cd $D/oprf-09aa563
L=$D/logs/oprf-09aa563; mkdir -p $L
export TMPDIR=$D/tmp/oprf CARGO_TARGET_DIR=$D/target-oprf CARGO_BUILD_JOBS=8 CARGO_TERM_COLOR=never; mkdir -p $TMPDIR
echo "head $(git rev-parse HEAD) tree $(git rev-parse HEAD^{tree})" > $L/summary.txt
run() { name=$1; shift; echo "== $name start $(date) / $(df --output=avail -h / | tail -1) /home $(df --output=avail -h /home | tail -1)" >> $L/summary.txt
  ( "$@" ) > $L/$name.log 2>&1; echo "== $name rc=$? end $(date)" >> $L/summary.txt; }
run ctgrind-native-release nix develop .#ctgrind --command admin/ctgrind unit --release -- high::oprf::tests::stays_secret
run ctgrind-native-debug nix develop .#ctgrind --command admin/ctgrind unit -- high::oprf::tests::stays_secret
run ctgrind-portable-release env RUSTFLAGS='--cfg rind_portable --cfg curve25519_dalek_backend="serial"' CARGO_TARGET_DIR=$D/target-oprf-portable nix develop .#ctgrind --command admin/ctgrind unit --release -- high::oprf::tests::stays_secret
run prove-bytes nix develop .#hol-light --command admin/prove bytes
echo ALLDONE >> $L/summary.txt
