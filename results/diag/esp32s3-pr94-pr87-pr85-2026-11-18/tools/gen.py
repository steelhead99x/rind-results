# Committed layout: <variant>-<sha>/<bench>/run-N.log (the run scripts wrote board/<variant>/<bench>/).
DIRS={'main': 'main-b9958b33', 'pr94-iram': 'pr94-iram-e874533a', 'pr94-flash': 'pr94-flash-e874533a', 'pr94h-iram': 'pr94h-iram-3654a771', 'pr94h-flash': 'pr94h-flash-3654a771', 'pr85': 'pr85-0fa33a92', 'pr87': 'pr87-68277ff7'}
import json,glob,re,statistics as st,collections
def load(bench,v):
    d=collections.defaultdict(list); n=0
    for f in sorted(glob.glob(f'{DIRS[v]}/{bench}/run-*.log')):
        txt=open(f).read()
        if '"type":"done"' not in txt.replace(' ',''): continue
        n+=1
        for line in txt.splitlines():
            m=re.search(r'RB:(\{.*)',line)
            if not m: continue
            try: j=json.loads(m.group(1))
            except Exception: continue
            if j.get('type')=='result' and 'cycles' in j: d[j['id']].append(j['cycles']['median'])
            if j.get('type')=='pb': d[j['id']].append(j['median'])
    return {k:st.median(x) for k,x in d.items()},n
def f(x): return f"{x:,.0f}"
def r(x): return f"{x:.3f}"
M,_=load('sha2','main'); I,_=load('sha2','pr94-iram'); F,_=load('sha2','pr94-flash')
H,_=load('hmac','main'); HI,_=load('hmac','pr94-iram'); HF,_=load('hmac','pr94-flash')
out=["| Row (rind) | main cycles | IRAM (this PR) | IRAM / main | `esp32s3-sha512-flash` | flash / main |","|---|---:|---:|---:|---:|---:|"]
for lab,key,a,b,c in [(f"SHA-512 {s}",f"sha512/rind/{s}",M,I,F) for s in ['32B','64B','1200B','1350B','8KB','16KB']]+[("SHA-512/256 16KB","sha512-256/rind/16KB",M,I,F)]+[(f"HMAC-SHA384 {s}",f"hmac-sha384/rind/{s}",H,HI,HF) for s in ['64B','1200B','16KB']]+[("HMAC-SHA384 reused key 16KB","hmac-sha384/rind/reused-key/16KB",H,HI,HF)]:
    out.append(f"| {lab} | {f(a[key])} | {f(b[key])} | {r(b[key]/a[key])} | {f(c[key])} | {r(c[key]/a[key])} |")
print("\n".join(out))
print()
P,_=load('poly1305','pr85'); Q,_=load('poly1305','pr87')
o=["| Row | #85 base (HACL*) cycles | this PR cycles | ratio |","|---|---:|---:|---:|"]
for lab,key in [("Poly1305 64 B","poly1305/64B"),("Poly1305 1200 B","poly1305/1200B"),("Poly1305 16 KB","poly1305/16384B"),("ChaCha20-Poly1305 seal 32 B","chacha20poly1305-seal/32B"),("ChaCha20-Poly1305 seal 64 B","chacha20poly1305-seal/64B"),("ChaCha20-Poly1305 seal 1200 B","chacha20poly1305-seal/1200B"),("ChaCha20-Poly1305 seal 16 KB","chacha20poly1305-seal/16384B")]:
    o.append(f"| {lab} | {f(P[key])} | {f(Q[key])} | {r(Q[key]/P[key])} |")
print("\n".join(o))
