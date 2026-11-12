<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# msi-001, rind's `admin/bench-report summary`

- `msi-001`: AMD Ryzen 5 7535HS with Radeon Graphics, Oracle Linux Server 10.2 (Linux 6.12.0-211.74.1.el10_2.x86_64), rustc 1.98.1 (48a229cea 2026-09-01) (noisy)

Each cell is rind's time, then that over the fastest other library's, and which library that is.

| operation | msi-001 |
|---|---|
| rustls-full-handshake-group/x25519mlkem768 | 306.59 µs, 0.78× aws-lc-rs |
| rustls-full-handshake-group/x25519 | 285.42 µs, 0.91× graviola |
| rustls-full-handshake-group/secp256r1 | 295.23 µs, 0.81× graviola |
| rustls-full-handshake-key/ed25519 | 204.57 µs, 0.98× graviola |
| rustls-full-handshake-key/rsa-2048 | 631.34 µs, 0.94× graviola |
| rustls-resumed-handshake/x25519 | 94.14 µs, 0.95× graviola |
| rustls-bulk/aes128-gcm | 325.57 µs, 1.01× ring |
| rustls-bulk/aes256-gcm | 345.46 µs, 1.00× ring |
| rustls-bulk/chacha20poly1305 | 1.46 ms, 1.12× aws-lc-rs |
| aes128-gcm/32B | 46 ns, 0.74× ring |
| aes128-gcm/1350B | 191 ns, 0.89× ring |
| aes128-gcm/16KB | 1.56 µs, 0.99× ring |
| aes128-gcm-open/32B | 39 ns, 0.58× ring |
| aes128-gcm-open/1350B | 195 ns, 0.88× ring |
| aes128-gcm-open/16KB | 1.69 µs, 0.98× ring |
| aes256-gcm/32B | 51 ns, 0.77× ring |
| aes256-gcm/1350B | 218 ns, 0.91× ring |
| aes256-gcm/16KB | 1.81 µs, 0.99× ring |
| aes256-gcm-open/32B | 64 ns, 0.69× graviola |
| aes256-gcm-open/1350B | 746 ns, 1.40× ring |
| aes256-gcm-open/16KB | 2.43 µs, 0.68× openssl |
| chacha20poly1305/32B | 130 ns, 0.97× ring |
| chacha20poly1305/1350B | 621 ns, 1.00× ring |
| chacha20poly1305/16KB | 5.34 µs, 1.00× aws-lc-rs |
| xchacha20poly1305/32B | 269 ns, 0.90× aws-lc-rs |
| xchacha20poly1305/1350B | 793 ns, 1.00× aws-lc-rs |
| xchacha20poly1305/16KB | 5.58 µs, 0.96× aws-lc-rs |
| quic-header-protection/aes128 | 5 ns, 0.64× aws-lc-rs |
| quic-header-protection/aes256 | 6 ns, 0.63× aws-lc-rs |
| quic-header-protection/chacha20 | 70 ns, 0.82× ring |
| x25519-ecdh | 28.33 µs, 1.01× graviola |
| p256-ecdh | 45.53 µs, 0.92× aws-lc-rs |
| ed25519-sign | 6.69 µs, 0.98× graviola |
| ed25519-verify | 29.58 µs, 1.02× rustcrypto |
| p256-ecdsa-sign | 10.31 µs, 0.83× graviola |
| p256-ecdsa-verify | 60.82 µs, 1.22× graviola |
| sha256/64B | 65 ns, 0.89× graviola |
| sha256/1350B | 748 ns, 1.04× ring |
| sha256/16KB | 9.73 µs, 0.97× ring |
| sha512/64B | 156 ns, 0.71× rustcrypto |
| sha512/1350B | 1.57 µs, 0.83× openssl |
| sha512/16KB | 16.99 µs, 0.95× openssl |
| hmac-sha256/64B | 151 ns, 0.83× rustcrypto |
| hmac-sha256/1350B | 746 ns, 0.95× rustcrypto |
| hkdf-sha256/extract-expand | 291 ns, 0.82× rustcrypto |
| hkdf-sha256/expand | 101 ns, 0.86× rustcrypto |
| rsa2048-pkcs1-sha256-sign | 632.61 µs, 0.81× openssl |
| rsa2048-pkcs1-sha256-verify | 13.33 µs, 0.81× aws-lc-rs |
| rsa2048-pss-sha256-sign | 695.27 µs, 1.19× aws-lc-rs |
| rsa2048-pss-sha256-verify | 15.98 µs, 0.90× aws-lc-rs |
| mlkem768-keygen | 9.42 µs, 0.94× aws-lc-rs |
| mlkem768-encaps | 9.81 µs, 0.95× aws-lc-rs |
| mlkem768-decaps | 6.75 µs, 0.75× graviola |
| mldsa65-keygen | 31.49 µs, 0.82× aws-lc-rs |
| mldsa65-sign | 88.68 µs, 0.75× aws-lc-rs |
| mldsa65-verify | 26.34 µs, 0.78× aws-lc-rs |
| p384-ecdh | 152.31 µs, 0.88× aws-lc-rs |
| p384-ecdsa-sign | 67.46 µs, 0.45× aws-lc-rs |
| p384-ecdsa-verify | 141.72 µs, 0.60× aws-lc-rs |
| p521-ecdh | 282.43 µs, 0.53× openssl |
| p521-ecdsa-sign | 140.63 µs, 0.47× aws-lc-rs |
| p521-ecdsa-verify | 493.18 µs, 1.14× aws-lc-rs |
