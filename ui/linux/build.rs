fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = std::path::Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let libs_dir = root_dir.join("libs/linux");
    println!("cargo:rustc-link-search=native={}", libs_dir.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libs_dir.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../libs/linux");
}
