/* Neutral <stdlib.h> for translation, see README.md.

   strtol() and strtoll() are implemented by the shim because errno is
   private (see errno.h). Other functions that set errno are not declared,
   so that a new use of them fails to translate. */

#ifndef MPD_C2RUST_STDLIB_H
#define MPD_C2RUST_STDLIB_H

#include <stddef.h>

void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void abort(void);

long mpd_c2rust_strtol(const char *nptr, char **endptr, int base);
long long mpd_c2rust_strtoll(const char *nptr, char **endptr, int base);

#define strtol mpd_c2rust_strtol
#define strtoll mpd_c2rust_strtoll

#endif
