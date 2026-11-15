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

1. Never add or invent a quest file. A follow-up a quest file mentions is
   left for the maintainer through `/quest-plan` (rind `CONTRIBUTING.md`,
   "Follow-ups") and raised as a question in the PR.
   `quest/m4/protocols/spake2plus.md` plans a `rind-rs-matter` adapter
   "once this lands", and `quest/m4/protocols/README.md` says a
   `rind-<consumer>` adapter is "planned when one is wanted".
   `quest/m4/protocols/rind-rs-matter.md` is not in upstream `quest/`.
2. Before starting work, map it to a quest path that already exists on
   upstream `main`. rind `AGENTS.md` says to run `quest context` and
   follow it before any work. A clean verification does not make a quest.
   Work that fits no quest of its own belongs in the PR of a quest that
   already owns that work, or it stays held: do not open or merge it on
   its own.
3. A verdict pins a head SHA. rind `CONTRIBUTING.md` ("Reviews") counts
   the `## Review` record as the required review when it names the head
   being merged. The branch can be rewritten after that verdict, so the
   tip is a new SHA. [steelhead99x/rind#1](https://github.com/steelhead99x/rind/pull/1)
   went from verified head `26ee84ec` to `64eb50db` with identical tree
   `4f96b4ef`. Before relying on the verdict, compare
   `git rev-parse <sha>^{tree}`. An identical tree means the verdict
   still covers the content. A different tree needs re-verification.
4. GitHub Actions on the fork can sit queued for a long time: dozens of
   jobs for 30 minutes or more. Wait until every required check has
   completed and passed. A queued or in-progress run is not a pass,
   required or advisory (rind `CONTRIBUTING.md`: fix failing CI checks
   before merging; a path-filtered workflow cannot be a required check).
   Do not push a merge of upstream `main` onto a branch whose CI is
   still running. rind `.github/workflows/check.yml` sets
   `cancel-in-progress: true` on `check-${{ github.ref }}`, so that push
   cancels the run and starts it over.
