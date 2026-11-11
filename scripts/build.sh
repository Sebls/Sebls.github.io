#!/bin/sh
set -eu
BASE_PATH=${1:-/}
case "$BASE_PATH" in /*/) ;; /) ;; *) echo 'Use a base path with leading and trailing slashes.' >&2; exit 1;; esac
case "$BASE_PATH" in *[!a-zA-Z0-9/_-]*|*..*) echo 'Use URL path segments containing letters, digits, hyphens, or underscores.' >&2; exit 1;; esac
trunk build --release --public-url "$BASE_PATH"
cargo run --locked --release --bin render -- dist "$BASE_PATH"
