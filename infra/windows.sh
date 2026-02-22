#!/bin/bash
cp .env.example .env

bun i
if [[ "$arch" == "arm64" ]]; then
    bun i @tauri-apps/cli-win32-arm64-msvc
fi

bun run init
bun tauri build
