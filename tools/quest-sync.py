#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Keep one GitHub issue per quests/q*.md in step with its frontmatter.

Usage: quest-sync.py [--dry-run]   (needs `gh` authenticated and GITHUB_REPOSITORY or a git remote)
"""
import glob, json, os, re, subprocess, sys

DRY = "--dry-run" in sys.argv
REPO = os.environ.get("GITHUB_REPOSITORY") or "steelhead99x/rind-results"
REF = os.environ.get("QUEST_REF", "main")
LABELS = {
    "quest": ("5319e7", "A quest from quests/"),
    "P0": ("b60205", "Security, correctness, or a blocking divergence"),
    "P1": ("d93f0b", "Performance and hardware-benchmarking gaps"),
    "P2": ("fbca04", "Portability, no_std, embedded, board CI"),
    "P3": ("0e8a16", "Docs, tests, toolchain drift, cleanup"),
    "size:S": ("c5def5", "Small"),
    "size:M": ("bfd4f2", "Medium"),
    "size:L": ("1d76db", "Large"),
    "needs:esp32-s3": ("f9d0c4", "Needs an ESP32-S3 board"),
    "needs:nrf52840": ("f9d0c4", "Needs an nRF52840 board"),
    "needs:rp2040": ("f9d0c4", "Needs an RP2040 board"),
    "needs:efr32mg24": ("f9d0c4", "Needs an EFR32MG24 board"),
    "needs:host": ("f9d0c4", "Needs a specific benchmark host"),
    "needs:none": ("ededed", "No hardware needed"),
}


def gh(*args, input=None):
    if DRY and args[0] in ("issue", "label") and args[1] in ("create", "edit", "close", "reopen"):
        print("dry-run: gh", " ".join(args))
        return ""
    r = subprocess.run(["gh", *args], input=input, capture_output=True, text=True)
    if r.returncode:
        sys.exit(f"gh {' '.join(args)} failed: {r.stderr.strip()}")
    return r.stdout


def parse(path):
    text = open(path, encoding="utf-8").read()
    m = re.match(r"---\n(.*?)\n---\n", text, re.S)
    if not m:
        sys.exit(f"{path}: no frontmatter")
    fm = {}
    for line in m.group(1).splitlines():
        k, _, v = line.partition(":")
        v = v.strip()
        if v.startswith("[") and v.endswith("]"):
            v = [x.strip() for x in v[1:-1].split(",") if x.strip()]
        elif v.startswith('"') and v.endswith('"'):
            v = v[1:-1].replace('\\"', '"')
        fm[k.strip()] = v
    for k in ("id", "title", "priority", "hardware", "size", "status", "claimable_by"):
        if k not in fm:
            sys.exit(f"{path}: frontmatter lacks {k}")
    if fm["priority"] not in ("P0", "P1", "P2", "P3") or fm["size"] not in ("S", "M", "L"):
        sys.exit(f"{path}: bad priority or size")
    return fm


def labels_for(fm):
    hw = fm["hardware"] if isinstance(fm["hardware"], list) else [fm["hardware"]]
    out = ["quest", fm["priority"], "size:" + fm["size"]]
    for h in hw:
        out.append("needs:host" if h.startswith("host:") else "needs:" + h)
    for l in out:
        if l not in LABELS:
            sys.exit(f"{fm['id']}: no label defined for {l}")
    return out


def body_for(fm, path):
    url = f"https://github.com/{REPO}/blob/{REF}/{path}"
    hw = fm["hardware"] if isinstance(fm["hardware"], list) else [fm["hardware"]]
    return (
        f"Quest file: [`{path}`]({url})\n\n"
        f"- Priority: {fm['priority']}\n- Size: {fm['size']}\n- Hardware: {', '.join(hw)}\n"
        f"- Status: {fm['status']}\n- Claimable by: {fm['claimable_by']}\n\n"
        "The quest file is the spec; this issue mirrors its frontmatter and is "
        "rewritten by `quest-sync` on every change. Quests are maintainer-only for "
        "now; comments and result PRs are welcome.\n"
    )


def main():
    have = {l["name"] for l in json.loads(gh("label", "list", "-R", REPO, "--limit", "200", "--json", "name"))}
    for name, (color, desc) in LABELS.items():
        if name not in have:
            gh("label", "create", name, "-R", REPO, "--color", color, "--description", desc)
    issues = json.loads(gh("issue", "list", "-R", REPO, "--label", "quest", "--state", "all",
                           "--limit", "500", "--json", "number,title,state,body,labels"))
    by_id = {}
    for i in issues:
        m = re.match(r"\[(Q\d+)\]", i["title"])
        if m:
            by_id[m.group(1)] = i
    for path in sorted(glob.glob("quests/q*.md")):
        fm = parse(path)
        title = f"[{fm['id']}] {fm['title']}"
        labels = labels_for(fm)
        body = body_for(fm, path)
        want_closed = fm["status"] in ("done", "dropped")
        cur = by_id.get(fm["id"])
        if cur is None:
            if want_closed:
                continue
            args = ["issue", "create", "-R", REPO, "--title", title, "--body-file", "-"]
            for l in labels:
                args += ["--label", l]
            print(gh(*args, input=body).strip() or f"would create {title}")
            continue
        n = str(cur["number"])
        old = {l["name"] for l in cur["labels"]}
        managed = set(LABELS)
        args = ["issue", "edit", n, "-R", REPO]
        if cur["title"] != title:
            args += ["--title", title]
        if (cur["body"] or "").strip() != body.strip():
            args += ["--body-file", "-"]
        for l in sorted((old & managed) - set(labels)):
            args += ["--remove-label", l]
        for l in sorted(set(labels) - old):
            args += ["--add-label", l]
        if len(args) > 5:
            gh(*args, input=body)
            print(f"updated #{n} {fm['id']}")
        if want_closed and cur["state"] == "OPEN":
            gh("issue", "close", n, "-R", REPO, "--reason", "completed" if fm["status"] == "done" else "not planned")
        elif not want_closed and cur["state"] == "CLOSED":
            gh("issue", "reopen", n, "-R", REPO)


main()
