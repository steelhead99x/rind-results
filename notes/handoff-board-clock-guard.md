<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Handoff: board clock guard

Status: **not implemented**. This branch adds this note only. The next agent implements the guard. Do not start a timed bench while doing that.

Snapshot: `main` at `88cbf8504400a9918d206b8a4895c58b7d278cc6`.

## Facts

- The Jetson Orin Nano bench board (`admin@192.0.2.1`, reached by `ssh jetson` from the user's x86 PC) is offline, has no NTP, and its clock sat at January 1970. Its RTCs are `nvvrs-pseq-rtc` (rtc0) and `tegra_rtc` (rtc1).
- Because of that, the bench harness ran benches under libfaketime with `FAKETIME_OFFSET=1794547759`. That constant is 1970-01-01 plus 1794547759 seconds, which is 2026-11-13 05:29:19 UTC, the same day as the published Orin window in `results/jetson-orin-nano/NOTES.md` (2026-11-13 03:18–05:05 UTC). The script names and their xtask ports are the orin row of the table in `hosts/README.md`: `orin-setup.sh`, `orin-env.sh`, `orin-run.sh`, `orin-faketime.sh`, `orin-runs.sh`, and `cargo xtask orin-setup` / `orin-run` / `orin-runs`. rustls benches need a sane wall clock (certificate validity). `results/jetson-orin-nano/NOTES.md` records the `NotValidYet` panic and the libfaketime control.
- On 2026-11-18 ~04:37 UTC the board clock was set from the PC's clock and written to rtc0 with `hwclock -w`. `tegra_rtc` (rtc1) still reads 1970. systemd-timesyncd NTP was disabled on the board because the board is offline. `FAKETIME_OFFSET` now defaults to 0 in the on-board scripts (`pipeline-0b1f1469.sh`, `post-tests-0b1f1469.sh`; `orin-run.sh` skips libfaketime when the offset is 0).
- Those three scripts are on the board's filesystem. They are not in this git tree (no match for `pipeline-0b1f1469`, `post-tests-0b1f1469`, or `1794547759`). Do not reintroduce a hardcoded offset in them, and do not commit board-local pipeline scripts into this repo.
- Risk: the clock can silently drift or reset (power loss, RTC battery, the 1970 rtc1). Logs and results then get wrong timestamps, or rustls benches fail or skew, and nobody notices.

Leave the published `results/jetson-orin-nano/` tree as measured. That includes `NOTES.md`, the JSON, `RUNS.md`, `RESULTS.md`, and `checks/faketime-control.txt`. `tools/xtask/src/host_report.rs` already prefers `raw/run-<n>.pc.txt` (`pc_started:` / `pc_finished:`) over the board clock when those lines exist. Keep that contract. Add new fields as extra lines. Do not rename `pc_started:` or `pc_finished:`.

## Where the code lives

`orin-run` and `orin-runs` are on-board commands. They read local `/proc/stat` and the staged toolchain under `~/rind-bench-orin`. They do not ssh. The old `scripts/orin-*.sh` wrappers are gone. `scripts/README.md` only records the Argon2 and Pi 5 A/B recipes.

| What | Where |
|---|---|
| CLI | `tools/xtask/src/main.rs`: `Cmd::OrinSetup`, `Cmd::OrinRun`, `Cmd::OrinRuns`, `Cmd::PiRun` |
| On-board runs | `tools/xtask/src/boards.rs`: `faketime_script`, `orin_setup`, `orin_run`, `orin_runs`, `pc_now`, `pc_stamp`, `pi_run` |
| Bench entry that can still inject libfaketime | `tools/xtask/src/bench.rs`: `Args.faketime_offset`, `write_meta`, `host_runs` |
| Runner and Orin env | `tools/xtask/src/upstream_run.rs`: `faketime_runner`, `orin_env` |
| Published time override | `tools/xtask/src/host_report.rs`: `pc_times` |
| Local-host facts | `tools/xtask/src/host_run.rs` (`date:` in `<out>.facts.txt`) |
| Host index | `hosts/README.md` (orin scripts table) |
| Pi 5 is reached over ssh from the desktop | `hosts/pi5/NOTES.md`, `SKILL-NOTES.md` item 9 (SSH alias `pi5`) |
| Skill text that already says a board with no RTC boots to 1970 | `skills/embedded-crypto-benchmark-port/SKILL.md`, section "Linux SBC hosts" |

Current faketime behavior on `main`:

- `orin_run` installs `orin-faketime.sh` and sets `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER` only when `FAKETIME_OFFSET` is set and non-empty. Unset already skips libfaketime.
- `orin_runs` parses `FAKETIME_OFFSET` as `i64` and defaults to 0, then `pc_now` **adds that offset to the local clock** and writes the sum as `pc_started` / `pc_finished`. That is how a 1970 board produced November 2026 stamps. Stop adding the offset. Those stamps must be the real PC time.
- `faketime_script` expands `${FAKETIME_OFFSET:?}` when the bench binary starts. `upstream_run::faketime_runner` bakes a literal offset into `faketime-runner.sh` for `cargo xtask bench --faketime-offset`.
- There is no clock comparison anywhere.

`pi_run` and `host_run` run on the machine where they are invoked. `mac_run`, Pixel adb, and bare-metal capture are out of scope.

## Implement this

Put the decision in a pure function in `tools/xtask/src/boards.rs` and call it before any `/proc/stat` sleep or bench. Suggested shape:

```text
clock_guard(pc_unix, board_unix, max_skew_s, override_ok) -> Result<ClockReport, ClockGuardError>
faketime_allowed(clock_sane, offset_secs) -> Result<(), String>
```

`|skew|` is the absolute difference of the two Unix epochs, in whole seconds. The threshold is exceeded only when `|skew| > max_skew_s` (120 passes, 121 fails). Default `max_skew_s` is 120. The PC clock is the reference.

How the two epochs are obtained (do not ssh from a command that is already on the board, and do not assume the board can reach the PC):

- `BOARD_SSH` set (for example `jetson` or `pi5`): this process is on the PC. Local `date -u +%s` is the PC. `ssh -o BatchMode=yes "$BOARD_SSH" date -u +%s` is the board.
- `BOARD_SSH` unset and `BOARD_PC_EPOCH` set: this process is on the board. Local `date -u +%s` is the board. The env value is the PC epoch, taken on the PC at the moment of the ssh, as in `ssh jetson "BOARD_PC_EPOCH=$(date -u +%s) cargo xtask orin-runs"`.
- Orin paths (`orin_run`, `orin_runs`, and `bench` host runs when the board is `jetson-orin-nano`) require one of those two. Missing both is a hard failure.
- `pi_run`, `host_run`, and `bench` host runs for any other host use the same helper when `BOARD_SSH` or `BOARD_PC_EPOCH` is set. When neither is set, still fail if the local UTC year is < 2025, and record `clock_check: local-only`.
- A non-integer `date` or a failed ssh is a hard failure. Do not start the run.

### 1. Pre-run check

Fail with a non-zero exit and a message that names the board epoch, the PC epoch, the skew in seconds, and the threshold, when `|skew|` exceeds the threshold. Also fail when the board UTC year is < 2025, including when the skew itself is inside the threshold.

- Threshold env: `BOARD_MAX_CLOCK_SKEW_S`. Absent or empty means 120. A non-integer is a hard failure.
- Override env: `BOARD_CLOCK_OVERRIDE=1` continues, prints a warning on stderr, and records `clock_override: 1` in the run log. Any other value is not an override.
- Call the check at the start of `orin_run` and again at the start of `orin_runs` (a single `orin-run` must be covered; a later run in the trio must be covered if the clock dies mid-suite). Call it from `bench::host_runs` before the first bench-report. Call it from `pi_run` and `host_run` as above.
- Do not re-enable systemd-timesyncd. The board is offline on purpose.

### 2. Record the clocks

On every run, write the measured skew, the PC time, and the board time into the run log and the results metadata:

```text
clock_pc_epoch: <i64>
clock_pc_utc: <pc_stamp>
clock_board_epoch: <i64>
clock_board_utc: <pc_stamp>
clock_skew_s: <board - pc, signed>
clock_max_skew_s: <threshold>
clock_override: 0|1
```

Write that block into:

- `~/rind-bench-orin/raw/run-<n>.pc.txt` (same file as `pc_started:` / `pc_finished:`)
- `raw/run-<n>.load.txt` and `raw/run-<n>.log`
- `cargo xtask bench`'s `run.log` and `meta.json` (`bench::write_meta`)
- `<out>.facts.txt` when `host_run` is the driver

`pc_started` / `pc_finished` stay real PC UTC from `pc_stamp`. Do not add `FAKETIME_OFFSET` inside `pc_now`. Do not put these fields into rind's bench-report result JSON. `host_report::pc_times` must keep working unchanged.

### 3. Optional auto-fix

Flag: `--set-clock` on `OrinRun`, `OrinRuns`, and `PiRun`, and the env `BOARD_CLOCK_AUTOSET=1` for an ssh one-liner. `bench` should honor the env.

When the check would fail and the flag is set, run, with the PC's current epoch substituted:

```text
sudo -n date -u -s @<pc epoch> && sudo -n hwclock -w
```

That writes rtc0, which is the RTC that held time after 2026-11-18. Do not run `hwclock` against `/dev/rtc1`. From the PC the same command is wrapped in `ssh -o BatchMode=yes "$BOARD_SSH" '...'`.

Re-read both clocks after a successful set and apply the same threshold. If `sudo -n` is not available, print that command with the real epoch filled in, and still exit non-zero. Do not start the bench. `BOARD_CLOCK_OVERRIDE=1` is the only way past a failed check.

### 4. RTC health

Warn, do not fail the run, when any `/sys/class/rtc/rtc*` `date` is a year < 2025, or when its `since_epoch` differs from the system clock by more than `BOARD_MAX_CLOCK_SKEW_S`. Include the rtc index and the `name` file (`nvvrs-pseq-rtc`, `tegra_rtc`). This warning is what flags rtc1 still sitting in 1970 after `hwclock -w` updated rtc0. Put the warning on stderr and in the same run log as the skew block. Read the sysfs files over `BOARD_SSH` when the process is on the PC, and locally when it is on the board.

### 5. `FAKETIME_OFFSET` is opt-in

After the skew check exists, a non-zero offset is an explicit opt-in. Never default it to `1794547759` or any other stale constant. Unset or empty means no libfaketime runner (`orin_run` is already there; keep `bench` there; keep `orin_runs` from turning 0 into a runner).

If the board clock is already sane (year >= 2025 and `|skew| <= threshold`, including after a successful auto-fix), a non-zero offset is a hard error. `BOARD_CLOCK_OVERRIDE` does not bypass that refusal. The published faketime control stays in `results/jetson-orin-nano/checks/faketime-control.txt`. A bad clock plus a non-zero offset still requires `BOARD_CLOCK_OVERRIDE=1` to run at all, because the skew check fails first.

Apply the same rule anywhere a runner is written: `boards::write_runner` / `faketime_script`, and `bench::host_runs` / `upstream_run::faketime_runner`.

### 6. Login warning, manual

Document this as an optional manual step. Do not have xtask install it. It needs root on the board. Suggested `/etc/profile.d/board-clock.sh`:

```sh
# /etc/profile.d/board-clock.sh
y=$(date -u +%Y)
if [ "$y" -lt 2025 ]; then
  echo "WARNING: board UTC year is $y (< 2025). Clock is wrong. rustls benches and logs will be wrong. Set the clock from the PC before measuring." >&2
fi
```

Put the same warning, the env names, and the auto-fix command into `skills/embedded-crypto-benchmark-port/SKILL.md` (the Linux SBC bullet that already says to set the clock from the PC) and a short pointer on the orin row of `hosts/README.md`.

### 7. Tests

Unit-test the pure function in `tools/xtask/src/boards.rs` `mod tests`. No ssh, no `/sys`, no `date -s`, no published tree.

| pc epoch | board epoch | max skew | override | offset | expect |
|---|---|---|---|---|---|
| 1763437020 (2025-11-18 03:37:00 UTC) | same | 120 | no | 0 | ok, skew 0 |
| same | pc + 120 | 120 | no | 0 | ok |
| same | pc + 121 | 120 | no | 0 | error |
| same | pc - 121 | 120 | no | 0 | error |
| same | pc + 200 | 300 | no | 0 | ok (env threshold) |
| 1763437020 | 21 (1970-01-01 00:00:21 UTC) | 120 | no | 0 | error (year and skew) |
| 1735689599 (2024-12-31 23:59:59 UTC) | same | 120 | no | 0 | error (year < 2025) |
| 1735689600 (2025-01-01 00:00:00 UTC) | same | 120 | no | 0 | ok |
| 1763437020 | pc + 10000 | 120 | yes | 0 | ok with warning |
| sane pair | sane pair | 120 | yes | 1794547759 | error (sane clock refuses offset) |
| 1763437020 | 21 | 120 | yes | 1794547759 | ok with warning (explicit faketime on a bad clock) |

`pc_stamp` already has tests. Assert the error text contains the skew, both epochs, and `BOARD_MAX_CLOCK_SKEW_S`.

Simulate a bad clock without touching the live board. From the PC, after the code exists, this must exit non-zero before any bench and must not create a `run-*.json`:

```sh
ssh jetson "BOARD_PC_EPOCH=0 cargo xtask orin-run 9"
```

On the PC, with `BOARD_SSH=jetson`, the same failure is a board epoch that is not within 120 seconds of the local clock. To exercise the threshold without a wrong board, the unit table above is enough. A live `date -u -s` drill is optional, must use `sudo -n`, and must restore the clock in the same session. Do not leave the board in 1970. Do not run `orin-runs` or `cargo xtask bench` as the test: those start real measurements.

Then run `cargo xtask ci` before pushing. That is the gate (`.github/workflows/ci.yml` only runs that command).

## Acceptance criteria

- A board whose UTC year is < 2025, or whose `|skew|` against the PC is greater than `BOARD_MAX_CLOCK_SKEW_S` (default 120), makes `orin-run`, `orin-runs`, and `cargo xtask bench --board jetson-orin-nano` exit non-zero before a bench, with a message that names both clocks and the threshold.
- `BOARD_CLOCK_OVERRIDE=1` is the only continuation, and the warning is in the run log.
- Every new run's `run-<n>.pc.txt` (and the bench `meta.json` / `run.log`, and `host_run`'s facts file when that path is used) records PC epoch, board epoch, and signed skew. `host-report` still reads `pc_started:` / `pc_finished:` as real PC UTC.
- `--set-clock` / `BOARD_CLOCK_AUTOSET=1` runs `sudo -n date -u -s @<pc epoch> && sudo -n hwclock -w` when sudo is non-interactive, then re-checks. Otherwise it prints that command with the epoch filled in and exits non-zero.
- An rtc whose `date` year is < 2025, or whose `since_epoch` is outside the threshold, produces a warning that names the rtc (`tegra_rtc` / rtc1 in the incident). The warning alone does not fail the run.
- Unset `FAKETIME_OFFSET` never preloads libfaketime. A non-zero offset on a sane clock fails the run. No source default is `1794547759`.
- The profile.d snippet is documented in the skill and is not installed by xtask.
- The decision table above is a unit test. `cargo xtask ci` passes.
- `results/jetson-orin-nano/` and every other published results tree are byte-for-byte unchanged.

