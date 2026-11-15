import serial, sys, time
p = serial.Serial(sys.argv[1], 115200, timeout=1)
# Reset via RTS (EN) with DTR high (IO0 not held low) -> normal boot.
p.dtr = False; p.rts = True; time.sleep(0.2); p.rts = False
out = open(sys.argv[2], "w")
deadline = time.time() + float(sys.argv[3])
while time.time() < deadline:
    line = p.readline().decode("utf-8", "replace")
    if line:
        out.write(line); out.flush()
        if "ALL-DONE" in line or "panic" in line.lower() or "Guru" in line or "RTCWDT_RTC_RST" in line:
            if "ALL-DONE" in line: break
            deadline = min(deadline, time.time() + 3)
