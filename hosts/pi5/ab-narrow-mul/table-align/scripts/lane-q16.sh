#!/bin/bash
# q16: fix/p256-table-align: build p256 bench, cool down <78C, A/B vs narrow-nist (529a9d64) 5 rounds, then tests.
cd $HOME/rind-work/lane-pi5
export PATH=$HOME/.cargo/bin:$PATH
./lane-build.sh align 3 p256 > build-align.log 2>&1
pick() { ls -t target-$1/release/deps/p256-* | grep -v "\.d$" | head -1; }
mkdir -p bins/p256-align; cp $(pick align) bins/p256-align/p256
for v in nist nista align; do echo "$v table $(nm bins/p256-$v/p256 | grep "4rind3mid4p2567precomp23CURVE_GENERATOR_PRECOMP" | cut -c1-16)"; done
while :; do t=$(vcgencmd measure_temp | tr -dc "0-9."); echo "cool $(date +%T) $t"; awk -v t=$t "BEGIN{exit !(t<78)}" && break; sleep 30; done
F="^(p256-ecdh|p256-ecdsa-sign|p256-ecdsa-verify|p256-keygen)/(rind|aws-lc-rs)$"
./lane-ab.sh out-align 5 3 "$F" nist:bins/p256-nist:- align:bins/p256-align:- > run-align.log 2>&1
echo "ab end $(date)"
./lane-test.sh align > test-align.log 2>&1
echo "q16 end $(date)"