## Files to change

- `tools/xtask/src/boards.rs` — guard, RTC warning, faketime refusal, log lines, tests. Call from `orin_run`, `orin_runs`, `pi_run`.
- `tools/xtask/src/main.rs` — `--set-clock` on `OrinRun`, `OrinRuns`, `PiRun`.
- `tools/xtask/src/bench.rs` — call the guard from `host_runs`; record the block in `write_meta` and `run.log`; refuse a sane-clock `--faketime-offset`.
- `tools/xtask/src/upstream_run.rs` — keep `faketime_runner` and `orin_env` free of a default offset.
- `tools/xtask/src/host_run.rs` — local year check, remote check when `BOARD_SSH` or `BOARD_PC_EPOCH` is set, same lines in the facts file.
- `hosts/README.md` — one pointer on the orin row: clock guard, `FAKETIME_OFFSET` opt-in.
- `skills/embedded-crypto-benchmark-port/SKILL.md` — replace the "set the clock from the PC" bullet with this guard, the envs, the auto-fix command, and the profile.d snippet.
- `notes/handoff-board-clock-guard.md` — set the status line to implemented, or delete this note once the skill and `hosts/README.md` carry the operational text.

Do not change `tools/xtask/src/host_report.rs` unless a test shows `pc_times` breaking. Do not add a new GitHub Actions job.

