#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Turns captured ESP32-S3 runs into rind's report shape and RESULTS.md.

Usage:
  tools/report.py --raw results/raw --rind ../rind-bench/rind \
      --firmware ../rind-bench/esp32 [--sizes results/sizes.json] [--out results]

It imports rind's own `admin/bench-report` for its OPERATIONS, SECTIONS,
LIBRARIES, and time formatting, so the IDs, library names, and sections are
the report's. It writes:

- `<out>/esp32s3.json`: one host in `admin/bench-report`'s schema 1, with
  `results` for every report operation. `ns` is the median over runs of each
  run's median; `ns_low` and `ns_high` are the lowest and highest run medians,
  so they show run-to-run spread, not criterion's confidence interval.
- `<out>/esp32s3-all.json`: every benchmark the firmware timed, report
  operation or not, with each run's statistics.
- `<out>/RESULTS.md`: the tables.
"""

import argparse
import datetime
import importlib.machinery
import importlib.util
import json
import os
import statistics
import subprocess
import tomllib


def load_bench_report(rind):
    path = os.path.join(rind, "admin", "bench-report")
    loader = importlib.machinery.SourceFileLoader("bench_report", path)
    spec = importlib.util.spec_from_loader("bench_report", loader)
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


def git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], capture_output=True, text=True, check=True).stdout.strip()


def read_runs(raw):
    """Every run under `raw`: one directory per firmware image, each holding
    capture.py's `runs.json` and `run-<n>.jsonl`."""
    out = []
    for image in sorted(os.listdir(raw)):
        d = os.path.join(raw, image)
        if not os.path.isfile(os.path.join(d, "runs.json")):
            continue
        with open(os.path.join(d, "runs.json")) as f:
            runs = json.load(f)
        for r in runs:
            with open(os.path.join(d, f"run-{r['run']}.jsonl")) as f:
                r["records"] = [json.loads(line) for line in f if line.strip()]
            r["image"] = image
            out.append(r)
    return out


def port_of(firmware):
    """The rind-bench commit the images came from and the patches applied."""
    root = os.path.dirname(os.path.abspath(firmware))
    patches = sorted(p for p in os.listdir(os.path.join(root, "patches")) if p.endswith(".patch"))
    return {
        "repo": "https://github.com/steelhead99x/rind-bench",
        "commit": git(root, "rev-parse", "HEAD"),
        "patches": patches,
    }


