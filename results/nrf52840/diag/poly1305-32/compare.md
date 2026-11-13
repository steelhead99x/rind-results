Cycles, poly1305-32 over hacl (below 1 is faster in poly1305-32).

## chacha20poly1305

Known-answer checks: hacl 21/21, poly1305-32 21/21. Runs: 3 and 3. Rows: 56 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 6 | 1.000 | 1.000 to 1.000 |
| libcrux | short | 8 | 1.000 | 1.000 to 1.000 |
| rind | long | 6 | 0.698 | 0.691 to 0.705 |
| rind | short | 8 | 0.690 | 0.658 to 0.699 |
| ring | long | 6 | 1.000 | 1.000 to 1.000 |
| ring | short | 8 | 1.000 | 0.998 to 1.001 |
| rustcrypto | long | 6 | 1.004 | 0.998 to 1.010 |
| rustcrypto | short | 8 | 0.999 | 0.997 to 1.008 |

14 of 56 rows moved by more than max(1%, run spread):

| Row | hacl cycles | poly1305-32 cycles | B/A |
|---|---:|---:|---:|
| `chacha20poly1305/rind/32B` | 9,877 | 6,503 | 0.658 |
| `chacha20poly1305-open/rind/32B` | 10,046 | 6,667 | 0.664 |
| `chacha20poly1305-open/rind/quic-128B` | 14,063 | 9,686 | 0.689 |
| `chacha20poly1305/rind/quic-128B` | 13,622 | 9,384 | 0.689 |
| `chacha20poly1305/rind/1200B` | 72,818 | 50,334 | 0.691 |
| `chacha20poly1305/rind/quic-256B` | 20,561 | 14,229 | 0.692 |
| `chacha20poly1305/rind/quic-512B` | 34,440 | 23,920 | 0.695 |
| `chacha20poly1305-open/rind/quic-256B` | 21,186 | 14,716 | 0.695 |
| `chacha20poly1305/rind/1350B` | 82,315 | 57,410 | 0.697 |
| `chacha20poly1305/rind/16KB` | 895,189 | 624,562 | 0.698 |
| `chacha20poly1305-open/rind/1200B` | 74,667 | 52,180 | 0.699 |
| `chacha20poly1305-open/rind/quic-512B` | 35,433 | 24,776 | 0.699 |
| `chacha20poly1305-open/rind/1350B` | 84,417 | 59,478 | 0.705 |
| `chacha20poly1305-open/rind/16KB` | 918,865 | 648,242 | 0.705 |

## xchacha20poly1305

Known-answer checks: hacl 21/21, poly1305-32 21/21. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | long | 3 | 1.001 | 0.998 to 1.006 |
| libcrux | short | 4 | 1.001 | 0.990 to 1.006 |
| rind | long | 3 | 0.910 | 0.763 to 0.931 |
| rind | short | 4 | 0.971 | 0.962 to 0.987 |
| rustcrypto | long | 3 | 0.993 | 0.991 to 0.999 |
| rustcrypto | short | 4 | 1.005 | 1.000 to 1.016 |

7 of 21 rows moved by more than max(1%, run spread):

| Row | hacl cycles | poly1305-32 cycles | B/A |
|---|---:|---:|---:|
| `xchacha20poly1305/rind/16KB` | 1,098,677 | 838,153 | 0.763 |
| `xchacha20poly1305/rind/1350B` | 297,393 | 270,641 | 0.910 |
| `xchacha20poly1305/rind/1200B` | 284,169 | 264,697 | 0.931 |
| `xchacha20poly1305/rind/quic-512B` | 244,513 | 235,321 | 0.962 |
| `xchacha20poly1305/rind/quic-256B` | 233,989 | 226,181 | 0.967 |
| `xchacha20poly1305/rind/quic-128B` | 227,485 | 221,701 | 0.975 |
| `xchacha20poly1305/rind/32B` | 222,925 | 219,949 | 0.987 |

