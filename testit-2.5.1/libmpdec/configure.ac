AC_PREREQ([2.67])
AC_INIT(mpdecimal, 2.5.1, mpdecimal-bugs@bytereef.org, mpdecimal, http://www.bytereef.org/mpdecimal/index.html)
AC_CONFIG_HEADER(config.h)
AC_CONFIG_FILES([Makefile tests/Makefile mpdecimal.h])
AC_ARG_VAR(MACHINE, [force configuration: x64, uint128, ansi64, ppro, ansi32, ansi-legacy])

LIBSTATIC=libmpdec.a
case $ac_sys_system in
    darwin*|Darwin*)
        LIBNAME="libmpdec.dylib"
        LIBSONAME="libmpdec.2.dylib"
        LIBSHARED="libmpdec.2.5.1.dylib"
        CONFIGURE_LDFLAGS="-dynamiclib -install_name @rpath/$LIBSONAME -compatibility_version 2.5 -current_version 2.5.1"
        ;;
    *)
        LIBNAME="libmpdec.so"
        LIBSONAME="libmpdec.so.2"
        LIBSHARED="libmpdec.so.2.5.1"
        CONFIGURE_LDFLAGS="-shared -Wl,-soname,$LIBSONAME"
        ;;
esac

AC_SUBST(LIBSTATIC)
AC_SUBST(LIBNAME)
AC_SUBST(LIBSONAME)
AC_SUBST(LIBSHARED)


LIBSTATIC_CXX=libmpdec++.a
case $ac_sys_system in
    darwin*|Darwin*)
        LIBNAME_CXX="libmpdec++.dylib"
        LIBSONAME_CXX="libmpdec++.2.dylib"
        LIBSHARED_CXX="libmpdec++.2.5.1.dylib"
        CONFIGURE_LDXXFLAGS="-dynamiclib -install_name @rpath/$LIBSONAME_CXX -compatibility_version 2.5 -current_version 2.5.1"
        ;;
    *)
        LIBNAME_CXX="libmpdec++.so"
        LIBSONAME_CXX="libmpdec++.so.2"
        LIBSHARED_CXX="libmpdec++.so.2.5.1"
        CONFIGURE_LDXXFLAGS="-shared -Wl,-soname,$LIBSONAME_CXX"
        ;;
esac

AC_SUBST(LIBSTATIC_CXX)
AC_SUBST(LIBNAME_CXX)
AC_SUBST(LIBSONAME_CXX)
AC_SUBST(LIBSHARED_CXX)


# Apparently purely informational for this particular build:
AC_CANONICAL_HOST
AC_SUBST(build)
AC_SUBST(host)


# Language and compiler:
AC_LANG_C
saved_cflags="$CFLAGS"
saved_cxxflags="$CXXFLAGS"
AC_PROG_CC
AC_PROG_CXX
CFLAGS="$saved_cflags"
CXXFLAGS="$saved_cxxflags"

# ar and ranlib:
AC_CHECK_TOOL(AR, ar, ar)
AC_PROG_RANLIB
AC_SUBST(RANLIB)

# System and machine type (only used for Solaris, Darwin and AIX):
AC_MSG_CHECKING(system as reported by uname -s)
ac_sys_system=`uname -s`
AC_MSG_RESULT($ac_sys_system)

# Checks for header files:
AC_HEADER_STDC
AC_CHECK_HEADERS(inttypes.h stdint.h)

# Type availability checks:
AC_TYPE_SIZE_T
AC_TYPE_INT32_T
AC_TYPE_INT64_T
AC_TYPE_UINT32_T
AC_TYPE_UINT64_T
AC_CHECK_TYPE(__uint128_t, AC_DEFINE(HAVE_UINT128_T, 1,
              [Define if your compiler provides __uint128_t.]),,)

# Sizes of various types:
AC_CHECK_SIZEOF(size_t, 4)
AC_CHECK_SIZEOF(__uint128_t, 16)

# x64 with gcc asm:
AC_MSG_CHECKING(for x64 gcc inline assembler)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([[]], [[
  __asm__ __volatile__ ("movq %rcx, %rax");
]])],[have_gcc_asm_for_x64=yes],[have_gcc_asm_for_x64=no])
AC_MSG_RESULT($have_gcc_asm_for_x64)
if test "$have_gcc_asm_for_x64" = yes; then
    AC_DEFINE(HAVE_GCC_ASM_FOR_X64, 1,
    [Define if we can use x64 gcc inline assembler.])
fi

