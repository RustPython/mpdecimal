/* Neutral <signal.h> for translation, see README.md.

   SIGFPE is 8 on Linux, macOS, the BSDs and Windows. raise() is redirected
   to the shim, which also covers targets without signals. */

#ifndef MPD_C2RUST_SIGNAL_H
#define MPD_C2RUST_SIGNAL_H

#define SIGFPE 8

int mpd_c2rust_raise(int sig);

#define raise mpd_c2rust_raise

#endif
