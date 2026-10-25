#ifdef __GNUC__
  #define _GNU_SOURCE
#endif

#include "mpdecimal.h"

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "malloc_fail.h"
#include "mptest.h"


/* This file contains coverage tests for rarely exercised code paths. */

#ifndef _MSC_VER
  #include <inttypes.h>
  #define ASSERT(p) if (!(p)) {abort();}
#else
  #define ASSERT(p) if (!(p)) {mpd_err_fatal("assertion failed");}
#endif

#define BUFSIZE 500
#if MPD_MAX_FLAG_LIST > BUFSIZE
  #error "insufficient buffer length"
#endif

extern const char *triple_cases[];

void mpd_set_func_fail(int n);
void mpd_set_func(void);


static void mpd_testcontext(mpd_context_t *ctx)
{
    mpd_defaultcontext(ctx);
    ctx->prec = 28;
}

void
mpd_custom_traphandler(mpd_context_t *ctx)
{
    if (ctx->status & MPD_Malloc_error) {
        fprintf(stderr, "\ncov: out of memory: SKIP\n\n");
        exit(0);
    }

    raise(SIGFPE);
}

static void
test_traphandler(mpd_context_t *ctx)
{
    (void)ctx;
}

#ifndef LEGACY_COMPILER
uint64_t
randbits(void)
{
    return (uint64_t)rand()<<48 | (uint64_t)rand()<<32 |
           (uint64_t)rand()<<16 | (uint64_t)rand();
}
#else
uint32_t
randbits(void)
{
    return (uint32_t)rand()<<16 | (uint32_t)rand();
}
#endif

static int
equalmem(const mpd_t *a, const mpd_t *b)
{
    mpd_ssize_t i;

    /* assume that both operands have the same dataflags */
    if (a->flags != b->flags) return 0;
    if (a->exp != b->exp) return 0;
    if (a->len != b->len) return 0;
    if (a->digits != b->digits) return 0;
    for (i = 0; i < a->len; i++)
        if (a->data[i] != b->data[i])
            return 0;
    return 1;
}

static void
check_equalmem(const mpd_t *a, const mpd_t *b)
{
    if (!equalmem(a, b)) {
        fprintf(stderr, "FAIL: decimals are not identical\n");
        abort();
    }
}


/******************************************************************************/
/*                               Random decimal                               */
/******************************************************************************/

static mpd_ssize_t rt_pow10[] = {
  1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000
};

/* string operands */
static char op[10000];

static inline char *
indicator(char *s)
{
    *s++ = random()%2 ? 'e' : 'E';
    *s = '\0';
    return s;
}

static inline char *
digits(char *s, mpd_ssize_t len)
{
    mpd_ssize_t n;

    for (n = 0; n < len; n++) {
        *s++ = random()%10 + '0';
    }
    *s = '\0';
    return s;
}

static inline char *
decimal_part(char *s, mpd_ssize_t len)
{
    long x = random()%100;
    mpd_ssize_t intlen, fraclen;

    if (x > 80) {
        s = digits(s, len);
    }
    else if (x > 20) {
        intlen = random()%(len+1);
        fraclen = len - intlen;
        s = digits(s, intlen);
        *s++ = '.';
        s = digits(s, fraclen);
    }
    else {
        intlen = random()%(len+1)+1;
        *s++ = '.';
        s = digits(s, intlen);
    }
    return s;
}

static inline char *
expdigits(char *s)
{
    mpd_ssize_t x, q, d;
    int j;

    x = 200;

    if (random()%2) {
        *s++ = '-';
         x = random() % (2*x+1);
    }
    else {
        if (random()%2)
            *s++ = '+';
        x = random() % (x+1);
    }

    j = mpd_exp_digits(x) - 1;

    for (; j != 0; --j) {
        d = rt_pow10[j];
        q = x / d;
        x -= d * q;
        *s++ = '0' + (char)q;
    }
    *s++ = '0' + (char)x;
    *s = '\0';

    return s;
}

static inline char *
exponentpart(char *s)
{
    s = indicator(s);
    s = expdigits(s);
    return s;
}

static inline char *
infinity(char *s)
{
    *s++ = 'i';
    *s++ = 'n';
    *s++ = 'f';
    if (random()%2) {
        *s++ = 'i';
        *s++ = 'n';
        *s++ = 'i';
        *s++ = 't';
        *s++ = 'y';
    }
    *s = '\0';
    return s;
}

static inline char *
dcnan(char *s, mpd_ssize_t len)
{
    if (random()%2) {
        *s++ = 's';
    }
    *s++ = 'N';
    *s++ = 'a';
    *s++ = 'N';
    *s = '\0';
    if (random()%2) {
        s = digits(s, len);
    }
    return s;
}

static inline char *
numeric_value(char *s, mpd_ssize_t maxprec)
{
    long x = random()%100;
    if (x > 95) {
        return infinity(s);
    }
    s = decimal_part(s, maxprec);
    if (x > 20) {
        s = exponentpart(s);
    }
    return s;
}

static inline char *
numeric_string(char *s, mpd_ssize_t maxprec)
{
    long x = random()%100;
    if (x > 98) {
        return dcnan(s, maxprec);
    }
    return numeric_value(s, maxprec);
}

static inline char *
randdec(char *s, mpd_ssize_t maxprec)
{
    return numeric_string(s, maxprec);
}

static void
single_triple(mpd_t *a, mpd_t *b, mpd_t *coeff, mpd_t *maxcoeff, const char *s)
{
    mpd_uint128_triple_t triple = { MPD_TRIPLE_ERROR, 0, 0, 0, 0 };
    uint32_t status = 0;

    mpd_qset_string_exact(a, s, &status);
    if (status != 0) {
        fprintf(stderr, "unexpected conversion error\n");
        abort();
    }

    status = 0;
    triple = mpd_as_uint128_triple(a);
    mpd_from_uint128_triple(b, &triple, &status);

    switch (triple.tag) {
    case MPD_TRIPLE_ERROR: {

        if (!mpd_qcopy(coeff, a, &status)) {
            fprintf(stderr, "unexpected conversion error\n");
            abort();
        }

        mpd_set_positive(coeff);
        coeff->exp = 0;

        status = 0;
        if (mpd_qcmp(coeff, maxcoeff, &status) > 0) {
            ASSERT(mpd_isqnan(b))
        }
        else {
            fprintf(stderr, "unexpected conversion error\n");
            abort();
        }

        break;
    }

    default:
        if (mpd_cmp_total(b, a) != 0) {
            fprintf(stderr, "unexpected conversion error\n");
            abort();
        }
        check_equalmem(b, a);
    }
}

static void
test_triple(void)
{
    uint32_t status = 0;
    mpd_t *a, *b;
    mpd_t *coeff, *maxcoeff;

    a = mpd_qnew();
    if (a == NULL) {
        fprintf(stderr, "out of memory\n");
        abort();
    }

    b = mpd_qnew();
    if (b == NULL) {
        fprintf(stderr, "out of memory\n");
        abort();
    }

    coeff = mpd_qnew();
    if (coeff == NULL) {
        fprintf(stderr, "out of memory\n");
        abort();
    }

    maxcoeff = mpd_qnew();
    if (maxcoeff == NULL) {
        fprintf(stderr, "out of memory\n");
        abort();
    }

    mpd_qset_string_exact(maxcoeff, "340282366920938463463374607431768211455", &status);
    if (status != 0) {
        fprintf(stderr, "out of memory\n");
        abort();
    }

    for (mpd_ssize_t prec = 1; prec <= 40; prec++) {
        for (int64_t i = 0; i < 10000000; i++) {
            randdec(op, prec);
            single_triple(a, b, coeff, maxcoeff, op);
        }
    }

    for (const char **s = triple_cases; *s != NULL; s++) {
        single_triple(a, b, coeff, maxcoeff, *s);
    }

    mpd_uint128_triple_t triple = { MPD_TRIPLE_QNAN, 2, 0, 0, 0 };

    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.sign = 0;
    triple.exp = 1;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.tag = MPD_TRIPLE_INF;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.tag = MPD_TRIPLE_NORMAL;
    triple.sign = 2;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.tag = MPD_TRIPLE_NORMAL;
    triple.sign = 0;
    triple.exp = INT64_MAX;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.tag = MPD_TRIPLE_NORMAL;
    triple.sign = 0;
    triple.exp = INT64_MIN;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.tag = MPD_TRIPLE_NORMAL;
    triple.sign = 0;
    triple.exp = MPD_SSIZE_MAX;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.tag = MPD_TRIPLE_NORMAL;
    triple.sign = 0;
    triple.exp = MPD_SSIZE_MIN;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))

    triple.tag = (enum mpd_triple_class)10;
    triple.sign = 0;
    status = 0;
    mpd_from_uint128_triple(b, &triple, &status);
    ASSERT(status == MPD_Conversion_syntax)
    ASSERT(mpd_isqnan(b))


    mpd_del(maxcoeff);
    mpd_del(coeff);
    mpd_del(b);
    mpd_del(a);
}


/*****************************************************************************/
/*                      Inttype conversion functions                         */
/*****************************************************************************/

#define TEST_GETSET_INTTYPE(TYPE, FMTSPEC) \
static void                                                \
getset_##TYPE(                                             \
    mpd_t *a, TYPE x, mpd_context_t *ctx,                  \
    TYPE (* getfunc)(const mpd_t *, mpd_context_t *ctx),   \
    void (* setfunc)(mpd_t *, TYPE, mpd_context_t *ctx))   \
{                                                          \
        char buf[BUFSIZE];                                 \
        char *s;                                           \
        TYPE y;                                            \
                                                           \
        setfunc(a, x, ctx);                                \
                                                           \
        if (getfunc != NULL) {                             \
                y = getfunc(a, ctx);                       \
                ASSERT(x == y)                             \
        }                                                  \
                                                           \
        snprintf(buf, BUFSIZE, "%" FMTSPEC, x);            \
        s = mpd_to_sci(a, 0);                              \
        ASSERT(strcmp(s, buf) == 0)                        \
        mpd_free(s);                                       \
}

#define TEST_GETSET_INTTYPE_EXACT(TYPE, FMTSPEC) \
static void                                                \
getset_exact_##TYPE(                                       \
    mpd_t *a, TYPE x, mpd_context_t *ctx,                  \
    TYPE (* getfunc)(const mpd_t *, mpd_context_t *ctx),   \
    void (* setfunc)(mpd_t *, TYPE, uint32_t *status))     \
{                                                          \
        uint32_t status = 0;                               \
        char buf[BUFSIZE];                                 \
        char *s;                                           \
        TYPE y;                                            \
                                                           \
        setfunc(a, x, &status);                            \
        ASSERT(status == 0)                                \
                                                           \
        if (getfunc != NULL) {                             \
                y = getfunc(a, ctx);                       \
                ASSERT(x == y)                             \
        }                                                  \
                                                           \
        snprintf(buf, BUFSIZE, "%" FMTSPEC, x);            \
        s = mpd_to_sci(a, 0);                              \
        ASSERT(strcmp(s, buf) == 0)                        \
        mpd_free(s);                                       \
}

TEST_GETSET_INTTYPE(mpd_ssize_t, PRI_mpd_ssize_t)
TEST_GETSET_INTTYPE(int32_t, PRIi32)
TEST_GETSET_INTTYPE(mpd_uint_t, PRI_mpd_uint_t)
TEST_GETSET_INTTYPE(uint32_t, PRIu32)
#ifndef LEGACY_COMPILER
TEST_GETSET_INTTYPE(int64_t, PRIi64)
TEST_GETSET_INTTYPE(uint64_t, PRIu64)
TEST_GETSET_INTTYPE_EXACT(int64_t, PRIi64)
TEST_GETSET_INTTYPE_EXACT(uint64_t, PRIu64)
#endif

static void
test_int_conversions(void)
{
    uint32_t status;
    mpd_context_t ctx;
    mpd_t *a;
    mpd_uint_t data[MPD_MINALLOC_MAX];
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, data};
    int i;

    mpd_testcontext(&ctx);
    a = mpd_new(&ctx);

#ifdef LEGACY_COMPILER
    #define mpd_get_i64 NULL
    #define mpd_get_u64 NULL
#endif

    /* mpd_ssize_t */
    getset_mpd_ssize_t(a, MPD_SSIZE_MIN, &ctx, mpd_get_ssize, mpd_set_ssize);
    getset_mpd_ssize_t(a, MPD_SSIZE_MAX, &ctx, mpd_get_ssize, mpd_set_ssize);
    for (i = 0; i < 100000; i++) {
        getset_mpd_ssize_t(a, (mpd_ssize_t)randbits(), &ctx,
                           mpd_get_ssize, mpd_set_ssize);
    }

    getset_mpd_ssize_t(&b, MPD_SSIZE_MIN, &ctx, mpd_get_ssize, mpd_sset_ssize);
    getset_mpd_ssize_t(&b, MPD_SSIZE_MAX, &ctx, mpd_get_ssize, mpd_sset_ssize);
    for (i = 0; i < 100000; i++) {
        getset_mpd_ssize_t(&b, (mpd_ssize_t)randbits(), &ctx,
                           mpd_get_ssize, mpd_sset_ssize);
    }

    /* int32_t */
    getset_int32_t(a, INT32_MIN, &ctx, mpd_get_i32, mpd_set_i32);
    getset_int32_t(a, INT32_MAX, &ctx, mpd_get_i32, mpd_set_i32);
    for (i = 0; i < 100000; i++) {
        getset_int32_t(a, (int32_t)randbits(), &ctx,
                       mpd_get_i32, mpd_set_i32);
    }

    getset_int32_t(&b, INT32_MIN, &ctx, mpd_get_i32, mpd_sset_i32);
    getset_int32_t(&b, INT32_MAX, &ctx, mpd_get_i32, mpd_sset_i32);
    for (i = 0; i < 100000; i++) {
        getset_int32_t(&b, (int32_t)randbits(), &ctx,
                       mpd_get_i32, mpd_sset_i32);
    }

#ifndef LEGACY_COMPILER
    /* int64_t */
    getset_int64_t(a, INT64_MIN, &ctx, mpd_get_i64, mpd_set_i64);
    getset_int64_t(a, INT64_MAX, &ctx, mpd_get_i64, mpd_set_i64);
    for (i = 0; i < 100000; i++) {
        getset_int64_t(a, (int64_t)randbits(), &ctx,
                       mpd_get_i64, mpd_set_i64);
    }
#endif

#ifdef CONFIG_64
    getset_int64_t(&b, INT64_MIN, &ctx, mpd_get_i64, mpd_sset_i64);
    getset_int64_t(&b, INT64_MAX, &ctx, mpd_get_i64, mpd_sset_i64);
    for (i = 0; i < 100000; i++) {
        getset_int64_t(&b, (int64_t)randbits(), &ctx,
                       mpd_get_i64, mpd_sset_i64);
    }
#endif

    /* mpd_uint_t */
    getset_mpd_uint_t(a, 0, &ctx, mpd_get_uint, mpd_set_uint);
    getset_mpd_uint_t(a, MPD_UINT_MAX, &ctx, mpd_get_uint, mpd_set_uint);
    for (i = 0; i < 100000; i++) {
        getset_mpd_uint_t(a, (mpd_uint_t)randbits(), &ctx,
                          mpd_get_uint, mpd_set_uint);
    }

    getset_mpd_uint_t(&b, 0, &ctx, mpd_get_uint, mpd_sset_uint);
    getset_mpd_uint_t(&b, MPD_UINT_MAX, &ctx, mpd_get_uint, mpd_sset_uint);
    for (i = 0; i < 100000; i++) {
        getset_mpd_uint_t(&b, (mpd_uint_t)randbits(), &ctx,
                          mpd_get_uint, mpd_sset_uint);
    }

    /* uint32_t */
    getset_uint32_t(a, 0, &ctx, mpd_get_u32, mpd_set_u32);
    getset_uint32_t(a, UINT32_MAX, &ctx, mpd_get_u32, mpd_set_u32);
    for (i = 0; i < 100000; i++) {
        getset_uint32_t(a, (uint32_t)randbits(), &ctx,
                        mpd_get_u32, mpd_set_u32);
    }

    getset_uint32_t(&b, 0, &ctx, mpd_get_u32, mpd_sset_u32);
    getset_uint32_t(&b, UINT32_MAX, &ctx, mpd_get_u32, mpd_sset_u32);
    for (i = 0; i < 100000; i++) {
        getset_uint32_t(&b, (uint32_t)randbits(), &ctx,
                        mpd_get_u32, mpd_sset_u32);
    }

#ifndef LEGACY_COMPILER
    /* uint64_t */
    getset_uint64_t(a, 0, &ctx, mpd_get_u64, mpd_set_u64);
    getset_uint64_t(a, UINT64_MAX, &ctx, mpd_get_u64, mpd_set_u64);
    for (i = 0; i < 100000; i++) {
        getset_uint64_t(a, (uint64_t)randbits(), &ctx,
                        mpd_get_u64, mpd_set_u64);
    }
#endif

#ifdef CONFIG_64
    getset_uint64_t(&b, 0, &ctx, mpd_get_u64, mpd_sset_u64);
    getset_uint64_t(&b, UINT64_MAX, &ctx, mpd_get_u64, mpd_sset_u64);
    for (i = 0; i < 100000; i++) {
        getset_uint64_t(&b, (uint64_t)randbits(), &ctx,
                        mpd_get_u64, mpd_sset_u64);
    }
#endif

    /* specials */
    status = 0;
    mpd_set_string(a, "nan", &ctx);
    ASSERT(mpd_qget_uint(a, &status) == MPD_UINT_MAX)
    ASSERT(status&MPD_Invalid_operation)

    /* non-integer */
    status = 0;
    mpd_set_string(a, "2345e-1", &ctx);
    ASSERT(mpd_qget_uint(a, &status) == MPD_UINT_MAX)
    ASSERT(status&MPD_Invalid_operation)

    /* too large */
    status = 0;
    mpd_set_uint(a, 8, &ctx);
    a->exp = MPD_RDIGITS;
    ASSERT(mpd_qget_uint(a, &status) == MPD_UINT_MAX)
    ASSERT(status&MPD_Invalid_operation)

    /* too large */
    status = 0;
    mpd_set_uint(a, ((mpd_uint_t)MPD_SSIZE_MAX)+1, &ctx);
    ASSERT(mpd_qget_ssize(a, &status) == MPD_SSIZE_MAX)
    ASSERT(status&MPD_Invalid_operation)

