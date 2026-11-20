#!/usr/bin/env python3
# Reset the ESP32-S3 (USB-Serial/JTAG) and log the serial output with a timestamp per line
# until the bench prints {"type":"done"} or the timeout passes. Raw output goes to <out>.log.
import sys, time, serial
port, out, timeout = sys.argv[1], sys.argv[2], float(sys.argv[3])
s = serial.Serial(port, 115200, timeout=0.2)
# USB-JTAG-Serial reset into normal boot: RTS (EN) low then high, DTR (BOOT) high.
s.dtr = False; s.rts = True; time.sleep(0.1); s.dtr = False; s.rts = False; time.sleep(0.05)
t0 = time.time(); buf = b""; done = False; checks = None
with open(out, "w") as f:
    while time.time() - t0 < timeout and not done:
        data = s.read(4096)
        if data:
            buf += data
            while b"\n" in buf:
                line, buf = buf.split(b"\n", 1)
                txt = line.decode("utf-8", "replace").rstrip("\r")
                f.write("%10.3f %s\n" % (time.time() - t0, txt)); f.flush()
                if '"type":"done"' in txt.replace(" ", ""): done = True
                if '"checks-done"' in txt: checks = txt
    if buf:
        f.write("%10.3f %s\n" % (time.time() - t0, buf.decode("utf-8", "replace")))
print("done" if done else "TIMEOUT", checks)
sys.exit(0 if done else 1)
