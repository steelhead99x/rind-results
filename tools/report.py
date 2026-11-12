#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Turns captured board runs into rind's report shape and RESULTS.md.

Usage:
  tools/report.py --raw results/raw --rind ../rind-bench/rind \
      --firmware ../rind-bench/esp32 [--sizes results/sizes.json] [--out results]
  tools/report.py --board nrf52840 --raw results/raw-nrf52840 --rind ../rind-bench/rind \
      --firmware ../rind-bench/nrf52840 --gaps results/nrf52840/gaps.toml --out results/nrf52840

It imports rind's own `admin/bench-report` for its OPERATIONS, SECTIONS,
LIBRARIES, and time formatting, so the IDs, library names, and sections are
the report's. It writes:

- `<out>/<board>.json` (`esp32s3.json`, `nrf52840.json`): one host in `admin/bench-report`'s schema 1, with
  `results` for every report operation. `ns` is the median over runs of each
  run's median; `ns_low` and `ns_high` are the lowest and highest run medians,
  so they show run-to-run spread, not criterion's confidence interval.
- `<out>/<board>-all.json`: every benchmark the firmware timed, report
  operation or not, with each run's statistics.
- `<out>/RESULTS.md`: the tables.
"""

import argparse
import datetime
import importlib.machinery
import importlib.util
import json
import os
import re
import statistics
import subprocess
import sys
import tomllib


def load_bench_report(rind):
    path = os.path.join(rind, "admin", "bench-report")
    loader = importlib.machinery.SourceFileLoader("bench_report", path)
    spec = importlib.util.spec_from_loader("bench_report", loader)
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


# What differs between the boards; the rest of the report is the same.
BOARDS = {
    "esp32s3": {
        "title": "rind on an ESP32-S3",
        "model": "Espressif ESP32-S3 (Xtensa LX7), 240 MHz",
        "arch": "xtensa",
        "os": lambda start: f"ESP-IDF {start['idf'] if start else '?'} (FreeRTOS), bare board",
        "criterion": "rind-bench-criterion-esp: CCOUNT cycles, 0.2 s warm-up, 2 ms samples, 11-51 samples per benchmark",
        "note": "One core of an ESP32-S3 at 240 MHz, timed with its cycle counter on core 1; no radio, PSRAM off.",
        "libs": ["rind", "ring", "rustcrypto", "libcrux", "esp32s3-hw"],
        "intro": "rind-bench's own benches, run unchanged but for the aws-lc-rs rows on one core of an ESP32-S3 "
        "at 240 MHz, timed with the core's cycle counter.",
        "layout": "Relinking an image moves its short-input rows far more than that: see `diag/layout-relink-2026-11-12/`, "
        "and read short-input ratios with the README's layout caveat.",
        "repro": [
            "git clone --recursive https://github.com/steelhead99x/rind-bench   # private; the port",
            "git clone https://github.com/steelhead99x/rind-results",
            "rind-results/tools/setup-toolchain.sh rind-bench   # espup, espflash, ldproxy, at these versions",
            "cd rind-bench && scripts/apply-patches.sh && scripts/vendor-ring.sh",
            "(cd esp32 && ./build-images.sh)   # images/<bench>.elf; check sha256 against images.sha256",
            "cd ../rind-results && PORT=/dev/serial/by-id/usb-Espressif_... tools/run-all.sh ../rind-bench/esp32/images",
            "python3 tools/report.py --raw results/raw --rind ../rind-bench/rind --firmware ../rind-bench/esp32 "
            "--sizes results/sizes.json --images results/images.json --meta results/meta-esp32s3.json --out results",
            "python3 tools/verify.py   # recomputes every published number from raw",
        ],
        "legend": "`esp32s3-hw` is ESP-IDF's mbedTLS on the chip's AES and SHA accelerators: hardware, not a Rust "
        "library. rind runs its portable Rust backend here (it has no Xtensa assembly).",
    },
    "nrf52840": {
        "title": "rind on an nRF52840 (Seeed T1000-E)",
        "model": "Nordic nRF52840 (Arm Cortex-M4F), 64 MHz",
        "arch": "arm",
        "os": lambda start: "none (bare metal, no_std); the board's UF2 bootloader and S140 SoftDevice stay in "
        "flash, unused",
        "criterion": "rind-bench-criterion-esp (no_std): DWT_CYCCNT cycles, 0.2 s warm-up, 2 ms samples, "
        "11-51 samples per benchmark",
        "note": "An nRF52840 at 64 MHz with the flash instruction cache on, timed with the DWT cycle counter; "
        "USB only between benchmarks, no radio.",
        "libs": ["rind", "ring", "rustcrypto", "libcrux"],
        "intro": "rind-bench's own benches, run unchanged but for the aws-lc-rs rows, on the nRF52840 of a Seeed "
        "T1000-E at 64 MHz, timed with the Cortex-M4's DWT cycle counter.",
        "layout": "No relinked build has been run on this board, so layout sensitivity here is unmeasured.",
        "repro": [
            "git clone --recursive https://github.com/steelhead99x/rind-bench && cd rind-bench",
            "scripts/apply-patches.sh && scripts/vendor-ring.sh && (cd nrf52840 && ./build-images.sh)",
            "cd ../rind-results && tools/run-all-nrf.sh ../rind-bench/nrf52840/images   # double-tap reset for the "
            "UF2 drive; back up the board's firmware first",
            "python3 tools/report.py --board nrf52840 --raw results/raw-nrf52840 --firmware ../rind-bench/nrf52840 "
            "--gaps results/nrf52840/gaps.toml --images results/nrf52840/images.json --out results/nrf52840",
            "python3 tools/verify.py",
        ],
        "legend": "rind runs its portable Rust backend here (its 32-bit Arm assembly needs ARM state and NEON, "
        "which an M-profile core lacks); ring keeps its Thumb-2 assembly for SHA-2, AES and GHASH. There is no "
        "hardware baseline: the chip's CryptoCell CC310 needs Nordic's closed library.",
    },
}


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
            # The flash clock the bootloader reports; espflash sets it in the image header.
            log = os.path.join(d, f"run-{r['run']}.log")
            if os.path.isfile(log):
                with open(log, errors="replace") as f:
                    m = re.search(r"Boot SPI Speed\s*:\s*(\d+)MHz", f.read())
                r["flash_mhz"] = int(m.group(1)) if m else None
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
    ap.add_argument("--board", choices=sorted(BOARDS), default="esp32s3")
    ap.add_argument("--raw", default="results/raw")
    ap.add_argument("--rind", required=True)
    ap.add_argument("--firmware", required=True)
    ap.add_argument("--sizes")
    ap.add_argument("--meta", help="JSON with toolchain and board facts")
    ap.add_argument("--gaps", default="results/gaps.toml", help="decisions on gaps over 10%%, as rind's gaps.toml")
    ap.add_argument("--out", default="results")
    ap.add_argument("--images", help="each image's provenance (sha256, rind and rind-bench commits, build time), "
                    "JSON; without it the rind and rind-bench checkouts' HEADs are reported")
    args = ap.parse_args()

    board = BOARDS[args.board]
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
    clocks, timed = [], []
    for r in ok_runs:
        for rec in r["records"]:
            t = rec.get("type")
            if t == "clock":
                clocks.append(rec["cycles"] * 1e9 / rec["wall_ns"])
            if t == "result" and rec.get("wall_ns"):
                # The share of the sampling window the counter timed, at the
                # nominal clock: under 1 by the setup and bookkeeping between
                # samples, and never over 1 unless the counter is wrong.
                timed.append((rec["sampled_cycles"], rec["wall_ns"]))
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
        "host": args.board,
        "commit": commit,
        "upstream_commit": git(args.rind, "merge-base", "HEAD", "origin/main"),
        "started": ok_runs[0]["started"] if ok_runs else None,
        "finished": ok_runs[-1]["finished"] if ok_runs else None,
        "cpu": {
            "model": board["model"],
            "arch": board["arch"],
            "logical_cpus": 1,
        },
        "os": board["os"](start),
        "toolchain": meta.get("toolchain", {}),
        "libraries": lock_versions(args.firmware, br),
        "rind_cpu": {"backend": "portable", "has_aes_hardware": False},
        "status": "ok" if len(ok_runs) == len(runs) else "partial",
        "criterion_args": [board["criterion"]],
        "load": {"runs": len(runs), "ok_runs": len(ok_runs)},
        "note": meta.get("note", board["note"]),
        "port": port_of(args.firmware),
        "results": results,
    }
    if args.images:
        with open(args.images) as f:
            prov = json.load(f)
        host["port"]["images"] = prov["images"]
        # The patches the images were built with, not those in the tree now.
        host["port"]["patches"] = prov.get("patches", host["port"]["patches"])
        ext = prov.get("file_ext", ".elf")
        commits = sorted({i["rind"] for i in host["port"]["images"].values()})
        host["commit"] = host["upstream_commit"] = commits[0] if len(commits) == 1 else commits
    flash = sorted({r.get("flash_mhz") for r in runs if r.get("flash_mhz")})
    if flash:
        host["flash_mhz"] = flash
    if clocks:
        host["clock_check"] = {"checks": len(clocks), "min_hz": min(clocks), "max_hz": max(clocks)}
    timed = [c * 1e9 / start["cpu_hz"] / w for c, w in timed]
    if timed:
        host["clock_check"] = dict(host.get("clock_check", {}), results=len(timed), timed_min=min(timed),
                                   timed_max=max(timed), over=sum(1 for x in timed if x > 1.001))
    os.makedirs(args.out, exist_ok=True)
    if args.images:
        # images.sha256 (sha256sum format) may list more files than ran (ELFs beside UF2s): write it
        # when missing, else require every image that ran to be in it.
        path = os.path.join(args.out, "images.sha256")
        if os.path.exists(path):
            with open(path) as f:
                listed = {line.split()[0] for line in f if line.strip()}
            missing = [n for n, i in host["port"]["images"].items() if i["sha256"] not in listed]
            if missing:
                sys.exit(f"{path} lacks the images that ran: {', '.join(sorted(missing))}")
        else:
            with open(path, "w") as f:
                for name, i in sorted(host["port"]["images"].items()):
                    f.write(f"{i['sha256']}  images/{name}{ext}\n")
    with open(os.path.join(args.out, f"{args.board}.json"), "w") as f:
        json.dump(host, f, indent=1)
        f.write("\n")
    with open(os.path.join(args.out, f"{args.board}-all.json"), "w") as f:
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
    write_markdown(args.out, br, host, everything, checks, skipped, runs, sizes, meta, gaps, board)


def harness_lines(host, everything, board):
    """The harness's own cost and its clock check, when the firmware recorded them."""
    out = []
    empty = {k.split("/")[-1]: v for k, v in everything.items() if k.startswith("harness/empty/")}
    cc = host.get("clock_check")
    if not empty and not cc:
        return out
    out += ["## Harness checks", ""]
    if cc and "checks" in cc:
        out.append(f"- Cycle counter against an independent wall clock (ESP-IDF's SYSTIMER, from the 40 MHz "
                   f"crystal) over a busy second at each boot ({cc['checks']} boots): "
                   f"{cc['min_hz'] / 1e6:.4f} to {cc['max_hz'] / 1e6:.4f} MHz.")
    if cc and "results" in cc:
        out.append(f"- Over each benchmark's samples ({cc['results']} results), the counter timed "
                   f"{100 * cc['timed_min']:.1f}% to {100 * cc['timed_max']:.2f}% of the wall-clock window (the rest is "
                   f"setup and bookkeeping between samples); {cc['over']} results exceed 100.1%, which a wrapped, "
                   "skipped or double-counted interval would show.")
    for k, v in sorted(empty.items()):
        out.append(f"- An empty `{k}` routine costs {v['cycles']:.1f} cycles per iteration (median over "
                   f"{v['runs']} runs): the loop, `black_box` and, for `iter_batched`, the moved input. It is "
                   "in every result, unsubtracted, as in criterion; against the fastest row here it is "
                   f"{100 * v['cycles'] / min(e['cycles'] for kk, e in everything.items() if not kk.startswith('harness/')):.2f}%.")
    out.append("")
    return out


