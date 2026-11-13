#!/usr/bin/env python3
"""pmu-sum.py FILE [BASE]: per size and variant, rind's median-of-alternations wall time (and PMU per block),
the spread of the per-alternation medians, the ratio to BASE, and RustCrypto alongside."""
import sys, statistics as st, collections
f = sys.argv[1]; base = sys.argv[2] if len(sys.argv) > 2 else None
rows = collections.defaultdict(list)
for l in open(f):
    if l.startswith('#'): print(l.strip()); continue
    if 'DEBUG' in l: continue
    p = l.rstrip('\n').split('\t'); a, v, size, impl = p[:4]
    kv = dict(x.split('=') for x in p[4:])
    rows[(size, v, impl)].append({k: float(x) for k, x in kv.items()})
sizes = sorted({k[0] for k in rows}, key=lambda s: int(s[:-3])); vs = []
for k in rows:
    if k[1] not in vs: vs.append(k[1])
for s in sizes:
    print(f"\n## {s}  (n = alternations; spread = (max-min)/median of the per-alternation medians)")
    for impl in ('rind', 'rustcrypto'):
        for v in vs:
            r = rows.get((s, v, impl))
            if not r: continue
            w = [x['wall_ms'] for x in r]; m = st.median(w)
            line = f"{impl:10} {v:14} n={len(w)} med={m:8.3f} ms min={min(w):8.3f} spread={(max(w)-min(w))/m*100:5.1f}%"
            if base and (s, base, impl) in rows:
                line += f" ratio={m/st.median([x['wall_ms'] for x in rows[(s, base, impl)]]):.3f}"
            if 'cycles/blk' in r[0]:
                for k in ('cycles/blk', 'inst/blk', 'stall_be/blk', 'stall_fe/blk', 'l1d_refill/blk', 'ipc', 'ghz'):
                    line += f" {k.replace('/blk','')}={st.median([x[k] for x in r]):.2f}"
            print(line)
