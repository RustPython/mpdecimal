#!/bin/sh
#
# Run the upstream short tests for the configuration matrix:
#
#   macos-aarch64-uint128    host
#   macos-x86_64-uint128     host, Rosetta 2
#   linux-aarch64-uint128    container (Containerfile)
#   linux-x86_64-uint128     container (Containerfile.amd64), Rosetta 2
#   linux-i686-ansi32        container, qemu-user
#   linux-s390x-uint128      container, qemu-user (big-endian)
#   linux-armv7-ansi32       container, qemu-user
#   wasi-wasm32-ansi32       container, wasi-sdk and wasmtime
#   windows-x86_64-uint128   container (Containerfile.amd64), MinGW-w64, Wine
#
# usage: c2rust/test-all.sh [LABEL...]   (default: all)
#
# Environment:
#   CRATE     crate to test instead of each LABEL's own: root (the crate of
#             the repository root)
#   FEATURES  cargo features of the crate, e.g. test-hooks

# The shell reads the whole block before running it: editing this file does
# not affect a running invocation.
{
set -u

here=$(cd "$(dirname "$0")" && pwd)

run() {
    label=$1
    shift
    if [ -n "$only" ]; then
        case " $only " in *" $label "*) ;; *) return 0 ;; esac
    fi
    log=work/$label/test-short${CRATE:+-$CRATE}${FEATURES:+-$FEATURES}.log
    mkdir -p "$here/work/$label"
    echo "=== $label${CRATE:+ ($CRATE${FEATURES:+ +$FEATURES})}"
    "$@" > "$here/$log" 2>&1 \
        && echo "PASS ($(grep -c PASS "$here/$log") lines)" \
        || { echo "FAIL, see c2rust/$log"; failed="$failed $label"; }
}

only="$*"
failed=
# native-static-libs of the staticlib for x86_64-pc-windows-gnu (rustc
# --print native-static-libs)
WINDOWS_LIBS="-lkernel32 -lntdll -luserenv -lws2_32 -ldbghelp"
# wasm-ld gives the program a 64 KiB stack below the static data, which the
# unoptimized crate overflows (squaretrans_pow2 has 32 KiB of buffers):
# 8 MiB like a native main thread, placed first so that an overflow traps
WASI_LIBS="-lm -Wl,-z,stack-size=8388608 -Wl,--stack-first"
: "${CRATE:=}"
: "${FEATURES:=}"

if [ "$(uname -s)" = Darwin ]; then
    run macos-aarch64-uint128 env CRATE="$CRATE" FEATURES="$FEATURES" "$here/test-short.sh" macos-aarch64-uint128
    run macos-x86_64-uint128 env CRATE="$CRATE" FEATURES="$FEATURES" RUST_TARGET=x86_64-apple-darwin \
        RUST_TOOLCHAIN=stable-x86_64-apple-darwin CC="cc -arch x86_64" \
        RUNNER="arch -x86_64" "$here/test-short.sh" macos-x86_64-uint128
    linux="$here/in-container.sh"
else
    linux=
fi

if [ -n "$linux" ]; then
    amd64="env CONTAINER_IMAGE=mpdecimal-amd64 CONTAINER_ARCH=amd64 $linux"
else
    amd64=
fi
crate="CRATE=$CRATE FEATURES=$FEATURES"

run linux-aarch64-uint128 $linux env $crate c2rust/test-short.sh linux-aarch64-uint128
run linux-x86_64-uint128 $amd64 env $crate c2rust/test-short.sh linux-x86_64-uint128
run linux-i686-ansi32 $linux env $crate RUST_TARGET=i686-unknown-linux-gnu \
    CC=i686-linux-gnu-gcc RUNNER="qemu-i386 -L /usr/i686-linux-gnu" \
    c2rust/test-short.sh linux-i686-ansi32
run linux-s390x-uint128 $linux env $crate RUST_TARGET=s390x-unknown-linux-gnu \
    CC=s390x-linux-gnu-gcc RUNNER="qemu-s390x -L /usr/s390x-linux-gnu" \
    c2rust/test-short.sh linux-s390x-uint128
run linux-armv7-ansi32 $linux env $crate RUST_TARGET=armv7-unknown-linux-gnueabihf \
    CC=arm-linux-gnueabihf-gcc RUNNER="qemu-arm -L /usr/arm-linux-gnueabihf" \
    c2rust/test-short.sh linux-armv7-ansi32
run wasi-wasm32-ansi32 $linux env $crate RUST_TARGET=wasm32-wasip1 \
    CC=/opt/wasi-sdk/bin/clang SYSLIBS="$WASI_LIBS" RUNNER="wasmtime run --dir=." COPY_TESTS=1 \
    c2rust/test-short.sh wasi-wasm32-ansi32
run windows-x86_64-uint128 $amd64 env $crate RUST_TARGET=x86_64-pc-windows-gnu \
    CC=x86_64-w64-mingw32-gcc SYSLIBS="$WINDOWS_LIBS" EXE=.exe RUNNER=/usr/lib/wine/wine64 \
    c2rust/test-short.sh windows-x86_64-uint128

if [ -n "$failed" ]; then
    echo "failed:$failed" >&2
    exit 1
fi
exit
}
