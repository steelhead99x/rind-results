#!/bin/bash
# Runs on the Pi inside ~/rind-bench-run only. No sudo, no system changes:
# if libssl-dev is missing, its .deb is downloaded and unpacked into ./sysroot.
D="$HOME/rind-bench-run"; cd "$D" || exit 1
rm -f status
export PATH="$HOME/.cargo/bin:$PATH" CARGO_HOME="$D/cargo"
echo "start $(date)"
command -v openssl >/dev/null || { echo "ABORT: openssl CLI missing"; echo no-openssl-cli > status; exit 1; }
if ! pkg-config --exists openssl; then
  echo "libssl-dev not installed; unpacking it locally into $D/sysroot"
  rm -rf sysroot debs && mkdir -p sysroot debs && cd debs && apt-get download libssl-dev || { echo "ABORT: apt-get download libssl-dev failed"; echo no-libssl-dev > "$D/status"; exit 1; }
  cd "$D" && dpkg -x debs/libssl-dev_*.deb sysroot
  ML=$(gcc -print-multiarch); INC="$D/sysroot/usr/include"
  cp -n "$INC/$ML/openssl/"*.h "$INC/openssl/" 2>/dev/null
  mkdir -p "$D/sysroot/lib"; ln -sf /usr/lib/$ML/libssl.so.3 "$D/sysroot/lib/libssl.so"; ln -sf /usr/lib/$ML/libcrypto.so.3 "$D/sysroot/lib/libcrypto.so"
  export OPENSSL_INCLUDE_DIR="$INC" OPENSSL_LIB_DIR="$D/sysroot/lib" OPENSSL_NO_PKG_CONFIG=1
  dpkg-query -W libssl3t64 libssl3 2>/dev/null; ls debs
fi
rm -rf src && mkdir src && tar xzf rind-src.tar.gz -C src
cd src && python3 host-run.py --host pi5 --commit b66514238d449fc211d75fd14554301bb690e421 --jobs 4 --target-dir ../target --out ../pi5.json
rc=$?; cd "$D"
echo "done rc=$rc" > status; echo "end $(date) rc=$rc"
