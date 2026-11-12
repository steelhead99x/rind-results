#!/bin/bash
# Runs on the desktop in ~/rind-bench-run/pixel. Waits for the desktop's own bench run
# to finish, then runs the cross-compiled benches on the phone over adb.
export PATH="/opt/homebrew/bin:$PATH"
S=<phone-serial>; P=/data/local/tmp/rind
D="$HOME/rind-bench-run/pixel"; mkdir -p "$D"; cd "$D" || exit 1
echo "waiting for desktop run $(date)"
while [ ! -f "$HOME/rind-bench-run/status" ]; do sleep 30; done
echo "desktop run finished: $(cat $HOME/rind-bench-run/status) $(date)"
sleep 60
adb -s $S get-state || { echo "ABORT: phone not connected"; echo no-device > status; exit 1; }
rm -rf pkg && mkdir pkg && tar xzf ../pixel-pkg.tar.gz -C pkg
adb -s $S shell dumpsys battery > battery-before.txt
adb -s $S shell getprop > getprop.txt
adb -s $S shell 'cat /proc/cpuinfo; uname -a' > cpuinfo.txt
adb -s $S shell "rm -rf $P && mkdir -p $P" && adb -s $S push pkg/bin pkg/run.sh $P/ > push.log 2>&1 || { echo "ABORT: push failed"; echo push-failed > status; exit 1; }
adb -s $S shell "chmod 755 $P/run.sh $P/bin/*; cd $P && nohup sh ./run.sh > $P/run.log 2>&1 &"
echo "started on phone $(date)"
while ! adb -s $S shell "test -f $P/done" 2>/dev/null; do
  sleep 60
  adb -s $S shell "tail -1 $P/run.log" 2>/dev/null | cut -c1-120 > progress.txt
  adb -s $S shell dumpsys battery 2>/dev/null | grep -E "temperature|level" | tr '\n' ' ' >> battery-during.txt; echo " $(date +%T)" >> battery-during.txt
  adb -s $S get-state >/dev/null 2>&1 || { echo "phone disconnected $(date)"; }
done
adb -s $S shell dumpsys battery > battery-after.txt
for f in run.log cpuid.json cpuid.err started finished stat-before stat-after criterion.tar; do adb -s $S pull $P/$f . >/dev/null 2>&1; done
tar czf pixel-result.tar.gz run.log cpuid.json cpuid.err started finished stat-before stat-after criterion.tar battery-before.txt battery-after.txt battery-during.txt getprop.txt cpuinfo.txt
adb -s $S shell "rm -rf $P"; adb -s $S shell "ls $P" > cleanup-check.txt 2>&1
rm -rf pkg
echo done > status; echo "end $(date)"
