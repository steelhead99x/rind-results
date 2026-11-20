// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

//! Local known-answer checks for the ESP32-S3 validation of #94 and #87. Not pushed.

extern crate alloc;
use alloc::{format, string::String, vec, vec::Vec};

#[allow(unused_imports)]
use crate::vectors::{hex, report};

pub fn run(f: &mut Vec<(String, String)>) {
    let _ = &f;
    #[cfg(feature = "sha2")]
    sha_fips(f);
    #[cfg(feature = "poly1305")]
    poly_diff(f);
}

#[cfg(feature = "sha2")]
fn sha_fips(f: &mut Vec<(String, String)>) {
    use rind::digest::{self, Context};
    let p = ["kat/"];
    let long = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
    let sha512 = [
        ("empty", &b""[..], "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"),
        ("abc", &b"abc"[..], "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"),
        ("896bit", &long[..], "8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909"),
    ];
    for (name, msg, want) in sha512 {
        let want = hex(want);
        let got = digest::digest(&digest::SHA512, msg);
        report(f, &format!("sha512/fips180-4/{name}"), &p, "rind", got.as_ref() == &want[..]);
        // Every message word at byte offsets 1, 2 and 3 (unaligned l32i).
        for off in 1..4usize {
            let mut buf = vec![0xeeu8; msg.len() + 8];
            buf[off..off + msg.len()].copy_from_slice(msg);
            let got = digest::digest(&digest::SHA512, &buf[off..off + msg.len()]);
            report(f, &format!("sha512/fips180-4/{name}/offset{off}"), &p, "rind", got.as_ref() == &want[..]);
        }
        // One byte at a time through the streaming context.
        let mut c = Context::new(&digest::SHA512);
        for b in msg {
            c.update(core::slice::from_ref(b));
        }
        report(f, &format!("sha512/fips180-4/{name}/bytewise"), &p, "rind", c.finish().as_ref() == &want[..]);
    }
    // FIPS 180-4's one-million-'a' message, 1000 B at a time.
    let mut c = Context::new(&digest::SHA512);
    let chunk = [b'a'; 1000];
    for _ in 0..1000 {
        c.update(&chunk);
    }
    let want = hex("e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b");
    report(f, "sha512/fips180-4/million-a", &p, "rind", c.finish().as_ref() == &want[..]);
    let want = hex("cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7");
    report(f, "sha384/fips180-4/abc", &p, "rind", digest::digest(&digest::SHA384, b"abc").as_ref() == &want[..]);
    let want = hex("53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23");
    report(f, "sha512-256/fips180-4/abc", &p, "rind", digest::digest(&digest::SHA512_256, b"abc").as_ref() == &want[..]);
    for (name, key, msg, want) in [
        ("rfc4231-tc1", vec![0x0bu8; 20], &b"Hi There"[..], "afd03944d84895626b0825f4ab46907f15f9dadbe4101ec682aa034c7cebc59cfaea9ea9076ede7f4af152e8b2fa9cb6"),
        ("rfc4231-tc2", b"Jefe".to_vec(), &b"what do ya want for nothing?"[..], "af45d2e376484031617f78d2b58a6b1b9c7ef464f5a01b47e42ec3736322445e8e2240ca5e69e2c78b3239ecfab21649"),
    ] {
        let want = hex(want);
        let k = rind::hmac::Key::new(&rind::hmac::HMAC_SHA384, &key);
        report(f, &format!("hmac-sha384/{name}"), &p, "rind", rind::hmac::sign(&k, msg).as_ref() == &want[..]);
    }
}

/// One library's ChaCha20-Poly1305 tag, sealing in `buf` (a reused buffer), so that no more
/// than one copy of a 16 KB message is live: 0 rind, 1 ring, 2 RustCrypto, 3 libcrux.
#[cfg(feature = "poly1305")]
fn tag_of(lib: usize, key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], pt: &[u8], buf: &mut Vec<u8>) -> [u8; 16] {
    buf.clear();
    buf.extend_from_slice(pt);
    let mut out = [0u8; 16];
    match lib {
        0 => {
            let k = rind::aead::Key::new(&rind::aead::CHACHA20_POLY1305, key).unwrap();
            let t = k
                .seal_in_place_separate_tag(rind::aead::Nonce::assume_unique_for_key(*nonce), aad, buf)
                .unwrap();
            out.copy_from_slice(t.as_ref());
        }
        1 => {
            let k = ring::aead::LessSafeKey::new(
                ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, key).unwrap(),
            );
            let t = k
                .seal_in_place_separate_tag(
                    ring::aead::Nonce::assume_unique_for_key(*nonce),
                    ring::aead::Aad::from(aad),
                    buf,
                )
                .unwrap();
            out.copy_from_slice(t.as_ref());
        }
        2 => {
            use chacha20poly1305::{AeadInOut, KeyInit};
            let k = chacha20poly1305::ChaCha20Poly1305::new_from_slice(key).unwrap();
            let t = k
                .encrypt_inout_detached(nonce.into(), aad, buf.as_mut_slice().into())
                .unwrap();
            out.copy_from_slice(t.as_slice());
        }
        _ => {
            let mut ct = vec![0u8; pt.len()];
            libcrux_chacha20poly1305::encrypt_detached(key, pt, &mut ct, &mut out, aad, nonce).unwrap();
            buf.clear();
            buf.extend_from_slice(&ct);
        }
    }
    out
}

/// rind's ChaCha20-Poly1305 against ring (C Poly1305), RustCrypto and libcrux over every
/// length up to 300 B and 1200, 1350, 8192 and 16384 B, as a MAC over the AAD alone (empty
/// message) and as a seal. The tag covers the ciphertext, so equal tags mean equal output.
#[cfg(feature = "poly1305")]
fn poly_diff(f: &mut Vec<(String, String)>) {
    let p = ["kat/"];
    let mut s = 0x9e3779b97f4a7c15u64;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut bad = [[0u32; 2]; 3]; // ring, rustcrypto, libcrux x (mac-only, seal)
    let mut cases = 0u32;
    let mut buf = Vec::new();
    for n in (0..=300usize).chain([1200, 1350, 8192, 16384]) {
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 12];
        for b in key.iter_mut().chain(nonce.iter_mut()) {
            *b = next() as u8;
        }
        let msg: Vec<u8> = (0..n).map(|_| next() as u8).collect();
        for (mode, (aad, pt)) in [(&msg[..], &[][..]), (&msg[..n.min(13)], &msg[..])].into_iter().enumerate() {
            let want = tag_of(0, &key, &nonce, aad, pt, &mut buf);
            let want_ct = buf.clone();
            cases += 1;
            for lib in 1..4 {
                let got = tag_of(lib, &key, &nonce, aad, pt, &mut buf);
                if got != want || buf != want_ct {
                    bad[lib - 1][mode] += 1;
                }
            }
        }
    }
    for (i, lib) in ["ring", "rustcrypto", "libcrux"].into_iter().enumerate() {
        report(f, &format!("poly1305/aad-only-vs-rind/{}-cases", cases / 2), &p, lib, bad[i][0] == 0);
        report(f, &format!("poly1305/seal-vs-rind/{}-cases", cases / 2), &p, lib, bad[i][1] == 0);
    }
}
