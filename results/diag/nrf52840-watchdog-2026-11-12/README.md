<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# nRF52840: the first image after Meshtastic resets at about 90 s

`run-1.log` and `run-1.jsonl` are the first attempt at the published set's aes_gcm (2026-11-12 12:06 PST): the first bench image flashed from the bootloader that the board's Meshtastic firmware had just entered. The board reset 89 s into the run, after 128 results, and came back in the UF2 bootloader; the capture saw "port lost: Broken pipe", and the second run found no port.

Every other image of the window ran clean, including the same aes_gcm image flashed again after the others (`results/raw-nrf52840/aes_gcm/`: three runs, 167 results each, image sha256 4e528f7b…, identical to the first).

The likely cause: Meshtastic's nRF52 firmware starts the hardware watchdog, which nothing but a power-on or pin reset stops; the bootloader feeds it while it waits in DFU, but the first image to run doesn't, so it fires about 90 s in; the image's reset path then sends the board to the bootloader. Images flashed after that run from a reset that cleared it. Not yet confirmed by reading `NRF_WDT->RUNSTATUS` from the firmware.

What it changes: nothing in the published numbers, which are all from complete runs. For future windows, run-all-nrf flashes the first image twice, or the firmware feeds the watchdog while `RUNSTATUS` is set.
