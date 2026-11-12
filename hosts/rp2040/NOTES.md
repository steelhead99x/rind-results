# `rp2040`: not runnable

> **Correction (2026-11-12):** the `no_std` blocker below is addressed by rind-bench `patches/0003` (rind `no_std` on `target_os = "none"`, M-profile portable backend; used for the nRF52840 results). With it, `cargo build -p rind --target thumbv6m-none-eabi` (getrandom custom backend) fails with only 3 errors in `rind/src/low/generic/reuse_cache.rs`: `AtomicU8::compare_exchange` and `fetch_max` don't exist on ARMv6-M (no CAS). A critical-section or `portable-atomic` fallback there would be the remaining step; not done.

- Board: RP2040 (B2, 2 MB flash), Cortex-M0+ (thumbv6m-none-eabi), 264 KB RAM.
- Status: **not benchmarked. rind lacks `no_std`**, so the library doesn't build for thumbv6m-none-eabi: 1672 errors, the first `can't find crate for std`. Every dependency of rind's portable backend does build there; FEASIBILITY.md has the details.
- The board's firmware was backed up in full before any experiment and left untouched. The backup image, USB serial numbers, and flash IDs are left out of this repo.
- QUEST-DRAFT.md is a drafted (not filed) rind quest for a `no_std` + `alloc` build and an embedded bench runner.
