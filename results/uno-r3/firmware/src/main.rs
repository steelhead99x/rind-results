#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

mod hash_wrap;
mod rind_port;

use core::panic::PanicInfo;
use hash_wrap::{
    bench_byte, chacha20_apply, chacha20_pattern_checksum, poly1305_pattern, sha256_oneshot,
    sha256_pattern, sha512_oneshot, sha512_pattern,
};
use rind_port::poly1305::Poly1305;

const F_CPU: u32 = 16_000_000;
const RUNS: u8 = 5;

static mut SHA512_KAT_OK: bool = false;

// USART0
const UCSR0A: *mut u8 = 0xC0 as *mut u8;
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
const UBRR0H: *mut u8 = 0xC5 as *mut u8;
const UDR0: *mut u8 = 0xC6 as *mut u8;
const UDRE0: u8 = 1 << 5;
const U2X0: u8 = 1 << 1;
const TXEN0: u8 = 1 << 3;
const UCSZ01: u8 = 1 << 2;
const UCSZ00: u8 = 1 << 1;

// Timer1, no prescale, 1 tick = 1 CPU cycle at 16 MHz.
const TCCR1A: *mut u8 = 0x80 as *mut u8;
const TCCR1B: *mut u8 = 0x81 as *mut u8;
const TCNT1: *mut u16 = 0x84 as *mut u16;
const TIFR1: *mut u8 = 0x36 as *mut u8;
const TIMSK1: *mut u8 = 0x6F as *mut u8;
const SREG: *mut u8 = 0x5F as *mut u8;
const TOIE1: u8 = 1 << 0;
const CS10: u8 = 1 << 0;

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const LED: u8 = 1 << 5; // Arduino pin 13

unsafe extern "C" {
    static mut t1_ovf: u16;
}

#[inline(always)]
unsafe fn wr8(p: *mut u8, v: u8) {
    unsafe { core::ptr::write_volatile(p, v); }
}
#[inline(always)]
unsafe fn rd8(p: *mut u8) -> u8 {
    unsafe { core::ptr::read_volatile(p) }
}
#[inline(always)]
unsafe fn wr16(p: *mut u16, v: u16) {
    unsafe { core::ptr::write_volatile(p, v); }
}
#[inline(always)]
unsafe fn rd16(p: *mut u16) -> u16 {
    unsafe { core::ptr::read_volatile(p) }
}

fn uart_init() {
    unsafe {
        // 115200 8N1, U2X, UBRR=16 at 16 MHz (actual 117647, 2.1% error).
        wr8(UBRR0H, 0);
        wr8(UBRR0L, 16);
        wr8(UCSR0A, U2X0);
        wr8(UCSR0C, UCSZ01 | UCSZ00);
        wr8(UCSR0B, TXEN0);
    }
}

fn putc(c: u8) {
    unsafe {
        while rd8(UCSR0A) & UDRE0 == 0 {}
        wr8(UDR0, c);
    }
}

fn puts(s: &str) {
    for b in s.as_bytes() {
        putc(*b);
    }
}

fn put_u32(mut n: u32) {
    if n == 0 {
        putc(b'0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        putc(buf[i]);
    }
}

fn put_hex(bytes: &[u8]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for b in bytes {
        putc(HEX[(b >> 4) as usize]);
        putc(HEX[(b & 0x0f) as usize]);
    }
}

fn sei() {
    unsafe {
        core::arch::asm!("sei", options(nomem, nostack, preserves_flags));
    }
}

fn cli() {
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }
}

fn timer_start() {
    unsafe {
        wr8(TCCR1A, 0);
        wr8(TCCR1B, 0);
        wr16(TCNT1, 0);
        core::ptr::write_volatile(&raw mut t1_ovf, 0);
        wr8(TIFR1, 1 << 0); // clear TOV1
        wr8(TIMSK1, TOIE1);
        wr8(TCCR1B, CS10);
    }
    sei();
}

fn cycles_now() -> u32 {
    // Snapshot under cli so the ISR cannot race. If TOV1 is pending (ISR
    // not yet entered) and TCNT1 has already wrapped, count that overflow
    // in the snapshot only — do not write t1_ovf, or we would double-count
    // when the ISR later runs.
    unsafe {
        cli();
        let ovf = core::ptr::read_volatile(&raw const t1_ovf);
        let t = rd16(TCNT1);
        let pending = rd8(TIFR1) & 1 != 0;
        sei();
        let ovf = if pending && t < 0x8000 { ovf.wrapping_add(1) } else { ovf };
        ((ovf as u32) << 16) | (t as u32)
    }
}

