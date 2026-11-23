#!/usr/bin/env bash
# 3 rounds x (sha2 hmac hkdf) x (main, pr), alternating main and PR. Flash, then a fresh reset boot captured to a log.
cd ~/tmp/tt/pr94hw/hw/esp32
P=/dev/ttyACM0
for round in 1 2 3; do
 for bench in sha2 hmac hkdf; do
  for pair in main-4fbb7188:4fbb7188 pr94-942bb48a:942bb48a; do
   label=${pair%%:*}; dir=${pair##*:}
   out=~/tmp/tt/pr94hw/raw/$label/$bench; mkdir -p $out
   [ -s $out/run-$round.log ] && grep -q '"type":"done"' $out/run-$round.log && continue
   echo "$(date +%T) round $round $label $bench flash" >> ~/tmp/tt/pr94hw/runall.log
   timeout 180 espflash flash --port $P --flash-freq 80mhz --non-interactive images-pr94/$dir/$bench.elf >> ~/tmp/tt/pr94hw/flash-$label-$bench-$round.log 2>&1 || { echo "FLASH FAIL" >> ~/tmp/tt/pr94hw/runall.log; continue; }
   python3 ~/tmp/tt/pr94hw/capture.py $P $out/run-$round.log 1800 >> ~/tmp/tt/pr94hw/runall.log 2>&1
   echo "$(date +%T) captured $label $bench $round $(wc -l < $out/run-$round.log) lines" >> ~/tmp/tt/pr94hw/runall.log
  done
 done
done
echo ALLDONE >> ~/tmp/tt/pr94hw/runall.log
