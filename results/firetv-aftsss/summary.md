<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Fire TV Stick (AFTSSS), rind against the fastest other library

- `firetv-aftsss`: Amazon Fire TV Stick (AFTSSS) MediaTek (mt8695, AFTSSS; 4x Cortex-A53 r0p4 part 0xd03, 1.69 GHz, AArch32 userland, crypto ext in AArch32), Android 9 (API 28, build PS7704.5024N), target armv7-linux-androideabi (API 28), Android NDK r28c, cross-compiled on an x86_64 Linux host (noisy)

Each cell is rind's time, then that over the fastest other library's, and which library that is. Below 1.00× rind is faster. Written from [firetv-aftsss.json](firetv-aftsss.json) in the shape of `admin/bench-report summary`.

| operation | firetv-aftsss |
|---|---|
| aes128-gcm/32B | 371.5 ns, 0.50× aws-lc-rs |
| aes128-gcm/1350B | 2.414 µs, 0.88× aws-lc-rs |
| aes128-gcm/16KB | 24.855 µs, 0.99× aws-lc-rs |
| aes128-gcm-open/32B | 409.1 ns, 0.54× aws-lc-rs |
| aes128-gcm-open/1350B | 3.571 µs, 0.86× aws-lc-rs |
| aes128-gcm-open/16KB | 37.816 µs, 1.11× aws-lc-rs |
| aes256-gcm/32B | 388.0 ns, 0.51× aws-lc-rs |
| aes256-gcm/1350B | 2.774 µs, 0.89× aws-lc-rs |
| aes256-gcm/16KB | 29.247 µs, 0.97× openssl |
| aes256-gcm-open/32B | 528.1 ns, 0.63× aws-lc-rs |
| aes256-gcm-open/1350B | 4.282 µs, 1.04× aws-lc-rs |
| aes256-gcm-open/16KB | 39.160 µs, 0.91× openssl |
| chacha20poly1305/32B | 2.698 µs, 1.11× aws-lc-rs |
| chacha20poly1305/1350B | 9.115 µs, 1.09× openssl |
| chacha20poly1305/16KB | 82.788 µs, 1.22× openssl |
| xchacha20poly1305/32B | 11.290 µs, 0.82× aws-lc-rs |
| xchacha20poly1305/1350B | 18.387 µs, 1.04× aws-lc-rs |
| xchacha20poly1305/16KB | 89.112 µs, 0.94× aws-lc-rs |
| quic-header-protection/aes128 | 48.0 ns, 1.01× aws-lc-rs |
| quic-header-protection/aes256 | 57.5 ns, 1.01× aws-lc-rs |
| quic-header-protection/chacha20 | 562.7 ns, 1.25× ring |
| x25519-ecdh | 578.760 µs, 0.99× ring |
| p256-ecdh | 1.560 ms, 1.09× openssl |
| ed25519-sign | 292.258 µs, 0.91× openssl |
| ed25519-verify | 778.065 µs, 0.90× ring |
| p256-ecdsa-sign | 446.208 µs, 1.13× openssl |
| p256-ecdsa-verify | 1.513 ms, 1.20× openssl |
| sha256/64B | 742.3 ns, 0.59× aws-lc-rs |
| sha256/1350B | 2.427 µs, 0.80× aws-lc-rs |
| sha256/16KB | 22.861 µs, 0.98× aws-lc-rs |
| sha512/64B | 1.819 µs, 1.01× ring |
| sha512/1350B | 13.650 µs, 1.00× ring |
| sha512/16KB | 152.367 µs, 1.00× ring |
| sha3-256/64B | 2.259 µs, 0.15× aws-lc-rs |
| sha3-256/1350B | 16.461 µs, 0.25× aws-lc-rs |
| sha3-256/16KB | 189.723 µs, 0.25× aws-lc-rs |
| hmac-sha256/64B | 1.564 µs, 0.33× ring |
| hmac-sha256/1350B | 3.346 µs, 0.41× aws-lc-rs |
| hkdf-sha256/extract-expand | 3.226 µs, 0.42× aws-lc-rs |
| hkdf-sha256/expand | 938.8 ns, 0.37× ring |
| mlkem768-keygen | 235.319 µs, 0.55× openssl |
| mlkem768-encaps | 284.449 µs, 0.66× aws-lc-rs |
| mlkem768-decaps | 216.007 µs, 0.68× openssl |
| mldsa65-sign | 2.933 ms, 0.75× aws-lc-rs |
| mldsa65-verify | 356.200 µs, 0.25× aws-lc-rs |
| p384-ecdh | 4.748 ms, 0.40× ring |
