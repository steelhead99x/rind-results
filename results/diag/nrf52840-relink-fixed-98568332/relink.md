rind's time over the fastest other Rust library, per layout (pad0, pad1031, pad2053), each the median over that layout's runs; then the range over layouts, and the ratio of the medians over layouts.

| Operation | pad0 | pad1031 | pad2053 | range | median-layout ratio | rind's cycles, spread over layouts |
|---|---:|---:|---:|---|---:|---:|
| chacha20poly1305/…/32B | 1.73× ring | 1.73× ring | 1.73× ring | 1.73× to 1.73× | 1.73× ring | 0.0% |
| chacha20poly1305/…/1350B | 1.77× ring | 1.77× ring | 1.77× ring | 1.77× to 1.77× | 1.77× ring | 0.0% |
| chacha20poly1305/…/16KB | 1.78× ring | 1.78× ring | 1.78× ring | 1.78× to 1.78× | 1.78× ring | 0.0% |
| sha256/…/64B | 1.43× ring | 1.44× ring | 1.44× ring | 1.43× to 1.44× | 1.44× ring | 0.2% |
| sha256/…/1350B | 1.60× ring | 1.60× ring | 1.60× ring | 1.60× to 1.60× | 1.60× ring | 0.0% |
| sha256/…/16KB | 1.62× ring | 1.62× ring | 1.62× ring | 1.62× to 1.62× | 1.62× ring | 0.0% |
| sha512/…/64B | 1.18× ring | 1.18× ring | 1.18× ring | 1.18× to 1.18× | 1.18× ring | 0.1% |
| sha512/…/1350B | 1.19× ring | 1.19× ring | 1.19× ring | 1.19× to 1.19× | 1.19× ring | 0.0% |
| sha512/…/16KB | 1.20× ring | 1.20× ring | 1.20× ring | 1.20× to 1.20× | 1.20× ring | 0.0% |
| hmac-sha256/…/64B | 1.23× ring | 1.23× ring | 1.23× ring | 1.23× to 1.23× | 1.23× ring | 0.1% |
| hmac-sha256/…/1350B | 1.51× ring | 1.51× ring | 1.51× ring | 1.51× to 1.51× | 1.51× ring | 0.2% |
| hkdf-sha256/extract-expand/… | 1.20× ring | 1.20× ring | 1.20× ring | 1.20× to 1.20× | 1.20× ring | 0.1% |
| hkdf-sha256/expand/… | 1.31× rustcrypto | 1.31× rustcrypto | 1.31× rustcrypto | 1.31× to 1.31× | 1.31× rustcrypto | 0.2% |

13 operations; the ratio's range over layouts: median 0.00, largest 0.00.
