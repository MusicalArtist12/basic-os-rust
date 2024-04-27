use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let assembly_files = ["boot", "long_mode_init", "multiboot_header"];

    println!("cargo::rustc-link-arg=-Tsrc/arch/x86_64/linker.ld");

    for file in assembly_files {
        Command::new("nasm").args(&["-f", "elf64"])
            .arg(&format!("src/arch/x86_64/{}.asm", file))
            .args(["-o", &format!("{}/{}.o", out_dir, file)])
            .status().unwrap();
        Command::new("ar").args(&["crus"])
            .arg(&format!("{}/lib{}.a", out_dir, file))
            .arg(&format!("{}/{}.o", out_dir, file))
            .status().unwrap();
        
        println!("cargo::rustc-link-search=native={}", out_dir);
        println!("cargo::rustc-link-lib=static={}", file);
        println!("cargo::warning={}/arch/x86_64/{}.o", out_dir, file);
        
    }

    println!("cargo::rerun-if-changed=build.rs");
}