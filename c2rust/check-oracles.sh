#!/bin/sh
#
# Compare the normalized translations of each configuration with the one
# that merge.py uses (linux-x86_64-uint128 for CONFIG_64, linux-i686-ansi32
# for CONFIG_32, linux-x86_64-uint128-hooks for CONFIG_64 with the test
# hooks). They must be identical, except for the differences recorded in
# expected/LABEL.diff (diff -r output without hunk line numbers):
#
#   linux-aarch64-uint128,  char is unsigned: mpd_validate_lconv() has no
#   linux-armv7-ansi32      check for negative grouping values, which cannot
#                           occur with an unsigned char (the check in the
#                           merged crate is false for every value there).
#   windows-*               __FILE__ (mpd_err_fatal) uses backslashes, as
#                           C compilers do for Windows targets.
#   windows-*-uint128       size_t and uint64_t are the same C type (unsigned
#                           long long, LLP64), so the translation has no
#                           conversions between them (mpd_size_t and
#                           mpd_uint_t) where LP64 has (unsigned long and
#                           unsigned long long). Both are 64-bit unsigned.
#
# usage: c2rust/check-oracles.sh

set -u

here=$(cd "$(dirname "$0")" && pwd)
cd "$here/out"

status=0

# diff -r without the line numbers of the hunks
difference() {
    diff -r "$1/mpdec/src" "$2/mpdec/src" | sed 's/^[0-9,]*[acd][0-9,]*$/@@/'
}

check() {
    base=$1
    label=$2
    expected=$here/expected/$label.diff
    [ -f "$expected" ] || expected=/dev/null
    if difference "$base" "$label" | diff "$expected" - > /dev/null; then
        echo "$label: as expected"
    else
        echo "$label: unexpected difference from $base:"
        difference "$base" "$label" | diff "$expected" -
        status=1
    fi
}

for label in macos-aarch64-uint128 macos-x86_64-uint128 linux-aarch64-uint128 \
             windows-x86_64-uint128 windows-aarch64-uint128; do
    check linux-x86_64-uint128 "$label"
done
for label in linux-armv7-ansi32 windows-i686-ansi32; do
    check linux-i686-ansi32 "$label"
done
check linux-x86_64-uint128-hooks macos-aarch64-uint128-hooks
exit $status
