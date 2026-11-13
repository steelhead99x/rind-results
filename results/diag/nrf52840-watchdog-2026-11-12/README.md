<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840: the first image after Meshtastic resets at about 90 s

## The first-window failure

`run-1.log` and `run-1.jsonl` are the first attempt at the published set's aes_gcm (2026-11-12 12:06 PST): the first bench image flashed from the bootloader that the board's Meshtastic firmware had just entered. The board reset 89 s into the run, after 128 results, and came back in the UF2 bootloader; the capture saw "port lost: Broken pipe", and the second run found no port.

Every other image of that window ran clean, including the same aes_gcm image flashed again after the others (`results/raw-nrf52840/aes_gcm/`: three runs, 167 results each, image sha256 4e528f7b…, identical to the first).

## Confirmation, 2026-11-12 13:58-14:00 PST

`probe-20261112-1358/` is a first-image probe flashed straight out of Meshtastic's DFU (rind-hardware's `wdt-probe` feature). The start record and the probe record both reported:

- `wdt_runstatus` = 1 (`NRF_WDT->RUNSTATUS`)
- `wdt_crv` = 2,949,120, which is 90 × 32,768, a 90-second timeout
- `wdt_rren` = 0x1 (only RR[0] enabled)
- `wdt_config` = 0
- `reset_reason` = 0x4 (a software reset from the bootloader, not a watchdog reset)

The probe then reloaded RR[0] once a second for eight seconds (eight `feed` records), which kept the board alive, wrote `stop-feed`, and stopped feeding. The port dropped at 14:00:26 PST, 89 seconds after the last feed, matching the CRV. The leftover Meshtastic nRF52 hardware watchdog is the cause.

A started nRF52 watchdog cannot be stopped except by a watchdog, pin, or power-on reset: a software reset and the UF2 bootloader leave it running. The bootloader feeds it while it waits in DFU; the first app image has to keep feeding it or it fires after 90 s. A watchdog timeout is what clears it, which is why every later image in a window runs clean.

The bench firmware now snapshots `RUNSTATUS`/`CRV`/`RREN` at boot, prints them in the start record, and reloads every enabled RR register between timed regions (and while waiting for `rind-bench:go`) when the watchdog was running. Published relink images were not rebuilt; the probe let the leftover watchdog time out before those runs.

What it changes: nothing in the published numbers, which are all from complete runs.