# x87 with gcc asm:
AC_MSG_CHECKING(for x87 gcc inline assembler)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([[]], [[
  unsigned short cw;
  __asm__ __volatile__ ("fnstcw %0" : "=m" (cw));
  __asm__ __volatile__ ("fldcw %0" : : "m" (cw));
]])],[have_gcc_asm_for_x87=yes],[have_gcc_asm_for_x87=no])
AC_MSG_RESULT($have_gcc_asm_for_x87)
if test "$have_gcc_asm_for_x87" = yes; then
    AC_DEFINE(HAVE_GCC_ASM_FOR_X87, 1,
    [Define if we can use x87 gcc inline assembler.])
fi

# gmp for extended tests:
GMPDEPS=
GMPPATH=
saved_cflags="$CFLAGS"
saved_ldflags="$LDFLAGS"
saved_libs="$LIBS"
LIBS=-lgmp
AC_MSG_CHECKING(for gmp)
AC_LINK_IFELSE([AC_LANG_PROGRAM([[
#include <gmp.h>
]], [[
   mpz_t x;
   mpz_init(x);
]])], [have_gmp=yes], [have_gmp=no])
if test $have_gmp = no; then
    CFLAGS="$CFLAGS -I/usr/local/include"
    LDFLAGS="$LDFLAGS -L/usr/local/lib"
    AC_LINK_IFELSE([AC_LANG_PROGRAM([[
    #include <gmp.h>
    ]], [[
       mpz_t x;
       mpz_init(x);
    ]])], [have_gmp=yes], [have_gmp=no])
    if test $have_gmp = yes; then
        GMPPATH=/usr/local
    fi
fi
AC_MSG_RESULT($have_gmp)
CFLAGS="$saved_cflags"
LDFLAGS="$saved_ldflags"
LIBS="$saved_libs"

if test $have_gmp = yes; then
    GMPDEPS="mpd_mpz_add mpd_mpz_divmod mpd_mpz_mul mpd_mpz_sub"
fi
AC_SUBST(GMPDEPS)
AC_SUBST(GMPPATH)

# Availability of -O2:
AC_MSG_CHECKING(for -O2)
saved_cflags="$CFLAGS"
CFLAGS="-O2"
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([[]], [[
]])],[have_O2=yes],[have_O2=no])
AC_MSG_RESULT($have_O2)
CFLAGS="$saved_cflags"

# Install program:
AC_PROG_INSTALL
AC_SUBST(INSTALL)

# _FORTIFY_SOURCE wrappers for memmove and bcopy are incorrect:
# http://sourceware.org/ml/libc-alpha/2010-12/msg00009.html
AC_MSG_CHECKING(for glibc _FORTIFY_SOURCE/memmove bug)
saved_cflags="$CFLAGS"
CFLAGS="-O2 -D_FORTIFY_SOURCE=2"
if test "$have_O2" = no; then
    CFLAGS=""
fi
AC_RUN_IFELSE([AC_LANG_SOURCE([[
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
void foo(void *p, void *q) { memmove(p, q, 19); }
int main() {
  char a[32] = "123456789000000000";
  foo(&a[9], a);
  if (strcmp(a, "123456789123456789000000000") != 0)
    return 1;
  foo(a, &a[9]);
  if (strcmp(a, "123456789000000000") != 0)
    return 1;
  return 0;
}
]])],
[have_glibc_memmove_bug=no],
[have_glibc_memmove_bug=yes],
[have_glibc_memmove_bug=undefined])
CFLAGS="$saved_cflags"
AC_MSG_RESULT($have_glibc_memmove_bug)
if test "$have_glibc_memmove_bug" = yes; then
    AC_DEFINE(HAVE_GLIBC_MEMMOVE_BUG, 1,
    [Define if glibc has incorrect _FORTIFY_SOURCE wrappers
     for memmove and bcopy.])
fi

# Solaris fixes:
MPD_GNU99=
case $ac_sys_system in
sun*|Sun*)
    case $CC in
        *gcc*)
            MPD_GNU99="-std=gnu99"
            ;;
        cc)
            CC=suncc
            ;;
    esac
esac