fn cycles_to_us(cycles: u32) -> u32 {
    // 16 MHz: 16 cycles per microsecond, exact.
    cycles / 16
}

fn led_init() {
    unsafe {
        wr8(DDRB, rd8(DDRB) | LED);
    }
}

fn led_toggle() {
    unsafe {
        wr8(PORTB, rd8(PORTB) ^ LED);
    }
}

fn report_kat(name: &str, pass: bool) {
    puts("KAT ");
    puts(name);
    puts(if pass { " PASS\r\n" } else { " FAIL\r\n" });
}

fn report_bench(op: &str, cycles: u32, digest: &[u8]) {
    puts("BENCH ");
    puts(op);
    puts(" cycles=");
    put_u32(cycles);
    puts(" us=");
    put_u32(cycles_to_us(cycles));
    if !digest.is_empty() {
        puts(" digest=");
        put_hex(digest);
    }
    puts("\r\n");
}

fn eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

unsafe extern "C" {
    fn kat_eq_sha256_empty(got: *const u8) -> u8;
    fn kat_eq_sha256_abc(got: *const u8) -> u8;
    fn kat_eq_sha512_abc(got: *const u8) -> u8;
    fn kat_eq_chacha(got: *const u8) -> u8;
    fn kat_eq_poly(got: *const u8) -> u8;
    fn kat_load_chacha_in(dst: *mut u8);
    fn kat_load_poly_key(dst: *mut u8);
    fn kat_load_poly_msg(dst: *mut u8);
}

fn kats() -> bool {
    let mut all = true;

    let empty = sha256_oneshot(b"");
    let p = unsafe { kat_eq_sha256_empty(empty.as_ptr()) } != 0;
    report_kat("s256-empty", p);
    all &= p;

    let abc = sha256_oneshot(b"abc");
    let p = unsafe { kat_eq_sha256_abc(abc.as_ptr()) } != 0;
    report_kat("s256-abc", p);
    all &= p;

    let abc512 = sha512_oneshot(b"abc");
    let p = unsafe { kat_eq_sha512_abc(abc512.as_ptr()) } != 0;
    report_kat("s512-abc", p);
    unsafe { SHA512_KAT_OK = p; }
    all &= p;

    let key: [u8; 32] = core::array::from_fn(|i| i as u8);
    let nonce = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00];
    let mut buf = [0u8; 114];
    unsafe { kat_load_chacha_in(buf.as_mut_ptr()) }
    chacha20_apply(&key, &nonce, 1, &mut buf);
    let p = unsafe { kat_eq_chacha(buf.as_ptr()) } != 0;
    report_kat("chacha-rfc", p);
    all &= p;

    let mut pkey = [0u8; 32];
    let mut pmsg = [0u8; 34];
    unsafe {
        kat_load_poly_key(pkey.as_mut_ptr());
        kat_load_poly_msg(pmsg.as_mut_ptr());
    }
    let mut poly = Poly1305::new(&pkey);
    poly.add_bytes(&pmsg);
    let tag = poly.finish();
    let p = unsafe { kat_eq_poly(tag.as_ptr()) } != 0;
    report_kat("poly-rfc", p);
    all &= p;

    all
}

fn bench_sha256(op: &str, len: u32) {
    // Prime I-cache / flash (first call not timed).
    let _ = sha256_pattern(len, bench_byte);
    let t0 = cycles_now();
    let d = sha256_pattern(len, bench_byte);
    let t1 = cycles_now();
    report_bench(op, t1.wrapping_sub(t0), &d);
}

fn bench_sha512(op: &str, len: u32) {
    let _ = sha512_pattern(len, bench_byte);
    let t0 = cycles_now();
    let d = sha512_pattern(len, bench_byte);
    let t1 = cycles_now();
    report_bench(op, t1.wrapping_sub(t0), &d);
}

