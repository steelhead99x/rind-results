//! Argon2id hashes under the core PMU: cycles, retired and speculated
//! instructions, front- and back-end stall cycles, per 1 KiB block.
//! Usage: argon2-pmu LABEL ROUNDS [SIZES...]   (sizes: 1 = 1MiB-3, 16 = 16MiB-1, 64 = 64MiB-1)
use std::hint::black_box;
use std::time::Instant;

use argon2::{Algorithm, Argon2, ParamsBuilder, Version};
use rind::argon2::{Params, Secret};

#[cfg(target_os = "linux")]
mod pmu {
    #[repr(C)]
    #[derive(Default)]
    struct Attr {
        type_: u32,
        size: u32,
        config: u64,
        sample_period: u64,
        sample_type: u64,
        read_format: u64,
        flags: u64,
        wakeup: u32,
        bp_type: u32,
        config1: u64,
        config2: u64,
        branch_sample_type: u64,
        sample_regs_user: u64,
        sample_stack_user: u32,
        clockid: i32,
        sample_regs_intr: u64,
        aux_watermark: u32,
        sample_max_stack: u16,
        reserved: u16,
    }
    // ARMv8 common events: CPU_CYCLES, INST_RETIRED, INST_SPEC, STALL_FRONTEND, STALL_BACKEND, L1D_CACHE_REFILL.
    pub const NAMES: [&str; 6] = ["cycles", "inst", "inst_spec", "stall_fe", "stall_be", "l1d_refill"];
    const EVENTS: [u64; 6] = [0x11, 0x08, 0x1b, 0x23, 0x24, 0x03];
    pub struct Group { fds: Vec<i32> }
    impl Group {
        pub fn new() -> Option<Self> {
            let mut fds = Vec::new();
            for (i, &e) in EVENTS.iter().enumerate() {
                let mut a = Attr { type_: 4, size: core::mem::size_of::<Attr>() as u32, config: e, ..Default::default() };
                a.read_format = 8; // PERF_FORMAT_GROUP
                // disabled (leader only), exclude_kernel, exclude_hv
                a.flags = (if i == 0 { 1 } else { 0 }) | (1 << 5) | (1 << 6);
                let leader = if i == 0 { -1 } else { fds[0] };
                let fd = unsafe { libc::syscall(libc::SYS_perf_event_open, &a as *const Attr, 0, -1, leader, 0) } as i32;
                if fd < 0 { eprintln!("perf_event_open {e:#x}: {}", std::io::Error::last_os_error()); return None; }
                fds.push(fd);
            }
            Some(Self { fds })
        }
        pub fn start(&self) { unsafe { libc::ioctl(self.fds[0], 0x2403, 1); libc::ioctl(self.fds[0], 0x2400, 1); } }
        pub fn stop(&self) -> [u64; 6] {
            unsafe { libc::ioctl(self.fds[0], 0x2401, 1); }
            let mut buf = [0u64; 7];
            let n = unsafe { libc::read(self.fds[0], buf.as_mut_ptr().cast(), 56) };
            assert_eq!(n, 56);
            assert_eq!(buf[0], 6);
            [buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]]
        }
    }
}

fn median(v: &mut Vec<f64>) -> f64 { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v[v.len() / 2] }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let label = &args[1];
    let rounds: usize = args[2].parse().unwrap();
    let sizes: Vec<u32> = if args.len() > 3 { args[3..].iter().map(|s| s.parse().unwrap()).collect() } else { vec![1] };
    #[cfg(target_os = "linux")]
    let pmu = pmu::Group::new();
    let secret = Secret::new(b"password").unwrap();
    let salt = b"somesalt";
    for mib in sizes {
        let (kib, t) = match mib { 1 => (1024, 3), 16 => (16 * 1024, 1), 64 => (64 * 1024, 1), _ => panic!() };
        let blocks = (kib * t) as f64;
        let params = Params::new(kib, t, 1).unwrap();
        let mut b = ParamsBuilder::new();
        b.m_cost(kib).t_cost(t).p_cost(1);
        let theirs = Argon2::new(Algorithm::Argon2id, Version::V0x13, b.build().unwrap());
        let mut out = [0u8; 32];
        let mut ours = [0u8; 32];
        secret.derive(params, salt, b"", b"", &mut ours).unwrap();
        theirs.hash_password_into(b"password", salt, &mut out).unwrap();
        assert_eq!(ours, out, "rind and RustCrypto disagree");
        // per impl: wall ns and counters per sample
        let mut wall = [Vec::new(), Vec::new()];
        let mut ctr: [Vec<[u64; 6]>; 2] = [Vec::new(), Vec::new()];
        for _ in 0..rounds {
            for which in 0..2 {
                #[cfg(target_os = "linux")]
                if let Some(p) = &pmu { p.start(); }
                let t0 = Instant::now();
                if which == 0 { secret.derive(params, salt, b"", b"", &mut out).unwrap(); } else { theirs.hash_password_into(b"password", salt, &mut out).unwrap(); }
                black_box(&out);
                let ns = t0.elapsed().as_nanos() as f64;
                #[cfg(target_os = "linux")]
                if let Some(p) = &pmu { ctr[which].push(p.stop()); }
                wall[which].push(ns);
            }
        }
        for which in 0..2 {
            let name = if which == 0 { "rind" } else { "rustcrypto" };
            let w = &mut wall[which];
            let mut ws = w.clone(); ws.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let med = median(&mut w.clone());
            let spread = (ws[ws.len() - 1] - ws[0]) / med * 100.0;
            print!("{label}\t{mib}MiB\t{name}\twall_ms={:.3}\tmin_ms={:.3}\tspread%={:.1}", med / 1e6, ws[0] / 1e6, spread);
            if !ctr[which].is_empty() {
                #[cfg(target_os = "linux")]
                for (k, n) in pmu::NAMES.iter().enumerate() {
                    let mut v: Vec<f64> = ctr[which].iter().map(|c| c[k] as f64 / blocks).collect();
                    print!("\t{n}/blk={:.1}", median(&mut v));
                }
                let mut ipc: Vec<f64> = ctr[which].iter().map(|c| c[1] as f64 / c[0] as f64).collect();
                let mut ghz: Vec<f64> = ctr[which].iter().zip(wall[which].iter()).map(|(c, w)| c[0] as f64 / w).collect();
                print!("\tipc={:.2}\tghz={:.2}", median(&mut ipc), median(&mut ghz));
            }
            println!();
        }
    }
}
