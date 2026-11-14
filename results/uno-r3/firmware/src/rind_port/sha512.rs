// Adapted for the Arduino Uno R3 (ATmega328P) bench.
// Source: steelhead99x/rind @ c08c91ea (origin/rp2040/armv6m-no-std).
// Changes: pub(crate) -> pub, tests stripped, K tables loaded from PROGMEM
// on AVR, Poly1305 Blockwise methods left enabled (originally cfg'd to
// 32/64-bit pointer widths). Algorithm is otherwise rind's.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

//! SHA-512's compression function as one rolled loop of rounds, for ARMv6-M.
//!
//! HACL*'s fully unrolled SHA-512 is about 27 KB of Thumb code there, larger
//! than the RP2040's 16 KB execute-in-place cache, so every block refetched
//! most of it from QSPI flash: about 2,200 cycles per byte. This loop body
//! stays in the cache. Every index and branch depends only on the round
//! number, never on the data.

macro_rules! CH {
    ($x:expr, $y:expr, $z:expr) => {
        ($x & $y) ^ (!$x & $z)
    };
}

macro_rules! MAJ {
    ($x:expr, $y:expr, $z:expr) => {
        ($x & $y) ^ ($x & $z) ^ ($y & $z)
    };
}

macro_rules! BSIG0 {
    ($x:expr) => {
        $x.rotate_right(28) ^ $x.rotate_right(34) ^ $x.rotate_right(39)
    };
}

macro_rules! BSIG1 {
    ($x:expr) => {
        $x.rotate_right(14) ^ $x.rotate_right(18) ^ $x.rotate_right(41)
    };
}

macro_rules! SSIG0 {
    ($x:expr) => {
        $x.rotate_right(1) ^ $x.rotate_right(8) ^ ($x >> 7)
    };
}

macro_rules! SSIG1 {
    ($x:expr) => {
        $x.rotate_right(19) ^ $x.rotate_right(61) ^ ($x >> 6)
    };
}

// 32-bit-limb compress on AVR so the 80-round body does not blow the
// 2 KB SRAM stack with u64 spills. Same arithmetic as firmware/src/sha512_compress.c
// (host-tested against hashlib for abc / pattern 32B / pattern 1350B).
#[cfg(target_arch = "avr")]
#[derive(Clone, Copy)]
struct U64 {
    lo: u32,
    hi: u32,
}

#[cfg(target_arch = "avr")]
fn add64(a: U64, b: U64) -> U64 {
    let lo = a.lo.wrapping_add(b.lo);
    let c = if lo < a.lo { 1u32 } else { 0u32 };
    U64 {
        lo,
        hi: a.hi.wrapping_add(b.hi).wrapping_add(c),
    }
}

#[cfg(target_arch = "avr")]
fn xor64(a: U64, b: U64) -> U64 {
    U64 { lo: a.lo ^ b.lo, hi: a.hi ^ b.hi }
}

#[cfg(target_arch = "avr")]
fn and64(a: U64, b: U64) -> U64 {
    U64 { lo: a.lo & b.lo, hi: a.hi & b.hi }
}

#[cfg(target_arch = "avr")]
fn not64(a: U64) -> U64 {
    U64 { lo: !a.lo, hi: !a.hi }
}

#[cfg(target_arch = "avr")]
fn rotr64(x: U64, n: u8) -> U64 {
    if n == 0 {
        return x;
    }
    if n < 32 {
        let m = 32 - n;
        U64 {
            lo: (x.lo >> n) | (x.hi << m),
            hi: (x.hi >> n) | (x.lo << m),
        }
    } else if n == 32 {
        U64 { lo: x.hi, hi: x.lo }
    } else {
        let n = n - 32;
        let m = 32 - n;
        U64 {
            lo: (x.hi >> n) | (x.lo << m),
            hi: (x.lo >> n) | (x.hi << m),
        }
    }
}

#[cfg(target_arch = "avr")]
fn shr64(x: U64, n: u8) -> U64 {
    if n == 0 {
        return x;
    }
    if n < 32 {
        let m = 32 - n;
        U64 {
            lo: (x.lo >> n) | (x.hi << m),
            hi: x.hi >> n,
        }
    } else if n == 32 {
        U64 { lo: x.hi, hi: 0 }
    } else {
        let n = n - 32;
        U64 { lo: x.hi >> n, hi: 0 }
    }
}

#[cfg(target_arch = "avr")]
fn ch(x: U64, y: U64, z: U64) -> U64 {
    xor64(and64(x, y), and64(not64(x), z))
}
#[cfg(target_arch = "avr")]
fn maj(x: U64, y: U64, z: U64) -> U64 {
    xor64(xor64(and64(x, y), and64(x, z)), and64(y, z))
}
#[cfg(target_arch = "avr")]
fn bsig0(x: U64) -> U64 {
    xor64(xor64(rotr64(x, 28), rotr64(x, 34)), rotr64(x, 39))
}
#[cfg(target_arch = "avr")]
fn bsig1(x: U64) -> U64 {
    xor64(xor64(rotr64(x, 14), rotr64(x, 18)), rotr64(x, 41))
}
#[cfg(target_arch = "avr")]
fn ssig0(x: U64) -> U64 {
    xor64(xor64(rotr64(x, 1), rotr64(x, 8)), shr64(x, 7))
}
#[cfg(target_arch = "avr")]
fn ssig1(x: U64) -> U64 {
    xor64(xor64(rotr64(x, 19), rotr64(x, 61)), shr64(x, 6))
}

