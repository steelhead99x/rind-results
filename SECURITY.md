<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Security

This repository publishes benchmark evidence and local tooling. Its timings,
test keys, and benchmark firmware are not a cryptographic service or a source
of production keys. Security defects in the rind library itself belong with
[upstream rind](https://github.com/moq-dev/rind/security).

## Reporting

For vulnerabilities in these tools, publication paths, or GitHub workflows,
use GitHub's private [Report a vulnerability](https://github.com/steelhead99x/rind-results/security/advisories/new)
flow when available. If private reporting is unavailable, use an established
private maintainer contact before sharing exploit details. Never put a live
credential or private capture in a public issue. Ordinary benchmark
discrepancies can be reported as issues with the measured commit and public
reproduction data.

The maintained tooling is on `main`. Historical measurements, toolchains,
and run recipes remain evidence of their recorded versions; they do not
promise that those versions still receive security updates.

## Trust boundaries

- Review and trust a `--rind` checkout before running it. The report and bench
  tools intentionally import its Python code and may build and execute its
  Rust harnesses. They are not sandboxes for arbitrary repositories.
- Host metadata is data, never an output path. Publishing commands refuse
  traversal in host IDs. `stage-repo` refuses symbolic links in copied render
  trees and staged destinations. Use a private staging directory; these
  checks do not isolate concurrent hostile processes sharing that directory.
- `md2png` renders trusted Markdown and embedded HTML in Chrome. It keeps
  Chrome's sandbox and creates a private temporary page/profile directory;
  it is not an HTML sanitizer. Do not run it as root or disable the browser
  sandbox to process an untrusted document.
- Hardware commands can reset or flash a board, replace local build trees,
  or change a governor. Historical [run recipes](scripts/README.md) need
  manual review and an isolated bench environment before reuse. The release
  audit does not run them on live hardware.
- Keep provider credentials in the provider's supported store or GitHub
  Actions secrets, never in captures, commits, screenshots, or reports.
  `stage-repo`'s identifier scrubber is not a general secret detector.

## Release checks

Run `cargo xtask ci` locally. The checked-in aliases and gate use the committed
Cargo lockfile. Scan the staged release and reachable Git history with a
secret scanner, including merge changes, and review any matches privately.
Preserve published measurements and their original provenance. Record new
measurements as a separate set rather than overwriting evidence.

Pullfrog account installation, provider authentication, and automatic review
settings are separate from committing its workflow. Verify them in the
Pullfrog console before relying on automatic reviews. This policy does not
make hosted CI or Pullfrog a required merge check.
