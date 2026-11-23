import re,sys,glob,os,json,statistics
d=sys.argv[1]; res={}
for f in sorted(glob.glob(d+'/r*.log')):
    b=os.path.basename(f)[:-4]; r,op,l=b.split('.',2)
    t=open(f).read()
    m=re.search(r'time:\s+\[([\d.]+) (\S+) ([\d.]+) (\S+) ([\d.]+) (\S+)\]',t)
    if not m: print('MISSING',b); continue
    u={'ns':1,'µs':1e3,'us':1e3,'ms':1e6,'s':1e9}[m.group(4)]
    res[(int(r[1:]),op,l)]=float(m.group(3))*u
rounds=sorted({k[0] for k in res})
MAXR=int(sys.argv[2]) if len(sys.argv)>2 else 99
rounds=[r for r in rounds if r<=MAXR]
out={}
for op in ['keygen','encaps','decaps']:
    disc=[];good=[]
    for r in rounds:
        a=res.get((r,op,'rind'));c=res.get((r,op,'rind-control'))
        if a and c and max(a,c)/min(a,c)<=1.02: good.append(r)
        else: disc.append(r)
    row={'op':op,'kept':good,'discarded':disc}
    for l in ['rind','libcrux-kem','rustcrypto']:
        v=[res[(r,op,l)] for r in good if (r,op,l) in res]
        row[l]=statistics.median(v) if v else None
        row[l+'_all']=[round(res[(r,op,l)],1) for r in rounds if (r,op,l) in res]
    row['control_all']=[round(res[(r,op,'rind-control')],1) for r in rounds if (r,op,'rind-control') in res]
    out[op]=row
print(json.dumps(out,indent=1))
