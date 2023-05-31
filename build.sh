#!/usr/bin/bash
TAGS="--release -j8"
function pred() {
	local error_message=$1
	echo -e "\033[31m$error_message\033[0m" >&2
}

cargo build $TAGS --target=aarch64-linux-android
if [[ ! $? -eq 0 ]]; then
	pred "Help: Try use rustup target add aarch64-linux-android"

else
    exit 0
fi
cargo build $TAGS --target=aarch64-unknown-linux-musl
if [[ ! $? -eq 0 ]]; then
	pred "Help: Try use rustup target add aarch64-unknown-linux-musl"
fi