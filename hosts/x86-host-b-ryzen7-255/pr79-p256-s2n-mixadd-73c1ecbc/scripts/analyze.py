#!/usr/bin/env python3
"""analyze.py RAWDIR OUT.md OUT.json [--side-a base --side-b branch] -- A/B table from run_ab.py raw files."""
import glob, json, os, random, statistics as st, sys, re
raw, outmd, outjson = sys.argv[1:4]
A = "base"; B = "branch"
rounds = {}
for p in sorted(glob.glob(raw + "/*.json")):
    d = json.load(open(p))
    for rid, v in d["results"].items():
        rounds.setdefault(rid, {}).setdefault(d["side"], {})[d["round"]] = v["ns"]
random.seed(1)
def fmt(ns):
    return f"{ns/1000:.2f} µs" if ns >= 1000 else f"{ns:.1f} ns"
def boot(ratios, n=4000):
    ms = sorted(st.median(random.choices(ratios, k=len(ratios))) for _ in range(n))
    return ms[int(.025*n)], ms[int(.975*n)]
rows = []
for rid in sorted(rounds):
    s = rounds[rid]
    if A not in s or B not in s: continue
    common = sorted(set(s[A]) & set(s[B]))
    a = [s[A][r] for r in common]; b = [s[B][r] for r in common]
    ratios = [y/x for x, y in zip(a, b)]
    ma, mb = st.median(a), st.median(b)
    lo, hi = boot(ratios)
    def spread(x): return (max(x)-min(x))/st.median(x)*100
    rows.append(dict(id=rid, n=len(common), base_ns=ma, branch_ns=mb, delta_pct=(mb/ma-1)*100,
        paired_median_pct=(st.median(ratios)-1)*100, ci_lo_pct=(lo-1)*100, ci_hi_pct=(hi-1)*100,
        faster_rounds=sum(1 for r in ratios if r < 1), base_spread_pct=spread(a), branch_spread_pct=spread(b),
        base_runs=a, branch_runs=b))
json.dump(rows, open(outjson, "w"), indent=1)
def verdict(r):
    if r["ci_hi_pct"] < 0: return "faster"
    if r["ci_lo_pct"] > 0: return "slower"
    return "no difference beyond noise"
with open(outmd, "w") as f:
    f.write("| Benchmark | n | base (median) | branch (median) | Δ medians | paired Δ, 95% CI | branch faster in | spread base / branch | read |\n|---|---:|---:|---:|---:|---|---:|---|---|\n")
    for r in rows:
        f.write(f"| `{r['id']}` | {r['n']} | {fmt(r['base_ns'])} | {fmt(r['branch_ns'])} | {r['delta_pct']:+.1f}% | {r['paired_median_pct']:+.1f}% [{r['ci_lo_pct']:+.1f}, {r['ci_hi_pct']:+.1f}] | {r['faster_rounds']}/{r['n']} | {r['base_spread_pct']:.1f}% / {r['branch_spread_pct']:.1f}% | {verdict(r)} |\n")
print(open(outmd).read())
