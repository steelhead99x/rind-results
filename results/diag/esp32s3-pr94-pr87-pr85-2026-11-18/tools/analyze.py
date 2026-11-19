# Committed layout: <variant>-<sha>/<bench>/run-N.log (the run scripts wrote board/<variant>/<bench>/).
DIRS={'main': 'main-b9958b33', 'pr94-iram': 'pr94-iram-e874533a', 'pr94-flash': 'pr94-flash-e874533a', 'pr94h-iram': 'pr94h-iram-3654a771', 'pr94h-flash': 'pr94h-flash-3654a771', 'pr85': 'pr85-0fa33a92', 'pr87': 'pr87-68277ff7'}
import json,glob,re,sys,statistics as st,collections
bench=sys.argv[1]; variants=sys.argv[2].split(','); base=sys.argv[3]
data=collections.defaultdict(lambda: collections.defaultdict(list))
kat=collections.defaultdict(lambda:[0,0]); seg={}
for v in variants:
    for f in sorted(glob.glob(f'{DIRS[v]}/{bench}/run-*.log')):
        txt=open(f).read()
        if '"type":"done"' not in txt.replace(' ',''): print("INCOMPLETE",f); continue
        seg.setdefault(v,set()).add(tuple(re.findall(r'segment \d: paddr=\w+ vaddr=\w+ size=(\w+)h',txt)))
        for line in txt.splitlines():
            m=re.search(r'RB:(\{.*)',line)
            if not m: continue
            try: j=json.loads(m.group(1))
            except Exception: continue
            t=j.get('type')
            if t=='check': kat[v][0 if j['pass'] else 1]+=1
            if t=='poly-kat': kat[v][0 if j['pass'] else 1]+=1
            if t=='result' and 'cycles' in j and not j['id'].startswith('harness'): data[v][j['id']].append(j['cycles']['median'])
            if t=='pb': data[v][j['id']].append(j['median'])
print('KAT pass/fail per variant (all runs):',{v:kat[v] for v in variants})
print('image segment signatures per variant (must be 1 each):',{v:len(seg[v]) for v in seg})
rows=sorted(data[base])
print('id',*[f'{v}_med' for v in variants],*[f'{v}/{base}' for v in variants if v!=base],'n','max_spread%',sep='\t')
for i in rows:
    meds={v:st.median(data[v][i]) for v in variants if data[v].get(i)}
    n=min(len(data[v][i]) for v in variants if data[v].get(i))
    sp=max((max(data[v][i])-min(data[v][i]))/st.median(data[v][i])*100 for v in variants if data[v].get(i))
    print(i,*[f'{meds[v]:.0f}' if v in meds else '-' for v in variants],*[f'{meds[v]/meds[base]:.4f}' if v in meds else '-' for v in variants if v!=base],n,f'{sp:.3f}',sep='\t')
