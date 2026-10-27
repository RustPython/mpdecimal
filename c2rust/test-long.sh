#!/bin/sh
#
# Run the long tests of mpdecimal-testit-2.5.1 (testit-2.5.1/libmpdec/tests,
# see runalltests.sh there) against a translated crate with the test hooks.
#
# usage: c2rust/test-long.sh LABEL [TEST...]
#
#   LABEL   configuration whose header (prepare.py) the tests are compiled
#           with, e.g. macos-aarch64-uint128
#   TEST    tests to run (default: all, in the order of runalltests.sh):
#           official additional (each with runtest, runtest_static and
#           runtest_static2, e.g. official-runtest), cov test_transpose
#           fntcov deccheck ppro_mulmod mpd_mpz_add mpd_mpz_sub mpd_mpz_mul
#           mpd_mpz_divmod karatsuba_fnt karatsuba_fnt2 genrandlocale
#           genrandformat genlocale
#
# The test programs are built in c2rust/work/LABEL/test-long-CRATE from a
# copy of the test directory, with c2rust/testit/runtest-version.patch
# applied (runtest.c checks the version of the library), against the
# internal headers of libmpdec/ (4.0.1) and the staticlib of the crate. The
# Python generators run with c2rust/testit/pyshim on PYTHONPATH (distutils).
# Each test's output goes to TEST.log there and its exit status to
# results.txt.
#
# Environment:
#   CRATE           root (default: the crate of the repository root, built
#                   with --features test-hooks) or a label whose crate
#                   c2rust/out/LABEL/mpdec was translated with --test-hooks
#   RUST_TOOLCHAIN  toolchain for the crate (default below)
#   RUST_PROFILE    release (default: optimized, with overflow checks and
#                   debug assertions, as the tests run for hours) or dev
#   RUST_TARGET     cargo --target triple for cross builds
#   CC              C compiler for the tests (default: cc)
#   RUNNER          command prefix to run test binaries (e.g. qemu-i386 -L ...)
#   SYSLIBS         libraries for linking the tests (default: by host OS)
#   EXE             suffix of executables (.exe for Windows)
#   COPY_TESTS      if set, the test data is copied instead of linked
#                   (wasmtime does not follow links out of the directory)
#   PYTHON          Python 3 for the generators (default: python3)
#   GMP_FLAGS       compiler flags to find GMP (default on macOS: Homebrew)
#   NO_GMP          if set, the tests against GMP (mpd_mpz_*) are skipped
#   TIMEOUT         if set, each test is stopped after TIMEOUT seconds
#   RUNTEST_ALL     --all (default) or empty: runtest* repeat the official and
#                   additional tests for every MPD_MINALLOC (hours) or not
#                   (minutes, for emulated targets)
#   TAG             suffix of the build directory (test-long-CRATE-TAG), for
#                   running several invocations at the same time

# The shell reads the whole block before running it: editing this file does
# not affect a running invocation.
{
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
testit=$root/testit-2.5.1/libmpdec/tests

label=${1:?usage: c2rust/test-long.sh LABEL [TEST...]}
shift
: "${CRATE:=root}"
: "${RUST_TOOLCHAIN:=1.99.0}"
: "${RUST_PROFILE:=release}"
: "${RUST_TARGET:=}"
: "${CC:=cc}"
: "${RUNNER:=}"
: "${PYTHON:=python3}"
: "${TIMEOUT:=}"
: "${NO_GMP:=}"
: "${TAG:=}"
: "${RUNTEST_ALL=--all}"
if [ -z "${GMP_FLAGS+set}" ]; then
    GMP_FLAGS=
    if [ "$(uname -s)" = Darwin ]; then
        brew=$(brew --prefix)
        GMP_FLAGS="-I$brew/include -L$brew/lib"
    fi
fi

work=$here/work/$label
build=$work/test-long-$CRATE${TAG:+-$TAG}
[ -f "$work/include/mpdecimal.h" ] || {
    echo "missing $work/include/mpdecimal.h; run c2rust/transpile.sh first" >&2
    exit 1
}

case $label in
    *-uint128 | *-uint128-hooks) config="-DCONFIG_64 -DANSI -DHAVE_UINT128_T" ;;
    *-ansi32 | *-ansi32-hooks) config="-DCONFIG_32 -DANSI" ;;
    *) echo "unknown machine in label $label" >&2; exit 2 ;;
esac

# the crate
if [ "$CRATE" = root ]; then
    crate=$root
    staticlib=libmpdecimal_core.a
    features="--features test-hooks"
else
    crate=$here/out/$CRATE/mpdec
    staticlib=libmpdec.a
    features=
fi
target_dir=$work/target-long-$CRATE
case $RUST_PROFILE in
    release)
        profile_dir=release
        profile="--release --config profile.release.overflow-checks=true
                 --config profile.release.debug-assertions=true" ;;
    dev) profile_dir=debug profile= ;;
    *) echo "unknown RUST_PROFILE $RUST_PROFILE" >&2; exit 2 ;;
esac
cargo_target=
profile_dir=$target_dir/$profile_dir
if [ -n "$RUST_TARGET" ]; then
    cargo_target="--target $RUST_TARGET"
    profile_dir=$target_dir/$RUST_TARGET/${profile_dir##*/}
fi
# shellcheck disable=SC2086
cargo "+$RUST_TOOLCHAIN" build --quiet --manifest-path "$crate/Cargo.toml" \
    --target-dir "$target_dir" $cargo_target $features $profile
lib=$profile_dir/$staticlib

if [ -z "${SYSLIBS+set}" ]; then
    case $(uname -s) in
        Darwin) SYSLIBS="-lm" ;;
        *) SYSLIBS="-lm -lpthread -ldl" ;;
    esac
