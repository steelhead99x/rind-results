//! Incremental SHA-256/SHA-512 using rind's compress + the same pad/split_tail
//! helpers as rind's mid/sha2.rs (FIPS 180).
use crate::rind_port::blockwise::{pad, sha256_digest, split_tail};
use crate::rind_port::{sha256, sha512};

pub const SHA256_IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
    0x5be0cd19,
];

pub const SHA512_IV: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

pub fn sha256_digest_bytes(h: &[u32; 8]) -> [u8; 32] {
    sha256_digest(h)
}

pub fn sha512_digest_bytes(h: &[u64; 8]) -> [u8; 64] {
    let mut r = [0u8; 64];
    for (i, word) in h.iter().enumerate() {
        r[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
    }
    r
}

/// Scratch in BSS so SHA-512 padding does not overflow the 2 KB SRAM
/// (stack + .data already leave only a few hundred bytes).
static mut SHA_WS: [u8; 256] = [0; 256];

fn ws() -> &'static mut [u8; 256] {
    unsafe { &mut *(&raw mut SHA_WS) }
}

/// One-shot SHA-256 of `message` (must already be in SRAM, so keep it short).
pub fn sha256_oneshot(message: &[u8]) -> [u8; 32] {
    let mut h = SHA256_IV;
    let bits = (message.len() as u64).saturating_mul(8).to_be_bytes();
    let (blocks, rest) = split_tail::<64, 8>(message);
    if !blocks.is_empty() {
        sha256::sha256_compress_blocks(&mut h, blocks);
    }
    let last = ws();
    last.fill(0);
    let last_pair: &mut [[u8; 64]; 2] = unsafe { &mut *(last.as_mut_ptr() as *mut [[u8; 64]; 2]) };
    sha256::sha256_compress_blocks(&mut h, pad(rest, bits, last_pair));
    sha256_digest(&h)
}

pub fn sha512_oneshot(message: &[u8]) -> [u8; 64] {
    let mut h = SHA512_IV;
    let bits = (message.len() as u128).saturating_mul(8).to_be_bytes();
    let (blocks, rest) = split_tail::<128, 16>(message);
    if !blocks.is_empty() {
        sha512::sha512_compress_blocks(&mut h, blocks);
    }
    let last = ws();
    last.fill(0);
    let last_pair: &mut [[u8; 128]; 2] = unsafe { &mut *(last.as_mut_ptr() as *mut [[u8; 128]; 2]) };
    sha512::sha512_compress_blocks(&mut h, pad(rest, bits, last_pair));
    sha512_digest_bytes(&h)
}

/// Stream `len` bytes of `pattern(i)` through SHA-256 without holding the message.
pub fn sha256_pattern(len: u32, pattern: impl Fn(u32) -> u8) -> [u8; 32] {
    let mut h = SHA256_IV;
    let block = &mut ws()[..64];
    let mut off = 0u32;
    let full = len / 64;
    for i in 0..full {
        for j in 0..64u32 {
            block[j as usize] = pattern(i * 64 + j);
        }
        sha256::sha256_compress_blocks(&mut h, block);
        off += 64;
    }
    let rem = (len % 64) as usize;
    for j in 0..rem {
        block[j] = pattern(off + j as u32);
    }
    // Copy rem bytes aside, then reuse ws for padding (block aliases ws).
    let mut rest_tmp = [0u8; 64];
    rest_tmp[..rem].copy_from_slice(&block[..rem]);
    let bits = (len as u64).saturating_mul(8).to_be_bytes();
    let (blocks, rest) = split_tail::<64, 8>(&rest_tmp[..rem]);
    if !blocks.is_empty() {
        sha256::sha256_compress_blocks(&mut h, blocks);
    }
    let last = ws();
    last.fill(0);
    let last_pair: &mut [[u8; 64]; 2] = unsafe { &mut *(last.as_mut_ptr() as *mut [[u8; 64]; 2]) };
    sha256::sha256_compress_blocks(&mut h, pad(rest, bits, last_pair));
    sha256_digest(&h)
}

