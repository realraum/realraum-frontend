#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# cd into the directory of this file
cd "$(dirname "${BASH_SOURCE[0]}")"

# Install dependencies for build
echo "Installing build dependencies for Cloudflare pages..."

# Install rustup & Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain nightly

# Add the .cargo/bin folder to PATH
export PATH=$PATH:$HOME/.cargo/bin

# Install the architecture wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install --locked trunk

# Install Tailwind CSS
npm i -g tailwindcss

echo "Finished installing build dependencies for Cloudflare pages."

# Build the project
trunk build --release