fi
syslibs=$SYSLIBS
: "${EXE:=}"
: "${COPY_TESTS:=}"

# the test programs (flags of testit's configure for gcc/clang, with the
# configuration of LABEL)
rm -rf "$build"
mkdir -p "$build"
cp "$testit"/*.c "$testit"/*.h "$testit"/*.py "$testit"/*.decTest "$build/"
for d in testdata decNumber; do
    [ -d "$testit/$d" ] || {
        echo "missing $testit/$d; run gettests.sh and getdn.sh there" >&2
        exit 1
    }
    if [ -n "$COPY_TESTS" ]; then
        cp -R "$testit/$d" "$build/$d"
    else
        ln -s "$testit/$d" "$build/$d"
    fi
done
patch -s -d "$build" -p1 < "$here/testit/runtest-version.patch"

cflags="-Wall -W -Wno-unknown-pragmas -Wno-overlength-strings -std=c99 -pedantic $config -O2"
inc="-I$work/include -I$root/libmpdec"
cd "$build"
compile() {
    out=$1
    shift
    case $out in *.o) exe= ;; *) exe=$EXE ;; esac
    # shellcheck disable=SC2086
    $CC $inc $cflags -o "$out$exe" "$@" > "build-$out.log" 2>&1 || {
        echo "build of $out failed, see $build/build-$out.log" >&2
        cat "build-$out.log" >&2
        exit 1
    }
}

all="official-runtest official-runtest_static official-runtest_static2
additional-runtest additional-runtest_static additional-runtest_static2
cov test_transpose fntcov deccheck ppro_mulmod
mpd_mpz_add mpd_mpz_sub mpd_mpz_mul mpd_mpz_divmod karatsuba_fnt karatsuba_fnt2
genrandlocale genrandformat genlocale"

selected=
for t in ${*:-$all}; do
    case $t in
        official | additional)
            selected="$selected $t-runtest $t-runtest_static $t-runtest_static2" ;;
        *) selected="$selected $t" ;;
    esac
done

# build the programs of the selected tests only (as in tests/Makefile.in)
built=" "
# shellcheck disable=SC2086
build_program() {
    case $built in *" $1 "*) return 0 ;; esac
    built="$built$1 "
    case $1 in
        malloc_fail.o) compile malloc_fail.o -c malloc_fail.c ;;
        runtest | runtest_static | runtest_static2 | fntcov)
            build_program malloc_fail.o
            compile $1 $1.c malloc_fail.o "$lib" $syslibs ;;
        cov)
            build_program malloc_fail.o
            compile cov cov.c triple_cases.c malloc_fail.o "$lib" $syslibs ;;
        karatsuba_fnt | karatsuba_fnt2 | ppro_mulmod | test_transpose)
            compile $1 $1.c "$lib" $syslibs ;;
        mpd_mpz_*) compile $1 $GMP_FLAGS $1.c "$lib" $syslibs -lgmp ;;
        deccheck)
            compile deccheck -IdecNumber deccheck.c decNumber/decNumber.c \
                decNumber/decContext.c "$lib" $syslibs ;;
        *) echo "unknown test program $1" >&2; exit 2 ;;
    esac
}
for t in $selected; do
    case $t in
        official-* | additional-*) build_program "${t#*-}" ;;
        gen*) build_program runtest ;;
        mpd_mpz_*) [ -n "$NO_GMP" ] || build_program "$t" ;;
        *) build_program "$t" ;;
    esac
done

command_of() {
    case $1 in
        official-* | additional-*) echo "$RUNNER ./${1#*-}$EXE $RUNTEST_ALL ${1%%-*}.decTest" ;;
        mpd_mpz_* | karatsuba_fnt*) echo "$RUNNER ./$1$EXE -q" ;;
        gen*) echo "{ $PYTHON $1.py; echo \$? > $1.status; } | $RUNNER ./runtest$EXE -" ;;
        cov | test_transpose | fntcov | deccheck | ppro_mulmod) echo "$RUNNER ./$1$EXE" ;;
        *) return 1 ;;
    esac
}

export PYTHONPATH="$here/testit/pyshim"
: > results.txt
failed=
for t in $selected; do
    cmd=$(command_of "$t") || { echo "unknown test $t" >&2; exit 2; }
    case $t in
        mpd_mpz_*) if [ -n "$NO_GMP" ]; then
                       echo "$t SKIP (NO_GMP)" | tee -a results.txt
                       continue
                   fi ;;
    esac
    start=$(date +%s)
    status=0
    if [ -n "$TIMEOUT" ]; then
        # kill the process group of the test after TIMEOUT seconds
        perl -e '$t = shift; $pid = fork; if (!$pid) { setpgrp(0, 0); exec @ARGV or die "exec: $!" }
                 $SIG{ALRM} = sub { kill "KILL", -$pid }; alarm $t; waitpid($pid, 0);
                 exit($? & 127 ? 128 + ($? & 127) : $? >> 8)' "$TIMEOUT" \
            sh -c "$cmd" > "$t.log" 2>&1 || status=$?
    else
        sh -c "$cmd" > "$t.log" 2>&1 || status=$?
    fi
    secs=$(($(date +%s) - start))
    # the exit status of a generator (gen*.py) is written to TEST.status
    if [ "$status" = 0 ] && [ -f "$t.status" ] && [ "$(cat "$t.status")" != 0 ]; then
        status="generator $(cat "$t.status")"
    fi
    if [ "$status" = 0 ]; then
        result=PASS
    else
        result="FAIL (exit $status)"
        failed="$failed $t"
    fi
    echo "$t $result ${secs}s: $cmd" | tee -a results.txt
done

if [ -n "$failed" ]; then
    echo "failed:$failed" >&2
    exit 1
fi
exit
}
