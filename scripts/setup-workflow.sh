#!/bin/bash

cp .env.example .env

#APP_VERSION=$(bun -p "require('./package.json').version")
#echo "APP_VERSION=$APP_VERSION" >> $GITHUB_ENV

# Install Linux Dependencies
if [[ "$os" == "linux" ]]; then
    sudo curl --output-dir /etc/apt/trusted.gpg.d -O https://apt.fruit.je/fruit.gpg
    deb http://apt.fruit.je/debian trixie mpv
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf librust-alsa-sys-dev
    sudo apt-get install -y libmpv-dev
    sudo apt-get install -y lld
fi

# Install NPM Packages
bun i

if [[ "$os" == "windows" ]] && [[ "$arch" == "arm64" ]]; then
    bun i @tauri-apps/cli-win32-arm64-msvc
fi


if [[ "$os" == "windows" ]] || [[ "$os" == "linux" ]] || [[ "$os" == "macos" ]]; then
    bun run init
fi

if [[ "$os" == "macos" ]]; then
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  brew install lld
fi

# Setup Android
if [[ "$os" == "android" ]]; then
    export ANDROID_NDK_HOME=$NDK_HOME
    export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
    export ANDROID_NDK_BIN="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"
    export RANLIB=$ANDROID_NDK_BIN/llvm-ranlib
    export AR=$ANDROID_NDK_BIN/llvm-ar
    export STRIP=$ANDROID_NDK_BIN/llvm-strip
    export LD=$ANDROID_NDK_BIN/ld
    export CC=$ANDROID_NDK_BIN/clang
    
    # Install required build tools for CMake
    sudo apt-get update
    sudo apt-get install -y build-essential cmake make
    
    cd src-tauri/gen/android
    echo "keyAlias=$ANDROID_KEY_ALIAS" > keystore.properties
    echo "password=$ANDROID_KEY_PASSWORD" >> keystore.properties
    base64 -d <<< "$ANDROID_KEY_BASE64" > $RUNNER_TEMP/keystore.jks
    echo "storeFile=$RUNNER_TEMP/keystore.jks" >> keystore.properties
    cd ../../../
    bun android:init
    bun tauri android build -v --target $ANDROID_ARCH
    # Note: It's not executed in the workflow for some reason. So we need to add it manually in the workflow.
    # mv ./src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk ./Fluyer_$APP_VERSION_$ANDROID_ARCH.apk
fi

# Build iOS
# if [[ "$os" == "ios" ]]; then
#     bun tauri ios build -v
#     mkdir -p ipas
#     mv "./src-tauri/gen/apple/build/arm64/Fluyer.ipa" "./ipas/Fluyer.ipa"
# fi