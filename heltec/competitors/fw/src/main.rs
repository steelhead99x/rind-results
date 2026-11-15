//! rind vs ring / RustCrypto+dalek / libcrux on one ESP32-S3 core.
//! Same harness for every library: esp_timer_get_time (us), warm-up of
//! iters/10 (min 1) untimed calls, then `iters` timed calls.
//! Inputs follow rind-bench: zero keys and nonces, 32 B zero AAD,
//! zero-filled messages, its X25519/P-256 peer keys and its verify vectors.
#![allow(unused_imports, dead_code, unused_variables)]
use std::hint::black_box;

fn now_us() -> i64 {
    unsafe { esp_idf_svc::sys::esp_timer_get_time() }
}

fn bench<F: FnMut()>(lib: &str, op: &str, iters: u32, mut f: F) {
    for _ in 0..(iters / 10).max(1) {
        f();
    }
    let t0 = now_us();
    for _ in 0..iters {
        f();
    }
    let dt = now_us() - t0;
    println!(
        "BENCH,{lib},{op},iters={iters},total_us={dt},per_op_us={:.2}",
        dt as f64 / iters as f64
    );
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn check(lib: &str, op: &str, ok: bool, out: &str) {
    println!("CHECK,{lib},{op},ok={ok},out={out}");
}

const AEAD_SIZES: [usize; 3] = [32, 1024, 1350];
const HASH_SIZES: [usize; 3] = [64, 1024, 1350];
const AAD: [u8; 32] = [0u8; 32];
const NONCE: [u8; 12] = [0u8; 12];
const K16: [u8; 16] = [0u8; 16];
const K32: [u8; 32] = [0u8; 32];
fn aead_iters(n: usize) -> u32 { if n <= 64 { 1000 } else { 200 } }
fn hash_iters(n: usize) -> u32 { if n <= 64 { 1000 } else { 200 } }
const ASYM_ITERS: u32 = 20;

const SHA256_ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
const SHA512_ABC: &str = "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f";

// rind-bench/benches/x25519.rs
const X25519_PEER: &[u8; 32] = b"\xe6\xdb\x68\x67\x58\x30\x30\xdb\x35\x94\xc1\xa4\x24\xb1\x5f\x7c\x72\x66\x24\xec\x26\xb3\x35\x3b\x10\xa9\x03\xa6\xd0\xab\x1c\x4c";
// rind-bench/benches/p256.rs
const P256_PEER: &[u8; 65] = b"\x04\
\x62\xd5\xbd\x33\x72\xaf\x75\xfe\x85\xa0\x40\x71\x5d\x0f\x50\x24\x28\xe0\x70\x46\x86\x8b\x0b\xfd\xfa\x61\xd7\x31\xaf\xe4\x4f\x26\
\xac\x33\x3a\x93\xa9\xe7\x0a\x81\xcd\x5a\x95\xb5\xbf\x8d\x13\x99\x0e\xb7\x41\xc8\xc3\x88\x72\xb4\xa0\x7d\x27\x5a\x01\x4e\x30\xcf";
// rind-bench/benches/p256.rs ecdsa_verify vector
const ECDSA_PK: &[u8; 65] = b"\x04\
\x29\x27\xb1\x05\x12\xba\xe3\xed\xdc\xfe\x46\x78\x28\x12\x8b\xad\x29\x03\x26\x99\x19\xf7\x08\x60\x69\xc8\xc4\xdf\x6c\x73\x28\x38\
\xc7\x78\x79\x64\xea\xac\x00\xe5\x92\x1f\xb1\x49\x8a\x60\xf4\x60\x67\x66\xb3\xd9\x68\x50\x01\x55\x8d\x1a\x97\x4e\x73\x41\x51\x3e";
const ECDSA_MSG: &[u8; 6] = b"\x31\x32\x33\x34\x30\x30";
const ECDSA_SIG: &[u8; 64] = b"\x2b\xa3\xa8\xbe\x6b\x94\xd5\xec\x80\xa6\xd9\xd1\x19\x0a\x43\x6e\xff\xe5\x0d\x85\xa1\xee\xe8\x59\xb8\xcc\x6a\xf9\xbd\x5c\x2e\x18\x4c\xd6\x0b\x85\x5d\x44\x2f\x5b\x3c\x7b\x11\xeb\x6c\x4e\x0a\xe7\x52\x5f\xe7\x10\xfa\xb9\xaa\x7c\x77\xa6\x7f\x79\xe6\xfa\xdd\x76";
// rind-bench/benches/ed25519.rs verify vector
const ED_PK: &[u8; 32] = b"\x7d\x4d\x0e\x7f\x61\x53\xa6\x9b\x62\x42\xb5\x22\xab\xbe\xe6\x85\xfd\xa4\x42\x0f\x88\x34\xb1\x08\xc3\xbd\xae\x36\x9e\xf5\x49\xfa";
const ED_MSG: &[u8; 4] = b"\x54\x65\x73\x74";
const ED_SIG: &[u8; 64] = b"\x7c\x38\xe0\x26\xf2\x9e\x14\xaa\xbd\x05\x9a\x0f\x2d\xb8\xb0\xcd\x78\x30\x40\x60\x9a\x8b\xe6\x84\xdb\x12\xf8\x2a\x27\x77\x4a\xb0\x7a\x91\x55\x71\x1e\xcf\xaf\x7f\x99\xf2\x77\xba\xd0\xc6\xae\x7e\x39\xd4\xee\xf6\x76\x57\x33\x36\xa5\xc5\x1e\xb6\xf9\x46\xb3\x0d";
// Ed25519 sign: a fixed seed, so every library's signature must match byte for byte.
const ED_SEED: [u8; 32] = [0x42u8; 32];

fn bad_sig<const N: usize>(s: &[u8; N]) -> [u8; N] {
    let mut b = *s;
    b[N - 1] ^= 1;
    b
}

#[cfg(feature = "rind")]
fn run_rind() {
    const L: &str = "rind";
    use rind::aead::{AES_128_GCM, CHACHA20_POLY1305, Key, Nonce};
    use rind::digest::{Context, SHA256, SHA512, digest};
    for (alg, name, key) in [(&AES_128_GCM, "aes128gcm_seal", &K16[..]), (&CHACHA20_POLY1305, "chacha20poly1305_seal", &K32[..])] {
        let k = Key::new(alg, key).unwrap();
        for n in AEAD_SIZES {
            let op = format!("{name}_{n}B");
            let mut m = vec![0u8; n];
            let tag = k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), &AAD, &mut m).unwrap();
            let ct = m.clone();
            let ok = k.open_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), &AAD, tag, &mut m).is_ok() && m.iter().all(|&b| b == 0);
            check(L, &op, ok, &format!("tag={},ct8={}", hex(tag.as_ref()), hex(&ct[..8])));
            let mut buf = vec![0u8; n];
            bench(L, &op, aead_iters(n), || {
                black_box(k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), &AAD, &mut buf).unwrap());
            });
        }
    }
    for (alg, name, kat) in [(&SHA256, "sha256", SHA256_ABC), (&SHA512, "sha512", SHA512_ABC)] {
        let abc = hex(digest(alg, b"abc").as_ref());
        for n in HASH_SIZES {
            let op = format!("{name}_{n}B");
            let data = vec![0u8; n];
            check(L, &op, abc == kat, &format!("abc_kat={},zeros={}", abc == kat, hex(digest(alg, &data).as_ref())));
            bench(L, &op, hash_iters(n), || {
                let mut c = Context::new(alg);
                c.update(black_box(&data));
                black_box(c.finish());
            });
        }
    }
    {
        use rind::agreement::{ECDH_P256, PrivateKey, X25519};
        for (alg, op, peer) in [(&X25519, "x25519_keygen_agree", &X25519_PEER[..]), (&ECDH_P256, "p256_ecdh_keygen_agree", &P256_PEER[..])] {
            let a = PrivateKey::generate(alg).unwrap();
            let b = PrivateKey::generate(alg).unwrap();
            let s1 = a.agree(b.public_key().as_ref()).unwrap();
            let s2 = b.agree(a.public_key().as_ref()).unwrap();
            check(L, op, s1.as_ref() == s2.as_ref(), "symmetric");
            bench(L, op, ASYM_ITERS, || {
                let k = PrivateKey::generate(alg).unwrap();
                black_box(k.public_key());
                black_box(k.agree(peer).unwrap());
            });
        }
    }
    {
        use rind::signature::{EcdsaKeyPair, Ed25519KeyPair, PublicKey, sign, verify};
        let kp = Ed25519KeyPair::from_seed(&ED_SEED).unwrap();
        let sig = kp.sign(ECDSA_MSG);
        check(L, "ed25519_sign", PublicKey::new(&verify::ED25519, kp.public_key().as_ref()).unwrap().verify(ECDSA_MSG, sig.as_ref()).is_ok(), &format!("sig={}", hex(sig.as_ref())));
        bench(L, "ed25519_sign", ASYM_ITERS, || { black_box(kp.sign(ECDSA_MSG)); });
        let pk = PublicKey::new(&verify::ED25519, ED_PK).unwrap();
        check(L, "ed25519_verify", pk.verify(ED_MSG, ED_SIG).is_ok() && pk.verify(ED_MSG, &bad_sig(ED_SIG)).is_err(), "vector_ok+tamper_rejected");
        bench(L, "ed25519_verify", ASYM_ITERS, || { pk.verify(ED_MSG, ED_SIG).unwrap(); });

        let kp = EcdsaKeyPair::generate(&sign::ECDSA_P256_SHA256_FIXED).unwrap();
        let sig = kp.sign(ECDSA_MSG).unwrap();
        check(L, "p256_ecdsa_sign", PublicKey::new(&verify::ECDSA_P256_SHA256_FIXED, kp.public_key().as_ref()).unwrap().verify(ECDSA_MSG, sig.as_ref()).is_ok(), "own_verify");
        bench(L, "p256_ecdsa_sign", ASYM_ITERS, || { black_box(kp.sign(ECDSA_MSG).unwrap()); });
        let good = PublicKey::new(&verify::ECDSA_P256_SHA256_FIXED, ECDSA_PK).unwrap().verify(ECDSA_MSG, ECDSA_SIG).is_ok();
        let bad = PublicKey::new(&verify::ECDSA_P256_SHA256_FIXED, ECDSA_PK).unwrap().verify(ECDSA_MSG, &bad_sig(ECDSA_SIG)).is_err();
        check(L, "p256_ecdsa_verify", good && bad, "vector_ok+tamper_rejected");
        bench(L, "p256_ecdsa_verify", ASYM_ITERS, || {
            PublicKey::new(&verify::ECDSA_P256_SHA256_FIXED, ECDSA_PK).unwrap().verify(ECDSA_MSG, ECDSA_SIG).unwrap();
        });
    }
}

