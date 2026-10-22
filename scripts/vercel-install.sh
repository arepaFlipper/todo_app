#!/bin/sh
# Installs the Rust toolchain and Trunk on Vercel's build machines, which have
# no Nix. Keep versions in sync with flake.nix.
set -eu

RUST_VERSION=1.96.1
TRUNK_VERSION=0.21.14
# Vercel's image ships rustup with CARGO_HOME=/rust, not ~/.cargo
# Trunk is the static musl build: the gnu one needs a newer glibc than Vercel has
BIN_DIR="${CARGO_HOME:-$HOME/.cargo}/bin"

curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal \
  --default-toolchain "$RUST_VERSION" --target wasm32-unknown-unknown
"$BIN_DIR/rustup" target add wasm32-unknown-unknown --toolchain "$RUST_VERSION"

mkdir -p "$BIN_DIR"
curl -sL "https://github.com/trunk-rs/trunk/releases/download/v$TRUNK_VERSION/trunk-x86_64-unknown-linux-musl.tar.gz" \
  | tar -xz -C "$BIN_DIR"
