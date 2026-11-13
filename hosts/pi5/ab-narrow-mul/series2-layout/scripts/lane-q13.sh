#!/bin/bash
cd $HOME/rind-work/lane-pi5
while pgrep -f "^/bin/bash ./lane-q1[12].sh" >/dev/null; do sleep 30; done
[ -d target-deta ] || cp -al target-detect target-deta
find deta/rind deta/rind-bench -name "*.rs" -exec touch {} +
./lane-build.sh deta 3 p256 > build-deta.log 2>&1
pick() { ls -t target-$1/release/deps/p256-* | grep -v "\.d$" | head -1; }
mkdir -p bins/p256-deta; cp $(pick deta) bins/p256-deta/p256
for v in narrow detect deta; do echo "$v table $(nm bins/p256-$v/p256 | grep "4rind3mid4p2567precomp23CURVE_GENERATOR_PRECOMP" | cut -c1-16)"; done
F="^(p256-ecdh|p256-ecdsa-sign|p256-ecdsa-verify|p256-keygen)/(rind|aws-lc-rs)$"
./lane-ab.sh out-deta 5 3 "$F" narrow:bins/p256-narrow:- detect:bins/p256-detect:- deta:bins/p256-deta:- > run-deta.log 2>&1
echo "q13 end $(date)"