#[cfg(feature = "ring")]
fn ring_chacha_large() {
    use ring::aead::{Aad, CHACHA20_POLY1305, LessSafeKey, Nonce, UnboundKey};
    let k = LessSafeKey::new(UnboundKey::new(&CHACHA20_POLY1305, &K32).unwrap());
    for n in [32usize, 1350, 1024] {
        let op = format!("chacha20poly1305_seal_{n}B");
        println!("TRY,ring,{op}");
        let mut m = vec![0u8; n];
        let tag = k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), Aad::from(&AAD), &mut m).unwrap();
        let ct = m.clone();
        let ok = k.open_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), Aad::from(&AAD), tag, &mut m, 0..).is_ok() && m.iter().all(|&b| b == 0);
        check("ring", &op, ok, &format!("tag={},ct8={}", hex(tag.as_ref()), hex(&ct[..8])));
        let mut buf = vec![0u8; n];
        bench("ring", &op, aead_iters(n), || {
            black_box(k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), Aad::from(&AAD), &mut buf).unwrap());
        });
    }
}

#[cfg(feature = "ring")]
fn run_ring() {
    const L: &str = "ring";
    use ring::aead::{AES_128_GCM, Aad, CHACHA20_POLY1305, LessSafeKey, Nonce, UnboundKey};
    use ring::digest::{Context, SHA256, SHA512, digest};
    use ring::rand::SystemRandom;
    let rng = SystemRandom::new();
    for (alg, name, key) in [(&AES_128_GCM, "aes128gcm_seal", &K16[..]), (&CHACHA20_POLY1305, "chacha20poly1305_seal", &K32[..])] {
        let k = LessSafeKey::new(UnboundKey::new(alg, key).unwrap());
        for n in AEAD_SIZES {
            // ring's ChaCha20-Poly1305 reset the chip (RTCWDT_RTC_RST) in runs 1-2;
            // it runs last, in ring_chacha_large(), so ring's other ops get measured.
            if name.starts_with("chacha") { continue; }
            let op = format!("{name}_{n}B");
            let mut m = vec![0u8; n];
            let tag = k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), Aad::from(&AAD), &mut m).unwrap();
            let ct = m.clone();
            let ok = k.open_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), Aad::from(&AAD), tag, &mut m, 0..).is_ok() && m.iter().all(|&b| b == 0);
            check(L, &op, ok, &format!("tag={},ct8={}", hex(tag.as_ref()), hex(&ct[..8])));
            let mut buf = vec![0u8; n];
            bench(L, &op, aead_iters(n), || {
                black_box(k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(NONCE), Aad::from(&AAD), &mut buf).unwrap());
            });
        }
    }
    for (alg, name, kat) in [(&SHA256, "sha256", SHA256_ABC), (&SHA512, "sha512", SHA512_ABC)] {
        let abc = hex(digest(alg, b"abc").as_ref());
        for n in HASH_SIZES {
            let op = format!("{name}_{n}B");
            let data = vec![0u8; n];
            check(L, &op, abc == kat, &format!("abc_kat={},zeros={}", abc == kat, hex(digest(alg, &data).as_ref())));
            bench(L, &op, hash_iters(n), || {
                let mut c = Context::new(alg);
                c.update(black_box(&data));
                black_box(c.finish());
            });
        }
    }
    {
        use ring::agreement::{ECDH_P256, EphemeralPrivateKey, UnparsedPublicKey, X25519, agree_ephemeral};
        for (alg, op, peer) in [(&X25519, "x25519_keygen_agree", &X25519_PEER[..]), (&ECDH_P256, "p256_ecdh_keygen_agree", &P256_PEER[..])] {
            let a = EphemeralPrivateKey::generate(alg, &rng).unwrap();
            let b = EphemeralPrivateKey::generate(alg, &rng).unwrap();
            let pa = a.compute_public_key().unwrap();
            let pb = b.compute_public_key().unwrap();
            let s1 = agree_ephemeral(a, &UnparsedPublicKey::new(alg, pb.as_ref()), |k| k.to_vec()).unwrap();
            let s2 = agree_ephemeral(b, &UnparsedPublicKey::new(alg, pa.as_ref()), |k| k.to_vec()).unwrap();
            check(L, op, s1 == s2, "symmetric");
            bench(L, op, ASYM_ITERS, || {
                let k = EphemeralPrivateKey::generate(alg, &rng).unwrap();
                black_box(k.compute_public_key().unwrap());
                agree_ephemeral(k, &UnparsedPublicKey::new(alg, peer), |m| { black_box(m); }).unwrap();
            });
        }
    }
    {
        use ring::signature::{ECDSA_P256_SHA256_FIXED, ECDSA_P256_SHA256_FIXED_SIGNING, ED25519, EcdsaKeyPair, Ed25519KeyPair, KeyPair, UnparsedPublicKey};
        let kp = Ed25519KeyPair::from_seed_unchecked(&ED_SEED).unwrap();
        let sig = kp.sign(ECDSA_MSG);
        check(L, "ed25519_sign", UnparsedPublicKey::new(&ED25519, kp.public_key().as_ref()).verify(ECDSA_MSG, sig.as_ref()).is_ok(), &format!("sig={}", hex(sig.as_ref())));
        bench(L, "ed25519_sign", ASYM_ITERS, || { black_box(kp.sign(ECDSA_MSG)); });
        let pk = UnparsedPublicKey::new(&ED25519, ED_PK);
        check(L, "ed25519_verify", pk.verify(ED_MSG, ED_SIG).is_ok() && pk.verify(ED_MSG, &bad_sig(ED_SIG)).is_err(), "vector_ok+tamper_rejected");
        bench(L, "ed25519_verify", ASYM_ITERS, || { pk.verify(ED_MSG, ED_SIG).unwrap(); });

        let doc = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
        let kp = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, doc.as_ref(), &rng).unwrap();
        let sig = kp.sign(&rng, ECDSA_MSG).unwrap();
        check(L, "p256_ecdsa_sign", UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, kp.public_key().as_ref()).verify(ECDSA_MSG, sig.as_ref()).is_ok(), "own_verify");
        bench(L, "p256_ecdsa_sign", ASYM_ITERS, || { black_box(kp.sign(&rng, ECDSA_MSG).unwrap()); });
        let pk = UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, ECDSA_PK);
        check(L, "p256_ecdsa_verify", pk.verify(ECDSA_MSG, ECDSA_SIG).is_ok() && pk.verify(ECDSA_MSG, &bad_sig(ECDSA_SIG)).is_err(), "vector_ok+tamper_rejected");
        bench(L, "p256_ecdsa_verify", ASYM_ITERS, || { pk.verify(ECDSA_MSG, ECDSA_SIG).unwrap(); });
    }
}

