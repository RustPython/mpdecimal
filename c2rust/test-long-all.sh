#!/bin/sh
#
# Run the long tests (test-long.sh) for the test matrix of test-all.sh.
#
#   full     all tests, with runtest* --all (hours); the test groups run at
#            the same time (TAG):
#              macos-aarch64-uint128    host
#              linux-aarch64-uint128    container, without fntcov
#              linux-x86_64-uint128     container (amd64), without fntcov
#              wasi-wasm32-ansi32       container, without cov (needs
#                                       signal handlers and tmpfile) and
#                                       the GMP tests
#   reduced  runtest* without --all, cov, test_transpose, ppro_mulmod,
#            genrandformat (emulated targets, minutes to an hour):
#              macos-x86_64-uint128, linux-i686-ansi32,
#              linux-s390x-uint128, linux-armv7-ansi32,
#              windows-x86_64-uint128
#
# On macOS the Linux labels run this script in a container (Containerfile,
# Containerfile.amd64 for linux-x86_64 and windows-x86_64). fntcov needs
# more than 48 GB of memory with CONFIG_64 and runs on the host only.
# Results: c2rust/work/LABEL/test-long-root*/results.txt.
#
# usage: c2rust/test-long-all.sh [LABEL...]   (default: all)
#
# On a Linux host (or in a container) the labels run natively or under the
# emulator of their target, so give the labels that match the host.

# The shell reads the whole block before running it: editing this file does
# not affect a running invocation.
{
set -u

here=$(cd "$(dirname "$0")" && pwd)

reduced="official additional cov test_transpose ppro_mulmod genrandformat"
groups_full="a1:additional-runtest a2:additional-runtest_static
a3:additional-runtest_static2
misc:official,cov,test_transpose,fntcov,ppro_mulmod,genrandlocale,genrandformat,genlocale
dc:deccheck
gmpk:mpd_mpz_add,mpd_mpz_sub,mpd_mpz_mul,mpd_mpz_divmod,karatsuba_fnt,karatsuba_fnt2"

# run the test groups of LABEL at the same time; $1 = LABEL, $2 = tests to
# leave out, then the command prefix (container, environment)
full() {
    label=$1
    omit=$2
    shift 2
    status=0
    "$@" "$here/test-long.sh" "$label" ppro_mulmod > /dev/null 2>&1 || status=$?
    [ "$status" = 0 ] || { echo "$label: build failed"; return 1; }
    pids=
    for g in $groups_full; do
        tag=${g%%:*}
        tests=
        for t in $(echo "${g#*:}" | tr , ' '); do
            case " $omit " in *" $t "*) ;; *) tests="$tests $t" ;; esac
        done
        [ -n "$tests" ] || continue
        # shellcheck disable=SC2086
        "$@" env TAG="$tag" "$here/test-long.sh" "$label" $tests \
            > "$here/work/$label/long-$tag.out" 2>&1 &
        pids="$pids $!"
    done
    for p in $pids; do
        wait "$p" || status=1
    done
    return $status
}

run() {
    label=$1
    shift
    if [ -n "$only" ]; then
        case " $only " in *" $label "*) ;; *) return 0 ;; esac
    fi
    mkdir -p "$here/work/$label"
    echo "=== $label"
    if "$@"; then
        echo PASS
    else
        echo "FAIL, see c2rust/work/$label/test-long-root*/results.txt"
        failed="$failed $label"
    fi
    cat "$here/work/$label"/test-long-root*/results.txt 2>/dev/null
}

only="$*"
failed=

wasi_cc="/opt/wasi-sdk/bin/clang -D_WASI_EMULATED_PROCESS_CLOCKS"
# see test-all.sh (stack size)
wasi_libs="-lm -lwasi-emulated-process-clocks -Wl,-z,stack-size=8388608 -Wl,--stack-first"

if [ "$(uname -s)" = Darwin ]; then
    run macos-aarch64-uint128 full macos-aarch64-uint128 "" env
    # shellcheck disable=SC2086
    run macos-x86_64-uint128 env RUST_TARGET=x86_64-apple-darwin \
        RUST_TOOLCHAIN=stable-x86_64-apple-darwin CC="cc -arch x86_64" \
        RUNNER="arch -x86_64" NO_GMP=1 RUNTEST_ALL= \
        "$here/test-long.sh" macos-x86_64-uint128 $reduced
    # the Linux labels run this script in a container, all groups in one
    in_container() {
        label=$1
        shift
        run "$label" env CONTAINER_CPUS=8 CONTAINER_MEMORY=24G "$@" \
            "$here/in-container.sh" c2rust/test-long-all.sh "$label"
    }
    for label in linux-aarch64-uint128 wasi-wasm32-ansi32 linux-i686-ansi32 \
                 linux-s390x-uint128 linux-armv7-ansi32; do
        in_container "$label"
    done
    for label in linux-x86_64-uint128 windows-x86_64-uint128; do
        in_container "$label" CONTAINER_IMAGE=mpdecimal-amd64 CONTAINER_ARCH=amd64
    done
else
    run linux-aarch64-uint128 full linux-aarch64-uint128 fntcov env
    run linux-x86_64-uint128 full linux-x86_64-uint128 fntcov env
    run wasi-wasm32-ansi32 full wasi-wasm32-ansi32 \
        "cov mpd_mpz_add mpd_mpz_sub mpd_mpz_mul mpd_mpz_divmod" \
        env RUST_TARGET=wasm32-wasip1 CC="$wasi_cc" SYSLIBS="$wasi_libs" \
        RUNNER="wasmtime run --dir=." NO_GMP=1 COPY_TESTS=1
    # shellcheck disable=SC2086
    {
    run linux-i686-ansi32 env RUST_TARGET=i686-unknown-linux-gnu \
        CC=i686-linux-gnu-gcc RUNNER="qemu-i386 -L /usr/i686-linux-gnu" NO_GMP=1 \
        RUNTEST_ALL= "$here/test-long.sh" linux-i686-ansi32 $reduced
    run linux-s390x-uint128 env RUST_TARGET=s390x-unknown-linux-gnu \
        CC=s390x-linux-gnu-gcc RUNNER="qemu-s390x -L /usr/s390x-linux-gnu" NO_GMP=1 \
        RUNTEST_ALL= "$here/test-long.sh" linux-s390x-uint128 $reduced
    run linux-armv7-ansi32 env RUST_TARGET=armv7-unknown-linux-gnueabihf \
        CC=arm-linux-gnueabihf-gcc RUNNER="qemu-arm -L /usr/arm-linux-gnueabihf" \
        NO_GMP=1 RUNTEST_ALL= "$here/test-long.sh" linux-armv7-ansi32 $reduced
    run windows-x86_64-uint128 env RUST_TARGET=x86_64-pc-windows-gnu \
        CC=x86_64-w64-mingw32-gcc \
        SYSLIBS="-lkernel32 -lntdll -luserenv -lws2_32 -ldbghelp" \
        EXE=.exe RUNNER=/usr/lib/wine/wine64 NO_GMP=1 RUNTEST_ALL= \
        "$here/test-long.sh" windows-x86_64-uint128 $reduced
    }
fi

if [ -n "$failed" ]; then
    echo "failed:$failed" >&2
    exit 1
fi
exit
}
