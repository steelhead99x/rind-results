#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Turns a `script --log-timing T -c CMD LOG` recording into an asciinema v2
cast (play with `asciinema play`, or on asciinema.org) and a plain text log,
replacing the home directory with `~` and dropping the user and host names.

Usage: tools/script2cast.py LOG TIMING OUT_STEM [TITLE]
"""
import json, os, pathlib, re, sys

log, timing, stem = sys.argv[1:4]
title = sys.argv[4] if len(sys.argv) > 4 else pathlib.Path(stem).name
data = open(log, "rb").read()
start = data.index(b"\n") + 1  # script's "Script started" header line
home = os.path.expanduser("~")
user = os.path.basename(home)
host = os.uname().nodename


def clean(s):
    s = s.replace(home, "~")
    s = re.sub(rf"\b{re.escape(user)}\b", "user", s)
    return re.sub(rf"\b{re.escape(host)}\b", "host", s)


events, t, pos = [], 0.0, start
for line in open(timing):
    parts = line.split()
    if len(parts) == 3 and parts[0] in "OI":  # advanced format: type delay bytes
        kind, delay, n = parts[0], float(parts[1]), int(parts[2])
    elif len(parts) == 2:  # classic format: delay bytes
        kind, delay, n = "O", float(parts[0]), int(parts[1])
    else:
        continue
    t += delay
    if kind == "O":
        chunk = data[pos : pos + n]
        pos += n
        events.append([round(t, 3), "o", clean(chunk.decode("utf-8", "replace"))])
with open(stem + ".cast", "w") as f:
    f.write(json.dumps({"version": 2, "width": 120, "height": 36, "title": title, "env": {"TERM": "xterm-256color"}}) + "\n")
    for e in events:
        f.write(json.dumps(e) + "\n")
text = data[start:].decode("utf-8", "replace")
text = text.rsplit("\nScript done", 1)[0]
with open(stem + ".txt", "w") as f:
    f.write(clean(text).replace("\r\n", "\n"))