## Done when

- [ ] Skew above the threshold exits non-zero and names both epochs, the skew, and `BOARD_MAX_CLOCK_SKEW_S`.
- [ ] Board UTC year < 2025 exits non-zero.
- [ ] `BOARD_MAX_CLOCK_SKEW_S` changes the threshold.
- [ ] `BOARD_CLOCK_OVERRIDE=1` continues, and the warning is in the run log.
- [ ] PC epoch, board epoch, and signed skew are in each new run's pc.txt / log / metadata.
- [ ] `pc_started:` / `pc_finished:` are still real PC UTC and `host-report` still parses them.
- [ ] Auto-fix sets the clock when `sudo -n` works; otherwise it prints the exact command and does not start a bench.
- [ ] A stale rtc (the `tegra_rtc` case) warns and does not by itself fail the run.
- [ ] Unset `FAKETIME_OFFSET` does not preload libfaketime.
- [ ] A non-zero offset on a sane clock exits non-zero.
- [ ] Nothing defaults `FAKETIME_OFFSET` to `1794547759`.
- [ ] The profile.d snippet is documented and not installed by xtask.
- [ ] The unit table is tested, and the `BOARD_PC_EPOCH=0` simulation is documented.
- [ ] `cargo xtask ci` passes.
- [ ] No published results tree is edited.

## Out of scope

- Editing, regenerating, or re-measuring any published number, raw run, or table.
- Opening a pull request against upstream rind, or changing any repo other than this one.
- Re-enabling NTP, writing rtc1, stopping the board's services, or running a real Orin or Pi 5 measurement.
- Resurrecting `scripts/orin-*.sh`. The ports in `boards.rs` are the commands.
