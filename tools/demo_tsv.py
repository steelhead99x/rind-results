#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Writes the OLED demo's results table from report.py's esp32s3.json: one
page per report operation rind and another library ran, in the report's
section order, as `heading<TAB>library<TAB>ns` lines.

Usage: tools/demo_tsv.py --rind ../rind-bench/rind results/esp32s3.json > demo.tsv
  --all   every operation; by default byte-size rows only at 32B and 1350B
"""

import argparse
import json
import re

from report import load_bench_report

ORDER = ["rind", "ring", "rustcrypto", "libcrux", "esp32s3-hw"]


# The title bar fits 25 columns of 5x8 text, less the page counter
# ("27/27"), so long names are shortened rather than cut: a cut once merged
# the three QUIC header-protection pages (one heading) and showed 1350B as
# "135".
SHORT = [
    ("quic-header-protection", "quic-hp"),
    ("xchacha20poly1305", "xchachapoly"),
    ("chacha20poly1305", "chachapoly"),
    ("aes128-gcm", "aes128gcm"),
    ("aes256-gcm", "aes256gcm"),
    ("-open", " open"),
    ("extract-expand", "ext+exp"),
    ("rustls-full-handshake-group", "tls-full"),
    ("rustls-full-handshake-key", "tls-full"),
    ("rustls-resumed-handshake", "tls-resumed"),
    ("rustls-bulk", "tls-bulk"),
    ("pkcs1-sha256", "pkcs1"),
    ("pss-sha256", "pss"),
]


def heading(op):
    h = op.replace("/{lib}", "").replace("{lib}/", "")
    for a, b in SHORT:
        h = h.replace(a, b)
    return h.replace("/", " ").upper()


def fits(h, page):
    """Title text from x=2 must end before the right-aligned counter at x=126."""
    return 2 + 5 * len(h) < 126 - 5 * len(page) - 2


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--rind", required=True)
    ap.add_argument("--all", action="store_true")
    ap.add_argument("host")
    args = ap.parse_args()
    br = load_bench_report(args.rind)
    with open(args.host) as f:
        host = json.load(f)
    by = {}
    for r in host["results"]:
        by.setdefault(r["operation"], {})[r["library"]] = r["ns"]
    lines = []
    for _, ops in br.SECTIONS:
        for op in ops:
            libs = by.get(op, {})
            if "rind" not in libs or len(libs) < 2:
                continue
            size = re.search(r"/(\d+[KM]?B)$", op)
            if size and not args.all and size.group(1) not in ("32B", "1350B"):
                continue
            lines.append((heading(op), [(lib, libs[lib]) for lib in sorted(libs, key=lambda x: ORDER.index(x) if x in ORDER else 99)]))
    heads = [h for h, _ in lines]
    dup = {h for h in heads if heads.count(h) > 1}
    if dup:
        raise SystemExit(f"duplicate OLED headings {sorted(dup)}: extend SHORT")
    for n, h in enumerate(heads, 1):
        if not fits(h, f"{n}/{len(heads)}"):
            raise SystemExit(f"OLED heading {h!r} overlaps its page counter: extend SHORT")
    print("# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0")
    print("# rind-bench OLED demo pages, from " + args.host)
    for h, bars in lines:
        for lib, ns in bars:
            print(f"{h}\t{lib}\t{ns:.1f}")


if __name__ == "__main__":
    main()
