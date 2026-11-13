export PATH=$HOME/rind-work/lane-pi5/prove-tools/bin:$HOME/.elan/bin:$HOME/.cargo/bin:$PATH
export CHARON_TOOLCHAIN_IS_IN_PATH=1 CHARON_MIRI_SYSROOTS=$HOME/rind-work/lane-pi5/prove-tools/charon-sysroots
charon() { LD_LIBRARY_PATH=$HOME/.rustup/toolchains/nightly-2026-09-17-aarch64-unknown-linux-gnu/lib PATH=$HOME/.rustup/toolchains/nightly-2026-09-17-aarch64-unknown-linux-gnu/bin:$PATH command $HOME/rind-work/lane-pi5/prove-tools/bin/charon "$@"; }
export -f charon
