Cycles, icache32 over published (below 1 is faster in icache32).

## chacha20poly1305

Known-answer checks: published 21/21, icache32 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 0.918 | 0.728 to 0.995 |
| libcrux | short | 8 | 0.566 | 0.297 to 0.984 |
| rind | long | 6 | 1.000 | 1.000 to 1.000 |
| rind | short | 8 | 1.000 | 1.000 to 1.000 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

13 of 56 rows moved by more than max(1%, run spread):

| Row | published cycles | icache32 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/libcrux/quic-128B` | 64,335 | 19,083 | 0.297 |
| `chacha20poly1305-open/libcrux/32B` | 34,045 | 12,102 | 0.355 |
| `chacha20poly1305-open/libcrux/quic-256B` | 77,614 | 31,971 | 0.412 |
| `chacha20poly1305-open/libcrux/quic-512B` | 104,744 | 57,751 | 0.551 |
| `chacha20poly1305/libcrux/quic-128B` | 32,454 | 18,870 | 0.581 |
| `chacha20poly1305/libcrux/quic-256B` | 45,961 | 31,768 | 0.691 |
| `chacha20poly1305-open/libcrux/1350B` | 200,985 | 146,284 | 0.728 |
| `chacha20poly1305/libcrux/quic-512B` | 72,652 | 57,500 | 0.791 |
| `chacha20poly1305-open/libcrux/1200B` | 153,397 | 129,004 | 0.841 |
| `chacha20poly1305/libcrux/1350B` | 170,595 | 146,060 | 0.856 |
| `chacha20poly1305/libcrux/1200B` | 131,504 | 128,801 | 0.979 |
| `chacha20poly1305-open/libcrux/16KB` | 1,687,563 | 1,656,926 | 0.982 |
| `chacha20poly1305/libcrux/32B` | 12,097 | 11,899 | 0.984 |

