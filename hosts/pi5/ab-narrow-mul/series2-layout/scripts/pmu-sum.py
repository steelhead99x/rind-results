import sys, collections, statistics as st
d = collections.defaultdict(lambda: collections.defaultdict(list))
for l in open(sys.argv[1]):
    p = l.rstrip("\n").split("\t")
    if len(p) < 3 or "=" not in p[2]: continue
    for kv in p[2:]:
        k, v = kv.split("="); d[(p[1], p[0])][k].append(float(v))
keys = ["ns", "cycles", "inst", "br_mis", "l1i_refill", "stall_fe"]
print("| op | variant | n | " + " | ".join(keys) + " | ns spread |"); print("|---" * (len(keys) + 4) + "|")
for (op, v) in sorted(d):
    m = d[(op, v)]; ns = m["ns"]
    print(f"| {op} | {v} | {len(ns)} | " + " | ".join(f"{st.median(m[k]):.1f}" for k in keys) + f" | {max(ns)/min(ns)-1:.2%} |")