#ifndef LEGACY_COMPILER
    ctx.prec = 1;
    ctx.emax = 1;
    ctx.emin = -1;

    getset_exact_int64_t(&b, INT64_MIN, &ctx, mpd_get_i64, mpd_qset_i64_exact);
    getset_exact_int64_t(&b, INT64_MAX, &ctx, mpd_get_i64, mpd_qset_i64_exact);
    for (i = 0; i < 100000; i++) {
        getset_exact_int64_t(&b, (uint64_t)randbits(), &ctx,
                             mpd_get_i64, mpd_qset_i64_exact);
    }

    getset_exact_uint64_t(&b, 0, &ctx, mpd_get_u64, mpd_qset_u64_exact);
    getset_exact_uint64_t(&b, INT64_MAX, &ctx, mpd_get_u64, mpd_qset_u64_exact);
    for (i = 0; i < 100000; i++) {
        getset_exact_uint64_t(&b, (uint64_t)randbits(), &ctx,
                             mpd_get_u64, mpd_qset_u64_exact);
    }
#endif

    mpd_del(a);
}


/*****************************************************************************/
/*                       Mixed arithmetic functions                          */
/*****************************************************************************/

#define TEST_ARITH_INTTYPE(TYPE, FMTSPEC) \
static void                                                                   \
arith_##TYPE(                                                                 \
    mpd_t *tmp, const mpd_t *a, TYPE x, mpd_context_t *ctx,                   \
    void (* func)(mpd_t *, const mpd_t *, TYPE, mpd_context_t *ctx),          \
    void (* ctrl)(mpd_t *, const mpd_t *, const mpd_t *, mpd_context_t *ctx)) \
{                                                                             \
        mpd_context_t maxctx;                                                 \
        char buf[BUFSIZE];                                                    \
        char *s1, *s2;                                                        \
                                                                              \
        func(tmp, a, x, ctx);                                                 \
        s1 = mpd_to_sci(tmp, 0);                                              \
                                                                              \
        snprintf(buf, BUFSIZE, "%" FMTSPEC, x);                               \
        mpd_maxcontext(&maxctx);                                              \
        mpd_set_string(tmp, buf, &maxctx);                                    \
        ctrl(tmp, a, tmp, ctx);                                               \
        s2 = mpd_to_sci(tmp, 0);                                              \
                                                                              \
        ASSERT(strcmp(s1, s2) == 0)                                           \
        mpd_free(s1);                                                         \
        mpd_free(s2);                                                         \
}

TEST_ARITH_INTTYPE(mpd_ssize_t, PRI_mpd_ssize_t)
TEST_ARITH_INTTYPE(int32_t, PRIi32)
TEST_ARITH_INTTYPE(mpd_uint_t, PRI_mpd_uint_t)
TEST_ARITH_INTTYPE(uint32_t, PRIu32)
#ifndef LEGACY_COMPILER
TEST_ARITH_INTTYPE(int64_t, PRIi64)
TEST_ARITH_INTTYPE(uint64_t, PRIu64)
#endif

static void (*ssize_funcs[])(mpd_t*, const mpd_t*, mpd_ssize_t, mpd_context_t*) =
  {mpd_add_ssize, mpd_sub_ssize, mpd_mul_ssize, mpd_div_ssize, NULL};

static void (*i32_funcs[])(mpd_t*, const mpd_t*, int32_t, mpd_context_t*) =
  {mpd_add_i32, mpd_sub_i32, mpd_mul_i32, mpd_div_i32, NULL};

static void (*uint_funcs[])(mpd_t*, const mpd_t*, mpd_uint_t, mpd_context_t*) =
  {mpd_add_uint, mpd_sub_uint, mpd_mul_uint, mpd_div_uint, NULL};

static void (*u32_funcs[])(mpd_t*, const mpd_t*, uint32_t, mpd_context_t*) =
  {mpd_add_u32, mpd_sub_u32, mpd_mul_u32, mpd_div_u32, NULL};

#ifndef LEGACY_COMPILER
static void (*i64_funcs[])(mpd_t*, const mpd_t*, int64_t, mpd_context_t*) =
  {mpd_add_i64, mpd_sub_i64, mpd_mul_i64, mpd_div_i64, NULL};

static void (*u64_funcs[])(mpd_t*, const mpd_t*, uint64_t, mpd_context_t*) =
  {mpd_add_u64, mpd_sub_u64, mpd_mul_u64, mpd_div_u64, NULL};
#endif

static void (*control_funcs[])(mpd_t*, const mpd_t*, const mpd_t*, mpd_context_t*) =
  {mpd_add, mpd_sub, mpd_mul, mpd_div, NULL};


/* mixed mpd_t and integer operations */
static void
test_mixed_arith(mpd_context_t *ctx)
{
    mpd_t *tmp, *a;
    int i, k;

    tmp = mpd_new(ctx);
    a = mpd_new(ctx);

    ctx->traps &= ~MPD_Division_by_zero;

    for (i = 0; ssize_funcs[i] != NULL; i++) {
        mpd_set_string(a, "123456e2", ctx);
        arith_mpd_ssize_t(tmp, a, MPD_SSIZE_MIN, ctx,
                          ssize_funcs[i], control_funcs[i]);
        arith_mpd_ssize_t(tmp, a, MPD_SSIZE_MAX, ctx,
                          ssize_funcs[i], control_funcs[i]);
        for (k = 0; k < 1000; k++) {
            arith_mpd_ssize_t(tmp, a, (mpd_ssize_t)randbits(), ctx,
                                  ssize_funcs[i], control_funcs[i]);
        }
    }

    for (i = 0; i32_funcs[i] != NULL; i++) {
        mpd_set_string(a, "-123456e2", ctx);
        arith_int32_t(tmp, a, INT32_MIN, ctx,
                      i32_funcs[i], control_funcs[i]);
        arith_int32_t(tmp, a, INT32_MAX, ctx,
                      i32_funcs[i], control_funcs[i]);
        for (k = 0; k < 1000; k++) {
            arith_int32_t(tmp, a, (int32_t)randbits(), ctx,
                              i32_funcs[i], control_funcs[i]);
        }
    }

    for (i = 0; uint_funcs[i] != NULL; i++) {
        mpd_set_string(a, "456789e2", ctx);
        arith_mpd_uint_t(tmp, a, 0, ctx,
                         uint_funcs[i], control_funcs[i]);
        arith_mpd_uint_t(tmp, a, MPD_UINT_MAX, ctx,
                         uint_funcs[i], control_funcs[i]);
        for (k = 0; k < 1000; k++) {
            arith_mpd_uint_t(tmp, a, (mpd_uint_t)randbits(), ctx,
                                 uint_funcs[i], control_funcs[i]);
        }
    }

    for (i = 0; u32_funcs[i] != NULL; i++) {
        mpd_set_string(a, "987654e2", ctx);
        arith_uint32_t(tmp, a, 0, ctx,
                       u32_funcs[i], control_funcs[i]);
        arith_uint32_t(tmp, a, UINT32_MAX, ctx,
                       u32_funcs[i], control_funcs[i]);
        for (k = 0; k < 1000; k++) {
            arith_uint32_t(tmp, a, (uint32_t)randbits(), ctx,
                               u32_funcs[i], control_funcs[i]);
        }
    }

#ifndef LEGACY_COMPILER
    for (i = 0; i64_funcs[i] != NULL; i++) {
        mpd_set_string(a, "-123456e2", ctx);
        arith_int64_t(tmp, a, INT64_MIN, ctx,
                      i64_funcs[i], control_funcs[i]);
        arith_int64_t(tmp, a, INT64_MAX, ctx,
                      i64_funcs[i], control_funcs[i]);
        for (k = 0; k < 1000; k++) {
            arith_int64_t(tmp, a, (int64_t)randbits(), ctx,
                              i64_funcs[i], control_funcs[i]);
        }
    }

    for (i = 0; u64_funcs[i] != NULL; i++) {
        mpd_set_string(a, "-123456e2", ctx);
        arith_uint64_t(tmp, a, 0, ctx,
                       u64_funcs[i], control_funcs[i]);
        arith_uint64_t(tmp, a, UINT64_MAX, ctx,
                       u64_funcs[i], control_funcs[i]);
        for (k = 0; k < 1000; k++) {
            arith_uint64_t(tmp, a, (uint64_t)randbits(), ctx,
                               u64_funcs[i], control_funcs[i]);
        }
    }
#endif

    mpd_del(tmp);
    mpd_del(a);
}

static void
test_context(void)
{
    mpd_context_t ctx;
    mpd_ssize_t ssize;

    /* context.c */
    mpd_traphandler = test_traphandler;
    mpd_init(&ctx, MPD_MAX_PREC+1);
    ASSERT(ctx.status&MPD_Invalid_context)
    ASSERT(ctx.newtrap == MPD_Invalid_context)
    mpd_traphandler = mpd_custom_traphandler;

    mpd_init(&ctx, 100);
    ASSERT(MPD_MINALLOC_MIN <= MPD_MINALLOC &&
           MPD_MINALLOC <= MPD_MINALLOC_MAX)

    fprintf(stderr, "  This warning is part of the coverage test: ");
    ssize = MPD_MINALLOC;
    mpd_setminalloc(2000);
    ASSERT(MPD_MINALLOC == ssize)

    /* DON'T do this */
    MPD_MINALLOC = MPD_MINALLOC_MIN;

    mpd_basiccontext(&ctx);
    ASSERT(mpd_getprec(&ctx) == 9)
    ASSERT(mpd_getemax(&ctx) == MPD_MAX_EMAX)
    ASSERT(mpd_getemin(&ctx) == MPD_MIN_EMIN)
    ASSERT(mpd_getround(&ctx) == MPD_ROUND_HALF_UP)
    ASSERT(mpd_gettraps(&ctx) == (MPD_Traps|MPD_Clamped))
    ASSERT(mpd_getstatus(&ctx) == 0)
    ASSERT(mpd_getclamp(&ctx) == 0)
    ASSERT(mpd_getcr(&ctx) == 1)

    ASSERT(mpd_ieee_context(&ctx, MPD_IEEE_CONTEXT_MAX_BITS*2) < 0)

    mpd_ieee_context(&ctx, 128);
    ASSERT(mpd_getprec(&ctx) == 34)
    ASSERT(mpd_getemax(&ctx) == 6144)
    ASSERT(mpd_getemin(&ctx) == -6143)
    ASSERT(mpd_getround(&ctx) == MPD_ROUND_HALF_EVEN)
    ASSERT(mpd_gettraps(&ctx) == 0)
    ASSERT(mpd_getstatus(&ctx) == 0)
    ASSERT(mpd_getclamp(&ctx) == 1)
    ASSERT(mpd_getcr(&ctx) == 1)

    ASSERT(!mpd_qsetprec(&ctx, -1))
    ASSERT(!mpd_qsetprec(&ctx, 0))
    ASSERT(!mpd_qsetprec(&ctx, MPD_MAX_PREC+1))

    ASSERT(!mpd_qsetemax(&ctx, -1))
    ASSERT(!mpd_qsetemax(&ctx, MPD_MAX_EMAX+1))

    ASSERT(!mpd_qsetemin(&ctx, 1))
    ASSERT(!mpd_qsetemin(&ctx, MPD_MIN_EMIN-1))

    ASSERT(!mpd_qsetround(&ctx, -1))
    ASSERT(!mpd_qsetround(&ctx, MPD_ROUND_GUARD))

    ASSERT(!mpd_qsettraps(&ctx, MPD_Max_status+1))
    ASSERT(!mpd_qsetstatus(&ctx, MPD_Max_status+1))

    ASSERT(!mpd_qsetclamp(&ctx, -1))
    ASSERT(!mpd_qsetclamp(&ctx, 2))

    ASSERT(!mpd_qsetcr(&ctx, -1))
    ASSERT(!mpd_qsetcr(&ctx, 2))

    mpd_qsetprec(&ctx, 2*MPD_RDIGITS);
    ASSERT(ctx.prec == 2*MPD_RDIGITS)

    mpd_qsetemax(&ctx, MPD_MAX_EMAX);
    ASSERT(ctx.emax == MPD_MAX_EMAX)

    mpd_qsetemin(&ctx, MPD_MIN_EMIN);
    ASSERT(ctx.emin == MPD_MIN_EMIN)

    mpd_qsetround(&ctx, MPD_ROUND_HALF_UP);
    ASSERT(ctx.round == MPD_ROUND_HALF_UP)

    mpd_qsettraps(&ctx, MPD_Traps);
    ASSERT(ctx.traps == MPD_Traps)

    mpd_qsetstatus(&ctx, 0);
    ASSERT(ctx.status == 0)

    mpd_qsetclamp(&ctx, 0);
    ASSERT(ctx.clamp == 0)

    mpd_qsetcr(&ctx, 1);
    ASSERT(ctx.allcr == 1)
}

/*
 * NOTE: Using setters like in this function is dangerous or meaningless
 * at best. This is only for testing.
 */
static void
test_attributes(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    char *s;

    mpd_testcontext(&ctx);

    /* specials */
    mpd_set_infinity(&a);
    ASSERT(mpd_isinfinite(&a))
    mpd_set_snan(&a);
    ASSERT(mpd_issnan(&a))

    /* data flags */
    mpd_set_dynamic(&a);
    ASSERT(mpd_isdynamic(&a))
    mpd_set_static(&a);
    ASSERT(mpd_isstatic(&a))
    mpd_set_const_data(&a);
    ASSERT(mpd_isconst_data(&a))

    mpd_set_static_data(&a);
    mpd_clear_flags(&a);

    /* switch to dynamic data */
    mpd_set_string(&a, "-2.3854727e+1875", &ctx);
    ASSERT(mpd_isstatic(&a))
    ASSERT(mpd_isstatic_data(&a))
    ASSERT(mpd_resize(&a, 2*MPD_MINALLOC_MAX, &ctx))
    ASSERT(a.alloc == 2*MPD_MINALLOC_MAX)
    ASSERT(mpd_isstatic(&a))
    ASSERT(mpd_isdynamic_data(&a))
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "-2.3854727e+1875") == 0)
    mpd_free(s);

    /* switch to dynamic data, zero new memory */
    mpd_set_string(&b, "123456789", &ctx);
    ASSERT(mpd_isstatic(&b))
    ASSERT(mpd_isstatic_data(&b))
    ASSERT(mpd_resize_zero(&b, 3*MPD_MINALLOC_MAX, &ctx))
    ASSERT(b.alloc == 3*MPD_MINALLOC_MAX)
    ASSERT(mpd_isstatic(&b))
    ASSERT(mpd_isdynamic_data(&b))
    ASSERT(b.data[0] == 0)


    /* rarely used attributes */
    mpd_set_string(&a, "2.3854727e+1875", &ctx);
    ASSERT(mpd_iscanonical(&a))
    ASSERT(mpd_isfinite(&a))
    ASSERT(!mpd_issigned(&a))
    mpd_set_negative(&a);
    ASSERT(mpd_issigned(&a))

    mpd_set_string(&a, "123000e-3", &ctx);
    ASSERT(mpd_isinteger(&a))
    ASSERT(mpd_isodd(&a))
    ASSERT(!mpd_iseven(&a))

    mpd_set_string(&a, "123000e-2", &ctx);
    ASSERT(mpd_isinteger(&a))
    ASSERT(mpd_iseven(&a))
    ASSERT(!mpd_isodd(&a))

    mpd_del(&a);
    mpd_del(&b);
}

static void
test_shift(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    char *s;

    /* mpd_qsshiftr */
    mpd_testcontext(&ctx);
    ctx.prec = 100;
    ctx.emax = MPD_MAX_EMAX;
    ctx.emin = MPD_MIN_EMIN;

    mpd_set_string(&a, "123456789123456789", &ctx);
    mpd_qsshiftr(&a, &a, 0);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "123456789123456789") == 0)
    mpd_free(s);

    mpd_set_string(&a, "0", &ctx);
    mpd_qsshiftr(&a, &a, 0);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "0") == 0)
    mpd_free(s);

    mpd_set_string(&a, "123", &ctx);
    mpd_qsshiftr(&a, &a, 7);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "0") == 0)
    mpd_free(s);

    mpd_set_string(&a, "123456789123456789", &ctx);
    mpd_qsshiftr(&b, &a, 0);
    s = mpd_to_sci(&b, 0);
    ASSERT(strcmp(s, "123456789123456789") == 0)
    mpd_free(s);

    mpd_set_string(&a, "0", &ctx);
    mpd_qsshiftr(&b, &a, 0);
    s = mpd_to_sci(&b, 0);
    ASSERT(strcmp(s, "0") == 0)
    mpd_free(s);

    mpd_set_string(&a, "123", &ctx);
    mpd_qsshiftr(&b, &a, 7);
    s = mpd_to_sci(&b, 0);
    ASSERT(strcmp(s, "0") == 0)
    mpd_free(s);

    /* mpd_qrotate */
    ctx.traps = 0;
    ctx.status = 0;
    mpd_set_string(&a, "1234556789", &ctx);
    mpd_set_string(&b, "1e100", &ctx);
    mpd_rotate(&a, &a, &b, &ctx);
    ASSERT(mpd_isnan(&a))
    ASSERT(ctx.status&MPD_Invalid_operation)

    mpd_del(&a);
    mpd_del(&b);
}

static void
test_check_nans(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};

    mpd_testcontext(&ctx);

    /* check nans */
    ctx.status = 0;
    mpd_set_string(&a, "NaN", &ctx);
    ASSERT(mpd_check_nan(&b, &a, &ctx))
    ASSERT(ctx.status == 0)
    ASSERT(mpd_isqnan(&b))
    ASSERT(!mpd_isnormal(&b, &ctx))
    ASSERT(!mpd_issubnormal(&b, &ctx))

    ctx.status = 0;
    mpd_set_string(&a, "1", &ctx);
    ASSERT(!mpd_check_nan(&b, &a, &ctx))
    ASSERT(ctx.status == 0)

    mpd_traphandler = test_traphandler;
    ctx.status = 0;
    mpd_set_string(&a, "sNaN", &ctx);
    mpd_set_string(&b, "1", &ctx);
    ASSERT(mpd_check_nans(&b, &a, &b, &ctx))
    ASSERT(ctx.status == MPD_Invalid_operation)
    ASSERT(mpd_isqnan(&b))
    ASSERT(!mpd_isnormal(&b, &ctx))
    ASSERT(!mpd_issubnormal(&b, &ctx))
    mpd_traphandler = mpd_custom_traphandler;

    ctx.status = 0;
    mpd_set_string(&a, "2", &ctx);
    mpd_set_string(&b, "1", &ctx);
    ASSERT(!mpd_check_nans(&b, &a, &b, &ctx))
    ASSERT(ctx.status == 0)

    mpd_del(&a);
    mpd_del(&b);
}

