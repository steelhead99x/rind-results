//! P-256 keygen / ECDH / ECDSA sign under the core PMU (series 2 delta).
//! Usage: det-pmu LABEL ROUNDS ITERS
use std::hint::black_box;
use std::time::Instant;

mod pmu {
    #[repr(C)]
    #[derive(Default)]
    struct Attr {
        type_: u32, size: u32, config: u64, sample_period: u64, sample_type: u64, read_format: u64,
        flags: u64, wakeup: u32, bp_type: u32, config1: u64, config2: u64, branch_sample_type: u64,
        sample_regs_user: u64, sample_stack_user: u32, clockid: i32, sample_regs_intr: u64,
        aux_watermark: u32, sample_max_stack: u16, reserved: u16,
    }
    // ARMv8 common events: CPU_CYCLES, INST_RETIRED, BR_MIS_PRED, L1I_CACHE_REFILL, STALL_FRONTEND, L1I_TLB_REFILL.
    pub const NAMES: [&str; 6] = ["cycles", "inst", "br_mis", "l1i_refill", "stall_fe", "l1i_tlb_refill"];
    const EVENTS: [u64; 6] = [0x11, 0x08, 0x10, 0x01, 0x23, 0x02];
    pub struct Group { fds: Vec<i32> }
    impl Group {
        pub fn new() -> Self {
            let mut fds = Vec::new();
            for (i, &e) in EVENTS.iter().enumerate() {
                let mut a = Attr { type_: 4, size: core::mem::size_of::<Attr>() as u32, config: e, ..Default::default() };
                a.read_format = 8;
                a.flags = (if i == 0 { 1 } else { 0 }) | (1 << 5) | (1 << 6);
                let leader = if i == 0 { -1 } else { fds[0] };
                let fd = unsafe { libc::syscall(libc::SYS_perf_event_open, &a as *const Attr, 0, -1, leader, 0) } as i32;
                assert!(fd >= 0, "perf_event_open {e:#x}: {}", std::io::Error::last_os_error());
                fds.push(fd);
            }
            Self { fds }
        }
        pub fn start(&self) { unsafe { libc::ioctl(self.fds[0], 0x2403, 1); libc::ioctl(self.fds[0], 0x2400, 1); } }
        pub fn stop(&self) -> [u64; 6] {
            unsafe { libc::ioctl(self.fds[0], 0x2401, 1); }
            let mut buf = [0u64; 7];
            let n = unsafe { libc::read(self.fds[0], buf.as_mut_ptr().cast(), 56) };
            assert_eq!(n, 56);
            [buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]]
        }
    }
}

const PUBLIC_KEY: &[u8; 65] = b"\x04\
\x62\xd5\xbd\x33\x72\xaf\x75\xfe\x85\xa0\x40\x71\x5d\x0f\x50\x24\x28\xe0\x70\x46\x86\x8b\x0b\xfd\xfa\x61\xd7\x31\xaf\xe4\x4f\x26\
\xac\x33\x3a\x93\xa9\xe7\x0a\x81\xcd\x5a\x95\xb5\xbf\x8d\x13\x99\x0e\xb7\x41\xc8\xc3\x88\x72\xb4\xa0\x7d\x27\x5a\x01\x4e\x30\xcf";

fn median(v: &mut [f64]) -> f64 { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v[v.len() / 2] }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (label, rounds, iters): (&str, usize, usize) = (&a[1], a[2].parse().unwrap(), a[3].parse().unwrap());
    let pmu = pmu::Group::new();
    use rind::agreement::*;
    use rind::digest;
    use rind::signature::{EcdsaKeyPair, sign};
    let key = EcdsaKeyPair::generate(&sign::ECDSA_P256_SHA256_FIXED).unwrap();
    let dg = digest::digest(&digest::SHA256, b"hello, world");
    let ops: [(&str, &dyn Fn()); 3] = [
        ("p256-keygen", &|| { let k = PrivateKey::generate(&ECDH_P256).unwrap(); black_box(k.public_key()); }),
        ("p256-ecdh", &|| { let k = PrivateKey::generate(&ECDH_P256).unwrap(); black_box(k.public_key()); black_box(k.agree(PUBLIC_KEY).unwrap()); }),
        ("p256-ecdsa-sign", &|| { black_box(key.sign_digest(&dg).unwrap()); }),
    ];
    for (name, f) in ops {
        for _ in 0..iters / 4 { f(); }
        let mut per: Vec<Vec<f64>> = vec![Vec::new(); 7];
        for _ in 0..rounds {
            let t = Instant::now();
            pmu.start();
            for _ in 0..iters { f(); }
            let c = pmu.stop();
            per[6].push(t.elapsed().as_nanos() as f64 / iters as f64);
            for i in 0..6 { per[i].push(c[i] as f64 / iters as f64); }
        }
        let mut line = format!("{label}\t{name}\tns={:.1}", median(&mut per[6]));
        for i in 0..6 { line += &format!("\t{}={:.1}", pmu::NAMES[i], median(&mut per[i])); }
        println!("{line}");
    }
}
