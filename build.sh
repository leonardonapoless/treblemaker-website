#!/bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
export PATH="/rust/bin:$HOME/.cargo/bin:/root/.cargo/bin:$PATH"
if [ -f "/rust/env" ]; then
  source "/rust/env"
elif [ -f "$HOME/.cargo/env" ]; then
  source "$HOME/.cargo/env"
fi
rustup target add wasm32-unknown-unknown
curl -L https://github.com/DioxusLabs/dioxus/releases/download/v0.7.0/dx-linux-x86_64 -o dx
chmod +x dx
./dx build --release