static void
test_finalize_round(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    char *s;

    mpd_testcontext(&ctx);

    /* finalize */
    ctx.prec = 5;
    ctx.status = 0;
    mpd_set_string(&a, "8.89871263726722323e201", &ctx);
    mpd_finalize(&a, &ctx);
    s = mpd_to_sci(&a, 1);
    ASSERT(strcmp(s, "8.8987E+201") == 0)
    ASSERT(ctx.status == (MPD_Rounded|MPD_Inexact))
    mpd_free(s);

    /* apply_round_excess, apply_round_fit */
    ctx.prec =  MPD_RDIGITS + 1;
    ctx.emax = 99;
    ctx.emin = -99;
    ctx.traps &= ~MPD_Underflow;

#ifdef CONFIG_64
    ctx.status = 0;
    mpd_set_string(&a, "99999999999999999999e-119", &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "1.0000000000000000000e-99") == 0)
    ASSERT(ctx.status == (MPD_Underflow|MPD_Subnormal|MPD_Rounded|MPD_Inexact))
    mpd_free(s);

    ctx.status = 0;
    mpd_set_string(&a, "99999999999999999999e79", &ctx);
    mpd_set_string(&b, "1e80", &ctx);
    mpd_quantize(&a, &a, &b, &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "1.0000000000000000000e+99") == 0)
    ASSERT(ctx.status == (MPD_Rounded|MPD_Inexact))
    mpd_free(s);
#else
    ctx.status = 0;
    mpd_set_string(&a, "9999999999e-109", &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "1.000000000e-99") == 0)
    ASSERT(ctx.status == (MPD_Underflow|MPD_Subnormal|MPD_Rounded|MPD_Inexact))
    mpd_free(s);

    ctx.status = 0;
    mpd_set_string(&a, "9999999999e89", &ctx);
    mpd_set_string(&b, "1e90", &ctx);
    mpd_quantize(&a, &a, &b, &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "1.000000000e+99") == 0)
    ASSERT(ctx.status == (MPD_Rounded|MPD_Inexact))
    mpd_free(s);
#endif

    mpd_del(&a);
    mpd_del(&b);
}

static void
test_baseconv(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    uint32_t u32, *u32data=&u32;
    uint16_t u16, *u16data=&u16;

    mpd_testcontext(&ctx);
    mpd_traphandler = test_traphandler;

#if MPD_SIZE_MAX == SIZE_MAX
    /* sizeinbase: too many input digits */
    mpd_set_string(&a, "1", &ctx);
    a.digits = MPD_SSIZE_MAX;

    ctx.status = 0;
    u16data = NULL;
    ASSERT(mpd_export_u16(&u16data, 1, 2, &a, &ctx) == SIZE_MAX)
    ASSERT(ctx.status & MPD_Invalid_operation)
    ASSERT(u16data == NULL)

    ctx.status = 0;
    u32data = NULL;
    ASSERT(mpd_export_u32(&u32data, 1, 2, &a, &ctx) == SIZE_MAX)
    ASSERT(ctx.status & MPD_Invalid_operation)
    ASSERT(u32data == NULL)
#endif

    /* special values */
    u16data = &u16;
    u32data = &u32;

    mpd_set_string(&a, "inf", &ctx);

    ctx.status = 0;
    ASSERT(mpd_export_u16(&u16data, 1, 1<<15, &a, &ctx) == SIZE_MAX)
    ASSERT(ctx.status & MPD_Invalid_operation)

    ctx.status = 0;
    ASSERT(mpd_export_u32(&u32data, 1, 1<<15, &a, &ctx) == SIZE_MAX)
    ASSERT(ctx.status & MPD_Invalid_operation)

    /* zero */
    ctx.status = 0;
    mpd_set_string(&a, "0.0", &ctx);
    ASSERT(mpd_export_u16(&u16data, 1, 1<<15, &a, &ctx) == 1)
    ASSERT(u16 == 0)
    ASSERT(ctx.status == 0)

    ctx.status = 0;
    mpd_set_string(&a, "0.0", &ctx);
    ASSERT(mpd_export_u32(&u32data, 1, 1<<30, &a, &ctx) == 1)
    ASSERT(u32 == 0)
    ASSERT(ctx.status == 0)

    ctx.status = 0;
    mpd_set_string(&a, "0.0e-999", &ctx);
    ASSERT(mpd_export_u16(&u16data, 1, 1<<15, &a, &ctx) == 1)
    ASSERT(u16 == 0)
    ASSERT(ctx.status == 0)

    ctx.status = 0;
    mpd_set_string(&a, "0.0e999", &ctx);
    ASSERT(mpd_export_u32(&u32data, 1, 1<<30, &a, &ctx) == 1)
    ASSERT(u32 == 0)
    ASSERT(ctx.status == 0)

    /* non-integer values */
    mpd_set_string(&a, "9.12987391", &ctx);

    ctx.status = 0;
    ASSERT(mpd_export_u16(&u16data, 1, 1<<15, &a, &ctx) == SIZE_MAX)
    ASSERT(ctx.status & MPD_Invalid_operation)

    ctx.status = 0;
    ASSERT(mpd_export_u32(&u32data, 1, 1<<15, &a, &ctx) == SIZE_MAX)
    ASSERT(ctx.status & MPD_Invalid_operation)


    /* import */
    mpd_set_string(&a, "123456789", &ctx);

    ctx.status = 0;
    mpd_import_u16(&a, u16data, SIZE_MAX-1, MPD_POS, 1<<15, &ctx);
    ASSERT(ctx.status & (MPD_Invalid_operation|MPD_Malloc_error))

    ctx.status = 0;
    mpd_import_u16(&a, u16data, MPD_SIZE_MAX/(sizeof (mpd_uint_t))+1, MPD_POS, 2, &ctx);
    ASSERT(ctx.status & (MPD_Invalid_operation|MPD_Malloc_error))

    ctx.status = 0;
    mpd_import_u32(&a, u32data, SIZE_MAX-1, MPD_POS, 1<<15, &ctx);
    ASSERT(ctx.status & (MPD_Invalid_operation|MPD_Malloc_error))

    mpd_traphandler = mpd_custom_traphandler;
    mpd_del(&a);
}

static void
test_set_string(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    char buf[BUFSIZE];
    char *s, *t;
    mpd_ssize_t mem;
    int n;

    mpd_testcontext(&ctx);
    ctx.traps = MPD_Malloc_error;

    /* Exponent limits */
    ctx.status = 0;
    ctx.prec = 28;
    snprintf(buf, BUFSIZE, "0.0000000000000000000000001e%" PRI_mpd_ssize_t,
             (mpd_ssize_t)MPD_MAX_EMAX+20);
    mpd_set_string(&a, buf, &ctx);
    t = mpd_to_sci(&a, 1);
    snprintf(buf, BUFSIZE, "1E+%" PRI_mpd_ssize_t, (mpd_ssize_t)MPD_MAX_EMAX-5);
    ASSERT(strcmp(t, buf) == 0)
    ASSERT(ctx.status == 0)
    mpd_free(t);

    ctx.status = 0;
    ctx.prec = MPD_MAX_PREC;
    snprintf(buf, BUFSIZE, "0.0000000000000000000000001e%" PRI_mpd_ssize_t,
             (mpd_ssize_t)MPD_MAX_EMAX+30);
    mpd_set_string(&a, buf, &ctx);
    t = mpd_to_sci(&a, 1);
    ASSERT(strcmp(t, "Infinity") == 0)
    ASSERT(ctx.status == (MPD_Inexact|MPD_Overflow|MPD_Rounded))
    mpd_free(t);

    ctx.status = 0;
    ctx.prec = MPD_MAX_PREC;
    snprintf(buf, BUFSIZE, "1000000000000000000000000000e%" PRI_mpd_ssize_t, mpd_etiny(&ctx)-27);
    mpd_set_string(&a, buf, &ctx);
    t = mpd_to_sci(&a, 1);
    snprintf(buf, BUFSIZE, "1E%" PRI_mpd_ssize_t, mpd_etiny(&ctx));
    ASSERT(strcmp(t, buf) == 0)
    ASSERT(ctx.status == (MPD_Subnormal|MPD_Rounded))
    mpd_free(t);

    ctx.status = 0;
    ctx.prec = 28;
    snprintf(buf, BUFSIZE, "-0.0000000000000000000000001e%" PRI_mpd_ssize_t, MPD_SSIZE_MAX);
    mpd_set_string(&a, buf, &ctx);
    t = mpd_to_sci(&a, 1);
    ASSERT(strcmp(t, "-Infinity") == 0)
    ASSERT(ctx.status == (MPD_Overflow|MPD_Rounded|MPD_Inexact))
    mpd_free(t);

    ctx.status = 0;
    ctx.prec = 28;
    snprintf(buf, BUFSIZE, "-0.0000000000000000000000001e%" PRI_mpd_ssize_t, MPD_SSIZE_MIN);
    mpd_set_string(&a, buf, &ctx);
    snprintf(buf, BUFSIZE, "-0E%" PRI_mpd_ssize_t, mpd_etiny(&ctx));
    t = mpd_to_sci(&a, 1);
    ASSERT(strcmp(t, buf) == 0)
    ASSERT(ctx.status == (MPD_Subnormal|MPD_Inexact|
                          MPD_Clamped|MPD_Underflow|
                          MPD_Rounded))
    mpd_free(t);

    ctx.status = 0;
    n = snprintf(buf, BUFSIZE, "1e%s", "184467440737095516161");
    mpd_set_string(&a, buf, &ctx);
    t = mpd_to_sci(&a, 1);
    ASSERT(strcmp(t, "Infinity") == 0)
    ASSERT(ctx.status == (MPD_Inexact|MPD_Overflow|MPD_Rounded))
    mpd_free(t);

    ctx.status = 0;
    n = snprintf(buf, BUFSIZE, "-1e%s", "-184467440737095516161");
    mpd_set_string(&a, buf, &ctx);
    t = mpd_to_sci(&a, 1);
    snprintf(buf, BUFSIZE, "-0E%" PRI_mpd_ssize_t, mpd_etiny(&ctx));
    ASSERT(strcmp(t, buf) == 0)
    ASSERT(ctx.status == (MPD_Subnormal|MPD_Inexact|
                          MPD_Clamped|MPD_Underflow|
                          MPD_Rounded))
    mpd_free(t);


    /* Exponent and precision limits: requires large amounts of memory. */
    mem = MPD_MAX_PREC+10+23+1;
    if ((s = malloc(mem)) == NULL) {
        mpd_del(&a);
        mpd_del(&b);
        return;
    }

    ctx.prec = MPD_MAX_PREC;

    /* .000...0001e2147483647 -> Infinity */
    s[0] = '.';
    memset(s+1, '0', MPD_MAX_PREC-1);
    s[MPD_MAX_PREC] = '1';
    s[MPD_MAX_PREC+1] = 'e';
    s[MPD_MAX_PREC+2] = '\0';

    ctx.status = 0;
    n = snprintf(s+MPD_MAX_PREC+2, 23, "%" PRI_mpd_ssize_t, MPD_SSIZE_MAX);
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        ASSERT(strcmp(t, "Infinity") == 0)
        ASSERT(ctx.status == (MPD_Inexact|MPD_Overflow|MPD_Rounded))
        mpd_free(t);

    }

    /* .000...0001e184467440737095516161 -> Infinity */
    ctx.status = 0;
    s[MPD_MAX_PREC+2] = '\0';
    n = snprintf(s+MPD_MAX_PREC+2, 23, "%s", "184467440737095516161");
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        ASSERT(strcmp(t, "Infinity") == 0)
        ASSERT(ctx.status == (MPD_Inexact|MPD_Overflow|MPD_Rounded))
        mpd_free(t);
    }

    /* .000...0001e425000000 -> 1 */
    ctx.status = 0;
    s[MPD_MAX_PREC+2] = '\0';
    n = snprintf(s+MPD_MAX_PREC+2, 23, "%" PRI_mpd_ssize_t, (mpd_ssize_t)MPD_MAX_EMAX);
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        ASSERT(strcmp(t, "1") == 0)
        ASSERT(ctx.status == 0)
        mpd_free(t);
    }

    /* .000...0001e850000000 -> 1 */
    ctx.status = 0;
    s[MPD_MAX_PREC+2] = '\0';
    n = snprintf(s+MPD_MAX_PREC+2, 23, "%" PRI_mpd_ssize_t, 2*(mpd_ssize_t)MPD_MAX_EMAX);
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        snprintf(buf, BUFSIZE, "1E+%" PRI_mpd_ssize_t, (mpd_ssize_t)MPD_MAX_EMAX);
        ASSERT(strcmp(t, buf) == 0)
        ASSERT(ctx.status == 0)
        mpd_free(t);
    }

    /* .000...0005e424999999 -> 0.5 */
    ctx.status = 0;
    s[MPD_MAX_PREC] = '5';
    s[MPD_MAX_PREC+1] = 'e';
    s[MPD_MAX_PREC+2] = '\0';
    n = snprintf(s+MPD_MAX_PREC+2, 23, "%" PRI_mpd_ssize_t, (mpd_ssize_t)MPD_MAX_EMAX-1);
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        ASSERT(strcmp(t, "0.5") == 0)
        ASSERT(ctx.status == 0)
        mpd_free(t);
    }

    /* .000...0001e-2147483648 -> 0E-849999999 */
    ctx.status = 0;
    s[MPD_MAX_PREC] = '1';
    s[MPD_MAX_PREC+1] = 'e';
    s[MPD_MAX_PREC+2] = '\0';
    n = snprintf(s+MPD_MAX_PREC+2, 23, "%" PRI_mpd_ssize_t, MPD_SSIZE_MIN);
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        snprintf(buf, BUFSIZE, "0E%" PRI_mpd_ssize_t, mpd_etiny(&ctx));
        ASSERT(strcmp(t, buf) == 0)
        ASSERT(ctx.status == (MPD_Subnormal|MPD_Inexact|
                              MPD_Clamped|MPD_Underflow|
                              MPD_Rounded))
        mpd_free(t);
    }

    /* .000...0001e-184467440737095516161 -> 0E-849999999 */
    ctx.status = 0;
    s[MPD_MAX_PREC] = '1';
    s[MPD_MAX_PREC+1] = 'e';
    s[MPD_MAX_PREC+2] = '\0';
    n = snprintf(s+MPD_MAX_PREC+2, 23, "%s", "-184467440737095516161");
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        snprintf(buf, BUFSIZE, "0E%" PRI_mpd_ssize_t, mpd_etiny(&ctx));
        ASSERT(strcmp(t, buf) == 0)
        ASSERT(ctx.status == (MPD_Subnormal|MPD_Inexact|
                              MPD_Clamped|MPD_Underflow|
                              MPD_Rounded))
        mpd_free(t);

    }

    /* 10000...0000e-425000000 -> 0.1 */
    ctx.status = 0;
    s[0] = '1';
    memset(s+1, '0', MPD_MAX_PREC-1);
    s[MPD_MAX_PREC] = 'e';
    s[MPD_MAX_PREC+1] = '\0';
    n = snprintf(s+MPD_MAX_PREC+1, 23, "%" PRI_mpd_ssize_t, (mpd_ssize_t)MPD_MIN_EMIN);
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        mpd_set_string(&b, "0.1", &ctx);
        ASSERT(mpd_cmp(&a, &b, &ctx) == 0)
        ASSERT(ctx.status == 0)
    }

    /* 10000...0000e-1275000000 -> 1 */
    ctx.status = 0;
    s[0] = '1';
    memset(s+1, '0', MPD_MAX_PREC-1);
    s[MPD_MAX_PREC] = 'e';
    s[MPD_MAX_PREC+1] = '\0';
    n = snprintf(s+MPD_MAX_PREC+1, 23, "%" PRI_mpd_ssize_t, 3*(mpd_ssize_t)MPD_MIN_EMIN+2);
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        snprintf(buf, BUFSIZE, "1E%" PRI_mpd_ssize_t, mpd_etiny(&ctx));
        ASSERT(strcmp(t, buf) == 0)
        ASSERT(ctx.status == (MPD_Subnormal|MPD_Rounded))
        mpd_free(t);
    }

    /* Too many fraction digits: .000...0001e184467440737095516161 -> NaN */
    s[0] = '.';
    memset(s+1, '0', MPD_MAX_PREC);
    s[MPD_MAX_PREC+1] = '1';
    s[MPD_MAX_PREC+2] = 'e';

    ctx.status = 0;
    s[MPD_MAX_PREC+3] = '\0';
    n = snprintf(s+MPD_MAX_PREC+3, 23, "%s", "184467440737095516161");
    mpd_set_string(&a, s, &ctx);
    if (n > 0 && !(ctx.status&MPD_Malloc_error)) {
        t = mpd_to_sci(&a, 1);
        ASSERT(strcmp(t, "NaN") == 0)
        ASSERT(ctx.status == MPD_Conversion_syntax)
        mpd_free(t);
    }

    /* Exactly MPD_MAX_PREC digits */
    ctx.status = 0;
    ctx.prec = MPD_MAX_PREC;
    memset(s, '9', MPD_MAX_PREC);
    s[MPD_MAX_PREC] = '\0';
    mpd_set_string(&a, s, &ctx);
    if (!(ctx.status&MPD_Malloc_error)) {
        ASSERT(ctx.status == 0)
        t = mpd_to_sci(&a, 0);
        if (t != NULL) {
            ASSERT(strcmp(t, s) == 0)
            mpd_free(t);
        }
    }

    /* Too many digits: 99999...9999 -> NaN */
    ctx.status = 0;
    memset(s, '9', MPD_MAX_PREC+1);
    s[MPD_MAX_PREC+1] = '\0';
    mpd_set_string(&a, s, &ctx);
    if (!(ctx.status&MPD_Malloc_error)) {
        ASSERT(ctx.status == MPD_Conversion_syntax)
    }

    /* Exact conversion */
    ctx.status = 0;
    ctx.prec = 1;
    snprintf(buf, BUFSIZE, "12345");
    mpd_qset_string_exact(&a, buf, &ctx.status);
    t = mpd_to_sci(&a, 1);
    ASSERT(strcmp(t, buf) == 0)
    ASSERT(ctx.status == 0)
    mpd_free(t);

    /* Exact conversion not possible */
    ctx.status = 0;
    ctx.prec = 28;
    n = snprintf(buf, BUFSIZE, "1e%s", "184467440737095516161");
    mpd_qset_string_exact(&a, buf, &ctx.status);
    t = mpd_to_sci(&a, 1);
    ASSERT(strcmp(t, "NaN") == 0)
    ASSERT(ctx.status == MPD_Invalid_operation)
    mpd_free(t);

    mpd_del(&a);
    mpd_del(&b);
    free(s);
}

