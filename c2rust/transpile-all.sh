#!/bin/sh
#
# Translate the full configuration matrix.
#
#   macos-aarch64-uint128   host (macOS), CONFIG_64
#   macos-x86_64-uint128    host (macOS), --target=x86_64-apple-darwin, CONFIG_64
#   linux-aarch64-uint128   container, native, CONFIG_64
#   linux-x86_64-uint128    container, x86_64 sysroot, CONFIG_64
#   linux-i686-ansi32       container, i686 sysroot, CONFIG_32
#
# translated for comparison only (check-oracles.sh), on any host, since the
# neutral libc headers need no sysroot:
#
#   linux-armv7-ansi32       --target=armv7-linux-gnueabihf, CONFIG_32
#   windows-x86_64-uint128   --target=x86_64-pc-windows-gnu, CONFIG_64 (LLP64)
#   windows-aarch64-uint128  --target=aarch64-w64-mingw32, CONFIG_64 (LLP64)
#   windows-i686-ansi32      --target=i686-pc-windows-gnu, CONFIG_32
#
# (c2rust 0.22.1 cannot translate for s390x, whose va_list is not
# implemented, or wasm32; these targets are only tested)
#
# and with the test hooks of testit (transpile.sh --test-hooks):
#
#   macos-aarch64-uint128-hooks   host (macOS), CONFIG_64
#   linux-x86_64-uint128-hooks    container, x86_64 sysroot, CONFIG_64
#   linux-i686-ansi32-hooks       container, i686 sysroot, CONFIG_32
#
# CPython selects ansi32 whenever sizeof(size_t) == 4 and uint128 when the
# compiler has __uint128_t, so these two machines cover its configurations
# (x64 and ppro differ only in using inline assembly).
#
# usage: c2rust/transpile-all.sh [LABEL...]
#
# Without LABEL: translate all, then check-oracles.sh, merge.py (CONFIG_64 and
# CONFIG_32, without and with the test hooks, then the two results with the
# feature test-hooks) and dedupe.py -> c2rust/work/merged/mpdec, and
# install.py -> the crate of the repository root.

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
    "$@" || failed="$failed $label"
}

only="$*"
failed=

if [ "$(uname -s)" = Darwin ]; then
    : "${LLVM_CONFIG_PATH:=$(brew --prefix llvm@21)/bin/llvm-config}"
    export LLVM_CONFIG_PATH
    run macos-aarch64-uint128 "$here/transpile.sh" macos-aarch64-uint128 uint128
    run macos-x86_64-uint128 "$here/transpile.sh" macos-x86_64-uint128 uint128 \
        --target=x86_64-apple-darwin
    run macos-aarch64-uint128-hooks "$here/transpile.sh" --test-hooks \
        macos-aarch64-uint128-hooks uint128
    linux="$here/in-container.sh"
else
    linux=
fi

run linux-armv7-ansi32 "$here/transpile.sh" linux-armv7-ansi32 ansi32 \
    --target=armv7-linux-gnueabihf
run windows-x86_64-uint128 "$here/transpile.sh" windows-x86_64-uint128 uint128 \
    --target=x86_64-pc-windows-gnu
run windows-aarch64-uint128 "$here/transpile.sh" windows-aarch64-uint128 uint128 \
    --target=aarch64-w64-mingw32
run windows-i686-ansi32 "$here/transpile.sh" windows-i686-ansi32 ansi32 \
    --target=i686-pc-windows-gnu
run linux-aarch64-uint128 $linux c2rust/transpile.sh linux-aarch64-uint128 uint128
run linux-x86_64-uint128 $linux c2rust/transpile.sh linux-x86_64-uint128 uint128 \
    --target=x86_64-linux-gnu --sysroot=/usr/x86_64-linux-gnu
run linux-i686-ansi32 $linux c2rust/transpile.sh linux-i686-ansi32 ansi32 \
    --target=i686-linux-gnu --sysroot=/usr/i686-linux-gnu
run linux-x86_64-uint128-hooks $linux c2rust/transpile.sh --test-hooks \
    linux-x86_64-uint128-hooks uint128 \
    --target=x86_64-linux-gnu --sysroot=/usr/x86_64-linux-gnu
run linux-i686-ansi32-hooks $linux c2rust/transpile.sh --test-hooks \
    linux-i686-ansi32-hooks ansi32 \
    --target=i686-linux-gnu --sysroot=/usr/i686-linux-gnu

if [ -n "$failed" ]; then
    echo "failed:$failed" >&2
    exit 1
fi

# With the full matrix: check the oracles, merge into c2rust/work/merged,
# deduplicate the merged crate and install it in the repository root.
if [ -z "$only" ]; then
    "$here/check-oracles.sh" || exit 1
    out=c2rust/out
    merged=c2rust/work/merged
    set -- \
        "python3 -I c2rust/merge.py $out/linux-x86_64-uint128/mpdec $out/linux-i686-ansi32/mpdec $merged/plain" \
        "python3 -I c2rust/merge.py $out/linux-x86_64-uint128-hooks/mpdec $out/linux-i686-ansi32-hooks/mpdec $merged/hooks" \
        "python3 -I c2rust/merge.py --feature test-hooks $merged/plain $merged/hooks $merged/mpdec" \
        "python3 -I c2rust/dedupe.py $merged/mpdec" \
        "python3 -I c2rust/install.py $merged/mpdec"
    (
        cd "$here/.." && rm -rf "$merged"
        for cmd in "$@"; do
            echo "\$ $cmd"
            $cmd || exit 1
        done
    ) > "$here/out/merged.log" 2>&1 || { tail -5 "$here/out/merged.log"; exit 1; }
    awk '/^(\$|total|common|installed)/' "$here/out/merged.log"
    {
        echo
        echo "## $(date -u '+%Y-%m-%dT%H:%M:%SZ') merged"
        echo
        echo '```sh'
        echo "c2rust/check-oracles.sh"
        for cmd in "$@"; do
            echo "$cmd"
        done
        echo '```'
    } >> "$here/RUNS.md"
fi
exit
}
