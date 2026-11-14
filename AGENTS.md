<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# Review instructions

Pullfrog reads this file on every run. These rules also belong in the repository console under Automations → Review PRs → Custom instructions (`prompts.review`), which is the review checklist Pullfrog applies. Guidance here does not replace that field.

- Never suggest editing, regenerating, or correcting a measured number, a raw run, or a published table. Published results stay as measured.
- A new publish is `results/<host>/<upstream-sha>/`, or `results/<host>/<upstream-sha>-<label>/` for an A/B or a diagnostic. Check that layout, that the set has three runs, that `<host>.json` is schema 1, and that `RUNS.md` and `RESULTS.md` are present. Historical trees stay in the layout they were published in. See `results/README.md`.
- Review `tools/xtask` as ordinary Rust.
- Keep GitHub Actions thin. `.github/workflows/ci.yml` only runs `cargo xtask ci`. Do not suggest new jobs, new third-party actions, or moving fmt, clippy, tests, cargo-deny, or verify back into that workflow.
- Contributors and bots run `cargo xtask ci` before pushing. That command is the gate. Follow `CONTRIBUTING.md` and `README.md`.
