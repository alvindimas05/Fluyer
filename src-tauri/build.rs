use std::env;

fn main() {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    println!("cargo:rustc-link-search=native={}", std::env::var("FLUYER_MPV_SOURCE").unwrap());
    tauri_build::build()
}
