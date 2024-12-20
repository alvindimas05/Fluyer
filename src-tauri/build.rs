// use tauri_build::WindowsAttributes;

fn main() {
    // tauri_build::try_build(
    //     tauri_build::Attributes::new()
    //         .windows_attributes(WindowsAttributes::new_without_app_manifest())
    //         .plugin(
    //             "tauri-plugin-fluyer",
    //             tauri_build::InlinedPlugin::new().commands(&["toast"]),
    //         ),
    // )
    // .expect("Failed to build Fluyer plugin.");

    // Note: Used for Rodio because it requires C++ libraries.
    // Source: https://kazlauskas.me/entries/writing-proper-buildrs-scripts
    let target_os = std::env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("android") => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=c++_shared");
        }
        _ => {}
    }
    tauri_build::build()
}
