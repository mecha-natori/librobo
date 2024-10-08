#!/usr/bin/env bash
set -euo pipefail
cd "$(realpath "$(dirname "$0")")"
if ! command -v cargo; then
    printf "RustがインストールされていないかPATHが通されていません。\n"
    exit 1
fi
RELEASE=false
TARGET_TRIPLE=""
while getopts rt: OPT; do
    case "$OPT" in
        r)
            RELEASE=true
            ;;
        t)
            TARGET_TRIPLE=$OPTARG
            ;;
        *)
            ;;
    esac
done
RELEASE_FLAG=""
LIB_PATH=""
if $RELEASE; then
    RELEASE_FLAG="--release"
    LIB_PATH="target/$TARGET_TRIPLE${TARGET_TRIPLE:+/}release/librobo.a"
else
    LIB_PATH="target/$TARGET_TRIPLE${TARGET_TRIPLE:+/}debug/librobo.a"
fi
cargo build "$RELEASE_FLAG"
install -dm755 bindings/c/lib
install -dm755 bindings/cxx/lib
install -Dm644 "$LIB_PATH" bindings/c/lib/librobo.a
install -Dm644 "$LIB_PATH" bindings/cxx/lib/librobo.a
