# Fluyer
Music Player with Beautiful UI and Cross-Platform.
![Preview App](preview.png)

## Downloads
Keep in mind that this app is still in beta and has some issues. Scroll below to check.
You can grab the latest version of the app below or check the [release page](https://github.com/alvindimas05/Fluyer/releases).
- [Windows](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_x64-setup.exe)
- [MacOS Intel](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_x64.dmg)
- [MacOS Silicon](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_aarch64.dmg)
- [Debian/Ubuntu](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_amd64.deb)
- [Linux AppImage](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_amd64.AppImage)
- [Android aarch64](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_aarch64.apk) (For most android mobile devices)
- [Android armv7](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_armv7.apk) (For older android mobile devices)
- [Android x86_64](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_x86_64.apk) (Usually for chromebooks)
- [Android i686](https://github.com/alvindimas05/Fluyer/releases/download/v0.0.1/Fluyer_0.0.1_i686.apk) (Usually for older chromebooks)

## Issues

### Android
- When headset is plugged/unplugged, the app requires restart. This issue is caused due to Rust [cpal](https://github.com/RustAudio/cpal) limitation.
- The app might has some delay when the Frontend communicate with the Backend. For example, when trying to pause, it might have miliseconds delay.
- Some features are disabled like animated background to prevent performance issues but it's planned to add option to enable this if you assume that your device can handle it.
- Sometimes it stops itself when running on background due to android security. Planned to fix this.

### iOS
- Even though it's already tested, iOS release is not planned for now due to Apple Developer pricing.

## Features Progress

### Basic Functionality
- [x] Play & Pause 
- [x] Next
- [ ] Previous
- [x] Volume Control
- [x] Mute
- [ ] Shuffle & Repeat
- [x] Playlist

### Advanced Functionality
- [ ] Search
- [ ] Group by Folder
- [x] Lyrics
- [x] Animated Background
- [x] Cover Art Caching

### API Functionality
- [x] MusicBrainz Integration (For Cover Art)
- [ ] Spotify Integration (For Spotify Canvas)
- [ ] Soulseek Integration (We love FLACs <3)


## Credits
- [Thank you, Freepik, for the free Icons](https://www.flaticon.com/authors/special/lineal/2?author_id=1)
- [Default Album Cover](https://www.freepik.com/free-vector/music-notes-rainbow-colourful-with-vinyl-record-white-backgro_24459713.htm)