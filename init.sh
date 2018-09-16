#!/bin/bash

rustup override add nightly
rustup component add rust-src
cargo install cargo-xbuild
cargo install bootimage
