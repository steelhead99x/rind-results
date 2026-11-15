//! Times a few rind operations on one ESP32-S3 core and prints them over UART0.
use std::hint::black_box;

fn now_us() -> i64 {
    unsafe { esp_idf_svc::sys::esp_timer_get_time() }
}

fn bench<F: FnMut()>(name: &str, bytes: usize, iters: u32, mut f: F) {
    // Warm-up.
    for _ in 0..(iters / 10).max(1) {
        f();
    }
    let t0 = now_us();
    for _ in 0..iters {
        f();
    }
    let dt = (now_us() - t0) as f64;
    let per_op_us = dt / iters as f64;
    let mhz = 240.0;
    let mbps = if bytes > 0 { (bytes as f64) / per_op_us } else { 0.0 }; // bytes/us == MB/s
    println!(
        "RIND,{name},iters={iters},total_us={dt:.0},per_op_us={per_op_us:.2},cycles_per_op_at_240MHz={:.0},MBps={mbps:.3}",
        per_op_us * mhz
    );
}

fn main() {
    esp_idf_svc::sys::link_patches();
    std::thread::sleep(std::time::Duration::from_millis(1500));
    let freq = unsafe { esp_idf_svc::sys::esp_rom_get_cpu_ticks_per_us() };
    println!("RIND-START rind 0.3.2 @ upstream 99b509a6, cpu_ticks_per_us={freq}");
    println!("RIND,cpu_check={:?}", rind::cpu::check().is_ok());

    let data = vec![0x5au8; 1024];

    // SHA-256 over 1 KiB, checked against a known answer for the empty string first.
    let empty = rind::digest::digest(&rind::digest::SHA256, b"");
    let hex: String = empty.as_ref().iter().map(|b| format!("{b:02x}")).collect();
    println!("RIND,kat_sha256_empty={}", hex == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    bench("sha256_1KiB", 1024, 2000, || {
        let mut ctx = rind::digest::Context::new(&rind::digest::SHA256);
        ctx.update(black_box(&data));
        black_box(ctx.finish());
    });
    bench("sha512_1KiB", 1024, 1000, || {
        let mut ctx = rind::digest::Context::new(&rind::digest::SHA512);
        ctx.update(black_box(&data));
        black_box(ctx.finish());
    });

    {
        use rind::aead::{AES_128_GCM, CHACHA20_POLY1305, Key, Nonce};
        let nonce = [0u8; 12];
        let mut buf = vec![0u8; 1024];
        let k = Key::new(&CHACHA20_POLY1305, &[7u8; 32]).unwrap();
        bench("chacha20poly1305_seal_1KiB", 1024, 1000, || {
            black_box(k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(nonce), &[], &mut buf).unwrap());
        });
        let k = Key::new(&AES_128_GCM, &[7u8; 16]).unwrap();
        bench("aes128gcm_seal_1KiB", 1024, 500, || {
            black_box(k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(nonce), &[], &mut buf).unwrap());
        });
        // Round-trip sanity check.
        let mut m = *b"hello heltec v3!";
        let tag = k.seal_in_place_separate_tag(Nonce::assume_unique_for_key(nonce), b"aad", &mut m).unwrap();
        let ok = k.open_in_place_separate_tag(Nonce::assume_unique_for_key(nonce), b"aad", tag, &mut m).is_ok() && &m == b"hello heltec v3!";
        println!("RIND,aes128gcm_roundtrip={ok}");
    }

    {
        use rind::agreement::{PrivateKey, X25519, ECDH_P256};
        let peer = PrivateKey::generate(&X25519).unwrap();
        let peer_pub = peer.public_key().as_ref().to_vec();
        let ours = PrivateKey::generate(&X25519).unwrap();
        bench("x25519_agree", 0, 50, || {
            black_box(ours.agree(&peer_pub).unwrap());
        });
        bench("x25519_keygen", 0, 50, || {
            let k = PrivateKey::generate(&X25519).unwrap();
            black_box(k.public_key());
        });
        let peer = PrivateKey::generate(&ECDH_P256).unwrap();
        let peer_pub = peer.public_key().as_ref().to_vec();
        let ours = PrivateKey::generate(&ECDH_P256).unwrap();
        bench("p256_ecdh_agree", 0, 20, || {
            black_box(ours.agree(&peer_pub).unwrap());
        });
    }

    {
        use rind::signature::{Ed25519KeyPair, EcdsaKeyPair, PublicKey, sign, verify};
        let msg = &data[..64];
        let kp = Ed25519KeyPair::generate().unwrap();
        let sig = kp.sign(msg);
        let pk = PublicKey::new(&verify::ED25519, kp.public_key().as_ref()).unwrap();
        println!("RIND,ed25519_verify_ok={}", pk.verify(msg, sig.as_ref()).is_ok());
        bench("ed25519_sign_64B", 0, 50, || {
            black_box(kp.sign(black_box(msg)));
        });
        bench("ed25519_verify_64B", 0, 50, || {
            pk.verify(msg, sig.as_ref()).unwrap();
        });

        let kp = EcdsaKeyPair::generate(&sign::ECDSA_P256_SHA256_FIXED).unwrap();
        let sig = kp.sign(msg).unwrap();
        let pk = PublicKey::new(&verify::ECDSA_P256_SHA256_FIXED, kp.public_key().as_ref()).unwrap();
        println!("RIND,p256_ecdsa_verify_ok={}", pk.verify(msg, sig.as_ref()).is_ok());
        bench("p256_ecdsa_sign_64B", 0, 20, || {
            black_box(kp.sign(black_box(msg)).unwrap());
        });
        bench("p256_ecdsa_verify_64B", 0, 20, || {
            pk.verify(msg, sig.as_ref()).unwrap();
        });
    }
    println!("RIND-DONE");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
