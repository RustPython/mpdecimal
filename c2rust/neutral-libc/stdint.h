/* Neutral <stdint.h> for translation, see README.md.

   The exact-width types are spelled the same on every platform (64-bit types
   as long long), so that the translation does not depend on whether a libc
   uses long or long long for them. All supported platforms have 8/16/32/64
   bit char/short/int/long long. */

#ifndef MPD_C2RUST_STDINT_H
#define MPD_C2RUST_STDINT_H

typedef signed char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long long int64_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;

typedef __INTPTR_TYPE__ intptr_t;
typedef __UINTPTR_TYPE__ uintptr_t;
typedef long long intmax_t;
typedef unsigned long long uintmax_t;

#define INT8_MAX 127
#define INT8_MIN (-INT8_MAX - 1)
#define UINT8_MAX 255
#define INT16_MAX 32767
#define INT16_MIN (-INT16_MAX - 1)
#define UINT16_MAX 65535
#define INT32_MAX 2147483647
#define INT32_MIN (-INT32_MAX - 1)
#define UINT32_MAX 4294967295U
#define INT64_MAX 9223372036854775807LL
#define INT64_MIN (-INT64_MAX - 1)
#define UINT64_MAX 18446744073709551615ULL

#define SIZE_MAX __SIZE_MAX__

#define INT32_C(c) c
#define UINT32_C(c) c ## U
#define INT64_C(c) c ## LL
#define UINT64_C(c) c ## ULL

#endif
