#!/bin/bash

# Install Linux Dependencies
if [[ "$os" == "linux" ]]; then
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf librust-alsa-sys-dev
fi

# Install NPM Packages
bun i

# Setup Android
if [[ "$os" == "android" ]]; then
    cd src-tauri/gen/android || exit
    {
        echo "keyAlias=$ANDROID_KEY_ALIAS"
        echo "password=$ANDROID_KEY_PASSWORD"
        base64 -d <<< "$ANDROID_KEY_BASE64" > "$RUNNER_TEMP/keystore.jks"
        echo "storeFile=$RUNNER_TEMP/keystore.jks"
    } > keystore.properties
    cd ../../../
    bun tauri android build -v --target "$ANDROID_ARCH"
    mkdir -p apks
    mv "./src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk" "./apks/Fluyer_${APP_VERSION}_${ANDROID_ARCH}.apk"
fi