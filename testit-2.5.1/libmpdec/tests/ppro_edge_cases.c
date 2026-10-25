#include <stdio.h>
#include <stdlib.h>
#include <float.h>
#ifdef _MSC_VER
  #pragma fenv_access (on)
#else
  #include <stdint.h>
  #include <fenv.h>
  #pragma STDC FENV_ACCESS ON
#endif



/* number-theory.c */
const uint32_t mpd_moduli[3]  = {2113929217UL, 2013265921UL, 1811939329UL};
const uint32_t mpd_roots[3]   = {5UL, 31UL, 13UL};

/* PentiumPro modular multiplication: These constants have to be loaded as
 * 80 bit long doubles, which are not supported by certain compilers. */
const uint32_t mpd_invmoduli[3][3] = {
    {4293885170U, 2181570688U, 16352U},  /* ((long double) 1 / 2113929217UL) */
    {1698898177U, 2290649223U, 16352U},  /* ((long double) 1 / 2013265921UL) */
    {2716021846U, 2545165803U, 16352U}   /* ((long double) 1 / 1811939329UL) */
};

const float MPD_TWO63 = 9223372036854775808.0; /* 2^63 */

static inline unsigned short
_mpd_get_control87(void)
{
    unsigned short cw;

    __asm__ __volatile__ ("fnstcw %0" : "=m" (cw));
    return cw;
}
 
static inline void
_mpd_set_control87(unsigned short cw)
{
    __asm__ __volatile__ ("fldcw %0" : : "m" (cw));
}

unsigned int
mpd_set_fenv(void)
{
    unsigned int cw;
#ifdef _MSC_VER
    unsigned int flags =
        _EM_INVALID|_EM_DENORMAL|_EM_ZERODIVIDE|_EM_OVERFLOW|
        _EM_UNDERFLOW|_EM_INEXACT|_RC_CHOP|_PC_64;
    unsigned int mask = _MCW_EM|_MCW_RC|_MCW_PC;
    unsigned int dummy;

    __control87_2(0, 0, &cw, NULL);
    __control87_2(flags, mask, &dummy, NULL);
#else
    cw = _mpd_get_control87();
    _mpd_set_control87(cw|0xF3F);
#endif
    return cw;
}

void
mpd_restore_fenv(unsigned int cw)
{
#ifdef _MSC_VER
    unsigned int mask = _MCW_EM|_MCW_RC|_MCW_PC;
    unsigned int dummy;

    __control87_2(cw, mask, &dummy, NULL);
#else
    _mpd_set_control87((unsigned short)cw);
#endif
}

/* (a * b + 1) % dmod */
static inline uint32_t
mulmod_pplus1(uint32_t a, uint32_t b, double *dmod, const uint32_t *dinvmod)
{
    uint32_t retval;
    uint32_t one = 1;

    asm (
        "fildl    %3\n\t"
        "fildl    %2\n\t"
        "fildl    %1\n\t"
        "fmulp    %%st, %%st(1)\n\t"
        "faddp    %%st, %%st(1)\n\t"
        "fldt    (%5)\n\t"
        "fmul    %%st(1), %%st\n\t"
        "flds    %6\n\t"
        "fadd    %%st, %%st(1)\n\t"
        "fsubrp    %%st, %%st(1)\n\t"
        "fldl    (%4)\n\t"
        "fmulp    %%st, %%st(1)\n\t"
        "fsubrp    %%st, %%st(1)\n\t"
        "fistpl    %0\n\t"
        : "=m" (retval)
        : "m" (a), "m" (b), "m" (one), "r" (dmod), "r" (dinvmod), "m" (MPD_TWO63)
        : "st", "memory"
    );

    return retval;
}

/* (a * b - 1) % dmod */
static inline uint32_t
mulmod_pminus1(uint32_t a, uint32_t b, double *dmod, const uint32_t *dinvmod)
{
    uint32_t retval;
    uint32_t one = 1;

    asm (
        "fildl    %3\n\t"
        "fildl    %2\n\t"
        "fildl    %1\n\t"
        "fmulp    %%st, %%st(1)\n\t"
        "fsubp    %%st, %%st(1)\n\t"
        "fldt    (%5)\n\t"
        "fmul    %%st(1), %%st\n\t"
        "flds    %6\n\t"
        "fadd    %%st, %%st(1)\n\t"
        "fsubrp    %%st, %%st(1)\n\t"
        "fldl    (%4)\n\t"
        "fmulp    %%st, %%st(1)\n\t"
        "fsubrp    %%st, %%st(1)\n\t"
        "fistpl    %0\n\t"
        : "=m" (retval)
        : "m" (a), "m" (b), "m" (one), "r" (dmod), "r" (dinvmod), "m" (MPD_TWO63)
        : "st", "memory"
    );

    return retval;
}

int
main(void)
{
    double dmod;
    const uint32_t *invmod;
    uint32_t x;

    (void)mpd_set_fenv();

        dmod = mpd_moduli[0];
    invmod = mpd_invmoduli[0];
    for (x = 0; x <= 2113929217; x++) {
        if (mulmod_pplus1(x,  2113929217, &dmod, invmod) != 1) {
            printf("%u\n", x);
        }
    }
    for (x = 1; x <= 2113929217; x++) {
        if (mulmod_pminus1(x,  2113929217, &dmod, invmod) != 2113929216) {
            printf("%u\n", x);
        }
    }

    dmod = mpd_moduli[1];
    invmod = mpd_invmoduli[1];
    for (x = 0; x <= 2013265921; x++) {
        if (mulmod_pplus1(x,  2013265921, &dmod, invmod) != 1) {
            printf("%u\n", x);
        }
    }
    for (x = 1; x <= 2013265921; x++) {
        if (mulmod_pminus1(x,  2013265921, &dmod, invmod) != 2013265920) {
            printf("%u\n", x);
        }
    }

    dmod = mpd_moduli[2];
    invmod = mpd_invmoduli[2];
    for (x = 0; x <= 1811939329; x++) {
        if (mulmod_pplus1(x,  1811939329, &dmod, invmod) != 1) {
            printf("%u\n", x);
        }
    }
    for (x = 1; x <= 1811939329; x++) {
        if (mulmod_pminus1(x,  1811939329, &dmod, invmod) != 1811939328) {
            printf("%u\n", x);
        }
    }

    return 0;
}
