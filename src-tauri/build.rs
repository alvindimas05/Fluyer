use std::env;

fn main() {
    // if let Ok(source) = env::var("FLUYER_MPV_SOURCE") {
    //     println!("cargo:rustc-link-search=native={}", source);
    // }
    println!("cargo:rustc-link-search=native={}", env::var("FLUYER_MPV_SOURCE").unwrap());
    tauri_build::build()
}
