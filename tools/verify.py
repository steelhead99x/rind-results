#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Independent re-check of the published numbers. Shares no code with
report.py: it rereads each run's raw JSONL, takes its own medians, and
compares them with what RESULTS.md, demo.tsv, the chart SVGs and msi-001's
summary.md show. It flags any cell, ratio, fastest library, unit or
cycles/time mismatch, and prints the cycle-counter cross-checks.

Usage: tools/verify.py [--board esp32s3|nrf52840|msi-001 ...]
Exit status 1 if anything mismatched.
"""
import argparse, glob, json, math, os, re, sys
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NOT_RIVALS = {"rind", "esp32s3-hw"}
# rind's admin/bench-report LIBRARIES: benchmark-ID library -> report library.
LIBRARIES = {
    "rind": "rind", "rind-dit-held": "rind-dit-held", "aws-lc-rs": "aws-lc-rs", "ring": "ring",
    "rustcrypto": "rustcrypto", "p256-rustcrypto": "rustcrypto", "p384-rustcrypto": "rustcrypto",
    "dalek": "rustcrypto", "graviola": "graviola", "libcrux": "libcrux", "libcrux-ml-kem": "libcrux",
    "openssl": "openssl", "esp32s3-hw": "esp32s3-hw",
}


def split_id(i):
    """'aes128-gcm/ring/32B' -> ('aes128-gcm/{lib}/32B', 'ring')."""
    parts = i.split("/")
    for j, p in enumerate(parts):
        if p in LIBRARIES:
            return "/".join(parts[:j] + ["{lib}"] + parts[j + 1:]), LIBRARIES[p]
    return None, None


def label(op):
    return op.replace("/{lib}", "").replace("{lib}/", "")
BOARDS = {
    "esp32s3": ("results/raw", "results/RESULTS.md", "media/charts/esp32s3-rind-vs-fastest.svg", "results/demo.tsv"),
    "nrf52840": ("results/raw-nrf52840", "results/nrf52840/RESULTS.md", "media/charts/nrf52840-rind-vs-fastest.svg", None),
}
UNIT = {"ns": 1, "µs": 1e3, "us": 1e3, "ms": 1e6, "s": 1e9}
problems = []


def bad(board, what):
    problems.append(f"{board}: {what}")


def med(xs):
    xs = sorted(xs)
    n = len(xs)
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2


def load_raw(rawdir):
    """id -> list of per-run records; plus every run's start record."""
    per = defaultdict(list)
    starts, checks = [], defaultdict(list)
    clocks = []
    for f in sorted(glob.glob(os.path.join(ROOT, rawdir, "*", "run-*.jsonl"))):
        for line in open(f):
            line = line.strip()
            if not line.startswith("{"):
                continue
            r = json.loads(line)
            if r.get("type") == "result":
                per[r["id"]].append(r)
            elif r.get("type") == "start":
                starts.append(r)
            elif r.get("type") == "clock":
                clocks.append(r)
            elif r.get("type") == "check":
                checks[(r["check"], r["library"])].append(r["pass"])
    return per, starts, checks, clocks


def flash_clocks(rawdir):
    """The SPI flash clock each ESP32-S3 boot reported, {MHz: [log, ...]}: one set of runs must
    not mix them, since every row that misses the instruction cache scales with it."""
    out = defaultdict(list)
    for f in sorted(glob.glob(os.path.join(ROOT, rawdir, "*", "run-*.log"))):
        m = re.search(r"Boot SPI Speed\s*:\s*(\d+)MHz", open(f, errors="replace").read())
        if m:
            out[int(m.group(1))].append(os.path.relpath(f, ROOT))
    return out


def num_with_precision(s):
    s = s.replace(",", "")
    v = float(s)
    dec = len(s.split(".")[1]) if "." in s else 0
    return v, 0.5 * 10 ** -dec


def parse_cell(c):
    """'88.65 µs (664.9 c/B)' -> (ns, ns_tol, kind, value, tol)."""
    m = re.match(r"([\d.,]+) (ns|µs|us|ms|s)(?: \(([\d.,]+) (c/B|c|kc)\))?", c.strip())
    if not m:
        return None
    t, tt = num_with_precision(m.group(1))
    u = UNIT[m.group(2)]
    out = [t * u, tt * u, None, None, None]
    if m.group(3):
        v, vt = num_with_precision(m.group(3))
        k = m.group(4)
        if k == "kc":
            v, vt, k = v * 1e3, vt * 1e3, "c"
        out[2:] = [k, v, vt]
    return out


