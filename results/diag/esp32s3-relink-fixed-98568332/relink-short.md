rind's time over the fastest other Rust library, per layout (pad0, pad1031, pad2053), each the median over that layout's runs; then the range over layouts, and the ratio of the medians over layouts.

| Operation | pad0 | pad1031 | pad2053 | range | median-layout ratio | rind's cycles, spread over layouts |
|---|---:|---:|---:|---|---:|---:|
| aes128-gcm/…/32B | 0.53× rustcrypto | 0.52× rustcrypto | 0.52× rustcrypto | 0.52× to 0.53× | 0.52× rustcrypto | 0.0% |
| aes128-gcm-open/…/32B | 0.66× ring | 0.76× ring | 0.76× ring | 0.66× to 0.76× | 0.76× ring | 0.7% |
| aes256-gcm/…/32B | 0.56× ring | 0.61× ring | 0.61× ring | 0.56× to 0.61× | 0.61× ring | 0.0% |
| aes256-gcm-open/…/32B | 0.66× ring | 0.74× ring | 0.74× ring | 0.66× to 0.74× | 0.74× ring | 0.7% |
| chacha20poly1305/…/32B | 1.45× ring | 1.45× ring | 1.45× ring | 1.45× to 1.45× | 1.45× ring | 0.0% |
| xchacha20poly1305/…/32B | 7.50× rustcrypto | 7.94× rustcrypto | 7.94× rustcrypto | 7.50× to 7.94× | 7.94× rustcrypto | 5.5% |
| quic-header-protection/aes128/… | 0.65× ring | 0.65× ring | 0.65× ring | 0.65× to 0.65× | 0.65× ring | 0.0% |
| quic-header-protection/aes256/… | 0.65× ring | 0.65× ring | 0.65× ring | 0.65× to 0.65× | 0.65× ring | 0.0% |
| quic-header-protection/chacha20/… | 0.86× ring | 0.86× ring | 0.86× ring | 0.86× to 0.86× | 0.86× ring | 0.0% |
| sha256/…/64B | 2.36× libcrux | 2.80× libcrux | 2.80× libcrux | 2.36× to 2.80× | 2.80× libcrux | 15.2% |
| sha512/…/64B | 32.56× ring | 32.57× ring | 32.57× ring | 32.56× to 32.57× | 32.57× ring | 0.0% |
| sha3-256/…/64B | 1.01× libcrux | 1.01× libcrux | 1.01× libcrux | 1.01× to 1.01× | 1.01× libcrux | 0.0% |
| sha3-512/…/64B | 1.07× libcrux | 1.07× libcrux | 1.07× libcrux | 1.07× to 1.07× | 1.07× libcrux | 0.0% |
| hmac-sha256/…/64B | 2.20× ring | 2.34× ring | 2.34× ring | 2.20× to 2.34× | 2.34× ring | 6.0% |
| hkdf-sha256/extract-expand/… | 5.50× ring | 5.81× ring | 5.81× ring | 5.50× to 5.81× | 5.81× ring | 5.4% |
| hkdf-sha256/expand/… | 6.93× ring | 7.02× ring | 7.02× ring | 6.93× to 7.02× | 7.02× ring | 1.4% |

16 operations; the ratio's range over layouts: median 0.05, largest 0.44.
