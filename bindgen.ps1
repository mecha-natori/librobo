Param([switch] $r, [switch] $s, [string] $t)

Set-Location $PSScriptRoot
if(-not (Get-Command -Name cargo -ErrorAction SilentlyContinue)) {
    Write-Output "RustがインストールされていないかPATHが通されていません。"
    exit 1
}
$RELEASE = $r
$FEATURE = "all"
$TARGET_TRIPLE = "$t"
$LIB_PATH = ""
if($s) {
    $FEATURE = "all-std"
}
if($RELEASE) {
    cargo build --no-default-features --features "$FEATURE" --release
    $LIB_PATH = "target/$( $TARGET_TRIPLE ? "$TARGET_TRIPLE/" : [string]::Empty )release/librobo.a"
} else {
    cargo build --no-default-features --features "$FEATURE"
    $LIB_PATH = "target/$( $TARGET_TRIPLE ? "$TARGET_TRIPLE/" : [string]::Empty )debug/librobo.a"
}
New-Item -Path bindings/c/lib -ItemType Directory -ErrorAction SilentlyContinue
New-Item -Path bindings/cxx/lib -ItemType Directory -ErrorAction SilentlyContinue
Copy-Item -Path "$LIB_PATH" -Destination bindings/c/lib/librobo.a -Force
Copy-Item -Path "$LIB_PATH" -Destination bindings/cxx/lib/librobo.a -Force
