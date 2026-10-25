#include <stdio.h>
#include <stdlib.h>
#include <float.h>
#ifdef _MSC_VER
  #pragma fenv_access (on)
#else
  #include <fenv.h>
  #pragma STDC FENV_ACCESS ON
#endif


#ifndef _MSC_VER
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
#else
static unsigned short __cdecl
_mpd_get_control87(void)
{
    unsigned short cw;

    __asm {
        fnstcw cw
    }

    return cw;
}
#endif

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

unsigned int
prev_mpd_set_fenv(void)
{
    unsigned int cw;
#ifdef _MSC_VER
    cw = _control87(0, 0);
    _control87((_RC_CHOP|_PC_64), (_MCW_RC|_MCW_PC));
#else
    cw = _mpd_get_control87();
    _mpd_set_control87(cw|0x780);
#endif
    return cw;
}

void
prev_mpd_restore_fenv(unsigned int cw)
{
#ifdef _MSC_VER
    _control87(cw, (_MCW_RC|_MCW_PC));
#else
    _mpd_set_control87((unsigned short)cw);
#endif
}


/*
 *  x87 floating point control word:
 *  ================================
 *
 *  ---- ---- --XX XXXX = MCW_EM - exception masks (1=handle exception
 *                                                  internally, 0=fault)
 *  ---- ---- ---- ---X = EM_INVALID - invalid operation
 *  ---- ---- ---- --X- = EM_DENORMAL - denormal operand
 *  ---- ---- ---- -X-- = EM_ZERODIVIDE - divide by zero
 *  ---- ---- ---- X--- = EM_OVERFLOW - overflow
 *  ---- ---- ---X ---- = EM_UNDERFLOW - underflow
 *  ---- ---- --X- ---- = EM_INEXACT - rounding was required
 *  ---- --XX ---- ---- = MCW_PC - precision control
 *  ---- --00 ---- ---- = PC_24 - single precision
 *  ---- --10 ---- ---- = PC_53 - double precision
 *  ---- --11 ---- ---- = PC_64 - extended precision
 *  ---- XX-- ---- ---- = MCW_RC - rounding control
 *  ---- 00-- ---- ---- = RC_NEAR - round to nearest
 *  ---- 01-- ---- ---- = RC_DOWN - round towards -Inf
 *  ---- 10-- ---- ---- = RC_UP - round towards +Inf
 *  ---- 11-- ---- ---- = RC_CHOP - round towards zero
 *  ---X ---- ---- ---- = MCW_IC - infinity control (obsolete,
 *                                                always affine)
 *  ---0 ---- ---- ---- = IC_AFFINE - -Inf < +Inf
 *  ---1 ---- ---- ---- = IC_PROJECTIVE - -Inf == +Inf
 *
 *       0011 0111 1111 -> default in gcc, icc
 *       0010 0111 1111 -> default in Visual Studio
 *
 *  The seventh bit is reserved in an Intel manual but used for
 *  _DN_SAVE|_DN_FLUSH in Visual Studio and apparently also in gcc.
 *  _mpd_fntmul() saves the existing control word and sets these bits
 *  unconditionally:
 *
 *       1111 0011 1111 -> RC_CHOP|PC_64 + all exceptions masked
 *
 *  This means that _DN_SAVE|_DN_FLUSH is left untouched. Starting from
 *  the default, all tested compilers have these settings in _mpd_fntmul():
 *
 *       1111 0111 1111 -> RC_CHOP|PC_64|_DN_FLUSH + all exceptions masked.
 *
 *
 *  The infinity control has no effect on x87 processors.
 */

int main( void )
{
    unsigned int cw;

    printf("unconditionally set in gcc: 0x%X\n", 0xF3F);

    printf("default controlword: 0x%X\n", _mpd_get_control87());

    cw = mpd_set_fenv();
    printf("new controlword setting: 0x%X\n", _mpd_get_control87());

    mpd_restore_fenv(cw);
    printf("restored controlword setting: 0x%X\n", _mpd_get_control87());

    cw = prev_mpd_set_fenv();
    printf("prev controlword setting: 0x%X\n", _mpd_get_control87());

    prev_mpd_restore_fenv(cw);
    printf("prev restored controlword setting: 0x%X\n", _mpd_get_control87());

    return 0;
}
