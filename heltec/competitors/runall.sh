#!/bin/sh
cd "$(dirname "$0")"
./run.sh 1 ring
./run.sh 2 rind ring rustcrypto libcrux
espflash flash --port /dev/ttyUSB0 --non-interactive -B 921600 --flash-size 8mb images/rind+rustcrypto-icache32.elf > logs/flash-icache32.log 2>&1
for r in 1 2; do timeout 900 ~/esp32-venv/bin/python capture.py /dev/ttyUSB0 logs/serial-rind+rustcrypto-icache32-run$r.log 850; echo "icache32 run$r done"; done
echo RUNALL-FINISHED