# Compiler dependent settings:
MPD_WARN=
MPD_WARNXX=
MPD_OPT="-O2"
MPD_PGEN=
MPD_PUSE=
MPD_CCOV=
MPD_LDCOV=
case $CC in
    *gcc*)
        MPD_WARN="-Wall -W -Wno-unknown-pragmas -Wno-overlength-strings -std=c99 -pedantic"
        MPD_WARNXX="-Wall -Wextra -std=c++11 -pedantic"
        MPD_OPT="-O2"
        MPD_PGEN="-fprofile-generate -fprofile-values"
        MPD_PUSE="-fprofile-use -freorder-blocks"
        MPD_CCOV="-O0 -g -fno-inline -fprofile-arcs -ftest-coverage"
        MPD_LDCOV="-fprofile-arcs"
        ;;
    *icc*)
        AR=xiar
        LD=icc
        CXX=icc
        LDXX=icc
        MPD_WARN="-Wall -Wextra -Wno-unknown-pragmas -std=c99 -pedantic -diag-disable=11074,11076"
        MPD_WARNXX="-Wall -Wextra -std=c++11 -pedantic -diag-disable=11074,11076"
        MPD_OPT="-O2"
        MPD_PGEN="-wd11505 -prof-gen"
        MPD_PUSE="-wd11505 -prof-use"
        ;;
    *clang*)
        MPD_WARN="-Wall -Wextra -Wno-unknown-pragmas -Wno-overlength-strings -std=c99 -pedantic"
        MPD_WARNXX="-Wall -Wextra -Wexit-time-destructors -std=c++11 -pedantic"
        MPD_OPT="-O2"
        ;;
    *xlc*)
        LD=xlc
        CXX=xlC
        LDXX=xlC
        MPD_WARN="-qlanglvl=stdc99"
        MPD_OPT="-O2 -qalias=ansi -qmaxmem=-1"
        PROFILE=
        ;;
    *suncc*)
        MPD_OPT="-O2"
        ;;
    *ccomp*)
        MPD_CONFIG="-DCONFIG_32 -DANSI -DLEGACY_COMPILER"
        MPD_OPT="-fstruct-passing -fstruct-assign"
        LD="gcc"
        LDFLAGS="-m32"
        ;;
esac

# Values for the MPD_HEADER_CONFIG variable:
CONFIG_64="
/* ABI: 64-bit */
#define MPD_CONFIG_64 1

#ifdef MPD_CONFIG_32
  #error \"cannot use MPD_CONFIG_32 with 64-bit header.\"
#endif

#ifdef CONFIG_32
  #error \"cannot use CONFIG_32 with 64-bit header.\"
#endif"

CONFIG_32="
/* ABI: 32-bit */
#define MPD_CONFIG_32 1

#ifdef MPD_CONFIG_64
  #error \"cannot use MPD_CONFIG_64 with 32-bit header.\"
#endif

#ifdef CONFIG_64
  #error \"cannot use CONFIG_64 with 32-bit header.\"
#endif"

CONFIG_32_LEGACY="
/* ABI: 32-bit */
#define MPD_CONFIG_32 1

/* libmpdec was compiled without support for int64_t */
#define MPD_LEGACY_COMPILER 1

#ifdef MPD_CONFIG_64
  #error \"cannot use MPD_CONFIG_64 with 32-bit header.\"
#endif

#ifdef CONFIG_64
  #error \"cannot use CONFIG_64 with 32-bit header.\"
#endif"

CONFIG_UNIVERSAL="
/* Mac OS X: support for building universal binaries */
#if defined(MPD_CONFIG_64) || defined(MPD_CONFIG_32)
  #error \"cannot use MPD_CONFIG_64 or MPD_CONFIG_32 with universal header.\"
#endif

#if defined(CONFIG_64) || defined(CONFIG_32)
  #error \"cannot use CONFIG_64 or CONFIG_32 with universal header.\"
#endif

#if defined(__ppc__)
  #define MPD_CONFIG_32 1
  #ifdef UNIVERSAL
    #define CONFIG_32
    #define ANSI
  #endif
#elif defined(__ppc64__)
  #define MPD_CONFIG_64 1
  #ifdef UNIVERSAL
    #define CONFIG_64
    #define ANSI
  #endif
#elif defined(__i386__)
  #define MPD_CONFIG_32 1
  #ifdef UNIVERSAL
    #define CONFIG_32
    #define ANSI
  #endif
#elif defined(__x86_64__)
  #define MPD_CONFIG_64 1
  #ifdef UNIVERSAL
    #define CONFIG_64
    #define ASM
  #endif
#elif defined(__arm64__)
  #define MPD_CONFIG_64 1
  #ifdef UNIVERSAL
    #define CONFIG_64
    #define ANSI
  #endif
#else
  #error \"unknown architecture for universal build.\"
#endif"

CONFIG_FULLCOV="
/* DO NOT USE IN PRODUCTION! */
#define MPD_CONFIG_32

