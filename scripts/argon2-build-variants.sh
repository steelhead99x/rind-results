#!/bin/bash
# build-argon2.sh WORKDIR JOBS: builds rind-bench's argon2 bench for each variant tarball in /tmp
# from ONE source dir, extracted with fresh mtimes (tar -m) so cargo rebuilds what changed.
D=$1; J=$2; mkdir -p $D/bins
while [ -d $HOME/rind-work/.bench.lock ]; do sleep 30; done
export RUSTFLAGS="--cfg rind_cpu_disable" CARGO_TARGET_DIR=$D/target-argon2
for v in ${VARIANTS:-a0-before a1-umlal a2-prefetch}; do
  rm -rf $D/argon2-src && mkdir -p $D/argon2-src && tar xzmf /tmp/$v.tar.gz -C $D/argon2-src
  cd $D/argon2-src/rind-bench || exit 1
  echo "build $v $(date)"
  bin=$(nice -n 19 cargo bench --jobs $J --no-run --bench argon2 --message-format=json 2>/dev/null | python3 -c "import sys,json
for l in sys.stdin:
  m=json.loads(l)
  if m.get('reason')=='compiler-artifact' and m['target']['name']=='argon2' and m.get('executable'): print(m['executable'], m['fresh'])")
  echo "  -> $bin"; cp ${bin% *} $D/bins/argon2-$v
done
md5 $D/bins/*; echo "end $(date)"
