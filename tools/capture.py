#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Runs the rind-bench ESP32-S3 firmware and records what it prints.

Usage:
  tools/capture.py [--port <PORT>] [--runs 3] [--out results/raw] [--timeout 5400]

Each run resets the board through the USB Serial/JTAG port's RTS line, reads
until the firmware's `{"type":"done"}` line, and writes the run's console log
and its `RB:` JSON lines to `<out>/run-<n>.log` and `<out>/run-<n>.jsonl`. A
run that reboots, panics, or times out is recorded as failed, never retried
silently. Flash the firmware first; see rind-bench's README.
"""

import argparse
import datetime
import json
import os
import sys
import time

import serial  # pyserial

PANIC_MARKS = ("abort() was called", "Guru Meditation", "panicked at", "Rebooting...")


def reset(port):
    # The USB Serial/JTAG peripheral resets the chip when RTS is asserted with
    # DTR clear, as esptool's hard reset does.
    port.dtr = False
    port.rts = True
    time.sleep(0.1)
    port.rts = False


def run_once(path, timeout):
    lines, records = [], []
    status, reason = "timeout", f"no done line within {timeout} s"
    with serial.Serial(path, 115200, timeout=0.5) as port:
        port.reset_input_buffer()
        reset(port)
        started = time.monotonic()
        buf = b""
        booted = False
        while time.monotonic() - started < timeout:
            buf += port.read(4096)
            while b"\n" in buf:
                raw, buf = buf.split(b"\n", 1)
                line = raw.decode("utf-8", "replace").rstrip("\r")
                stamp = round(time.monotonic() - started, 3)
                lines.append(f"{stamp:10.3f} {line}")
                if line.startswith("RB:"):
                    rec = json.loads(line[3:])
                    records.append(rec)
                    if rec.get("type") == "start":
                        booted = True
                    if rec.get("type") == "done":
                        return lines, records, "ok", None
                elif booted and any(m in line for m in PANIC_MARKS):
                    status, reason = "failed", line.strip()
                    # Keep the backtrace that follows.
                    end = time.monotonic() + 2
                    while time.monotonic() < end:
                        buf += port.read(4096)
                    lines.extend(buf.decode("utf-8", "replace").splitlines())
                    return lines, records, status, reason
    return lines, records, status, reason


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--port", default="<PORT>")
    ap.add_argument("--runs", type=int, default=3)
    ap.add_argument("--out", default="results/raw")
    ap.add_argument("--timeout", type=int, default=5400)
    args = ap.parse_args()
    os.makedirs(args.out, exist_ok=True)
    summary = []
    for n in range(1, args.runs + 1):
        began = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
        print(f"run {n}: started {began}", file=sys.stderr)
        lines, records, status, reason = run_once(args.port, args.timeout)
        finished = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
        with open(os.path.join(args.out, f"run-{n}.log"), "w") as f:
            f.write("\n".join(lines) + "\n")
        with open(os.path.join(args.out, f"run-{n}.jsonl"), "w") as f:
            for r in records:
                f.write(json.dumps(r) + "\n")
        results = sum(1 for r in records if r.get("type") == "result")
        print(f"run {n}: {status} ({results} results) {reason or ''}", file=sys.stderr)
        summary.append(
            {"run": n, "started": began, "finished": finished, "status": status, "reason": reason, "results": results}
        )
    with open(os.path.join(args.out, "runs.json"), "w") as f:
        json.dump(summary, f, indent=2)
        f.write("\n")
    sys.exit(0 if all(s["status"] == "ok" for s in summary) else 1)


if __name__ == "__main__":
    main()
