#!/bin/bash
# lane-q8.sh: build det-pmu harness variants, then interleaved PMU rounds under the bench lock.
export PATH=$HOME/.cargo/bin:$PATH
cd $HOME/rind-work/lane-pi5
L=$HOME/rind-work/.bench.lock
waitfree() { while [ -d $L ]; do sleep 30; done; }
VARS="narrow:narrow: detect:detect: nist:nist: det2:det2: narrow-a6:narrow:-Cllvm-args=-align-all-functions=6 detect-a6:detect:-Cllvm-args=-align-all-functions=6"
mkdir -p pmu8
for s in $VARS; do IFS=: read v t f <<< "$s"
  waitfree
  d=det-pmu-$v; rm -rf $d/src; mkdir -p $d; cp -r det-pmu/src $d/; sed "s|TREE|$t|" det-pmu/Cargo.toml > $d/Cargo.toml
  [ -f $d/Cargo.lock ] || cp det-pmu/Cargo.lock $d/ 2>/dev/null
  echo "build $v $(date)"
  (cd $d && RUSTFLAGS="--cfg rind_cpu_disable $f" nice -n 19 cargo build --release --jobs 3 --offline 2>&1 | tail -2)
  cp $d/target/release/det-pmu pmu8/$v
done
md5sum pmu8/*
for v in narrow detect nist det2 narrow-a6 detect-a6; do echo "== $v"; nm pmu8/$v | grep -E "p256_scalarmulbase(_narrow)?18p256_scalarmulbase$|p256_scalarmul_narrow14|p256_montjmixadd_narrow16|p256_montjdouble16|bignum_montmul_p256|bignum_montsqr_p256" ; done
echo "builds done $(date)"
R=7; set -- narrow detect nist det2 narrow-a6 detect-a6
for r in $(seq 1 $R); do
  until mkdir $L 2>/dev/null; do sleep 30; done; echo "pi5-lane (series 2 PMU r$r) $(date)" > $L/owner
  echo "# r$r $(date +%T) load=$(cut -d" " -f1-3 /proc/loadavg) $(vcgencmd measure_temp) $(vcgencmd get_throttled) freq=$(cat /sys/devices/system/cpu/cpu3/cpufreq/scaling_cur_freq)"
  order="$*"; for v in $order; do taskset -c 3 pmu8/$v $v 9 2000; done
  echo "# r$r end $(date +%T) $(vcgencmd measure_temp) $(vcgencmd get_throttled)"
  rm -rf $L
  set -- "${@:2}" "$1"
  sleep 15
done
echo "q8 end $(date)"