static void
test_memory(void)
{
    mpd_context_t ctx;
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    mpd_t *a;
    char *s, *ptr;
    uint8_t err;

    mpd_testcontext(&ctx);
    ctx.traps = 0;

    /* Allocation failures */
    ctx.status = 0;
    a = mpd_new(&ctx);
    mpd_set_alloc_fail(&ctx);
    mpd_resize(a, 20, &ctx);
    mpd_set_alloc(&ctx);
    ASSERT(ctx.status & MPD_Malloc_error)
    ASSERT(mpd_isnan(a))
    mpd_del(a);

    ctx.status = 0;
    mpd_set_alloc_fail(&ctx);
    mpd_resize_zero(&b, MPD_MINALLOC_MAX+1, &ctx);
    mpd_set_alloc(&ctx);
    ASSERT(ctx.status & MPD_Malloc_error)
    ASSERT(mpd_isnan(&b))

    for (alloc_fail = 1; alloc_fail <= 10; alloc_fail++) {
        ctx.status = 0;

        mpd_set_alloc_fail(&ctx);
        a = mpd_new(&ctx);
        mpd_set_alloc(&ctx);

        if (!(ctx.status & MPD_Malloc_error)) {
            mpd_del(a);
            break;
        }
        ASSERT(a == NULL)
    }
    ASSERT(alloc_fail == 3)

#if MPD_SIZE_MAX < SIZE_MAX
    s = mpd_callocfunc_em((size_t)MPD_SIZE_MAX+1, 16);
    ASSERT(s == NULL)
    s = mpd_callocfunc_em(16, (size_t)MPD_SIZE_MAX+1);
    ASSERT(s == NULL)
#endif

    /* test overflow */
    s = mpd_callocfunc_em((size_t)MPD_SIZE_MAX, (size_t)MPD_SIZE_MAX);
    ASSERT(s == NULL)
    s = mpd_alloc((size_t)MPD_SIZE_MAX, (size_t)MPD_SIZE_MAX);
    ASSERT(s == NULL)
    s = mpd_calloc((size_t)MPD_SIZE_MAX, (size_t)MPD_SIZE_MAX);
    ASSERT(s == NULL)

    ptr = malloc(1);
    if (ptr != NULL) {
        err = 0;
        s = mpd_realloc(ptr, (size_t)MPD_SIZE_MAX,
                        (size_t)MPD_SIZE_MAX, &err);
        ASSERT(s == ptr)
        ASSERT(err == 1)
        mpd_free(ptr);
    }

    s = mpd_sh_alloc(1, (size_t)MPD_SIZE_MAX, (size_t)MPD_SIZE_MAX);
    ASSERT(s == NULL)
    s = mpd_sh_alloc((size_t)MPD_SIZE_MAX, 8, 16);
    ASSERT(s == NULL)

    mpd_del(&b);
}


typedef struct {
    mpd_t dec;
    mpd_uint_t data[1];
} DecObject;

#define MPD(v) (&(v)->dec)

static DecObject *
dec_new(mpd_ssize_t nwords)
{
    DecObject *dec;

    dec = mpd_sh_alloc(sizeof(DecObject), nwords, sizeof *dec->data);
    if (dec == NULL) {
        return NULL;
    }

    MPD(dec)->flags = MPD_STATIC|MPD_STATIC_DATA;
    MPD(dec)->exp = 0;
    MPD(dec)->digits = 0;
    MPD(dec)->len = 0;
    MPD(dec)->alloc = nwords;
    MPD(dec)->data = dec->data;

    return dec;
}

static void
dec_free(DecObject *v)
{
    mpd_del(MPD(v));
    mpd_free(v);
}


#define DECALLOC_MAX (MPD_MINALLOC_MAX+10)
char nines[DECALLOC_MAX*MPD_RDIGITS+1];

static void
assert_equal(const mpd_t *a, const mpd_t *b, mpd_ssize_t alloc,
             mpd_ssize_t len, mpd_ssize_t unchanged)
{
    mpd_ssize_t i;

    ASSERT(a->flags == b->flags)
    ASSERT(a->exp == b->exp)
    ASSERT(a->digits == b->digits)
    ASSERT(a->len == len)
    ASSERT(a->alloc == alloc)
    if (!mpd_isnan(b)) {
        for (i = 0; i < unchanged; i++) {
            ASSERT(a->data[i] == b->data[i])
        }
    }
}

static void
test_resize(void)
{
    DecObject *dcalc, *dexpected;
    mpd_context_t ctx;
    mpd_t *calc, *expected;
    mpd_ssize_t nwords;
    mpd_ssize_t decalloc, minalloc;
    mpd_ssize_t i;

    mpd_maxcontext(&ctx);
    ctx.traps = 0;

    /* Allocation failures */
    for (minalloc=MPD_MINALLOC_MIN; minalloc <= MPD_MINALLOC_MAX; minalloc++) {
        for (decalloc=minalloc; decalloc <= DECALLOC_MAX; decalloc++) {

            /* Case nwords <= MINALLOC */
            for (nwords = -10; nwords <= MPD_MINALLOC; nwords++) {

                for (i = 0; i < decalloc*MPD_RDIGITS ; i++) {
                      nines[i] = '9';
                }
                nines[i] = '\0';

                /***** Case alloc failure *****/

                /** Dynamic mpd_t **/
                calc = mpd_new(&ctx);
                mpd_set_string(calc, nines, &ctx);
                expected = mpd_new(&ctx);
                mpd_set_string(expected, nines, &ctx);

                ctx.status = 0;
                mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                ASSERT(mpd_resize(calc, nwords, &ctx))
                mpd_set_alloc(&ctx);
                ASSERT(ctx.status == 0)

                /* Resize failure, whole area is still valid */
                assert_equal(calc, expected,
                             expected->alloc,  /* alloc */
                             expected->len,    /* len */
                             expected->alloc); /* unchanged */
                mpd_del(calc);
                mpd_del(expected);


                /** Static mpd_t **/
                dcalc = dec_new(decalloc);
                mpd_set_string(MPD(dcalc), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dcalc)))
                ASSERT(mpd_isstatic_data(MPD(dcalc)))

                dexpected = dec_new(decalloc);
                mpd_set_string(MPD(dexpected), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dexpected)))
                ASSERT(mpd_isstatic_data(MPD(dexpected)))

                ctx.status = 0;
                mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                ASSERT(mpd_resize(MPD(dcalc), nwords, &ctx))
                mpd_set_alloc(&ctx);
                ASSERT(ctx.status == 0)

                /* Resize no-op, whole area is still valid */
                assert_equal(MPD(dcalc), MPD(dexpected),
                             MPD(dexpected)->alloc,  /* alloc */
                             MPD(dexpected)->len,    /* len */
                             MPD(dexpected)->alloc); /* unchanged data words */
                dec_free(dcalc);
                dec_free(dexpected);


                /***** Case alloc_success *****/

                /** Dynamic mpd_t **/
                calc = mpd_new(&ctx);
                mpd_set_string(calc, nines, &ctx);
                expected = mpd_new(&ctx);
                mpd_set_string(expected, nines, &ctx);

                ctx.status = 0;
                ASSERT(mpd_resize(calc, nwords, &ctx))
                ASSERT(ctx.status == 0)

                /* Resize successful, alloc reduced */
                assert_equal(calc, expected,
                             MPD_MINALLOC,   /* alloc */
                             expected->len,  /* len: unchanged */
                             MPD_MINALLOC);  /* unchanged data words */
                mpd_del(calc);
                mpd_del(expected);


                /** Static mpd_t **/
                dcalc = dec_new(decalloc);
                mpd_set_string(MPD(dcalc), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dcalc)))
                ASSERT(mpd_isstatic_data(MPD(dcalc)))

                dexpected = dec_new(decalloc);
                mpd_set_string(MPD(dexpected), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dexpected)))
                ASSERT(mpd_isstatic_data(MPD(dexpected)))

                ctx.status = 0;
                ASSERT(mpd_resize(MPD(dcalc), nwords, &ctx))
                ASSERT(ctx.status == 0)

                /* Resize no-op, alloc the same */
                assert_equal(MPD(dcalc), MPD(dexpected),
                             MPD(dexpected)->alloc,  /* alloc */
                             MPD(dexpected)->len,    /* len: unchanged */
                             MPD(dexpected)->alloc); /* unchanged data words */
                dec_free(dcalc);
                dec_free(dexpected);
            }

            /* Case nwords > MINALLOC */
            for (nwords = MPD_MINALLOC+1; nwords <= MPD_MINALLOC_MAX+20; nwords++) {

                for (i = 0; i < decalloc*MPD_RDIGITS ; i++) {
                      nines[i] = '9';
                }
                nines[i] = '\0';

                /* nwords <= alloc */
                if (nwords <= decalloc) {

                    /***** Case alloc failure *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, nines, &ctx);

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(mpd_resize(calc, nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status == 0)

                    /* Resize failure, whole area is still valid */
                    assert_equal(calc, expected,
                                 expected->alloc,  /* alloc */
                                 expected->len,    /* len: unchanged */
                                 expected->alloc); /* unchanged data words */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(mpd_resize(MPD(dcalc), nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status == 0)

                    /* Resize no-op, whole area is still valid */
                    assert_equal(MPD(dcalc), MPD(dexpected),
                                 MPD(dexpected)->alloc,  /* alloc */
                                 MPD(dexpected)->len,    /* len: unchanged */
                                 MPD(dexpected)->alloc); /* unchanged data words */
                    dec_free(dcalc);
                    dec_free(dexpected);


                    /***** Case alloc_success *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, nines, &ctx);

                    ctx.status = 0;
                    ASSERT(mpd_resize(calc, nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize successful, alloc decreased */
                    assert_equal(calc, expected,
                                 nwords,        /* alloc */
                                 expected->len, /* len: unchanged */
                                 nwords);       /* unchanged data words */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    ASSERT(mpd_resize(MPD(dcalc), nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize no-op */
                    assert_equal(MPD(dcalc), MPD(dexpected),
                                 MPD(dexpected)->alloc,  /* alloc */
                                 MPD(dexpected)->len,    /* len: unchanged */
                                 MPD(dexpected)->alloc); /* unchanged data words */
                    dec_free(dcalc);
                    dec_free(dexpected);
                }
                else { /* nwords > alloc */

                    /***** Case alloc failure *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, "NaN", &ctx);

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(!mpd_resize(calc, nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status & MPD_Malloc_error)

                    /* Resize failure: NaN */
                    assert_equal(calc, expected,
                                 decalloc,  /* alloc: unchanged */
                                 0,         /* len: 0 (NaN) */
                                 0);        /* data is undefined */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), "NaN", &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(!mpd_resize(MPD(dcalc), nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status & MPD_Malloc_error)

                    /* Resize failure: NaN */
                    assert_equal(MPD(dcalc), MPD(dexpected),
                                 decalloc,  /* alloc: unchanged */
                                 0,         /* len: 0 (NaN) */
                                 0);        /* data is undefined */
                    dec_free(dcalc);
                    dec_free(dexpected);


                    /***** Case alloc_success *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, nines, &ctx);

                    ctx.status = 0;
                    ASSERT(mpd_resize(calc, nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize successful */
                    assert_equal(calc, expected,
                                 nwords,           /* new alloc */
                                 expected->len,    /* len: unchanged */
                                 expected->alloc); /* unchanged: old area */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    ASSERT(mpd_resize(MPD(dcalc), nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize successful */
                    mpd_set_dynamic_data(MPD(dexpected));
                    assert_equal(MPD(dcalc), MPD(dexpected),
                                 nwords,                 /* new alloc */
                                 MPD(dexpected)->len,    /* len: unchanged */
                                 MPD(dexpected)->alloc); /* unchanged: old area */
                    mpd_set_static_data(MPD(dexpected));
                    dec_free(dcalc);
                    dec_free(dexpected);
                }
            }
        }
    }

}

static void
assert_equal_zero(const mpd_t *a, const mpd_t *b, mpd_ssize_t alloc,
                  mpd_ssize_t len, mpd_ssize_t nzero)
{
    mpd_ssize_t i;

    ASSERT(a->flags == b->flags)
    ASSERT(a->exp == b->exp)
    ASSERT(a->digits == b->digits)
    ASSERT(a->len == len)
    ASSERT(a->alloc == alloc)
    if (!mpd_isnan(b)) {
        for (i = 0; i < nzero; i++) {
            ASSERT(a->data[i] == 0)
        }
    }
}

static void
test_resize_zero(void)
{
    DecObject *dcalc, *dexpected;
    mpd_context_t ctx;
    mpd_t *calc, *expected;
    mpd_ssize_t nwords, nzero;
    mpd_ssize_t decalloc, minalloc;
    mpd_ssize_t i;

    mpd_maxcontext(&ctx);
    ctx.traps = 0;

    /* Allocation failures */
    for (minalloc=MPD_MINALLOC_MIN; minalloc <= MPD_MINALLOC_MAX; minalloc++) {
        for (decalloc=minalloc; decalloc <= DECALLOC_MAX; decalloc++) {

            /* Case nwords <= MINALLOC */
            for (nwords = -10; nwords <= MPD_MINALLOC; nwords++) {

                for (i = 0; i < decalloc*MPD_RDIGITS ; i++) {
                      nines[i] = '9';
                }
                nines[i] = '\0';

                /* Expected number of zero words in the resized coefficient */
                nzero = nwords <= MPD_MINALLOC ? MPD_MINALLOC : nwords;

                /***** Case alloc failure *****/

                /** Dynamic mpd_t **/
                calc = mpd_new(&ctx);
                mpd_set_string(calc, nines, &ctx);
                expected = mpd_new(&ctx);
                mpd_set_string(expected, nines, &ctx);

                ctx.status = 0;
                mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                ASSERT(mpd_resize_zero(calc, nwords, &ctx))
                mpd_set_alloc(&ctx);
                ASSERT(ctx.status == 0)

                /* Resize failure, whole area is still valid */
                assert_equal_zero(calc, expected,
                                  expected->alloc,  /* alloc */
                                  expected->len,    /* len */
                                  nzero);           /* zero words */
                mpd_del(calc);
                mpd_del(expected);


                /** Static mpd_t **/
                dcalc = dec_new(decalloc);
                mpd_set_string(MPD(dcalc), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dcalc)))
                ASSERT(mpd_isstatic_data(MPD(dcalc)))

                dexpected = dec_new(decalloc);
                mpd_set_string(MPD(dexpected), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dexpected)))
                ASSERT(mpd_isstatic_data(MPD(dexpected)))

                ctx.status = 0;
                mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                ASSERT(mpd_resize_zero(MPD(dcalc), nwords, &ctx))
                mpd_set_alloc(&ctx);
                ASSERT(ctx.status == 0)

                /* Resize no-op, whole area is still valid */
                assert_equal_zero(MPD(dcalc), MPD(dexpected), 
                                  MPD(dexpected)->alloc,  /* alloc */
                                  MPD(dexpected)->len,    /* len */
                                  nzero);                 /* zero words */
                dec_free(dcalc);
                dec_free(dexpected);


                /***** Case alloc_success *****/

                /** Dynamic mpd_t **/
                calc = mpd_new(&ctx);
                mpd_set_string(calc, nines, &ctx);
                expected = mpd_new(&ctx);
                mpd_set_string(expected, nines, &ctx);

                ctx.status = 0;
                ASSERT(mpd_resize_zero(calc, nwords, &ctx))
                ASSERT(ctx.status == 0)

                /* Resize successful, alloc reduced */
                assert_equal_zero(calc, expected,
                                  MPD_MINALLOC,   /* alloc */
                                  expected->len,  /* len: unchanged */
                                  nzero);         /* zero words */
                mpd_del(calc);
                mpd_del(expected);


                /** Static mpd_t **/
                dcalc = dec_new(decalloc);
                mpd_set_string(MPD(dcalc), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dcalc)))
                ASSERT(mpd_isstatic_data(MPD(dcalc)))

                dexpected = dec_new(decalloc);
                mpd_set_string(MPD(dexpected), nines, &ctx);
                ASSERT(mpd_isstatic(MPD(dexpected)))
                ASSERT(mpd_isstatic_data(MPD(dexpected)))

                ctx.status = 0;
                ASSERT(mpd_resize_zero(MPD(dcalc), nwords, &ctx))
                ASSERT(ctx.status == 0)

                /* Resize no-op, alloc the same */
                assert_equal_zero(MPD(dcalc), MPD(dexpected),
                                  MPD(dexpected)->alloc,  /* alloc */
                                  MPD(dexpected)->len,    /* len: unchanged */
                                  nzero);                 /* zero words */
                dec_free(dcalc);
                dec_free(dexpected);
            }

            /* Case nwords > MINALLOC */
            for (nwords = MPD_MINALLOC+1; nwords <= MPD_MINALLOC_MAX+20; nwords++) {

                for (i = 0; i < decalloc*MPD_RDIGITS ; i++) {
                      nines[i] = '9';
                }
                nines[i] = '\0';

                /* Expected number of zero words in the resized coefficient */
                nzero = nwords <= MPD_MINALLOC ? MPD_MINALLOC : nwords;

                /* nwords <= alloc */
                if (nwords <= decalloc) {

                    /***** Case alloc failure *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, nines, &ctx);

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(mpd_resize_zero(calc, nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status == 0)

                    /* Resize failure, whole area is still valid */
                    assert_equal_zero(calc, expected,
                                      expected->alloc,  /* alloc */
                                      expected->len,    /* len: unchanged */
                                      nzero);           /* zero words */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(mpd_resize_zero(MPD(dcalc), nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status == 0)

                    /* Resize no-op, whole area is still valid */
                    assert_equal_zero(MPD(dcalc), MPD(dexpected),
                                      MPD(dexpected)->alloc,  /* alloc */
                                      MPD(dexpected)->len,    /* len: unchanged */
                                      nzero);                 /* zero words */
                    dec_free(dcalc);
                    dec_free(dexpected);


                    /***** Case alloc_success *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, nines, &ctx);

                    ctx.status = 0;
                    ASSERT(mpd_resize_zero(calc, nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize successful, alloc decreased */
                    assert_equal_zero(calc, expected,
                                      nwords,        /* alloc */
                                      expected->len, /* len: unchanged */
                                      nzero);        /* nzero */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    ASSERT(mpd_resize_zero(MPD(dcalc), nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize no-op */
                    assert_equal_zero(MPD(dcalc), MPD(dexpected),
                                      MPD(dexpected)->alloc,  /* alloc */
                                      MPD(dexpected)->len,    /* len: unchanged */
                                      nzero);                 /* zero words */
                    dec_free(dcalc);
                    dec_free(dexpected);
                }
                else { /* nwords > alloc */

                    /***** Case alloc failure *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, "NaN", &ctx);

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(!mpd_resize_zero(calc, nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status & MPD_Malloc_error)

                    /* Resize failure: NaN */
                    assert_equal_zero(calc, expected,
                                      decalloc,  /* alloc: unchanged */
                                      0,         /* len: 0 (NaN) */
                                      0);        /* data is undefined */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), "NaN", &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    mpd_set_alloc_fail(&ctx); alloc_fail = 0;
                    ASSERT(!mpd_resize_zero(MPD(dcalc), nwords, &ctx))
                    mpd_set_alloc(&ctx);
                    ASSERT(ctx.status & MPD_Malloc_error)

                    /* Resize failure: NaN */
                    assert_equal_zero(MPD(dcalc), MPD(dexpected),
                                      decalloc,  /* alloc: unchanged */
                                      0,         /* len: 0 (NaN) */
                                      0);        /* data is undefined */
                    dec_free(dcalc);
                    dec_free(dexpected);


                    /***** Case alloc_success *****/

                    /** Dynamic mpd_t **/
                    calc = mpd_new(&ctx);
                    mpd_set_string(calc, nines, &ctx);
                    expected = mpd_new(&ctx);
                    mpd_set_string(expected, nines, &ctx);

                    ctx.status = 0;
                    ASSERT(mpd_resize_zero(calc, nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize successful */
                    assert_equal_zero(calc, expected,
                                      nwords,           /* new alloc */
                                      expected->len,    /* len: unchanged */
                                      nzero);           /* zero words */
                    mpd_del(calc);
                    mpd_del(expected);


                    /** Static mpd_t **/
                    dcalc = dec_new(decalloc);
                    mpd_set_string(MPD(dcalc), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dcalc)))
                    ASSERT(mpd_isstatic_data(MPD(dcalc)))

                    dexpected = dec_new(decalloc);
                    mpd_set_string(MPD(dexpected), nines, &ctx);
                    ASSERT(mpd_isstatic(MPD(dexpected)))
                    ASSERT(mpd_isstatic_data(MPD(dexpected)))

                    ctx.status = 0;
                    ASSERT(mpd_resize_zero(MPD(dcalc), nwords, &ctx))
                    ASSERT(ctx.status == 0)

                    /* Resize successful */
                    mpd_set_dynamic_data(MPD(dexpected));
                    assert_equal_zero(MPD(dcalc), MPD(dexpected),
                                      nwords,                 /* new alloc */
                                      MPD(dexpected)->len,    /* len: unchanged */
                                      nzero);                 /* zero words */
                    mpd_set_static_data(MPD(dexpected));
                    dec_free(dcalc);
                    dec_free(dexpected);
                }
            }
        }
    }

}

