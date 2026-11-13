set -ex
export PATH="$HOME/.cargo/bin:$PATH"
cd ~/rind-work/lane-pi5/prove-tools
curl -fsSL -o aeneas.tgz https://github.com/AeneasVerif/aeneas/releases/download/nightly-2026.11.03-ab34bd2/aeneas-linux-aarch64.tar.gz
want='010M/V6/mqFYvTEvLPOoB2ppHyewhgX2Zk39twWtF14='
got=$(openssl dgst -sha256 -binary aeneas.tgz | base64)
if [ "$got" != "$want" ]; then
    echo "aeneas tarball digest mismatch" >&2
    exit 1
fi
mkdir -p bin && tar xzf aeneas.tgz -C bin aeneas charon charon-driver
rustup toolchain install nightly-2026-09-17 --profile minimal -c rustc-dev,rust-src,llvm-tools,miri -t x86_64-unknown-linux-gnu,aarch64-unknown-linux-gnu
# elan v4.2.4. The installer is pinned to that commit, not master.
if ! test -x "$HOME/.elan/bin/elan"; then
    elan_commit=227caca133724d5516bee25c2aeb3e609478f2d8
    elan_sha256=a620ff1641616222c8d37c54845492004bb84d6877cdbc944dd65c1aa685bf53
    curl -fsSL -o elan-init.sh "https://raw.githubusercontent.com/leanprover/elan/${elan_commit}/elan-init.sh"
    if ! printf '%s  %s\n' "$elan_sha256" elan-init.sh | sha256sum -c -; then
        rm -f elan-init.sh
        echo "elan-init.sh: sha256 mismatch" >&2
        exit 1
    fi
    sh elan-init.sh -y --default-toolchain none --no-modify-path
fi
echo SETUP-DONE
