#!/usr/bin/env python3
"""Rename BitPrairie identifiers in a historical camera-firmware tree
to match current develop (steelhead / moqXAIO), without changing
NVS namespace bp_xiao_ap or bp_/BP_ C prefixes."""
from __future__ import annotations

import os
import sys
from pathlib import Path

# Longer / more specific first.
REPLACEMENTS = [
    ("bitprairie.camera_event.v1", "moqxaio.camera_event.v1"),
    ('"project":"bitprairie-cam"', '"project":"moqxaio"'),
    ("bitprairie: ap_identity", "moqxaio: ap_identity"),
    ("BitPrairie camera", "moqXAIO camera"),
    ('"BitPrairie-"', '"moqXAIO-"'),
    ("BitPrairie-", "moqXAIO-"),
    ("physical BitPrairie board", "physical moqXAIO board"),
    ("#include \"bitprairie.h\"", "#include \"steelhead.h\""),
    ("BITPRAIRIE_H", "STEELHEAD_H"),
    ("bitprairie_core", "steelhead_core"),
    ("bitprairie_cam", "steelhead_cam"),
    ("bitprairie-cam", "steelhead-cam"),
    ("bitprairie-mark", "steelhead-mark"),
    ("CONFIG_BITPRAIRIE", "CONFIG_STEELHEAD"),
    ("BITPRAIRIE_", "STEELHEAD_"),
    ("BITPRAIRIE", "STEELHEAD"),
    ("BitPrairie", "moqXAIO"),
    ("bitprairie: ", "steelhead: "),
    ("bitprairie", "steelhead"),
    ("BITPrairie", "STEELHEAD"),  # just in case
]

SKIP_SUFFIXES = {
    ".bin", ".elf", ".o", ".obj", ".a", ".so", ".dylib", ".png", ".jpg",
    ".jpeg", ".webp", ".gif", ".ico", ".pdf", ".woff", ".woff2", ".ttf",
    ".1bit", ".mp4", ".zip", ".gz", ".tgz", ".bundle",
}

# .inc files are C arrays (text). Process them.

def is_probably_text(path: Path) -> bool:
    if path.suffix.lower() in SKIP_SUFFIXES:
        return False
    try:
        data = path.read_bytes()
    except OSError:
        return False
    if b"\0" in data[:4096]:
        return False
    return True


def replace_text(s: str) -> str:
    for old, new in REPLACEMENTS:
        s = s.replace(old, new)
    return s


def walk_files(root: Path):
    for dirpath, dirnames, filenames in os.walk(root):
        # skip git and build dirs
        dirnames[:] = [d for d in dirnames if d not in {".git", "build-xiao", "build-unset-s3", "target", "node_modules"}]
        for name in filenames:
            yield Path(dirpath) / name


def rename_paths(root: Path) -> list[tuple[Path, Path]]:
    """Rename files and directories whose names contain bitprairie (case-insensitive). Depth-first for dirs."""
    renamed = []
    # files first (deepest first), then dirs
    all_paths = []
    for dirpath, dirnames, filenames in os.walk(root, topdown=False):
        pdir = Path(dirpath)
        if ".git" in pdir.parts:
            continue
        for name in filenames:
            all_paths.append(pdir / name)
        if pdir != root:
            all_paths.append(pdir)
    for path in all_paths:
        new_name = replace_text(path.name)
        # also handle BitPrairie / BITPRAIRIE in filenames via case variants already in replace_text
        if new_name != path.name:
            dest = path.with_name(new_name)
            dest.parent.mkdir(parents=True, exist_ok=True)
            path.rename(dest)
            renamed.append((path, dest))
    return renamed


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: debrand-camera-firmware.py ROOT", file=sys.stderr)
        return 2
    root = Path(sys.argv[1]).resolve()
    cam = root / "camera-firmware"
    if not cam.is_dir():
        print(f"no camera-firmware in {root}", file=sys.stderr)
        return 2
    changed_files = 0
    for path in walk_files(cam):
        if not is_probably_text(path):
            continue
        raw = path.read_bytes()
        # try utf-8 then latin-1
        for enc in ("utf-8", "latin-1"):
            try:
                text = raw.decode(enc)
                break
            except UnicodeDecodeError:
                text = None
        else:
            continue
        new = replace_text(text)
        if new != text:
            path.write_text(new, encoding=enc)
            changed_files += 1
    renamed = rename_paths(cam)
    print(f"updated_files={changed_files} renamed={len(renamed)}")
    for a, b in renamed:
        print(f"  rename {a.relative_to(root)} -> {b.relative_to(root)}")
    # leftover prairie under camera-firmware?
    leftovers = []
    for path in walk_files(cam):
        if not is_probably_text(path):
            # still check strings-like ascii
            try:
                data = path.read_bytes()
            except OSError:
                continue
            if b"prairie" in data.lower():
                leftovers.append(str(path.relative_to(root)) + " (binary)")
            continue
        try:
            text = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        if "prairie" in text.lower():
            leftovers.append(str(path.relative_to(root)))
    if leftovers:
        print("LEFTOVER prairie:")
        for x in leftovers:
            print(" ", x)
        return 1
    print("no leftover prairie in camera-firmware text")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
