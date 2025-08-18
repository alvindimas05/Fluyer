fn main() {
    if !std::env::var("TARGET").unwrap().contains("android") {
        // if let Ok(source) = std::env::var("FLUYER_MPV_SOURCE") {
        //     println!("cargo:rustc-link-search=native={}", source);
        // }
        println!("cargo:rustc-link-search=native={}", std::env::var("FLUYER_MPV_SOURCE").unwrap());
    }
    tauri_build::build()
}
