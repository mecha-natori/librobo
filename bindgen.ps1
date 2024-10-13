Param([switch] $h, [switch] $n, [switch] $r, [switch] $s, [string] $t)

if($h) {
    Write-Output "バインディングの生成スクリプト"
    Write-Output
    Write-Output "使い方: bindgen.ps1 [OPTIONS...]"
    Write-Output
    Write-Output "オプション:"
    Write-Output "  -h                  ヘルプを表示"
    Write-Output "  -n                  no_std環境としてビルド"
    Write-Output "  -r                  releaseプロファイルでビルド"
    Write-Output "  -s                  std環境としてビルド"
    Write-Output "  -t <TARGET_TRIPLE>  Rustのターゲットトリプルを指定"
    exit 0
}
$RELEASE = $r
$TARGET_TRIPLE = "$t"
$NO_STD = $n
$STD = $s
if($NO_STD -and $STD) {
    Write-Output "-nと-sは同時に指定できません。"
    exit 1
}
if(-not ($NO_STD -or $STD)) {
    Write-Output "-nか-sを指定してください。"
    exit 1
}
[array] $BUILD_ARGS = @("--no-default-features", "--crate-type", "staticlib")
if($NO_STD) {
    $BUILD_ARGS += "--features"
    $BUILD_ARGS += "all,bind-c"
}
if($STD) {
    $BUILD_ARGS += "--features"
    $BUILD_ARGS += "all-std,bind-c"
}
if($RELEASE) {
    $BUILD_ARGS += "--release"
}
Set-Location $PSScriptRoot
if(-not (Get-Command -Name cargo -ErrorAction SilentlyContinue)) {
    Write-Output "RustがインストールされていないかPATHが通されていません。"
    exit 1
}
$LIB_PATH = ""
if($RELEASE) {
    $LIB_PATH = "target/$( $TARGET_TRIPLE ? "$TARGET_TRIPLE/" : [string]::Empty )release/librobo.a"
} else {
    $LIB_PATH = "target/$( $TARGET_TRIPLE ? "$TARGET_TRIPLE/" : [string]::Empty )debug/librobo.a"
}
cargo build @BUILD_ARGS
New-Item -Path bindings/c/lib -ItemType Directory -Force
New-Item -Path bindings/cxx/lib -ItemType Directory -Force
Copy-Item -Path "$LIB_PATH" -Destination bindings/c/lib/librobo.a -Force
Copy-Item -Path "$LIB_PATH" -Destination bindings/cxx/lib/librobo.a -Force
