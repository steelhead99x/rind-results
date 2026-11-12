#!/usr/bin/env python3
"""Turns the phone's criterion output into a host JSON in admin/bench-report's schema,
parsing exactly as run() does (benchmark.json + estimates.json, slope else mean)."""
import importlib.machinery, importlib.util, json, os, glob, sys, re
RIND="<rind>"; D=sys.argv[1]; OUT=sys.argv[2]; D2=sys.argv[3] if len(sys.argv)>3 else None
l=importlib.machinery.SourceFileLoader("br",f"{RIND}/admin/bench-report"); s=importlib.util.spec_from_loader("br",l); br=importlib.util.module_from_spec(s); l.exec_module(br)
rd=lambda f: open(os.path.join(D,f)).read().strip()
props={}
for line in open(os.path.join(D,"getprop.txt"),errors="replace"):
    m=re.match(r"\[(.+?)\]: \[(.*)\]",line)
    if m: props[m.group(1)]=m.group(2)
def stat(f):
    v=[int(x) for x in rd(f).split()[1:9]]; idle=v[3]+v[4]; return sum(v)-idle, sum(v)
def stat_in(d,f):
    v=[int(x) for x in open(os.path.join(d,f)).read().split()[1:9]]; idle=v[3]+v[4]; return sum(v)-idle, sum(v)
def bat2(f):
    return {k.strip():v.strip() for k,v in (x.split(":",1) for x in open(os.path.join(D2,f)).read().splitlines() if ":" in x)}
b0,t0=stat("stat-before"); b1,t1=stat("stat-after")
busy_share=(b1-b0)/max(1,t1-t0)
results=[]
paths=sorted(glob.glob(os.path.join(D,"criterion","**","new","benchmark.json"),recursive=True))+(sorted(glob.glob(os.path.join(D2,"criterion","**","new","benchmark.json"),recursive=True)) if D2 else [])
for path in paths:
    meta=json.load(open(path)); found=br.operation_of(meta["full_id"])
    if not found: continue
    op,lib=found; est=json.load(open(os.path.join(os.path.dirname(path),"estimates.json"))); e=est.get("slope") or est["mean"]
    results.append({"operation":op,"library":br.LIBRARIES.get(lib,lib),"id":meta["full_id"],"ns":e["point_estimate"],
                    "ns_low":e["confidence_interval"]["lower_bound"],"ns_high":e["confidence_interval"]["upper_bound"],"throughput":meta.get("throughput")})
order={op:i for i,op in enumerate(br.OPERATIONS)}; results.sort(key=lambda r:(order[r["operation"]],r["library"]))
locked=br.locked_versions()
libs={lib:{c:", ".join(locked.get(c,["?"])) for c in cr} for lib,cr in br.CRATES.items()}
libs["openssl"]["linked"]="vendored openssl-src "+", ".join(locked.get("openssl-src",["?"]))+" (static, built by openssl-sys's vendored feature)"
bat=lambda f: {k.strip():v.strip() for k,v in (x.split(":",1) for x in rd(f).splitlines() if ":" in x)}
bb,ba=bat("battery-before.txt"),bat("battery-after.txt")
h={"schema":1,"host":"pixel9pro","commit":"b66514238d449fc211d75fd14554301bb690e421","started":rd("started"),
   "cpu":{"model":f"Google Tensor G4 ({props.get('ro.product.model','Pixel 9 Pro')})","arch":"aarch64","logical_cpus":8},
   "os":f"Android {props.get('ro.build.version.release','?')} (API {props.get('ro.build.version.sdk','?')}, build {props.get('ro.build.id','?')})",
   "toolchain":{"rustc":"rustc 1.98.1 (48a229cea 2026-09-01)","cargo":"cargo 1.98.1 (797e8a9bc 2026-08-05)","target":"aarch64-linux-android (API 31), Android NDK r28c, cross-compiled on x86_64"},
   "libraries":libs,"rind_cpu":json.loads(rd("cpuid.json")),"refused":{},"status":"ok","finished":rd("finished"),
   "criterion_args":br.CRITERION_ARGS+[br.operation_regex(())],
   "load":{"limit_cpus":1.0,"system_busy_share_during_run":round(busy_share,3),"note":"Android: whole-system busy share across the run, benchmarks included; 1 of 8 CPUs is 0.125",
           "noisy":None},
   "battery":{"before":bb,"after":ba},"results":results,
   "build_cost":{"status":"failed","started":"","reason":"not measured: the binaries were cross-compiled on another host"}}
temp=lambda d: int(d.get("temperature","0"))/10
h["battery"]["temperature_c"]={"before":temp(bb),"after":temp(ba)}
if D2:
    r2=lambda f: open(os.path.join(D2,f)).read().strip()
    c0,c1=stat_in(D2,"stat-before"),stat_in(D2,"stat-after")
    b2,a2=bat2("battery-before.txt"),bat2("battery-after.txt")
    h["second_pass"]={"why":"the first pass's rustls bench panicked: it reads rind-rustls/tests/keys from the build machine's CARGO_MANIFEST_DIR path at runtime; rebuilt from a source copy at /data/local/tmp/rind/src and rerun alone with the same criterion args, with the keys pushed to that path",
      "started":r2("started"),"finished":r2("finished"),"system_busy_share_during_run":round((c1[0]-c0[0])/max(1,c1[1]-c0[1]),3),
      "battery":{"before":b2,"after":a2},"temperature_c":{"before":temp(b2),"after":temp(a2)}}
    h["finished"]=r2("finished")
json.dump(h,open(OUT,"w"),indent=1); print(len(results),"results;",busy_share,h["battery"]["temperature_c"])