def lock_versions(firmware, br):
    with open(os.path.join(firmware, "Cargo.lock"), "rb") as f:
        lock = tomllib.load(f)
    have = {}
    for p in lock["package"]:
        have.setdefault(p["name"], []).append(p["version"])
    libs = {}
    for lib, crates in br.CRATES.items():
        found = {c: ", ".join(have[c]) for c in crates if c in have}
        if found:
            libs[lib] = found
    for c in ["esp-idf-svc", "esp-idf-sys", "esp-idf-hal"]:
        if c in have:
            libs.setdefault("esp-idf", {})[c] = ", ".join(have[c])
    return libs


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--raw", default="results/raw")
    ap.add_argument("--rind", required=True)
    ap.add_argument("--firmware", required=True)
    ap.add_argument("--sizes")
    ap.add_argument("--meta", help="JSON with toolchain and board facts")
    ap.add_argument("--gaps", default="results/gaps.toml", help="decisions on gaps over 10%%, as rind's gaps.toml")
    ap.add_argument("--out", default="results")
    args = ap.parse_args()

    br = load_bench_report(args.rind)
    runs = read_runs(args.raw)
    ok_runs = [r for r in runs if r["status"] == "ok"]
    meta = {}
    if args.meta:
        with open(args.meta) as f:
            meta = json.load(f)

    # id -> list of per-run result records
    per_id = {}
    checks = {}
    skipped = {}
    start = None
    for r in ok_runs:
        for rec in r["records"]:
            t = rec.get("type")
            if t == "result":
                per_id.setdefault(rec["id"], []).append(rec)
            elif t == "check":
                checks.setdefault((rec["check"], rec["library"]), []).append(rec["pass"])
            elif t == "skipped":
                skipped[rec["id"]] = rec.get("reason")
            elif t == "start":
                start = rec

    everything = {}
    results = []
    for full_id, recs in sorted(per_id.items()):
        medians = [x["ns"] for x in recs]
        agg = {
            "id": full_id,
            "ns": statistics.median(medians),
            "ns_low": min(medians),
            "ns_high": max(medians),
            "runs": len(recs),
            "run_spread_pct": 100 * (max(medians) - min(medians)) / statistics.median(medians),
            "throughput": recs[0]["throughput"],
            "cycles": statistics.median(x["cycles"]["median"] for x in recs),
            "cycles_per_byte": recs[0]["cycles_per_byte"] and statistics.median(x["cycles_per_byte"] for x in recs),
            "per_run": [
                {k: x[k] for k in ("ns", "ns_low", "ns_high", "cycles", "samples", "iters_per_sample")} for x in recs
            ],
        }
        everything[full_id] = agg
        hit = br.operation_of(full_id)
        if hit:
            op, row = hit
            results.append(
                {
                    "operation": op,
                    "library": br.LIBRARIES.get(row, row),
                    "id": full_id,
                    "ns": agg["ns"],
                    "ns_low": agg["ns_low"],
                    "ns_high": agg["ns_high"],
                    "throughput": agg["throughput"],
                    "cycles": agg["cycles"],
                    "cycles_per_byte": agg["cycles_per_byte"],
                    "run_spread_pct": agg["run_spread_pct"],
                }
            )

    commit = git(args.rind, "rev-parse", "HEAD")
    host = {
        "schema": 1,
        "host": "esp32s3",
        "commit": commit,
        "upstream_commit": git(args.rind, "merge-base", "HEAD", "origin/main"),
        "started": ok_runs[0]["started"] if ok_runs else None,
        "finished": ok_runs[-1]["finished"] if ok_runs else None,
        "cpu": {
            "model": "Espressif ESP32-S3 (Xtensa LX7), 240 MHz",
            "arch": "xtensa",
            "logical_cpus": 1,
        },
        "os": f"ESP-IDF {start['idf'] if start else '?'} (FreeRTOS), bare board",
        "toolchain": meta.get("toolchain", {}),
        "libraries": lock_versions(args.firmware, br),
        "rind_cpu": {"backend": "portable", "has_aes_hardware": False},
        "status": "ok" if len(ok_runs) == len(runs) else "partial",
        "criterion_args": [
            "rind-bench-criterion-esp: CCOUNT cycles, 0.2 s warm-up, 2 ms samples, 11-51 samples per benchmark"
        ],
        "load": {"runs": len(runs), "ok_runs": len(ok_runs)},
        "note": meta.get(
            "note",
            "One core of an ESP32-S3 at 240 MHz, timed with its cycle counter on core 1; no radio, PSRAM off.",
        ),
        "port": port_of(args.firmware),
        "results": results,
    }
    os.makedirs(args.out, exist_ok=True)
    with open(os.path.join(args.out, "esp32s3.json"), "w") as f:
        json.dump(host, f, indent=1)
        f.write("\n")
    with open(os.path.join(args.out, "esp32s3-all.json"), "w") as f:
        json.dump(
            {
                "runs": [{k: v for k, v in r.items() if k != "records"} for r in runs],
                "checks": [
                    {"check": c, "library": lib, "pass": all(p), "runs": len(p)} for (c, lib), p in sorted(checks.items())
                ],
                "skipped": skipped,
                "benchmarks": everything,
            },
            f,
            indent=1,
        )
        f.write("\n")

    sizes = None
    if args.sizes and os.path.exists(args.sizes):
        with open(args.sizes) as f:
            sizes = json.load(f)
    gaps = load_gaps(br, host, args.gaps)
    write_markdown(args.out, br, host, everything, checks, skipped, runs, sizes, meta, gaps)


