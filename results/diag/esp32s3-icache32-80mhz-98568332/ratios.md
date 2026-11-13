rind's time over the fastest other Rust library, per layout (published, icache32-80mhz), each the median over that layout's runs; then the range over layouts, and the ratio of the medians over layouts.

| Operation | published | icache32-80mhz | range | median-layout ratio | rind's cycles, spread over layouts |
|---|---:|---:|---|---:|---:|
| chacha20poly1305/…/32B | 1.45× ring | 1.45× ring | 1.45× to 1.45× | 1.45× ring | 0.0% |
| chacha20poly1305/…/1350B | 1.27× ring | 1.27× ring | 1.27× to 1.27× | 1.27× ring | 0.0% |
| chacha20poly1305/…/16KB | 1.26× ring | 1.26× ring | 1.26× to 1.26× | 1.26× ring | 0.0% |
| xchacha20poly1305/…/32B | 7.50× rustcrypto | 1.10× rustcrypto | 1.10× to 7.50× | 4.30× rustcrypto | 148.9% |
| xchacha20poly1305/…/1350B | 2.72× rustcrypto | 1.05× rustcrypto | 1.05× to 2.72× | 1.88× rustcrypto | 88.4% |
| xchacha20poly1305/…/16KB | 1.21× rustcrypto | 1.04× rustcrypto | 1.04× to 1.21× | 1.13× rustcrypto | 15.4% |
| x25519-ecdh/… | 1.00× rustcrypto | 1.00× rustcrypto | 1.00× to 1.00× | 1.00× rustcrypto | 6.6% |
| p256-ecdh/… | 0.59× rustcrypto | 0.70× ring | 0.59× to 0.70× | 0.57× rustcrypto | 8.9% |
| p256-ecdsa-sign/… | 0.97× rustcrypto | 0.96× ring | 0.96× to 0.97× | 0.95× rustcrypto | 18.2% |
| p256-ecdsa-verify/… | 0.60× rustcrypto | 0.66× ring | 0.60× to 0.66× | 0.59× rustcrypto | 6.4% |

10 operations; the ratio's range over layouts: median 0.06, largest 6.40.
