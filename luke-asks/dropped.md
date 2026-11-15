# Asks considered and dropped (not drafts, not to be posted)

Checked against upstream main `b61332fe` on 2026-11-12, America/Los_Angeles.

## (b) #609 wipe feature — dropped

Already answered. On 2026-11-12 Luke superseded the earlier "on by default" decision: wiping is mandatory and the `wipe` feature is removed (rejected: a deprecated no-op `wipe = []`, and on-by-default with an opt-out). That is recorded in the #609 PR body and matches the current #609 head `ca47dcf3` (`rind`, `rind-aws-lc-rs`, and `rind-rcgen` no longer have a `wipe` feature). #683 and `quest/m4/fips/module.md` on main already say the same, pointing at #609. The earlier note that "#609's diff keeps it" is stale.

## (f) Dart questions after #671 — dropped

No longer open as asks. #671 merged the `rind_cryptography` package as `ad306e3d` with no rind-dart changes, on the released rind 0.1.0 API.

- SHA-224 stays refused: `sha224()` throws `UnsupportedError('rind has no SHA-224')`. That is in #671's approach ("everything else throws") and in the landed code.
- AES-GCM nonces other than 12 bytes stay refused in the landed package (`_unsupported('AES-GCM with $nonceLength-byte nonces')`); those cases are skipped in `compat/cryptography.txt`.
- `withRandom` is documented and implemented to pick nonces, secret keys, and X25519 and Ed25519 key seeds. ECDSA and ECDH `newKeyPair` call rind's generator (`EcdsaKeyPair.generate` / `AgreementPrivateKey.generate`), not that `Random`.
- The two follow-up quests Luke kept already have their answers: `rind-dart-cipher-modes.md` has a decided plan (CBC/CTR with the package's MAC over ciphertext only, unauthenticated IV called out in the README); `rind-cryptography-release.md` already has a maintainer Waiting item to publish 0.1.0 by hand. Posting that wait again would only repeat the quest.

Fork #36 was closed as superseded and was not reopened as an ask.

## (g) SSH to the Mac mini for apple-nightlies — dropped

No quest asks a contributor for SSH access. `quest/m0/mac-mini-runner.md` has a Waiting item for the maintainer: the machine arrives, is on the network, and agents can reach it over SSH with an account that can install Nix and register a runner. `quest/m2/confidence/apple-nightlies.md` only requires that runner quest first. Asking for Jack's SSH would invent a request the quests do not make.
