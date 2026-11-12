#!/bin/sh
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# Flashes each rind-bench image in turn and captures RUNS runs of it.
# Usage: tools/run-all.sh IMAGES_DIR [bench...]
#   env: PORT (default <PORT>), RUNS (default 3), OUT (default results/raw),
#        FLASH_FREQ (default 80mhz), ESPFLASH (default espflash),
#        PYTHON (default python3 with pyserial)
# espflash writes the SPI flash clock into the image header at flash time and does not read
# sdkconfig: without --flash-freq it is 40 MHz, and the app keeps the bootloader's clock, so
# every row that misses the instruction cache runs at half the flash bandwidth the build asked for.
set -eu
images=$1; shift
port=${PORT:-<PORT>}
runs=${RUNS:-3}
out=${OUT:-results/raw}
espflash=${ESPFLASH:-espflash}
python=${PYTHON:-python3}
freq=${FLASH_FREQ:-80mhz}
benches=${*:-$(cd "$images" && ls *.elf | sed 's/\.elf$//')}
for b in $benches; do
    echo "== $b"
    "$espflash" flash --port "$port" --chip esp32s3 --non-interactive --flash-freq "$freq" "$images/$b.elf"
    "$python" "$(dirname "$0")/capture.py" --port "$port" --runs "$runs" --out "$out/$b" || echo "$b: a run failed; see $out/$b/runs.json"
done
