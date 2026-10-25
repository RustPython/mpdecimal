#include <stdio.h>
#include <stdint.h>
#include <math.h>
#include "mpdecimal.h"
#include "bits.h"
#include "constants.h"
#include "typearith.h"


unsigned short _Py_get_387controlword(void) {
    unsigned short cw;
    __asm__ __volatile__ ("fnstcw %0" : "=m" (cw));
    return cw;
}

void _Py_set_387controlword(unsigned short cw) {
    __asm__ __volatile__ ("fldcw %0" : : "m" (cw));
}


/* Internal function: Set a decimal from a triple, no error checking. */
static void
_settriple(mpd_t *result, uint8_t sign, mpd_uint_t a, mpd_ssize_t exp)
{
    mpd_minalloc(result);
    mpd_set_flags(result, sign);
    result->exp = exp;
    _mpd_div_word(&result->data[1], &result->data[0], a, MPD_RADIX);
    result->len = (result->data[1] == 0) ? 1 : 2;
    mpd_setdigits(result);
}


static void
_mpd_qpow_uint(mpd_t *result, const mpd_t *base, mpd_uint_t exp,
               uint8_t resultsign, const mpd_context_t *ctx, uint32_t *status)
{
    uint32_t workstatus = 0;
    mpd_uint_t n;

    if (exp == 0) {
        _settriple(result, resultsign, 1, 0); /* GCOV_NOT_REACHED */
        return; /* GCOV_NOT_REACHED */
    }

    if (!mpd_qcopy(result, base, status)) {
        return;
    }

    n = mpd_bits[mpd_bsr(exp)];
    while (n >>= 1) {
        mpd_qmul(result, result, result, ctx, &workstatus);
        if (exp & n) {
            mpd_qmul(result, result, base, ctx, &workstatus);
        }
        if (mpd_isspecial(result) ||
            (mpd_iszerocoeff(result) && (workstatus & MPD_Clamped))) {
            break;
        }
    }

    *status |= workstatus;
    mpd_set_sign(result, resultsign);
}

#if 0
/* Internal function: Set a decimal from a triple, no error checking. */
static void
_settriple(mpd_t *result, uint8_t sign, mpd_uint_t a, mpd_ssize_t exp)
{
    mpd_minalloc(result);
    mpd_set_flags(result, sign);
    result->exp = exp;
    _mpd_div_word(&result->data[1], &result->data[0], a, MPD_RADIX);
    result->len = (result->data[1] == 0) ? 1 : 2;
    mpd_setdigits(result);
}
#endif

/*
 * Returns the space needed to import a base 'base' integer of length 'srclen'.
 */
static mpd_ssize_t
_mpd_importsize(size_t srclen, uint32_t base)
{
    double x;

    assert(srclen > 0);
    assert(base >= 2);

#if SIZE_MAX == UINT64_MAX
    if (srclen > (1ULL<<53)) {
        return MPD_SSIZE_MAX;
    }
#endif

    x = (double)srclen * (log10(base)/MPD_RDIGITS);
    return (x >= MPD_MAXIMPORT) ? MPD_SSIZE_MAX : (mpd_ssize_t)x + 1;
}


int
main(void)
{
    mpd_context_t ctx;
    uint32_t status = 0;
    mpd_t *pow_10;
    mpd_t *base;
    mpd_t *pow_base;
    mpd_ssize_t adj, intbase = (1U<<15);
    size_t sizeinbase, importsize;
    unsigned short old_387controlword, new_387controlword;

    old_387controlword = _Py_get_387controlword();
    new_387controlword = (old_387controlword & ~0x0f00) | 0x0200;
    _Py_set_387controlword(new_387controlword);


    mpd_maxcontext(&ctx);

    pow_10 = mpd_qnew();
    base = mpd_qnew();
    pow_base = mpd_qnew();

    _settriple(base, MPD_POS, intbase, 0);

    for (adj = 1; adj < 425000000; adj++) {
        _settriple(pow_10, MPD_POS, 1, adj);

        sizeinbase = mpd_sizeinbase(pow_10, intbase);
        _mpd_qpow_uint(pow_base, base, sizeinbase, MPD_POS, &ctx, &status);

        if ((status & MPD_Invalid_operation) ||
            mpd_qcmp(pow_base, pow_10, &status) < 0) {
            printf("%zu\n", adj);
            mpd_print(pow_base);
            mpd_print(pow_10);
            abort();
        }

        if (adj % 1000 == 0) {
            printf("%zu\n", adj);
        }
    }

    return 0;
}
