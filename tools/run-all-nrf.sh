#!/bin/sh
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# Flashes each rind-bench nRF52840 image in turn through the board's UF2
# bootloader and captures RUNS runs of it.
# Usage: tools/run-all-nrf.sh IMAGES_DIR [bench...]
#   env: RUNS (default 3), OUT (default results/raw-nrf52840),
#        DRIVE (default /run/media/$USER/T1000-E, the bootloader's drive),
#        PYTHON (default python3 with pyserial)
set -eu
images=$1; shift
runs=${RUNS:-3}
out=${OUT:-results/raw-nrf52840}
drive=${DRIVE:-/run/media/$USER/T1000-E}
python=${PYTHON:-python3}
port=/dev/serial/by-id/usb-rind-bench_rind-bench_nRF52840_rind-bench-if00
benches=${*:-$(cd "$images" && ls *.uf2 | sed 's/\.uf2$//')}

wait_for() { # test, seconds
    i=0
    while ! eval "$1"; do
        i=$((i + 1)); [ $i -le $(($2 * 10)) ] || return 1; sleep 0.1
    done
}

for b in $benches; do
    echo "== $b"
    if [ ! -d "$drive" ] && [ -e "$port" ]; then
        # 1200 baud, then drop DTR: the firmware resets into the bootloader.
        "$python" -c "import serial,sys; s=serial.Serial(sys.argv[1],1200); s.dtr=False; s.close()" "$port" 2>/dev/null || true
    fi
    wait_for "[ -f '$drive/INFO_UF2.TXT' ]" 30 || { echo "$b: the bootloader drive did not appear"; exit 1; }
    # sync only this file: a global sync can wait minutes on other disks.
    cp "$images/$b.uf2" "$drive/" && { sync -f "$drive" 2>/dev/null || true; }
    wait_for "[ -e '$port' ]" 30 || { echo "$b: the firmware did not enumerate"; exit 1; }
    "$python" "$(dirname "$0")/capture_nrf.py" --port "$port" --runs "$runs" --out "$out/$b" || echo "$b: a run failed; see $out/$b/runs.json"
done
