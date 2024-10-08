#!/usr/bin/env bash
set -euo pipefail
cd "$(realpath "$(dirname "$0")")"
if ! command -v cargo; then
    printf "RustがインストールされていないかPATHが通されていません。\n"
    exit 1
fi
RELEASE=false
FEATURE="all"
TARGET_TRIPLE=""
while getopts rst: OPT; do
    case "$OPT" in
        r)
            RELEASE=true
            ;;
        s)
            FEATURE="all-std"
            ;;
        t)
            TARGET_TRIPLE=$OPTARG
            ;;
        *)
            ;;
    esac
done
LIB_PATH=""
if $RELEASE; then
    cargo build --no-default-features --features "$FEATURE" --release
    LIB_PATH="target/$TARGET_TRIPLE${TARGET_TRIPLE:+/}release/librobo.a"
else
    cargo build --no-default-features --features "$FEATURE"
    LIB_PATH="target/$TARGET_TRIPLE${TARGET_TRIPLE:+/}debug/librobo.a"
fi
install -dm755 bindings/c/lib
install -dm755 bindings/cxx/lib
install -Dm644 "$LIB_PATH" bindings/c/lib/librobo.a
install -Dm644 "$LIB_PATH" bindings/cxx/lib/librobo.a
