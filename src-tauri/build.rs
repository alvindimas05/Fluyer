use std::env;

fn main() {
    #[cfg(any(desktop, target_os = "windows"))]
    println!("cargo:rustc-link-search=native={}", std::env::var("FLUYER_MPV_SOURCE").unwrap());
    tauri_build::build()
}
