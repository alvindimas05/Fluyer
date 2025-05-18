fn main() {
    println!("cargo:rustc-link-search=native=C:\\mpv");
    tauri_build::build()
}
