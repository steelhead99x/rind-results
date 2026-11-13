Cycles, icache32 over published (below 1 is faster in icache32).

## x25519

Known-answer checks: published 12/12, icache32 12/12. Runs: 3 and 3. Rows: 9 in both.

| Library | Inputs | Rows | Median | Range |
|---|---|---:|---:|---|
| dalek | per op | 2 | 0.914 | 0.891 to 0.937 |
| libcrux | per op | 2 | 0.061 | 0.061 to 0.061 |
| rind | per op | 3 | 0.937 | 0.893 to 0.968 |
| ring | per op | 2 | 0.532 | 0.105 to 0.959 |

9 of 9 rows moved by more than max(1%, run spread):

| Row | published cycles | icache32 cycles | B/A |
|---|---:|---:|---:|
| `x25519-ecdh/libcrux` | 350,672,703 | 21,270,588 | 0.061 |
| `x25519-keygen/libcrux` | 175,361,999 | 10,648,294 | 0.061 |
| `x25519-ecdh/ring` | 89,359,431 | 9,356,883 | 0.105 |
| `x25519-keygen/dalek` | 1,701,372 | 1,515,598 | 0.891 |
| `x25519-keygen/rind` | 1,697,554 | 1,515,923 | 0.893 |
| `x25519-ecdh/rind` | 6,790,030 | 6,358,903 | 0.937 |
| `x25519-ecdh/dalek` | 6,785,590 | 6,361,227 | 0.937 |
| `x25519-keygen/ring` | 4,295,869 | 4,118,326 | 0.959 |
| `x25519-ecdh-stored/rind` | 4,996,058 | 4,836,000 | 0.968 |

