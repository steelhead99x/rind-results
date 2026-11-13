Cycles, icache32 over published (below 1 is faster in icache32).

## p256

Known-answer checks: published 12/12, icache32 12/12. Runs: 3 and 3. Rows: 21 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| libcrux | per op | 4 | 0.960 | 0.959 to 0.961 |
| p256-rustcrypto | per op | 2 | 0.957 | 0.941 to 0.973 |
| rind | per op | 9 | 0.910 | 0.833 to 0.985 |
| ring | per op | 4 | 0.096 | 0.085 to 0.132 |
| rustcrypto | per op | 2 | 0.930 | 0.869 to 0.992 |

20 of 21 rows moved by more than max(1%, run spread):

| Row | published cycles | icache32 cycles | B/A |
|---|---:|---:|---:|
| `p256-ecdsa-verify/ring` | 174,927,880 | 14,867,866 | 0.085 |
| `p256-ecdh/ring` | 175,475,134 | 15,444,112 | 0.088 |
| `p256-keygen/ring` | 40,190,689 | 4,150,933 | 0.103 |
| `p256-ecdsa-sign/ring` | 41,289,166 | 5,435,376 | 0.132 |
| `p256-ecdsa-sign/rind` | 6,234,613 | 5,196,490 | 0.833 |
| `p256-ecdsa-sign-digest/rind` | 6,109,776 | 5,188,770 | 0.849 |
| `p256-ecdsa-keygen/rind` | 3,846,599 | 3,307,694 | 0.860 |
| `p256-ecdsa-sign/rustcrypto` | 6,420,520 | 5,577,790 | 0.869 |
| `p256-keygen/rind` | 3,551,724 | 3,181,176 | 0.896 |
| `p256-ecdh-stored/rind` | 8,284,513 | 7,537,401 | 0.910 |
| `p256-ecdh/rind` | 11,841,011 | 10,836,626 | 0.915 |
| `p256-ecdsa-verify/rind` | 10,517,866 | 9,864,001 | 0.938 |
| `p256-keygen/p256-rustcrypto` | 4,510,402 | 4,245,576 | 0.941 |
| `p256-ecdsa-sign/libcrux` | 124,266,473 | 119,151,560 | 0.959 |
| `p256-ecdsa-verify/libcrux` | 328,223,167 | 315,038,950 | 0.960 |
| `p256-ecdh/libcrux` | 395,420,261 | 379,858,007 | 0.961 |
| `p256-keygen/libcrux` | 120,046,684 | 115,411,486 | 0.961 |
| `p256-ecdh/p256-rustcrypto` | 20,230,291 | 19,682,005 | 0.973 |
| `p256-ecdsa-verify-digest/rind` | 8,406,007 | 8,258,218 | 0.982 |
| `p256-ecdsa-verify-parsed/rind` | 8,626,270 | 8,499,067 | 0.985 |

