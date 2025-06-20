# Fluyer
Music Player with Beautiful UI and Cross-Platform.
> [!NOTE]
> This project is on slow progress. I'm working on it to have fun :)

![Preview App](preview.png)

## Downloads
> [!WARNING]
> Keep in mind that this app is still in beta and has some issues. Scroll below to check.

You can grab the latest version of the app below or check the [release page](https://github.com/alvindimas05/Fluyer/releases).
- [Windows](https://github.com/alvindimas05/Fluyer/releases/download/v0.1.0/Fluyer_0.1.0_x64-setup.exe)
- [MacOS Intel](https://github.com/alvindimas05/Fluyer/releases/download/v0.1.0/Fluyer_0.1.0_x64.dmg) ([Click here](#macos) before installing)
- [MacOS Silicon](https://github.com/alvindimas05/Fluyer/releases/download/v0.1.0/Fluyer_0.1.0_aarch64.dmg) ([Click here](#macos) before installing)
- [Android aarch64](https://github.com/alvindimas05/Fluyer/releases/download/v0.1.0/Fluyer_0.1.0_aarch64.apk) (For most android mobile devices)
- [Android armv7](https://github.com/alvindimas05/Fluyer/releases/download/v0.1.0/Fluyer_0.1.0_armv7.apk) (For older android mobile devices)
- [Android x86_64](https://github.com/alvindimas05/Fluyer/releases/download/v0.1.0/Fluyer_0.1.0_x86_64.apk) (Usually for chromebooks)
- [Android i686](https://github.com/alvindimas05/Fluyer/releases/download/v0.1.0/Fluyer_0.1.0_i686.apk) (Usually for older chromebooks)

## Issues

### MacOS

> [!IMPORTANT]
> This step is necessary. I didn't notorize the app because Apple Developer is simply too expensive for me. It will show "Apple could not verify 'Fluyer.app' is free of malware". It refers to the [lack of notarizaion](https://support.apple.com/en-us/102445), not to any anomalies detected.
- After moving `Fluyer` to the `Applications` directory, run this command at the terminal.
```
/usr/bin/xattr -cr /Applications/Fluyer.app
```

### Universal
- High resource usage due to Animated Background. For now, the animation is disabled but the background is still enabled.

### iOS
- Even though it's already tested, iOS release is not planned for now due to Apple Developer pricing.

## Features Progress

### Basic Functionality
- [x] Play & Pause 
- [x] Next
- [x] Previous
- [x] Volume Control
- [x] Mute
- [ ] Shuffle & Repeat
- [x] Playlist

### Advanced Functionality
- [x] Search
- [x] Lyrics
- [x] Animated Background
- [x] Cover Art Caching

### API Functionality
- [x] MusicBrainz Integration (For Cover Art)

[//]: # (- [ ] Spotify Integration &#40;For Spotify Canvas&#41;)

[//]: # (- [ ] Soulseek Integration &#40;We love FLACs <3&#41;)

## Credits
- [Default Album Cover](https://www.freepik.com/free-vector/music-notes-rainbow-colourful-with-vinyl-record-white-backgro_24459713.htm)