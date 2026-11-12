# rind on RP2040 (thumbv6m-none-eabi): not buildable at rind a4759c48 (2026-11-12)

Command: RUSTFLAGS='--cfg getrandom_backend="custom"' cargo build -p rind --release --target thumbv6m-none-eabi (rustc 1.99.1)

- Without the cfg: getrandom 0.4.3 fails ("target is not supported ... define a custom backend").
- With it: 1672 errors in rind itself, the first being `error[E0463]: can't find crate for std`.
  rind has no `#![no_std]`; it uses std::thread_local! (low/cpu.rs:121, low/entry.rs:8),
  std::sync::Mutex (low/abi.rs) and Vec/Box/vec! in ~30 mid/high modules.
- Every dependency of the portable backend DOES compile for thumbv6m:
  curve25519-dalek 5, fiat-crypto 0.3, libcrux-sha2 0.0.9, libcrux-poly1305 0.0.7,
  libcrux-hacl-rs 0.0.6 (alloc), subtle, rand_core, getrandom (custom backend).

Reduced build needed: `#![no_std]` + `extern crate alloc` in rind; core:: paths instead of std::;
replace thread_local! (CPU-feature cache / DIT guards) with a static or cfg them out on target_os="none";
gate std-only code (abi.rs Mutex, tests) behind a `std` feature; getrandom custom backend
(RP2040 ROSC). ARMv6-M has no atomic CAS, so the arm cpu.rs AtomicU8 load/store is OK but any
compare_exchange / OnceLock needs portable-atomic or critical-section.
