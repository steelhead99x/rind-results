# builds

A cargo workspace of `#![no_std]` lib crates (`crates/tv-*`), one per library. It records which crates compile for the RP2040 (`thumbv6m-none-eabi`) and nearby bare-metal targets, and is a starting point for an embedded bench runner.

## Toolchain

Rust stable (checked with rustc 1.99.1):

```
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabihf \
  thumbv8m.base-none-eabi thumbv8m.main-none-eabihf \
  riscv32i-unknown-none-elf riscv32imc-unknown-none-elf riscv32imac-unknown-none-elf
```

C compilers, needed only by ring and aws-lc-rs:

- Arm: `arm-none-eabi-gcc` (checked with 15.3.1)
- riscv32: `clang` (checked with 21.1.8, no libc headers)
- With Nix: `nix shell nixpkgs#gcc-arm-embedded nixpkgs#llvmPackages.clang-unwrapped`

## Run

```
./build-matrix.sh thumbv6m-none-eabi          # one target, all crates
cargo build --release --target thumbv6m-none-eabi -p tv-sha2

# crates that pull getrandom 0.4
RUSTFLAGS='--cfg getrandom_backend="custom"' cargo build --release \
  --target thumbv6m-none-eabi -p tv-rind-portable-deps

nix shell nixpkgs#gcc-arm-embedded nixpkgs#llvmPackages.clang-unwrapped \
  --command ./build-matrix.sh thumbv6m-none-eabi

CC_riscv32imac_unknown_none_elf=clang AR_riscv32imac_unknown_none_elf=llvm-ar \
  ./build-matrix.sh riscv32imac-unknown-none-elf
```

`build-matrix.sh` retries with the getrandom custom-backend cfg if the first build fails there. It prints `crate`, `PASS`/`FAIL`, the cfg used, and the first error. Logs go to `logs/<target>/`, which git ignores, as does `target/`.

## Crates

Versions match `rind-bench/Cargo.toml`, pinned with `=`.

- `tv-rind` — rind 0.3.2
- `tv-rind-portable-deps` — dependencies of rind's portable backend
- `tv-ring`, `tv-ring-gr` — ring 0.17.14; `-gr` adds getrandom 0.2's `custom` feature
- `tv-aws-lc-rs` — aws-lc-rs 1.19.0
- `tv-rustls-*` — rustls 0.23.46 with no provider, ring, aws-lc-rs, or rustls-rustcrypto. `-pa` adds once_cell `portable-atomic` and portable-atomic `critical-section`
- `tv-sha2`, `tv-hmac`, `tv-aes-gcm`, `tv-chacha20poly1305`, `tv-p256`, `tv-curve25519-dalek`, `tv-x25519-dalek`, `tv-ed25519-dalek` — RustCrypto and dalek crates

Results: [../COMPETITOR-BUILDS.md](../COMPETITOR-BUILDS.md).

## Later: on-device runner

Add a binary with `cortex-m-rt` and `rp2040-hal` or `embassy-rp`, a `memory.x`, a `critical-section` implementation, and a getrandom custom backend. Time passing crates with the RP2040 1 MHz timer (`TIMERAWL`) or SysTick (no DWT on M0+), and print cycles/op over USB CDC or defmt-rtt. Add rind once it builds `no_std`. Flash only after a verified full-flash backup (`picotool save -r 0x10000000 0x10200000`), and restore afterwards.
