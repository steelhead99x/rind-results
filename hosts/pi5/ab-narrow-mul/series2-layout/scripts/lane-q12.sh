#!/bin/bash
cd $HOME/rind-work/lane-pi5
while pgrep -f "^/bin/bash ./lane-q11.sh" >/dev/null; do sleep 30; done
[ -d target-nista ] || cp -al target-nist target-nista
find nista/rind nista/rind-bench -name "*.rs" -exec touch {} +
./lane-build.sh nista 3 p256 > build-nista.log 2>&1
pick() { ls -t target-$1/release/deps/p256-* | grep -v "\.d$" | head -1; }
mkdir -p bins/p256-nista; cp $(pick nista) bins/p256-nista/p256
md5sum bins/p256-nista/p256 bins/p256-nist/p256
echo "nista table $(nm bins/p256-nista/p256 | grep "4rind3mid4p2567precomp23CURVE_GENERATOR_PRECOMP" | cut -c1-16)"
F="^(p256-ecdh|p256-ecdsa-sign|p256-ecdsa-verify|p256-keygen)/(rind|aws-lc-rs)$"
./lane-ab.sh out-nista 5 3 "$F" nist:bins/p256-nist:- nista:bins/p256-nista:- > run-nista.log 2>&1
echo "q12 end $(date)"
