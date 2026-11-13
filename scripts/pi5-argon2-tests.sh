#!/bin/bash
# Full rind lib tests + crosschecks (KATs, CAVP, Wycheproof, RustCrypto differential) on the Pi 5, for each argon2 commit.
D=$HOME/rind-work; cd $D
export PATH=$HOME/.cargo/bin:$PATH OPENSSL_INCLUDE_DIR=$D/sysroot/usr/include OPENSSL_LIB_DIR=$D/sysroot/lib OPENSSL_NO_PKG_CONFIG=1 CARGO_TARGET_DIR=$D/target-test
sleep 60
while pgrep -f "rind-timing|dudect|run-pr33.sh|run-argon2-pi.sh" >/dev/null; do sleep 30; done
for v in umlal prefetch; do
  while [ -d $D/.bench.lock ]; do sleep 30; done
  rm -rf $D/test-src && mkdir $D/test-src && tar xmf /tmp/t-$v.tar -C $D/test-src && tar xzmf /tmp/thirdparty-vectors.tgz -C $D/test-src
  cd $D/test-src
  echo "== $v $(date)"
  nice -n 10 cargo test --jobs 4 --release -p rind --lib 2>&1 | grep -E "test result|FAILED|panicked"
  nice -n 10 cargo test --jobs 4 --release -p rind --test crosschecks 2>&1 | grep -E "test result|FAILED|panicked"
  nice -n 10 cargo test --jobs 4 --release -p rind --lib argon2 -- --include-ignored --skip ct_qemu 2>&1 | grep -E "^test |test result"
  cd $D
done
echo "end $(date)"