static void
test_output(void)
{
    uint32_t status;
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_spec_t spec;
    FILE *fp;
    char buf[BUFSIZE];
    const char *decstring, *fmt;
    char *s;
    int n;
#ifdef CONFIG_32
    mpd_ssize_t i, j;
    char *t;
#endif

    mpd_testcontext(&ctx);
    ctx.traps = 0;

    /* printing of flags and signals */
    status = MPD_Max_status;

    s = "Clamped Conversion_syntax Division_by_zero Division_impossible "
        "Division_undefined Fpu_error Inexact Invalid_context Invalid_operation "
        "Malloc_error Not_implemented Overflow Rounded Subnormal Underflow";

    n = mpd_snprint_flags(buf, BUFSIZE, status);
    ASSERT(n > 0 && (size_t)n == strlen(s))
    ASSERT(strcmp(buf, s) == 0)

    s = "[Clamped, Conversion_syntax, Division_by_zero, Division_impossible, "
        "Division_undefined, Fpu_error, Inexact, Invalid_context, Invalid_operation, "
        "Malloc_error, Not_implemented, Overflow, Rounded, Subnormal, Underflow]";

    n = mpd_lsnprint_flags(buf, BUFSIZE, status, NULL);
    ASSERT(n > 0 && (size_t)n == strlen(s))
    ASSERT(strcmp(buf, s) == 0)

    s = "[Clamped, IEEE_Invalid_operation, Division_by_zero, Inexact, "
        "Not_implemented, Overflow, Rounded, Subnormal, Underflow]";

    n = mpd_lsnprint_signals(buf, BUFSIZE, status, NULL);
    ASSERT(n > 0 && (size_t)n == strlen(s))
    ASSERT(strcmp(buf, s) == 0)

    /* simple printing to file */
    if ((fp = tmpfile()) != NULL) {
        mpd_set_string(&a, "1.23456789e441", &ctx);
        mpd_fprint(fp, &a);
        rewind(fp);
        if (fgets(buf, BUFSIZE, fp) != NULL) {
            ASSERT(strcmp(buf, "1.23456789e441\n"))
        }
        fclose(fp);
    }

    /* simple printing to stdout */
    mpd_set_string(&a, "1.23456789e441", &ctx);
    printf("  This line is part of the coverage test: ");
    mpd_print(&a);
    fflush(stdout);

    /* mpd_qformat_spec */

    /* MPD_Malloc_error */
    mpd_maxcontext(&ctx);
    ctx.traps = 0;
#ifdef CONFIG_64
    decstring = "1e999999999999999999";
    fmt = ".1000000000000000000f";
#else
    decstring = "1e425000000";
    fmt = ".425000001f";
#endif

    ctx.status = 0;
    mpd_set_string(&a, decstring, &ctx);
    s = mpd_format(&a, "f", &ctx);
    if (s == NULL) {
        ASSERT(ctx.status&MPD_Malloc_error)
    }
    else {
        free(s);
    }

    ctx.status = 0;
    s = mpd_format(&a, fmt, &ctx);
    if (s == NULL) {
        ASSERT(ctx.status&MPD_Invalid_operation)
    }
    else {
        free(s);
    }

    /* General edge cases */
    mpd_set_string(&a, "9e+1", &ctx);

    /* Invalid precision */
    status = 0;
    snprintf(buf, BUFSIZE, ".%" PRI_mpd_ssize_t "g", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

    status = 0;
    snprintf(buf, BUFSIZE, ".%" PRI_mpd_ssize_t "e", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

    status = 0;
    snprintf(buf, BUFSIZE, ".%" PRI_mpd_ssize_t "f", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

    status = 0;
    snprintf(buf, BUFSIZE, ".%" PRI_mpd_ssize_t "%%", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

    /* Invalid min_width */
    status = 0;
    snprintf(buf, BUFSIZE, "%" PRI_mpd_ssize_t "g", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

    status = 0;
    snprintf(buf, BUFSIZE, "%" PRI_mpd_ssize_t "e", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

    status = 0;
    snprintf(buf, BUFSIZE, "%" PRI_mpd_ssize_t "f", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

    status = 0;
    snprintf(buf, BUFSIZE, "%" PRI_mpd_ssize_t "%%", (mpd_ssize_t)MPD_MAX_PREC+1);
    s = mpd_qformat(&a, buf, &ctx, &status);
    ASSERT(s == NULL && (status&MPD_Invalid_operation))

#ifdef CONFIG_32
    /* type: 'g' */
    s = malloc(425000000 + 4);
    if (s != NULL) {
        /* 999...999e+0 */
        memset(s, '9', 425000000);
        s[425000000] = 'e';
        s[425000001] = '+';
        s[425000002] = '0';
        s[425000003] = '\0';
        mpd_set_string(&a, s, &ctx);

        status = 0;
        t = mpd_qformat(&a, ".425000000g", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            /* 999...999 */
            ASSERT(status == 0 && strncmp(t, s, 425000000) == 0)
            mpd_free(t);
        }

        status = 0;
        t = mpd_qformat(&a, ".1g", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            /* 1e425000000 */
            ASSERT(status == 0 && strcmp(t, "1e+425000000") == 0)
            mpd_free(t);
        }


        /* 999...999e+1 */
        s[425000002] = '1';
        mpd_set_string(&a, s, &ctx);

        status = 0;
        t = mpd_qformat(&a, ".425000000g", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 9.999...999e+42500000 */
            ASSERT(strncmp(t, "9.", 2) == 0)
            ASSERT(strncmp(t+2, s, 424999999) == 0)
            ASSERT(strcmp(t+425000001, "e+425000000") == 0)
            mpd_free(t);
        }
        free(s);

        status = 0;
        t = mpd_qformat(&a, ".1g", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            /* 1e+42500001 */
            ASSERT(status == 0 && strcmp(t, "1e+425000001") == 0)
            mpd_free(t);
        }
    }

    /* type: 'g': min_width */
    mpd_set_string(&a, "9e+0", &ctx);

    status = 0;
    t = mpd_qformat(&a, "\xf3\x80\x80\x80<425000000.425000000g",
                    &ctx, &status);
    if (t == NULL) {
        ASSERT(status&MPD_Malloc_error)
    }
    else {
        ASSERT(status == 0)
        ASSERT(t[0] == '9')
        for (i = 0; i < 424999999; i++) {
            for (j = 0; j < 4; j++) {
                ASSERT(t[1 + i*4+j] == "\xf3\x80\x80\x80"[j])
            }
        }
        ASSERT(t[1 + 4*424999999] == '\0')
        mpd_free(t);
    }

    status = 0;
    t = mpd_qformat(&a, "\xf3\x80\x80\x80>425000000.425000000g",
                    &ctx, &status);
    if (t == NULL) {
        ASSERT(status&MPD_Malloc_error)
    }
    else {
        ASSERT(status == 0)
        for (i = 0; i < 424999999; i++) {
            for (j = 0; j < 4; j++) {
                ASSERT(t[i*4+j] == "\xf3\x80\x80\x80"[j])
            }
        }
        ASSERT(t[4*424999999] == '9')
        ASSERT(t[4*424999999 + 1] == '\0')
        mpd_free(t);
    }

    status = 0;
    t = mpd_qformat(&a, "\xf3\x80\x80\x80=425000000.425000000g",
                    &ctx, &status);
    if (t == NULL) {
        ASSERT(status&MPD_Malloc_error)
    }
    else {
        ASSERT(status == 0)
        for (i = 0; i < 424999999; i++) {
            for (j = 0; j < 4; j++) {
                ASSERT(t[i*4+j] == "\xf3\x80\x80\x80"[j])
            }
        }
        ASSERT(t[4*424999999] == '9')
        ASSERT(t[4*424999999 + 1] == '\0')
        mpd_free(t);
    }

    status = 0;
    t = mpd_qformat(&a, "\xf3\x80\x80\x80^425000000.425000000g",
                    &ctx, &status);
    if (t == NULL) {
        ASSERT(status&MPD_Malloc_error)
    }
    else {
        ASSERT(status == 0)
        for (i = 0; i < 424999999/2; i++) {
            for (j = 0; j < 4; j++) {
                ASSERT(t[i*4+j] == "\xf3\x80\x80\x80"[j])
            }
        }
        ASSERT(t[(424999999/2)*4] == '9')
        for (i = 0; i < 424999999-424999999/2; i++) {
            for (j = 0; j < 4; j++) {
                ASSERT(t[(424999999/2)*4+1 + i*4+j] == "\xf3\x80\x80\x80"[j])
            }
        }
        ASSERT(t[4*424999999 + 1] == '\0')
        mpd_free(t);
    }

    s = malloc(425000000 + 2);
    if (s != NULL) {
        /* 999...999.9 */
        memset(s, '9', 424999999);
        s[424999999] = '.';
        s[425000000] = '9';
        s[425000001] = '\0';
        mpd_set_string(&a, s, &ctx);
        free(s);

        /* type: 'g': locale: min_width */
        spec.min_width = MPD_MAX_PREC;
        spec.prec = MPD_MAX_PREC;
        spec.type = 'g';
        spec.align = '>';
        spec.sign = '-';
        /* four-byte fill character */
        spec.fill[0] = '\xf3';
        spec.fill[3] = spec.fill[2] = spec.fill[1] = '\x80';
        spec.fill[4] = '\0';
        /* four-byte dot and separator */
        spec.dot = "\xf3\x80\x80\x80";
        spec.sep = "\xf2\x81\x81\x81";
        /* absurd grouping */
        spec.grouping = "\001";

        status = 0;
        t = mpd_qformat_spec(&a, &spec, &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            j = (mpd_ssize_t)strlen(t);
            ASSERT(j == 425000000 + 424999999*4)
            for (i = 0; i < 424999998; i++) {
                ASSERT(t[i*5] == '9')
                for (j = 0; j < 4; j++) {
                    ASSERT(t[i*5+1+j] == spec.sep[j])
                }
            }
            ASSERT(t[i*5] == '9')
            for (j = 0; j < 4; j++) {
                ASSERT(t[i*5+1+j] == spec.dot[j])
            }
            i++;
            ASSERT(t[i*5] == '9')
            ASSERT(t[i*5+1] == '\0')

            mpd_free(t);
        }
    }

    /* type: 'e' */
    s = malloc(425000000 + 4);
    if (s != NULL) {
        /* 999...999e+0 */
        memset(s, '9', 425000000);
        s[425000000] = 'e';
        s[425000001] = '+';
        s[425000002] = '0';
        s[425000003] = '\0';
        mpd_set_string(&a, s, &ctx);

        status = 0;
        t = mpd_qformat(&a, ".425000000e", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 9.999...990e+424999999 */
            ASSERT(strncmp(t, "9.", 2) == 0)
            ASSERT(strncmp(t+2, s, 424999999) == 0)
            ASSERT(strcmp(t+2+424999999, "0e+424999999") == 0)
            mpd_free(t);
        }

        status = 0;
        t = mpd_qformat(&a, ".0e", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            /* 1e425000000 */
            ASSERT(status == 0 && strcmp(t, "1e+425000000") == 0)
            mpd_free(t);
        }

        /* 999...999e+1 */
        s[425000002] = '1';
        mpd_set_string(&a, s, &ctx);

        status = 0;
        t = mpd_qformat(&a, ".425000000e", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 9.999...990e+425000000 */
            ASSERT(strncmp(t, "9.", 2) == 0)
            ASSERT(strncmp(t+2, s, 424999999) == 0)
            ASSERT(strcmp(t+2+424999999, "0e+425000000") == 0)
            mpd_free(t);
        }
        free(s);

        status = 0;
        t = mpd_qformat(&a, ".0e", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 1e+42500001 */
            ASSERT(strcmp(t, "1e+425000001") == 0)
            mpd_free(t);
        }


        /* 9e-849999999 */
        mpd_set_string(&a, "9e-849999999", &ctx);

        status = 0;
        t = mpd_qformat(&a, ".425000000e", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 1.000...000e-849999999 */
            ASSERT(strncmp(t, "9.", 2) == 0)
            for (i = 2; i < 425000000+2; i++) { 
                ASSERT(t[i] == '0')
            }
            ASSERT(strcmp(t+425000000+2, "e-849999999") == 0)
            mpd_free(t);
        }
    }

    /* type: 'f' */
    s = malloc(425000000 + 4);
    if (s != NULL) {
        /* 999...999e+0 */
        memset(s, '9', 425000000);
        s[425000000] = 'e';
        s[425000001] = '+';
        s[425000002] = '0';
        s[425000003] = '\0';
        mpd_set_string(&a, s, &ctx);

        status = 0;
        t = mpd_qformat(&a, ".425000000f", &ctx, &status);
        ASSERT(t == NULL && status&MPD_Invalid_operation)

        status = 0;
        t = mpd_qformat(&a, ".0f", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            ASSERT(strncmp(t, s, 425000000) == 0)
            ASSERT(t[425000000] == '\0')
            mpd_free(t);
        }
        free(s);

        mpd_set_string(&a, "9e+0", &ctx);
        status = 0;
        t = mpd_qformat(&a, ".425000000f", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 9.000...000 */
            ASSERT(strncmp(t, "9.0", 2) == 0)
            for (i = 2; i < 425000000+2; i++) {
                ASSERT(t[i] == '0')
            }
            ASSERT(t[i] == '\0')
            mpd_free(t);
        }

        mpd_set_string(&a, "9e-849999999", &ctx);
        status = 0;
        t = mpd_qformat(&a, ".425000000f", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 0.000...000 */
            ASSERT(strncmp(t, "0.0", 2) == 0)
            for (i = 2; i < 425000000+2; i++) {
                ASSERT(t[i] == '0')
            }
            ASSERT(t[i] == '\0')
            mpd_free(t);
        }

        mpd_set_string(&a, "9e-849999999", &ctx);
        status = 0;
        t = mpd_qformat(&a, "f", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 0.000...0009 */
            ASSERT(strncmp(t, "0.0", 2) == 0)
            for (i = 2; i < 849999999+1; i++) {
                ASSERT(t[i] == '0')
            }
            ASSERT(t[i++] == '9')
            ASSERT(t[i] == '\0')
            mpd_free(t);
        }

        /* restriction for _mpd_add_lconv */
        status = 0;
        mpd_set_string(&a, "9e-849999999", &ctx);
        t = mpd_qformat(&a, ",f", &ctx, &status);
        ASSERT(t == NULL && status&MPD_Invalid_operation)
    }

    /* type: '%' */
    s = malloc(424999998 + 4);
    if (s != NULL) {
        /* 999...999e+0 */
        memset(s, '9', 424999998);
        s[424999998] = 'e';
        s[424999999] = '+';
        s[425000000] = '0';
        s[425000001] = '\0';
        mpd_set_string(&a, s, &ctx);

        status = 0;
        t = mpd_qformat(&a, ".425000000%", &ctx, &status);
        ASSERT(t == NULL && status&MPD_Invalid_operation)

        status = 0;
        t = mpd_qformat(&a, ".0%", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            ASSERT(strncmp(t, s, 424999998) == 0)
            ASSERT(strcmp(t+424999998, "00%") == 0)
            mpd_free(t);
        }
        free(s);

        mpd_set_string(&a, "9e+0", &ctx);
        status = 0;
        t = mpd_qformat(&a, ".424999998%", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 900.000...000 */
            ASSERT(strncmp(t, "900.", 4) == 0)
            for (i = 4; i < 424999998+4; i++) {
                ASSERT(t[i] == '0')
            }
            ASSERT(t[i++] == '%')
            ASSERT(t[i] == '\0')
            mpd_free(t);
        }

        mpd_set_string(&a, "9e-849999999", &ctx);
        status = 0;
        t = mpd_qformat(&a, ".425000000f", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 0.000...000 */
            ASSERT(strncmp(t, "0.0", 2) == 0)
            for (i = 2; i < 425000000+2; i++) {
                ASSERT(t[i] == '0')
            }
            ASSERT(t[i] == '\0')
            mpd_free(t);
        }

        mpd_set_string(&a, "9e-849999999", &ctx);
        status = 0;
        t = mpd_qformat(&a, "%", &ctx, &status);
        if (t == NULL) {
            ASSERT(status&MPD_Malloc_error)
        }
        else {
            ASSERT(status == 0)
            /* 0.000...09 */
            ASSERT(strncmp(t, "0.0", 2) == 0)
            for (i = 2; i < 849999999-1; i++) {
                ASSERT(t[i] == '0')
            }
            ASSERT(t[i++] == '9')
            ASSERT(t[i++] == '%')
            ASSERT(t[i] == '\0')
            mpd_free(t);
        }

        /* restriction for _mpd_add_lconv */
        status = 0;
        mpd_set_string(&a, "9e-849999999", &ctx);
        t = mpd_qformat(&a, ",%", &ctx, &status);
        ASSERT(t == NULL && status&MPD_Invalid_operation)
    }
#endif

    /* four-byte dot and separator */
    spec.dot = "\xf3\x80\x80\x80";
    spec.sep = "\xf2\x81\x81\x81";
    spec.grouping = "\003\003";

    ASSERT(mpd_validate_lconv(&spec) >= 0)
    spec.sep = spec.grouping = "";
    ASSERT(mpd_validate_lconv(&spec) >= 0)

    /* strlen(spec.dot) == 0 */
    spec.dot = "";
    ASSERT(mpd_validate_lconv(&spec) < 0)

    /* strlen(spec.dot) > 4 */
    spec.dot = "\xf3\x80\x80\x80\x80";
    ASSERT(mpd_validate_lconv(&spec) < 0)
    spec.dot = ".";

    /* strlen(spec.sep) > 4 */
    spec.sep = "\xf3\x80\x80\x80\x80";
    ASSERT(mpd_validate_lconv(&spec) < 0)
    spec.sep = "";

#if CHAR_MAX == SCHAR_MAX
    spec.grouping = "\xff";
    ASSERT(*spec.grouping < 0)
    ASSERT(mpd_validate_lconv(&spec) < 0)
#endif

    spec.grouping = "\003\003";

    /* both separator and 'N' format specifier are present */
    ASSERT(mpd_parse_fmt_str(&spec, ",n", 1) == 0)

    mpd_del(&a);
}

typedef struct {
    uint8_t sign;
    mpd_uint_t value;
} signed_uint_t;
#ifdef CONFIG_64
  #define MPD_MIN_ETINY_ MPD_MIN_ETINY
  #define MPD_MAX_EMAX_ MPD_MAX_EMAX
  #define MPD_MIN_EMIN_ MPD_MIN_EMIN
#else
  #define MPD_MIN_ETINY_ -1999999997
  #define MPD_MAX_EMAX_ 999999999
  #define MPD_MIN_EMIN_ -999999999
#endif

static void
test_scaleb(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    signed_uint_t a_exp, b_exp;
    int i;

    mpd_maxcontext(&ctx);
    ctx.traps = 0;

#ifdef CONFIG_32
    /* generally not safe */
    ctx.prec = 999999999;
    ctx.emax = 999999999;
    ctx.emin = -999999999;

    ctx.status = 0;
    mpd_set_string(&a, "1e999999999", &ctx);
    mpd_set_string(&b, "1147483648", &ctx);
    mpd_scaleb(&a, &a, &b, &ctx);
    ASSERT(ctx.status&MPD_Overflow)
    ASSERT(mpd_ispositive(&a))
    ASSERT(mpd_isinfinite(&a))

    ctx.status = 0;
    mpd_set_string(&a, "1e999999999", &ctx);
    mpd_set_string(&b, "1147483649", &ctx);
    mpd_scaleb(&a, &a, &b, &ctx);
    ASSERT(ctx.status&MPD_Overflow)
    ASSERT(mpd_ispositive(&a))
    ASSERT(mpd_isinfinite(&a))

    ctx.status = 0;
    mpd_set_string(&a, "1e-1999999997", &ctx);
    mpd_set_string(&b, "-147483651", &ctx);
    mpd_scaleb(&a, &a, &b, &ctx);
    ASSERT(ctx.status&(MPD_Underflow|MPD_Clamped))
    ASSERT(mpd_ispositive(&a))
    ASSERT(mpd_iszero(&a))

    ctx.status = 0;
    mpd_set_string(&a, "1e-1999999997", &ctx);
    mpd_set_string(&b, "-147483652", &ctx);
    mpd_scaleb(&a, &a, &b, &ctx);
    ASSERT(ctx.status&(MPD_Underflow|MPD_Clamped))
    ASSERT(mpd_ispositive(&a))
    ASSERT(mpd_iszero(&a))
#endif

    for (i = 0; i < 10000; i++) {
        ctx.status = 0;

        b_exp.value = (mpd_uint_t)randbits();
        b_exp.sign = random()%2;
        mpd_sset_uint(&b, b_exp.value, &ctx);
        mpd_set_sign(&b, b_exp.sign);

        a_exp.sign = random()%2;
        if (a_exp.sign) {
            a_exp.value = random()%(MPD_MAX_EMAX_+1);
        }
        else {
            a_exp.value = random()%(-MPD_MIN_ETINY_+1);
        }
        mpd_set_string(&a, "1", &ctx);
        a.exp = a_exp.value;
        a.exp *= (a_exp.sign?-1:1);

        mpd_scaleb(&a, &a, &b, &ctx);
        if (b_exp.value > 2 * (mpd_uint_t)(ctx.prec + ctx.emax)) {
            ASSERT(ctx.status&MPD_Invalid_operation)
            ASSERT(mpd_isnan(&a))
        }
        else {
        #if !defined(LEGACY_COMPILER) || defined (__GNUC__) || defined(_MSC_VER)
            int64_t x;
            x = (int64_t)a_exp.value * (a_exp.sign?-1:1);
            x += (int64_t)b_exp.value * (b_exp.sign?-1:1);
            if (x < MPD_MIN_ETINY_) {
                ASSERT(ctx.status&MPD_Underflow)
                ASSERT(mpd_iszero(&a))
            }
            else if (MPD_MIN_ETINY_ <= x && x < MPD_MIN_EMIN_) {
                ASSERT(ctx.status&MPD_Subnormal)
                ASSERT(a.exp == x)
            }
            else if (MPD_MIN_EMIN_ <= x && x <= MPD_MAX_EMAX_) {
                ASSERT(ctx.status == 0)
                ASSERT(a.exp == x)
            }
            else {
                ASSERT(ctx.status&MPD_Overflow)
                ASSERT(mpd_isinfinite(&a))
            }
        #endif
            ASSERT(!(ctx.status&MPD_Invalid_operation))
        }
    }

    mpd_del(&a);
    mpd_del(&b);
}

static sig_atomic_t sigfpe_raised = 0;
void
sigfpe_handler(int signum)
{
    (void)signum;
    sigfpe_raised = 1;
}

static void
_mpd_qadd_exact(mpd_t *result, const mpd_t *a, const mpd_t *b,
                const mpd_context_t *ctx, uint32_t *status)
{
    uint32_t workstatus = 0;

    mpd_qadd(result, a, b, ctx, &workstatus);
    *status |= workstatus;
    if (workstatus & (MPD_Inexact|MPD_Rounded|MPD_Clamped)) {
        mpd_seterror(result, MPD_Invalid_operation, status);
    }
}

static void
_mpd_qsub_exact(mpd_t *result, const mpd_t *a, const mpd_t *b,
                const mpd_context_t *ctx, uint32_t *status)
{
    uint32_t workstatus = 0;

    mpd_qsub(result, a, b, ctx, &workstatus);
    *status |= workstatus;
    if (workstatus & (MPD_Inexact|MPD_Rounded|MPD_Clamped)) {
        mpd_seterror(result, MPD_Invalid_operation, status);
    }
}

static void
_mpd_qmul_exact(mpd_t *result, const mpd_t *a, const mpd_t *b,
                const mpd_context_t *ctx, uint32_t *status)
{
    uint32_t workstatus = 0;

    mpd_qmul(result, a, b, ctx, &workstatus);
    *status |= workstatus;
    if (workstatus & (MPD_Inexact|MPD_Rounded|MPD_Clamped)) {
        mpd_seterror(result, MPD_Invalid_operation, status);
    }
}

static void
test_misc(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    char buf[BUFSIZE];
    char *s, *t;

    mpd_testcontext(&ctx);
    ctx.traps = 0;

    /* signcpy */
    mpd_set_string(&b, "3", &ctx);
    mpd_signcpy(&b, &a);
    s = mpd_to_sci(&b, 0);
    ASSERT(strcmp(s, "-3"))
    mpd_free(s);

    /* isnormal */
    mpd_set_string(&b, "0", &ctx);
    ASSERT(mpd_iszero(&b))
    ASSERT(!mpd_isnormal(&b, &ctx))
    ASSERT(!mpd_issubnormal(&b, &ctx))

    /* maxcoeff */
    ctx.prec = 28;
    ctx.status = 0;
    mpd_set_string(&a, "1e2025", &ctx);
    mpd_maxcoeff(&a, &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "9999999999999999999999999999e2025"))
    ASSERT(ctx.status == 0)
    mpd_free(s);

    /* canonical */
    ctx.status = 0;
    mpd_set_string(&a, "1.182736812e999999", &ctx);
    mpd_canonical(&b, &a, &ctx);
    s = mpd_to_sci(&a, 0);
    t = mpd_to_sci(&b, 0);
    ASSERT(strcmp(s, t) == 0)
    ASSERT(ctx.status == 0)
    mpd_free(s);
    mpd_free(t);

    /* mpd_msword */
    mpd_set_string(&a, "2.3854727e+1875", &ctx);
    ASSERT(mpd_msd(mpd_msword(&a)) == 2)

    /* mpd_radix */
    ASSERT(mpd_radix() == 10)

    /* rescale, greater than MAX_EMAX+1 */
    ctx.status = 0;
    mpd_set_string(&a, "3.3", &ctx);
    mpd_rescale(&a, &a, MPD_MAX_EMAX+2, &ctx);
    s = mpd_to_sci(&a, 1);
    ASSERT(strcmp(s, "NaN") == 0)
    ASSERT(ctx.status == MPD_Invalid_operation)
    mpd_free(s);

    /* rescale_fmt, greater than MAX_EMAX+1 */
    ctx.status = 0;
    mpd_set_string(&a, "3.3", &ctx);
    mpd_qrescale_fmt(&a, &a, MPD_MAX_EMAX+2, &ctx, &ctx.status);
    s = mpd_to_sci(&a, 1);
    ASSERT(strcmp(s, "NaN") == 0)
    ASSERT(ctx.status == MPD_Invalid_operation)
    mpd_free(s);

    /* CONFIG_64: Over/Underflow in power */
    ctx.status = 0;
    mpd_set_string(&a, "1.000000001", &ctx);
    mpd_set_string(&b, "12345676891234567891123456789", &ctx);
    mpd_pow(&a, &a, &b, &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "Infinity") == 0)
    ASSERT(ctx.status == (MPD_Rounded|MPD_Overflow|MPD_Inexact))
    mpd_free(s);

    ctx.status = 0;
    mpd_set_string(&a, "1.000000001", &ctx);
    mpd_set_string(&b, "-12345676891234567891123456789", &ctx);
    mpd_pow(&a, &a, &b, &ctx);
    s = mpd_to_sci(&a, 1);
    snprintf(buf, BUFSIZE, "0E%" PRI_mpd_ssize_t, mpd_etiny(&ctx));
    ASSERT(strcmp(s, buf) == 0)
    ASSERT(ctx.status == (MPD_Underflow|MPD_Rounded|MPD_Clamped|MPD_Inexact|MPD_Subnormal))
    mpd_free(s);

    ctx.status = 0;
    mpd_set_string(&a, "2.2122163", &ctx);
    mpd_set_string(&b, "2899999999999999999.891239129", &ctx);
    mpd_pow(&a, &a, &b, &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "Infinity") == 0)
    ASSERT(ctx.status == (MPD_Rounded|MPD_Overflow|MPD_Inexact))
    mpd_free(s);

#if defined(CONFIG_64)
    /* CONFIG_64: prec too large for exp() */
    ctx.status = 0;
    ctx.prec = MPD_MAX_PREC;
    mpd_set_string(&a, "2.2122163", &ctx);
    mpd_exp(&a, &a, &ctx);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "NaN") == 0)
    ASSERT(ctx.status == MPD_Invalid_operation)
    mpd_free(s);

    mpd_testcontext(&ctx);
    ctx.traps = 0;
#endif

#if defined(CONFIG_32) && !defined(LEGACY_COMPILER)
    /* Allocation failure in mpd_set_u64 */
    mpd_resize(&a, MPD_MINALLOC_MAX+1, &ctx);
    mpd_minalloc(&a);

    ctx.status = 0;
    mpd_set_alloc_fail(&ctx);
    mpd_set_u64(&a, UINT64_MAX, &ctx);
    mpd_set_alloc(&ctx);

    ASSERT(ctx.status & MPD_Malloc_error)
    ASSERT(mpd_isnan(&a))
#endif

    /* _mpd_qadd_exact() */
    ctx.status = 0;
    ctx.prec = 38;
    mpd_set_string(&a, "2.2122163", &ctx);
    mpd_set_string(&b, "2899999999999999999.891239129", &ctx);
    ctx.prec = 1;
    _mpd_qadd_exact(&a, &a, &b, &ctx, &ctx.status);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "NaN") == 0)
    ASSERT(ctx.status & MPD_Invalid_operation)
    mpd_free(s);

    /* _mpd_qsub_exact() */
    ctx.status = 0;
    ctx.prec = 38;
    mpd_set_string(&a, "2.2122163", &ctx);
    mpd_set_string(&b, "2899999999999999999.891239129", &ctx);
    ctx.prec = 1;
    _mpd_qsub_exact(&a, &a, &b, &ctx, &ctx.status);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "NaN") == 0)
    ASSERT(ctx.status & MPD_Invalid_operation)
    mpd_free(s);

    /* _mpd_qmul_exact() */
    ctx.status = 0;
    ctx.prec = 38;
    mpd_set_string(&a, "2.2122163", &ctx);
    mpd_set_string(&b, "2899999999999999999.891239129", &ctx);
    ctx.prec = 1;
    _mpd_qmul_exact(&a, &a, &b, &ctx, &ctx.status);
    s = mpd_to_sci(&a, 0);
    ASSERT(strcmp(s, "NaN") == 0)
    ASSERT(ctx.status & MPD_Invalid_operation)
    mpd_free(s);

