# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
"""Print rind and every other library's `ns` for the headline operations of
each bench-report JSON given, exactly as the report records it (the median of
the per-round medians; three rounds in a Full run).

    python3 tools/headline.py m4/*.json a17pro/*.json
"""
import json
import sys

OPS = [
    "ed25519-verify/{lib}",
    "ed25519-sign/{lib}",
    "chacha20poly1305/{lib}/1350B",
    "mlkem768-keygen/{lib}",
    "mlkem768-encaps/{lib}",
    "mldsa65-sign/{lib}",
    "sha3-256/{lib}/1350B",
    "sha512/{lib}/1350B",
    "spake2plus-p256-prover-start/{lib}",
    "spake2plus-p256-verifier-start/{lib}",
    "spake2plus-p256-prover-finish/{lib}",
    "spake2plus-p256-verifier-finish/{lib}",
    "rustls-full-handshake-group/x25519/{lib}",
    "rustls-full-handshake-group/x25519mlkem768/{lib}",
]


def fmt(ns):
    return f"{ns / 1e3:.2f} µs" if ns >= 1e4 else f"{ns:.1f} ns"


for path in sys.argv[1:]:
    r = json.load(open(path))
    print(f"{path}: {r['host']}, {r['os']}, commit {r['commit'][:12]}, "
          f"{len(r['results'])} results, {len(r.get('errors') or [])} errors")
    for op in OPS:
        xs = sorted((x for x in r["results"] if x["operation"] == op),
                    key=lambda x: (x["library"] != "rind", x["ns"]))
        print(f"  {op.replace('/{lib}', '')}: "
              + ", ".join(f"{x['library']} {fmt(x['ns'])}" for x in xs))
    print()
