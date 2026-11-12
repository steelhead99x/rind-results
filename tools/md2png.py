#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Screenshots a Markdown file (or the part from one heading to the next
heading of the same level) as GitHub-like HTML in headless Chrome.

Usage: tools/md2png.py IN.md OUT.png [--section "Heading"] [--width 1200]
Needs python-markdown, Pillow and google-chrome (or CHROME=...).
"""
import argparse, os, re, subprocess, tempfile
import markdown
from PIL import Image, ImageChops

p = argparse.ArgumentParser()
p.add_argument("src"); p.add_argument("out")
p.add_argument("--section"); p.add_argument("--width", type=int, default=1200)
a = p.parse_args()
text = open(a.src).read()
if a.section:
    m = re.search(rf"^(#+) {re.escape(a.section)}\s*$", text, re.M)
    level = len(m.group(1))
    rest = text[m.start():]
    end = re.search(rf"^#{{1,{level}}} ", rest[m.end() - m.start():], re.M)
    text = rest[: m.end() - m.start() + end.start()] if end else rest
body = markdown.markdown(text, extensions=["tables", "fenced_code"])
css = """body{font:14px -apple-system,Segoe UI,Helvetica,Arial,sans-serif;color:#1f2328;margin:24px;max-width:%dpx}
table{border-collapse:collapse;margin:12px 0}th,td{border:1px solid #d1d9e0;padding:4px 10px}
tr:nth-child(2n){background:#f6f8fa}code{background:#eff1f3;padding:1px 4px;border-radius:4px;font-size:90%%}
pre{background:#f6f8fa;padding:12px;overflow:auto}h1,h2{border-bottom:1px solid #d1d9e0;padding-bottom:4px}""" % (a.width - 48)
with tempfile.TemporaryDirectory() as d:
    html = os.path.join(d, "page.html")
    open(html, "w").write(f"<!doctype html><meta charset=utf-8><style>{css}</style>{body}")
    chrome = os.environ.get("CHROME", "google-chrome")
    # Measure the page height first, then shoot at that height.
    # Shoot tall, then crop the blank space below the content.
    height = min(16000, 200 + 40 * (body.count("<tr") + body.count("<li") + body.count("<p") + body.count("<h") + body.count("\n")))
    subprocess.run([chrome, "--headless=new", "--disable-gpu", "--no-sandbox", "--hide-scrollbars",
                    f"--user-data-dir={d}/p", f"--window-size={a.width},{height}",
                    f"--screenshot={os.path.abspath(a.out)}", f"file://{html}"],
                   check=True, capture_output=True)
    img = Image.open(a.out).convert("RGB")
    box = ImageChops.difference(img, Image.new("RGB", img.size, (255, 255, 255))).getbbox()
    if box:
        img.crop((0, 0, img.width, min(img.height, box[3] + 24))).save(a.out, optimize=True)
