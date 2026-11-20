<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# x86-host-b-zen5 (Ryzen 7 255, Zen 4, AVX-512): rind against the other libraries, and SPHINCS+ #143

Host: AMD Ryzen 7 255 w/ Radeon 780M Graphics (Zen 4, 16 logical CPUs, AVX-512 reported by rind-cpuid), Oracle Linux Server 10.1, rustc 1.98.1, governor `performance`, benchmarks pinned to CPUs 2 and 10. Measured 2026-11-18 (UTC 07:23 to 09:55). rind is upstream `d6ca9a756d3d1e71489e61e44c04f33c3c893589`, unpatched, 3 runs, noisy flag false on every run. Full tables: [RESULTS.md](RESULTS.md), [RUNS.md](RUNS.md), [x86-host-b-zen5.json](x86-host-b-zen5.json).

## 1. rind against each library, per algorithm

Each cell is the geometric mean of rind time / that library's time over the operations the two share (median of 3 runs per operation). Below 1.00x rind is faster. In brackets: operations where rind is faster / operations compared.

| Algorithm | ops | aws-lc-rs | ring | RustCrypto+dalek | graviola | libcrux | OpenSSL |
|---|---|---|---|---|---|---|---|
| rustls handshake (group) | 3 | 0.78× (3/3) | 0.76× (2/2) | - | 0.94× (3/3) | - | 0.40× (3/3) |
| rustls handshake (key) | 2 | 0.78× (2/2) | 0.52× (2/2) | - | 0.72× (2/2) | - | 0.37× (2/2) |
| rustls resumed handshake | 1 | 0.56× (1/1) | 0.47× (1/1) | - | 0.96× (1/1) | - | 0.23× (1/1) |
| rustls bulk | 3 | 0.95× (2/3) | 0.91× (2/3) | - | 0.68× (3/3) | - | 0.93× (2/3) |
| AES128-GCM | 6 | 0.75× (6/6) | 0.74× (6/6) | 0.24× (6/6) | 0.68× (6/6) | 0.05× (6/6) | 0.65× (6/6) |
| AES256-GCM | 6 | 0.78× (6/6) | 0.78× (6/6) | 0.24× (6/6) | 0.70× (6/6) | 0.03× (6/6) | 0.69× (6/6) |
| ChaCha20-Poly1305 | 3 | 0.99× (1/3) | 1.00× (1/3) | 0.24× (3/3) | 0.51× (3/3) | 0.29× (3/3) | 0.76× (2/3) |
| XChaCha20-Poly1305 | 3 | 0.98× (3/3) | - | 0.31× (3/3) | 0.58× (3/3) | 0.33× (3/3) | - |
| QUIC header protection | 3 | 0.59× (3/3) | 0.52× (3/3) | - | - | - | - |
| X25519 ECDH | 1 | 0.98× (1/1) | 0.52× (1/1) | 0.46× (1/1) | 1.00× (0/1) | 0.38× (1/1) | 0.45× (1/1) |
| P-256 (ECDH/ECDSA) | 3 | 0.89× (2/3) | 0.84× (3/3) | 0.16× (3/3) | 0.89× (3/3) | 0.15× (3/3) | 0.61× (3/3) |
| Ed25519 | 2 | 0.93× (2/2) | 0.50× (2/2) | 0.69× (1/2) | 0.99× (2/2) | 0.27× (2/2) | 0.26× (2/2) |
| SHA-256 | 3 | 0.87× (3/3) | 0.92× (3/3) | 0.95× (3/3) | 0.98× (3/3) | 0.20× (3/3) | 0.63× (3/3) |
| SHA-512 | 3 | 0.69× (3/3) | 0.69× (3/3) | 0.71× (3/3) | 0.71× (3/3) | 0.65× (3/3) | 0.75× (3/3) |
| SHA3 | 6 | 0.95× (6/6) | - | 0.82× (6/6) | - | 0.84× (6/6) | - |
| HMAC-SHA256 | 2 | 0.57× (2/2) | 0.69× (2/2) | 0.87× (2/2) | 0.84× (2/2) | 0.18× (2/2) | 0.12× (2/2) |
| HKDF-SHA256 | 2 | 0.50× (2/2) | 0.60× (2/2) | 0.79× (2/2) | - | 0.18× (2/2) | 0.08× (2/2) |
| RSA-2048 | 4 | 0.90× (4/4) | 0.54× (4/4) | 0.19× (4/4) | 0.52× (4/4) | 0.03× (2/2) | 0.73× (4/4) |
| ML-KEM-768 | 3 | 0.58× (3/3) | - | 0.10× (3/3) | 1.09× (0/3) | 0.39× (3/3) | 0.18× (3/3) |
| ML-DSA-65 | 3 | 0.78× (3/3) | - | 0.14× (3/3) | - | 0.54× (3/3) | 0.15× (3/3) |
| P-384 | 3 | 0.71× (3/3) | 0.17× (3/3) | 0.19× (3/3) | 0.47× (3/3) | - | 0.23× (3/3) |
| P-521 | 3 | 0.68× (3/3) | - | 0.38× (3/3) | - | - | 0.41× (3/3) |
| **All shared ops** | | **0.78× (64/68)** | **0.64× (46/49)** | **0.33× (55/56)** | **0.72× (47/51)** | **0.19× (48/48)** | **0.43× (54/56)** |

