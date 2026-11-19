#!/usr/bin/env bash
# Interleaved A/B on the MSI's ESP32-S3: each round flashes every variant once, in a rotating order.
# usage: run.sh <rounds> <bench> <variant:dir>...    (log dir: ~/tmp/tt/a-brief/board/<variant>/<bench>/run-N.log)
cd ~/tmp/tt/a-brief
rounds=$1; bench=$2; shift 2
variants=("$@")
P=/dev/ttyACM0
n=${#variants[@]}
for r in $(seq 1 $rounds); do
  for k in $(seq 0 $((n-1))); do
    v=${variants[$(( (k + r - 1) % n ))]}
    out=board/$v/$bench; mkdir -p $out
    [ -s $out/run-$r.log ] && grep -q '"type":"done"' $out/run-$r.log && continue
    (
      flock 9
      echo "$(date +%T) round $r $v $bench flash" >> board/run.log
      timeout 240 espflash flash --port $P --flash-freq 80mhz --non-interactive images/$v/$bench.elf >> board/flash-$v-$bench-$r.log 2>&1 || { echo "FLASH FAIL $v $bench $r" >> board/run.log; exit 0; }
      python3 capture.py $P $out/run-$r.log 900 >> board/run.log 2>&1
      echo "$(date +%T) captured $v $bench $r $(wc -l < $out/run-$r.log) lines" >> board/run.log
    ) 9>/tmp/esp32-board.lock
  done
done
echo "ALLDONE $bench" >> board/run.log
