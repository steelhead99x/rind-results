import glob, collections, statistics as st
d = collections.defaultdict(lambda: collections.defaultdict(list))
for f in glob.glob("stat-*.csv"):
    p = f[:-4].split("-"); v = p[1]; op = "-".join(p[2:-1])
    for l in open(f):
        q = l.strip().split(",")
        if len(q) > 3 and q[0] and q[2]: d[(op, v)][q[2]].append(float(q[0]))
print("| op | binary | runs | IPC | backend stall cycles / kinst | L1D misses / kinst | L1I misses / kinst | branch misses / kinst |")
print("|---|---|---|---|---|---|---|---|")
for (op, v), m in sorted(d.items()):
    i = st.median(m["instructions:u"]); c = st.median(m["cycles:u"])
    g = lambda k: st.median(m[k]) / i * 1000
    n = len(m["cycles:u"])
    print(f"| {op} | {v} | {n} | {i/c:.3f} | {g('stalled-cycles-backend:u'):.1f} | {g('L1-dcache-load-misses'):.3f} | {g('L1-icache-load-misses'):.4f} | {g('branch-misses:u'):.4f} |")
