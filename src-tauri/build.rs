fn main() {
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/fluyer");

    if !std::env::var("TARGET").unwrap().contains("android") {
        if let Ok(source) = std::env::var("FLUYER_LIBS_SOURCE") {
            println!("cargo:rustc-link-search=native={}", source);
        } else {
            println!("cargo:rustc-link-search=native=src-tauri/libs");
        }
    }
    tauri_build::build()
}
