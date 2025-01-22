#!/usr/bin/env bash
set -euo pipefail
RELEASE=false
TARGET_TRIPLE=""
NO_STD=false
STD=false
while getopts hnrst: OPT; do
    case "$OPT" in
        h)
            printf "バインディングの生成スクリプト\n"
            printf "\n"
            printf "使い方: bindgen.sh [OPTIONS...]\n"
            printf "\n"
            printf "オプション:\n"
            printf "  -h                  ヘルプを表示\n"
            printf "  -r                  releaseプロファイルでビルド\n"
            printf "  -s                  std環境としてビルド\n"
            printf "  -t <TARGET_TRIPLE>  Rustのターゲットトリプルを指定\n"
            exit 0
            ;;
        n)
            NO_STD=true
            ;;
        r)
            RELEASE=true
            ;;
        s)
            STD=true
            ;;
        t)
            TARGET_TRIPLE=$OPTARG
            ;;
        *)
            ;;
    esac
done
if ($NO_STD && $STD); then
    printf "-nと-sは同時に指定できません。\n"
    exit 1
fi
if ! ($NO_STD || $STD); then
    printf "-nか-sを指定してください。\n"
    exit 1
fi
BUILD_ARGS=(--no-default-features --example robo)
if $NO_STD; then
    BUILD_ARGS+=(--features "all,bind-c" --config 'profile.dev.panic="abort"' --config 'profile.release.panic="abort"')
fi
if $STD; then
    BUILD_ARGS+=(--features "all-std,bind-c")
fi
if $RELEASE; then
    BUILD_ARGS+=(--release)
fi
cd "$(realpath "$(dirname "$0")")"
if ! command -v cargo; then
    printf "RustがインストールされていないかPATHが通されていません。\n"
    exit 1
fi
LIB_PATH=""
if $RELEASE; then
    LIB_PATH="target/$TARGET_TRIPLE${TARGET_TRIPLE:+/}release/examples/librobo.a"
else
    LIB_PATH="target/$TARGET_TRIPLE${TARGET_TRIPLE:+/}debug/examples/librobo.a"
fi
if [[ -n $TARGET_TRIPLE ]]; then
    BUILD_ARGS+=(--target "$TARGET_TRIPLE")
fi
printf "Executing cargo build %s\n" "${BUILD_ARGS[*]}"
cargo build "${BUILD_ARGS[@]}"
install -dm755 bindings/c/lib
install -dm755 bindings/cxx/lib
install -Dm644 "$LIB_PATH" bindings/c/lib/librobo.a
install -Dm644 "$LIB_PATH" bindings/cxx/lib/librobo.a
