fn main() {
    if !std::env::var("TARGET").unwrap().contains("android") {
        // Note: Required to satisfy both rust-analyzer and cargo run or build
        // For some reason, cargo can't detect the Ok() condition.
        // So we need to condition it if the condition works or not
        let mut is_linked = false;
        if let Ok(source) = std::env::var("FLUYER_MPV_SOURCE") {
            is_linked = true;
            println!("cargo:rustc-link-search=native={}", source);
        }
        if !is_linked {
            println!(
                "cargo:rustc-link-search=native={}",
                std::env::var("FLUYER_MPV_SOURCE").unwrap()
            );
        }
    }
    tauri_build::build()
}
