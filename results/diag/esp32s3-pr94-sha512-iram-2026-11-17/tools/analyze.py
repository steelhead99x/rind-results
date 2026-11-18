#!/usr/bin/env python3
import json, glob, os, re, statistics as st, sys
root = sys.argv[1]
labels = ["main-4fbb7188", "pr94-942bb48a"]
data = {l: {} for l in labels}; fails = {l: 0 for l in labels}; boots = {l: 0 for l in labels}
for l in labels:
    for f in sorted(glob.glob(f"{root}/{l}/*/run-*.log")):
        bench = f.split("/")[-2]; ok = False
        boots[l] += 1
        for line in open(f, errors="replace"):
            m = re.search(r'RB:(\{.*\})', line)
            if not m: continue
            try: r = json.loads(m.group(1))
            except Exception: continue
            if r.get("type") == "checks-done": fails[l] += r["failed"]
            if r.get("type") == "check" and not r.get("pass"): fails[l] += 0
            if r.get("type") == "result":
                data[l].setdefault((bench, r["id"]), []).append(r["ns"])
print("boots", boots, "known-answer failures", fails)
rows = []
for k in sorted(data[labels[0]]):
    if k not in data[labels[1]]: continue
    a, b = data[labels[0]][k], data[labels[1]][k]
    if len(a) < 3 or len(b) < 3: continue
    ma, mb = st.median(a), st.median(b)
    sp = max((max(x) - min(x)) / st.median(x) * 100 for x in (a, b))
    rows.append((k[0], k[1], ma, mb, mb / ma, sp))
json.dump(rows, open(f"{root}/summary.json", "w"))
with open(f"{root}/ratios.tsv", "w") as o:
    o.write("image\tid\tmain_ns_median_of_3\tpr_ns_median_of_3\tpr_over_main\tmax_run_spread_pct\n")
    for r in rows:
        o.write("%s\t%s\t%.3f\t%.3f\t%.4f\t%.4f\n" % r)
sel = [r for r in rows if "rind" in r[1] and ("sha512" in r[1] or "sha384" in r[1] or "hkdf-sha384" in r[1] or "hkdf-sha512" in r[1])]
for r in sel: print("%-6s %-50s %12.0f %12.0f %.3fx spread %.3f%%" % (r[0], r[1], r[2], r[3], r[4], r[5]))