#ifdef CONFIG_32
    /* newtondiv */
    s = malloc(425000000 + 4);
    if (s != NULL) {
        /* 999...999e+0 */
        uint32_t workstatus = 0;
        ctx.status = 0;
        ctx.prec = MPD_MAX_PREC;
        ctx.emax = MPD_MAX_EMAX;
        ctx.emin = MPD_MIN_EMIN;
        memset(s, '9', 425000000);
        s[425000000] = 'e';
        s[425000001] = '+';
        s[425000002] = '0';
        s[425000003] = '\0';
        mpd_qset_string(&b, s, &ctx, &workstatus);
        free(s);
        if (workstatus == 0) {
            mpd_qset_string(&a, "1", &ctx, &workstatus);
            mpd_qdiv(&a, &a, &b, &ctx, &workstatus);
            s = mpd_to_sci(&a, 0);
            ASSERT(strcmp(s, "NaN") == 0)
            ASSERT(workstatus & (MPD_Malloc_error|MPD_Division_impossible))
            mpd_free(s);
            mpd_minalloc(&b);
            mpd_minalloc(&a);
        }
    }

    /* sqrt */
    s = malloc(425000000 + 4);
    if (s != NULL) {
        uint32_t workstatus = 0;
        ctx.status = 0;
        ctx.prec = MPD_MAX_PREC;
        ctx.emax = MPD_MAX_EMAX;
        ctx.emin = MPD_MIN_EMIN;
        s[0] = '1';
        memset(s+1, '0', 424999998);
        s[424999999] = 'e';
        s[425000000] = '+';
        s[425000001] = '0';
        s[425000002] = '\0';
        mpd_qset_string(&a, s, &ctx, &workstatus);
        free(s);
        if (workstatus == 0) {
            mpd_qsqrt(&a, &a, &ctx, &workstatus);
            s = mpd_to_sci(&a, 0);
            ASSERT(strcmp(s, "NaN") == 0)
            ASSERT(workstatus & (MPD_Malloc_error|MPD_Division_impossible))
            mpd_minalloc(&a);
            mpd_free(s);
        }
    }

