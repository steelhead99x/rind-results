#!/usr/bin/env python3
"""Print expected SHA digests for the Uno R3 pattern benches."""
import hashlib
def pat(n):
    return bytes(i & 0xFF for i in range(n))
for name, n, h in [
    ("sha256/32B", 32, hashlib.sha256),
    ("sha256/1350B", 1350, hashlib.sha256),
    ("sha256/16KB", 16*1024, hashlib.sha256),
    ("sha512/32B", 32, hashlib.sha512),
    ("sha512/1350B", 1350, hashlib.sha512),
    ("sha512/16KB", 16*1024, hashlib.sha512),
]:
    print(f"{name} {h(pat(n)).hexdigest()}")
