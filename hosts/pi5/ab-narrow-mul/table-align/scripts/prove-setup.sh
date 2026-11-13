set -ex
export PATH=$HOME/.cargo/bin:$PATH
cd ~/rind-work/lane-pi5/prove-tools
curl -fsSL -o aeneas.tgz https://github.com/AeneasVerif/aeneas/releases/download/nightly-2026.11.03-ab34bd2/aeneas-linux-aarch64.tar.gz
echo "hash: $(openssl dgst -sha256 -binary aeneas.tgz | base64) want 010M/V6/mqFYvTEvLPOoB2ppHyewhgX2Zk39twWtF14="
mkdir -p bin && tar xzf aeneas.tgz -C bin aeneas charon charon-driver
rustup toolchain install nightly-2026-09-17 --profile minimal -c rustc-dev,rust-src,llvm-tools,miri -t x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu
test -x $HOME/.elan/bin/elan || (curl -fsSL https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -o elan-init.sh && sh elan-init.sh -y --default-toolchain none --no-modify-path)
echo SETUP-DONE
