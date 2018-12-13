#!/usr/bin/env bash
set -ue

target="$1"

cargo build --release --target "$target" --locked
mkdir -p dist
cp "target/$target/release/$APP_NAME" "dist/$APP_NAME-$target"
