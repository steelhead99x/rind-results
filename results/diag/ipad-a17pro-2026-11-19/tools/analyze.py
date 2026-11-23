import sys, statistics as st, collections
OPS=["ed25519-verify/rind","ed25519-verify/aws-lc-rs","ed25519-sign/rind","chacha20/32","chacha20/200","chacha20/576","chacha20/1200","chacha20/1350","chacha20/16384","mlkem768-keygen","mlkem768-encaps","mldsa65-sign","sha3-256/4096","sha512/4096"]
BASE={"ed":"main","cc":"cx4","kk":"k0"}
LAB={"main":"main e97dc453","g824":"#128 b14b1ed7","cx4":"x4 6ecdfee9","cx8":"x8 1e015593","cx9":"x9 aa6670cf","k0":"pre-#884 51808a98","k1":"#884 91d7dc0c"}
d=collections.defaultdict(dict)  # (set,op,round)->variant->ns
vs=collections.defaultdict(list)
for l in open(sys.argv[1]):
    p=l.split()
    if len(p)!=6 or p[1]!='r': continue
    s,_,r,v,op,ns=p; d[(s,int(op[2:]),int(r))][v]=float(ns)
    if v not in vs[s]: vs[s].append(v)
for s in ["ed","cc","kk"]:
    if s not in vs: continue
    rounds=sorted({r for (ss,o,r) in d if ss==s})
    ctl={}
    good=[]
    for r in rounds:
        c=d[(s,1,r)]
        spread=max(c.values())/min(c.values()) if len(c)==len(vs[s]) else 9
        ctl[r]=spread
        if spread<=1.02: good.append(r)
    print(f"== set {s}: base {LAB[BASE[s]]}; rounds {len(rounds)}; control (aws-lc-rs ed25519 verify, max/min across variants) per round: "+" ".join(f"{ctl[r]:.3f}" for r in rounds)+f"; clean rounds {len(good)}")
    use=good if len(good)>=5 else rounds
    if len(good)<5: print(f"   WARNING only {len(good)} rounds within 1.02; using all {len(rounds)}")
    for op in sorted({o for (ss,o,r) in d if ss==s}):
        if op==1: continue
        row=[]
        for v in vs[s]:
            xs=[d[(s,op,r)][v] for r in use if v in d[(s,op,r)]]
            row.append((v,st.median(xs),(max(xs)/min(xs)-1)*100))
        b=[x for x in row if x[0]==BASE[s]][0][1]
        print(f"  {OPS[op]:28s} "+"  ".join(f"{LAB[v].split()[0]+' '+v}: {m:10.1f}ns ({m/b:.3f}x, sp {sp:.1f}%)" for v,m,sp in row))