#[cfg(feature = "rustcrypto")]
fn run_rustcrypto() {
    const L: &str = "rustcrypto";
    {
        use aes_gcm::{AeadInOut, Aes128Gcm, KeyInit};
        let k = Aes128Gcm::new_from_slice(&K16).unwrap();
        for n in AEAD_SIZES {
            let op = format!("aes128gcm_seal_{n}B");
            let mut m = vec![0u8; n];
            let tag = k.encrypt_inout_detached((&NONCE).into(), &AAD, m.as_mut_slice().into()).unwrap();
            let ct = m.clone();
            let ok = k.decrypt_inout_detached((&NONCE).into(), &AAD, m.as_mut_slice().into(), &tag).is_ok() && m.iter().all(|&b| b == 0);
            check(L, &op, ok, &format!("tag={},ct8={}", hex(tag.as_slice()), hex(&ct[..8])));
            let mut buf = vec![0u8; n];
            bench(L, &op, aead_iters(n), || {
                black_box(k.encrypt_inout_detached((&NONCE).into(), &AAD, buf.as_mut_slice().into()).unwrap());
            });
        }
    }
    {
        use chacha20poly1305::{AeadInOut, ChaCha20Poly1305, KeyInit};
        let k = ChaCha20Poly1305::new_from_slice(&K32).unwrap();
        for n in AEAD_SIZES {
            let op = format!("chacha20poly1305_seal_{n}B");
            let mut m = vec![0u8; n];
            let tag = k.encrypt_inout_detached((&NONCE).into(), &AAD, m.as_mut_slice().into()).unwrap();
            let ct = m.clone();
            let ok = k.decrypt_inout_detached((&NONCE).into(), &AAD, m.as_mut_slice().into(), &tag).is_ok() && m.iter().all(|&b| b == 0);
            check(L, &op, ok, &format!("tag={},ct8={}", hex(tag.as_slice()), hex(&ct[..8])));
            let mut buf = vec![0u8; n];
            bench(L, &op, aead_iters(n), || {
                black_box(k.encrypt_inout_detached((&NONCE).into(), &AAD, buf.as_mut_slice().into()).unwrap());
            });
        }
    }
    {
        use sha2::{Digest, Sha256, Sha512};
        let abc = hex(&Sha256::digest(b"abc"));
        for n in HASH_SIZES {
            let op = format!("sha256_{n}B");
            let data = vec![0u8; n];
            check(L, &op, abc == SHA256_ABC, &format!("abc_kat={},zeros={}", abc == SHA256_ABC, hex(&Sha256::digest(&data))));
            bench(L, &op, hash_iters(n), || {
                let mut c = Sha256::new();
                c.update(black_box(&data));
                black_box(c.finalize());
            });
        }
        let abc = hex(&Sha512::digest(b"abc"));
        for n in HASH_SIZES {
            let op = format!("sha512_{n}B");
            let data = vec![0u8; n];
            check(L, &op, abc == SHA512_ABC, &format!("abc_kat={},zeros={}", abc == SHA512_ABC, hex(&Sha512::digest(&data))));
            bench(L, &op, hash_iters(n), || {
                let mut c = Sha512::new();
                c.update(black_box(&data));
                black_box(c.finalize());
            });
        }
    }
    {
        use x25519_dalek::{EphemeralSecret, PublicKey};
        let a = EphemeralSecret::random();
        let b = EphemeralSecret::random();
        let pa = PublicKey::from(&a);
        let pb = PublicKey::from(&b);
        let s1 = a.diffie_hellman(&pb);
        let s2 = b.diffie_hellman(&pa);
        check(L, "x25519_keygen_agree", s1.as_bytes() == s2.as_bytes(), "symmetric");
        bench(L, "x25519_keygen_agree", ASYM_ITERS, || {
            let k = EphemeralSecret::random();
            black_box(PublicKey::from(&k));
            let peer = PublicKey::from(*X25519_PEER);
            black_box(*k.diffie_hellman(&peer).as_bytes());
        });
    }
    {
        use p256::PublicKey;
        use p256::ecdh::EphemeralSecret;
        use p256::elliptic_curve::Generate;
        let a = EphemeralSecret::generate();
        let b = EphemeralSecret::generate();
        let s1 = a.diffie_hellman(&b.public_key());
        let s2 = b.diffie_hellman(&a.public_key());
        check(L, "p256_ecdh_keygen_agree", s1.raw_secret_bytes() == s2.raw_secret_bytes(), "symmetric");
        bench(L, "p256_ecdh_keygen_agree", ASYM_ITERS, || {
            let k = EphemeralSecret::generate();
            black_box(k.public_key());
            let peer = PublicKey::from_sec1_bytes(P256_PEER).unwrap();
            black_box(k.diffie_hellman(&peer));
        });
    }
    {
        use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
        let sk = SigningKey::from_bytes(&ED_SEED);
        let sig: Signature = sk.sign(ECDSA_MSG);
        check(L, "ed25519_sign", sk.verifying_key().verify(ECDSA_MSG, &sig).is_ok(), &format!("sig={}", hex(&sig.to_bytes())));
        bench(L, "ed25519_sign", ASYM_ITERS, || { let s: Signature = sk.sign(ECDSA_MSG); black_box(s); });
        let vk = VerifyingKey::from_bytes(ED_PK).unwrap();
        let good = vk.verify(ED_MSG, &Signature::from_slice(ED_SIG).unwrap()).is_ok();
        let bad = vk.verify(ED_MSG, &Signature::from_slice(&bad_sig(ED_SIG)).unwrap()).is_err();
        check(L, "ed25519_verify", good && bad, "vector_ok+tamper_rejected");
        bench(L, "ed25519_verify", ASYM_ITERS, || { vk.verify(ED_MSG, &Signature::from_slice(ED_SIG).unwrap()).unwrap(); });
    }
    {
        use p256::ecdsa::{Signature, SigningKey, VerifyingKey, signature::Signer, signature::Verifier};
        use p256::elliptic_curve::Generate;
        let sk = SigningKey::generate();
        let sig: Signature = sk.sign(ECDSA_MSG);
        check(L, "p256_ecdsa_sign", sk.verifying_key().verify(ECDSA_MSG, &sig).is_ok(), "own_verify");
        bench(L, "p256_ecdsa_sign", ASYM_ITERS, || { let s: Signature = sk.sign(ECDSA_MSG); black_box(s); });
        let vk = VerifyingKey::from_sec1_bytes(ECDSA_PK).unwrap();
        let good = vk.verify(ECDSA_MSG, &Signature::from_slice(ECDSA_SIG).unwrap()).is_ok();
        let bad = Signature::from_slice(&bad_sig(ECDSA_SIG)).map(|s| vk.verify(ECDSA_MSG, &s).is_err()).unwrap_or(true);
        check(L, "p256_ecdsa_verify", good && bad, "vector_ok+tamper_rejected");
        bench(L, "p256_ecdsa_verify", ASYM_ITERS, || { vk.verify(ECDSA_MSG, &Signature::from_slice(ECDSA_SIG).unwrap()).unwrap(); });
    }
}

