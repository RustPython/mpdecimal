/* Neutral <stddef.h> for translation, see README.md. */

#ifndef MPD_C2RUST_STDDEF_H
#define MPD_C2RUST_STDDEF_H

/* c2rust translates these typedefs of the compiler's types as usize/isize */
typedef __SIZE_TYPE__ size_t;
typedef __PTRDIFF_TYPE__ ptrdiff_t;

#define NULL ((void *)0)
#define offsetof(type, member) __builtin_offsetof(type, member)

#endif
