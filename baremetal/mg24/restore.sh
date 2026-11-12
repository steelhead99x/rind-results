#!/bin/sh
# Restore the EFR32MG24's original flash and verify by readback.
# Place backup.bin (the unpublished full-flash image) in this directory first.
set -e
cd "$(dirname "$0")"
C=EFR32MG24B220F1536IM48
echo "53b5c80d8bc284a281922a8ab8c937791fa8f5b62f4e210b7e57ac8317261b1b  backup.bin" | shasum -a 256 -c
probe-rs download --chip $C --binary-format bin --base-address 0x08000000 --verify backup.bin
probe-rs read --chip $C -o readback.bin -f binary b32 0x08000000 393216
echo "53b5c80d8bc284a281922a8ab8c937791fa8f5b62f4e210b7e57ac8317261b1b  readback.bin" | shasum -a 256 -c
probe-rs reset --chip $C
rm -f readback.bin
