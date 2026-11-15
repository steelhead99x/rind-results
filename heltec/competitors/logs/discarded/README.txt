serial-libcrux-run1.log (in logs/) was captured 00:18-00:25 PT while images/ring.elf was, by mistake,
a byte-identical copy of images/libcrux.elf (md5 e7f94b231efdcf287e6a74049e0b9f47): a ring rebuild had
gone to the wrong target dir. It is a complete libcrux run (START..ALL-DONE) and is used as libcrux run 1.
serial-libcrux-run1-interrupted-by-my-flash-attempt.log: a second libcrux capture cut off at 00:32 when
I started flashing ring while it was still running (chip put in download mode). Not used.
serial-ring-run1-attempt1-reset.log (in logs/): ring's first image; it reset (RTCWDT_RTC_RST) at
ChaCha20-Poly1305 1024 B, looping 18 times.
