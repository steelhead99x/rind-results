#!/bin/bash
# SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
# cargo's target runner on the Jetson when its clock is unset and it is
# offline: runs one built binary (a bench, rind-cpuid) under libfaketime with
# only the wall clock moved by FAKETIME_OFFSET seconds, so the rustls benches'
# certificates are valid. The monotonic clock criterion times with is left
# alone. cargo and rustc never run under it: rustc deadlocked under the
# preload, and a moved wall clock reran openssl-sys's build script.
D="$HOME/rind-bench-orin"
exec env LD_PRELOAD="$D/faketime/usr/lib/aarch64-linux-gnu/faketime/libfaketimeMT.so.1" \
  FAKETIME="+${FAKETIME_OFFSET:?}s" FAKETIME_DONT_FAKE_MONOTONIC=1 FAKETIME_DONT_RESET=1 "$@"
