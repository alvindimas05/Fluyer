#!/bin/bash
cp .env.example .env

bun i
if [[ "$arch" == "arm64" ]]; then
    bun i @tauri-apps/cli-win32-arm64-msvc
fi

bun run init

# Required for wgpu
cd src-tauri
cargo update

cd ..
bun tauri build
