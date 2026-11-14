fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if !target.starts_with("avr") {
        return;
    }
    let src_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let gcc = std::env::var("AVR_GCC").unwrap_or_else(|_| "avr-gcc".into());

    for (name, file) in [("timer_ovf", "timer_ovf.c"), ("progmem_k", "progmem_k.c"), ("wdt_off", "wdt_off.c"), ("kat_vec", "kat_vec.c"), ("sha512_compress", "sha512_compress.c")] {
        let src = src_dir.join(file);
        let obj = out.join(format!("{name}.o"));
        let status = std::process::Command::new(&gcc)
            .args([
                "-mmcu=atmega328p",
                "-Os",
                "-DF_CPU=16000000UL",
                "-c",
                "-o",
            ])
            .arg(&obj)
            .arg(&src)
            .status()
            .expect("avr-gcc");
        if !status.success() {
            panic!("avr-gcc failed for {file}");
        }
        println!("cargo:rerun-if-changed={}", src.display());
        println!("cargo:rustc-link-arg={}", obj.display());
    }
    println!("cargo:rerun-if-env-changed=AVR_GCC");
}

