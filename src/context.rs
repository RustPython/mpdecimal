use crate::common::*;
use crate::shim::mpd_c2rust_raise;
use crate::shim::mpd_c2rust_stderr;
use crate::mpalloc::MPD_MINALLOC;
extern "C" {
    fn fprintf(stream: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputc(c: ::core::ffi::c_int, stream: *mut FILE) -> ::core::ffi::c_int;
    fn abort() -> !;
}
pub const SIGFPE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "64")]
pub const MPD_IEEE_CONTEXT_MAX_BITS: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "32")]
pub const MPD_IEEE_CONTEXT_MAX_BITS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MPD_Max_status: ::core::ffi::c_uint =
    (0x8000 as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
#[no_mangle]
pub unsafe extern "C" fn mpd_dflt_traphandler(mut ctx: *mut mpd_context_t) {
    mpd_c2rust_raise(SIGFPE);
}
#[no_mangle]
pub static mut mpd_traphandler: Option<unsafe extern "C" fn(*mut mpd_context_t) -> ()> =
    unsafe { Some(mpd_dflt_traphandler as unsafe extern "C" fn(*mut mpd_context_t) -> ()) };
#[no_mangle]
pub unsafe extern "C" fn mpd_setminalloc(mut n: mpd_ssize_t) {
    static mut minalloc_is_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if minalloc_is_set != 0 {
        fprintf(
            mpd_c2rust_stderr(),
            b"%s:%d: warning: \0" as *const u8 as *const ::core::ffi::c_char,
            b"libmpdec/context.c\0" as *const u8 as *const ::core::ffi::c_char,
            56 as ::core::ffi::c_int,
        );
        fprintf(
            mpd_c2rust_stderr(),
            b"mpd_setminalloc: ignoring request to set MPD_MINALLOC a second time\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fputc('\n' as i32, mpd_c2rust_stderr());
        return;
    }
    if n < MPD_MINALLOC_MIN as mpd_ssize_t || n > MPD_MINALLOC_MAX as mpd_ssize_t {
        fprintf(
            mpd_c2rust_stderr(),
            b"%s:%d: error: \0" as *const u8 as *const ::core::ffi::c_char,
            b"libmpdec/context.c\0" as *const u8 as *const ::core::ffi::c_char,
            60 as ::core::ffi::c_int,
        );
        fprintf(
            mpd_c2rust_stderr(),
            b"illegal value for MPD_MINALLOC\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fputc('\n' as i32, mpd_c2rust_stderr());
        abort();
    }
    MPD_MINALLOC = n;
    minalloc_is_set = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_init(mut ctx: *mut mpd_context_t, mut prec: mpd_ssize_t) {
    let mut ideal_minalloc: mpd_ssize_t = 0;
    mpd_defaultcontext(ctx);
    if mpd_qsetprec(ctx, prec) == 0 {
        mpd_addstatus_raise(ctx, MPD_Invalid_context as uint32_t);
        return;
    }
    ideal_minalloc = 2 as mpd_ssize_t
        * ((prec + MPD_RDIGITS as mpd_ssize_t - 1 as mpd_ssize_t) / MPD_RDIGITS as mpd_ssize_t);
    if ideal_minalloc < MPD_MINALLOC_MIN as mpd_ssize_t {
        ideal_minalloc = MPD_MINALLOC_MIN as mpd_ssize_t;
    }
    if ideal_minalloc > MPD_MINALLOC_MAX as mpd_ssize_t {
        ideal_minalloc = MPD_MINALLOC_MAX as mpd_ssize_t;
    }
    mpd_setminalloc(ideal_minalloc);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_maxcontext(mut ctx: *mut mpd_context_t) {
    (*ctx).prec = MPD_MAX_PREC as mpd_ssize_t;
    (*ctx).emax = MPD_MAX_EMAX as mpd_ssize_t;
    (*ctx).emin = MPD_MIN_EMIN as mpd_ssize_t;
    (*ctx).round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    (*ctx).traps = MPD_Traps as uint32_t;
    (*ctx).status = 0 as uint32_t;
    (*ctx).newtrap = 0 as uint32_t;
    (*ctx).clamp = 0 as ::core::ffi::c_int;
    (*ctx).allcr = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_defaultcontext(mut ctx: *mut mpd_context_t) {
    (*ctx).prec = (2 as ::core::ffi::c_int * MPD_RDIGITS) as mpd_ssize_t;
    (*ctx).emax = MPD_MAX_EMAX as mpd_ssize_t;
    (*ctx).emin = MPD_MIN_EMIN as mpd_ssize_t;
    (*ctx).round = MPD_ROUND_HALF_UP as ::core::ffi::c_int;
    (*ctx).traps = MPD_Traps as uint32_t;
    (*ctx).status = 0 as uint32_t;
    (*ctx).newtrap = 0 as uint32_t;
    (*ctx).clamp = 0 as ::core::ffi::c_int;
    (*ctx).allcr = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_basiccontext(mut ctx: *mut mpd_context_t) {
    (*ctx).prec = 9 as mpd_ssize_t;
    (*ctx).emax = MPD_MAX_EMAX as mpd_ssize_t;
    (*ctx).emin = MPD_MIN_EMIN as mpd_ssize_t;
    (*ctx).round = MPD_ROUND_HALF_UP as ::core::ffi::c_int;
    (*ctx).traps = (MPD_Traps | MPD_Clamped) as uint32_t;
    (*ctx).status = 0 as uint32_t;
    (*ctx).newtrap = 0 as uint32_t;
    (*ctx).clamp = 0 as ::core::ffi::c_int;
    (*ctx).allcr = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_ieee_context(
    mut ctx: *mut mpd_context_t,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bits <= 0 as ::core::ffi::c_int
        || bits > MPD_IEEE_CONTEXT_MAX_BITS
        || bits % 32 as ::core::ffi::c_int != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctx).prec = (9 as ::core::ffi::c_int * (bits / 32 as ::core::ffi::c_int)
        - 2 as ::core::ffi::c_int) as mpd_ssize_t;
    (*ctx).emax = 3 as mpd_ssize_t
        * ((1 as mpd_ssize_t) << bits / 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int);
    (*ctx).emin = 1 as mpd_ssize_t - (*ctx).emax;
    (*ctx).round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    (*ctx).traps = 0 as uint32_t;
    (*ctx).status = 0 as uint32_t;
    (*ctx).newtrap = 0 as uint32_t;
    (*ctx).clamp = 1 as ::core::ffi::c_int;
    (*ctx).allcr = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_getprec(mut ctx: *const mpd_context_t) -> mpd_ssize_t {
    return (*ctx).prec;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_getemax(mut ctx: *const mpd_context_t) -> mpd_ssize_t {
    return (*ctx).emax;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_getemin(mut ctx: *const mpd_context_t) -> mpd_ssize_t {
    return (*ctx).emin;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_getround(mut ctx: *const mpd_context_t) -> ::core::ffi::c_int {
    return (*ctx).round;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_gettraps(mut ctx: *const mpd_context_t) -> uint32_t {
    return (*ctx).traps;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_getstatus(mut ctx: *const mpd_context_t) -> uint32_t {
    return (*ctx).status;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_getclamp(mut ctx: *const mpd_context_t) -> ::core::ffi::c_int {
    return (*ctx).clamp;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_getcr(mut ctx: *const mpd_context_t) -> ::core::ffi::c_int {
    return (*ctx).allcr;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsetprec(
    mut ctx: *mut mpd_context_t,
    mut prec: mpd_ssize_t,
) -> ::core::ffi::c_int {
    if prec <= 0 as mpd_ssize_t || prec > MPD_MAX_PREC as mpd_ssize_t {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).prec = prec;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsetemax(
    mut ctx: *mut mpd_context_t,
    mut emax: mpd_ssize_t,
) -> ::core::ffi::c_int {
    if emax < 0 as mpd_ssize_t || emax > MPD_MAX_EMAX as mpd_ssize_t {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).emax = emax;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsetemin(
    mut ctx: *mut mpd_context_t,
    mut emin: mpd_ssize_t,
) -> ::core::ffi::c_int {
    if emin > 0 as mpd_ssize_t || emin < MPD_MIN_EMIN as mpd_ssize_t {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).emin = emin;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsetround(
    mut ctx: *mut mpd_context_t,
    mut round: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !(0 as ::core::ffi::c_int <= round && round < MPD_ROUND_GUARD as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).round = round;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsettraps(
    mut ctx: *mut mpd_context_t,
    mut flags: uint32_t,
) -> ::core::ffi::c_int {
    if flags > MPD_Max_status as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).traps = flags;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsetstatus(
    mut ctx: *mut mpd_context_t,
    mut flags: uint32_t,
) -> ::core::ffi::c_int {
    if flags > MPD_Max_status as uint32_t {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).status = flags;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsetclamp(
    mut ctx: *mut mpd_context_t,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if c != 0 as ::core::ffi::c_int && c != 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).clamp = c;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsetcr(
    mut ctx: *mut mpd_context_t,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if c != 0 as ::core::ffi::c_int && c != 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).allcr = c;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_addstatus_raise(mut ctx: *mut mpd_context_t, mut flags: uint32_t) {
    (*ctx).status |= flags;
    if flags & (*ctx).traps != 0 {
        (*ctx).newtrap = flags & (*ctx).traps;
        mpd_traphandler.expect("non-null function pointer")(ctx);
    }
}
