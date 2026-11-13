# setup-toolchain transcripts

What `tools/xtask/tests/setup_toolchain.rs` expects from `cargo xtask setup-toolchain`: for
each case, two runs in a row (stdout, stderr, exit code), the stubbed tools' log (each outside
command and its arguments, in order) and the files left in `RIND_HARDWARE_DIR/.tools` and
`~/.cargo/bin`. `$T` is the test's scratch directory.

- `with-rustup.txt`: rustup is already installed.
- `installs-rustup.txt`: no rustup, so rustup's installer is fetched and run first.

Both came from the old `tools/setup-toolchain.sh` (last in commit `df3cd46`), run with the
same stubs:

```
git show df3cd46:tools/setup-toolchain.sh > old-setup-toolchain.sh
SETUP_TOOLCHAIN_SCRIPT="$PWD/old-setup-toolchain.sh" cargo test -p xtask --test setup_toolchain
```
