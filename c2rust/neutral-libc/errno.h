/* Neutral <errno.h> for translation, see README.md.

   libmpdec reads errno only after strtol()/strtoll() (io.c, io.h). These are
   redirected to the shim by stdlib.h, and the shim reports their errors
   through this private errno, so neither the location nor the values of the
   errno of the C library are needed. */

#ifndef MPD_C2RUST_ERRNO_H
#define MPD_C2RUST_ERRNO_H

int *mpd_c2rust_errno(void);

#define errno (*mpd_c2rust_errno())
#define EINVAL 22
#define ERANGE 34

#endif
