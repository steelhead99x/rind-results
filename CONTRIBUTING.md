<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->

# Contributing

Run `cargo xtask ci` locally before pushing. It is the gate:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `cargo test --workspace --locked`
- `cargo deny --locked check` (installs cargo-deny 0.20.2 when the `cargo deny` subcommand is missing)
- `cargo verify` on the published trees, then `cargo xtask verify --dir` on every results tree that gate names (both aliases build with `--locked`)

GPT review or PrairieDog reviews pull requests. The review rules are in [AGENTS.md](AGENTS.md), which any reviewer or agent reads. GitHub Actions is a thin backstop: [`.github/workflows/ci.yml`](.github/workflows/ci.yml) runs `cargo xtask ci` and nothing else.

## Measured results

Do not edit, delete, or re-measure a published number. A new publish is `results/<host>/<upstream-sha>/` (or `results/<host>/<upstream-sha>-<label>/`) with three runs, a schema-1 `<host>.json`, `RUNS.md`, and `RESULTS.md`. Historical trees stay where they were published. [results/README.md](results/README.md) is the layout.

## Quests

[`quests/`](quests/README.md) holds this repo's prioritized work: one markdown file per quest, with frontmatter (`id`, `title`, `priority`, `hardware`, `size`, `status`, `claimable_by`). The file is the spec.

- Only the maintainer claims quests for now (`claimable_by: maintainer`). Comments and PRs with results are welcome, and the results rules above still apply.
- Each quest has one issue labelled `quest`, its priority (`P0`–`P3`), `size:S|M|L`, and `needs:<board>` or `needs:none`. `quest-sync` (`.github/workflows/quest-sync.yml`, running `tools/quest-sync.py`) creates and updates those issues from the frontmatter on every push to `main`. Edit the quest file, not the issue. When `status` is `done` or `dropped`, the issue closes.
- `claim-guard` replies to claim-style comments ("I'll take", "claim", "assign me", "working on it") on quest issues from anyone without write access. The reply says quests are maintainer-only for now. It assigns nothing and closes nothing.
- To change a quest, open a PR that edits its file. To preview the issue changes, run `python3 tools/quest-sync.py --dry-run`.
