#![no_std]
pub use hmac; pub type HmacSha256 = hmac::Hmac<sha2::Sha256>;
