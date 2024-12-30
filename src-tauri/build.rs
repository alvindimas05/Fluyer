fn main() {
    // Note: Used for Rodio because it requires C++ libraries.
    // Source: https://kazlauskas.me/entries/writing-proper-buildrs-scripts
    let target_os = std::env::var("CARGO_CFG_TARGET_OS");
    if target_os.unwrap().contains("android") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=c++_shared");
    }
    tauri_build::build()
}
