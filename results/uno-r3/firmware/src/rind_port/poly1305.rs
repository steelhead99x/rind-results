// Adapted for the Arduino Uno R3 (ATmega328P) bench.
// Source: steelhead99x/rind @ c08c91ea (origin/rp2040/armv6m-no-std).
// Changes: pub(crate) -> pub, tests stripped, K tables loaded from PROGMEM
// on AVR, Poly1305 Blockwise methods left enabled (originally cfg'd to
// 32/64-bit pointer widths). Algorithm is otherwise rind's.
// Written for Graviola by Joe Birr-Pixton, 2024.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
// Originally from cifra, but later adopting the 64x64
// multiplication layout from aws-lc.

use super::blockwise::Blockwise;

pub struct Poly1305 {
    /// Current accumulator, 131 bits split into limbs of 64, 64, and 3 bits
    h: [u64; 3],

    /// First half of key: block multiplier
    r: [u64; 2],

    /// Second half of key: added to result
    s: [u64; 2],

    /// Unprocessed input
    bw: Blockwise<16>,
}

impl Poly1305 {
    pub fn new(key: &[u8; 32]) -> Self {
        let h = [0; 3];
        // Clamp r by ANDing it with 0x0ffffffc0ffffffc0ffffffc0fffffff (RFC 8439 section 2.5.1)
        let r = [
            read64(&key[0..8]) & 0x0ffffffc0fffffff,
            read64(&key[8..16]) & 0x0ffffffc0ffffffc,
        ];
        let s = [read64(&key[16..24]), read64(&key[24..32])];
        Self {
            h,
            r,
            s,
            bw: Blockwise::new(),
        }
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) {
        let bytes = self.bw.add_leading(bytes);

        if let Some(block) = self.bw.take() {
            self.process_whole_block(&block);
        }

        let (full_blocks, remainder) = bytes.as_chunks::<16>();
        for block in full_blocks {
            self.process_whole_block(block);
        }

        self.bw.add_trailing(remainder);
    }

    pub fn finish(mut self) -> [u8; 16] {
        if let Some(block) = self.bw.clone().peek_remaining() {
            self.process_last_block(block);
        }

        full_reduce(&mut self.h);

        // add s with carry
        let prev = self.h[0];
        self.h[0] = self.h[0].wrapping_add(self.s[0]);
        let carry = (self.h[0] < prev) as u64;
        self.h[1] = self.h[1].wrapping_add(self.s[1]).wrapping_add(carry);

        let mut r = [0u8; 16];
        r[0..8].copy_from_slice(&self.h[0].to_le_bytes());
        r[8..16].copy_from_slice(&self.h[1].to_le_bytes());
        r
    }

    fn process_whole_block(&mut self, inp: &[u8; 16]) {
        let block = [read64(&inp[0..8]), read64(&inp[8..16]), 0x01];
        self.process_block(&block);
    }

    fn process_last_block(&mut self, inp: &[u8]) {
        let mut bytes = [0u8; 16];
        bytes[..inp.len()].copy_from_slice(inp);
        bytes[inp.len()] = 0x01;

        let block = [read64(&bytes[0..8]), read64(&bytes[8..16]), 0];
        self.process_block(&block);
    }

    fn process_block(&mut self, block: &[u64; 3]) {
        add(&mut self.h, block);
        mul(&mut self.h, &self.r);
    }
}

fn read64(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes.try_into().unwrap())
}

fn add(h: &mut [u64; 3], x: &[u64; 3]) {
    // Add h += x, with carry. The generated code is a little less efficient
    // than hand-written assembly, but this is portable.
    // FIXME: try u64::carrying_add once the MSRV >= 1.91
    let carry: bool;
    (h[0], carry) = h[0].overflowing_add(x[0]);
    let (carry1, carry2): (bool, bool);
    (h[1], carry1) = h[1].overflowing_add(carry as u64);
    (h[1], carry2) = h[1].overflowing_add(x[1]);
    h[2] = h[2].wrapping_add((carry1 | carry2) as u64);
    h[2] = h[2].wrapping_add(x[2]);
}

