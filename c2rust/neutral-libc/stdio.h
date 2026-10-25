/* Neutral <stdio.h> for translation, see README.md.

   FILE is only handled through pointers, so it is declared as an opaque
   type. The stderr object is obtained from the shim. */

#ifndef MPD_C2RUST_STDIO_H
#define MPD_C2RUST_STDIO_H

#include <stddef.h>

typedef struct mpd_c2rust_FILE FILE;

FILE *mpd_c2rust_stderr(void);

#define stderr (mpd_c2rust_stderr())

int fprintf(FILE *stream, const char *format, ...);
int printf(const char *format, ...);
int snprintf(char *s, size_t n, const char *format, ...);
int fputc(int c, FILE *stream);
int fputs(const char *s, FILE *stream);

#endif
