#!/bin/bash
mkdir build
cargo build --release
cp ../target/release/envman ./build/
cp PKGBUILD ./build/
cd build
makepkg -si
cd ..
