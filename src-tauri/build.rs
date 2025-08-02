use std::env;

fn main() {
    #[cfg(desktop)]
    println!("cargo:rustc-link-search=native={}", std::env::var("FLUYER_MPV_SOURCE").unwrap());
    tauri_build::build()
}