# The libraries a gap is measured against: the other Rust libraries, as in
# rind's gaps.toml. The hardware baseline is never a competitor.
RIVALS = ["ring", "rustcrypto", "libcrux", "graviola", "aws-lc-rs"]


def gaps_of(br, host):
    """Each operation where another Rust library beats rind by more than
    admin/bench-report's SLOWER (1.1), as {operation: (ratio, library)}."""
    by = {}
    for r in host["results"]:
        by.setdefault(r["operation"], {})[r["library"]] = r["ns"]
    out = {}
    for op, libs in by.items():
        others = {lib: ns for lib, ns in libs.items() if lib in RIVALS}
        if "rind" in libs and others:
            best = min(others, key=others.get)
            if libs["rind"] / others[best] > br.SLOWER:
                out[op] = (libs["rind"] / others[best], best)
    return out


def load_gaps(br, host, path):
    """rind's gaps.toml rule on this host: every gap needs an entry with a
    quest or a reason, and every entry must cover a gap."""
    import fnmatch

    found = gaps_of(br, host)
    entries = []
    if os.path.exists(path):
        with open(path, "rb") as f:
            entries = tomllib.load(f).get("gap", [])
    out, problems = [], []
    for i, e in enumerate(entries, 1):
        if set(e) - {"operations", "on", "quest", "reason"} or not {"operations", "on"} <= set(e):
            problems.append(f"entry {i}: needs operations and on, and no other keys but quest or reason")
        if ("quest" in e) == ("reason" in e):
            problems.append(f"entry {i}: needs a quest or a reason, not both")
        if host["host"] not in e.get("on", []):
            continue
        covered = {op: v for op, v in found.items() if any(fnmatch.fnmatchcase(op, g) for g in e["operations"])}
        if not covered:
            problems.append(f"entry {i} ({', '.join(e['operations'])}): names no gap over 10%; update or remove it")
        for op in covered:
            del found[op]
        out.append((e, covered))
    if found:
        problems.append(
            "gaps over 10% without a decision: "
            + ", ".join(f"{op} ({ratio:.2f}x {lib})" for op, (ratio, lib) in sorted(found.items()))
        )
    if problems:
        raise SystemExit(f"{path}:\n  " + "\n  ".join(problems))
    return out


def cell(br, r):
    if r is None:
        return "-"
    s = br.human(r["ns"])
    if r.get("cycles_per_byte"):
        s += f" ({r['cycles_per_byte']:.1f} c/B)"
    else:
        s += f" ({r['cycles'] / 1e3:,.0f} kc)" if r["cycles"] >= 1e4 else f" ({r['cycles']:,.0f} c)"
    return s


