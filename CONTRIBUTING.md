<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# Contributing

Run `cargo xtask ci` locally before pushing. It is the gate:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny --locked check` (installs cargo-deny 0.20.2 when the `cargo deny` subcommand is missing)
- `cargo verify` on the published trees, then `cargo xtask verify --dir` on every results tree that gate names

Pullfrog reviews pull requests. Its review rules are in [AGENTS.md](AGENTS.md), which the agent reads, and in the repository console under Automations → Review PRs → Custom instructions. GitHub Actions is a thin backstop: [`.github/workflows/ci.yml`](.github/workflows/ci.yml) runs `cargo xtask ci` and nothing else.

Pullfrog runs OpenAI models on a ChatGPT Codex subscription. Connect it with `npx pullfrog auth codex --repo steelhead99x/rind-results`; that credential stays in Pullfrog's encrypted store. [`.github/workflows/pullfrog.yml`](.github/workflows/pullfrog.yml) maps `OPENAI_API_KEY` from a repository Actions secret only as a fallback. This repository does not set that secret, and the workflow does not pin a model.

## Measured results

Do not edit, delete, or re-measure a published number. A new publish is `results/<host>/<upstream-sha>/` (or `results/<host>/<upstream-sha>-<label>/`) with three runs, a schema-1 `<host>.json`, `RUNS.md`, and `RESULTS.md`. Historical trees stay where they were published. [results/README.md](results/README.md) is the layout.