fn sha512_pad_len(dst: &mut [u8; 256], rem: usize, len: u32) -> usize {
    // SHA-512 128-bit length; for len <= 16 KiB the high 64 bits are zero.
    dst[rem] = 0x80;
    let need_two = rem + 1 + 16 > 128;
    if need_two {
        for i in (rem + 1)..128 {
            dst[i] = 0;
        }
    } else {
        for i in (rem + 1)..112 {
            dst[i] = 0;
        }
        for i in 112..128 {
            dst[i] = 0;
        }
    }
    let bits = (len as u64).wrapping_mul(8);
    let off = if need_two { 256 - 16 } else { 128 - 16 };
    if need_two {
        for i in 128..256 {
            dst[i] = 0;
        }
    }
    for i in 0..8 {
        dst[off + 8 + i] = (bits >> (56 - 8 * i)) as u8;
    }
    if need_two { 256 } else { 128 }
}

pub fn sha512_pattern(len: u32, pattern: impl Fn(u32) -> u8) -> [u8; 64] {
    let mut h = SHA512_IV;
    let mut off = 0u32;
    let full = len / 128;
    for i in 0..full {
        let block = &mut ws()[..128];
        for j in 0..128u32 {
            block[j as usize] = pattern(i * 128 + j);
        }
        sha512::sha512_compress_blocks(&mut h, block);
        off += 128;
    }
    let rem = (len % 128) as usize;
    {
        let block = &mut ws()[..128];
        for j in 0..rem {
            block[j] = pattern(off + j as u32);
        }
    }
    let n = sha512_pad_len(ws(), rem, len);
    sha512::sha512_compress_blocks(&mut h, &ws()[..n]);
    sha512_digest_bytes(&h)
}

/// XOR ChaCha20 keystream (RFC 8439, 96-bit nonce, 32-bit counter) into a
/// buffer, one block at a time, using rind's `chacha20_block`.
#[inline(never)]
pub fn chacha20_apply(key: &[u8; 32], nonce: &[u8; 12], mut counter: u32, buf: &mut [u8]) {
    let mut i = 0;
    while i < buf.len() {
        let mut cn = [0u8; 16];
        cn[..4].copy_from_slice(&counter.to_le_bytes());
        cn[4..].copy_from_slice(nonce);
        let ks = crate::rind_port::chacha20::chacha20_block(key, &cn);
        let n = core::cmp::min(64, buf.len() - i);
        for j in 0..n {
            buf[i + j] ^= ks[j];
        }
        counter = counter.wrapping_add(1);
        i += n;
    }
}

/// Stream-XOR `len` bytes of `pattern(i)` with ChaCha20, accumulating a
/// 32-bit checksum of the keystream-xor output so we can spot a silent
/// empty run without keeping the whole buffer.
pub fn chacha20_pattern_checksum(
    key: &[u8; 32],
    nonce: &[u8; 12],
    counter: u32,
    len: u32,
    pattern: impl Fn(u32) -> u8,
) -> u32 {
    let mut sum = 0u32;
    let mut off = 0u32;
    let mut ctr = counter;
    let mut tmp = [0u8; 64];
    while off < len {
        let n = core::cmp::min(64, len - off) as usize;
        for j in 0..n {
            tmp[j] = pattern(off + j as u32);
        }
        chacha20_apply(key, nonce, ctr, &mut tmp[..n]);
        for j in 0..n {
            sum = sum.wrapping_add(tmp[j] as u32);
        }
        ctr = ctr.wrapping_add(1);
        off += n as u32;
    }
    sum
}

pub fn poly1305_pattern(key: &[u8; 32], len: u32, pattern: impl Fn(u32) -> u8) -> [u8; 16] {
    let mut p = crate::rind_port::poly1305::Poly1305::new(key);
    let mut tmp = [0u8; 16];
    let mut off = 0u32;
    while off < len {
        let n = core::cmp::min(16, len - off) as usize;
        for j in 0..n {
            tmp[j] = pattern(off + j as u32);
        }
        p.add_bytes(&tmp[..n]);
        off += n as u32;
    }
    p.finish()
}

/// Pattern used for length benches (reproducible on the host).
pub fn bench_byte(i: u32) -> u8 {
    i as u8
}
