# Building Kumandra Node 

## Table of Contents
1. [Manual](#Manual)
1. [Build with Docker](#Build-with-Docker) TODO!
3. [Build with Nix](#Build-with-Nix) TODO!

## Manual
These are build dependencies we use in our linux images for `kumandra-node`:
```
rust-nightly-2022-05-18
bash-4.4
glibc-2.31
binutils-2.36,1
clang-11.0.0rc2
protobuf-3.13.0
openssl-1.1.1g
git-2.28.0
nss-cacert-3.56
pkg-config-0.29.2
rocksdb-6.29.3
```

Example build using KOOMPI OS(Arch linux also work) and bash shell:
```
pi -Syu --needed --noconfirm curl git clang make rustup
git clone https://github.com/kumandra/kumandra.git
cd kumandra
rustup show
rustup target add x86_64-unknown-linux-gnu wasm32-unknown-unknown
cargo build --release
```
