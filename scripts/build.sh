#!/usr/bin/env bash

mkdir -p output
git submodule update --init &&
rustc --out-dir output vendor/inotify-rs/src/lib.rs &&
rustc -o output/zwobot -L output source/rust/zwobot/main.rs
