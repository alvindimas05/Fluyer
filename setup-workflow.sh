#!/bin/bash

if [[ "$os" == "windows" || "$os" == "macos" ]]; then
    bun i
else if [[ "$os" == "linux" ]; then
    apt-get update
    apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf librust-alsa-sys-dev
    bun i
else if [[ "$os" == "android" ]]; then
    bun i
    cd src-tauri/gen/android
    echo "keyAlias=$ANDROID_KEY_ALIAS" > keystore.properties
    echo "password=$ANDROID_KEY_PASSWORD" >> keystore.properties
    base64 -d <<< "$ANDROID_KEY_BASE64" > $RUNNER_TEMP/keystore.jks
    echo "storeFile=$RUNNER_TEMP/keystore.jks" >> keystore.properties
    cd ../../../
    bun tauri android build -v --target $ANDROID_ARCH
    mkdir apks
    mv ./src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk ./apks/Fluyer_$APP_VERSION_$ANDROID_ARCH.apk