#if defined(MPD_CONFIG_64)
  #error \"cannot use MPD_CONFIG_64 with full_coverage.\"
#endif

#if defined(CONFIG_64)
  #error \"cannot use CONFIG_64 with full_coverage.\"
#endif"

# Auto-detect machine dependent settings:
M64=
M32=
if test -n "$MACHINE"; then
    M64="-m64 "
    M32="-m32 "
    case "$MACHINE" in
        x64|uint128|ansi64|full_coverage|ppro|ansi32|ansi-legacy|universal)
            : ;;
        *)
            echo "configure: error: invalid MACHINE variable: $MACHINE"
            exit 1 ;;
    esac
elif test $ac_cv_sizeof_size_t -eq 8; then
    if test $have_gcc_asm_for_x64 = yes; then
        MACHINE="x64"
    elif test $ac_cv_type___uint128_t = yes; then
        MACHINE="uint128"
    else
        MACHINE="ansi64"
    fi
else
    # Macros for detecting uint64_t and int64_t are unreliable:
    case $CC in
        *ccomp*)
            MACHINE="ansi-legacy"
            ;;
        *)
            MACHINE="ansi32"
            ;;
    esac
    if test $have_gcc_asm_for_x87 = yes; then
        case $CC in
            *gcc*|*clang*) # icc >= 11.0 works as well
                case $ac_sys_system in
                    darwin*|Darwin*)
                    ;;
                *)
                    MACHINE="ppro"
                    ;;
                esac
            ;;
        esac
    fi
fi

# Set configuration variables:
MPD_PREC=9
MPD_DPREC=18
case "$MACHINE" in
    x64)
        MPD_HEADER_CONFIG=$CONFIG_64
        MPD_CONFIG=$M64"-DCONFIG_64 -DASM"
        CONFIGURE_LDFLAGS="$M64 $CONFIGURE_LDFLAGS"
        MPD_PREC=19
        MPD_DPREC=38
        ;;
    uint128)
        MPD_HEADER_CONFIG=$CONFIG_64
        MPD_CONFIG=$M64"-DCONFIG_64 -DANSI -DHAVE_UINT128_T"
        CONFIGURE_LDFLAGS="$M64 $CONFIGURE_LDFLAGS"
        MPD_PREC=19
        MPD_DPREC=38
        ;;
    ansi64)
        MPD_HEADER_CONFIG=$CONFIG_64
        MPD_CONFIG=$M64"-DCONFIG_64 -DANSI"
        CONFIGURE_LDFLAGS="$M64 $CONFIGURE_LDFLAGS"
        MPD_PREC=19
        MPD_DPREC=38
        ;;
    full_coverage)
        # Formerly ansi64c32, for testing only!
        MPD_HEADER_CONFIG=$CONFIG_FULLCOV
        MPD_CONFIG=$M64"-DCONFIG_32 -DTEST_COVERAGE -DANSI"
        CONFIGURE_LDFLAGS="$M64 $CONFIGURE_LDFLAGS"
        ;;
    ppro)
        MPD_HEADER_CONFIG=$CONFIG_32
        MPD_CONFIG=$M32"-DCONFIG_32 -DPPRO -DASM"
        CONFIGURE_LDFLAGS="$M32 $CONFIGURE_LDFLAGS"
        # Some versions of gcc miscompile inline asm:
        # http://gcc.gnu.org/bugzilla/show_bug.cgi?id=46491
        # http://gcc.gnu.org/ml/gcc/2010-11/msg00366.html
        case $CC in
            *gcc*)
                AC_MSG_CHECKING(for gcc ipa-pure-const bug)
                saved_cflags="$CFLAGS"
                CFLAGS="-O2"
                AC_RUN_IFELSE([AC_LANG_SOURCE([[
                __attribute__((noinline)) int
                foo(int *p) {
                  int r;
                  asm ( "movl \$6, (%1)\n\t"
                        "xorl %0, %0\n\t"
                        : "=r" (r) : "r" (p) : "memory"
                  );
                  return r;
                }
                int main() {
                  int p = 8;
                  if ((foo(&p) ? : p) != 6)
                    return 1;
                  return 0;
                }
                ]])],
                [have_ipa_pure_const_bug=no],
                [have_ipa_pure_const_bug=yes],
                [have_ipa_pure_const_bug=undefined])
                CFLAGS="$saved_cflags"
                AC_MSG_RESULT($have_ipa_pure_const_bug)
                if test "$have_ipa_pure_const_bug" = yes; then
                    MPD_CONFIG="$MPD_CONFIG -fno-ipa-pure-const"
                    AC_DEFINE(HAVE_IPA_PURE_CONST_BUG, 1,
                [Define if gcc has the ipa-pure-const bug.])
                fi
            ;;
        esac
        ;;
    ansi32)
        MPD_HEADER_CONFIG=$CONFIG_32
        MPD_CONFIG=$M32"-DCONFIG_32 -DANSI"
        CONFIGURE_LDFLAGS="$M32 $CONFIGURE_LDFLAGS"
        ;;
    ansi-legacy)
        MPD_HEADER_CONFIG=$CONFIG_32_LEGACY
        MPD_CONFIG=$M32"-DCONFIG_32 -DANSI -DLEGACY_COMPILER"
        CONFIGURE_LDFLAGS="$M32 $CONFIGURE_LDFLAGS"
        ;;
    universal)
        MPD_HEADER_CONFIG=$CONFIG_UNIVERSAL
        MPD_CONFIG="-DUNIVERSAL"
