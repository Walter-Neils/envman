#!/bin/bash
rm -r build
mkdir build
cargo build --release
cp ../target/release/envman ./build/
cp PKGBUILD ./build/
cd build
makepkg -si
cd ..
