Cycles, icache32 over published (below 1 is faster in icache32).

## xchacha20poly1305

Known-answer checks: published 21/21, icache32 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 0.377 | 0.366 to 0.865 |
| libcrux | short | 4 | 0.152 | 0.136 to 0.223 |
| rind | long | 3 | 0.387 | 0.364 to 0.857 |
| rind | short | 4 | 0.170 | 0.146 to 0.243 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

14 of 21 rows moved by more than max(1%, run spread):

| Row | published cycles | icache32 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/libcrux/32B` | 302,388 | 41,023 | 0.136 |
| `xchacha20poly1305/libcrux/quic-128B` | 350,464 | 48,004 | 0.137 |
| `xchacha20poly1305/rind/32B` | 288,632 | 42,247 | 0.146 |
| `xchacha20poly1305/rind/quic-128B` | 320,984 | 49,575 | 0.154 |
| `xchacha20poly1305/libcrux/quic-256B` | 363,322 | 60,887 | 0.168 |
| `xchacha20poly1305/rind/quic-256B` | 333,500 | 62,084 | 0.186 |
| `xchacha20poly1305/libcrux/quic-512B` | 389,191 | 86,651 | 0.223 |
| `xchacha20poly1305/rind/quic-512B` | 358,520 | 87,104 | 0.243 |
| `xchacha20poly1305/rind/1200B` | 428,783 | 155,890 | 0.364 |
| `xchacha20poly1305/libcrux/1350B` | 478,861 | 175,232 | 0.366 |
| `xchacha20poly1305/libcrux/1200B` | 418,971 | 157,915 | 0.377 |
| `xchacha20poly1305/rind/1350B` | 445,838 | 172,556 | 0.387 |
| `xchacha20poly1305/rind/16KB` | 1,912,913 | 1,639,923 | 0.857 |
| `xchacha20poly1305/libcrux/16KB` | 1,949,460 | 1,685,940 | 0.865 |

