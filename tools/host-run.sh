#!/bin/sh
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# Runs rind's own `admin/bench-report run` on this Linux host, outside Nix:
# rind's pinned stable toolchain through rustup (RUSTUP_TOOLCHAIN), with the
# dev-shell check satisfied by hand, pinned with taskset to one physical
# core (both its SMT threads), and the host's power settings recorded.
# Usage: tools/host-run.sh RIND_CHECKOUT HOST_NAME OUT_JSON [CPUS]
set -eu
rind=$(cd "$1" && pwd); host=$2; out=$(realpath -m "$3"); cpus=${4:-2,3}
toolchain=$(grep -o 'rust-bin\.stable\."[0-9.]*"' "$rind/flake.nix" | grep -o '[0-9][0-9.]*')
rustup toolchain install "$toolchain" --profile minimal >/dev/null
mkdir -p "$(dirname "$out")"
facts=${out%.json}.facts.txt
{
    echo "date: $(date -u +%FT%TZ)"
    echo "kernel: $(uname -srvm)"
    grep -m1 "model name" /proc/cpuinfo
    echo "governor: $(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null || echo ?)"
    echo "cpufreq boost: $(cat /sys/devices/system/cpu/cpufreq/boost 2>/dev/null || echo ?)"
    echo "energy_performance_preference: $(cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference 2>/dev/null || echo ?)"
    echo "scaling driver: $(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_driver 2>/dev/null || echo ?)"
    echo "pinned to CPUs: $cpus"
    echo "load: $(cat /proc/loadavg)"
    echo "rustc: $(RUSTUP_TOOLCHAIN=$toolchain rustc -V)"
} > "$facts"
cd "$rind"
commit=$(git rev-parse HEAD)
# rind refuses a host where other work keeps more than 10% of the CPUs busy.
# BUSY_SHARE=0.2 (say) relaxes that on a host with a resident load you may
# not stop; the edit is reverted afterwards, recorded in the facts file, and
# bench-report still marks the run "noisy" if the load during it exceeds the
# relaxed limit.
if [ -n "${BUSY_SHARE:-}" ]; then
    sed -i "s/^BUSY_SHARE = .*/BUSY_SHARE = $BUSY_SHARE/" admin/bench-report
    trap 'git checkout -- admin/bench-report' EXIT
    echo "BUSY_SHARE relaxed to $BUSY_SHARE (rind: 0.1)" >> "$facts"
fi
IN_RIND_DEV_SHELL=1 RUSTUP_TOOLCHAIN=$toolchain taskset -c "$cpus" python3 admin/bench-report run \
    --host "$host" --commit "$commit" --jobs "$(nproc)" \
    --target-dir "${CARGO_TARGET_DIR:-target/bench-report}" --out "$out"
