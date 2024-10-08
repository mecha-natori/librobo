Param([switch] $r, [string] $t)

Set-Location $PSCommandPath
if(-not (Get-Command -Name cargo -ErrorAction SilentlyContinue)) {
    Write-Output "RustがインストールされていないかPATHが通されていません。"
    exit 1
}
$RELEASE = $r
$TARGET_TRIPLE = "$t"
$RELEASE_FLAG = ""
$LIB_PATH = ""
if($RELEASE) {
    $RELEASE_FLAG = "--release"
    $LIB_PATH = "target/$TARGET_TRIPLE$( $TARGET_TRIPLE -and "/" )release/librobo.a"
} else {
    $LIB_PATH = "target/$TARGET_TRIPLE$( $TARGET_TRIPLE -and "/" )debug/librobo.a"
}
cargo build "$RELEASE_FLAG"
New-Item -Path bindings/c/lib -ItemType Directory -ErrorAction SilentlyContinue
New-Item -Path bindings/cxx/lib -ItemType Directory -ErrorAction SilentlyContinue
Copy-Item -Path "$LIB_PATH" -Destination bindings/c/lib/librobo.a -Force
Copy-Item -Path "$LIB_PATH" -Destination bindings/cxx/lib/librobo.a -Force
