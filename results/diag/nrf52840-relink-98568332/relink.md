rind's time over the fastest other Rust library, per layout (pad0, pad1031), each the median over that layout's runs; then the range over layouts, and the ratio of the medians over layouts.

| Operation | pad0 | pad1031 | range | median-layout ratio | rind's cycles, spread over layouts |
|---|---:|---:|---|---:|---:|
| aes128-gcm/…/32B | 0.91× rustcrypto | 0.90× rustcrypto | 0.90× to 0.91× | 0.91× rustcrypto | 0.0% |
| aes128-gcm/…/1350B | 1.03× rustcrypto | 1.02× rustcrypto | 1.02× to 1.03× | 1.02× rustcrypto | 0.3% |
| aes128-gcm/…/16KB | 1.05× rustcrypto | 1.03× rustcrypto | 1.03× to 1.05× | 1.04× rustcrypto | 0.3% |
| aes128-gcm-open/…/32B | 0.89× rustcrypto | 0.88× rustcrypto | 0.88× to 0.89× | 0.89× rustcrypto | 0.1% |
| aes128-gcm-open/…/1350B | 1.03× rustcrypto | 1.01× rustcrypto | 1.01× to 1.03× | 1.02× rustcrypto | 0.2% |
| aes128-gcm-open/…/16KB | 1.05× rustcrypto | 1.03× rustcrypto | 1.03× to 1.05× | 1.04× rustcrypto | 0.3% |
| aes256-gcm/…/32B | 0.89× rustcrypto | 0.88× rustcrypto | 0.88× to 0.89× | 0.88× rustcrypto | 0.1% |
| aes256-gcm/…/1350B | 1.03× rustcrypto | 1.02× rustcrypto | 1.02× to 1.03× | 1.02× rustcrypto | 0.2% |
| aes256-gcm/…/16KB | 1.05× rustcrypto | 1.04× rustcrypto | 1.04× to 1.05× | 1.04× rustcrypto | 0.2% |
| aes256-gcm-open/…/32B | 0.89× rustcrypto | 0.88× rustcrypto | 0.88× to 0.89× | 0.88× rustcrypto | 0.2% |
| aes256-gcm-open/…/1350B | 1.03× rustcrypto | 1.02× rustcrypto | 1.02× to 1.03× | 1.02× rustcrypto | 0.1% |
| aes256-gcm-open/…/16KB | 1.05× rustcrypto | 1.04× rustcrypto | 1.04× to 1.05× | 1.04× rustcrypto | 0.2% |
| chacha20poly1305/…/32B | 1.74× ring | 1.74× ring | 1.74× to 1.74× | 1.74× ring | 0.1% |
| chacha20poly1305/…/1350B | 1.78× ring | 1.76× ring | 1.76× to 1.78× | 1.77× ring | 0.9% |
| chacha20poly1305/…/16KB | 1.79× ring | 1.77× ring | 1.77× to 1.79× | 1.78× ring | 1.1% |
| sha256/…/64B | 1.43× ring | 1.41× ring | 1.41× to 1.43× | 1.42× ring | 0.4% |
| sha256/…/1350B | 1.60× ring | 1.60× ring | 1.60× to 1.60× | 1.60× ring | 0.2% |
| sha256/…/16KB | 1.62× ring | 1.62× ring | 1.62× to 1.62× | 1.62× ring | 0.2% |
| sha512/…/64B | 1.18× ring | 1.19× ring | 1.18× to 1.19× | 1.18× ring | 0.2% |
| sha512/…/1350B | 1.20× ring | 1.20× ring | 1.20× to 1.20× | 1.20× ring | 0.1% |
| sha512/…/16KB | 1.20× ring | 1.20× ring | 1.20× to 1.20× | 1.20× ring | 0.1% |

21 operations; the ratio's range over layouts: median 0.01, largest 0.02.
