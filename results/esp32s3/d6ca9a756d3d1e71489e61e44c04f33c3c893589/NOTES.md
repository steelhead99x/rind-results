# ESP32-S3 comparable set at kixel-ai/rind `d6ca9a75`

Recorded 2026-11-13 (America/Los_Angeles).

## What this is

A full 12-image, 3-run ESP32-S3 set on the comparable pin:

- `commit` (upstream): [`d6ca9a756d3d1e71489e61e44c04f33c3c893589`](https://github.com/kixel-ai/rind/commit/d6ca9a756d3d1e71489e61e44c04f33c3c893589)
- `fork_commit` (deterministic unsigned apply-patches): `eb053e787735b1898fa64ebc91ae13b31db663ec`, matching [rind-hardware PR #3](https://github.com/steelhead99x/rind-hardware/pull/3)
- rind-hardware: `a8c8bf4b240c5120f6e0f9f200ef5593656aae1c` (fresh worktree; local `~/rind-hardware` main was not reset)
- Patches 0001–0005 plus the vendored ring Xtensa Poly1305 align patch, each with sha256 in `meta.json` and `esp32s3.json`
- Optional `poly1305-32` was **not** applied
- The rind worktree was clean (`git status --porcelain` empty) at flash and at report time

## Board and harness

- Seeed XIAO ESP32-S3, rev v0.2, Xtensa LX7, 240 MHz, 8 MB flash
- MAC `98:3d:ae:61:37:68`
- Flashed only to `/dev/ttyACM1` (`usb-Espressif_USB_JTAG_serial_debug_unit_98:3D:AE:61:37:68-if00`)
- The T1000-E on `/dev/ttyACM0` was never opened or flashed
- No erase, no NVS erase, no efuse
- `--flash-freq 80mhz`; all 36 boots printed `Boot SPI Speed : 80MHz`
- Known-answer checks: 90 per run, 270 total, 0 failures, all 36 runs `ok`
- Images built earlier the same morning into `~/rind-bench/wt/hw-a8c8bf4b/esp32/images/`
- Raw capture: `~/rind-bench/run-d6ca9a75-eb053e78-out/raw/` (copied here under `raw/`)
- Run window: 07:27:07–07:57:12 PST (America/Los_Angeles)

## Gaps file

`gaps-staging.toml` was used only so `cargo xtask report` could write RESULTS.md. It is **not** a published gap verdict and is not included in this tree. The owner has not decided these gaps for this pin. The "Known gaps" section in RESULTS.md was generated from that staging file.

## Older trees

Nothing previously under `results/` was moved, overwritten, or deleted. This directory is new.
