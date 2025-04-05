#!/bin/bash
rm -rf bin
mkdir bin

echo "Building Xinux statically for Linux..."

# Set environment variables for static linking
export RUSTFLAGS="-C target-feature=+crt-static"
export CC=musl-gcc
export CXX=musl-g++

# Build the project
cargo build --release --target x86_64-unknown-linux-musl

echo "Build complete. Binary located at: target/x86_64-unknown-linux-musl/release/xinux"
cp target/x86_64-unknown-linux-musl/release/xinux bin/