Headlines:

- rind is the fastest library on 59 of the 68 operations it shares with another library (RESULTS.md).
- Largest wins: ECDSA sign on P-521 (0.44x) and P-384 (0.49x) against aws-lc-rs, AES-GCM 32 B (0.62 to 0.75x against the fastest rival), ML-DSA-65 sign (0.72x against aws-lc-rs), QUIC header protection (0.47 to 0.58x), HKDF/HMAC small inputs, rustls handshakes (0.92 to 0.98x against graviola, geometric means 0.37 to 0.78x against aws-lc-rs, ring and OpenSSL).
- Losses (rind more than 2% slower than the fastest rival on the medians):
  - ChaCha20-Poly1305 16 KB: 4.89 us against OpenSSL 3.68 us (1.33x), and rustls-bulk ChaCha20-Poly1305 709 us against OpenSSL 597 us (1.19x). ChaCha20-Poly1305 1350 B is level with ring (565 ns against 552 ns, 1.02x).
  - ML-KEM-768 keygen 6.77 us against graviola 5.95 us (1.14x) and encaps 6.92 us against 6.24 us (1.11x). Decaps is 1.02x.
  - Ed25519 verify 25.67 us against RustCrypto+dalek 24.01 us (1.07x; that rival has an 11.9% run spread).
  - X25519 ECDH is a tie with graviola (24.40 us against 24.38 us).
- These gaps are recorded as measured in [gaps.toml](gaps.toml). No cause was diagnosed here.

## 2. SPHINCS+ / SLH-DSA SHAKE, PR #143 (AVX-512)

What ran: the reference C implementation (`shake-avx2` from sphincsplus) built twice, `-march=native` (sp-native, so AVX-512 is allowed) and `-march=haswell` (sp-haswell, AVX2 only); 7 repeats each, pinned to one CPU, median cycles. rind SLH-DSA SHAKE at the commit before `a53864c` ("before") and after `c7e2dab` ("after", XKCP Keccak-f permuting four interleaved states): criterion, 1 s warm-up, 5 s measurement, pinned to one CPU, 2 interleaved rounds. rind times are converted to cycles with the TSC rate implied by the C runs (about 3.79 cycles per ns), so they are comparable with the C cycle counts only to a few percent.

### C reference, this host

| Set | Op | sp-native | sp-haswell | haswell/native |
|---|---|---|---|---|
| SHAKE-128f | keygen | 1.50M | 3.00M | 2.00× |
| SHAKE-128f | sign | 34.89M | 70.13M | 2.01× |
| SHAKE-128f | verify | 2.96M | 5.08M | 1.71× |
| SHAKE-128s | keygen | 96.00M | 192.79M | 2.01× |
| SHAKE-128s | sign | 724.37M | 1453.07M | 2.01× |
| SHAKE-128s | verify | 1.24M | 1.90M | 1.53× |
| SHAKE-256f | keygen | 5.73M | 11.48M | 2.00× |
| SHAKE-256f | sign | 113.45M | 226.29M | 1.99× |
| SHAKE-256f | verify | 4.32M | 7.27M | 1.68× |

