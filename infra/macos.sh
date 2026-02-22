#!/bin/bash
cp .env.example .env

bun i
bun run init

/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install lld

bun tauri build
