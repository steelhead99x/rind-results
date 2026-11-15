#!/bin/sh
cd "$(dirname "$0")"
while pgrep -f "runall.sh" >/dev/null; do sleep 5; done
./run.sh 1 ring
./run.sh 2 ring
espflash flash --port /dev/ttyUSB0 --non-interactive -B 921600 --flash-size 8mb images/rustcrypto-sha2compact.elf > logs/flash-sha2compact.log 2>&1
for r in 1 2; do timeout 900 ~/esp32-venv/bin/python capture.py /dev/ttyUSB0 logs/serial-rustcrypto-sha2compact-run$r.log 850; echo "sha2compact run$r done"; done
echo RUNALL2-FINISHED
