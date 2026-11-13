Built in a separate worktree (rind-bench 2f0c5e8, rind 98568332 with patches 0001-0004 only), so v0 crate hashes differ from the published images; functions are matched by unique demangled name.

| bench | library | functions | shift to pad1031 (B) | shift to pad2053 (B) |
|---|---|---:|---:|---:|
| aes_gcm | rind | 19 | +1064 | +1064 |
| aes_gcm | ring | 16 | +1064 (11/16) | +1064 (11/16) |
| aes_gcm | ring(asm) | 4 | +1080 (3/4) | +1080 (3/4) |
| aes_gcm | aes | 16 | +3372 (7/16) | +3372 (7/16) |
| aes_gcm | aes_gcm | 10 | +1052 | +1052 |
| aes_gcm | ghash | 2 | +1052 | +1052 |
| aes_gcm | polyval | 6 | +1080 | +1080 |
| aes_gcm | cipher | 1 | +1052 | +1052 |
| chacha20poly1305 | rind | 12 | +1040 (11/12) | +1040 (11/12) |
| chacha20poly1305 | libcrux_chacha20poly1305 | 7 | +1040 | +1040 |
| chacha20poly1305 | libcrux_poly1305 | 6 | +1040 | +1040 |
| chacha20poly1305 | ring | 15 | +1040 (13/15) | +1040 (13/15) |
| chacha20poly1305 | ring(asm) | 4 | +1040 | +1040 |
| chacha20poly1305 | chacha20 | 3 | +1040 | +1040 |
| chacha20poly1305 | chacha20poly1305 | 4 | +1040 (2/4) | +1040 (2/4) |
| chacha20poly1305 | poly1305 | 5 | +1040 | +1040 |
| chacha20poly1305 | cipher | 2 | +-3676 (1/2) | +-3676 (1/2) |
| xchacha20poly1305 | rind | 15 | +1052 (9/15) | +1052 (9/15) |
| xchacha20poly1305 | libcrux_chacha20poly1305 | 7 | +1052 | +1052 |
| xchacha20poly1305 | libcrux_poly1305 | 6 | +1052 | +1052 |
| xchacha20poly1305 | ring | 15 | +1052 (12/15) | +1052 (12/15) |
| xchacha20poly1305 | ring(asm) | 3 | +1052 | +1052 |
| xchacha20poly1305 | chacha20 | 5 | +1052 (3/5) | +1052 (3/5) |
| xchacha20poly1305 | chacha20poly1305 | 4 | +3536 (2/4) | +3536 (2/4) |
| xchacha20poly1305 | poly1305 | 5 | +1052 | +1052 |
| xchacha20poly1305 | cipher | 3 | +1044 | +1044 |
| cipher | rind | 40 | +1068 (37/40) | +1068 (37/40) |
| cipher | libcrux_chacha20poly1305 | 6 | +1052 (5/6) | +1052 (5/6) |
| cipher | libcrux_poly1305 | 6 | +1068 | +1068 |
| cipher | ring | 31 | +1052 (13/31) | +1052 (13/31) |
| cipher | ring(asm) | 6 | +1068 | +1068 |
| cipher | chacha20 | 3 | +1052 (2/3) | +1052 (2/3) |
| cipher | chacha20poly1305 | 2 | +1052 | +1052 |
| cipher | poly1305 | 5 | +1052 | +1052 |
| cipher | aes | 11 | +-7764 (7/11) | +-7764 (7/11) |
| cipher | aes_gcm | 2 | +2568 | +2568 |
| cipher | ghash | 2 | +-1980 | +-1980 |
| cipher | polyval | 6 | +1052 | +1052 |
| cipher | cipher | 2 | +1052 | +1052 |
| sha2 | rind | 23 | +1052 | +1052 |
| sha2 | libcrux_sha2 | 8 | +1052 | +1052 |
| sha2 | ring | 15 | +1052 | +1052 |
| sha2 | sha2 | 4 | +1052 | +1052 |
| sha3 | rind | 23 | +1056 | +1056 |
| sha3 | libcrux_sha3 | 6 | +1044 | +1044 |
| sha3 | sha3 | 3 | +1056 | +1056 |
| hmac | rind | 44 | +1036 | +1036 |
| hmac | libcrux_sha2 | 14 | +1036 | +1036 |
| hmac | ring | 21 | +1036 | +1036 |
| hmac | sha2 | 4 | +1036 | +1036 |
| hmac | hmac | 3 | +1040 | +1040 |
| hkdf | rind | 72 | +1072 | +1072 |
| hkdf | libcrux_sha2 | 17 | +1072 | +1072 |
| hkdf | libcrux_hmac | 9 | +1072 | +1072 |
| hkdf | libcrux_hkdf | 14 | +1072 (13/14) | +1072 (13/14) |
| hkdf | libcrux_sha3 | 20 | +1072 | +1072 |
| hkdf | ring | 35 | +1072 (18/35) | +1072 (18/35) |
| hkdf | ring(asm) | 3 | +1072 | +1072 |
| hkdf | sha2 | 4 | +1072 | +1072 |
| hkdf | hmac | 4 | +1048 | +1048 |
| hkdf | hkdf | 8 | +1048 | +1048 |
