#!/bin/sh
#
# Translate libmpdec for one configuration and record the run.
#
# usage: c2rust/transpile.sh [--test-hooks] LABEL MACHINE [CLANG_ARG...]
#
#   --test-hooks  translate the sources with the test hooks of testit
#             (prepare.py --test-hooks, see test-hooks/README.md)
#   LABEL     output name: c2rust/out/LABEL/mpdec (translated crate after
#             normalize.py, named mpdec so that the staticlib is libmpdec.a
#             in every configuration), c2rust/out/LABEL/transpile.log,
#             c2rust/work/LABEL (prepared input and the translation before
#             normalize.py in raw/, not committed)
#   MACHINE   uint128 | ansi64 | ansi32 (see prepare.py)
#   CLANG_ARG extra clang arguments, e.g. --target=i686-linux-gnu
#
# Environment:
#   C2RUST_FLAGS  overrides the default c2rust transpile flags below
#
# Every invocation appends its exact commands and result to c2rust/RUNS.md.

set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")

hooks=
if [ "${1:-}" = --test-hooks ]; then
    hooks=" --test-hooks"
    shift
fi

if [ $# -lt 2 ]; then
    sed -n '3,17p' "$0" >&2
    exit 2
fi

label=$1
machine=$2
shift 2

work=$here/work/$label
out=$here/out/$label
crate=$out/mpdec
log=$out/transpile.log

: "${C2RUST_FLAGS:=--emit-build-files --overwrite-existing}"

rm -rf "$work" "$out"
mkdir -p "$work" "$out"

prepare_cmd="python3 -I c2rust/prepare.py --machine $machine$hooks --workdir c2rust/work/$label -- $*"
transpile_cmd="c2rust transpile $C2RUST_FLAGS -o c2rust/out/$label/mpdec c2rust/work/$label/compile_commands.json"
case $machine in
    *32) config=CONFIG_32 ;;
    *) config=CONFIG_64 ;;
esac
normalize_cmd="python3 -I c2rust/normalize.py c2rust/out/$label/mpdec $config"

cd "$root"
{
    echo "\$ $prepare_cmd"
    # shellcheck disable=SC2086
    # shellcheck disable=SC2086
    python3 -I c2rust/prepare.py --machine "$machine"$hooks --workdir "$work" -- "$@"
    echo "\$ $transpile_cmd"
} > "$work/prepare.log" 2>&1

status=0
# shellcheck disable=SC2086
c2rust transpile $C2RUST_FLAGS -o "$crate" "$work/compile_commands.json" \
    > "$work/transpile.log" 2>&1 || status=$?
if [ "$status" = 0 ]; then
    mkdir -p "$work/raw"
    cp -R "$crate" "$work/raw/"
    {
        echo "\$ $normalize_cmd"
        python3 -I "$here/normalize.py" "$crate" "$config"
    } >> "$work/transpile.log" 2>&1 || status=$?
fi
cat "$work/prepare.log" "$work/transpile.log" > "$log"

if command -v llvm-config >/dev/null 2>&1; then
    llvm=$(llvm-config --version)
elif [ -n "${LLVM_CONFIG_PATH:-}" ]; then
    llvm=$("$LLVM_CONFIG_PATH" --version)
else
    llvm=unknown
fi

warnings=$(grep -c 'warning' "$work/transpile.log" || true)
errors=$(grep -c 'error' "$work/transpile.log" || true)

{
    echo
    echo "## $(date -u '+%Y-%m-%dT%H:%M:%SZ') $label"
    echo
    echo "- host: $(uname -sm), $(c2rust --version 2>&1 | head -1), LLVM $llvm"
    echo "- git: $(git -C "$root" rev-parse --short HEAD)$(git -C "$root" diff --quiet HEAD -- . ':!c2rust/out' ':!c2rust/RUNS.md' || echo '-dirty')"
    echo "- exit: $status, lines with 'warning': $warnings, with 'error': $errors"
    echo
    echo '```sh'
    echo "$prepare_cmd"
    echo "$transpile_cmd"
    echo "$normalize_cmd"
    echo '```'
} >> "$here/RUNS.md"

echo "$label: exit $status (warnings: $warnings, errors: $errors), log: $log"
exit "$status"
