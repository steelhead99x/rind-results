R=$HOME/rind-pr82-hw-tools/root
L=$R/usr/lib/aarch64-linux-gnu
export PATH=$HOME/rind-pr82-hw-tools/bin:$HOME/.cargo/bin:$PATH
export LIBCLANG_PATH=$L
export BINDGEN_EXTRA_CLANG_ARGS="-I$(echo $R/usr/lib/llvm-19/lib/clang/*/include)"
export PKG_CONFIG_PATH=$HOME/rind-pr82-hw-tools/pkgconfig
export C_INCLUDE_PATH=$R/usr/include:$R/usr/include/aarch64-linux-gnu
export CARGO_BUILD_JOBS=3