fn mul(h: &mut [u64; 3], r: &[u64; 2]) {
    fn mul64(a: u64, b: u64) -> (u64, u64) {
        let product = (a as u128) * (b as u128);
        (product as u64, (product >> 64) as u64)
    }

    // Multiply t = r * h
    // The following implementation depends on these invariants to ensure that
    // certain 64-bit addition and multiplication results fit in a u64:
    //  * h uses at most 131 bits, and thus h[2] uses at most 3 bits
    //  * r[0] uses at most 60 bits
    //  * r[1] uses at most 60 bits
    // Hence wrapping arithmetic: a debug build's overflow check would branch on
    // the secret, as would these assertions without declassifying.
    debug_assert!(h[2] < 1 << 3);
    debug_assert!(r[0] < 1 << 60);
    debug_assert!(r[1] < 1 << 60);

    //         h[2] h[1] h[0]
    // x                 r[0]
    // ----------------------
    //               d1   d0
    //          d3   d2
    //          d4
    // ----------------------
    //          e2   e1   e0
    let (d0, d1) = mul64(r[0], h[0]);
    let (d2, d3) = mul64(r[0], h[1]);
    let d4 = r[0].wrapping_mul(h[2]);
    let e0 = d0;
    let (e1, carry) = d1.overflowing_add(d2);
    let e2 = d3.wrapping_add(d4).wrapping_add(carry as u64);

    //         h[2] h[1] h[0]
    // x            r[1]
    // ----------------------
    //          d1   d0
    //     d3   d2
    //     d4
    // ----------------------
    //     s5   e4   e3
    let (d0, d1) = mul64(r[1], h[0]);
    let (d2, d3) = mul64(r[1], h[1]);
    let d4 = r[1].wrapping_mul(h[2]);
    let e3 = d0;
    let (e4, carry) = d1.overflowing_add(d2);
    let e5 = d3.wrapping_add(d4).wrapping_add(carry as u64);

    //          e2   e1   e0
    // +   e5   e4   e3
    // ----------------------
    //     f3   f2   f1   f0
    let f0 = e0;
    let (f1, carry) = e1.overflowing_add(e3);
    let (f2, carry1) = e2.overflowing_add(carry as u64);
    let (f2, carry2) = f2.overflowing_add(e4);
    let f3 = e5.wrapping_add((carry1 | carry2) as u64);
    debug_assert!(f3 < 1u64 << 63);

    // Partially reduce the result so it fits within 131 bits
    let (carry1, carry2): (bool, bool);
    (h[0], carry1) = f0.overflowing_add(f2 & !0x03);
    (h[0], carry2) = h[0].overflowing_add((f3 << 62) | (f2 >> 2));
    let carry3: bool;
    (h[1], carry3) = f1.overflowing_add(
        f3.wrapping_add(f3 >> 2)
            .wrapping_add(carry1 as u64)
            .wrapping_add(carry2 as u64),
    );
    h[2] = (f2 & 0x03).wrapping_add(carry3 as u64);
}

fn full_reduce(h: &mut [u64; 3]) {
    let (g0, carry0) = h[0].overflowing_sub(0u64.wrapping_sub(5));
    let (g1, carry1a) = h[1].overflowing_sub(carry0 as u64);
    let (g1, carry1b) = g1.overflowing_sub(0u64.wrapping_sub(1));
    let (g2, carry2a) = h[2].overflowing_sub((carry1a as u64).wrapping_add(carry1b as u64));
    let (_, carry2b) = g2.overflowing_sub(3);

    let positive_mask = ((carry2a | carry2b) as u64).wrapping_sub(1);
    let negative_mask = !positive_mask;
    h[0] = (h[0] & negative_mask) | (g0 & positive_mask);
    h[1] = (h[1] & negative_mask) | (g1 & positive_mask);
    // `finish` reads only the low 128 bits, so `h[2]` is left as it is.
}

