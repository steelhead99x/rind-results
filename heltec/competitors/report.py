import re, glob, collections, sys
libs = ["rind", "ring", "rustcrypto", "libcrux"]
ops = ["aes128gcm_seal_32B","aes128gcm_seal_1024B","aes128gcm_seal_1350B","chacha20poly1305_seal_32B","chacha20poly1305_seal_1024B","chacha20poly1305_seal_1350B","sha256_64B","sha256_1024B","sha256_1350B","sha512_64B","sha512_1024B","sha512_1350B","x25519_keygen_agree","p256_ecdh_keygen_agree","ed25519_sign","ed25519_verify","p256_ecdsa_sign","p256_ecdsa_verify"]
bench = collections.defaultdict(dict); iters = {}; checks = collections.defaultdict(dict)
for run in (1, 2):
    for lib in libs:
        try: txt = open(f"logs/serial-{lib}-run{run}.log", errors="replace").read()
        except FileNotFoundError: continue
        for m in re.finditer(r"^BENCH,(\w+),(\w+),iters=(\d+),total_us=(\d+),per_op_us=([\d.]+)", txt, re.M):
            l, op, it, tot, per = m.groups(); bench[(l, op)][run] = per; iters[(l, op)] = it
        for m in re.finditer(r"^CHECK,(\w+),(\w+),ok=(\w+),out=(.*)$", txt, re.M):
            l, op, ok, out = m.groups(); checks[(l, op)][run] = (ok, out.strip())
print("| Operation | " + " | ".join(libs) + " | Fastest (run 1 / run 2) |")
print("|---|" + "---|" * (len(libs) + 1))
for op in ops:
    row = []; best = {}
    for run in (1, 2):
        c = [(float(bench[(l, op)][run]), l) for l in libs if run in bench.get((l, op), {})]
        best[run] = min(c)[1] if c else "-"
    for l in libs:
        b = bench.get((l, op))
        row.append(f"{b.get(1,'-')} / {b.get(2,'-')} (n={iters[(l,op)]})" if b else "did not run")
    print(f"| {op} | " + " | ".join(row) + f" | {best[1]} / {best[2]} |")
print()
bad = [(k, v) for k, v in checks.items() for r, (ok, o) in v.items() if ok != "true"]
print("checks not ok:", bad)
print("cross-library deterministic outputs (run 1):")
for op in ops:
    outs = {l: checks[(l, op)][1][1] for l in libs if 1 in checks.get((l, op), {})}
    vals = set(o for o in outs.values() if o not in ("symmetric", "own_verify", "vector_ok+tamper_rejected"))
    if vals: print(f"  {op}: {'ALL MATCH' if len(vals)==1 else 'MISMATCH'} across {sorted(outs)}")
