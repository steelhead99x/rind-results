#!/usr/bin/env python3
"""crit-parse.py ROOT: ROOT/<run-label>/... criterion homes. Prints, per id and
label prefix (text before the last '-rN'), every run's slope-or-mean ns, then
median, min, max and spread (max/min-1)."""
import json, os, re, sys, statistics as st, collections
root = sys.argv[1]; data = collections.defaultdict(list)
for label in sorted(os.listdir(root)):
    m = re.match(r"(.*)-r(\d+)$", label)
    if not m: continue
    mode = m.group(1)
    for dp, dn, fn in os.walk(os.path.join(root, label)):
        if dp.endswith("/new") and "benchmark.json" in fn:
            b = json.load(open(os.path.join(dp, "benchmark.json")))
            e = json.load(open(os.path.join(dp, "estimates.json")))
            est = e.get("slope") or e["mean"]
            data[(b["full_id"], mode)].append(est["point_estimate"])
def fmt(ns):
    return f"{ns/1e6:.3f} ms" if ns >= 1e6 else f"{ns/1e3:.2f} us" if ns >= 1e3 else f"{ns:.0f} ns"
print("| id | mode | runs | median | min | max | spread |\n|---|---|---|---|---|---|---|")
for (fid, mode), v in sorted(data.items()):
    print(f"| {fid} | {mode} | {len(v)} | {fmt(st.median(v))} | {fmt(min(v))} | {fmt(max(v))} | {100*(max(v)/min(v)-1):.1f}% |")