The -march=native build takes about half the cycles of -march=haswell on keygen and sign in all three sets (verify 1.5 to 1.7x).

### rind before and after #143 (cycles)

| Set | Op | before r1 | before r2 | after r1 | after r2 | gain (best-of-2) | gain (mean) | after best vs sp-native | after best vs sp-haswell |
|---|---|---|---|---|---|---|---|---|---|
| SHAKE-128f | keygen | 1.64M | 1.66M | 1.01M | 1.26M | 38.5% | 31.2% | 0.67× | 0.34× |
| SHAKE-128f | sign | 38.65M | 39.29M | 23.76M | 27.16M | 38.5% | 34.7% | 0.68× | 0.34× |
| SHAKE-128f | verify | 2.64M | 2.63M | 1.97M | 1.95M | 25.9% | 25.6% | 0.66× | 0.38× |
| SHAKE-128s | keygen | 105.48M | 105.05M | 64.78M | 64.65M | 38.5% | 38.5% | 0.67× | 0.34× |
| SHAKE-128s | sign | 801.59M | 804.77M | 491.94M | 694.18M | 38.6% | 26.2% | 0.68× | 0.34× |
| SHAKE-128s | verify | 1.07M | 1.01M | 0.75M | 0.74M | 26.9% | 28.5% | 0.59× | 0.39× |
| SHAKE-256f | keygen | 6.40M | 6.49M | 3.97M | 3.98M | 37.9% | 38.3% | 0.69× | 0.35× |
| SHAKE-256f | sign | 129.14M | 131.37M | 80.65M | 80.93M | 37.5% | 38.0% | 0.71× | 0.36× |
| SHAKE-256f | verify | 3.94M | 3.98M | 2.83M | 2.83M | 28.3% | 28.6% | 0.65× | 0.39× |

Round 2 of "after" was noisy for SHAKE-128f keygen/sign and 128s sign (1.26M against 1.01M, 27.2M against 23.8M, 694M against 492M cycles). The other rows agree between rounds to within 1%. "Best-of-2" is therefore the fairer figure; the mean is shown for honesty.

### Comparison with the AVX2-only Arrow Lake reference

| | Arrow Lake PC (AVX2 only, as given) | This host (Zen 4, AVX-512) |
|---|---|---|
| SHAKE-128f sign, C reference | about 43.7M cycles | 34.89M (sp-native); 70.13M (sp-haswell) |
| SHAKE-128f verify, C reference | about 3.24M cycles | 2.96M (sp-native); 5.08M (sp-haswell) |
| rind sign gain from #143 | 0 to 6% | 37.5 to 38.6% best-of-2 (128f 38.5%, 128s 38.6%, 256f 37.5%); 26 to 38% by mean |
| rind verify gain from #143 | not given | 26 to 28% |
| rind (after) sign against C native | not given | 0.68x on 128f (23.76M against 34.89M), 0.71x on 256f |

On Zen 4 with AVX-512 the #143 rind change is a large win (about 38% on sign and keygen, 26 to 28% on verify), where the AVX2-only Arrow Lake machine saw 0 to 6% on sign. After #143, rind signs SHAKE-128f in about 23.8M cycles, about 32% fewer than the AVX-512-capable C reference and about 66% fewer than the AVX2-only C build. Before #143 rind was slower than the C reference on sign (38.7M against 34.9M, 1.11x) and keygen (1.64M against 1.50M), and faster on verify (2.64M against 2.96M). This run did not isolate whether the gain comes from AVX-512 paths or from the interleaved-state change alone.

Caveats: 2 rounds of rind only; the cycle conversion is approximate; the C -march=native build was compiled by the system compiler on this host and its vectorization is whatever that compiler did; the Arrow Lake figures are the ones supplied, not re-measured here.

Raw logs, the run script and `summary.json`: [sphincs-143/](sphincs-143/).