def close(shown, tol, mine, rel=1e-4):
    return abs(shown - mine) <= tol + rel * abs(mine)


def check_board(board, hz_expect):
    rawdir, md, svg, tsv = BOARDS[board]
    per, starts, checks, clocks = load_raw(rawdir)
    fc = flash_clocks(rawdir)
    if len(fc) > 1:
        bad(board, "runs mix flash clocks: " + ", ".join(f"{k} MHz x{len(v)}" for k, v in sorted(fc.items())))
    elif fc:
        print(f"{board}: flash clock {next(iter(fc))} MHz in all {sum(map(len, fc.values()))} run logs")
    for c in clocks:
        rate = c["cycles"] * 1e9 / c["wall_ns"]
        if abs(rate / hz_expect - 1) > 0.001:
            bad(board, f"cycle counter ran at {rate / 1e6:.3f} MHz against the wall clock")
    if clocks:
        print(f"{board}: {len(clocks)} boot clock checks, "
              + ", ".join(f"{c['cycles'] * 1e3 / c['wall_ns']:.4f}" for c in clocks[:3]) + " MHz ...")
    hz = {s.get("cpu_hz") for s in starts}
    if hz != {hz_expect}:
        bad(board, f"cpu_hz in start records {hz}, expected {hz_expect}")
    agg = {}
    worst_ct = (0, None)
    for i, recs in per.items():
        ns = [r["ns"] for r in recs]
        cyc = [r["cycles"]["median"] for r in recs]
        # cycles vs wall time, per run
        for r in recs:
            want = r["cycles"]["median"] * 1e9 / hz_expect
            d = abs(r["ns"] - want) / want
            if d > worst_ct[0]:
                worst_ct = (d, i)
            if d > 0.01:
                bad(board, f"{i}: ns {r['ns']:.1f} vs cycles/{hz_expect / 1e6:.0f}MHz {want:.1f} ({100 * d:.2f}%)")
            tp = r.get("throughput") or {}
            if "Bytes" in tp and r.get("cycles_per_byte"):
                cpb = r["cycles"]["median"] / tp["Bytes"]
                if abs(cpb - r["cycles_per_byte"]) > 0.01 * cpb:
                    bad(board, f"{i}: cycles_per_byte {r['cycles_per_byte']} vs {cpb:.3f}")
            if r.get("wall_ns") and r["sampled_cycles"] * 1e9 / hz_expect > 1.001 * r["wall_ns"]:
                bad(board, f"{i}: counted more cycles than the wall clock allows")
            c = r["cycles"]
            if not (c["min"] <= c["median"] <= c["max"] and c["p5"] <= c["median"] <= c["p95"]):
                bad(board, f"{i}: cycle percentiles out of order {c}")
        agg[i] = {
            "ns": med(ns),
            "cycles": med(cyc),
            "runs": len(recs),
            "spread": 100 * (max(ns) - min(ns)) / med(ns),
            "bytes": (recs[0].get("throughput") or {}).get("Bytes"),
        }
    table = defaultdict(dict)
    for i, a in agg.items():
        op, lib = split_id(i)
        if op:
            table[op][lib] = a
    runs = {a["runs"] for a in agg.values()}
    print(f"{board}: {len(agg)} ids, runs per id {sorted(runs)}, worst ns-vs-cycles {100 * worst_ct[0]:.4f}% ({worst_ct[1]})")
    failed = [k for k, v in checks.items() if not all(v)]
    print(f"{board}: {len(checks)} known-answer checks, {sum(len(v) for v in checks.values())} results, failing: {failed or 'none'}")
    if failed:
        bad(board, f"failing checks {failed}")

    # time must grow with input size, or the work may have been optimised out
    sized = defaultdict(dict)
    for i, a in agg.items():
        if a["bytes"]:
            parts = i.split("/")
            shape = re.sub(r"[\d]+[KM]?B$", "", parts[-1])  # "quic-" sizes are another message shape
            sized["/".join(parts[:-1]) + "/" + shape][a["bytes"]] = a["ns"]
    for g, d in sized.items():
        sz = sorted(d)
        for s0, s1 in zip(sz, sz[1:]):
            if d[s1] < 0.98 * d[s0]:
                bad(board, f"{g}: {s1} B takes less time than {s0} B")
    # cycles/byte across sizes: a larger input should not cost more per byte
    groups = defaultdict(dict)
    for i, a in agg.items():
        if a["bytes"]:
            parts = i.split("/")
            groups["/".join(parts[:-1])][a["bytes"]] = a["cycles"] / a["bytes"]
    for g, d in sorted(groups.items()):
        sizes = sorted(d)
        for s0, s1 in zip(sizes, sizes[1:]):
            if d[s1] > 1.05 * d[s0] and s1 >= 1024:
                print(f"  note {board}: {g} costs more per byte at {s1} B ({d[s1]:.1f} c/B) than at {s0} B ({d[s0]:.1f})")

    # RESULTS.md tables
    text = open(os.path.join(ROOT, md)).read()
    header = None
    cells = rows = 0
    for line in text.splitlines():
        if line.startswith("| Operation |"):
            header = [h.strip() for h in line.strip("|").split("|")]
            continue
        if not line.startswith("| ") or header is None or line.startswith("|---"):
            if not line.startswith("|"):
                header = None
            continue
        f = [x.strip() for x in line.strip("|").split("|")]
        op = f[0].replace("…", "{lib}")
        if "{lib}" not in op:
            continue
        rows += 1
        t = table.get(op, {})
        if header[1:3] == ["rind", "fastest other Rust library"]:
            rivals = {l: a["ns"] for l, a in t.items() if l not in NOT_RIVALS and not l.startswith("rind-")}
            fl = min(rivals, key=rivals.get)
            shown_lib = f[2].split(":")[0]
            if shown_lib != fl:
                bad(board, f"{op}: fastest shown {shown_lib}, raw says {fl}")
            ratio = t["rind"]["ns"] / rivals[fl]
            r, rt = num_with_precision(f[3].rstrip("×"))
            if not close(r, rt, ratio):
                bad(board, f"{op}: ratio shown {f[3]}, raw {ratio:.4f}")
            sp, spt = num_with_precision(f[4].rstrip("%"))
            if not close(sp, spt, t["rind"]["spread"]):
                bad(board, f"{op}: spread shown {f[4]}, raw {t['rind']['spread']:.3f}")
            pairs = [("rind", f[1]), (fl, f[2].split(": ", 1)[1])]
        else:
            pairs = list(zip(header[1:], f[1:]))
        for lib, c in pairs:
            i = op.replace("{lib}", lib)
            if c == "-":
                if lib in t:
                    bad(board, f"{i}: shown '-' but raw has it")
                continue
            p = parse_cell(c)
            if p is None:
                bad(board, f"{i}: unparsed cell {c!r}")
                continue
            if lib not in t:
                bad(board, f"{i}: shown {c} but not in raw")
                continue
            a = t[lib]
            cells += 1
            if not close(p[0], p[1], a["ns"]):
                bad(board, f"{i}: time shown {c}, raw median {a['ns']:.1f} ns")
            if p[2] == "c/B":
                if not close(p[3], p[4], a["cycles"] / a["bytes"]):
                    bad(board, f"{i}: c/B shown {p[3]}, raw {a['cycles'] / a['bytes']:.2f}")
            elif p[2] == "c":
                if not close(p[3], p[4], a["cycles"]):
                    bad(board, f"{i}: cycles shown {c}, raw {a['cycles']:.0f}")
    print(f"{board}: RESULTS.md {rows} rows, {cells} cells checked")

    # chart: op label then "ratio lib"
    by_label = {label(op): t for op, t in table.items()}
    lab = [re.sub("<[^>]*>", "", t) for t in re.findall(r"<text[^>]*>[^<]*</text>", open(os.path.join(ROOT, svg)).read())]
    n = 0
    charted = set()
    for a, b in zip(lab, lab[1:]):
        m = re.fullmatch(r"([\d.]+)x (\S+)", b)
        if not m or "/" not in a and a not in by_label:
            continue
        t = by_label.get(a)
        if not t:
            bad(board, f"chart label {a} not found in raw")
            continue
        rivals = {l: v["ns"] for l, v in t.items() if l not in NOT_RIVALS and not l.startswith("rind-")}
        fl = min(rivals, key=rivals.get)
        r = t["rind"]["ns"] / rivals[fl]
        n += 1
        charted.add(a)
        if m.group(2) != fl or abs(float(m.group(1)) - r) > 0.005 + 1e-9:
            bad(board, f"chart {a}: shows {b}, raw {r:.3f}x {fl}")
    print(f"{board}: chart {n} bars checked")

    if tsv:
        # Pages are runs of adjacent lines with one heading, as the firmware
        # reads them; each must equal exactly one report operation's row.
        pages = []
        for line in open(os.path.join(ROOT, tsv)):
            if line.startswith("#") or not line.strip():
                continue
            h, lib, ns = line.rstrip("\n").split("\t")
            if not pages or pages[-1][0] != h:
                pages.append((h, {}))
            if lib in pages[-1][1]:
                bad(board, f"demo.tsv page {h!r} has {lib} twice (merged pages?)")
            pages[-1][1][lib] = float(ns)
        heads = [h for h, _ in pages]
        for h in {h for h in heads if heads.count(h) > 1}:
            bad(board, f"demo.tsv heading {h!r} on {heads.count(h)} pages")
        n = 0
        for k, (h, libs) in enumerate(pages, 1):
            if 2 + 5 * len(h) >= 126 - 5 * len(f"{k}/{len(pages)}") - 2:
                bad(board, f"demo.tsv heading {h!r} overlaps its page counter")
            match = [op for op, t in table.items() if set(t) == set(libs)
                     and all(abs(t[l]["ns"] - v) <= 1e-3 * v for l, v in libs.items())]
            if len(match) != 1:
                bad(board, f"demo.tsv page {h!r}: matches {match or 'no report row'}")
            else:
                n += len(libs)
        ratios = []
        wins = 0
        for h, libs in pages:
            riv = [v for l, v in libs.items() if l not in NOT_RIVALS]
            if "rind" in libs and riv:
                ratios.append(libs["rind"] / min(riv))
                wins += libs["rind"] <= min(riv)
        g = math.exp(sum(map(math.log, ratios)) / len(ratios))
        print(f"{board}: demo.tsv {n} bars on {len(pages)} pages checked; OLED summary should read {wins}/{len(ratios)}, geomean {g:.3f}")
    return agg