#if 0 /* too slow */
    s = malloc(99999999 + 4);
    if (s != NULL) {
        ctx.status = 0;
        ctx.prec = 99999999;
        ctx.emax = MPD_MAX_EMAX;
        ctx.emin = MPD_MIN_EMIN;
        s[0] = '1';
        memset(s+1, '0', 99999998);
        s[99999999] = 'e';
        s[100000000] = '+';
        s[100000001] = '0';
        s[100000002] = '\0';
        mpd_set_string(&a, s, &ctx);
        free(s);
        if (ctx.status == 0) {
            mpd_sqrt(&a, &a, &ctx);
            s = mpd_to_sci(&a, 0);
            ASSERT(s[0] == '1');
            for (i = 1; i < 49999999; i++) {
                ASSERT(s[i] == '0')
            }
            ASSERT(ctx.status == 0)
            mpd_minalloc(&a);
            mpd_free(s);
        }
    }
#endif
#endif

    /* Default trap handler: SIGFPE */
#if !defined(__OpenBSD__) && !defined(_AIX)
    signal(SIGFPE, sigfpe_handler);
    ctx.traps = MPD_Rounded;
    mpd_set_string(&a, "1", &ctx);
    ASSERT(sigfpe_raised == 0)
    mpd_div_uint(&a, &a, 7, &ctx);
    ASSERT(sigfpe_raised == 1)
    signal(SIGFPE, SIG_DFL);
#endif

    mpd_del(&a);
    mpd_del(&b);
}

static const char *three_pow_9999 = "5437833951142086247677522430603849056040441511941793313116067753972752436047730262493385210282616103224417348407745265144468527666392401284127159715122704167151064118388741463675141209614518895519886548337338059149425176663457758141815475250411845780622462535937873312408538957620672168268531058019969840153889500221039642303857257817418616875656175417293046622361693490846546977910081136717395638109639118192284819091398520739337779248542834156891019808999704734180257725329399947495758727895099796309275098928813417666481595231394558441173901477398595268756707796127643923399663613766953835665369648260688338387803448298315269779229718935895557820854346054930730221780041448044660254401864788251654483854690780167535459263528371374604973000545725678152461673313020077641731490371411964326188440805209448047394947630694488742625253500425336600510234345841946385964625247064165901766823230354151456642265325152113412259244784889046357533858323278104256551185728132049195690420147982648287411914819903734824354312588051256676736777394545177272724485539340915325018437737532838143195848515765511536532398278678848956830590642541738117652501151649467878607448576258295058603166719483945796471266031776570664364033805809468924775375483335155962912182084982886200828164845312962885514248992474789461778498694388307106200644992341698593456701435993425452285783449735288692790602021847464512033891134005993650532982326888110130501948601224749449626023670137639555778607949966541162071683434684192596616170803421278270573078983103807803941195105468361768721645557045818995131428559108257258348683656955173631198717229005717951918399116066553023397651590436115875703684157044825441995454279528653184069881794029685060957284955581544579177739174960039467262531698020285239003381362378831844139514747163145693358238719468298638045483934010815569320352919211115230565581097956541248470290017283863557619782353737524482190681696155153538942027526577228668195072761444484132052576678768793140607478606000501968401306069735656554108902317541170809126746822928038178367165577994467080073280016318911229769873440537931252385575854153883977799180088931987955307196971107353524224449949751101132624747460653591779060910012594566236249593912806582566293440533728770168777623234977101025613148263656113102100424457004695749353648950597899324371477568919762848692540943667508933518283232289236760315315534556531265784880712207410397565009107076503970984313440293254374383920185319832320387713818562759614316529096133033424563960006258049189630883395384021778406371024194139233205633998510083312578406366595586318297511455278411947718704266576054707820406605781301594134172370534402191776842616917521547665106021661966938519326648628643708117101084270418418098599027427315552932901993025830823283214727722195316948317721575632048722765936059466156817187106842060180770280248281035823122986569242278869490605686866257243390251815491146924677123742479217016140535408252509028652838649091050342466887649387037094925942701177166496304557600261055962683725718475761701287245552689801599231966919606821746224656960886720761702533073806247885668957512614225961236269171950132563937477960187257550788282432339836008616321611294408388556328978360424865431327707288015425756196800526736353329500170420823389050165087302712229564620441360519682112096321012363437344507466907453627498309394269691802304773319014975834806932426039279225810482326361918810996917860862700813413537013028172133696649622811117916148034879911358173055022544313806449328541478522447357605303584828041270972495770979426890134409396271001326128027110747494882190472524801617654225055205367175789141884956502904000596131282057260152485321015254981188654962506132160432053300823998911836284764321512265593134792410321907787208394010266099244928618202020107806363882237046226163642946702483307818944921254371408666923438977081222233141898276060385016281053793610689281826856586152583696880332002414006085133542425636062694200598506722351567109267455663249027611827035181292815629965410232030627214515825384999524671969132019826228326191393495599639479547721539513783896419459680775495889826577794904332251417234602679276212199884815909265308198794094408907834462180197107089534407630935854169933671921724786542596119293868788706344317276622758205846648973740443428085675686217720764160895559758422254669960661863549511514979741432229930398451927735322830807545425885578446162766368079353429736463551489105430530406570955350887115675618420429793745627021022739749688255695498394725605543025701222377168354988063636816732167392990825787293766869562864594169931260485723465632498969620631011544965238372371533838218127115713030781944490742164552310332015005451936054328321734553839727400715138288853220739598988109739080499368402184066667";

static const char *small = "2873";

static const char *five_pow_9999 = "100247454984129040185951118674859554986431355626768516788428580845584790619015680803782213924968448267230436985724116744162654886353782546032964337909591562371748418222321224578769755680421336267547105216204991579144325942478817982804545796958084386975812342618954509718441821640520746651458033349006164894788489386408105733188461799044303384497161957359029746319216484276156090830167757773403304607984474626207728399635069267684575441210965512900647074233868487307465074388652403777886987998070560948339435101045850899066361137952974879597984655047768304714722115337098145360571963848730749146493938419071227963890446727691411354004280048118977143705005393255706620294445944028806458157235366060629124733151099272087119898064730114502605113646962740980616622088993478063095701528621941854494043762375136965685752969378453076439004777330424743418463845159118900992081808987073631904264755695403548355522428436364201354282132511836995332199985145011808098664492949703233539507946188089768016491172958885523167210910958277136075248111141141062128083782963686545126514780108875164857845022011076207514423897756560427450908148616071924350876291381734481768062579454770596761316628244432141545764410607647758305186980063791709206249986404819624174565617704542517140552439164593999947429802055682254653591169061406884592460338668363999428398025922529686796324178958415803853955843718152201088182155525426152961819626219326496567753846248270955462112101286783336090088034200615014527622429396149772594589257985948535051640671785088888951225697171376828975482993848505733392715713727598155271359788848873614255101982617566498645671745543892715387582833106486321747259164787170171920591300221908846041430068586274908920346105708598057471023445371487175798526935507756615346388620286246209043956346674921720491233794205383338627231795955268671279509665464537966965361650346927308204215582009955804728453691593210959560964947271128827512966119039090342047513575813287927451772600886235203677355225025729781525995154136552746838322439577129797574882833135408593342309971317730719681757100102137084463420059727308619188356240133137382161335208358398332102961126400207132374968330517664145141440923678428408296845122438491438794908419810746169519572667541839817589648104074802408698971293261141416249496394440589273563839041919177115312997095958074483038135533400119746230369962964356024718441783528428406030406014469096286676405665930638568662579425949247286396207550134629139781634277168538984489054962005182447683244304592870262126143239768786577964214406153501745418237881168542720044939222114647181308975769381077415571542809363427452763789986399956163686443345628665350200416915198396889792625285865971125294538604926524056877837913934125323028297296160933490753284010499241779579901067959381415149907585009527896219952678824590854333670842878092353415083789978151451963038347052961994096014124672252314677302884708423373991835245400210312683158013323346165077976510052109901471222985316132382071644437275010467325253279280584126491119681579335449815152265639526573450169875756410862595748429248874590230716064947375154603186117836401110522732676141641774418028150224221614037283906866376484676473446388823344997174476905589245906795040081201800235160510642842230716751267031420153188381203290124544211210957121581437122881741914121965745906438864814697660660433807276392172877715238265244412648906746631316275081731800257064336961067073723513656669373443265810546532524012731748366333757593940642668094708548708485701628218251083890353997693371217841690104148381540419920096667099704240232781336386013398667247218830558728105818937202998209417713421007252048230596893429774142698034342108922335752805621373165094401208392773190528055271890040862197747643941164068162495363093819492245238757693134024227491053476083455086511514574321762625348221285413543015540089240592663805785456393806332332547081390435014128689325175600858843505185146557508894297503528457563333115042485179641160829642307202370188133120628805019692380157580570228395729736967197936590984317477471899834898524941775909899005139873124236109790137019567917137631499569960760564342850274516010539282544886940734804400212763222850377215846548679398319367484716873279637845292976283966623980150785690775842744671798377032885433619270654572210109094537761090220728769602752968712100411463912843492629131941509805016845240505652185412814397333478509745291934234575329371466168717812806328420119886369083590142362411806688713422442551593246218269417311182636566802999325747571972141272635908911728891197639847806536644117132946448482982660058361033776791683242928607300495723556719231498646558852129116447602173541068511828052845761733122020642966199402089289030702034545601391133881061373713856884669010432415226620216551696840918572962555399529337255969701382373391297167178627921690174345420069590872741274632707801650106017358071236781656096186712279753094478201270184823174959159381753832552987032537024243450117434196379235466732364904303548150924813158207277854946984370453497978859232915008903871930122875730882839605806996353958169033167559181209263006513103870539292553221293354821014241157566853441236613832955534731066254267351920664092034625285948220614923492010322155875403024551854196369146808965072359033231164060183935567324047356670635802143726660249415329713985640089000490443094168056980956826745835698880364240561274483767888597055000121949959600371148918390198295217885327467444472037260873146908761678154485624054035741947554053522731324606671013553086037064421471370698268979101829486164876478520624762684746001759534019006814663246154369745586629372149349173159088872884826199287070529097396590344688104429660459416727066253381100684153997213755267855583920120073712462822863948713657109391932635501829789007740395369451324039932668121895842601247886334430486126615659923839289186221228400745154199401528727611514194038331081855431274686621283256224920455519205277666206223203993572190480333941044105873180159910653597328161913481264072462348795424423826391290720997121553147509488194197491908430820440690452657592193675509597543178018750599498681477651501783226348344772444035447377792681508065323815816706209115092044961135554021749806303567433536642052125158334822890877393458220635427042227864341821374029709366541380042643295381657445791490952403465843363868312598546459683822089090234601413163213905737774545771609040860157199695063289879719568012463359356597161991667741137105818559651391500166613798826810153754001981073201322296020391996558030100238025622434425141405025733945788921204426519176284785323877786347150787077862484552498007465482208870091993878907059622061340893852201267523733882680885737088060533260126717081174569523656784208901971175491745625811867740895958809776754345037791425652775224494618873322178518653683435864152918609199470788473090647141847311089000528399432444876736648651245525146297278110929986040436551173935276787205508874166568107633157325153661076910793781280517578125";

static const char *powmod_result = "21808545079390503732407067212917542883053003446538364303220786803463642420259989688389402566286208286827711231083997839766138513374519593570283458104469284339996150845874172337704240602425598275087811131832945273190203233295652697160219712957580004156793345916727785234247973255974225269464407054221344630053606626633191737090764230974162618853756635704621428184787177446935028148793215133957451949292959014384147336651312479136750416608543146775521768046630920076115838027439649667201254591180725889405582408116948865382292722627052407149018230343691983567966851863919016034839021747053169118784908457954412399930158832254102586110102033546079006379056668748370788762697065143822616629318138794710190782648371744976632094775033478534421979058084998539008430575449388055485736969664659601277104365735578815546708002234545929348030831114213509431428697592022930281807224770149340019545731799449727716280638224957846485829880331721049811088887185883983120255142298741490366771111580331821650264747245221467500051087306323460661469723718171871739302398496801504979294920039884314985804381011168823608930124619979169505385856829153528272906403795212467471668833499992684495421815241114307960157569547021390211802185073165110914529933359389606399903287216094456488472299165221154793753725541790468732873049033651126453056289015635717684874727752624295185515685798541585492410987722797778755087899078705362061935739562969385381692349170995260936699256720112495345068331485280159406962241040536282728391265737668685242117309138816577717341335452968215760748653323465682497476561521164629486997488155313839368426107305327735350293079014277841843801126257299237962176757880766807433856897545403065066595353897896822676782528754729489481659363789998462240650501090621477727566003623183507592918508472659637170231819613129782260862556801141441291683615422738762662502581377375353564154745399797644547797137652287995995040460068932373701023690253960506959476358087382863296193297709369738200995086002655604267283157241403794603126985535765553997359703243404555469658074270419711116819596441205390122435495638999484413118370730532478142282332044569207693943721623493070248155383218107960233856211672667172241488393244697757011187799229323669589360904255967801296911857624984485780787415965420656012684189001142712319675092771002092770354075217279076628725021137688955671582072194864880927611044600488864000560411249848738921425795872443595494703521016037478210763310963513075339684494892807771665496651875566187964374366577466641372131849681883405312908423954786612841936850775491917471514496458753332624742946562388861876354789991217094078682083334545516305080792713541490856992928198814569701231512793873423908619802804376777932553027051066800546275820208061203563343987857992800314750967201296594857850178542475276240398937923726865307438852024195940458074291026134762919656626818156345055649881323574899888631912802036245218145777842943149445972572572150723207049485600136028151731484791904898928190018060825858871393942902175344451767542679502547127878063490159400905416257127822224487907398971222706054758745619440552021458548819063971060591209464551186477455709719198135747198608505167253000649863300857319897086903434737268897334127608417762990391437124516528299269622494509898790096554886211205026292106797670051767975613097349973827713555819424629121803691065541129078368794510068461270208281719538977223736731176458227397718916157123937925604965770660617655076943860009529894122317198829016540867454260064538552739486652024593980995391365883597216737870106345876418276559685480678884876725506623102468246055829932986711340513562032625870375779340395312624442365649845291600075265746400053747253709802071492935696467238770257053784796646939927754719384223324374208083214920494078834856268050731441908077138262515713138532780042685785927868665867101568120448413317946390483857653275135571197854241920991278997712310917850889088799238939520787573343547068779247558850030995898862479778786062088493332710500175850191477169370919821284660818535120486250091739111513258457226073289612267554046016238770762747732012108853248009418296684282575489329489891934656548516454160638164338590582799901089667569675471069525597430828415763037445560721090337370868679612316290987923748910774854807047066796623728872628447034257622028626556422905800981138090627441385181545484093592776104896524999067734059389598504618336160399625571425691706124284403859648485879762427768926888962930062075308379190613564482954658804321948359259949086979492491136044381731147504704337930443022103399857980405772065274859616067935424899503258563515445529905211231569645771212464592454778628866908014021185632879229917380271121711966110650649735867348767244191746073277666134555088656800934665245945106940802909640913848041474742779060764627665747889436712730019944757029252264801210658492103696941411496961664638688013610017642633934168985728109278334229616662132092214546364706700337185232027845417184859229716818169592631740915584348914616104568166196864943836423850492529397471387173803245451937376907973615017790207016389415378857797886690654368525345805140302308625326707092250342300991501103237022350407260110733656159321260141192464621689232636162394330022567797696604655354126284575005081730579391227131916932376018670802092279076754735019859959365861039459464409172939300131748047443382490968283542821761073967447590247625985335535104520312843795330853483857489849135602364250854989019373642938735475147024962555420565039385284121074548392523367408969943665092184042673033656584263200972855109288871807002468593095946212716092684693287048954656480884909599817003205628314495071519865395802150046038959843126173669906347753173113879087115885483690705472242228988739363309901984399519902034531545774896141064732330817355944339336522490279337924995782594844610492863822520898746248372002508800285324365704021101806350103762012463283647392861036199966251885891288270854319871758017625989839682017381356816383606943097144956586050806114924590504998883341115924312186447935351828241297224585012069121299563093233820153412315183686893433994436246591404016369132023899255832279215633072725191557115213299808709872057494034735787686840548013531154604568908292649730499091434780868115356264320167549272015717692260034378835260382697640676645452281519774543117636669419673022480960389838989691333845181168240337015751745352084362865418349702118857598413799109430764994897897076861555788454426984448897407000617233018352104696007171315688352677312082101291765229398056061606662356417943811011178090488472978970045437933444302634642564541548368352774346658366203471870803693106158879954915487808286988631762704644311876817155329859833308849331835987755572501198963020333166849715258471183431664925346812511348242252429211599396670931863642480101984984595800311025828834297926877029945955697227866864046935190561473796218164847832820864252344402501292416629110205655550296975360373434063470845128791662444331024631846423927590478987";

static const char *ndivmod_result_1 = "3.14172335600907029478458049886621315192743764172335600907029478458049886621315192743764172335600907029478458049886621315192743764172335600907029478458049886621315192743764172335600907029478458049886621315192743764172335600907029478458049886621315192743764172335600907029478458049886621315192743764172";

