#!/bin/bash
D=$HOME/rind-work; cd $D/pr33/rind-bench || exit 1
export PATH=$HOME/.cargo/bin:$PATH OPENSSL_INCLUDE_DIR=$D/sysroot/usr/include OPENSSL_LIB_DIR=$D/sysroot/lib OPENSSL_NO_PKG_CONFIG=1 RUSTFLAGS="--cfg rind_cpu_disable" CARGO_TARGET_DIR=$D/target-pr33
echo "start $(date)"
nice -n 19 cargo bench --jobs 2 --no-run --bench x25519 --bench p256 --bench p384 --bench p521 --bench ed25519 --bench rsa_priv --bench rsa_pub 2>&1 | grep -v "^ *Compiling"
echo "end $(date) rc=${PIPESTATUS[0]}"