def check_msi():
    d = json.load(open(os.path.join(ROOT, "results/host/msi-001.json")))
    by = defaultdict(dict)
    for r in d["results"]:
        by[r["operation"]][r["library"]] = r["ns"]
    n = 0
    for line in open(os.path.join(ROOT, "results/host/summary.md")):
        m = re.match(r"\| (\S+) \| ([\d.,]+ \S+), ([\d.]+)× (\S+) \|", line)
        if not m:
            continue
        shown_op = m.group(1)
        ops = [o for o in by if o.replace("/{lib}", "").replace("{lib}/", "") == shown_op]
        if len(ops) != 1:
            bad("msi-001", f"summary row {shown_op}: matched {ops}")
            continue
        t = by[ops[0]]
        rivals = {l: v for l, v in t.items() if l != "rind" and not l.startswith("rind")}
        fl = min(rivals, key=rivals.get)
        ratio = t["rind"] / rivals[fl]
        p = parse_cell(m.group(2))
        n += 1
        if not close(p[0], p[1], t["rind"]):
            bad("msi-001", f"{shown_op}: time {m.group(2)} vs {t['rind']:.1f} ns")
        if m.group(4) != fl or abs(float(m.group(3)) - ratio) > 0.005 + 1e-9:
            bad("msi-001", f"{shown_op}: shows {m.group(3)}x {m.group(4)}, raw {ratio:.3f}x {fl}")
    print(f"msi-001: summary.md {n} rows checked, noisy={d.get('load', {}).get('noisy', d.get('noisy'))}")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--board", action="append")
    a = ap.parse_args()
    boards = a.board or ["esp32s3", "nrf52840", "msi-001"]
    for b in boards:
        if b == "msi-001":
            check_msi()
        else:
            check_board(b, {"esp32s3": 240_000_000, "nrf52840": 64_000_000}[b])
    for p in problems:
        print("MISMATCH", p)
    print(f"{len(problems)} mismatches")
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
