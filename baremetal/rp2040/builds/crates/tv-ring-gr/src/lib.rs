#![no_std]
pub fn sha256(d: &[u8]) -> [u8; 32] { let mut o = [0u8; 32]; o.copy_from_slice(ring::digest::digest(&ring::digest::SHA256, d).as_ref()); o }
