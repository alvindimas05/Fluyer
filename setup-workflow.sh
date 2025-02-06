#!/bin/bash

cp .env.example .env

# Install Linux Dependencies
if [[ "$os" == "linux" ]]; then
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf librust-alsa-sys-dev
fi

# Install NPM Packages
bun i

# Setup Android
if [[ "$os" == "android" ]]; then
    export ANDROID_NDK_HOME=$NDK_HOME
    export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
    export RANLIB=$ANDROID_NDK_ROOT/llvm-ranlib
    export AR=$ANDROID_NDK_ROOT/llvm-ar
    export STRIP=$ANDROID_NDK_ROOT/llvm-strip
    export LD=$ANDROID_NDK_ROOT/ld
    
    cd src-tauri/gen/android
    echo "keyAlias=$ANDROID_KEY_ALIAS" > keystore.properties
    echo "password=$ANDROID_KEY_PASSWORD" >> keystore.properties
    base64 -d <<< "$ANDROID_KEY_BASE64" > $RUNNER_TEMP/keystore.jks
    echo "storeFile=$RUNNER_TEMP/keystore.jks" >> keystore.properties
    cd ../../../
    bun tauri android build -v --target $ANDROID_ARCH
    # Note: It's not executed in the workflow for some reason. So we need to add it manually in the workflow.
    # mv ./src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk ./Fluyer_$APP_VERSION_$ANDROID_ARCH.apk
fi

# Setup iOS
if [[ "$os" == "ios" ]]; then
    bun tauri ios build -v
    mkdir -p ipas
    mv "./src-tauri/gen/apple/build/arm64/Fluyer.ipa" "./ipas/Fluyer.ipa"
fi