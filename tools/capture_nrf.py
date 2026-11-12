#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Runs the rind-bench nRF52840 firmware and records what it prints.

Usage:
  tools/capture_nrf.py [--port PATH] [--runs 3] [--out results/raw-nrf52840/BENCH] [--timeout 7200]

The firmware enumerates as a USB CDC-ACM port ("rind-bench nRF52840") and
waits for the host to send `rind-bench:go` before it starts. Each run first
restarts it by opening the port at 300 baud and dropping DTR, waits for it to
come back and for ModemManager to finish probing the new port (it opens new
ports and would read some of the output), sends the go line, then reads until
the firmware's `{"type":"done"}` line. A run whose first record is not the
start record is marked failed, since that means another reader took lines. It writes each run's
console log and `RB:` JSON lines to `<out>/run-<n>.log` and `.jsonl`. A run
that panics or times out is recorded as failed, never retried silently. Flash
the image first; see rind-bench's nrf52840/README.md.
"""

import argparse
import datetime
import json
import os
import sys
import time

import serial  # pyserial

GO = b"rind-bench:go\n"
SETTLE_S = 15
DEFAULT_PORT = "/dev/serial/by-id/usb-rind-bench_rind-bench_nRF52840_rind-bench-if00"


def wait_for(path, present, timeout=20):
    end = time.monotonic() + timeout
    while time.monotonic() < end:
        if os.path.exists(path) == present:
            return True
        time.sleep(0.1)
    return False


def restart(path):
    # The firmware restarts when DTR drops on a port set to 300 baud.
    if not wait_for(path, True):
        sys.exit(f"{path} is not there; is the image flashed?")
    # It may restart as soon as the line coding arrives, before DTR rises,
    # so the port can vanish under us here.
    try:
        with serial.Serial(path, 300) as port:
            port.dtr = True
            time.sleep(0.1)
            port.dtr = False
    except (OSError, serial.SerialException):
        pass
    wait_for(path, False, 5)
    if not wait_for(path, True):
        sys.exit(f"{path} did not come back after the restart")
    time.sleep(SETTLE_S)


def run_once(path, timeout):
    lines, records = [], []
    status, reason = "timeout", f"no done line within {timeout} s"
    restart(path)
    with serial.Serial(path, 115200, timeout=0.5) as port:
        port.write(GO)
        started = time.monotonic()
        buf = b""
        while time.monotonic() - started < timeout:
            try:
                buf += port.read(4096)
            except serial.SerialException as e:
                return lines, records, "failed", f"port lost: {e}"
            while b"\n" in buf:
                raw, buf = buf.split(b"\n", 1)
                line = raw.decode("utf-8", "replace").rstrip("\r")
                stamp = round(time.monotonic() - started, 3)
                lines.append(f"{stamp:10.3f} {line}")
                if line.startswith("RB:"):
                    rec = json.loads(line[3:])
                    records.append(rec)
                    if records[0].get("type") != "start":
                        return lines, records, "failed", "the first record was not the start record"
                    if rec.get("type") == "done":
                        return lines, records, "ok", None
                    if rec.get("type") == "panic":
                        return lines, records, "failed", rec.get("message")
    return lines, records, status, reason


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--port", default=DEFAULT_PORT)
    ap.add_argument("--runs", type=int, default=3)
    ap.add_argument("--out", default="results/raw-nrf52840")
    ap.add_argument("--timeout", type=int, default=7200)
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
