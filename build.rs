// build.rs
fn main() {
    println!("cargo:rerun-if-changed=src/asm/boot.S");
}