use std::{env, fs, path::PathBuf};

fn main() {
    // memory.x für cortex-m-rt in den Linker-Suchpfad legen
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
