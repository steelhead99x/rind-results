/* Optiboot leaves the watchdog enabled when it jumps to the app.
   Disable it before main, or the 328P resets in a loop. */
#include <avr/wdt.h>
#include <avr/io.h>

void wdt_off(void) __attribute__((naked, used, section(".init3")));
void wdt_off(void) {
    MCUSR = 0;
    wdt_disable();
}
