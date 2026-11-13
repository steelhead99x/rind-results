#!/bin/bash
# PR 33 A/B on the Pi 5: one binary per bench, built with --cfg rind_cpu_disable.
# narrow = default detection (Cortex-A76 -> narrow schedules); wide = RIND_CPU_FORCE_wide_mul=1 (main behaviour, _alt).
D=$HOME/rind-work; cd $D/pr33/rind-bench || exit 1
while pgrep -f "rind-timing|dudect|build-argon2.sh|build-pr33.sh" >/dev/null; do sleep 30; done
until mkdir $D/.bench.lock 2>/dev/null; do sleep 30; done; echo "argon2-agent (PR33 A/B pi5) $(date)" > $D/.bench.lock/owner; trap "rm -rf $D/.bench.lock" EXIT
echo "start $(date) load: $(cat /proc/loadavg)"
F="^(x25519-ecdh|p256-ecdh|p256-ecdsa-sign|p256-ecdsa-verify|p384-ecdh|p384-ecdsa-sign|p384-ecdsa-verify|p521-ecdh|p521-ecdsa-sign|p521-ecdsa-verify|ed25519-sign|ed25519-verify|rsa2048-pkcs1-sha256-sign|rsa2048-pss-sha256-sign|rsa2048-pkcs1-sha256-verify|rsa2048-pss-sha256-verify)/(rind|aws-lc-rs)$"
OUT=$D/out-pr33; rm -rf $OUT; mkdir -p $OUT
for r in 1 2 3; do for mode in narrow wide; do
  echo "round $r $mode $(date) load: $(cat /proc/loadavg)"
  for b in x25519 p256 p384 p521 ed25519 rsa_priv rsa_pub; do
    f=$(ls -t $D/target-pr33/release/deps/$b-* | grep -v "\.d$" | head -1)
    if [ $mode = wide ]; then E="env RIND_CPU_FORCE_wide_mul=1"; else E=env; fi
    CRITERION_HOME=$OUT/$mode-r$r $E taskset -c 3 $f --bench --noplot --warm-up-time 1 --measurement-time 3 "$F" > /dev/null 2>>$OUT/stderr.log
  done
done; done
echo "end $(date)"; python3 $D/crit-parse.py $OUT > $D/pr33-results.md
