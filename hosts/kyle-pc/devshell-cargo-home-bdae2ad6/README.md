<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# kyle-pc: `just ci` for kixel-ai/rind#876 at `bdae2ad6` (2026-11-17)

Local full `just ci` (`CI_STRICT=1`) for
[kixel-ai/rind#876](https://github.com/kixel-ai/rind/pull/876),
"fix(devshell): stop rustup shims shadowing the flake's cargo", at
`bdae2ad60fcc6171f1e544ababd030e4177b4e8b`, in the flake's dev shell on an
x86_64 Linux workstation.

Result: **EXIT=0**. Every module passed, in 730 s wall and 2777 s CPU. The PR touches root
orchestration, so the dispatcher ran every module.

| module | wall s | CPU s |
|---|---|---|
| check rs | 219 | 1104 |
| check cross | 107 | 256 |
| check wasm | 98 | 141 |
| check msrv | 125 | 166 |
| check vartime | 33 | 501 |
| check dart | 244 | 93 |
| check adapters | 146 | 261 |
| check site | 468 | 32 |
| all other modules (py, openssl, patch, structure, fixtures, deny, mutants, versions, license, testing, audit, compat, bench, quest, gh, shell, toolchain, nix) | 0-20 each | |
| **whole run** | **730** | **2777** |

[`just-ci-summary.log`](just-ci-summary.log) holds the dispatcher's pass lines and the timing table. It leaves out the
compile output from the 860 KB raw log.
