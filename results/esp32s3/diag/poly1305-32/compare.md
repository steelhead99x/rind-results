Cycles, poly1305-32 over hacl (below 1 is faster in poly1305-32).

## chacha20poly1305

Known-answer checks: hacl 21/21, poly1305-32 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 0.976 | 0.897 to 1.011 |
| libcrux | short | 8 | 0.897 | 0.497 to 1.321 |
| rind | long | 6 | 0.735 | 0.730 to 0.737 |
| rind | short | 8 | 0.721 | 0.696 to 0.731 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 1.000 to 1.000 |
| rustcrypto | long | 6 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 8 | 1.000 | 1.000 to 1.000 |

26 of 56 rows moved by more than max(1%, run spread):

| Row | hacl cycles | poly1305-32 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305-open/libcrux/32B` | 34,045 | 16,910 | 0.497 |
| `chacha20poly1305/libcrux/32B` | 12,097 | 15,977 | 1.321 |
| `chacha20poly1305/rind/32B` | 12,879 | 8,962 | 0.696 |
| `chacha20poly1305-open/rind/32B` | 12,984 | 9,076 | 0.699 |
| `chacha20poly1305/rind/quic-128B` | 20,216 | 14,475 | 0.716 |
| `chacha20poly1305-open/rind/quic-128B` | 20,397 | 14,649 | 0.718 |
| `chacha20poly1305-open/libcrux/quic-128B` | 64,335 | 46,300 | 0.720 |
| `chacha20poly1305/rind/quic-256B` | 32,711 | 23,669 | 0.724 |
| `chacha20poly1305-open/rind/quic-256B` | 32,966 | 23,938 | 0.726 |
| `chacha20poly1305/rind/quic-512B` | 57,721 | 42,051 | 0.729 |
| `chacha20poly1305/rind/1200B` | 126,525 | 92,387 | 0.730 |
| `chacha20poly1305-open/rind/quic-512B` | 58,139 | 42,477 | 0.731 |
| `chacha20poly1305-open/rind/1200B` | 127,379 | 93,173 | 0.731 |
| `chacha20poly1305/rind/1350B` | 143,125 | 105,056 | 0.734 |
| `chacha20poly1305/rind/16KB` | 1,610,381 | 1,184,190 | 0.735 |
| `chacha20poly1305-open/rind/1350B` | 144,083 | 105,963 | 0.735 |
| `chacha20poly1305-open/rind/16KB` | 1,621,040 | 1,194,335 | 0.737 |
| `chacha20poly1305-open/libcrux/quic-256B` | 77,614 | 59,776 | 0.770 |
| `chacha20poly1305-open/libcrux/quic-512B` | 104,744 | 86,272 | 0.824 |
| `chacha20poly1305-open/libcrux/1200B` | 153,397 | 137,537 | 0.897 |
| `chacha20poly1305-open/libcrux/1350B` | 200,985 | 188,506 | 0.938 |
| `chacha20poly1305/libcrux/1350B` | 170,595 | 163,734 | 0.960 |
| `chacha20poly1305/libcrux/quic-256B` | 45,961 | 44,576 | 0.970 |
| `chacha20poly1305/libcrux/quic-128B` | 32,454 | 31,488 | 0.970 |
| `chacha20poly1305/libcrux/quic-512B` | 72,652 | 70,534 | 0.971 |
| `chacha20poly1305/libcrux/1200B` | 131,504 | 132,949 | 1.011 |

## xchacha20poly1305

Known-answer checks: hacl 21/21, poly1305-32 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 0.991 | 0.963 to 0.999 |
| libcrux | short | 4 | 0.998 | 0.951 to 0.998 |
| rind | long | 3 | 0.663 | 0.644 to 0.719 |
| rind | short | 4 | 0.616 | 0.490 to 0.627 |
| rustcrypto | long | 3 | 1.000 | 1.000 to 1.000 |
| rustcrypto | short | 4 | 1.000 | 1.000 to 1.000 |

9 of 21 rows moved by more than max(1%, run spread):

| Row | hacl cycles | poly1305-32 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rind/32B` | 288,956 | 141,457 | 0.490 |
| `xchacha20poly1305/rind/quic-128B` | 317,231 | 194,695 | 0.614 |
| `xchacha20poly1305/rind/quic-256B` | 329,744 | 203,898 | 0.618 |
| `xchacha20poly1305/rind/quic-512B` | 354,764 | 222,301 | 0.627 |
| `xchacha20poly1305/rind/1200B` | 425,009 | 273,878 | 0.644 |
| `xchacha20poly1305/rind/1350B` | 445,856 | 295,542 | 0.663 |
| `xchacha20poly1305/rind/16KB` | 1,909,178 | 1,372,709 | 0.719 |
| `xchacha20poly1305/libcrux/32B` | 302,445 | 287,479 | 0.951 |
| `xchacha20poly1305/libcrux/1200B` | 419,505 | 404,035 | 0.963 |

