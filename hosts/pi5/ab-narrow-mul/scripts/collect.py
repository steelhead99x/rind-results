#!/usr/bin/env python3
"""collect.py ROOT RUNLOG META.json: criterion homes ROOT/<mode>-r<N> -> one JSON (host-json style rows plus
mode and round), with per-round load lines from RUNLOG. META supplies host/commit/mode descriptions."""
import json, os, re, sys
root, runlog, meta = sys.argv[1], sys.argv[2], json.load(open(sys.argv[3]))
rows = []
for label in sorted(os.listdir(root)):
    m = re.match(r"(.*)-r(\d+)$", label)
    if not m: continue
    for dp, dn, fn in os.walk(os.path.join(root, label)):
        if dp.endswith("/new") and "benchmark.json" in fn:
            b = json.load(open(os.path.join(dp, "benchmark.json")))
            e = json.load(open(os.path.join(dp, "estimates.json")))
            est = e.get("slope") or e["mean"]
            fid = b["full_id"]; op, lib = fid.rsplit("/", 1)
            rows.append({"operation": op + "/{lib}", "library": lib, "id": fid, "mode": m.group(1), "round": int(m.group(2)),
                         "ns": est["point_estimate"], "ns_low": est["confidence_interval"]["lower_bound"],
                         "ns_high": est["confidence_interval"]["upper_bound"]})
rows.sort(key=lambda r: (r["id"], r["mode"], r["round"]))
meta.update({"schema": "rind-results/ab-v1", "criterion_args": ["--warm-up-time", "1", "--measurement-time", "3"],
             "log": open(runlog).read().splitlines(), "results": rows})
json.dump(meta, sys.stdout, indent=1)
