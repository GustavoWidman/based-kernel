use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rerun-if-changed=asm/boot.asm");
    println!("cargo:rerun-if-changed=linker.ld");

    let asm_output = PathBuf::from(&out_dir).join("boot.o");
    let asm_source = PathBuf::from(&manifest_dir).join("asm/boot.asm");

    let nasm_status = Command::new("nasm")
        .args(&[
            "-f",
            "elf64",
            asm_source.to_str().unwrap(),
            "-o",
            asm_output.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute nasm - make sure nasm is installed");

    if !nasm_status.success() {
        panic!("nasm failed to assemble boot.asm");
    }

    println!("cargo:rustc-link-arg={}", asm_output.display());

    let linker_script = PathBuf::from(&manifest_dir).join("linker.ld");
    println!("cargo:rustc-link-arg=-T");
    println!("cargo:rustc-link-arg={}", linker_script.display());

    println!("cargo:rustc-link-arg=--gc-sections");
}
