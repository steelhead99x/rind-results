#!/bin/bash
# board-side. baseline 5 rounds then AB. core 3 timing.
cd ~/w-pi4-a; mkdir -p out-old; mv out/* out-old/ 2>/dev/null; O=out; B=~/w-pi4-a/bin; mkdir -p $O
meta(){ echo "$1 $(date +%T) temp=$(cat /sys/class/thermal/thermal_zone0/temp) thr=$(vcgencmd get_throttled) gov=$(cat /sys/devices/system/cpu/cpu3/cpufreq/scaling_governor) freq=$(cat /sys/devices/system/cpu/cpu3/cpufreq/scaling_cur_freq) load=$(cut -d' ' -f1-3 /proc/loadavg)" >> $1META; }
M(){ echo "$1 $(date +%T) temp=$(cat /sys/class/thermal/thermal_zone0/temp) thr=$(vcgencmd get_throttled) gov=$(cat /sys/devices/system/cpu/cpu3/cpufreq/scaling_governor) freq=$(cat /sys/devices/system/cpu/cpu3/cpufreq/scaling_cur_freq) load=$(cut -d' ' -f1-3 /proc/loadavg)" >> $META; }
warm(){ timeout 10 taskset -c 3 sh -c 'while :; do :; done'; }
S='(32B|quic-32B|quic-64B|quic-128B|quic-256B|quic-512B|64B|1200B|16KB)'
S2='(64B|1200B|16KB)'
run(){ bin=$1; shift; for f in "$@"; do taskset -c 3 $B/$bin --bench "$f" --warm-up-time 1 --measurement-time 2 --noplot 2>&1; done; }
META=$O/meta.txt
for r in 1 2 3 4 5; do
  while [ $(cat /sys/class/thermal/thermal_zone0/temp) -gt 72000 ]; do sleep 5; done
  warm; M "round $r start"
  { run sha2 "^sha(256|512)/(ring|aws-lc-rs|rind)/$S2\$"
    run hmac "^hmac-sha256/(ring|aws-lc-rs|rind)/$S2\$"
    run aes_gcm "^aes(128|256)-gcm/(ring|aws-lc-rs|rind)/$S\$" "^aes(128|256)-gcm-open/(ring|aws-lc-rs|rind)/$S\$"
    run chacha20poly1305 "^chacha20poly1305(-open)?/(ring|aws-lc-rs|rind)/$S\$"
    run x25519 "^x25519-(ecdh|keygen)/(ring|aws-lc-rs|rind)\$"
    run ed25519 "^ed25519-(sign|verify|keygen)/(ring|aws-lc-rs|rind)\$"
    run p256 "^p256-(ecdh|keygen|ecdsa-sign|ecdsa-verify)/(ring|aws-lc-rs|rind)\$"
    run mlkem "^mlkem768-(keygen|encaps|decaps)/(aws-lc-rs|rind)\$"
    run argon2 "^argon2id-16MiB-t1-p1/"
  } > $O/r$r.txt
  M "round $r end"
done
echo BASEDONE >> $META
touch ~/w-pi4-a/baseline.done
A=ab; mkdir -p $A; META=$A/meta.txt
V=("default:" "narrow:RIND_CPU_DISABLE_wide_mul=1" "wide:RIND_CPU_FORCE_wide_mul=1")
for r in 1 2 3 4 5; do
  while [ $(cat /sys/class/thermal/thermal_zone0/temp) -gt 72000 ]; do sleep 5; done
  warm; M "round $r start"
  for i in 0 1 2; do
    v=${V[$(( (i + r) % 3 ))]}; name=${v%%:*}; ev=${v#*:}
    warm
    { for spec in "82-x25519:^x25519-(ecdh|keygen)/(aws-lc-rs|rind)\$" "82-ed25519:^ed25519-(sign|verify|keygen)/(aws-lc-rs|rind)\$" "82-p256:^p256-(ecdh|keygen|ecdsa-sign|ecdsa-verify)/(aws-lc-rs|rind)\$"; do
        bin=${spec%%:*}; f=${spec#*:}
        env $ev taskset -c 3 $B/$bin --bench "$f" --warm-up-time 1 --measurement-time 2 --noplot 2>&1
      done; } > $A/n82-r$r-$name.txt
    M "  r$r $name done"
  done
  for a in main 163; do
    for k in "8192 3" "65536 3"; do set -- $k; KIB=$1 T=$2 N=9 taskset -c 3 $B/argon-$a >> $A/argon-r$r-$a.txt 2>&1; done
  done
  M "round $r end"
done
taskset -c 3 $B/sweep > $A/sweep.txt 2>&1
echo ABDONE >> $META
