| bench | library | functions | shift to pad1031 (B) | shift to pad2053 (B) |
|---|---|---:|---:|---:|
| aes_gcm | rind | 20 | +1048 (19/20) | +2072 (19/20) |
| aes_gcm | ring | 17 | +1048 (16/17) | +2072 (16/17) |
| aes_gcm | ring(asm) | 4 | +1048 | +2072 |
| aes_gcm | aes | 16 | +3356 (7/16) | +4380 (7/16) |
| aes_gcm | aes_gcm | 6 | +1040 | +2064 |
| aes_gcm | ghash | 2 | +1040 | +2064 |
| aes_gcm | polyval | 6 | +1048 | +2072 |
| chacha20poly1305 | rind | 13 | +1032 (11/13) | +2056 (11/13) |
| chacha20poly1305 | libcrux_chacha20poly1305 | 7 | +1032 | +2056 |
| chacha20poly1305 | libcrux_poly1305 | 6 | +1032 | +2056 |
| chacha20poly1305 | ring | 16 | +1032 (15/16) | +2056 (15/16) |
| chacha20poly1305 | ring(asm) | 4 | +1032 | +2056 |
| chacha20poly1305 | chacha20 | 3 | +1032 | +2056 |
| chacha20poly1305 | chacha20poly1305 | 4 | +1032 (2/4) | +2056 (2/4) |
| chacha20poly1305 | poly1305 | 5 | +1032 | +2056 |
| chacha20poly1305 | cipher | 1 | +2132 | +3156 |
| xchacha20poly1305 | rind | 15 | +1044 | +2068 |
| xchacha20poly1305 | libcrux_chacha20poly1305 | 7 | +1044 | +2068 |
| xchacha20poly1305 | libcrux_poly1305 | 6 | +1044 | +2068 |
| xchacha20poly1305 | ring | 15 | +1044 (14/15) | +2068 (14/15) |
| xchacha20poly1305 | ring(asm) | 3 | +1044 | +2068 |
| xchacha20poly1305 | chacha20 | 5 | +1044 (3/5) | +2068 (3/5) |
| xchacha20poly1305 | chacha20poly1305 | 4 | +1044 | +2068 |
| xchacha20poly1305 | poly1305 | 5 | +1044 | +2068 |
| xchacha20poly1305 | cipher | 5 | +1036 (4/5) | +2060 (4/5) |
| cipher | rind | 40 | +1044 (37/40) | +2068 (37/40) |
| cipher | libcrux_chacha20poly1305 | 6 | +1044 (5/6) | +2068 (5/6) |
| cipher | libcrux_poly1305 | 6 | +1044 | +2068 |
| cipher | ring | 31 | +1044 | +2068 |
| cipher | ring(asm) | 6 | +1044 | +2068 |
| cipher | chacha20 | 3 | +1044 | +2068 |
| cipher | chacha20poly1305 | 2 | +1044 | +2068 |
| cipher | poly1305 | 5 | +1044 | +2068 |
| cipher | aes | 11 | +-7772 (7/11) | +-6748 (7/11) |
| cipher | aes_gcm | 2 | +2560 | +3584 |
| cipher | ghash | 2 | +-1988 | +-964 |
| cipher | polyval | 6 | +1044 | +2068 |
| cipher | cipher | 1 | +1044 | +2068 |
| sha2 | rind | 23 | +1044 | +2068 |
| sha2 | libcrux_sha2 | 8 | +1044 | +2068 |
| sha2 | ring | 15 | +1044 | +2068 |
| sha2 | sha2 | 4 | +1044 | +2068 |
| sha3 | rind | 23 | +1044 | +2068 |
| sha3 | libcrux_sha3 | 6 | +1036 | +2060 |
| sha3 | sha3 | 3 | +1044 | +2068 |
| hmac | rind | 44 | +1044 | +2068 |
| hmac | libcrux_sha2 | 14 | +1044 | +2068 |
| hmac | ring | 21 | +1044 | +2068 |
| hmac | sha2 | 4 | +1044 | +2068 |
| hmac | hmac | 3 | +1036 | +2060 |
| hkdf | rind | 80 | +1044 (76/80) | +2068 (76/80) |
| hkdf | libcrux_sha2 | 17 | +1044 | +2068 |
| hkdf | libcrux_hmac | 9 | +1044 | +2068 |
| hkdf | libcrux_hkdf | 15 | +1044 (13/15) | +2068 (13/15) |
| hkdf | libcrux_sha3 | 20 | +1044 | +2068 |
| hkdf | ring | 37 | +1044 (36/37) | +2068 (36/37) |
| hkdf | ring(asm) | 3 | +1044 | +2068 |
| hkdf | sha2 | 4 | +1044 | +2068 |
| hkdf | hmac | 4 | +1036 | +2060 |
| hkdf | hkdf | 9 | +1036 | +2060 |
