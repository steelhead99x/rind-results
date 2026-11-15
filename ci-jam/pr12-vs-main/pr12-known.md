## Known issues

On Nov 12, 2026, within about an hour, the maintainer opened his own
upstream PRs for four quests the fork had already claimed with fork draft
PRs:

- [moq-dev/rind#674](https://github.com/moq-dev/rind/pull/674) (ecvrf, fork [steelhead99x/rind#19](https://github.com/steelhead99x/rind/pull/19))
- [moq-dev/rind#677](https://github.com/moq-dev/rind/pull/677) (fips/drbg, fork [steelhead99x/rind#24](https://github.com/steelhead99x/rind/pull/24))
- [moq-dev/rind#671](https://github.com/moq-dev/rind/pull/671) (rind-dart-cryptography, fork [steelhead99x/rind#36](https://github.com/steelhead99x/rind/pull/36))
- [moq-dev/rind#670](https://github.com/moq-dev/rind/pull/670) (android-armv7-abi, fork [steelhead99x/rind#21](https://github.com/steelhead99x/rind/pull/21))

Fork draft-PR claims are not visible upstream and don't reserve a quest.
Check upstream open PRs and `quest/*` branches, not just claim commits,
before claiming, again partway through, and again before marking ready.
When upstream takes a quest, stop and hold the fork PR with a comment.
Prefer quests whose files show no recent upstream activity.

These limits are current. Report a step they block. An agent does not
invent a quest, a requirement, or a result to get past one.

1. `quest start` from a fork opens its claim PR on the upstream that holds
   `quest/README.md` (quest README, "Claim a quest",
   [`f3301ef`](https://github.com/moq-dev/quest/blob/f3301efd8c7831c7aa33ae2e6f13995170f3e205/README.md)).
   An agent token that cannot open pull requests on moq-dev/rind gets
   403 `Resource not accessible by integration`. Open the fork draft in
   [Working from a fork](#working-from-a-fork) so the fork can see the
   work. It does not reserve the quest on moq-dev/rind.
2. `quest unblocked --unclaimed` drops a quest someone already holds, and
   it does not read `## Waiting`. Outside waits are `quest waiting`
   (quest README, "Prioritize ready work" and "Find what waits on you",
   same commit). Read each candidate's `## Waiting` section before
   claiming it.
3. A claim on a legacy branch name is invisible to a search for
   `quest/<milestone>/<area>/<name>`.
   [moq-dev/rind#639](https://github.com/moq-dev/rind/pull/639) is
   `quest/oss-fuzz` for `quest/m4/oss-fuzz.md`. Match the files the
   branch or PR touches.
4. A cloud VM has no Nix, so it cannot run `nix develop .#ctgrind`,
   `admin/ctgrind`, `nix develop .#prove`, or `admin/prove` (rind
   `CONTRIBUTING.md`, "Building and testing", at the commit the README
   cites). It also lacks the `claude` and `codex` CLIs, so `quest review`
   exits 2. Report each of those steps as not run.
5. [rind-git](../rind-git/SKILL.md) names a quest branch
   `quest/<milestone>/<area>/<name>`. A cloud agent checkout is
   `cursor/<name>`. Only a `quest start` pull request on moq-dev/rind
   reserves the quest. A `cursor/<name>` branch and a fork draft do not.
   An upstream-bound rebuild still follows `rind-git`.
6. The ready backlog is often already held or waiting. FROST and the OPRF
   (`quest/m4/protocols/frost.md`, `quest/m4/protocols/oprf.md`) require
   hash-to-curve, open as
   [moq-dev/rind#632](https://github.com/moq-dev/rind/pull/632). OPAQUE
   (`quest/m4/protocols/opaque.md`) also requires Argon2id, open as
   [moq-dev/rind#634](https://github.com/moq-dev/rind/pull/634). HPKE's
   post-quantum KEM (`quest/m4/protocols/hpke-pq.md`) requires ML-KEM,
   open as
   [moq-dev/rind#606](https://github.com/moq-dev/rind/pull/606).
   Report that and stop.