/* Neutral <locale.h> for translation, see README.md.

   Only the members of struct lconv that libmpdec reads are declared. They are
   the first three members, in this order, in the C standard and in every
   supported C library (glibc, musl, Apple and BSD libc, the Windows CRT,
   wasi-libc), so the declaration is a prefix of the real struct lconv and the
   result of localeconv() of the C library can be read through it. */

#ifndef MPD_C2RUST_LOCALE_H
#define MPD_C2RUST_LOCALE_H

struct lconv {
    char *decimal_point;
    char *thousands_sep;
    char *grouping;
};

struct lconv *localeconv(void);

#endif
