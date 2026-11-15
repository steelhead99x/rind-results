#!/usr/bin/env bash
D=~/rind-results/gates-680-681
cd $D && git clone -q --recurse-submodules --branch ed25519-cctv-wipe-canaries-r2 https://github.com/steelhead99x/rind r2-a784b93 && cd r2-a784b93 && git checkout -q a784b93e3d12ba030aa35e26cccd3fca76bbc403 && [ "$(git rev-parse HEAD)" = a784b93e3d12ba030aa35e26cccd3fca76bbc403 ] && echo "verified $(git rev-parse HEAD) tree $(git rev-parse HEAD^{tree})" && exec $D/run-gates-692.sh r2-a784b93
