#!/usr/bin/env python3
"""Alternating A/B criterion runs. usage: run_ab.py SET ROUNDS CPUS BENCHSPEC...
BENCHSPEC = side=dir:target ... given via --side; --bench name:regex[:cargo-dir-kind]"""
import argparse, json, os, subprocess, sys, time, glob, shutil
ap = argparse.ArgumentParser()
ap.add_argument("set"); ap.add_argument("--rounds", type=int, default=8)
ap.add_argument("--cpus", default="2,10")
ap.add_argument("--side", action="append", required=True)   # name=workdir:target
ap.add_argument("--bench", action="append", required=True)  # bench:regex
ap.add_argument("--mt", default="3"); ap.add_argument("--wu", default="1")
a = ap.parse_args()
home = os.path.expanduser("~/rind-bot")
out = f"{home}/out/{a.set}"; os.makedirs(out + "/raw", exist_ok=True)
sides = {}
for s in a.side:
    n, rest = s.split("=", 1); d, t = rest.split(":"); sides[n] = (f"{home}/{d}", f"{home}/{t}")
names = list(sides)
env = dict(os.environ, PATH=os.path.expanduser("~/.cargo/bin") + ":" + os.environ["PATH"])
log = open(out + "/run.log", "a")
def L(m):
    m = time.strftime("%H:%M:%S ") + m; print(m, flush=True); log.write(m + "\n"); log.flush()
def harvest(target, tag):
    res = {}
    for p in glob.glob(f"{target}/criterion/**/new/benchmark.json", recursive=True):
        meta = json.load(open(p)); est = json.load(open(os.path.join(os.path.dirname(p), "estimates.json")))
        e = est.get("slope") or est["mean"]
        res[meta["full_id"]] = {"ns": e["point_estimate"], "lo": e["confidence_interval"]["lower_bound"],
            "hi": e["confidence_interval"]["upper_bound"], "mean": est["mean"]["point_estimate"],
            "median": est["median"]["point_estimate"], "estimator": "slope" if est.get("slope") else "mean"}
    return res
for r in range(1, a.rounds + 1):
    order = names if r % 2 else names[::-1]
    for side in order:
        wd, target = sides[side]
        for spec in a.bench:
            bench, rx = spec.split(":", 1)
            shutil.rmtree(target + "/criterion", ignore_errors=True)
            la = open("/proc/loadavg").read().split()[0]
            L(f"round {r} side {side} bench {bench} load1={la}")
            cmd = ["taskset", "-c", a.cpus, "cargo", "bench", "--locked", "-q", "-p", "rind-bench", "--bench", bench, "--",
                   "--noplot", "--warm-up-time", a.wu, "--measurement-time", a.mt, rx]
            if bench == "xover": cmd = [c for c in cmd if c not in ("--locked", "-p", "rind-bench")]
            e2 = dict(env, CARGO_TARGET_DIR=target)
            p = subprocess.run(cmd, cwd=wd, env=e2, capture_output=True, text=True)
            if p.returncode: L("FAILED rc=%d: %s" % (p.returncode, p.stderr[-500:]))
            res = harvest(target, side)
            la2 = open("/proc/loadavg").read().split()[0]
            json.dump({"round": r, "side": side, "bench": bench, "load1_before": la, "load1_after": la2, "results": res},
                      open(f"{out}/raw/{side}-r{r:02d}-{bench}.json", "w"), indent=1, sort_keys=True)
            L(f"  {len(res)} rows")
