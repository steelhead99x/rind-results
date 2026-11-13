#!/usr/bin/env python3
"""cmp.py ROOT BASE MODE...: per benchmark id (rind rows), median ns over rounds per mode, spread
(max/min-1), ratio MODE/BASE, and ratio to aws-lc-rs median (all modes pooled for aws-lc-rs)."""
import json, os, re, sys, statistics as st, collections
root, base, modes = sys.argv[1], sys.argv[2], sys.argv[3:]
d = collections.defaultdict(list)
for label in os.listdir(root):
    m = re.match(r"(.*)-r(\d+)$", label)
    if not m: continue
    for dp, dn, fn in os.walk(os.path.join(root, label)):
        if dp.endswith("/new") and "benchmark.json" in fn:
            b = json.load(open(os.path.join(dp, "benchmark.json")))
            e = json.load(open(os.path.join(dp, "estimates.json")))
            d[(b["full_id"], m.group(1))].append((e.get("slope") or e["mean"])["point_estimate"])
def f(ns): return f"{ns/1e3:.1f}"
ops = sorted({k[0].rsplit("/",1)[0] for k in d})
print("| op | " + " | ".join(f"{m} µs (n, spread)" for m in [base]+modes) + " | " + " | ".join(f"{m}/{base}" for m in modes) + " | aws-lc-rs µs | " + " | ".join(f"{m}/aws" for m in [base]+modes) + " |")
print("|---" * (2 + 2*len(modes) + 2 + len(modes)) + "|")
for op in ops:
    aws = [x for k, v in d.items() if k[0] == op + "/aws-lc-rs" for x in v]
    meds = {}; cells = []
    for m in [base]+modes:
        v = d.get((op + "/rind", m))
        if v: meds[m] = st.median(v); cells.append(f"{f(meds[m])} ({len(v)}, {100*(max(v)/min(v)-1):.1f}%)")
        else: cells.append("–")
    rat = [f"{meds[m]/meds[base]:.3f}" if m in meds and base in meds else "–" for m in modes]
    a = st.median(aws) if aws else None
    ra = [f"{meds[m]/a:.2f}" if a and m in meds else "–" for m in [base]+modes]
    print(f"| {op} | " + " | ".join(cells) + " | " + " | ".join(rat) + f" | {f(a) if a else '–'} | " + " | ".join(ra) + " |")
