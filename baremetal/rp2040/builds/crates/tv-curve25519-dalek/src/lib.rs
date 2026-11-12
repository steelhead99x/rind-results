#![no_std]
pub fn base(s: [u8; 32]) -> [u8; 32] { curve25519_dalek::MontgomeryPoint::mul_base_clamped(s).to_bytes() }
