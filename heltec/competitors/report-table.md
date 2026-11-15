| Operation | rind | ring | rustcrypto | libcrux | Fastest (run 1 / run 2) |
|---|---|---|---|---|---|
| aes128gcm_seal_32B | 88.89 / 88.89 (n=1000) | 115.96 / 115.96 (n=1000) | 109.86 / 109.86 (n=1000) | 314.81 / 314.86 (n=1000) | rind / rind |
| aes128gcm_seal_1024B | 1293.74 / 1293.74 (n=200) | 1512.95 / 1512.95 (n=200) | 1758.81 / 1758.07 (n=200) | 4305.05 / 4304.86 (n=200) | rind / rind |
| aes128gcm_seal_1350B | 1713.41 / 1713.41 (n=200) | 2008.67 / 2008.67 (n=200) | 2661.18 / 2642.30 (n=200) | 5657.65 / 5657.65 (n=200) | rind / rind |
| chacha20poly1305_seal_32B | 53.76 / 53.76 (n=1000) | did not run | 39.19 / 39.19 (n=1000) | 48.90 / 48.90 (n=1000) | rustcrypto / rustcrypto |
| chacha20poly1305_seal_1024B | 455.31 / 455.31 (n=200) | did not run | 425.14 / 425.19 (n=200) | 456.87 / 456.87 (n=200) | rustcrypto / rustcrypto |
| chacha20poly1305_seal_1350B | 598.81 / 598.81 (n=200) | did not run | 563.35 / 563.33 (n=200) | 750.64 / 750.59 (n=200) | rustcrypto / rustcrypto |
| sha256_64B | 366.66 / 366.66 (n=1000) | 53.92 / 53.92 (n=1000) | 46.35 / 46.35 (n=1000) | 43.22 / 43.25 (n=1000) | libcrux / libcrux |
| sha256_1024B | 1025.71 / 1025.83 (n=200) | 421.86 / 421.87 (n=200) | 374.95 / 374.95 (n=200) | 353.21 / 353.06 (n=200) | libcrux / libcrux |
| sha256_1350B | 1234.55 / 1234.49 (n=200) | 544.71 / 544.72 (n=200) | 484.40 / 484.43 (n=200) | 456.37 / 456.68 (n=200) | libcrux / libcrux |
| sha512_64B | 6421.41 / 6421.40 (n=1000) | 102.64 / 102.64 (n=1000) | 6482.77 / 6482.77 (n=1000) | 6342.86 / 6342.87 (n=1000) | ring / ring |
| sha512_1024B | 55369.02 / 55369.06 (n=200) | 872.43 / 872.43 (n=200) | 56906.31 / 56906.25 (n=200) | 55239.53 / 55239.44 (n=200) | ring / ring |
| sha512_1350B | 67568.38 / 67568.44 (n=200) | 1064.92 / 1064.92 (n=200) | 69492.27 / 69492.24 (n=200) | 67461.10 / 67461.18 (n=200) | ring / ring |
| x25519_keygen_agree | 30222.95 / 30223.50 (n=20) | 694186.30 / 694166.05 (n=20) | 30105.65 / 30119.95 (n=20) | 2804038.70 / 2804066.40 (n=20) | rustcrypto / rustcrypto |
| p256_ecdh_keygen_agree | 62373.10 / 62360.65 (n=20) | 1368683.60 / 1368658.65 (n=20) | 87180.60 / 87189.75 (n=20) | 3108763.65 / 3108768.20 (n=20) | rind / rind |
| ed25519_sign | 24720.75 / 24719.20 (n=20) | 28320.05 / 28315.10 (n=20) | 32445.70 / 32446.90 (n=20) | 2072761.40 / 2072758.15 (n=20) | rind / rind |
| ed25519_verify | 31343.85 / 31342.45 (n=20) | 654330.20 / 654333.70 (n=20) | 31770.75 / 31772.50 (n=20) | 2624392.10 / 2624390.85 (n=20) | rind / rind |
| p256_ecdsa_sign | 40848.60 / 40853.60 (n=20) | 314725.55 / 314719.25 (n=20) | 32892.60 / 32932.60 (n=20) | 970898.20 / 970947.25 (n=20) | rustcrypto / rustcrypto |
| p256_ecdsa_verify | 52710.35 / 52710.40 (n=20) | 1366637.25 / 1366637.25 (n=20) | 76216.70 / 76215.60 (n=20) | 2578682.00 / 2578687.90 (n=20) | rind / rind |

checks not ok: []
cross-library deterministic outputs (run 1):
  aes128gcm_seal_32B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  aes128gcm_seal_1024B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  aes128gcm_seal_1350B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  chacha20poly1305_seal_32B: ALL MATCH across ['libcrux', 'rind', 'rustcrypto']
  chacha20poly1305_seal_1024B: ALL MATCH across ['libcrux', 'rind', 'rustcrypto']
  chacha20poly1305_seal_1350B: ALL MATCH across ['libcrux', 'rind', 'rustcrypto']
  sha256_64B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  sha256_1024B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  sha256_1350B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  sha512_64B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  sha512_1024B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  sha512_1350B: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
  ed25519_sign: ALL MATCH across ['libcrux', 'rind', 'ring', 'rustcrypto']
