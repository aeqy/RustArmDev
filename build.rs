use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out_dir.join("memory.x"))
    .unwrap()
    .write_all(include_bytes!("memory.x"))
    .unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=memory.x");
}