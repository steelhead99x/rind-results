<!-- SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0 -->
# Jetson Orin Nano, rind 98568332

`index.html` is 3 slides at 1280×720. Open it in a browser. Arrow keys or `#N`.

1. Title. 6× Cortex-A78AE r0p1, part `0xd42`, schedutil 729.6–1728 MHz, cpu5.
2. Published baseline. Fastest on 65 of 68 shared operations. The two ratios above 1.00× are sha256 64 B at 1.07× and xchacha20poly1305 32 B at 1.01×.
3. #82 and #79 have no Jetson time. This core keeps `_alt`.

## Sources

`results/jetson-orin-nano/RESULTS.md`, `NOTES.md`, and `facts.txt`.
#82 and #79: the pull bodies.
