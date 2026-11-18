#!/usr/bin/env python3
# Bench-only: remove the verified-garbage rows (that crate has no Xtensa support) from the
# sha2, hmac and hkdf bench files. No rind row, rind call, or other library's row is touched.
import re, sys
d = sys.argv[1] + "/rind-bench/benches/"

def drop_fn(t, name):
    m = re.search(r'(?m)^((?://[^\n]*\n)*)fn ' + name + r'\b', t)
    assert m, name
    i = t.index('{', t.index('\n', m.end()) if False else m.end())
    # the fn signature may contain no braces before the body
    depth = 0
    j = i
    while True:
        c = t[j]
        if c == '{': depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0: break
        j += 1
    end = j + 1
    while t[end] == '\n': end += 1
    return t[:m.start()] + t[end:]

def drop_stmt(t, start_pat):
    # remove a statement starting at the line matching start_pat through its terminating ');'
    m = re.search(start_pat, t)
    assert m, start_pat
    ls = t.rfind('\n', 0, m.start()) + 1
    depth = 0; j = m.start()
    while True:
        c = t[j]
        if c in '([{': depth += 1
        elif c in ')]}': depth -= 1
        elif c == ';' and depth == 0: break
        j += 1
    return t[:ls] + t[j + 2:]

def gen(t):
    t = re.sub(r'<V: verified_garbage::hmac::HmacHash>', '<V>', t)
    t = re.sub(r'bench::<verified_garbage::hashes::\w+::\w+>', 'bench::<()>', t)
    return t

# sha2.rs
p = d + "sha2.rs"; t = open(p).read()
t = drop_fn(t, 'test_vg'); t = drop_fn(t, 'vg_row')
while 'vg_row::<' in t: t = drop_stmt(t, r'vg_row::<')
open(p, 'w').write(t)

# hmac.rs
p = d + "hmac.rs"; t = open(p).read(); t = gen(t)
t = drop_stmt(t, r'assert_eq!\(\s*verified_garbage::hmac')
t = drop_stmt(t, r'group\.bench_with_input\(\s*BenchmarkId::new\("verified-garbage", size_name\)')
t = drop_stmt(t, r'// Its keyed state, cloned per message[^\n]*\n\s*let vg_key')
t = drop_stmt(t, r'group\.bench_with_input\(\s*BenchmarkId::new\("verified-garbage/reused-key"')
open(p, 'w').write(t)

# hkdf.rs
p = d + "hkdf.rs"; t = open(p).read(); t = gen(t)
t = drop_fn(t, 'vg_expand'); t = drop_fn(t, 'vg')
t = drop_stmt(t, r'vg::<V>\(&mut group')
open(p, 'w').write(t)
