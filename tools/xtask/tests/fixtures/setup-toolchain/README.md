# setup-toolchain transcripts

What `tools/xtask/tests/setup_toolchain.rs` expects from `cargo xtask setup-toolchain`. Each
case covers two runs in a row (exit code and stdout), the stubbed tools' log (each outside
command and its arguments, in order), and the files left in `RIND_HARDWARE_DIR/.tools` and
`~/.cargo/bin`. `$T` is the test's scratch directory. Every stub download is the same logging
script, and the test writes its own pins for it.

- `with-rustup.txt`: rustup is already installed.
- `installs-rustup.txt`: no rustup, so the pinned rustup-init is downloaded, checked and run.
- `espup-mismatch.txt`: the espup pin is wrong, so the download is deleted and the run stops.
- `unknown-arch.txt`: `uname -m` names an arch with no pins, so nothing is downloaded.

stderr is not compared, because the error messages are worded differently.

All four came from the old `tools/setup-toolchain.sh` as main last had it (commit `ac606ee`,
which added the checksum pins), run with the same stubs:

```
git show ac606ee:tools/setup-toolchain.sh > old-setup-toolchain.sh
SETUP_TOOLCHAIN_SCRIPT="$PWD/old-setup-toolchain.sh" cargo test -p xtask --test setup_toolchain
```
