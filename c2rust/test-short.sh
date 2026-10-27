#!/bin/sh
#
# Run the upstream short tests (tests/runshort.sh, tests/runshort_alloc.sh)
# against a translated crate.
#
# usage: c2rust/test-short.sh LABEL
#
# The crate is built as a staticlib and linked with tests/runtest.c and
# tests/test.c, which are compiled against the header generated for LABEL by
# prepare.py.
#
# Environment:
#   CRATE           crate to test: LABEL (default, c2rust/out/LABEL/mpdec)
#                   or root (the crate of the repository root)
#   FEATURES        cargo features of the crate, e.g. test-hooks
#   RUST_TOOLCHAIN  toolchain for the generated crate (default below)
#   RUST_TARGET     cargo --target triple for cross builds
#   CC              C compiler for the tests (default: cc)
#   RUNNER          command prefix to run test binaries (e.g. qemu-i386 -L ...)
#   SYSLIBS         libraries for linking the tests (default: by host OS)
#   EXE             suffix of executables (.exe for Windows)
#   COPY_TESTS      if set, the test files are copied instead of linked
#                   (wasmtime does not follow links out of the directory)

# The shell reads the whole block before running it: editing this file does
# not affect a running invocation.
{
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")

label=${1:?usage: c2rust/test-short.sh LABEL}
: "${CRATE:=$label}"
: "${FEATURES:=}"
work=$here/work/$label
if [ "$CRATE" = root ]; then
    crate=$root
    staticlib=libmpdecimal_core.a
else
    crate=$here/out/$CRATE/mpdec
    staticlib=libmpdec.a
fi
if [ "$CRATE" = "$label" ]; then
    build=$work/test-short
    target_dir=$work/target
else
    build=$work/test-short-$CRATE${FEATURES:+-$FEATURES}
    target_dir=$work/target-$CRATE
fi

: "${RUST_TOOLCHAIN:=1.99.0}"
: "${RUST_TARGET:=}"
: "${CC:=cc}"
: "${RUNNER:=}"

# the header of a configuration that is only tested (not translated)
if [ ! -f "$work/include/mpdecimal.h" ]; then
    case $label in
        *-uint128) machine=uint128 ;;
        *-ansi32) machine=ansi32 ;;
        *) echo "unknown machine in label $label" >&2; exit 2 ;;
    esac
    python3 -I "$here/prepare.py" --machine "$machine" --workdir "$work" > /dev/null
fi

cargo_target=
profile_dir=$target_dir/debug
if [ -n "$RUST_TARGET" ]; then
    cargo_target="--target $RUST_TARGET"
    profile_dir=$target_dir/$RUST_TARGET/debug
fi

# shellcheck disable=SC2086
cargo "+$RUST_TOOLCHAIN" build --quiet --manifest-path "$crate/Cargo.toml" \
    --target-dir "$target_dir" $cargo_target ${FEATURES:+--features "$FEATURES"}

if [ -z "${SYSLIBS+set}" ]; then
    case $(uname -s) in
        Darwin) SYSLIBS="-lm" ;;
        *) SYSLIBS="-lm -lpthread -ldl" ;;
    esac
fi
: "${EXE:=}"
: "${COPY_TESTS:=}"

rm -rf "$build"
mkdir -p "$build"
for f in runshort.sh runshort_alloc.sh gettests.sh official.decTest additional.decTest \
         testdata_dist testdata; do
    if [ -n "$COPY_TESTS" ]; then
        cp -RL "$root/tests/$f" "$build/$f"
    else
        ln -s "$root/tests/$f" "$build/$f"
    fi
done

# shellcheck disable=SC2086
$CC -std=c99 -I"$work/include" -o "$build/runtest.bin$EXE" \
    "$root/tests/runtest.c" "$root/tests/test.c" \
    "$profile_dir/$staticlib" $SYSLIBS

printf '#!/bin/sh\nexec %s "%s/runtest.bin%s" "$@"\n' "$RUNNER" "$build" "$EXE" \
    > "$build/runtest"
chmod +x "$build/runtest"

cd "$build"
./runshort.sh
./runshort_alloc.sh
exit
}
