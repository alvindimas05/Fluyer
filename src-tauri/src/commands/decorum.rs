#[cfg(windows)]
#[tauri::command]
pub fn decorum_show_snap_overlay() {
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Meta, Press).unwrap();
    enigo.key(Key::Unicode('z'), Click).unwrap();
    enigo.key(Key::Meta, Release).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    // Press Alt to hide the ugly numbers
    enigo.key(Key::Alt, Click).unwrap();
}