fn bench_chacha20(op: &str, len: u32) {
    let key: [u8; 32] = core::array::from_fn(|i| i as u8);
    let nonce = [0u8; 12];
    let _ = chacha20_pattern_checksum(&key, &nonce, 1, len, bench_byte);
    let t0 = cycles_now();
    let sum = chacha20_pattern_checksum(&key, &nonce, 1, len, bench_byte);
    let t1 = cycles_now();
    let d = sum.to_le_bytes();
    report_bench(op, t1.wrapping_sub(t0), &d);
}

fn bench_poly1305(op: &str, len: u32) {
    let key: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(17));
    let _ = poly1305_pattern(&key, len, bench_byte);
    let t0 = cycles_now();
    let tag = poly1305_pattern(&key, len, bench_byte);
    let t1 = cycles_now();
    report_bench(op, t1.wrapping_sub(t0), &tag);
}

fn bench_chacha20poly1305(op: &str, len: u32) {
    // One-shot composed of rind generic ChaCha20 + Poly1305, not the full
    // AEAD construction (no 64-bit length block / RFC 8439 AAD framing) —
    // timed as "cipher then MAC of the same length" so the costs add honestly.
    let key: [u8; 32] = core::array::from_fn(|i| i as u8);
    let nonce = [0u8; 12];
    let mac_key: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_add(1));
    let _ = chacha20_pattern_checksum(&key, &nonce, 1, len, bench_byte);
    let _ = poly1305_pattern(&mac_key, len, bench_byte);
    let t0 = cycles_now();
    let sum = chacha20_pattern_checksum(&key, &nonce, 1, len, bench_byte);
    let tag = poly1305_pattern(&mac_key, len, bench_byte);
    let t1 = cycles_now();
    let mut d = [0u8; 20];
    d[..4].copy_from_slice(&sum.to_le_bytes());
    d[4..].copy_from_slice(&tag);
    report_bench(op, t1.wrapping_sub(t0), &d);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i16 {
    // Small delay so the 16U2 can reopen CDC after the bootloader reset.
    // ~200 ms at 16 MHz, rough busy loop.
    for _ in 0..200 {
        for _ in 0..4000u16 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }

    led_init();
    uart_init();
    timer_start();

    puts("RIND UNO-R3\r\n");
    puts("MCU atmega328p F_CPU ");
    put_u32(F_CPU);
    puts("\r\n");
    puts("SRC c08c91ea generic\r\n");

    let kat_ok = kats();
    if !kat_ok {
        puts("KAT FAIL\r\n");
    } else {
        puts("KAT PASS\r\n");
    }

    // Known short delay to confirm Timer1 + overflow accounting.
    let t0 = cycles_now();
    for _ in 0..1000u16 {
        unsafe { core::arch::asm!("nop"); }
    }
    let t1 = cycles_now();
    puts("TIMER nop1000 cycles=");
    put_u32(t1.wrapping_sub(t0));
    puts("\r\n");

    for run in 1..=RUNS {
        led_toggle();
        puts("BEGIN run=");
        put_u32(run as u32);
        puts("\r\n");
        bench_sha256("sha256/32B", 32);
        bench_sha256("sha256/1350B", 1350);
        bench_sha256("sha256/16KB", 16 * 1024);
        if unsafe { SHA512_KAT_OK } {
            bench_sha512("sha512/32B", 32);
            bench_sha512("sha512/1350B", 1350);
            bench_sha512("sha512/16KB", 16 * 1024);
        } else {
            puts("SKIP sha512 (kat fail)\r\n");
        }
        bench_chacha20("chacha20/64B", 64);
        bench_chacha20("chacha20/1350B", 1350);
        bench_poly1305("poly1305/16B", 16);
        bench_poly1305("poly1305/1350B", 1350);
        bench_chacha20poly1305("chacha20+poly1305/1350B", 1350);
        puts("END run=");
        put_u32(run as u32);
        puts("\r\n");
    }

    puts("SKIP x25519\r\n");
    puts("SKIP p256/ed25519/mlkem/aes/sha3\r\n");
    puts("DONE\r\n");
    loop {
        led_toggle();
        for _ in 0..1000 {
            for _ in 0..4000u16 {
                unsafe {
                    core::arch::asm!("nop");
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Best-effort, may itself fail if UART is not up.
    puts("PANIC\r\n");
    loop {
        unsafe {
            wr8(PORTB, rd8(PORTB) ^ LED);
            for _ in 0..200 {
                for _ in 0..2000u16 {
                    core::arch::asm!("nop");
                }
            }
        }
    }
}
