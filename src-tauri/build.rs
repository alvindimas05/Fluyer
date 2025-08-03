use std::env;
fn main() {
    if !std::env::var("TARGET").unwrap().contains("android"){
        println!("cargo:rustc-link-search=native={}", std::env::var("FLUYER_MPV_SOURCE").unwrap());
    }
    tauri_build::build()
}
