<!-- CURSOR_AGENT_PR_BODY_BEGIN -->
## Problem

From a fork, the skill told people to skip `quest start` and branch by hand until the work was meant to go upstream. The quest then had no claim other contributors could see. An agent token also cannot open that upstream claim: `quest start` returns 403 `Resource not accessible by integration`. Several other limits make an agent invent work or report a step it did not run: `quest unblocked --unclaimed` ignores `## Waiting`, legacy claim branches are not the conventional name, cloud VMs lack Nix and the reviewer CLIs, a cloud checkout is named `cursor/<name>`, and the large protocol quests are blocked on open required PRs.

## Approach

Claim with `quest start` (or the `quest-spawn` / `quest-start` stubs) when the token can open pull requests on moq-dev/rind, after the overlap check. On 403, open a draft PR on the fork right away whose title and body name the quest path, and treat that draft as the claim. The overlap check includes those drafts and legacy branch names, matched by the files they touch. An agent still works only quests the maintainer already wrote. A `## Known issues` section records the six current limits and tells the agent to report a blocked step instead of inventing work. Implementation PRs can still target the fork's `main`.

## Impact

- `rind-quest-run`: claim with `quest start` when the token can open a pull request on moq-dev/rind. On 403, the claim is a draft PR on the fork that names the quest path. A claim must not be taken on a quest already in flight, including a fork draft and a legacy branch such as `quest/oss-fuzz`.
- `rind-quest-run`: an agent works only quests that already exist on upstream moq-dev/rind `quest/`, written by the maintainer, and records an unclear or missing requirement, vector, test result, or benchmark number as a question.
- `rind-quest-run`: known limits are recorded: `quest unblocked --unclaimed` ignores `## Waiting`; cloud VMs report Nix, ctgrind, `admin/prove`, and `quest review` as not run; a `cursor/<name>` checkout is not the claim branch; FROST, the OPRF, OPAQUE, and HPKE's post-quantum KEM stay blocked on open required PRs.

## Testing

`cargo +stable run --locked --quiet` passed on this head.

Sources:

- Claim PR from a fork: moq-dev/quest README at `f3301efd8c7831c7aa33ae2e6f13995170f3e205` (the `quest` revision in rind `flake.lock` at `c33990e612de138167c7dd4b87116d72beec65e1`), section "Claim a quest": a fork opens the PR on its upstream. The 403 text is the GitHub API response when that token cannot open the pull request.
- `--unclaimed` and `## Waiting`: same quest README, "Prioritize ready work" (`--unclaimed` drops held quests and does not inspect outside conditions) and "Find what waits on you" (`quest waiting` lists `## Waiting`).
- Legacy branch: [moq-dev/rind#639](https://github.com/moq-dev/rind/pull/639), head `quest/oss-fuzz`, file `quest/m4/oss-fuzz.md`.
- Nix commands: rind `CONTRIBUTING.md` "Building and testing" at `c33990e612de138167c7dd4b87116d72beec65e1` (`nix develop .#ctgrind`, `admin/ctgrind`, `nix develop .#prove`, `admin/prove`). `quest review` needs the `claude` and `codex` CLIs, as the skill already records.
- Branch names: `skills/rind-git/SKILL.md` "Branch naming" (`quest/<milestone>/<area>/<name>`) against a cloud checkout named `cursor/<name>`.
- Blocked protocol quests, on rind `c33990e`: `quest/m4/protocols/frost.md` and `quest/m4/protocols/oprf.md` require `quest/m4/protocols/hash-to-curve.md`, open as [moq-dev/rind#632](https://github.com/moq-dev/rind/pull/632); `quest/m4/protocols/opaque.md` requires `quest/m4/protocols/argon2.md`, open as [moq-dev/rind#634](https://github.com/moq-dev/rind/pull/634); `quest/m4/protocols/hpke-pq.md` requires `quest/m3/parity/ml-kem.md`, open as [moq-dev/rind#606](https://github.com/moq-dev/rind/pull/606).
- Existing-quest rule: rind `AGENTS.md` at that commit, "Quests: before any work, run `quest context` and follow it".

## Alternatives

Keep telling every agent to run `quest start` and to treat an empty `quest unblocked --unclaimed` list as a reason to write a new quest. The 403 leaves the quest unlocked on moq-dev/rind, and a new quest file is not the maintainer's plan.

## Follow-ups

None.

<!-- CURSOR_AGENT_PR_BODY_END -->

<div><a href="https://cursor.com/agents/bc-02173834-8ded-5c3a-90cc-cb2ff1bfe851?cursor_ref=pr_footer&cursor_cta=open_in_web"><picture><source media="(prefers-color-scheme: dark)" srcset="https://cursor.com/assets/images/open-in-web-dark.png"><source media="(prefers-color-scheme: light)" srcset="https://cursor.com/assets/images/open-in-web-light.png"><img alt="Open in Web" width="114" height="28" src="https://cursor.com/assets/images/open-in-web-dark.png"></picture></a>&nbsp;<a href="https://cursor.com/background-agent?bcId=bc-02173834-8ded-5c3a-90cc-cb2ff1bfe851&cursor_ref=pr_footer&cursor_cta=open_in_cursor"><picture><source media="(prefers-color-scheme: dark)" srcset="https://cursor.com/assets/images/open-in-cursor-dark.png"><source media="(prefers-color-scheme: light)" srcset="https://cursor.com/assets/images/open-in-cursor-light.png"><img alt="Open in Cursor" width="131" height="28" src="https://cursor.com/assets/images/open-in-cursor-dark.png"></picture></a>&nbsp;</div>


