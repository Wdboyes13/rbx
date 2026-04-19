use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        let proj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_path = PathBuf::from(proj_dir).join("dist").join("libzstd_static.lib");
        println!("cargo:rustc-link-arg={}", lib_path.display());
    }
}
