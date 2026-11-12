#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Compares two sets of board runs of the same benches, row by row: a relinked build against the
published one, a newer rind against the pinned one, a cache configuration against the default.

    tools/compare.py A_RAW B_RAW BENCH... [--label-a A] [--label-b B] [--out FILE.md]

A_RAW and B_RAW hold `<bench>/run-*.jsonl` as tools/capture.py writes them. Each row's value is
the median over runs of each run's median cycles. The Markdown has, per bench: the known-answer
checks of both sides, a per-library summary of B/A split by input size (short: under 1 KB or no
size; long: 1 KB and up), and every row whose B/A falls outside max(1%, either side's run spread).
"""
import argparse
import glob
import json
import os
import re
import statistics
from collections import defaultdict


def load(raw, bench):
    rows, checks = defaultdict(list), []
    for f in sorted(glob.glob(os.path.join(raw, bench, "run-*.jsonl"))):
        for line in open(f):
            try:
                r = json.loads(line)
            except ValueError:
                continue
            if r.get("type") == "result":
                rows[r["id"]].append(r["cycles"]["median"])
            elif r.get("type") == "check":
                checks.append(bool(r["pass"]))
    out = {}
    for k, v in rows.items():
        m = statistics.median(v)
        out[k] = (m, 100 * (max(v) - min(v)) / m if m else 0.0, len(v))
    return out, checks


def size_of(i):
    m = re.search(r"(\d+)(KB|B)$", i.split("/")[-1])
    if not m:
        return None
    return int(m.group(1)) * (1024 if m.group(2) == "KB" else 1)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("a")
    ap.add_argument("b")
    ap.add_argument("benches", nargs="+")
    ap.add_argument("--label-a", default="A")
    ap.add_argument("--label-b", default="B")
    ap.add_argument("--out")
    args = ap.parse_args()
    L = [f"Cycles, {args.label_b} over {args.label_a} (below 1 is faster in {args.label_b}).", ""]
    for bench in args.benches:
        A, ca = load(args.a, bench)
        B, cb = load(args.b, bench)
        common = sorted(set(A) & set(B) - {k for k in A if k.startswith("harness/")})
        L += [f"## {bench}", "",
              f"Known-answer checks: {args.label_a} {sum(ca)}/{len(ca)}, {args.label_b} {sum(cb)}/{len(cb)}. "
              f"Runs: {max((A[k][2] for k in common), default=0)} and {max((B[k][2] for k in common), default=0)}. "
              f"Rows: {len(common)} in both"
              + (f"; only in {args.label_b}: {len(set(B) - set(A))}" if set(B) - set(A) else "") + ".", ""]
        groups = defaultdict(list)
        for k in common:
            p = k.split("/")
            lib = p[1] if len(p) > 2 else p[-1]
            s = size_of(k)
            groups[(lib, "per op" if s is None else "long" if s >= 1024 else "short")].append(B[k][0] / A[k][0])
        L += ["| Library | Inputs | Rows | Median | Range |", "|---|---|---:|---:|---|"]
        for (lib, kind), v in sorted(groups.items()):
            L.append(f"| {lib} | {kind} | {len(v)} | {statistics.median(v):.3f} | {min(v):.3f} to {max(v):.3f} |")
        moved = [(B[k][0] / A[k][0], k) for k in common
                 if abs(B[k][0] / A[k][0] - 1) * 100 > max(1.0, A[k][1], B[k][1])]
        L += ["", f"{len(moved)} of {len(common)} rows moved by more than max(1%, run spread)"
              + (":" if moved else "."), ""]
        if moved:
            L += [f"| Row | {args.label_a} cycles | {args.label_b} cycles | B/A |", "|---|---:|---:|---:|"]
            for r, k in sorted(moved, key=lambda x: abs(x[0] - 1), reverse=True):
                L.append(f"| `{k}` | {A[k][0]:,.0f} | {B[k][0]:,.0f} | {r:.3f} |")
            L.append("")
        new = sorted(set(B) - set(A))
        if new:
            L += [f"Rows only in {args.label_b}:", "", "| Row | cycles | spread | c/B |", "|---|---:|---:|---:|"]
            for k in new:
                s = size_of(k)
                L.append(f"| `{k}` | {B[k][0]:,.0f} | {B[k][1]:.2f}% | {B[k][0] / s:.1f} |" if s
                         else f"| `{k}` | {B[k][0]:,.0f} | {B[k][1]:.2f}% | |")
            L.append("")
    text = "\n".join(L) + "\n"
    if args.out:
        with open(args.out, "w") as f:
            f.write(text)
    else:
        print(text)


if __name__ == "__main__":
    main()