#[cfg(target_arch = "avr")]
fn load_be64(p: &[u8]) -> U64 {
    U64 {
        hi: u32::from_be_bytes([p[0], p[1], p[2], p[3]]),
        lo: u32::from_be_bytes([p[4], p[5], p[6], p[7]]),
    }
}

#[cfg(target_arch = "avr")]
fn k_u64(t: u8) -> U64 {
    unsafe extern "C" {
        fn sha512_k_load_halves(i: u8, lo: *mut u32, hi: *mut u32);
    }
    let mut lo = 0u32;
    let mut hi = 0u32;
    unsafe { sha512_k_load_halves(t, &mut lo, &mut hi) }
    U64 { lo, hi }
}

#[cfg(target_arch = "avr")]
static mut COMP_W: [U64; 16] = [U64 { lo: 0, hi: 0 }; 16];
#[cfg(target_arch = "avr")]
static mut COMP_S: [U64; 8] = [U64 { lo: 0, hi: 0 }; 8];

#[cfg(target_arch = "avr")]
#[inline(never)]
fn sha512_compress_block(state: &mut [u64; 8], block: &[u8; 128]) {
    let w = unsafe { &mut *(&raw mut COMP_W) };
    let s = unsafe { &mut *(&raw mut COMP_S) };
    for t in 0..16 {
        w[t] = load_be64(&block[t * 8..t * 8 + 8]);
    }
    for t in 0..8 {
        let v = state[t];
        s[t] = U64 {
            lo: v as u32,
            hi: (v >> 32) as u32,
        };
    }
    for t in 0..80u8 {
        let wt = if t < 16 {
            w[t as usize]
        } else {
            let wt = add64(
                add64(ssig1(w[((t as usize) - 2) & 15]), w[((t as usize) - 7) & 15]),
                add64(ssig0(w[((t as usize) - 15) & 15]), w[(t as usize) & 15]),
            );
            w[(t as usize) & 15] = wt;
            wt
        };
        let a = s[0];
        let b = s[1];
        let c = s[2];
        let d = s[3];
        let e = s[4];
        let f = s[5];
        let g = s[6];
        let h = s[7];
        let t1 = add64(add64(add64(add64(h, bsig1(e)), ch(e, f, g)), k_u64(t)), wt);
        let t2 = add64(bsig0(a), maj(a, b, c));
        s[0] = add64(t1, t2);
        s[1] = a;
        s[2] = b;
        s[3] = c;
        s[4] = add64(d, t1);
        s[5] = e;
        s[6] = f;
        s[7] = g;
    }
    for t in 0..8 {
        let v = state[t];
        let st = add64(
            U64 {
                lo: v as u32,
                hi: (v >> 32) as u32,
            },
            s[t],
        );
        state[t] = (st.hi as u64) << 32 | (st.lo as u64);
    }
}

// Not inlined, so callers share one copy of the loop in the cache.
#[cfg(not(target_arch = "avr"))]
#[inline(never)]
fn sha512_compress_block(state: &mut [u64; 8], block: &[u8; 128]) {
    let mut s = *state;

    // A 16-word window into the whole W array, indexed mod 16.
    let mut w = [0u64; 16];
    let (words, _) = block.as_chunks::<8>();
    for (w, word) in w.iter_mut().zip(words) {
        *w = u64::from_be_bytes(*word);
    }

    for t in 0..80 {
        let w_t = if t < 16 {
            w[t]
        } else {
            // W[t] = SSIG1(W[t - 2]) + W[t - 7] + SSIG0(W[t - 15]) + W[t - 16]
            let w_t = SSIG1!(w[(t - 2) % 16])
                .wrapping_add(w[(t - 7) % 16])
                .wrapping_add(SSIG0!(w[(t - 15) % 16]))
                .wrapping_add(w[t % 16]);
            w[t % 16] = w_t;
            w_t
        };

        let [a, b, c, d, e, f, g, h] = s;
        let t1 = h
            .wrapping_add(BSIG1!(e))
            .wrapping_add(CH!(e, f, g))
            .wrapping_add(k_at(t))
            .wrapping_add(w_t);
        let t2 = BSIG0!(a).wrapping_add(MAJ!(a, b, c));
        s = [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g];
    }

    for (h, s) in state.iter_mut().zip(s) {
        *h = h.wrapping_add(s);
    }
}

/// Compresses whole 128-byte `blocks` into `state`.
pub fn sha512_compress_blocks(state: &mut [u64; 8], blocks: &[u8]) {
    assert!(blocks.len().is_multiple_of(128));

    let (chunks, _) = blocks.as_chunks::<128>();
    for block in chunks {
        sha512_compress_block(state, block);
    }
}


#[cfg(not(target_arch = "avr"))]
fn k_at(t: usize) -> u64 {
    K[t]
}

#[cfg(target_arch = "avr")]
fn k_at(t: usize) -> u64 {
    unsafe extern "C" {
        fn sha512_k_load(i: u8) -> u64;
    }
    unsafe { sha512_k_load(t as u8) }
}

#[cfg(not(target_arch = "avr"))]
static K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

