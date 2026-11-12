#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Attributes the code and read-only data linked into rind-bench's images to
the libraries raced, by demangled symbol.

Usage: tools/sizes.py --nm xtensa-esp32s3-elf-nm IMAGES_DIR > results/sizes.json

Every function or read-only object in any image counts once, under the crate
its demangled path starts with (a trait impl, `<crate::Type as ...>`, counts
for the type's crate). Generic code from core/alloc instantiated for a
library's types counts for core, so each figure is a floor. Crates rind's
portable backend shares with competitors are their own group, `shared`.
"""

import argparse
import json
import os
import re
import subprocess

GROUPS = {
    "rind": ["rind"],
    "ring": ["ring"],
    "rustcrypto": [
        "aes_gcm", "aes", "ghash", "polyval", "universal_hash", "aead", "cipher", "ctr", "chacha20",
        "chacha20poly1305", "poly1305", "sha2", "sha1", "hmac", "hkdf", "digest", "block_buffer", "p256",
        "primeorder", "elliptic_curve", "crypto_bigint", "ecdsa", "rfc6979", "ff", "group", "ml_kem",
        "module_lattice", "kem", "keccak", "sha3", "ed25519_dalek", "x25519_dalek", "ed25519", "signature",
        "subtle", "crypto_common", "inout", "hybrid_array",
    ],
    "libcrux": [
        "libcrux_aesgcm", "libcrux_aes", "libcrux_chacha20poly1305", "libcrux_ecdh", "libcrux_ed25519",
        "libcrux_hkdf", "libcrux_hmac", "libcrux_ml_kem", "libcrux_p256", "libcrux_curve25519",
        "libcrux_sha3", "libcrux_secrets", "libcrux_traits", "libcrux_intrinsics", "libcrux_platform",
    ],
    "shared": ["curve25519_dalek", "fiat_crypto", "libcrux_hacl_rs", "libcrux_sha2", "libcrux_poly1305"],
}
CRATE_GROUP = {c: g for g, cs in GROUPS.items() for c in cs}


def crate_of(name):
    name = name.lstrip("<&*").removeprefix("mut ").removeprefix("dyn ")
    m = re.match(r"([A-Za-z_][A-Za-z0-9_]*)::", name)
    if m:
        return m.group(1)
    if name.startswith("ring_core_"):
        return "ring"
    if name.startswith(("esp_aes", "esp_gcm", "esp_sha", "mbedtls_")):
        return "esp32s3-hw"
    return None


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--nm", default="xtensa-esp32s3-elf-nm")
    ap.add_argument("images")
    args = ap.parse_args()
    seen = {}
    per_image = {}
    for elf in sorted(os.listdir(args.images)):
        if not elf.endswith(".elf"):
            continue
        out = subprocess.run(
            [args.nm, "-S", "-C", "--size-sort", os.path.join(args.images, elf)],
            capture_output=True, text=True, check=True,
        ).stdout
        image = {}
        for line in out.splitlines():
            parts = line.split(" ", 3)
            if len(parts) != 4 or parts[2] not in "tTrR":
                continue
            size, name = int(parts[1], 16), parts[3]
            crate = crate_of(name)
            group = "esp32s3-hw" if crate == "esp32s3-hw" else CRATE_GROUP.get(crate)
            if not group:
                continue
            image[group] = image.get(group, 0) + size
            # A symbol's hash suffix makes the same function in two images one key.
            seen[(group, crate, name)] = max(seen.get((group, crate, name), 0), size)
        per_image[elf.removesuffix(".elf")] = image
    libraries, crates = {}, {}
    for (group, crate, _), size in seen.items():
        libraries[group] = libraries.get(group, 0) + size
        crates[crate] = crates.get(crate, 0) + size
    json.dump(
        {
            "note": (
                "Bytes of code and read-only data, summed over symbols across every image, each counted once, "
                "by the crate its demangled name starts with. `shared` is curve25519-dalek, fiat-crypto and "
                "libcrux-hacl-rs/sha2/poly1305: rind's portable backend and the dalek or libcrux rows both link "
                "them. Generic core/alloc code instantiated for a library counts for core, so each is a floor. "
                "`esp32s3-hw` is ESP-IDF's mbedTLS AES/GCM/SHA wrappers around the accelerators."
            ),
            "libraries": dict(sorted(libraries.items())),
            "crates": dict(sorted(crates.items(), key=lambda x: -x[1])),
            "per_image": per_image,
        },
        __import__("sys").stdout,
        indent=1,
    )
    print()


if __name__ == "__main__":
    main()
