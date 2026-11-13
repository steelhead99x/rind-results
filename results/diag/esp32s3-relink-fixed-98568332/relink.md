rind's time over the fastest other Rust library, per layout (pad0, pad1031, pad1039), each the median over that layout's runs; then the range over layouts, and the ratio of the medians over layouts.

| Operation | pad0 | pad1031 | pad1039 | range | median-layout ratio | rind's cycles, spread over layouts |
|---|---:|---:|---:|---|---:|---:|
| aes128-gcm/…/32B | 0.53× rustcrypto | 0.52× rustcrypto | 0.52× ring | 0.52× to 0.53× | 0.52× rustcrypto | 0.0% |
| aes128-gcm/…/1350B | 0.82× ring | 0.82× ring | 0.82× ring | 0.82× to 0.82× | 0.82× ring | 0.0% |
| aes128-gcm/…/16KB | 0.88× rustcrypto | 0.88× rustcrypto | 0.88× rustcrypto | 0.88× to 0.88× | 0.88× rustcrypto | 0.0% |
| aes128-gcm-open/…/32B | 0.66× ring | 0.76× ring | 0.77× ring | 0.66× to 0.77× | 0.76× ring | 1.7% |
| aes128-gcm-open/…/1350B | 0.85× ring | 0.86× ring | 0.87× ring | 0.85× to 0.87× | 0.87× ring | 1.1% |
| aes128-gcm-open/…/16KB | 0.87× rustcrypto | 0.87× rustcrypto | 0.87× rustcrypto | 0.87× to 0.87× | 0.87× rustcrypto | 0.2% |
| aes256-gcm/…/32B | 0.56× ring | 0.61× ring | 0.56× ring | 0.56× to 0.61× | 0.56× ring | 0.0% |
| aes256-gcm/…/1350B | 0.81× ring | 0.83× rustcrypto | 0.82× rustcrypto | 0.81× to 0.83× | 0.82× rustcrypto | 0.0% |
| aes256-gcm/…/16KB | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× to 0.91× | 0.91× rustcrypto | 0.0% |
| aes256-gcm-open/…/32B | 0.66× ring | 0.74× ring | 0.75× ring | 0.66× to 0.75× | 0.74× ring | 1.7% |
| aes256-gcm-open/…/1350B | 0.84× ring | 0.84× ring | 0.85× ring | 0.84× to 0.85× | 0.85× ring | 0.9% |
| aes256-gcm-open/…/16KB | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× rustcrypto | 0.91× to 0.91× | 0.91× rustcrypto | 0.1% |
| chacha20poly1305/…/32B | 1.45× ring | 1.45× ring | 1.45× ring | 1.45× to 1.45× | 1.45× ring | 0.0% |
| chacha20poly1305/…/1350B | 1.27× ring | 1.27× ring | 1.27× ring | 1.27× to 1.27× | 1.27× ring | 0.0% |
| chacha20poly1305/…/16KB | 1.26× ring | 1.26× ring | 1.26× ring | 1.26× to 1.26× | 1.26× ring | 0.0% |
| xchacha20poly1305/…/32B | 7.50× rustcrypto | 7.94× rustcrypto | 7.59× rustcrypto | 7.50× to 7.94× | 7.59× rustcrypto | 5.8% |
| xchacha20poly1305/…/1350B | 2.72× rustcrypto | 2.75× rustcrypto | 2.73× rustcrypto | 2.72× to 2.75× | 2.73× rustcrypto | 1.1% |
| xchacha20poly1305/…/16KB | 1.21× rustcrypto | 1.22× rustcrypto | 1.21× rustcrypto | 1.21× to 1.22× | 1.21× rustcrypto | 0.3% |
| quic-header-protection/aes128/… | 0.65× ring | 0.65× ring | 0.65× ring | 0.65× to 0.65× | 0.65× ring | 0.0% |
| quic-header-protection/aes256/… | 0.65× ring | 0.65× ring | 0.65× ring | 0.65× to 0.65× | 0.65× ring | 0.0% |
| quic-header-protection/chacha20/… | 0.86× ring | 0.86× ring | 0.86× ring | 0.86× to 0.86× | 0.86× ring | 0.0% |
| sha256/…/64B | 2.36× libcrux | 2.80× libcrux | 3.07× libcrux | 2.36× to 3.07× | 2.79× libcrux | 26.0% |
| sha256/…/1350B | 1.17× libcrux | 1.21× libcrux | 1.18× libcrux | 1.17× to 1.21× | 1.19× libcrux | 3.3% |
| sha256/…/16KB | 1.02× libcrux | 1.02× libcrux | 1.01× libcrux | 1.01× to 1.02× | 1.02× libcrux | 0.4% |
| sha512/…/64B | 32.56× ring | 32.57× ring | 32.61× ring | 32.56× to 32.61× | 32.57× ring | 0.1% |
| sha512/…/1350B | 32.96× ring | 32.97× ring | 32.97× ring | 32.96× to 32.97× | 32.97× ring | 0.0% |
| sha512/…/16KB | 32.95× ring | 32.95× ring | 32.95× ring | 32.95× to 32.95× | 32.95× ring | 0.0% |
| sha3-256/…/64B | 1.01× libcrux | 1.01× libcrux | 1.01× libcrux | 1.01× to 1.01× | 1.01× libcrux | 0.0% |
| sha3-256/…/1350B | 0.96× libcrux | 0.96× libcrux | 0.96× libcrux | 0.96× to 0.96× | 0.96× libcrux | 0.0% |
| sha3-256/…/16KB | 0.95× libcrux | 0.95× libcrux | 0.95× libcrux | 0.95× to 0.95× | 0.95× libcrux | 0.0% |
| sha3-512/…/64B | 1.07× libcrux | 1.07× libcrux | 1.07× libcrux | 1.07× to 1.07× | 1.07× libcrux | 0.0% |
| sha3-512/…/1350B | 0.89× libcrux | 0.89× libcrux | 0.89× libcrux | 0.89× to 0.89× | 0.89× libcrux | 0.0% |
| sha3-512/…/16KB | 0.88× libcrux | 0.88× libcrux | 0.88× libcrux | 0.88× to 0.88× | 0.88× libcrux | 0.0% |
| hmac-sha256/…/64B | 2.20× ring | 2.34× ring | 2.07× ring | 2.07× to 2.34× | 2.20× ring | 12.2% |
| hmac-sha256/…/1350B | 1.23× libcrux | 1.30× libcrux | 1.22× libcrux | 1.22× to 1.30× | 1.24× libcrux | 5.2% |
| hkdf-sha256/extract-expand/… | 5.50× ring | 5.81× ring | 5.79× ring | 5.50× to 5.81× | 5.79× ring | 5.4% |
| hkdf-sha256/expand/… | 6.93× ring | 7.02× ring | 7.11× ring | 6.93× to 7.11× | 7.02× ring | 2.6% |

37 operations; the ratio's range over layouts: median 0.01, largest 0.71.
