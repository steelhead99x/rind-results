| operation | rind old (b6651423) | rind new (8feee30e) | new/old | fastest other (new) | rind/fastest |
|---|---|---|---|---|---|
| rustls-full-handshake-group/x25519mlkem768 | 264.91 µs | 263.21 µs | 0.99 | graviola 296.38 µs | 0.89 |
| rustls-full-handshake-group/x25519 | 283.66 µs | 236.14 µs | 0.83 | graviola 249.38 µs | 0.95 |
| rustls-full-handshake-group/secp256r1 | 307.37 µs | 370.36 µs | 1.20 | graviola 288.51 µs | 1.28 |
| rustls-full-handshake-key/ed25519 | 194.18 µs | 192.67 µs | 0.99 | graviola 194.63 µs | 0.99 |
| rustls-full-handshake-key/rsa-2048 | 637.75 µs | 633.54 µs | 0.99 | aws-lc-rs 701.78 µs | 0.90 |
| rustls-resumed-handshake/x25519 | 109.20 µs | 101.02 µs | 0.93 | graviola 94.56 µs | 1.07 |
| rustls-bulk/aes128-gcm | 375.82 µs | 380.33 µs | 1.01 | aws-lc-rs 373.57 µs | 1.02 |
| rustls-bulk/aes256-gcm | 518.93 µs | 427.67 µs | 0.82 | aws-lc-rs 422.95 µs | 1.01 |
| rustls-bulk/chacha20poly1305 | 1838.10 µs | 1455.43 µs | 0.79 | openssl 1370.17 µs | 1.06 |
| aes128-gcm/32B | 71 ns | 86 ns | 1.20 | rustcrypto 47 ns | 1.83 |
| aes128-gcm/1350B | 270 ns | 289 ns | 1.07 | openssl 259 ns | 1.12 |
| aes128-gcm/16KB | 1.96 µs | 1.98 µs | 1.01 | aws-lc-rs 1.96 µs | 1.01 |
| aes128-gcm-open/32B | 64 ns | 67 ns | 1.04 | rustcrypto 48 ns | 1.40 |
| aes128-gcm-open/1350B | 271 ns | 284 ns | 1.05 | aws-lc-rs 248 ns | 1.14 |
| aes128-gcm-open/16KB | 2.12 µs | 2.98 µs | 1.40 | openssl 2.27 µs | 1.31 |
| aes256-gcm/32B | 77 ns | 108 ns | 1.40 | rustcrypto 56 ns | 1.93 |
| aes256-gcm/1350B | 307 ns | 444 ns | 1.45 | openssl 460 ns | 0.96 |
| aes256-gcm/16KB | 2.36 µs | 3.55 µs | 1.50 | openssl 2.46 µs | 1.44 |
| aes256-gcm-open/32B | 71 ns | 76 ns | 1.07 | rustcrypto 49 ns | 1.53 |
| aes256-gcm-open/1350B | 303 ns | 313 ns | 1.03 | aws-lc-rs 288 ns | 1.09 |
| aes256-gcm-open/16KB | 2.58 µs | 2.55 µs | 0.99 | aws-lc-rs 2.51 µs | 1.02 |
| chacha20poly1305/32B | 256 ns | 256 ns | 1.00 | ring 211 ns | 1.21 |
| chacha20poly1305/1350B | 1.08 µs | 1.08 µs | 1.00 | ring 1.03 µs | 1.05 |
| chacha20poly1305/16KB | 10.51 µs | 10.42 µs | 0.99 | openssl 9.30 µs | 1.12 |
| xchacha20poly1305/32B | 790 ns | 798 ns | 1.01 | aws-lc-rs 780 ns | 1.02 |
| xchacha20poly1305/1350B | 1.62 µs | 1.64 µs | 1.01 | aws-lc-rs 1.60 µs | 1.03 |
| xchacha20poly1305/16KB | 11.05 µs | 11.20 µs | 1.01 | aws-lc-rs 10.86 µs | 1.03 |
| quic-header-protection/aes128 | 50 ns | 53 ns | 1.07 | aws-lc-rs 4 ns | 12.62 |
| quic-header-protection/aes256 | 55 ns | 56 ns | 1.01 | aws-lc-rs 5 ns | 11.20 |
| quic-header-protection/chacha20 | 129 ns | 130 ns | 1.01 | ring 98 ns | 1.34 |
| x25519-ecdh | 25.01 µs | 25.13 µs | 1.00 | graviola 25.15 µs | 1.00 |
| p256-ecdh | 41.72 µs | 42.90 µs | 1.03 | graviola 46.86 µs | 0.92 |
| ed25519-sign | 6.13 µs | 6.81 µs | 1.11 | graviola 6.75 µs | 1.01 |
| ed25519-verify | 25.15 µs | 29.61 µs | 1.18 | graviola 29.83 µs | 0.99 |
| p256-ecdsa-sign | 10.68 µs | 18.68 µs | 1.75 | graviola 19.85 µs | 0.94 |
| p256-ecdsa-verify | 40.07 µs | 71.96 µs | 1.80 | aws-lc-rs 46.33 µs | 1.55 |
| sha256/64B | 117 ns | 105 ns | 0.90 | graviola 54 ns | 1.97 |
| sha256/1350B | 708 ns | 674 ns | 0.95 | ring 585 ns | 1.15 |
| sha256/16KB | 10.94 µs | 6.86 µs | 0.63 | ring 6.74 µs | 1.02 |
| sha512/64B | 234 ns | 145 ns | 0.62 | rustcrypto 102 ns | 1.41 |
| sha512/1350B | 1.36 µs | 1.10 µs | 0.81 | rustcrypto 971 ns | 1.13 |
| sha512/16KB | 12.02 µs | 11.84 µs | 0.99 | rustcrypto 11.53 µs | 1.03 |
| sha3-256/{lib}/64B | - | 223 ns | - | rustcrypto 160 ns | 1.40 |
| sha3-256/{lib}/1350B | - | 1.74 µs | - | rustcrypto 1.61 µs | 1.08 |
| sha3-256/{lib}/16KB | - | 20.02 µs | - | rustcrypto 19.34 µs | 1.04 |
| sha3-512/{lib}/64B | - | 218 ns | - | rustcrypto 163 ns | 1.34 |
| sha3-512/{lib}/1350B | - | 3.21 µs | - | rustcrypto 3.02 µs | 1.06 |
| sha3-512/{lib}/16KB | - | 38.53 µs | - | rustcrypto 36.24 µs | 1.06 |
| hmac-sha256/64B | 241 ns | 397 ns | 1.65 | ring 266 ns | 1.49 |
| hmac-sha256/1350B | 772 ns | 1.47 µs | 1.91 | ring 776 ns | 1.90 |
| hkdf-sha256/extract-expand | 418 ns | 502 ns | 1.20 | rustcrypto 351 ns | 1.43 |
| hkdf-sha256/expand | 135 ns | 169 ns | 1.25 | rustcrypto 112 ns | 1.51 |
| rsa2048-pkcs1-sha256-sign | 465.30 µs | 472.46 µs | 1.02 | aws-lc-rs 508.99 µs | 0.93 |
| rsa2048-pkcs1-sha256-verify | 13.97 µs | 15.45 µs | 1.11 | aws-lc-rs 16.33 µs | 0.95 |
| rsa2048-pss-sha256-sign | 465.78 µs | 470.67 µs | 1.01 | aws-lc-rs 511.02 µs | 0.92 |
| rsa2048-pss-sha256-verify | 14.11 µs | 16.03 µs | 1.14 | aws-lc-rs 17.48 µs | 0.92 |
| mlkem768-keygen | 7.80 µs | 10.25 µs | 1.31 | graviola 10.22 µs | 1.00 |
| mlkem768-encaps | 9.07 µs | 9.24 µs | 1.02 | graviola 8.74 µs | 1.06 |
| mlkem768-decaps | 5.70 µs | 6.79 µs | 1.19 | graviola 6.40 µs | 1.06 |
| mldsa65-keygen | 26.53 µs | 28.33 µs | 1.07 | aws-lc-rs 46.57 µs | 0.61 |
| mldsa65-sign | 69.88 µs | 71.83 µs | 1.03 | aws-lc-rs 101.53 µs | 0.71 |
| mldsa65-verify | 22.82 µs | 94.25 µs | 4.13 | libcrux 91.10 µs | 1.03 |
| p384-ecdh | 150.79 µs | 356.05 µs | 2.36 | aws-lc-rs 257.06 µs | 1.39 |
| p384-ecdsa-sign | 39.08 µs | 35.26 µs | 0.90 | aws-lc-rs 98.36 µs | 0.36 |
| p384-ecdsa-verify | 137.12 µs | 218.99 µs | 1.60 | graviola 266.71 µs | 0.82 |
| p521-ecdh | 278.06 µs | 231.33 µs | 0.83 | aws-lc-rs 253.60 µs | 0.91 |
| p521-ecdsa-sign | 57.25 µs | 59.30 µs | 1.04 | rustcrypto 142.70 µs | 0.42 |
| p521-ecdsa-verify | 208.90 µs | 219.19 µs | 1.05 | aws-lc-rs 393.62 µs | 0.56 |