def write_markdown(out, br, host, everything, checks, skipped, runs, sizes, meta, gaps):
    libs = ["rind", "ring", "rustcrypto", "libcrux", "esp32s3-hw"]
    by = {}
    for r in host["results"]:
        by[(r["operation"], r["library"])] = r
    lines = []
    w = lines.append
    w("# rind on an ESP32-S3")
    w("")
    w(
        "rind-bench's own benches, run unchanged but for the aws-lc-rs rows on one core of an ESP32-S3 "
        "at 240 MHz, timed with the core's cycle counter. Every operation and ID is `admin/bench-report`'s."
    )
    w("")
    w(f"- rind: upstream `{host['upstream_commit']}`, plus the port's patches: "
      + ", ".join(f"`{p}`" for p in host["port"]["patches"]))
    w(f"- Firmware: [rind-bench]({host['port']['repo']}) `{host['port']['commit']}`")
    w(f"- OS: {host['os']}")
    tc = host.get("toolchain") or {}
    if tc:
        w(f"- Toolchain: {tc.get('rustc', '?')}; {tc.get('gcc', '')}")
    w(f"- Runs: {sum(1 for r in runs if r['status'] == 'ok')} of {len(runs)} completed, "
      "one firmware image per rind-bench bench file")
    for r in runs:
        w(f"  - {r['image']} run {r['run']}: {r['status']}, {r['results']} results, {r['started']} to {r['finished']} (UTC)"
          + (f", {r['reason']}" if r.get("reason") else ""))
    w("")
    w("Each cell is the median over runs of each run's median time per operation, with cycles per byte for "
      "byte-throughput rows or cycles (c) / kilocycles (kc) per operation otherwise. "
      "`esp32s3-hw` is ESP-IDF's mbedTLS on the chip's AES and SHA accelerators: hardware, not a Rust library. "
      "rind runs its portable Rust backend here (it has no Xtensa assembly).")
    w("")
    w("## Headline: rind against the fastest other Rust library")
    w("")
    w("| Operation | rind | fastest other Rust library | rind's time over theirs | run-to-run spread (rind) |")
    w("|---|---|---|---|---|")
    for _, ops in br.SECTIONS:
        for op in ops:
            ours = by.get((op, "rind"))
            others = [(lib, by[(op, lib)]) for lib in libs[1:4] if (op, lib) in by]
            if not ours or not others:
                continue
            lib, best = min(others, key=lambda x: x[1]["ns"])
            ratio = ours["ns"] / best["ns"]
            w(f"| {op.replace('{lib}', '…')} | {cell(br, ours)} | {lib}: {cell(br, best)} | {ratio:.2f}× | {ours['run_spread_pct']:.1f}% |")
    w("")
    for section, ops in br.SECTIONS:
        rows = [op for op in ops if any((op, lib) in by for lib in libs)]
        if not rows:
            continue
        w(f"## {section}")
        w("")
        w("| Operation | " + " | ".join(libs) + " |")
        w("|---|" + "---|" * len(libs))
        for op in rows:
            w(f"| {op.replace('{lib}', '…')} | " + " | ".join(cell(br, by.get((op, lib))) for lib in libs) + " |")
        w("")
    w("## Known gaps")
    w("")
    w("Every operation where another Rust library beats rind by more than 10% here, with the decision on it, "
      "as rind's `benchmarks/gaps.toml` requires; `tools/report.py` refuses a gap without an entry in "
      "`results/gaps.toml`, and an entry that covers no gap.")
    w("")
    if gaps:
        w("| Operations | rind's time over the fastest | Fastest | Decision |")
        w("|---|---|---|---|")
        for e, covered in gaps:
            ratios = sorted(r for r, _ in covered.values())
            spread = f"{ratios[0]:.2f}×" + (f" to {ratios[-1]:.2f}×" if ratios[-1] - ratios[0] >= 0.005 else "")
            fastest = ", ".join(sorted({lib for _, lib in covered.values()}))
            decision = e.get("reason") or f"quest: {e['quest']}"
            ops = ", ".join(f"`{op.replace('{lib}', '…')}`" for op in sorted(covered))
            w(f"| {ops} | {spread} | {fastest} | {decision} |")
    else:
        w("None.")
    w("")
    w("## Known-answer checks")
    w("")
    w("Run on the board before any timing, every run. A library failing an operation's check is not raced there.")
    w("")
    w("| Check | Library | Result |")
    w("|---|---|---|")
    for (c, lib), passes in sorted(checks.items()):
        w(f"| {c} | {lib} | {'pass' if all(passes) else 'FAIL'} ({len(passes)} runs) |")
    w("")
    if skipped:
        w("Skipped benchmarks: " + ", ".join(f"`{k}` ({v})" for k, v in sorted(skipped.items())))
        w("")
    if sizes:
        w("## Code size")
        w("")
        w(sizes.get("note", ""))
        w("")
        w("| Library | Bytes of linked code and read-only data |")
        w("|---|---|")
        for lib, n in sorted(sizes["libraries"].items(), key=lambda x: -x[1]):
            w(f"| {lib} | {n:,} |")
        w("")
    notes = meta.get("markdown_notes")
    if notes:
        w(notes)
        w("")
    w(f"Generated by `tools/report.py` on {datetime.date.today().isoformat()}; "
      "every benchmark, with each run's statistics, is in `esp32s3-all.json`.")
    with open(os.path.join(out, "RESULTS.md"), "w") as f:
        f.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
