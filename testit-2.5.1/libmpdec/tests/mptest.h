/*
 * Copyright (c) 2008-2020 Stefan Krah. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */


/* This header file is required for various tests. */


#ifndef TESTS_MPTEST_H_
#define TESTS_MPTEST_H_


#ifdef _MSC_VER
  #include "vccompat.h"
#else
  #include <inttypes.h>
  #define IMPORTEXPORT
#endif


#define PRI_mpd_size_t "zu"

#if defined(CONFIG_64)
  #define PRI_mpd_uint_t PRIu64
  #define PRI_mpd_ssize_t PRIi64
  #if defined(_MSC_VER)
    #undef PRI_mpd_size_t
    #define PRI_mpd_size_t PRIu64
    #define PRI_time_t "lld"
  #else
    #define PRI_time_t "ld"
  #endif
#elif defined(CONFIG_32)
  #define PRI_mpd_uint_t PRIu32
  #define PRI_mpd_ssize_t PRIi32
  #if defined(_MSC_VER)
    #undef PRI_mpd_size_t
    #define PRI_mpd_size_t PRIu32
    #define PRI_time_t "lld"
  #elif defined (__OpenBSD__)
    #define PRI_time_t "lld"
  #elif defined(__FreeBSD__)
    #if defined(__x86_64__)
      #define PRI_time_t "ld"
    #else
      #define PRI_time_t "d"
    #endif
  #elif defined(_AIX)
      #define PRI_time_t "d"
  #else
    #define PRI_time_t "ld"
  #endif
#else
  #error "define CONFIG_64 or CONFIG_32"
#endif


#ifdef TEST_COVERAGE
  #undef PRI_mpd_size_t
  #define PRI_mpd_size_t PRIu32
#endif


/* cov.c */
IMPORTEXPORT mpd_uint_t mpd_qsshiftr(mpd_t *result, const mpd_t *a, mpd_ssize_t n);

/* newton division undergoes the same rigorous tests as standard division */
IMPORTEXPORT void mpd_test_newtondiv(mpd_t *q, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx);
IMPORTEXPORT void mpd_test_newtondivint(mpd_t *q, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx);
IMPORTEXPORT void mpd_test_newtonrem(mpd_t *r, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx);
IMPORTEXPORT void mpd_test_newtondivmod(mpd_t *q, mpd_t *r, const mpd_t *a, const mpd_t *b, mpd_context_t *ctx);

/* fenv */
IMPORTEXPORT unsigned int mpd_set_fenv(void);
IMPORTEXPORT void mpd_restore_fenv(unsigned int);

IMPORTEXPORT mpd_uint_t *_mpd_fntmul(const mpd_uint_t *u, const mpd_uint_t *v, mpd_size_t ulen, mpd_size_t vlen, mpd_size_t *rsize);
IMPORTEXPORT mpd_uint_t *_mpd_kmul(const mpd_uint_t *u, const mpd_uint_t *v, mpd_size_t ulen, mpd_size_t vlen, mpd_size_t *rsize);
IMPORTEXPORT mpd_uint_t *_mpd_kmul_fnt(const mpd_uint_t *u, const mpd_uint_t *v, mpd_size_t ulen, mpd_size_t vlen, mpd_size_t *rsize);


#endif /* TESTS_MPTEST_H_ */