esac

# Set maximum heap size on 32-bit AIX:
MPD_MAXDATA=
case $ac_sys_system in
AIX)
    case $MACHINE in
    ansi32|ansi-legacy)
        case $CC in
        *gcc*)
            MPD_MAXDATA="-Wl,-bmaxdata:0x80000000"
            ;;
        *xlc*)
            MPD_MAXDATA="-bmaxdata:0x80000000"
            ;;
        esac
    ;;
    esac
;;
esac

# Substitute variables and generate output:
if test -z "$LD"; then
    LD="$CC"
fi
if test -z "$LDXX"; then
    LDXX="$CXX"
fi
AC_SUBST(LD)
AC_SUBST(LDXX)
AC_SUBST(AR)
AC_SUBST(MPD_HEADER)
AC_SUBST(MPD_WARN)
AC_SUBST(MPD_CONFIG)
AC_SUBST(MPD_HEADER_CONFIG)
AC_SUBST(MPD_OPT)
AC_SUBST(MPD_GNU99)
AC_SUBST(MPD_MAXDATA)

AC_SUBST(MPD_CCOV)
AC_SUBST(MPD_LDCOV)
AC_SUBST(MPD_PGEN)
AC_SUBST(MPD_PUSE)
AC_SUBST(MPD_PREC)
AC_SUBST(MPD_DPREC)

if test -z "$CFLAGS"; then
    CONFIGURE_CFLAGS="$MPD_WARN $MPD_CONFIG $MPD_OPT"
else
    CONFIGURE_CFLAGS="$MPD_WARN $MPD_CONFIG $MPD_OPT $CFLAGS"
fi

if test "$have_glibc_memmove_bug" = yes; then
    CONFIGURE_CFLAGS="$CONFIGURE_CFLAGS -U_FORTIFY_SOURCE"
fi

if test -z "$CXXFLAGS"; then
    CONFIGURE_CXXFLAGS="$MPD_WARNXX -O3 -DNDEBUG"
else
    CONFIGURE_CXXFLAGS="$MPD_WARNXX -O3 -DNDEBUG $CXXFLAGS"
fi

if test -n "$LDFLAGS"; then
    CONFIGURE_LDFLAGS="$CONFIGURE_LDFLAGS $LDFLAGS"
fi

if test -n "$LDXXFLAGS"; then
    CONFIGURE_LDXXFLAGS="$CONFIGURE_LDXXFLAGS $LDXXFLAGS"
fi


AC_SUBST(CONFIGURE_CFLAGS)
AC_SUBST(CONFIGURE_CXXFLAGS)
AC_SUBST(CONFIGURE_LDFLAGS)
AC_SUBST(CONFIGURE_LDXXFLAGS)

AC_OUTPUT

GLIBC_MEMMOVE_BUG_WARN="
***************************** WARNING *********************************

Detected glibc _FORTIFY_SOURCE/memmove bug. See:

    http://sourceware.org/ml/libc-alpha/2010-12/msg00009.html

Enabling -U_FORTIFY_SOURCE workaround. If -D_FORTIFY_SOURCE is also
present in the command line, make sure that the order of the two
options is:

   ... -D_FORTIFY_SOURCE=2 ... -U_FORTIFY_SOURCE ...

A better solution is to upgrade glibc or to report the bug to your
OS vendor.

***************************** WARNING *********************************
"
if test "$have_glibc_memmove_bug" = yes; then
    echo "$GLIBC_MEMMOVE_BUG_WARN"
fi

