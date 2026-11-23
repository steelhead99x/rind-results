<!-- Proposed path: quest/m2/bench/ab-target-dir.md. No milestone README row: rind's milestone READMEs list no quests. -->
# [XS] A/B helpers use a target dir per variant

## Goal

Interleaved A/B scripts that build more than one revision, including the `ab.sh` in [steelhead99x/rind#234](https://github.com/steelhead99x/rind/pull/234) and any shared helper that shares one `CARGO_TARGET_DIR` across revisions, build each variant into its own target dir and refuse to time if the hashes are wrong. New must differ from base; the control binary must match base. Check by running the hash guard against a good pair and against a pair that reuses the base binary as new.

Without the split, cargo can reuse base artifacts for the new revision and the run silently times base against itself, which prints a clean 1.000 new/base.

## Plan

Planned with the maintainer on 2026-11-20, after a #234 A/B on aarch64 produced identical new and base binaries from two worktrees that shared one `CARGO_TARGET_DIR` per limb width.

- Give each variant (base, new) its own `CARGO_TARGET_DIR`. When limb width or other `RUSTFLAGS` change the build, split on those too.
- After the binaries are copied and before the first timed round, hash them. Exit non-zero if new matches base, or if the control binary does not match base, and print which pair failed.
- Apply the same split and guard to in-tree helpers that build several revisions into one target dir (for example the results repo's variant-build scripts that loop tarballs into a single `CARGO_TARGET_DIR`). Helpers that already use one target dir per source tree, or that only run pre-built binaries, are out of scope except to add the hash guard if they copy those binaries.
- Keep the existing control rule: a cell is VALID only if ctl/base is within 1.02.

## Acceptance

- Two revisions that differ in the timed code produce distinct hashes for each new vs base binary, and a control binary that is a copy of base has the same hash as base.
- A run that points both variants at one shared target dir, or that copies the base binary as new, is rejected by the hash guard before any timing round.
- A clean A/B of two distinct revisions still prints per-cell new/base and ctl/base, and still marks VALID or NOISY from the control bound.
