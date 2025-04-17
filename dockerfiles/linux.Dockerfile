FROM debian:latest

# Install Debian Dependencies
RUN apt update -y
RUN apt install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf librust-alsa-sys-dev
RUN apt install -y unzip

# Install Bun
RUN curl -fsSL https://bun.sh/install | BUN_INSTALL=/usr bash

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Change Work Directory
WORKDIR /usr/src/app
COPY . /usr/src/app

# Install Bun dependencies
RUN bun install

# Build
RUN bun tauri build