def images_lines(host):
    imgs = host["port"].get("images")
    if not imgs:
        return [f"- Firmware: [rind-bench]({host['port']['repo']}) `{host['port']['commit']}`"]
    out = ["- Firmware images (one per bench file; `images.sha256` beside this file):", "",
           "  | Image | sha256 | rind | rind-bench | Built |", "  |---|---|---|---|---|"]
    for name, i in sorted(imgs.items()):
        out.append(f"  | {name} | `{i['sha256'][:16]}…` | `{i['rind'][:8]}` | {i['rind_bench']} | {i['built']} |")
    out.append("")
    return out


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


def write_markdown(out, br, host, everything, checks, skipped, runs, sizes, meta, gaps, board):
    libs = board["libs"]
    by = {}
    for r in host["results"]:
        by[(r["operation"], r["library"])] = r
    lines = []
    w = lines.append
    w(f"# {board['title']}")
    w("")
    w(board["intro"] + " Every operation and ID is `admin/bench-report`'s.")
    w("")
    up = host["upstream_commit"]
    up = ", ".join(f"`{c}`" for c in up) + " (per image, below)" if isinstance(up, list) else f"`{up}`"
    w(f"- rind: upstream {up}, plus the port's patches: "
      + ", ".join(f"`{p}`" for p in host["port"]["patches"]))
    for line in images_lines(host):
        w(line)
    w(f"- OS: {host['os']}")
    if host.get("flash_mhz"):
        w("- SPI flash clock, as the bootloader reported it each boot: "
          + ", ".join(f"{m} MHz" for m in host["flash_mhz"])
          + (" (espflash's default; see the README)" if host["flash_mhz"] == [40] else ""))
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
      + board["legend"])
    w("")
    for line in harness_lines(host, everything, board):
        w(line)
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
    w("## Confidence")
    w("")
    n = max((everything[r["id"]]["runs"] for r in host["results"]), default=0)
    if n >= 2:
        w(f"Each run is a fresh boot of the same image. With {n} runs the lowest and highest run medians "
          f"bound the board's median with probability {100 * (1 - 2 ** (1 - n)):.0f}% (distribution-free: "
          "each run median falls on either side with probability one half); `ns_low` and `ns_high` in "
          f"`{host['host']}.json` are that interval, and `RUNS.md` lists every run. That interval covers run-to-run "
          "noise only. " + board["layout"])
    w("")
    w("## Reproduce")
    w("")
    w("```sh")
    for line in board["repro"]:
        w(line)
    w("```")
    w("")
    notes = meta.get("markdown_notes")
    if notes:
        w(notes)
        w("")
    w(f"Generated by `tools/report.py` on {datetime.date.today().isoformat()}; "
      f"every benchmark, with each run's statistics, is in `{host['host']}-all.json`.")
    with open(os.path.join(out, "RESULTS.md"), "w") as f:
        f.write("\n".join(lines) + "\n")
    rl = [f"# Every run, {board['title'][len('rind on '):]}", "",
          "Each run's median cycles per call for every report row, the median over runs (published), and the "
          "spread, (max - min) / median. Generated by `tools/report.py`.", "",
          "| Benchmark | " + " | ".join(f"run {i + 1}" for i in range(n)) + " | median | spread |",
          "|---|" + "---:|" * (n + 2)]
    for r in sorted(host["results"], key=lambda r: r["id"]):
        e = everything[r["id"]]
        cells = [f"{x['cycles']['median']:,.0f}" for x in e["per_run"]] + [""] * (n - len(e["per_run"]))
        rl.append(f"| `{r['id']}` | " + " | ".join(cells) + f" | {e['cycles']:,.0f} | {e['run_spread_pct']:.2f}% |")
    with open(os.path.join(out, "RUNS.md"), "w") as f:
        f.write("<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->\n" + "\n".join(rl) + "\n")


if __name__ == "__main__":
    main()
