fn main() {
    // Note: This is for Android build only. Uncomment when required.
    // Source: https://kazlauskas.me/entries/writing-proper-buildrs-scripts
    // println!("cargo:rustc-link-lib=dylib=stdc++");
    // println!("cargo:rustc-link-lib=c++_shared");
    tauri_build::build()
}
