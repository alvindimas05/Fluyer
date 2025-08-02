use std::env;

fn main() {
    let target = std::env::var("TARGET").unwrap();

    if target.contains("windows")
        || target.contains("linux")
        || target.contains("darwin")
    {
        println!("cargo:rustc-link-search=native={}", std::env::var("FLUYER_MPV_SOURCE").unwrap());
    }
    tauri_build::build()
}
