<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Public release and security audit — 2026-11-14

Repository: `steelhead99x/rind-results`.
Reviewed release: [`641f5e0f5e1f08afd7193369433459186b43369f`](https://github.com/steelhead99x/rind-results/commit/641f5e0f5e1f08afd7193369433459186b43369f).
Candidate fixes: branch `codex/public-release-audit`.

The release has reproducible local-tooling and result-integrity defects that
should be fixed before tagging a final release. The candidate addresses them
without changing a published measurement, raw run, table, chart, presentation,
or vendored snapshot. No credential leaks were detected in the scanned tree
or reachable Git history. No vulnerability advisories matched the locked
registry versions; two informational maintenance advisories remain in the
RP2040 benchmark build workspace.

## Scope and method

- Inventoried 2,734 tracked paths at the reviewed commit, the five maintained
  Rust workspace crates, four Cargo lockfiles, two workflows, and local
  publication, rendering, download, command-execution, and cleanup boundaries.
- Reproduced hostile-metadata, symlink, output-overlap, and verifier attacks
  in disposable directories. Added nine regression tests. Existing published
  data served only as read-only fixtures or copies in test scratch space.
- Ran Gitleaks 8.30.1 with its default rules and redacted output on the tracked
  release export and all locally fetched Git refs, including merge changes.
  The history pass examined 186 commits with diffs and about 93.77 MB; the
  tracked-tree pass examined about 49.38 MB. The official scanner download
  was checked against its release SHA-256 manifest.
- Queried OSV for all 254 distinct crates.io package/version pairs in the
  tracked lockfiles, including the separate RP2040 workspace. The maintained
  workspace was also checked by cargo-deny 0.20.2. The checked-in
  [dependency inventory](dependencies.json) records versions, lockfiles,
  checksums, available workspace license metadata, and advisory IDs.
- Checked workflow syntax with actionlint 1.7.12 and confirmed that all five
  distinct action references resolve to commits in their named repositories.
  Reviewed triggers, token permissions, checkout credentials, and secret use.
- Classified privacy-pattern matches, checked all 77 PNGs for textual/EXIF
  metadata, inspected static page data flows, and spot-checked screenshot
  content. Ran ShellCheck 0.11.0 on all 33 tracked shell recipes.

## Findings addressed in the candidate

Severity describes local tooling and publication integrity. This repository
does not expose a web API that accepts these inputs.

| ID | Severity | Reproduced behavior at the reviewed release | Candidate change |
|---|---|---|---|
| A1 | Medium | Host IDs from capture JSON could become paths outside the selected staging/render/publish directory. A real renderer invocation created an SVG outside `--out`. | Restrict host IDs to safe single components in `stage-repo`, `render-side`, and `host-report`; independently confine renderer-returned file paths to `--out`. |
| A2 | Medium | `stage-repo` followed links in a render tree and existing staged destinations, allowing unintended local files to be copied or overwritten. | Refuse input-tree and destination symbolic links. Use a private staging directory; this is not isolation from concurrent hostile filesystem mutation. |
| A3 | Medium | A missing path component followed by a parent component bypassed `render-side`'s protection against writing inside the rind checkout. | Normalize parents throughout resolution and resolve existing prefixes before comparing output boundaries. |
| A4 | Medium | `render-ours --out` could delete its own results directory, host input directory, or their ancestor before reading inputs. | Reject overlapping source/output paths before cleanup. Normal report regeneration still passes the published byte-for-byte fixtures. |
| A5 | Medium | The independent verifier accepted a summary that duplicated one operation/library row and omitted another while keeping the same total row count. | Count unique operation/library pairs, reject duplicate summary rows, and detect missing unique results. The copied published fixture passes before mutation and fails after mutation. |
| A6 | Low | `md2png` disabled Chrome's sandbox and used a predictable shared temporary path for its page/profile. | Keep Chrome's sandbox and create a private, unique directory atomically, with cleanup on error and success. This tool still renders trusted HTML; it is not a sanitizer. |

Tests and fixes are in
[`tools/stage-repo`](../../tools/stage-repo/),
[`tools/render-side`](../../tools/render-side/),
[`tools/verify`](../../tools/verify/), and
[`tools/xtask`](../../tools/xtask/).

Release preparation also adds [SECURITY.md](../../SECURITY.md), credential
ignore rules, locked Cargo aliases/checks, and a safe-reuse notice for
[historical recipes](../../scripts/README.md). The root [LICENSE](../../LICENSE)
now accurately identifies the copied upstream snapshot and bundled font
notices instead of claiming no rind code is copied. Existing license texts
and third-party notices remain intact.

## Validation

| Check | Result |
|---|---|
| `cargo xtask ci`, Rust 1.98.1 | Passed: rustfmt, clippy with warnings denied, 115 tests passed, 1 pre-existing ignored test, all 13 verifier invocations reported zero mismatches, cargo-deny passed. |
| Nine new security regressions | Passed. Relevant counterexamples failed against the previous implementation before applying fixes. A parallel executable-fixture race exposed during the full run was fixed; its CI tests also passed ten repeated runs. |
| Secret scan, tracked base tree | Zero findings. |
| Secret scan, fetched Git history including merge diffs | Zero findings. |
| OSV, 254 locked registry versions | No vulnerability advisories matched; two informational unmaintained notices below. |
| actionlint and pinned commit resolution | Passed; all five immutable references resolved. |
| Privacy classification | Home paths and private IPs were test/example data; serial/MAC matches in publication artifacts were masked placeholders. No unredacted identifier was identified by these checks. |
| PNG metadata | No textual or EXIF chunks found in 77 PNGs. Pixel content was not exhaustively OCR-reviewed. |
| ShellCheck | No errors after declaring Bash for two sourced environment snippets; 33 warnings and 266 informational findings remain in historical recipes. They are not represented as hardened production automation. |

The staged candidate's secret scan and tracked-data immutability check also
passed before publication. Hosted CI is a backstop;
the owner's instruction was not to wait for it or introduce a required CI
merge gate.

## Remaining decisions and limits

1. **Merge the candidate fixes before final release.** The reviewed `main`
   commit still has the behaviors above; this report does not certify that
   unmodified commit as release-ready.
2. **RP2040 dependency maintenance.** `paste 1.0.15`
   ([RUSTSEC-2024-0436](https://rustsec.org/advisories/RUSTSEC-2024-0436.html))
   and `proc-macro-error2 2.0.1`
   ([RUSTSEC-2026-0173](https://rustsec.org/advisories/RUSTSEC-2026-0173.html))
   are unmaintained, with no patched versions identified by those notices.
   They occur only in `baremetal/rp2040/builds/Cargo.lock`, outside the
   maintained tooling workspace. Preserve the historical lock and its
   measurements; choose maintained dependencies for a future build and
   record that build as new evidence.
3. **Pullfrog activation is unverified.** Its pinned workflow is present on
   `main`, but the console requires GitHub sign-in and the local CLI has no
   GitHub OAuth token. App installation, Codex/provider connection, automatic
   review/re-review, and the console's custom review instructions cannot be
   confirmed until that authentication is completed. Repository files alone
   do not prove activation.
4. **Account settings need owner verification.** `main` was unprotected and
   no repository rulesets were returned. The connector did not expose the
   secret-scanning, push-protection, or private-vulnerability-reporting
   settings. Those settings were not changed. Confirm a usable private
   reporting route before treating the new security policy as operational.
5. **Trust and coverage.** Trusted upstream Python code and benchmark
   harnesses are intentionally executable. This review is not a proof of
   the rind cryptographic implementation, a hardware/firmware penetration
   test, or a complete audit of every transitive dependency's implementation.
   The actual Rust 1.85 toolchain/MSRV and all board toolchains were not run;
   dependency metadata was compatible with the declared 1.85 floor.
   No live board, device, governor, service, or account permission was changed.

## Reproduction

From the candidate checkout:

```sh
cargo xtask ci
cargo test -p stage-repo -p render-side --test security --locked
cargo test -p xtask -p verify --test security --locked
actionlint
gitleaks git --log-opts='--all --full-history --diff-merges=first-parent' --redact
```

Run OSV queries from the package/version list in `dependencies.json` using
the documented [batch API](https://google.github.io/osv.dev/post-v1-querybatch/).
Recheck advisories at the actual release date. Scanners can miss secrets or
unknown vulnerabilities; zero findings are the result of these checks, not
a guarantee of absence.