#[cfg(feature = "libcrux")]
fn run_libcrux() {
    const L: &str = "libcrux";
    {
        let key = libcrux_aes::AesGcm128Key::from(K16);
        let nonce = libcrux_aes::AesGcm128Nonce::from(NONCE);
        for n in AEAD_SIZES {
            let op = format!("aes128gcm_seal_{n}B");
            let pt = vec![0u8; n];
            let mut ct = vec![0u8; n];
            let mut tag = libcrux_aes::AesGcm128Tag::from([0u8; 16]);
            key.encrypt(&mut ct, &mut tag, &nonce, &AAD, &pt).unwrap();
            let t: &[u8; 16] = tag.as_ref();
            check(L, &op, true, &format!("tag={},ct8={}", hex(t), hex(&ct[..8])));
            bench(L, &op, aead_iters(n), || { key.encrypt(&mut ct, &mut tag, &nonce, &AAD, &pt).unwrap(); });
        }
    }
    for n in AEAD_SIZES {
        let op = format!("chacha20poly1305_seal_{n}B");
        let pt = vec![0u8; n];
        let mut ct = vec![0u8; n];
        let mut tag = [0u8; 16];
        libcrux_chacha20poly1305::encrypt_detached(&K32, &pt, &mut ct, &mut tag, &AAD, &NONCE).unwrap();
        let mut back = vec![1u8; n];
        let ok = libcrux_chacha20poly1305::decrypt_detached(&K32, &mut back, &ct, &tag, &AAD, &NONCE).is_ok() && back == pt;
        check(L, &op, ok, &format!("tag={},ct8={}", hex(&tag), hex(&ct[..8])));
        bench(L, &op, aead_iters(n), || {
            libcrux_chacha20poly1305::encrypt_detached(&K32, &pt, &mut ct, &mut tag, &AAD, &NONCE).unwrap();
        });
    }
    {
        let abc = hex(&libcrux_sha2::sha256(b"abc"));
        for n in HASH_SIZES {
            let op = format!("sha256_{n}B");
            let data = vec![0u8; n];
            check(L, &op, abc == SHA256_ABC, &format!("abc_kat={},zeros={}", abc == SHA256_ABC, hex(&libcrux_sha2::sha256(&data))));
            bench(L, &op, hash_iters(n), || { black_box(libcrux_sha2::sha256(black_box(&data))); });
        }
        let abc = hex(&libcrux_sha2::sha512(b"abc"));
        for n in HASH_SIZES {
            let op = format!("sha512_{n}B");
            let data = vec![0u8; n];
            check(L, &op, abc == SHA512_ABC, &format!("abc_kat={},zeros={}", abc == SHA512_ABC, hex(&libcrux_sha2::sha512(&data))));
            bench(L, &op, hash_iters(n), || { black_box(libcrux_sha2::sha512(black_box(&data))); });
        }
    }
    {
        use libcrux_ecdh::{X25519PublicKey, x25519_derive, x25519_key_gen};
        let (a, pa) = x25519_key_gen(&mut getrandom::SysRng).unwrap();
        let (b, pb) = x25519_key_gen(&mut getrandom::SysRng).unwrap();
        let s1 = x25519_derive(&pb, &a).unwrap();
        let s2 = x25519_derive(&pa, &b).unwrap();
        check(L, "x25519_keygen_agree", AsRef::<[u8]>::as_ref(&s1) == AsRef::<[u8]>::as_ref(&s2), "symmetric");
        bench(L, "x25519_keygen_agree", ASYM_ITERS, || {
            let (k, p) = x25519_key_gen(&mut getrandom::SysRng).unwrap();
            black_box(p);
            let peer = X25519PublicKey(*X25519_PEER);
            black_box(x25519_derive(&peer, &k).unwrap());
        });
    }
    {
        use libcrux_ecdh::p256::uncompressed_to_coordinates;
        use libcrux_ecdh::{P256PublicKey, p256_derive, p256_key_gen};
        let (a, pa) = p256_key_gen(&mut getrandom::SysRng).unwrap();
        let (b, pb) = p256_key_gen(&mut getrandom::SysRng).unwrap();
        let s1 = p256_derive(&pb, &a).unwrap();
        let s2 = p256_derive(&pa, &b).unwrap();
        check(L, "p256_ecdh_keygen_agree", AsRef::<[u8]>::as_ref(&s1) == AsRef::<[u8]>::as_ref(&s2), "symmetric");
        bench(L, "p256_ecdh_keygen_agree", ASYM_ITERS, || {
            let (k, p) = p256_key_gen(&mut getrandom::SysRng).unwrap();
            black_box(p);
            let peer = P256PublicKey(uncompressed_to_coordinates(P256_PEER).unwrap());
            black_box(p256_derive(&peer, &k).unwrap());
        });
    }
    {
        let sig = libcrux_ed25519::sign(ECDSA_MSG, &ED_SEED).unwrap();
        let pk = {
            let mut pk = [0u8; 32];
            libcrux_ed25519::secret_to_public(&mut pk, &ED_SEED);
            pk
        };
        check(L, "ed25519_sign", libcrux_ed25519::verify(ECDSA_MSG, &pk, &sig).is_ok(), &format!("sig={}", hex(&sig)));
        bench(L, "ed25519_sign", ASYM_ITERS, || { black_box(libcrux_ed25519::sign(ECDSA_MSG, &ED_SEED).unwrap()); });
        let good = libcrux_ed25519::verify(ED_MSG, ED_PK, ED_SIG).is_ok();
        let bad = libcrux_ed25519::verify(ED_MSG, ED_PK, &bad_sig(ED_SIG)).is_err();
        check(L, "ed25519_verify", good && bad, "vector_ok+tamper_rejected");
        bench(L, "ed25519_verify", ASYM_ITERS, || { libcrux_ed25519::verify(ED_MSG, ED_PK, ED_SIG).unwrap(); });
    }
    {
        use libcrux_ecdsa::p256::rand::{generate_key_pair, sign};
        use libcrux_ecdsa::p256::{PublicKey, Signature, uncompressed_to_coordinates, verify};
        let (sk, pk) = generate_key_pair(&mut getrandom::SysRng).unwrap();
        let sig = sign(libcrux_sha2::Algorithm::Sha256, ECDSA_MSG, &sk, &mut getrandom::SysRng).unwrap();
        check(L, "p256_ecdsa_sign", verify(libcrux_sha2::Algorithm::Sha256, ECDSA_MSG, &sig, &pk).is_ok(), "own_verify");
        bench(L, "p256_ecdsa_sign", ASYM_ITERS, || {
            black_box(sign(libcrux_sha2::Algorithm::Sha256, ECDSA_MSG, &sk, &mut getrandom::SysRng).unwrap());
        });
        let s = Signature::from_bytes(*ECDSA_SIG);
        let sb = Signature::from_bytes(bad_sig(ECDSA_SIG));
        let p = PublicKey(uncompressed_to_coordinates(ECDSA_PK).unwrap());
        let good = verify(libcrux_sha2::Algorithm::Sha256, ECDSA_MSG, &s, &p).is_ok();
        let bad = verify(libcrux_sha2::Algorithm::Sha256, ECDSA_MSG, &sb, &p).is_err();
        check(L, "p256_ecdsa_verify", good && bad, "vector_ok+tamper_rejected");
        bench(L, "p256_ecdsa_verify", ASYM_ITERS, || {
            let p = PublicKey(uncompressed_to_coordinates(ECDSA_PK).unwrap());
            verify(libcrux_sha2::Algorithm::Sha256, ECDSA_MSG, &s, &p).unwrap();
        });
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();
    std::thread::sleep(std::time::Duration::from_millis(1500));
    let mhz = unsafe { esp_idf_svc::sys::esp_rom_get_cpu_ticks_per_us() };
    println!("START heltec-competitors cpu_ticks_per_us={mhz} warmup=iters/10 timer=esp_timer_get_time");
    #[cfg(feature = "rind")]
    run_rind();
    #[cfg(feature = "ring")]
    run_ring();
    #[cfg(feature = "ring")]
    ring_chacha_large();
    #[cfg(feature = "rustcrypto")]
    run_rustcrypto();
    #[cfg(feature = "libcrux")]
    run_libcrux();
    println!("ALL-DONE");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
