#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Build the rind-results benchmark slide deck.

Run from this directory:

    python3 build.py

Writes rind-results-deck.pptx beside this script. Needs python-pptx and
Pillow, and the PNG files in img/.
"""
from __future__ import annotations

from pathlib import Path
from pptx import Presentation
from pptx.util import Inches, Pt, Emu
from pptx.dml.color import RGBColor
from pptx.enum.shapes import MSO_SHAPE
from pptx.enum.text import PP_ALIGN, MSO_ANCHOR
from PIL import Image

BG=RGBColor(0x0F,0x14,0x19); PANEL=RGBColor(0x17,0x1E,0x26); LINE=RGBColor(0x2A,0x34,0x40)
TXT=RGBColor(0xE8,0xE8,0xE3); MUTED=RGBColor(0x9A,0xA4,0xAE); GREEN=RGBColor(0x3D,0xBE,0x6B)
AMBER=RGBColor(0xF2,0xA3,0x3A); RED=RGBColor(0xE0,0x5A,0x4A)
SANS="Inter"; MONO="JetBrains Mono"
ROOT = Path(__file__).resolve().parent
IMG = ROOT / "img"
prs=Presentation(); prs.slide_width=Inches(13.333); prs.slide_height=Inches(7.5)
BL=prs.slide_layouts[6]

def tb(s,x,y,w,h,paras,size=18,color=TXT,font=SANS,bold=False,align=PP_ALIGN.LEFT,anchor=MSO_ANCHOR.TOP,spacing=6):
    t=s.shapes.add_textbox(Inches(x),Inches(y),Inches(w),Inches(h)); f=t.text_frame; f.word_wrap=True
    f.margin_left=f.margin_right=Inches(0.05); f.margin_top=f.margin_bottom=Inches(0.03); f.vertical_anchor=anchor
    if isinstance(paras,str): paras=[paras]
    for i,p in enumerate(paras):
        para=f.paragraphs[0] if i==0 else f.add_paragraph()
        runs=p if isinstance(p,list) else [(p,{})] if isinstance(p,str) else [p]
        for r in runs:
            if isinstance(r,str): r=(r,{})
            run=para.add_run(); run.text=r[0]; o=r[1]
            run.font.size=Pt(o.get('size',size)); run.font.name=o.get('font',font)
            run.font.bold=o.get('bold',bold); run.font.color.rgb=o.get('color',color)
        para.alignment=align; para.space_after=Pt(spacing)
    return t

def rect(s,x,y,w,h,fill=PANEL,line=None,shape=MSO_SHAPE.RECTANGLE):
    r=s.shapes.add_shape(shape,Inches(x),Inches(y),Inches(w),Inches(h))
    r.fill.solid(); r.fill.fore_color.rgb=fill
    if line: r.line.color.rgb=line; r.line.width=Pt(1)
    else: r.line.fill.background()
    r.shadow.inherit=False
    return r

def base(title,kicker=None,num=None):
    s=prs.slides.add_slide(BL); s.background.fill.solid(); s.background.fill.fore_color.rgb=BG
    rect(s,0.6,0.55,0.08,0.62,fill=GREEN)
    tb(s,0.82,0.45,11.5,0.8,title,size=30,bold=True,anchor=MSO_ANCHOR.MIDDLE)
    if kicker: tb(s,0.82,1.2,11.8,0.5,kicker,size=15,color=MUTED)
    if num: tb(s,12.2,6.95,0.8,0.35,str(num),size=11,color=MUTED,align=PP_ALIGN.RIGHT)
    tb(s,0.6,6.95,8,0.35,"steelhead99x/rind-results",size=11,color=MUTED,font=MONO)
    return s

def notes(s,text): s.notes_slide.notes_text_frame.text=text

def pic(s,path,x,y,w=None,h=None):
    im=Image.open(path); ar=im.width/im.height
    if w and h:
        if w/h>ar: w=h*ar
        else: h=w/ar
    elif w: h=w/ar
    else: w=h*ar
    return s.shapes.add_picture(path,Inches(x),Inches(y),Inches(w),Inches(h))

from pptx.oxml.ns import qn
from lxml import etree
def border(cell,color="2A3440"):
    tcPr=cell._tc.get_or_add_tcPr()
    for i,tag in enumerate(('a:lnL','a:lnR','a:lnT','a:lnB')):
        ln=etree.Element(qn(tag),w="9525"); tcPr.insert(i,ln)
        sf=etree.SubElement(ln,qn('a:solidFill')); c=etree.SubElement(sf,qn('a:srgbClr'),val=color)
def table(s,x,y,w,rows,colw,rowh=0.36,size=13,hl=None,first_mono=False,right_from=1):
    hl=hl or {}
    shp=s.shapes.add_table(len(rows),len(rows[0]),Inches(x),Inches(y),Inches(w),Inches(rowh*len(rows)))
    t=shp.table
    tblPr=shp._element.graphic.graphicData.tbl.tblPr
    for k in ('bandRow','firstRow'): tblPr.set(k,'0')
    for i,cw in enumerate(colw): t.columns[i].width=Inches(cw)
    for r,row in enumerate(rows):
        t.rows[r].height=Inches(rowh)
        for c,val in enumerate(row):
            cell=t.cell(r,c); cell.fill.solid(); cell.fill.fore_color.rgb=PANEL if r==0 else (BG if r%2 else RGBColor(0x13,0x19,0x20))
            cell.margin_left=cell.margin_right=Inches(0.08); cell.margin_top=cell.margin_bottom=Inches(0.02)
            cell.vertical_anchor=MSO_ANCHOR.MIDDLE
            f=cell.text_frame; f.word_wrap=True; p=f.paragraphs[0]; p.text=""
            run=p.add_run(); run.text=val
            run.font.size=Pt(size); run.font.name=SANS
            run.font.bold=(r==0); run.font.color.rgb=MUTED if r==0 else hl.get((r,c),TXT)
            if c>=right_from: p.alignment=PP_ALIGN.RIGHT
            border(cell)
    return t

# 1 Title
s=prs.slides.add_slide(BL); s.background.fill.solid(); s.background.fill.fore_color.rgb=BG
rect(s,0.8,2.1,0.12,2.0,fill=GREEN)
tb(s,1.1,1.95,11,1.2,"rind benchmark results",size=48,bold=True)
tb(s,1.1,3.05,11,1.2,"Four more hosts, three microcontrollers, and how to read the speed chart",size=24,color=MUTED)
tb(s,1.1,5.3,11,1.0,[[("Kyle Douglas",{'bold':True})],[("steelhead99x/rind-results  ·  hosts measured 2026-11-12 at rind b6651423",{'color':MUTED,'size':15,'font':MONO})]],size=18)
notes(s,"Source: README.md of steelhead99x/rind-results (hosts table, commit b6651423).")

# 2 What rind is
s=base("What rind is, and what was measured",num=2)
tb(s,0.82,1.6,6.2,4.8,[
 [("rind",{'bold':True,'color':GREEN}),(" is a pure-Rust crypto library and a drop-in for ring, aws-lc-rs, and rustls. Its assembly lives in Rust source, so it needs no C toolchain.",{})],
 "This repo runs rind's own benchmark harness on hosts the upstream report does not cover, then adds microcontroller boards.",
 "Each operation is timed for rind and six others: aws-lc-rs, ring, RustCrypto + dalek, graviola, libcrux, OpenSSL. Every library runs its native path on each host.",
 "Operations cover the TLS 1.3 handshake through rustls, AEADs, QUIC header protection, key agreement, signatures, hashes, RSA, ML-KEM, and ML-DSA.",
],size=17,spacing=12)
for i,(big,lab) in enumerate([("202 / 248","times where rind is fastest or within 10%, on the four new hosts"),("533 / 602","the same count across all ten hosts, with the six from upstream BENCHMARKS.md")]):
    y=1.7+i*2.45
    rect(s,7.6,y,5.1,2.15,fill=PANEL)
    tb(s,7.85,y+0.2,4.7,1.0,big,size=48,bold=True,color=GREEN)
    tb(s,7.85,y+1.2,4.7,0.9,lab,size=15,color=MUTED)
notes(s,"Sources: kixel-ai/rind README.md (drop-in, no C toolchain); rind-results README.md intro; titles of benchmarks/overview.svg (202 of 248) and benchmarks/overview-all.svg (533 of 602).")

# 3 Hosts and method
s=base("Hosts and method",num=3)
table(s,0.82,1.6,7.4,[
 ["Host","CPU","Notes"],
 ["xeon-vm","Intel Xeon, 8 vCPUs","cloud VM, shared"],
 ["m1pro","Apple M1 Pro","daily-use laptop, noisy"],
 ["pi5","Raspberry Pi 5, 4× Cortex-A76","idle"],
 ["pixel9pro","Google Tensor G4","over adb, noisy"],
],[1.5,3.4,2.5],rowh=0.5,size=14,right_from=99)
tb(s,0.82,4.35,7.4,2.3,["All four measured 2026-11-12 at rind b6651423. Times are criterion's estimates.",
 "The Pixel's benches were cross-compiled on xeon-vm and run over adb with the same criterion arguments."],size=14,color=MUTED,spacing=8)
steps=[("1","admin/bench-report run","rind's own harness, driven by cargo xtask. Same operations, criterion arguments, parsing, and JSON."),
       ("2","Raw JSON per host","hosts/<host>/ keeps the raw results, a trimmed log, and notes."),
       ("3","cargo verify","A separate tool that shares no code with xtask recomputes every published number from raw.")]
for i,(n,h,d) in enumerate(steps):
    y=1.6+i*1.65
    rect(s,8.75,y,0.6,0.6,fill=GREEN,shape=MSO_SHAPE.OVAL)
    tb(s,8.75,y,0.6,0.6,n,size=18,bold=True,color=BG,align=PP_ALIGN.CENTER,anchor=MSO_ANCHOR.MIDDLE)
    tb(s,9.55,y-0.05,3.6,0.45,h,size=16,bold=True,font=MONO if i!=1 else SANS)
    tb(s,9.55,y+0.4,3.6,1.1,d,size=13,color=MUTED)
notes(s,"Sources: README.md Hosts table and method paragraph; per-host notes in hosts/*/; README intro on cargo verify (tools/verify).")

# 4 How to read
s=base("How to read the speed chart",num=4)
pic(s, str(IMG / "ov_top.png"),0.82,1.55,w=7.6)
tb(s,0.82,4.2,7.6,0.4,"Top of benchmarks/overview-dark.svg: the legend and the TLS 1.3 rows",size=12,color=MUTED)
tb(s,8.8,1.55,4.0,5.3,[
 [("Row",{'bold':True,'color':GREEN}),(" one operation",{})],
 [("Column",{'bold':True,'color':GREEN}),(" one library",{})],
 [("Squares",{'bold':True,'color':GREEN}),(" one per host, left to right: xeon-vm, m1pro, pi5, pixel9pro",{})],
 [("Color",{'bold':True,'color':GREEN}),(" that library's time over the fastest library on the same host. Green is fastest or within 10%, then 1.25×, 1.5×, 2×, 4×, 8×. Dark red is over 8×.",{})],
 [("Shield",{'bold':True,'color':GREEN}),(" formally verified code",{})],
 [("No square",{'bold':True,'color':GREEN}),(" the library lacks the operation, or the host did not time it",{})],
],size=15,spacing=11)
tb(s,0.82,4.75,7.6,2.0,[
 [("rind (DIT held)",{'bold':True,'font':MONO}),(" is timed only on Apple CPUs and never counts as the fastest. Elsewhere it would run the same code as rind's own column.",{})],
 "Each square's tooltip, in the SVG opened on its own, gives its time. The tables under each host give them all."],size=14,color=MUTED,spacing=8)
notes(s,"Source: README.md, Speed section (legend, square order, DIT held column).")

# 5 Headline wins
s=base("Headline wins on xeon-vm",kicker="Times from the xeon-vm table. Lower is better. RustCrypto and libcrux columns left out for space.",num=5)
rows=[["Operation","rind","aws-lc-rs","ring","graviola","OpenSSL"],
 ["full handshake, X25519","228.44 µs","299.27 µs","334.33 µs","280.39 µs","586.79 µs"],
 ["full handshake, X25519MLKEM768","255.27 µs","348.37 µs","-","318.02 µs","706.83 µs"],
 ["full handshake, Ed25519 cert","179.11 µs","235.59 µs","282.36 µs","208.12 µs","645.60 µs"],
 ["resumed handshake, X25519","80.08 µs","138.13 µs","163.86 µs","97.55 µs","370.21 µs"],
 ["AES-128-GCM seal, 32 B","30 ns","76 ns","62 ns","56 ns","84 ns"],
 ["AES-128-GCM seal, 1350 B","124 ns","153 ns","190 ns","180 ns","168 ns"],
 ["AES-256-GCM open, 1350 B","120 ns","170 ns","201 ns","192 ns","201 ns"],
 ["AES-128 header protection","3 ns","8 ns","8 ns","-","-"],
 ["Ed25519 verify","30.03 µs","54.93 µs","97.09 µs","30.97 µs","89.50 µs"],
]
hl={(r,1):GREEN for r in range(1,len(rows))}
table(s,0.82,1.75,11.7,rows,[3.7,1.6,1.6,1.6,1.6,1.6],rowh=0.42,size=14,hl=hl)
tb(s,0.82,6.05,11.7,0.6,"The one gap on this host: ChaCha20-Poly1305 seal at 16 KB, 1.67× OpenSSL's time (6.69 µs against 3.99 µs).",size=14,color=AMBER)
notes(s,"Source: README.md, 'Times on xeon-vm' table and 'Gaps on these hosts' table.")

# 6 What DIT is
s=base("What DIT is, and why it matters",num=6)
cards=[("The risk: timing side channels","If crypto code takes longer for some secret values than others, someone who can time it can learn about the key. rind's secret-handling code already avoids branches, memory indices, and variable-time instructions that depend on a secret."),
       ("The CPU switch: DIT","Data Independent Timing is an Arm CPU mode. While it is on, the instructions Arm lists as data-independent take the same time whatever values they process. Apple M-series cores have it."),
       ("Extra on Apple M3 and later","From the M3, DIT also turns off the data memory-dependent prefetcher that GoFetch leaks keys through, even from constant-time code. The M1 and M2 have no such switch.")]
for i,(h,d) in enumerate(cards):
    x=0.82+i*4.1
    rect(s,x,1.65,3.8,3.75,fill=PANEL)
    rect(s,x,1.65,3.8,0.07,fill=GREEN if i<2 else AMBER)
    tb(s,x+0.25,1.9,3.3,0.9,h,size=18,bold=True)
    tb(s,x+0.25,2.85,3.3,2.5,d,size=15,color=MUTED,spacing=6)
tb(s,0.82,5.95,11.7,0.8,"DIT is a second layer, not the constant-time guarantee. Arm says it changes nothing on its current cores, so rind sets it only on Apple's platforms. aws-lc-rs and ring never set it.",size=15)
notes(s,"Sources: kixel-ai/rind README.md 'Data-independent timing' section; rind/src/low/cpu.rs (Dit docs); admin/ct-vartime comment citing Arm DDI 0601 'DIT, Data Independent Timing' list of data-independent-time instructions; PLATFORMS.md. The general description of a timing side channel is a plain-language gloss of the README's constant-time wording, not a quote.")

# 7 What rind does
s=base("What rind does with DIT, and what it costs",num=7)
def bar(y,label,segs):
    tb(s,0.82,y-0.05,3.0,0.5,label,size=14,bold=True)
    x=3.6
    for w,t,c in segs:
        r=rect(s,x,y,w,0.42,fill=c); 
        tb(s,x,y,w,0.42,t,size=11,color=BG if c!=PANEL else TXT,align=PP_ALIGN.CENTER,anchor=MSO_ANCHOR.MIDDLE,bold=True)
        x+=w+0.04
D=AMBER; W=RGBColor(0x5B,0x8D,0xD9)
bar(1.75,"Per call (default)",[(0.55,"on",D),(0.85,"work",W),(0.55,"off",D)]*3)
bar(2.5,"With Dit::hold()",[(0.55,"on",D),(0.85,"work",W),(0.85,"work",W),(0.85,"work",W),(0.55,"off",D)])
tb(s,0.82,3.25,6.0,3.4,[
 "On Apple's platforms, each rind call that handles a secret turns DIT on at entry and back off on return. That leaves the rest of the thread as the application set it.",
 "The switch costs about 30 to 55 ns a call on the M4, per rind's gaps.toml. rind's README puts it at about 45 to 60 ns on M1 and M4, a fifth of sealing a 1350-byte packet.",
 "A fixed cost per call only shows when the work is tiny. On m1pro, a 16 KB AES-128-GCM seal is 1.96 µs, and 1.90 µs with DIT held."],size=14,color=MUTED,spacing=10)
rect(s,7.3,3.3,5.4,2.85,fill=PANEL)
tb(s,7.55,3.45,5.0,0.4,"Pay it once for a batch",size=16,bold=True)
tb(s,7.55,3.95,5.0,0.5,"let _dit = rind::cpu::Dit::hold();",size=14,font=MONO,color=GREEN)
tb(s,7.55,4.55,5.0,2.0,["The guard keeps DIT on for this thread until it drops, for direct calls and adapters such as rind-rustls.",
 "It is tied to its thread, so it cannot cross an .await on a multi-threaded runtime. Off Apple's platforms it does nothing."],size=13,color=MUTED,spacing=8)
notes(s,"Sources: kixel-ai/rind README.md (per-call on/off, 45-60 ns on M1 and M4, guard example); benchmarks/gaps.toml DIT entry (about 30 to 55 ns a call on the M4); rind/src/low/cpu.rs (about 55 ns on M1, thread-bound guard); rind-results README m1pro table (AES-128-GCM seal 16 KB). The two cost figures differ by source and are both shown as written.")

# 8 Tradeoff on chart
s=base("The Apple DIT tradeoff on the chart",kicker="m1pro: the red and orange squares in rind's column mostly go green in the DIT held column",num=8)
pic(s, str(IMG / "m1_dit.png"),0.82,1.75,w=7.9)
tb(s,0.82,5.05,7.9,1.7,[
 [("AES-128 header protection",{'bold':True}),(": rind 50 ns, held 5 ns, aws-lc-rs 4 ns.",{})],
 [("HMAC-SHA256, 64 B",{'bold':True}),(": rind 241 ns, held 142 ns, RustCrypto 144 ns.",{})],
 [("AES-128-GCM seal, 32 B",{'bold':True}),(": rind 71 ns, held 17 ns, RustCrypto 42 ns.",{})]],size=14,color=MUTED,spacing=6)
tb(s,9.1,1.75,3.7,5.0,[
 [("A deliberate choice. ",{'bold':True,'color':TXT}),("Keeping DIT per call was decided with the maintainer from kixel-ai/rind#387's findings. Each call is safe on its own.",{})],
 [("Hold closes most of it. ",{'bold':True,'color':TXT}),("Of 17 m1pro gaps in the DIT entry, holding DIT closes 14. MoQ holds it across batches.",{})],
 [("Other off-green squares ",{'bold':True,'color':TXT}),("in the ten-host chart are stale data fixed upstream: #394 (SHA-256 64 B on nixos) and #415 (Ed25519 verify on the OVH VPSes).",{})]],size=14,color=MUTED,spacing=12)
notes(s,"Sources: rind-results README m1pro table; hosts/m1pro/NOTES.md 'DIT and the gaps table (draft decision)' (17 gaps, 14 closed; three stay open: aes128 header protection 1.15x, sha256/64B 1.17x, sha512/1350B 1.40x); kixel-ai/rind benchmarks/gaps.toml DIT entry (#387, #431, 'as MoQ does'); upstream gaps.toml entries for #394 (nixos sha256/64B) and #415 (ovh-iad, ovh-sea ed25519-verify), both 'closed after this data'.")

# 9 Microcontrollers
s=base("Microcontrollers",kicker="rind's time over the fastest other Rust library. Below 1× means rind is faster.",num=9)
for i,(name,sub,rows,gap) in enumerate([
 ("ESP32-S3","Xtensa LX7, 240 MHz, flash at 80 MHz, ESP-IDF",
  [["AES-128-GCM seal, 1350 B","1.71 ms","0.82× ring"],["P-256 ECDH","49.34 ms","0.59× RustCrypto"],["ML-KEM-768 decaps","5.20 ms","0.28× RustCrypto"],["QUIC header prot., AES-128","24.20 µs","0.65× ring"]],
  "Gaps: SHA-512 about 33× ring and XChaCha 32 B 7.50×, both cache misses. 16 Nov: a fork fix cuts rind's SHA-512 time to 0.033× of before (not published yet)."),
 ("nRF52840 (Seeed T1000-E)","Cortex-M4F, 64 MHz, bare metal, no_std",
  [["AES-128-GCM seal, 1350 B","5.28 ms","1.03× RustCrypto"],["P-256 ECDH","73.81 ms","0.43× ring"],["ML-KEM-768 decaps","10.25 ms","0.46× libcrux"],["QUIC header prot., AES-128","74.63 µs","0.73× ring"]],
  "Gaps: SHA-256 16 KB 1.62× ring (assembly against portable Rust), ChaCha20-Poly1305 1350 B 1.78× ring")]):
    x=0.82+i*6.1
    tb(s,x,1.7,5.8,0.45,name,size=19,bold=True,color=GREEN)
    tb(s,x,2.15,5.8,0.4,sub,size=13,color=MUTED)
    table(s,x,2.65,5.7,[["Operation","rind","vs best"]]+rows,[2.75,1.15,1.8],rowh=0.4,size=13)
    tb(s,x,4.75,5.7,0.8,gap,size=13,color=AMBER)
rect(s,0.82,5.65,11.8,1.05,fill=PANEL)
tb(s,1.05,5.72,11.4,0.95,[[("RP2040 (Cortex-M0+): ",{'bold':True,'color':TXT}),("every image now builds for thumbv6m-none-eabi (patches 0003 and 0005). Known-answer checks passed on a Pico on 2026-11-12. No timing results are published in this repo yet.",{})]],size=14,color=MUTED,anchor=MSO_ANCHOR.MIDDLE)
notes(s,"Update 2026-11-17: the SHA-512 follow-up is steelhead99x/rind#69 (merged on the fork 2026-11-16 19:01 PST), board-run comment 6936378505: #69 head 504049d4 against upstream main 19f94a07, 3 runs per image, SHA-512 and SHA-512/256 0.0329x to 0.0333x of the old rind time at every size. That run is in a PR comment, not on rind-results main. Sources: results/RESULTS.md (ESP32-S3 headline table and Known gaps); results/nrf52840/RESULTS.md (headline and Known gaps); hosts/rp2040/NOTES.md and README intro (RP2040 status). ESP32-S3 SHA-512 is 32.56x to 32.96x ring; the xchacha 32 B gap is 1.10x RustCrypto with a 32 KB instruction cache.")

# 10 Corrections
s=base("Corrections along the way",num=10)
cards=[("40 MHz flash","The first ESP32-S3 set ran its flash at espflash's default 40 MHz, not the configured 80 MHz. Cache-missing rows were slowed by up to 1.9×. SHA-256 64 B went from 6.65× to 2.29× libcrux. No gap verdict changed.","results/diag/esp32s3-40mhz-2026-11-11"),
 ("LLVM-Xtensa crash","The ChaCha20-Poly1305 images crashed in ring, not rind. ring's align(64) Poly1305 state made LLVM-Xtensa realign the stack with add instead of movsp (espressif/llvm-project#140). A ring patch keeps it at 16 bytes; 3 of 3 runs pass.","results/diag/esp32s3-chacha-crash.md"),
 ("Relink layouts","Short-input rows were rerun with the libraries moved in memory. On the ESP32-S3, SHA-256 64 B measured 2.36× to 3.07× libcrux across three alignments. Paddings that kept the same alignment timed the same.","results/diag/esp32s3-relink-fixed-98568332")]
for i,(h,d,src) in enumerate(cards):
    x=0.82+i*4.1
    rect(s,x,1.6,3.8,5.0,fill=PANEL); rect(s,x,1.6,3.8,0.07,fill=AMBER)
    tb(s,x+0.25,1.85,3.3,0.5,h,size=19,bold=True)
    tb(s,x+0.25,2.5,3.3,3.3,d,size=16,color=MUTED,spacing=6)
    tb(s,x+0.25,5.85,3.3,0.6,src,size=10,color=MUTED,font=MONO)
notes(s,"Sources: README.md correction notes; results/diag/esp32s3-40mhz-2026-11-11/README.md; results/diag/esp32s3-chacha-crash.md; results/diag/esp32s3-relink-fixed-98568332 as summarized in README.md.")

# 11 Build cost
s=base("Build cost",kicker="Cold cargo build --release, median of three, on xeon-vm (8 jobs)",num=11)
rows=[["Build","Wall s","CPU s","Stripped MB","Tools needed","Crates"],
 ["rind","6.9","14","0.89","none","5"],
 ["aws-lc-rs","19.4","51","2.99","cc","14"],
 ["ring","8.0","11","0.74","cc","9"],
 ["RustCrypto + dalek","4.3","21","0.54","none","57"],
 ["graviola","3.2","6","0.91","none","5"],
 ["OpenSSL, vendored","34.4","191","5.13","cc, make, perl","19"],
 ["rustls + rind-rustls","7.9","34","3.00","none","14"],
 ["rustls + aws-lc-rs","26.8","77","4.11","cc","20"],
 ["rustls + ring","12.8","29","1.93","cc","16"]]
hl={(1,c):GREEN for c in range(6)}; hl.update({(7,c):GREEN for c in range(6)})
table(s,0.82,1.75,8.2,rows,[2.5,0.9,0.9,1.35,1.75,0.8],rowh=0.43,size=14,hl=hl)
tb(s,9.4,1.75,3.4,4.6,["rind needs no build tools beyond rustc and a linker.",
 "A rustls client over rind built in 7.9 s here, against 26.8 s over aws-lc-rs and 12.8 s over ring.",
 "On pi5 the same three took 31.4 s, 77.2 s, and 38.8 s.",
 "Not measured on m1pro (host busy) or pixel9pro (cross-compiled)."],size=14,color=MUTED,spacing=12)
notes(s,"Source: README.md, Build cost section (xeon-vm and pi5 tables; m1pro and pixel9pro not measured).")

# 12 Gaps and next steps
s=base("Open gaps and next steps",num=12)
tb(s,0.82,1.6,5.9,0.45,"Gaps over 10% on these hosts",size=18,bold=True,color=AMBER)
tb(s,0.82,2.15,5.9,4.6,[
 [("pi5: ",{'bold':True,'color':TXT}),("aws-lc-rs leads on X25519 (2.32×), RSA-2048 (about 1.66×), P-256 ECDH (1.53×), and the handshakes.",{})],
 [("m1pro: ",{'bold':True,'color':TXT}),("three DIT-listed gaps stay open with DIT held (AES-128 header protection 1.15×, SHA-256 64 B 1.17×, SHA-512 1350 B 1.40×), plus SHA-256 16 KB at 1.56×.",{})],
 [("xeon-vm, pixel9pro: ",{'bold':True,'color':TXT}),("ChaCha20-Poly1305 seal at 16 KB against OpenSSL (1.67×, 1.14×).",{})],
 [("Boards: ",{'bold':True,'color':TXT}),("Poly1305 on 32-bit cores, SHA-2 on the nRF52840, instruction-cache misses on the ESP32-S3.",{})]],size=16,color=MUTED,spacing=14)
tb(s,7.3,1.6,5.5,0.45,"Next steps named in the repo",size=18,bold=True,color=GREEN)
tb(s,7.3,2.15,5.5,4.6,[
 "Profile SHA-256 16 KB on the M1 Pro before it gets a reason or a quest.",
 "A 32-bit radix-2^26 Poly1305 for targets without 64-bit multiplies. An optional 32-bit-limb Poly1305 already beats rind's previous code on both boards.",
 "An M-profile SHA-256 block function for the nRF52840.",
 "Publish RP2040 timings.",
 "Add m1pro to bench-report's HOSTS so gaps.toml can list it."],size=16,color=MUTED,spacing=12)
notes(s,"Sources: README.md 'Gaps on these hosts'; hosts/m1pro/NOTES.md; results/RESULTS.md and results/nrf52840/RESULTS.md Known gaps; README intro (32-bit-limb Poly1305); hosts/rp2040/NOTES.md.")

# 13 Since this deck: Intel's timing list and SHA-512 (added 2026-11-17)
s=base("Since this deck: SHA-512 and Intel's timing list",kicker="Intel's version of the DIT idea, and a change that trades speed for it",num=13)
tb(s,0.82,1.75,6.4,4.9,[
 "Intel publishes a list of instructions whose timing does not depend on the data they process. It plays the same role as Arm's DIT list.",
 [("The gap: ",{'bold':True,'color':TXT}),("the SHA-512 instructions on Arrow Lake and later CPUs (VSHA512RNDS2, VSHA512MSG1, VSHA512MSG2) are not on that list. rind used them for SHA-384 and SHA-512, and so for HMAC keys, HKDF, and Ed25519 signing.",{})],
 [("The change: ",{'bold':True,'color':TXT}),("kixel-ai/rind#821 sends those CPUs to the AVX2 routine that already has a proof, and admin/ct-vartime now refuses vsha512 instructions on x86.",{})],
 [("Status: ",{'bold':True,'color':TXT}),("open upstream, ready for review.",{})]],size=16,color=MUTED,spacing=14)
for i,(big,lab,col) in enumerate([("2.0–2.2×","SHA-512 time at 32 and 64 B, against the old path",AMBER),("4.4–4.7×","SHA-512 time from 1200 B up, against the old path",AMBER),("0.81–0.86×","of aws-lc-rs's time, still, at every size",GREEN)]):
    y=1.7+i*1.62
    rect(s,7.7,y,5.0,1.42,fill=PANEL)
    tb(s,7.95,y+0.12,4.6,0.7,big,size=34,bold=True,color=col)
    tb(s,7.95,y+0.85,4.6,0.5,lab,size=14,color=MUTED)
tb(s,7.7,6.55,5.0,0.35,"Core Ultra 9 275HX, five alternating rounds, one P-core",size=11,color=MUTED)
notes(s,"Sources: kixel-ai/rind#821 (head 1b277b04, opened by steelhead99x 2026-11-16 20:17 PST); its commits 628fdb69 (AVX2 routine where the CPU has SHA512), 4342478f (ct-vartime refuses VSHA512), and a674bc6e (the Arrow Lake measurement: five alternating rounds of upstream main and the branch on a Core Ultra 9 275HX pinned to one P-core; SHA-512 takes 2.0 to 2.2x its former time at 32 and 64 B and 4.4 to 4.7x from 1200 B, and stays 0.81 to 0.86x aws-lc-rs's at every size); quest/m2/confidence/intel-vsha512.md on kixel-ai/rind main (Intel's list updated 2026-02-23). Checked 2026-11-17.")

# 14 Newer sets on main (added 2026-11-17)
s=base("Newer sets on main since these four hosts",kicker="Each one is its own publish, at its own rind commit. Counts are from each set's RESULTS.md.",num=14)
rows=[["Set on main","Hardware","rind commit","Runs","rind fastest"],
 ["jetson-orin-nano","NVIDIA Jetson Orin Nano, aarch64","98568332","3","65 of 68"],
 ["intel-core-ultra-9-275hx","Intel Core Ultra 9 275HX","d6ca9a75","3","55 of 68"],
 ["pi5-nofan-1p5ghz","Raspberry Pi 5, no fan, held at 1.5 GHz","a6577cb4","5","45 of 68"],
 ["pi-host-a-bench","Raspberry Pi 2, Cortex-A7 at 900 MHz","a6577cb4","5","38 of 68"],
 ["esp32s3 (d6ca9a75)","ESP32-S3 board, 12 images, comparable set","d6ca9a75","3 each","see RESULTS.md"],
 ["uno-r3","Arduino Uno R3, generic primitives only","c08c91ea","-","not the full crate"]]
hl={(1,4):GREEN}
table(s,0.82,1.75,11.7,rows,[2.9,4.0,1.6,0.9,2.3],rowh=0.5,size=14,hl=hl,right_from=3)
tb(s,0.82,5.45,11.7,1.2,["\"rind fastest\" counts the operations rind shares with at least one other library. The commits and clocks differ, so these rows are not a ranking of hosts.",
 "The Pi 5 set runs at 1.5 GHz on purpose (no fan), so compare its ratios, not its times."],size=14,color=MUTED,spacing=8)
notes(s,"Sources, all on rind-results main 88cbf85 (checked 2026-11-17): results/jetson-orin-nano/RESULTS.md (65 of 68, rind 98568332, 3 runs); results/intel-core-ultra-9-275hx/d6ca9a75.../RESULTS.md (55 of 68, 3 runs, PR #29); results/pi5-nofan-1p5ghz/a6577cb4.../RESULTS.md (45 of 68, five rounds at 1.5 GHz, PR #28); results/pi-host-a-bench/a6577cb4.../RESULTS.md (38 of 68, five rounds, PR #24); results/esp32s3/d6ca9a75.../RESULTS.md (36 of 36 runs: 12 images, 3 runs each; PR #17); results/uno-r3/RESULTS.md (generic primitives from c08c91ea, not the full rind crate, PR #25).")

prs.save(str(ROOT / "rind-results-deck.pptx"))