static const char *ndivmod_result_2 = "1892737191486977461774285565821040395419575883028817721237754178201445331029491911762403484261265611982045718206663858386518805313746049872651291233944554182788396838979722054881705955313093942053563017172759505447067586725881572621585616167912233129349969556539461647201022957751713250354518293776529704195575878949195837905971896212119254046521467252799528932252590842619751819669363430810092460184350545837899345315488520967399157413345922087327191022972399837862950826776679410893059076886564495756796066456252494836923632172431102833683919762408143149584652905021804358997446437092570078546943838587082609950505899164049867657232759810614534570433117318110243724949544534648332841768835638096642702351093205766632599813271274408146527323545327420171410258723640820620164110814971097920706035783226400294951252220916981810868518447763778837629736980801234384255003566677398503921623122295214568967025870223499273323788647716340535166675364872295251149037844807535397038085676290514544870140904943868717143860977393406431164906855045310571780189884908080516887726327021523892515088240781591206589766195154489716961570011326744906944831587765216804249024913420917180161213616249197979976075889932673395184139855833438539775626691032076562099610889308348834259716270557940440485293766959550804656630245175185209258839189816115069076471088058971615832155743033515034037800912581783679789032765056036773593102097768224897494587052288461346893847431304434234520921667235134410745451943851093838014678316540646804933198393005154125980228021395183351452017249232794638054795878561871652749280771651644145909751907067775131193316782480526635362913884593148581866928004540489189695598426567578553452795612262333818641588915969907615446480743111743636360087608082243382876220453316683551736691523551390773880347622508979686058295835019427625938166696557993466956345665839380873118792209287804927817536496253924288962993977081501405827367743326567583778305868932991344257771207368184177030100454577271830282389860680701898401984795524250194614252287827371450876145345629817988191186918314970842244995976518597720983493161905285060145903267988857624541314643615507665932631378263522844630959784425197277779641494115755738339573238011340955897473210486867444753784627559032727327931870398925548938539664811937260478123547023285528844927799832844708907950206636793920472775228781495565720611045790647847471242851447062442591371077993910188299447768984773537899032434848531959759037180147149187022808595939099695288534634966254503874306785723641412602287164703842406267189000525612012731780844620809717472158529508671057333346420666917754201324958503759691658257003318622412731236553051695630931119394939772690051975860991522348805116980254235439644482780169619057755970063705475399680896180898780131203730498285984005820663395774265978704207137780041831106954489481738887268703992078760483637523733681259047473112361813834641377349022791234603599211767578318492395697406575374292539814705669000033040704038001102156736706439352960655302375815689440202327617665398469056467792175289530715706459088369899325340248198331447227869459628476932113204409736146930930712662011899497929268458904399525237945117163694538248714350497166872560191899654468505195181412043343799635687260762415880051879625857049325523691131375343028082879253853467582122291857802613106031151517366609708250019064851728150307973175226606083963038803455051137769229696436081691894679268017065148671666080620539894854210941482739769639072406037004457518039868248702557381693457565202938138173510536365968287551134492262791968718422245300207059585963287435300262521773479069674830888230057954369648658590769946836112943028546903926623270087889360306450418830777584027788473638817094418932521682008465683183852556216591054383894402437180684930861079790130952371612581518441805714912955535806268244665311165012607808263800304928780026551010718277302221311123072103231359860888270941178588629844213587212911312425515740463476195359365006351906582605999018897603769566797853125870941356013012530290522787724336869553890077060040236560904819951806304832543132425924391053216090002713158486686883826871799260776957843653292469651304836092728474707435496065822168302961235967926964822022945257496186075920394663692553548317882569703517524068560299493013604481617200738170609431103292176211162534167025252179434382088907841952801410347279959217127582814439811502319097239150488489292190585272922182572365670363853423987356637111183360162069969029843282174819246605776096625667603794369083594972150387621586053660770271780642984255784741686249275994173507039145712669324615604250450736247258047871158446963841181006189329791417391072854274979117804363402133074046137779229864502310577394090059032018137744197876564010506114.3985381134702401670727462582666202575704838148277062304211625478593804385659589279498781761225200139227288548555516881308736512356421858684302123216150365471632439958231813435433344935607379046293073442394709363035154890358510268012530455969369996519317786286112077967281587191089453532892446919596240863209189001044204664114166376609815523842673163940132265924121127741037243299686738600765750087017055342847198050817960320222763661677688827010093978419770274973894883397145840584754611903933170901496693351896971806474068917507831534980856247824573616428820048729550991994430908458057779324747650539505743125652627915071353985381134702401670727462582666202575704838148277062304211625478593804385659589279498781761225200139227288548555516881308736512356421858684302123216150365471632439958231813435433344935607379046293073442394709363035154890358510268012530455969369996519317786286112077967281587191089453532892446919596240863209189001044204664114166376609815523842673163940132265924121127741037243299686738600765750087017055342847198050817960320222763661677688827010093978419770274973894883397145840584754611903933170901496693351896971806474068917507831534980856247824573616428820048729550991994430908458057779324747650539505743125652627915071353985381134702401670727462582666202575704838148277062304211625478593804385659589279498781761225200139227288548555516881308736512356421858684302123216150365471632439958231813435433344935607379046293073442394709363035154890358510268012530455969369996519317786286112077967281587191089453532892446919596240863209189001044204664114166376609815523842673163940132265924121127741037243299686738600765750087017055342847198050817960320222763661677688827010093978419770274973894883397145840584754611903933170901496693351896971806474068917507831534980856247824573616428820048729550991994430908458057779324747650539505743125652627915071353985381134702401670727462582666202575704838148277062304211625478593804385659589279498781761225200139227288548555516881308736512356421858684302123216150365471632439958231813435433344935607379046293073442394709363035154890358510268012530455969369996519317786286112077967281587191089453532892446919596240863209189001044204664114166376609815523842673163940132265924121127741037243299686738600765750087017055342847198050817960320222763661677688827010093978419770274973894883397145840584754611903933170901496693351896971806474068917507831534980856247824573616428820048729550991994430908458057779324747650539505743125652627915071353985381134702401670727462582666202575704838148277062304211625478593804385659589279498781761225200139227288548555516881308736512356421858684302123216150365471632439958231813435433344935607379046293073442394709363035154890358510268012530455969369996519317786286112077967281587191089453532892446919596240863209189001044204664114166376609815523842673163940132265924121127741037243299686738600765750087017055342847198050817960320222763661677688827010093978419770274973894883397145840584754611903933170901496693351896971806474068917507831534980856247824573616428820048729550991994430908458057779324747650539505743125652627915071353985381134702401670727462582666202575704838148277062304211625478593804385659589279498781761225200139227288548556";

const char *sqrt_result = "2331916368813874561520109075522878967103534249021893897325652060993062062281966598498849985330743119016095046407839113418673396131535939299965883446605502250696541434141755578976270260264090932356207608232868385541675876088034212186092908379109085156908920883312059003237512027696658964891407414355274049179950685325942508627596717774959742037921510798423441639201632919683450357160306403073566069169785826647363800069948679256834819682647514628386538836084502682317169603695344841486672623508802412811959792906751443407521495092947995397584906220135484178951136846418672315185381025889926280011024530521659809825730287084081855959698722649688701017248595766215207094587205813656741140540604221425930089114513814482992004867810224888605808178447418812402788826983811591296977193340618673275316990361469083364568897465032115311395692919804281907481939121602544793836284080726287074441915613996827313018668742616227573178641813829635780437989743910968742637895338618404535644647562074543188528986251527833254208681404120300742023492768486616366563210728182842398248546164176898835246530917783248524280372422710586671469148620657363087272826405728397097156430302803390430967795990566529008327580855490767546402749072665194220717361987288652434043230118133015284255320323024817523093630725200440079100405324035605804009438100465462504715262098172589784914779385741563861382625162157073017180076587357599978202003173970214825274016726562762578090668058479436519868426110915802239099296929374633047255948460062228621332525702506821364563730963787120114605366135152175711490268964630207640579095473578185428578215057924412471402220554482962840590512815042730986135602168423814997945099011819316776098273585262653288216397049276571980042739745231681937430313918082476906010004527140398623085412184823665517584156875148293346170885081666385416289017899279101377458513676212410049570257439067784854445734840080854696756808146902643978462995498013289551853683855714560328927052050275659752107369542649765568595082080914905128289708864554403497715113926123316287613998301238974934649794122429800837181552269898302519393491934346996957786625556761638498142495828101850510792190379416466499290634188651311169756199066572801578219163058533134082952454515732373188841420575956448198105647391265228194144016181885332896026875525852411377270117777663413205480719030413993864249589332402267406675372852515.4054242066662535555492601570666105507278060866432225950694425982170493143398777466034362919075670614126506618687490625691390118469718666839154256634198186208818704296806349063122982826340146901853103503257106143117209145631607413110283732744711366178056957216661675112747172693931712255622953889726138904013052132446291515265280102080629448424073952951844910691817178747788662860441195732826092079255196799215331987997511396239829412025786572049257413122252212023014917220345736816993386758128351526769121759638553888519227010443449987668646396658021166906266686596127217407096935567560289829597199176870710500910064767119754315001224800915982378798845758059252793936864014303078196693341222984890683226455100228568701998432882845458690534943263522322971565940335832482795887249386481518443943805151974914563038918996183248058194857217206984347166715126551573275357693822353007145371591401324924214076576858602880228898819871395327147781084995008024348554068373812897756051856572086181949281331444754260008117560606431629627067284369171744065410867560868575724543195342790816821228703453755773820136830035746995161134434010166597646097973886592754194648498879045478328192111209323654448919700496035454213853674908652371262208828324542852843975726199182606583271695281032485186988592161403471341950685644613274215271008451137833423197197375631430493350238179381012249208255914438481081995770935738106105560627099257896442066878760950020918688990195999248262901086608730364405009111286086418563342879452015852514046330942457610045682467820609180075684242817881317717957891766819102982212833212635771792696390619273290508438870319304128230881153345353811498691516205182511373359903141356850322138736983010162828714306097382898508270541329909824714803293005374634064973985297818821859440515728099541484246508265283873409951551902835456644053396205730197002142722261487208657395939949585948984513813707160027770803510528663605323712005195485571832524118463090459719413543370866115576781047425875298448891993234770271913371124258102314228901119809622573858278024496703261060091237220814956752363444500365770901475954847997637495871506289215818274715542829029912115054302021541999416074732290289471693553520647223936216088094131828566432092434679033492419600262999195920095508449367603681607000685085662502278255859028183084551863542761086488753195528780104288699609375245307904402802451656665130012670092701190818449998116914930787548705734190858300019608471378104966949330182460131980297316266018496138649144757019373660901644037528073256529100253421992747824919410212139099865006962527319512240880548477979917899553149224144134612288063613147114907541147284764135415422740298536690964073215747433533117477481580653668664555375947246311445142494873645391285844954324669732319862584836454918306974857139107800995320719817981201404365686873484397118632535152653899089852805100057870036572565866665512541681593739512020410888083919102586880576443758815312379809222287396115786441618991929892769730508282630688735021808606963013032534078667930885893782008153983582355754881643716688729341766115803957289542404615958133064557147692951209354082208311924541602032867986127698501730855423027756786937965699242090841555827424874688888877121995080532169642034047723425520013390420386014172661892100217125354422600925140649058583723603533949094921674340338171585999523691400801291256136443957535054060023729358109184815727982688875261084693130457401068610431996495154451621233200971766328387619972925005967329386281069895094220471453208469392200462993219028273625571013711661789482506131618798210699870516061415584767123359737702747590764516376330610679001169893766616461557786499105478877203111310166344030607059041979236679685463645041780506373277653375799764566696353287324115551228854172526362039977459883015286758011633719783546263206129382812525551521862186609323408926166791479509520006921564128992901941374259070713097311248732382752929179160690007908804800013032433248286466481831112293689342687358714695179860972100860487356589102216123869166130108860948283519997996101412089888934908870014819581653021652034057562051745306919410095518009055868517807034542374963610579859592422995144723861855725998250681044641642162356936398375190017541660209753762521664175008862585565302658773685194856356191429862231300676381097317075279769852863410303369441362085124804467233244102660926852139386750909272341502974338095198915053082693971802353266763587256757704860475071464362410167225505852250650372516967607365393812475292035558173183519004166946850402603688111428043662006607061343197035715745282546675253663317881409003873803242841697696752585729523810455644346274811278755338195153078948336558432439633161459936477165295218182528007791352682258563299570740213094758202400088112113343362406756958412837954499820384747116275224904230109225851649746458147742978650649395323238063665132167101087656134568514337905591090820131453759078113128440289514776309748617639264677328745317581683813040947444726141305250245039350791401615712728202978543534329649605835180645993073326450630504059910375605618303256191463518869382978097937925122687629544407916945672195350166274795497595792755897230128031846607064815925310778904625212122245206435226251759035614393035740714259658151115457304724334172547368436075813705734895983654730338963162134932579062508591075245182797927829692844508535644113230317794507312522806578546418899185778738171158851904902717881035105257427322016197511929352807017324815104800859616525752591583942583219544676817911083464539479959796155134513155180086123748116292615643802839380583768538712899718817993363009792";

static void
test_arith_exact_fail(void)
{
    mpd_context_t ctx;
    mpd_uint_t adata[MPD_MINALLOC_MAX];
    mpd_uint_t bdata[MPD_MINALLOC_MAX];
    mpd_uint_t cdata[MPD_MINALLOC_MAX];
    mpd_uint_t rdata[MPD_MINALLOC_MAX];
    mpd_t a = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, adata};
    mpd_t b = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, bdata};
    mpd_t c = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, cdata};
    mpd_t result = {MPD_STATIC|MPD_STATIC_DATA, 0, 0, 0, MPD_MINALLOC_MAX, rdata};
    mpd_t *aa, *bb, *cc, *ret;
    char *calc;
    int n;

    mpd_testcontext(&ctx);
    ctx.traps = 0;

    aa = mpd_qnew();
    bb = mpd_qnew();
    cc = mpd_qnew();
    ret = mpd_qnew();

    /* powmod */
    mpd_set_string(&a, "999", &ctx);
    mpd_set_string(&b, "8477", &ctx);
    mpd_set_string(&c, "101", &ctx);

    for (n = 1; n < INT_MAX; n++) {
        ctx.status = 0;

        mpd_set_func_fail(n);
        mpd_qpowmod(&result, &a, &b, &c, &ctx, &ctx.status);
        mpd_set_func();

        if (!(ctx.status&MPD_Invalid_operation)) {
            break;
        }
        ASSERT(mpd_isnan(&result))
    }
    calc = mpd_to_sci(&result, 0);
    ASSERT(strcmp(calc, "99") == 0)
    ASSERT(ctx.status == 0)
    mpd_free(calc);

    ctx.prec = 999999;
    mpd_set_string(aa, three_pow_9999, &ctx);
    mpd_set_string(bb, small, &ctx);
    mpd_set_string(cc, five_pow_9999, &ctx);

    for (n = 1; n < INT_MAX; n++) {
        ctx.status = 0;

        mpd_set_func_fail(n);
        mpd_qpowmod(ret, aa, bb, cc, &ctx, &ctx.status);
        mpd_set_func();

        if (!(ctx.status&MPD_Invalid_operation)) {
            break;
        }
        ASSERT(mpd_isnan(ret))
    }
    calc = mpd_to_sci(ret, 0);
    ASSERT(strcmp(calc, powmod_result) == 0)
    ASSERT(ctx.status == 0)
    mpd_free(calc);

    /* ndivmod */
    ctx.prec = 300;
    mpd_set_string(&a, "2771", &ctx);
    mpd_set_string(&b, "882", &ctx);

    for (n = 1; n < INT_MAX; n++) {
        ctx.status = 0;

        mpd_set_func_fail(n);
        mpd_test_newtondiv(&result, &a, &b, &ctx);
        mpd_set_func();

        if (!(ctx.status&MPD_Invalid_operation)) {
            break;
        }
        ASSERT(mpd_isnan(&result))
    }
    calc = mpd_to_sci(&result, 0);
    ASSERT(strcmp(calc, ndivmod_result_1) == 0)
    ASSERT(ctx.status == (MPD_Inexact|MPD_Rounded))
    mpd_free(calc);

    ctx.prec = 8000;
    mpd_set_string(aa, three_pow_9999, &ctx);
    mpd_set_string(bb, small, &ctx);

    for (n = 1; n < INT_MAX; n++) {
        ctx.status = 0;

        mpd_set_func_fail(n);
        mpd_test_newtondiv(ret, aa, bb, &ctx);
        mpd_set_func();

        if (!(ctx.status&MPD_Invalid_operation)) {
            break;
        }
        ASSERT(mpd_isnan(ret))
    }
    calc = mpd_to_sci(ret, 0);
    ASSERT(strcmp(calc, ndivmod_result_2) == 0)
    ASSERT(ctx.status == (MPD_Inexact|MPD_Rounded))
    mpd_free(calc);

    /* sqrt */
    ctx.prec = 28;
    mpd_set_string(&a, "99812", &ctx);

    for (n = 1; n < INT_MAX; n++) {
        ctx.status = 0;

        mpd_set_func_fail(n);
        mpd_qsqrt(&result, &a, &ctx, &ctx.status);
        mpd_set_func();

        if (!(ctx.status&MPD_Invalid_operation)) {
            break;
        }
        ASSERT(mpd_isnan(&result))
    }
    calc = mpd_to_sci(&result, 0);
    ASSERT(strcmp(calc, "315.9303720758737054793883204") == 0)
    ASSERT(ctx.status == (MPD_Inexact|MPD_Rounded))
    mpd_free(calc);

    ctx.prec = 8000;
    mpd_set_string(aa, three_pow_9999, &ctx);

    for (n = 1; n < INT_MAX; n++) {
        ctx.status = 0;

        mpd_set_func_fail(n);
        mpd_qsqrt(ret, aa, &ctx, &ctx.status);
        mpd_set_func();

        if (!(ctx.status&MPD_Invalid_operation)) {
            break;
        }
        ASSERT(mpd_isnan(ret))
    }
    calc = mpd_to_sci(ret, 0);
    ASSERT(strcmp(calc, sqrt_result) == 0)
    ASSERT(ctx.status == (MPD_Inexact|MPD_Rounded))
    mpd_free(calc);


    mpd_del(&a);
    mpd_del(&b);
    mpd_del(&c);
    mpd_del(&result);

    mpd_del(aa);
    mpd_del(bb);
    mpd_del(cc);
    mpd_del(ret);
}

int
main(void)
{
    mpd_context_t ctx;

    srand((unsigned int)time(NULL));
    fprintf(stderr, "Running cov ... \n\n");

    mpd_traphandler = mpd_custom_traphandler;

    /* context.c */
    test_context(); /* Run this first! */

    /* mpdecimal.c */
    test_check_nans();
    test_finalize_round();
    test_attributes();

    test_int_conversions();
    mpd_testcontext(&ctx);
    test_mixed_arith(&ctx);

    ctx.traps = 0;
    for (ctx.prec = 1; ctx.prec < 23; ctx.prec++) {
        for (ctx.emax = 1; ctx.emax < 23; ctx.emax++) {
            if (ctx.emax < ctx.prec) continue;
            ctx.emin = -ctx.emax;
            //test_mixed_arith(&ctx);
        }
    }

    test_shift();
    test_baseconv();
    test_scaleb();
    test_misc();

    /* memory.c */
    test_memory();

    /* io.c */
    test_set_string();
    test_output();

    test_resize();
    test_resize_zero();
    test_arith_exact_fail();

    test_triple();

    fprintf(stderr, "\ncov: PASS\n\n");
    return 0;
}


