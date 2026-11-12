#!/usr/bin/env python3
"""minc.py ROOT BASE MODE: min-of-rounds per rind benchmark id, ratio MODE/BASE (for noisy hosts)."""
import json, os, re, sys, collections
root, base, mode = sys.argv[1:4]
d = collections.defaultdict(list)
for label in os.listdir(root):
    m = re.match(r"(.*)-r(\d+)$", label)
    if not m: continue
    for dp, dn, fn in os.walk(os.path.join(root, label)):
        if dp.endswith("/new") and "benchmark.json" in fn:
            b = json.load(open(os.path.join(dp, "benchmark.json"))); e = json.load(open(os.path.join(dp, "estimates.json")))
            d[(b["full_id"], m.group(1))].append((e.get("slope") or e["mean"])["point_estimate"])
print(f"| op | {base} min µs (n) | {mode} min µs (n) | {mode}/{base} (min) |\n|---|---|---|---|")
for fid in sorted({k[0] for k in d if k[0].endswith("/rind")}):
    a, b = d[(fid, base)], d[(fid, mode)]
    print(f"| {fid.rsplit('/',1)[0]} | {min(a)/1e3:.1f} ({len(a)}) | {min(b)/1e3:.1f} ({len(b)}) | {min(b)/min(a):.3f} |")
