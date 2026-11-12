rind's time over the fastest other Rust library, per layout (pad0, pad1031, pad2053), each the median over that layout's runs; then the range over layouts, and the ratio of the medians over layouts.

| Operation | pad0 | pad1031 | pad2053 | range | median-layout ratio | rind's cycles, spread over layouts |
|---|---:|---:|---:|---|---:|---:|
| aes128-gcm/…/32B | 0.53× rustcrypto | 0.58× ring | 0.58× ring | 0.53× to 0.58× | 0.58× ring | 0.0% |
| aes128-gcm/…/1350B | 0.82× ring | 0.82× ring | 0.82× ring | 0.82× to 0.82× | 0.82× ring | 0.0% |
| aes128-gcm/…/16KB | 0.88× rustcrypto | 0.88× rustcrypto | 0.88× rustcrypto | 0.88× to 0.88× | 0.88× rustcrypto | 0.0% |
| aes128-gcm-open/…/32B | 0.66× ring | 0.77× ring | 0.77× ring | 0.66× to 0.77× | 0.77× ring | 0.8% |
| aes128-gcm-open/…/1350B | 0.85× ring | 0.86× ring | 0.86× ring | 0.85× to 0.86× | 0.86× ring | 0.9% |
| aes128-gcm-open/…/16KB | 0.87× rustcrypto | 0.87× rustcrypto | 0.87× rustcrypto | 0.87× to 0.87× | 0.87× rustcrypto | 0.1% |
| aes256-gcm/…/32B | 0.56× ring | 0.62× ring | 0.62× ring | 0.56× to 0.62× | 0.62× ring | 0.0% |
| aes256-gcm/…/1350B | 0.81× ring | 0.83× rustcrypto | 0.83× rustcrypto | 0.81× to 0.83× | 0.83× rustcrypto | 0.0% |
| aes256-gcm/…/16KB | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× to 0.91× | 0.91× rustcrypto | 0.0% |
| aes256-gcm-open/…/32B | 0.66× ring | 0.74× ring | 0.74× ring | 0.66× to 0.74× | 0.74× ring | 0.8% |
| aes256-gcm-open/…/1350B | 0.84× ring | 0.84× ring | 0.84× ring | 0.84× to 0.84× | 0.84× ring | 0.7% |
| aes256-gcm-open/…/16KB | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× to 0.91× | 0.91× rustcrypto | 0.1% |
| chacha20poly1305/…/32B | 1.45× ring | 1.45× ring | 1.45× ring | 1.45× to 1.45× | 1.45× ring | 0.0% |
| chacha20poly1305/…/1350B | 1.27× ring | 1.27× ring | 1.27× ring | 1.27× to 1.27× | 1.27× ring | 0.0% |
| chacha20poly1305/…/16KB | 1.26× ring | 1.26× ring | 1.26× ring | 1.26× to 1.26× | 1.26× ring | 0.0% |
| xchacha20poly1305/…/32B | 7.50× rustcrypto | 7.93× rustcrypto | 7.93× rustcrypto | 7.50× to 7.93× | 7.93× rustcrypto | 5.4% |
| xchacha20poly1305/…/1350B | 2.72× rustcrypto | 2.74× rustcrypto | 2.74× rustcrypto | 2.72× to 2.74× | 2.74× rustcrypto | 1.0% |
| xchacha20poly1305/…/16KB | 1.21× rustcrypto | 1.22× rustcrypto | 1.22× rustcrypto | 1.21× to 1.22× | 1.22× rustcrypto | 0.2% |
| quic-header-protection/aes128/… | 0.65× ring | 0.65× ring | 0.65× ring | 0.65× to 0.65× | 0.65× ring | 0.0% |
| quic-header-protection/aes256/… | 0.65× ring | 0.65× ring | 0.65× ring | 0.65× to 0.65× | 0.65× ring | 0.0% |
| quic-header-protection/chacha20/… | 0.86× ring | 0.86× ring | 0.86× ring | 0.86× to 0.86× | 0.86× ring | 0.0% |
| x25519-ecdh/… | 1.00× rustcrypto | 1.00× rustcrypto | 1.00× rustcrypto | 1.00× to 1.00× | 1.00× rustcrypto | 0.1% |
| p256-ecdh/… | 0.59× rustcrypto | 0.58× rustcrypto | 0.58× rustcrypto | 0.58× to 0.59× | 0.58× rustcrypto | 0.3% |
| ed25519-sign/… | 0.79× rustcrypto | 0.79× rustcrypto | 0.79× rustcrypto | 0.79× to 0.79× | 0.79× rustcrypto | 0.6% |
| ed25519-verify/… | 0.99× rustcrypto | 0.99× rustcrypto | 0.99× rustcrypto | 0.99× to 0.99× | 0.99× rustcrypto | 0.1% |
| p256-ecdsa-sign/… | 0.97× rustcrypto | 0.93× rustcrypto | 0.93× rustcrypto | 0.93× to 0.97× | 0.93× rustcrypto | 0.1% |
| p256-ecdsa-verify/… | 0.60× rustcrypto | 0.60× rustcrypto | 0.60× rustcrypto | 0.60× to 0.60× | 0.60× rustcrypto | 0.3% |
| sha256/…/64B | 2.36× libcrux | 3.55× libcrux | 2.78× libcrux | 2.36× to 3.55× | 2.78× libcrux | 42.3% |
| sha256/…/1350B | 1.17× libcrux | 1.28× libcrux | 1.21× libcrux | 1.17× to 1.28× | 1.21× libcrux | 8.6% |
| sha256/…/16KB | 1.02× libcrux | 1.02× libcrux | 1.02× libcrux | 1.02× to 1.02× | 1.02× libcrux | 0.7% |
| sha512/…/64B | 32.56× ring | 32.57× ring | 32.57× ring | 32.56× to 32.57× | 32.57× ring | 0.0% |
| sha512/…/1350B | 32.96× ring | 32.96× ring | 32.96× ring | 32.96× to 32.96× | 32.96× ring | 0.0% |
| sha512/…/16KB | 32.95× ring | 32.95× ring | 32.95× ring | 32.95× to 32.95× | 32.95× ring | 0.0% |
| sha3-256/…/64B | 1.01× libcrux | 1.01× libcrux | 1.01× libcrux | 1.01× to 1.01× | 1.01× libcrux | 0.0% |
| sha3-256/…/1350B | 0.96× libcrux | 0.96× libcrux | 0.96× libcrux | 0.96× to 0.96× | 0.96× libcrux | 0.0% |
| sha3-256/…/16KB | 0.95× libcrux | 0.95× libcrux | 0.95× libcrux | 0.95× to 0.95× | 0.95× libcrux | 0.0% |
| sha3-512/…/64B | 1.07× libcrux | 1.07× libcrux | 1.07× libcrux | 1.07× to 1.07× | 1.07× libcrux | 0.0% |
| sha3-512/…/1350B | 0.89× libcrux | 0.89× libcrux | 0.89× libcrux | 0.89× to 0.89× | 0.89× libcrux | 0.0% |
| sha3-512/…/16KB | 0.88× libcrux | 0.88× libcrux | 0.88× libcrux | 0.88× to 0.88× | 0.88× libcrux | 0.0% |
| hmac-sha256/…/64B | 2.20× ring | 2.35× ring | 2.33× ring | 2.20× to 2.35× | 2.33× ring | 6.3% |
| hmac-sha256/…/1350B | 1.23× libcrux | 1.27× libcrux | 1.27× libcrux | 1.23× to 1.27× | 1.27× libcrux | 2.3% |
| hkdf-sha256/extract-expand/… | 5.50× ring | 5.88× ring | 5.88× ring | 5.50× to 5.88× | 5.88× ring | 6.5% |
| hkdf-sha256/expand/… | 6.93× ring | 7.08× ring | 7.09× ring | 6.93× to 7.09× | 7.08× ring | 2.3% |
| mlkem768-keygen/… | 0.51× rustcrypto | 0.51× rustcrypto | 0.51× rustcrypto | 0.51× to 0.51× | 0.51× rustcrypto | 0.4% |
| mlkem768-encaps/… | 0.47× rustcrypto | 0.47× rustcrypto | 0.47× rustcrypto | 0.47× to 0.47× | 0.47× rustcrypto | 0.3% |
| mlkem768-decaps/… | 0.28× rustcrypto | 0.28× rustcrypto | 0.28× rustcrypto | 0.28× to 0.28× | 0.28× rustcrypto | 0.8% |

46 operations; the ratio's range over layouts: median 0.00, largest 1.18.
