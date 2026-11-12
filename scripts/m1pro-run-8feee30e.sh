#!/bin/bash
D=$HOME/rind-work; cd $D/bench-main || exit 1
export PATH="$HOME/.rustup/toolchains/1.98.1-aarch64-apple-darwin/bin:/opt/homebrew/bin:$PATH" HOSTRUN_SKIP_BUILD_COST=1 OPENSSL_DIR=/opt/homebrew/opt/openssl@3
echo "start $(date) free=$(df -g $HOME | awk 'NR==2{print $4}')G"
python3 host-run.py --host m1pro --commit 8feee30ed443e3329e246e85840a7b9dc07f0ab6 --jobs 8 --target-dir $D/target-bench --out $D/m1pro-8feee30e.json
echo "end $(date) rc=$?"
