/* Neutral <limits.h> for translation, see README.md.

   CHAR_MIN/CHAR_MAX keep the signedness of char of the target, which is a
   real platform difference (e.g. signed on macOS arm64, unsigned on Linux
   aarch64). */

#ifndef MPD_C2RUST_LIMITS_H
#define MPD_C2RUST_LIMITS_H

#define CHAR_BIT 8
#define SCHAR_MAX 127
#define SCHAR_MIN (-SCHAR_MAX - 1)
#define UCHAR_MAX 255
#ifdef __CHAR_UNSIGNED__
#define CHAR_MIN 0
#define CHAR_MAX UCHAR_MAX
#else
#define CHAR_MIN SCHAR_MIN
#define CHAR_MAX SCHAR_MAX
#endif
#define SHRT_MAX 32767
#define SHRT_MIN (-SHRT_MAX - 1)
#define USHRT_MAX 65535
#define INT_MAX 2147483647
#define INT_MIN (-INT_MAX - 1)
#define UINT_MAX 4294967295U
#define LONG_MAX __LONG_MAX__
#define LONG_MIN (-LONG_MAX - 1L)
#define ULONG_MAX (LONG_MAX * 2UL + 1UL)
#define LLONG_MAX 9223372036854775807LL
#define LLONG_MIN (-LLONG_MAX - 1LL)
#define ULLONG_MAX 18446744073709551615ULL

#endif
