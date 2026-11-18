. ~/.cargo/env
cd ~/bench; mkdir -p out/sphincs/raw
for w in before after; do
  d=~/bench/wt-pr143-$w
  (cd $d && CARGO_TARGET_DIR=~/bench/target-pr143-$w cargo bench -p <bench crate> --bench slhdsa --no-run 2>&1 | tail -3)
done
echo BUILT
# C reference: 7 repeats each variant/param, pinned to cpu 2
for r in 1 2 3 4 5 6 7; do for v in native haswell; do for p in sphincs-shake-128f sphincs-shake-128s sphincs-shake-256f; do
  taskset -c 2 out/sphincs/bench-$v-$p > out/sphincs/raw/c-$v-$p-r$r.txt 2>&1
done; done; done
echo C_DONE
# rind: interleaved before/after, 2 rounds, SHAKE sets only
for r in 1 2; do for w in before after; do
  d=~/bench/wt-pr143-$w
  (cd $d && CARGO_TARGET_DIR=~/bench/target-pr143-$w taskset -c 2 cargo bench -p <bench crate> --bench slhdsa -- --noplot --warm-up-time 1 --measurement-time 5 "shake.*/rind$" > ~/bench/out/sphincs/raw/rind-$w-r$r.txt 2>&1)
done; done
echo ALL_DONE
