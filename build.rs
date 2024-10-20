fn main() {
    // If uncommented, this causes:
    // "ld: cannot open linker script file linkall.x: No such file or directory"
    // println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
}
