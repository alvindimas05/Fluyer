fn main() {
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/fluyer");

    if !std::env::var("TARGET").unwrap().contains("android") {
        println!("cargo:rustc-link-search=native={}", "libs");
    }

    tauri_build::build()
}
