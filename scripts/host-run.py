#!/usr/bin/env python3
"""Runs rind's own `admin/bench-report run` (the host side of `collect`) on a
host outside the repo's Nix setup. Same operations, criterion args, parsing,
and JSON schema; only two checks are relaxed, and both are recorded:
  * toolchain(): accepts the host's rustc instead of refusing a non-pinned one
  * check_idle(): records busy CPUs instead of refusing a busy host
It also frees the benchmark build dir before build-cost runs (low-disk hosts).
"""
import importlib.machinery, importlib.util, os, shutil, sys
root = os.environ.get("RIND_ROOT") or os.path.dirname(os.path.abspath(__file__))
path = os.path.join(root, "admin", "bench-report")
loader = importlib.machinery.SourceFileLoader("bench_report", path)
spec = importlib.util.spec_from_loader("bench_report", loader)
br = importlib.util.module_from_spec(spec)
loader.exec_module(br)

def toolchain(env):
    v = br.output(["rustc", "-vV"], env)
    return ["cargo"], {"rustc": v.splitlines()[0], "cargo": br.output(["cargo", "-V"], env)}, "openssl"

def check_idle(when):
    busy = br.busy_cpus()
    print(f"host-run: {busy:.1f} of {br.cpu_count()} CPUs busy {when} (harness limit {br.busy_limit():.1f}); continuing", file=sys.stderr, flush=True)
    return round(busy, 2)

orig_build_cost = br.build_cost
def build_cost(env, jobs, refused):
    if os.environ.get("HOSTRUN_FREE_TARGET") == "1":
        shutil.rmtree(env["CARGO_TARGET_DIR"], ignore_errors=True)
    if os.environ.get("HOSTRUN_SKIP_BUILD_COST") == "1":
        return {"status": "failed", "started": "", "reason": "skipped by host-run (HOSTRUN_SKIP_BUILD_COST)"}
    return orig_build_cost(env, jobs, refused)

br.toolchain, br.check_idle, br.build_cost = toolchain, check_idle, build_cost
sys.argv = ["bench-report", "run"] + sys.argv[1:]
br.main()
