#!/bin/sh
# Usage: run.sh <run-number>. Flashes each image to the Heltec on /dev/ttyUSB0 and captures serial.
set -u
cd "$(dirname "$0")"
run=$1; shift; for lib in ${*:-rind ring rustcrypto libcrux}; do
  espflash flash --port /dev/ttyUSB0 --non-interactive -B 921600 --flash-size 8mb images/$lib.elf > logs/flash-$lib-run$run.log 2>&1 || { echo "flash $lib failed"; continue; }
  timeout 900 ~/esp32-venv/bin/python capture.py /dev/ttyUSB0 logs/serial-$lib-run$run.log 850
  echo "$lib run$run done: $(grep -c '^BENCH' logs/serial-$lib-run$run.log) bench lines"
done
