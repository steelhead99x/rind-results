#![no_std]
use sha2::Digest; pub fn h(d: &[u8]) -> [u8; 32] { sha2::Sha256::digest(d).into() }
