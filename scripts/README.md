# scripts

These are the shell scripts the Argon2 and PR #33 A/B runs used, kept as the
record of what ran. The Python summaries they called are now `cargo xtask`
commands that print the same bytes (`tools/xtask/tests/crit.rs` checks them
against the scripts' output):

| Was | Now |
|---|---|
| `python3 crit-parse.py ROOT` | `cargo xtask crit-parse ROOT` |
| `python3 argon2-pmu-sum.py FILE [BASE]` | `cargo xtask argon2-pmu-sum FILE [BASE]` |
| `hosts/pi5/ab-narrow-mul/scripts/cmp.py ROOT BASE MODE...` | `cargo xtask ab-cmp ROOT BASE MODE...` |
| `hosts/pi5/ab-narrow-mul/scripts/minc.py ROOT BASE MODE` | `cargo xtask ab-min ROOT BASE MODE` |
| `hosts/pi5/ab-narrow-mul/scripts/collect.py ROOT RUNLOG META.json` | `cargo xtask ab-collect ROOT RUNLOG META.json` |
| `series2-layout/scripts/pmu-sum.py FILE` | `cargo xtask ab-pmu-sum FILE` |
| `series2-layout/scripts/perf-sum.py` (in the `perf-stat` directory) | `cargo xtask ab-perf-sum` (same directory) |

`cargo xtask` runs from inside this repository. Relative paths are taken
from the directory the command runs in, as the scripts took them.

`scripts/render-ours.py` is `cargo xtask render-ours`. It draws every
filesystem-safe host id in the results directory. A CPU slug such as
`intel-core-ultra-9-275hx` is a normal host id. The four hosts published
first stay first, and `box/` is still xeon-vm's directory.
