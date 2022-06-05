#!/bin/bash

# Download Dependencies
sudo pacman -Syu --needed --noconfirm curl git clang rustup

# Download stable and nightly
rustup default stable
rustup update

rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
