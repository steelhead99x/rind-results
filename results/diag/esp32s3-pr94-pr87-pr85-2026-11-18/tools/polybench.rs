// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

//! Board-local Poly1305 and ChaCha20-Poly1305 check and timing for rind PR 87.
//! Not part of any PR: it calls a bench-only `rind::__board_bench` shim.

use std::hint::black_box;

fn ccount() -> u32 {
    // SAFETY: reads a special register; no memory is touched.
    unsafe { esp_idf_svc::sys::xthal_get_ccount() }
}

fn pat(n: usize) -> Vec<u8> {
    (0..n as u32)
        .map(|i| (i.wrapping_mul(0x9e3779b9) >> 24) as u8)
        .collect()
}

fn hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks(2)
        .map(|p| u8::from_str_radix(core::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}

const PAT: &[(usize, &str)] = &[
    (0, "7548af82e1c4db3e1d7057aa89ecc326"),
    (1, "7aed6722e494104a2bbcf73194e4a05a"),
    (15, "d271b44f4b08e99ac7aa19569b97a830"),
    (16, "ee20db63d8c220ed5f3bc2be2fa4780e"),
    (17, "c85da0d5cd81c824af612d5476023066"),
    (31, "8455860d357eb52f8ca14af73e9e7446"),
    (32, "bee64dad8722f0bcb17d7540c9d26372"),
    (33, "c617428995af7a281c2f03f3f7aa8d05"),
    (63, "01b55583df45b1c186d5206f19611c0b"),
    (64, "e5f8a941c34d9135aefa86bc997a1280"),
    (65, "7a1365f91573b5b826c497606e97c06e"),
    (1200, "504baefa1b2cbe26feecd79441219f05"),
    (16384, "09ef83bd815c0c8ca586e84ba81077f0"),
];
const FF: &[(usize, &str)] = &[
    (16, "fbffff17faffff17faffff17faffff17"),
    (17, "7cfe7ff768f81f2763f8bf565df85f86"),
    (32, "5400801f3f00204f3900c07e330060ae"),
    (33, "86fa6a437bf4ec24a274504dc37495bc"),
    (64, "900fe32bc15fa8d7bca8efe4c7e37eb1"),
    (1200, "e9cddd12328a3534bc81d7edfe7f4e52"),
];

fn kat() -> usize {
    let mut failed = 0;
    let mut one = |name: String, key: &[u8; 32], msg: &[u8], want: &str| {
        let got = rind::__board_bench::poly1305(key, msg);
        let ok = got[..] == hex(want)[..];
        if !ok {
            failed += 1;
        }
        criterion::emit(&format!(
            "{{\"type\":\"poly-kat\",\"check\":\"{name}\",\"pass\":{ok}}}"
        ));
    };
    // RFC 8439 section 2.5.2.
    let key: [u8; 32] = hex("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b")
        .try_into()
        .unwrap();
    one(
        "rfc8439-2.5.2".into(),
        &key,
        b"Cryptographic Forum Research Group",
        "a8061dc1305136c6c22b8baf0c0127a9",
    );
    // Tags from OpenSSL (python cryptography) for a fixed key and pattern.
    let key: [u8; 32] = core::array::from_fn(|i| 0xa5 ^ (i as u8).wrapping_mul(29));
    for (len, want) in PAT {
        one(format!("openssl-pat/{len}"), &key, &pat(*len), want);
    }
    // All-ones key and message: the accumulator's largest values.
    for (len, want) in FF {
        one(format!("openssl-ff/{len}"), &[0xff; 32], &vec![0xff; *len], want);
    }
    failed
}

fn median_min(mut v: Vec<u32>) -> (u32, u32, u32) {
    v.sort_unstable();
    (v[v.len() / 2], v[0], v[v.len() - 1])
}

fn row(id: &str, iters: u32, mut f: impl FnMut()) {
    for _ in 0..2 {
        f();
    }
    let mut samples = Vec::new();
    for _ in 0..31 {
        let t0 = ccount();
        for _ in 0..iters {
            f();
        }
        samples.push(ccount().wrapping_sub(t0) / iters);
    }
    let (med, min, max) = median_min(samples);
    criterion::emit(&format!(
        "{{\"type\":\"pb\",\"id\":\"{id}\",\"median\":{med},\"min\":{min},\"max\":{max},\"iters\":{iters}}}"
    ));
}

pub fn benches() {
    let failed = kat();
    // The AEAD's own known-answer check (RFC 8439 section 2.8.2) runs in checks.
    criterion::emit(&format!(
        "{{\"type\":\"poly-kat-done\",\"failed\":{failed}}}"
    ));
    let key: [u8; 32] = core::array::from_fn(|i| 0xa5 ^ (i as u8).wrapping_mul(29));
    for (size, iters) in [(64usize, 200u32), (1200, 20), (16384, 2)] {
        let msg = pat(size);
        let mut acc = 0u8;
        row(&format!("poly1305/{size}B"), iters, || {
            acc ^= rind::__board_bench::poly1305(black_box(&key), black_box(&msg))[0];
        });
        black_box(acc);
    }
    let aead = rind::aead::Key::new(&rind::aead::CHACHA20_POLY1305, &[0u8; 32]).unwrap();
    for (size, iters) in [(32usize, 50u32), (64, 50), (1200, 5), (16384, 1)] {
        let mut buf = vec![0u8; size];
        let aad = [0u8; 32];
        let mut acc = 0u8;
        row(&format!("chacha20poly1305-seal/{size}B"), iters, || {
            let tag = aead
                .seal_in_place_separate_tag(
                    rind::aead::Nonce::assume_unique_for_key([0u8; 12]),
                    black_box(&aad),
                    black_box(&mut buf),
                )
                .unwrap();
            acc ^= tag.as_ref()[0];
        });
        black_box(acc);
    }
}
