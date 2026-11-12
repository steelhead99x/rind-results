#!/bin/bash
# rustls bench only (first pass couldn't find its test keys); same args, cleanup after.
export PATH="/opt/homebrew/bin:$PATH"
S=<phone-serial>; P=/data/local/tmp/rind
D="$HOME/rind-bench-run/pixel2"; rm -rf "$D"; mkdir -p "$D"; cd "$D" || exit 1
adb -s $S get-state || { echo no-device > status; exit 1; }
adb -s $S shell dumpsys battery > battery-before.txt
mkdir pkg && tar xzf ../pixel-pkg2.tar.gz -C pkg
adb -s $S shell "rm -rf $P && mkdir -p $P" && adb -s $S push pkg/bin pkg/src pkg/run2.sh $P/ > push.log 2>&1 || { echo push-failed > status; exit 1; }
adb -s $S shell "chmod 755 $P/run2.sh $P/bin/*; cd $P && sh ./run2.sh > $P/run.log 2>&1"
adb -s $S shell dumpsys battery > battery-after.txt
for f in run.log started finished stat-before stat-after criterion.tar; do adb -s $S pull $P/$f . >/dev/null 2>&1; done
tar czf pixel-result2.tar.gz run.log started finished stat-before stat-after criterion.tar battery-before.txt battery-after.txt
adb -s $S shell "rm -rf $P"; adb -s $S shell "ls $P" > cleanup-check.txt 2>&1
rm -rf pkg; echo done > status; echo "end $(date)"
