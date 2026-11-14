#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Capture Uno R3 bench serial, verify SHA digests, and print medians.

Pass the serial device as --port. This script does not hard-code a path.
"""
from __future__ import annotations
import argparse, hashlib, json, statistics, sys, time
from collections import defaultdict
from pathlib import Path

try:
    import serial
except ImportError:
    serial = None

BAUD = 115200

EXPECTED_SHA = {
    "sha256/32B": None,
    "sha256/1350B": None,
    "sha256/16KB": None,
    "sha512/32B": None,
    "sha512/1350B": None,
    "sha512/16KB": None,
}

def pattern_bytes(n: int) -> bytes:
    return bytes(i & 0xFF for i in range(n))

EXPECTED_SHA["sha256/32B"] = hashlib.sha256(pattern_bytes(32)).hexdigest()
EXPECTED_SHA["sha256/1350B"] = hashlib.sha256(pattern_bytes(1350)).hexdigest()
EXPECTED_SHA["sha256/16KB"] = hashlib.sha256(pattern_bytes(16 * 1024)).hexdigest()
EXPECTED_SHA["sha512/32B"] = hashlib.sha512(pattern_bytes(32)).hexdigest()
EXPECTED_SHA["sha512/1350B"] = hashlib.sha512(pattern_bytes(1350)).hexdigest()
EXPECTED_SHA["sha512/16KB"] = hashlib.sha512(pattern_bytes(16 * 1024)).hexdigest()


def parse_lines(text: str):
    kats, benches, notes = [], [], []
    for line in text.splitlines():
        line = line.strip()
        if line.startswith("KAT "):
            parts = line.split()
            if len(parts) >= 3:
                kats.append((parts[1], parts[2]))
        elif line.startswith("BENCH "):
            rec = {"raw": line}
            toks = line.split()
            rec["op"] = toks[1]
            for t in toks[2:]:
                if "=" in t:
                    k, v = t.split("=", 1)
                    rec[k] = int(v) if v.isdigit() else v
            benches.append(rec)
        elif line.startswith("SKIP ") or line.startswith("NOTE ") or line.startswith("SRC "):
            notes.append(line)
    return kats, benches, notes


def medians(benches):
    by = defaultdict(list)
    for b in benches:
        if "cycles" in b:
            by[b["op"]].append(b)
    out = {}
    for op, rows in by.items():
        cyc = [r["cycles"] for r in rows]
        us = [r["us"] for r in rows if "us" in r]
        out[op] = {
            "n": len(cyc),
            "cycles_all": cyc,
            "cycles_median": int(statistics.median(cyc)),
            "cycles_min": min(cyc),
            "cycles_max": max(cyc),
            "us_median": int(statistics.median(us)) if us else None,
            "us_all": us,
            "digests": [r.get("digest") for r in rows],
        }
    return out


def verify(med):
    problems = []
    for op, ev in EXPECTED_SHA.items():
        if op not in med:
            problems.append(f"missing {op}")
            continue
        for i, d in enumerate(med[op]["digests"]):
            if d != ev:
                problems.append(f"{op} run {i+1} digest {d} != {ev}")
    return problems


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", help="serial device (required unless --file is set)")
    ap.add_argument("--file", help="parse an existing capture instead of opening serial")
    ap.add_argument("--seconds", type=int, default=0, help="max seconds to wait (0=until DONE or EOF)")
    ap.add_argument("--out-dir", type=Path, default=Path("."), help="directory for the capture and medians JSON")
    args = ap.parse_args()
    out_dir = args.out_dir
    out_dir.mkdir(parents=True, exist_ok=True)
    raw_path = out_dir / "capture.log"

    if args.file:
        text = Path(args.file).read_text(errors="replace")
    else:
        if not args.port:
            sys.exit("pass --port <serial-device>, or --file <capture>")
        if serial is None:
            sys.exit("pyserial is not installed")
        buf = []
        with serial.Serial(args.port, BAUD, timeout=1) as ser:
            ser.dtr = False
            time.sleep(0.1)
            ser.dtr = True
            t0 = time.time()
            while True:
                line = ser.readline()
                if line:
                    s = line.decode("ascii", errors="replace")
                    sys.stdout.write(s)
                    sys.stdout.flush()
                    buf.append(s)
                    if s.strip() == "DONE":
                        break
                if args.seconds and time.time() - t0 > args.seconds:
                    buf.append("\nTIMEOUT\n")
                    break
        text = "".join(buf)
        raw_path.write_text(text)

    kats, benches, notes = parse_lines(text)
    med = medians(benches)
    problems = verify(med)
    report = {
        "board": "arduino-uno-r3",
        "mcu": "ATmega328P",
        "clock_hz": 16_000_000,
        "source": "rind generic @ c08c91ea (not the full rind crate)",
        "kats": kats,
        "notes": notes,
        "digest_problems": problems,
        "ops": med,
    }
    (out_dir / "medians.json").write_text(json.dumps(report, indent=2) + "\n")
    print("KATS", kats)
    print("DIGEST_PROBLEMS", problems or "none")
    for op, m in med.items():
        print(f"{op}: median {m['cycles_median']} cyc / {m['us_median']} us  n={m['n']} range {m['cycles_min']}-{m['cycles_max']}")


if __name__ == "__main__":
    main()
