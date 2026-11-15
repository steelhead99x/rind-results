# Draft issue — DO NOT POST until Jack approves this exact text

- Target: new issue on moq-dev/rind (no existing upstream PR for this work; it is still on the fork)
- Suggested title: A few API questions from fork work on OPRF, FROST, and portable RSA
- Checked against: upstream main `b61332fe` (2026-11-12, America/Los_Angeles), `AGENTS.md`, `CONTRIBUTING.md`, `quest/m4/protocols/oprf.md`, `quest/m4/protocols/frost.md`, and `quest/m3/cold-speed/portable-rsa-karatsuba.md`. None of those answer the questions below. Numbers for the RSA gap are taken from the updated quest file on fork PR #54 head `8ed6a7ba`, not from a looser range in other notes.

---

Hi Luke — I've been implementing a few of the open protocol and portable-speed quests on my fork, and I wanted to check a few API and recording choices before I open them here. The work is still on the fork (links below) because some checks, and in one case a constant-time fix, are still in progress. The quest files on main don't answer these.

### OPRF (RFC 9497, `quest/m4/protocols/oprf.md`)

The draft implements OPRF and VOPRF for the four suites in the quest, with no POPRF. A few choices I'd like your view on:

1. `Output`: I left it without `PartialEq`, and its `Debug` does not print the bytes, because the value is derived from the client's input. The caller can still use `AsRef`. Does that match what you want?

2. Empty input: RFC 9497 allows a zero-length input (it only requires the length to be less than 2^16−1). This implementation accepts it. The `voprf` 0.6.0-pre.2 crate rejects it. I'd like to keep accepting it, since that matches the RFC.

3. Differential: aws-lc-rs has no OPRF, so the differential is against the `voprf` crate, as the quest says. I used 0.6.0-pre.2, the published crate on p256 0.14. 0.5.0 is the last non-pre release and pulls p256 0.13. Is 0.6.0-pre.2 acceptable?

4. Errors: a bad element is `KeyRejected`, a proof that does not verify is `BadSignature`, an input longer than 65534 bytes is `WrongLength`, and `DeriveKeyPair` exhausting its 256 counters is currently `Fault`. Does that mapping look right, or would you rather `DeriveKeyPair` exhaustion were a different variant? RFC 9497 calls that `DeriveKeyPairError`, and `Fault` is otherwise used for an internal bug.

Fork draft: https://github.com/steelhead99x/rind/pull/50

### FROST (RFC 9591, `quest/m4/protocols/frost.md`)

The draft has a trusted dealer, two-round signing, and aggregation for Ed25519, ristretto255, and P-256. A FROST(Ed25519, SHA-512) signature verifies with `signature::verify::ED25519`. Two API choices:

1. Feldman VSS coefficient commitments (appendix C.2) are not returned. The dealer is trusted, and share verification uses the verifying share `[share]B`. Exposing the commitments would let a participant run `vss_verify` across a process boundary. Do you want those exposed?

2. `aggregate` returns `frost::AggregateError::InvalidShare` with that participant's 32-byte identifier, as a type separate from `rind::Error`, so the identifier can be returned without growing every `Error`. Is that all right as it is?

Fork draft: https://github.com/steelhead99x/rind/pull/51

### Portable RSA Karatsuba (`quest/m3/cold-speed/portable-rsa-karatsuba.md`)

The draft adds a constant-time Karatsuba product and square above the measured crossover. Montgomery reduction stays schoolbook, and the quest goal is not fully met. On the Xeon measurements recorded in the updated quest file, after the scratch moved to the stack, 6144- and 8192-bit verification stayed about 1.15× to 1.22× aws-lc's C without assembly.

The quest says to keep pace, or to record why not in `benchmarks/gaps.toml`. How would you like the remaining gap recorded — write it into `gaps.toml` and close the quest, or leave the quest open / move the remainder to a follow-up for the reduction?

Fork draft: https://github.com/steelhead99x/rind/pull/54

Thanks,
Jack
