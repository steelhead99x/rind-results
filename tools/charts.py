#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Draws one SVG per host from schema-1 results: for each report operation,
rind's time over the fastest other Rust library's, on a log scale (left of
1x: rind is faster). Hardware baselines are never counted.

Usage: tools/charts.py OUT_DIR HOST_JSON...
"""
import json, math, os, sys
from html import escape

NOT_RIVALS = {"rind", "esp32s3-hw"}


def ratios(host):
    by = {}
    for r in host["results"]:
        by.setdefault(r["operation"], {})[r["library"]] = r["ns"]
    out = []
    for op, t in by.items():
        if "rind" not in t:
            continue
        rivals = {l: ns for l, ns in t.items() if l not in NOT_RIVALS and not l.startswith("rind-")}
        if not rivals:
            continue
        lib = min(rivals, key=rivals.get)
        out.append((op.replace("/{lib}", "").replace("{lib}/", ""), t["rind"] / rivals[lib], lib))
    return out


def svg(host, rows):
    lo, hi = 1 / 16, 64
    w, left, right, bar, top = 900, 300, 150, 16, 70
    h = top + len(rows) * (bar + 4) + 40
    span = math.log(hi / lo)
    x = lambda v: left + (math.log(min(max(v, lo), hi) / lo) / span) * (w - left - right)
    s = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" font-family="sans-serif" font-size="12">',
         f'<rect width="{w}" height="{h}" fill="#fff"/>',
         f'<text x="10" y="22" font-size="16" font-weight="bold">{escape(host["host"])}: rind\'s time over the fastest other Rust library</text>',
         f'<text x="10" y="42" fill="#555">{escape(host["cpu"]["model"])}; log scale, left of 1x rind is faster{"; noisy run" if host.get("load", {}).get("noisy") else ""}</text>']
    for v in (1 / 8, 1 / 4, 1 / 2, 1, 2, 4, 8, 16, 32, 64):
        s.append(f'<line x1="{x(v):.1f}" y1="{top - 8}" x2="{x(v):.1f}" y2="{h - 30}" stroke="{"#000" if v == 1 else "#ddd"}"/>')
        s.append(f'<text x="{x(v):.1f}" y="{h - 14}" text-anchor="middle" fill="#555">{v:g}x</text>')
    for i, (op, r, lib) in enumerate(rows):
        y = top + i * (bar + 4)
        a, b = sorted((x(1), x(r)))
        s.append(f'<text x="{left - 6}" y="{y + 12}" text-anchor="end">{escape(op)}</text>')
        s.append(f'<rect x="{a:.1f}" y="{y}" width="{max(b - a, 1):.1f}" height="{bar}" fill="{"#2a7" if r <= 1 else "#d54"}"/>')
        s.append(f'<text x="{w - right + 6}" y="{y + 12}">{r:.2f}x {escape(lib)}</text>')
    s.append("</svg>")
    return "\n".join(s)


out = sys.argv[1]
os.makedirs(out, exist_ok=True)
for path in sys.argv[2:]:
    host = json.load(open(path))
    with open(os.path.join(out, f'{host["host"]}-rind-vs-fastest.svg'), "w") as f:
        f.write(svg(host, ratios(host)))
