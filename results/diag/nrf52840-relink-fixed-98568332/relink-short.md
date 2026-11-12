rind's time over the fastest other Rust library, per layout (pad0, pad1031, pad2053), each the median over that layout's runs; then the range over layouts, and the ratio of the medians over layouts.

| Operation | pad0 | pad1031 | pad2053 | range | median-layout ratio | rind's cycles, spread over layouts |
|---|---:|---:|---:|---|---:|---:|
| chacha20poly1305/…/32B | 1.73× ring | 1.73× ring | 1.73× ring | 1.73× to 1.73× | 1.73× ring | 0.0% |
| sha256/…/64B | 1.43× ring | 1.44× ring | 1.44× ring | 1.43× to 1.44× | 1.44× ring | 0.2% |
| sha512/…/64B | 1.18× ring | 1.18× ring | 1.18× ring | 1.18× to 1.18× | 1.18× ring | 0.1% |
| hmac-sha256/…/64B | 1.23× ring | 1.23× ring | 1.23× ring | 1.23× to 1.23× | 1.23× ring | 0.1% |
| hkdf-sha256/extract-expand/… | 1.20× ring | 1.20× ring | 1.20× ring | 1.20× to 1.20× | 1.20× ring | 0.1% |
| hkdf-sha256/expand/… | 1.31× rustcrypto | 1.31× rustcrypto | 1.31× rustcrypto | 1.31× to 1.31× | 1.31× rustcrypto | 0.2% |

6 operations; the ratio's range over layouts: median 0.00, largest 0.00.
