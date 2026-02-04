fn main() {
    #[cfg(target_os = "linux")]
    {
        // Production rpath for installed packages
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/fluyer");
        // Development rpath - $ORIGIN allows finding libs relative to the executable
        // Binary is at target/debug/fluyer, libs are at libs/
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../libs");
    }

    if !std::env::var("TARGET").unwrap().contains("android") {
        println!("cargo:rustc-link-search=native={}", "libs");
    }

    tauri_build::build()
}
