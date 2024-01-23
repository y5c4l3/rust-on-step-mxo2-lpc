#!/bin/sh

uid=${SUDO_UID:-$(id -u)}
gid=${SUDO_GID:-$(id -g)}

if ! docker ps >/dev/null ; then
    echo "Docker is not running or you have no permissions." >&2
    exit 1
fi

debug_binary="./target/debug/rust-on-step-mxo2-lpc"
release_binary="./target/debug/rust-on-step-mxo2-lpc"

if [ -f "$debug_binary" ] || [ -f "$release_binary" ]; then
    if [ "$release_binary" -nt "$debug_binary" ]; then
        echo "Using release binary at $release_binary" >&2
        binary="$release_binary"
    else
        echo "Using debug binary at $debug_binary" >&2
        binary="$debug_binary"
    fi
else
    echo "Cannot find binaries, run \`cargo build\` or \`cargo build --release\` first."
    exit 1
fi

image="y5c4l3/step-mxo2-lpc-toolchains:latest"
docker run -v "$(pwd):/app" --rm "$image" \
    sh -c \
    "cd /app; ./target/debug/rust-on-step-mxo2-lpc; chown -R $uid:$gid ./firmware"