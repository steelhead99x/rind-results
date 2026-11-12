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
| xchacha20poly1305/…/32B | 1.03× rustcrypto | 1.03× rustcrypto | 1.03× to 1.03× | 1.03× rustcrypto | 0.3% |
| xchacha20poly1305/…/1350B | 1.06× rustcrypto | 1.07× rustcrypto | 1.06× to 1.07× | 1.06× rustcrypto | 0.4% |
| xchacha20poly1305/…/16KB | 1.17× rustcrypto | 1.15× rustcrypto | 1.15× to 1.17× | 1.16× rustcrypto | 0.1% |
| quic-header-protection/aes128/… | 0.73× ring | 0.74× ring | 0.73× to 0.74× | 0.73× ring | 0.0% |
| quic-header-protection/aes256/… | 0.74× ring | 0.74× ring | 0.74× to 0.74× | 0.74× ring | 0.0% |
| quic-header-protection/chacha20/… | 0.98× ring | 0.95× ring | 0.95× to 0.98× | 0.96× ring | 1.5% |
| x25519-ecdh/… | 1.00× rustcrypto | 1.00× rustcrypto | 1.00× to 1.00× | 1.00× rustcrypto | 0.3% |
| p256-ecdh/… | 0.43× ring | 0.43× ring | 0.43× to 0.43× | 0.43× ring | 0.0% |
| ed25519-sign/… | 0.98× rustcrypto | 0.98× rustcrypto | 0.98× to 0.98× | 0.98× rustcrypto | 0.1% |
| ed25519-verify/… | 1.03× ring | 1.03× ring | 1.03× to 1.03× | 1.03× ring | 0.0% |
| p256-ecdsa-sign/… | 0.40× ring | 0.40× ring | 0.40× to 0.40× | 0.40× ring | 0.3% |
| p256-ecdsa-verify/… | 0.42× ring | 0.42× ring | 0.42× to 0.42× | 0.42× ring | 0.0% |
| sha256/…/64B | 1.43× ring | 1.41× ring | 1.41× to 1.43× | 1.42× ring | 0.4% |
| sha256/…/1350B | 1.60× ring | 1.60× ring | 1.60× to 1.60× | 1.60× ring | 0.2% |
| sha256/…/16KB | 1.62× ring | 1.62× ring | 1.62× to 1.62× | 1.62× ring | 0.2% |
| sha512/…/64B | 1.18× ring | 1.19× ring | 1.18× to 1.19× | 1.18× ring | 0.2% |
| sha512/…/1350B | 1.20× ring | 1.20× ring | 1.20× to 1.20× | 1.20× ring | 0.1% |
| sha512/…/16KB | 1.20× ring | 1.20× ring | 1.20× to 1.20× | 1.20× ring | 0.1% |
| sha3-256/…/64B | 1.11× libcrux | 1.11× libcrux | 1.11× to 1.11× | 1.11× libcrux | 0.1% |
| sha3-256/…/1350B | 1.04× libcrux | 1.04× libcrux | 1.04× to 1.04× | 1.04× libcrux | 0.0% |
| sha3-256/…/16KB | 1.03× libcrux | 1.03× libcrux | 1.03× to 1.03× | 1.03× libcrux | 0.0% |
| sha3-512/…/64B | 1.11× libcrux | 1.12× libcrux | 1.11× to 1.12× | 1.11× libcrux | 0.0% |
| sha3-512/…/1350B | 1.02× libcrux | 1.03× libcrux | 1.02× to 1.03× | 1.03× libcrux | 0.0% |
| sha3-512/…/16KB | 1.02× libcrux | 1.02× libcrux | 1.02× to 1.02× | 1.02× libcrux | 0.1% |
| hmac-sha256/…/64B | 1.22× ring | 1.23× ring | 1.22× to 1.23× | 1.23× ring | 0.0% |
| hmac-sha256/…/1350B | 1.51× ring | 1.51× ring | 1.51× to 1.51× | 1.51× ring | 0.2% |
| hkdf-sha256/extract-expand/… | 1.20× ring | 1.21× ring | 1.20× to 1.21× | 1.21× ring | 0.7% |
| hkdf-sha256/expand/… | 1.32× rustcrypto | 1.31× rustcrypto | 1.31× to 1.32× | 1.31× rustcrypto | 0.0% |
| mlkem768-keygen/… | 0.96× libcrux | 0.97× libcrux | 0.96× to 0.97× | 0.97× libcrux | 0.2% |
| mlkem768-encaps/… | 0.95× libcrux | 0.95× libcrux | 0.95× to 0.95× | 0.95× libcrux | 0.0% |
| mlkem768-decaps/… | 0.46× libcrux | 0.46× libcrux | 0.46× to 0.46× | 0.46× libcrux | 0.4% |

46 operations; the ratio's range over layouts: median 0.01, largest 0.03.
