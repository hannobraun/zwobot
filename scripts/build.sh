#!/usr/bin/env bash

RUSTC="rustc"
if [[ -n "$1" ]]; then
	RUSTC=$1
fi

mkdir -p output
git submodule update --init &&
$RUSTC --out-dir output vendor/inotify-rs/src/lib.rs &&
$RUSTC -o output/zwobot -L output src/main.rs
