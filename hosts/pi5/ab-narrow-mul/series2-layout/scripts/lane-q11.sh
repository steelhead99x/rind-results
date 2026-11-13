#!/bin/bash
cd $HOME/rind-work/lane-pi5
for t in det2 det3; do
  [ -d target-$t ] || cp -al target-nist target-$t
  find $t/rind $t/rind-bench -name "*.rs" -exec touch {} +
  ./lane-build.sh $t 3 p256 > build-$t.log 2>&1
done
pick() { ls -t target-$1/release/deps/p256-* | grep -v "\.d$" | head -1; }
mkdir -p bins/p256-nist bins/p256-det2 bins/p256-det3
cp bins/nist/p256 bins/p256-nist/; cp $(pick det2) bins/p256-det2/p256; cp $(pick det3) bins/p256-det3/p256
md5sum bins/p256-{nist,det2,det3}/p256
for v in nist det2 det3; do echo "$v table $(nm bins/p256-$v/p256 | grep "4rind3mid4p2567precomp23CURVE_GENERATOR_PRECOMP" | cut -c1-16)"; done
F="^(p256-ecdh|p256-ecdsa-sign|p256-ecdsa-verify|p256-keygen)/(rind|aws-lc-rs)$"
./lane-ab.sh out-det3 5 3 "$F" nist:bins/p256-nist:- det2:bins/p256-det2:- det3:bins/p256-det3:- > run-det3.log 2>&1
./lane-test.sh det3 > test-det3.log 2>&1
echo "q11 end $(date)"
