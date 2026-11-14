#include <stdint.h>
#ifdef __AVR__
#include <avr/interrupt.h>
volatile uint16_t t1_ovf;
ISR(TIMER1_OVF_vect) { t1_ovf++; }
#else
volatile uint16_t t1_ovf;
#endif
