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


def heading(op):
    h = op.replace("/{lib}", "").replace("{lib}/", "")
    return h.replace("/", " ").upper()[:21]


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
    print("# rind-bench OLED demo pages, from " + args.host)
    for _, ops in br.SECTIONS:
        for op in ops:
            libs = by.get(op, {})
            if "rind" not in libs or len(libs) < 2:
                continue
            size = re.search(r"/(\d+[KM]?B)$", op)
            if size and not args.all and size.group(1) not in ("32B", "1350B"):
                continue
            for lib in sorted(libs, key=lambda x: ORDER.index(x) if x in ORDER else 99):
                print(f"{heading(op)}\t{lib}\t{libs[lib]:.1f}")


if __name__ == "__main__":
    main()
