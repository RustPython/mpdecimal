use crate::common::*;
use crate::shim::mpd_c2rust_assert_fail;
use crate::shim::mpd_c2rust_errno;
use crate::shim::mpd_c2rust_stderr;
#[cfg(target_pointer_width = "32")]
use crate::shim::mpd_c2rust_strtol;
#[cfg(target_pointer_width = "64")]
use crate::shim::mpd_c2rust_strtoll;
use crate::context::mpd_maxcontext;
use crate::mpdecimal::mpd_seterror;
use crate::mpdecimal::mpd_setspecial;
use crate::mpdecimal::mpd_qfinalize;
use crate::mpdecimal::mpd_qcopy;
use crate::mpdecimal::mpd_qrescale;
use crate::mpdecimal::mpd_qrescale_fmt;
use crate::mpdecimal::mpd_msword;
use crate::mpdecimal::mpd_word_digits;
use crate::mpdecimal::mpd_isinfinite;
use crate::mpdecimal::mpd_isnan;
use crate::mpdecimal::mpd_isnegative;
use crate::mpdecimal::mpd_isqnan;
use crate::mpdecimal::mpd_isspecial;
use crate::mpdecimal::mpd_iszero;
use crate::mpdecimal::mpd_setdigits;
use crate::mpdecimal::mpd_set_negative;
use crate::mpdecimal::mpd_set_flags;
use crate::mpalloc::mpd_free;
use crate::mpalloc::mpd_alloc;
use crate::mpalloc::mpd_realloc;
use crate::mpdecimal::mpd_del;
use crate::mpdecimal::mpd_qresize;
extern "C" {
    fn isdigit(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isupper(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tolower(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn localeconv() -> *mut lconv;
    fn fprintf(stream: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn printf(format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        s: *mut ::core::ffi::c_char,
        n: size_t,
        format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputs(s: *const ::core::ffi::c_char, stream: *mut FILE) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn memmove(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(s: *const ::core::ffi::c_char) -> size_t;
    fn strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut ::core::ffi::c_char,
    pub thousands_sep: *mut ::core::ffi::c_char,
    pub grouping: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_spec_t {
    pub min_width: mpd_ssize_t,
    pub prec: mpd_ssize_t,
    pub type_0: ::core::ffi::c_char,
    pub align: ::core::ffi::c_char,
    pub sign: ::core::ffi::c_char,
    pub sign_coerce: ::core::ffi::c_char,
    pub fill: [::core::ffi::c_char; 5],
    pub dot: *const ::core::ffi::c_char,
    pub sep: *const ::core::ffi::c_char,
    pub grouping: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_mbstr_t {
    pub nbytes: mpd_ssize_t,
    pub nchars: mpd_ssize_t,
    pub cur: mpd_ssize_t,
    pub data: *mut ::core::ffi::c_char,
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const MPD_NUM_FLAGS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
unsafe extern "C" fn mod_mpd_ssize_t(mut a: mpd_ssize_t, mut m: mpd_ssize_t) -> mpd_ssize_t {
    let mut r: mpd_ssize_t = a % m;
    return if r < 0 as mpd_ssize_t { r + m } else { r };
}
pub const SCHAR_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const CHAR_MAX: ::core::ffi::c_int = ::core::ffi::c_char::MAX as ::core::ffi::c_int;
unsafe extern "C" fn _mpd_strneq(
    mut s: *const ::core::ffi::c_char,
    mut l: *const ::core::ffi::c_char,
    mut u: *const ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    loop {
        n = n.wrapping_sub(1);
        if !(n != SIZE_MAX as size_t) {
            break;
        }
        if *s as ::core::ffi::c_int != *l as ::core::ffi::c_int
            && *s as ::core::ffi::c_int != *u as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        s = s.offset(1);
        u = u.offset(1);
        l = l.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn strtoexp(mut s: *const ::core::ffi::c_char) -> mpd_ssize_t {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut retval: mpd_ssize_t = 0;
    *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
    retval = mpd_c2rust_strtoll(s, &raw mut end, 10 as ::core::ffi::c_int) as mpd_ssize_t;
    if *mpd_c2rust_errno() == 0 as ::core::ffi::c_int
        && !(*s as ::core::ffi::c_int != '\0' as i32 && *end as ::core::ffi::c_int == '\0' as i32)
    {
        *mpd_c2rust_errno() = EINVAL;
    }
    return retval;
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn strtoexp(mut s: *const ::core::ffi::c_char) -> mpd_ssize_t {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut retval: mpd_ssize_t = 0;
    *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
    retval = mpd_c2rust_strtol(s, &raw mut end, 10 as ::core::ffi::c_int) as mpd_ssize_t;
    if *mpd_c2rust_errno() == 0 as ::core::ffi::c_int
        && !(*s as ::core::ffi::c_int != '\0' as i32 && *end as ::core::ffi::c_int == '\0' as i32)
    {
        *mpd_c2rust_errno() = EINVAL;
    }
    return retval;
}
unsafe extern "C" fn string_to_coeff(
    mut data: *mut mpd_uint_t,
    mut s: *const ::core::ffi::c_char,
    mut dpoint: *const ::core::ffi::c_char,
    mut r: ::core::ffi::c_int,
    mut len: size_t,
) {
    let mut j: ::core::ffi::c_int = 0;
    if r > 0 as ::core::ffi::c_int {
        len = len.wrapping_sub(1);
        *data.offset(len as isize) = 0 as mpd_uint_t;
        j = 0 as ::core::ffi::c_int;
        while j < r {
            if s == dpoint {
                s = s.offset(1);
            }
            *data.offset(len as isize) = (10 as mpd_uint_t)
                .wrapping_mul(*data.offset(len as isize))
                .wrapping_add((*s as ::core::ffi::c_int - '0' as i32) as mpd_uint_t);
            j += 1;
            s = s.offset(1);
        }
    }
    loop {
        len = len.wrapping_sub(1);
        if !(len != SIZE_MAX as size_t) {
            break;
        }
        *data.offset(len as isize) = 0 as mpd_uint_t;
        j = 0 as ::core::ffi::c_int;
        while j < MPD_RDIGITS {
            if s == dpoint {
                s = s.offset(1);
            }
            *data.offset(len as isize) = (10 as mpd_uint_t)
                .wrapping_mul(*data.offset(len as isize))
                .wrapping_add((*s as ::core::ffi::c_int - '0' as i32) as mpd_uint_t);
            j += 1;
            s = s.offset(1);
        }
    }
}
unsafe extern "C" fn scan_dpoint_exp(
    mut s: *const ::core::ffi::c_char,
    mut dpoint: *mut *const ::core::ffi::c_char,
    mut exp: *mut *const ::core::ffi::c_char,
    mut end: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut coeff: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    *dpoint = ::core::ptr::null::<::core::ffi::c_char>();
    *exp = ::core::ptr::null::<::core::ffi::c_char>();
    while *s as ::core::ffi::c_int != '\0' as i32 {
        match *s as ::core::ffi::c_int {
            46 => {
                if !(*dpoint).is_null() || !(*exp).is_null() {
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                *dpoint = s;
            }
            69 | 101 => {
                if !(*exp).is_null() {
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                *exp = s;
                if *s.offset(1 as isize) as ::core::ffi::c_int == '+' as i32
                    || *s.offset(1 as isize) as ::core::ffi::c_int == '-' as i32
                {
                    s = s.offset(1);
                }
            }
            _ => {
                if isdigit(*s as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                if coeff.is_null() && (*exp).is_null() {
                    if *s as ::core::ffi::c_int == '0' as i32 {
                        if isdigit(
                            *s.offset(1 as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                        ) == 0
                        {
                            if !(*s.offset(1 as isize) as ::core::ffi::c_int == '.' as i32
                                && isdigit(*s.offset(2 as isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int)
                                    != 0)
                            {
                                coeff = s;
                            }
                        }
                    } else {
                        coeff = s;
                    }
                }
            }
        }
        s = s.offset(1);
    }
    *end = s;
    return coeff;
}
unsafe extern "C" fn scan_payload(
    mut s: *const ::core::ffi::c_char,
    mut end: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut coeff: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    while *s as ::core::ffi::c_int == '0' as i32 {
        s = s.offset(1);
    }
    coeff = s;
    while isdigit(*s as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        s = s.offset(1);
    }
    *end = s;
    return if *s as ::core::ffi::c_int == '\0' as i32 {
        coeff
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_string(
    mut dec: *mut mpd_t,
    mut s: *const ::core::ffi::c_char,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut q: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    let mut len: mpd_ssize_t = 0;
    let mut coeff: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dpoint: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut exp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut digits: size_t = 0;
    let mut sign: uint8_t = MPD_POS;
    mpd_set_flags(dec, 0 as uint8_t);
    (*dec).len = 0 as mpd_ssize_t;
    (*dec).exp = 0 as mpd_ssize_t;
    if *s as ::core::ffi::c_int == '+' as i32 {
        s = s.offset(1);
    } else if *s as ::core::ffi::c_int == '-' as i32 {
        mpd_set_negative(dec);
        sign = MPD_NEG;
        s = s.offset(1);
    }
    if _mpd_strneq(
        s,
        b"nan\0" as *const u8 as *const ::core::ffi::c_char,
        b"NAN\0" as *const u8 as *const ::core::ffi::c_char,
        3 as size_t,
    ) != 0
    {
        s = s.offset(3 as isize);
        mpd_setspecial(dec, sign, MPD_NAN);
        if *s as ::core::ffi::c_int == '\0' as i32 {
            return;
        }
        coeff = scan_payload(s, &raw mut end);
        if coeff.is_null() {
            current_block = 1;
        } else {
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as i64 as size_t;
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                current_block = 1;
            } else {
                current_block = 2;
            }
        }
    } else if _mpd_strneq(
        s,
        b"snan\0" as *const u8 as *const ::core::ffi::c_char,
        b"SNAN\0" as *const u8 as *const ::core::ffi::c_char,
        4 as size_t,
    ) != 0
    {
        s = s.offset(4 as isize);
        mpd_setspecial(dec, sign, MPD_SNAN);
        if *s as ::core::ffi::c_int == '\0' as i32 {
            return;
        }
        coeff = scan_payload(s, &raw mut end);
        if coeff.is_null() {
            current_block = 1;
        } else {
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as i64 as size_t;
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                current_block = 1;
            } else {
                current_block = 2;
            }
        }
    } else if _mpd_strneq(
        s,
        b"inf\0" as *const u8 as *const ::core::ffi::c_char,
        b"INF\0" as *const u8 as *const ::core::ffi::c_char,
        3 as size_t,
    ) != 0
    {
        s = s.offset(3 as isize);
        if *s as ::core::ffi::c_int == '\0' as i32
            || _mpd_strneq(
                s,
                b"inity\0" as *const u8 as *const ::core::ffi::c_char,
                b"INITY\0" as *const u8 as *const ::core::ffi::c_char,
                6 as size_t,
            ) != 0
        {
            mpd_setspecial(dec, sign, MPD_INF);
            return;
        }
        current_block = 1;
    } else {
        coeff = scan_dpoint_exp(s, &raw mut dpoint, &raw mut exp, &raw mut end);
        if coeff.is_null() {
            current_block = 1;
        } else {
            if !exp.is_null() {
                end = exp;
                exp = exp.offset(1);
                (*dec).exp = strtoexp(exp);
                if *mpd_c2rust_errno() != 0 {
                    if !(*mpd_c2rust_errno() == ERANGE
                        && ((*dec).exp == MPD_SSIZE_MAX as mpd_ssize_t
                            || (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t))
                    {
                        current_block = 1;
                    } else {
                        current_block = 3;
                    }
                } else {
                    current_block = 3;
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    digits = end.offset_from(coeff) as i64 as size_t;
                    if !dpoint.is_null() {
                        let mut fracdigits: size_t =
                            (end.offset_from(dpoint) as i64 - 1 as i64) as size_t;
                        if dpoint > coeff {
                            digits = digits.wrapping_sub(1);
                        }
                        if fracdigits as u64 > MPD_MAX_PREC as u64 {
                            current_block = 1;
                        } else {
                            if (*dec).exp < MPD_SSIZE_MIN as mpd_ssize_t + fracdigits as mpd_ssize_t
                            {
                                (*dec).exp = MPD_SSIZE_MIN as mpd_ssize_t;
                            } else {
                                (*dec).exp -= fracdigits as mpd_ssize_t;
                            }
                            current_block = 4;
                        }
                    } else {
                        current_block = 4;
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            if digits as u64 > MPD_MAX_PREC as u64 {
                                current_block = 1;
                            } else {
                                if (*dec).exp > MPD_EXP_INF as mpd_ssize_t {
                                    (*dec).exp = MPD_EXP_INF as mpd_ssize_t;
                                }
                                if (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t {
                                    (*dec).exp = (MPD_SSIZE_MIN + 1 as i64) as mpd_ssize_t;
                                }
                                current_block = 2;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        2 => {
            _mpd_idiv_word(
                &raw mut q,
                &raw mut r,
                digits as mpd_ssize_t,
                MPD_RDIGITS as mpd_ssize_t,
            );
            len = if r == 0 as mpd_ssize_t {
                q
            } else {
                q + 1 as mpd_ssize_t
            };
            if !(len == 0 as mpd_ssize_t) {
                if mpd_qresize(dec, len, status) == 0 {
                    mpd_seterror(dec, MPD_Malloc_error as uint32_t, status);
                    return;
                }
                (*dec).len = len;
                string_to_coeff(
                    (*dec).data,
                    coeff,
                    dpoint,
                    r as ::core::ffi::c_int,
                    len as size_t,
                );
                mpd_setdigits(dec);
                mpd_qfinalize(dec, ctx, status);
                return;
            }
        }
        _ => {}
    }
    mpd_seterror(dec, MPD_Conversion_syntax as uint32_t, status);
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_string(
    mut dec: *mut mpd_t,
    mut s: *const ::core::ffi::c_char,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut q: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    let mut len: mpd_ssize_t = 0;
    let mut coeff: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dpoint: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut exp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut digits: size_t = 0;
    let mut sign: uint8_t = MPD_POS;
    mpd_set_flags(dec, 0 as uint8_t);
    (*dec).len = 0 as mpd_ssize_t;
    (*dec).exp = 0 as mpd_ssize_t;
    if *s as ::core::ffi::c_int == '+' as i32 {
        s = s.offset(1);
    } else if *s as ::core::ffi::c_int == '-' as i32 {
        mpd_set_negative(dec);
        sign = MPD_NEG;
        s = s.offset(1);
    }
    if _mpd_strneq(
        s,
        b"nan\0" as *const u8 as *const ::core::ffi::c_char,
        b"NAN\0" as *const u8 as *const ::core::ffi::c_char,
        3 as size_t,
    ) != 0
    {
        s = s.offset(3 as isize);
        mpd_setspecial(dec, sign, MPD_NAN);
        if *s as ::core::ffi::c_int == '\0' as i32 {
            return;
        }
        coeff = scan_payload(s, &raw mut end);
        if coeff.is_null() {
            current_block = 1;
        } else {
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as ::core::ffi::c_int as size_t;
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                current_block = 1;
            } else {
                current_block = 2;
            }
        }
    } else if _mpd_strneq(
        s,
        b"snan\0" as *const u8 as *const ::core::ffi::c_char,
        b"SNAN\0" as *const u8 as *const ::core::ffi::c_char,
        4 as size_t,
    ) != 0
    {
        s = s.offset(4 as isize);
        mpd_setspecial(dec, sign, MPD_SNAN);
        if *s as ::core::ffi::c_int == '\0' as i32 {
            return;
        }
        coeff = scan_payload(s, &raw mut end);
        if coeff.is_null() {
            current_block = 1;
        } else {
            if *coeff as ::core::ffi::c_int == '\0' as i32 {
                return;
            }
            digits = end.offset_from(coeff) as ::core::ffi::c_int as size_t;
            if digits > ((*ctx).prec - (*ctx).clamp as mpd_ssize_t) as size_t {
                current_block = 1;
            } else {
                current_block = 2;
            }
        }
    } else if _mpd_strneq(
        s,
        b"inf\0" as *const u8 as *const ::core::ffi::c_char,
        b"INF\0" as *const u8 as *const ::core::ffi::c_char,
        3 as size_t,
    ) != 0
    {
        s = s.offset(3 as isize);
        if *s as ::core::ffi::c_int == '\0' as i32
            || _mpd_strneq(
                s,
                b"inity\0" as *const u8 as *const ::core::ffi::c_char,
                b"INITY\0" as *const u8 as *const ::core::ffi::c_char,
                6 as size_t,
            ) != 0
        {
            mpd_setspecial(dec, sign, MPD_INF);
            return;
        }
        current_block = 1;
    } else {
        coeff = scan_dpoint_exp(s, &raw mut dpoint, &raw mut exp, &raw mut end);
        if coeff.is_null() {
            current_block = 1;
        } else {
            if !exp.is_null() {
                end = exp;
                exp = exp.offset(1);
                (*dec).exp = strtoexp(exp);
                if *mpd_c2rust_errno() != 0 {
                    if !(*mpd_c2rust_errno() == ERANGE
                        && ((*dec).exp == MPD_SSIZE_MAX as mpd_ssize_t
                            || (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t))
                    {
                        current_block = 1;
                    } else {
                        current_block = 3;
                    }
                } else {
                    current_block = 3;
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    digits = end.offset_from(coeff) as ::core::ffi::c_int as size_t;
                    if !dpoint.is_null() {
                        let mut fracdigits: size_t = (end.offset_from(dpoint) as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as size_t;
                        if dpoint > coeff {
                            digits = digits.wrapping_sub(1);
                        }
                        if fracdigits > MPD_MAX_PREC as size_t {
                            current_block = 1;
                        } else {
                            if (*dec).exp < MPD_SSIZE_MIN as mpd_ssize_t + fracdigits as mpd_ssize_t
                            {
                                (*dec).exp = MPD_SSIZE_MIN as mpd_ssize_t;
                            } else {
                                (*dec).exp -= fracdigits as mpd_ssize_t;
                            }
                            current_block = 4;
                        }
                    } else {
                        current_block = 4;
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            if digits > MPD_MAX_PREC as size_t {
                                current_block = 1;
                            } else {
                                if (*dec).exp > MPD_EXP_INF as mpd_ssize_t {
                                    (*dec).exp = MPD_EXP_INF as mpd_ssize_t;
                                }
                                if (*dec).exp == MPD_SSIZE_MIN as mpd_ssize_t {
                                    (*dec).exp =
                                        (MPD_SSIZE_MIN + 1 as ::core::ffi::c_int) as mpd_ssize_t;
                                }
                                current_block = 2;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        2 => {
            _mpd_idiv_word(
                &raw mut q,
                &raw mut r,
                digits as mpd_ssize_t,
                MPD_RDIGITS as mpd_ssize_t,
            );
            len = if r == 0 as mpd_ssize_t {
                q
            } else {
                q + 1 as mpd_ssize_t
            };
            if !(len == 0 as mpd_ssize_t) {
                if mpd_qresize(dec, len, status) == 0 {
                    mpd_seterror(dec, MPD_Malloc_error as uint32_t, status);
                    return;
                }
                (*dec).len = len;
                string_to_coeff(
                    (*dec).data,
                    coeff,
                    dpoint,
                    r as ::core::ffi::c_int,
                    len as size_t,
                );
                mpd_setdigits(dec);
                mpd_qfinalize(dec, ctx, status);
                return;
            }
        }
        _ => {}
    }
    mpd_seterror(dec, MPD_Conversion_syntax as uint32_t, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_string_exact(
    mut dec: *mut mpd_t,
    mut s: *const ::core::ffi::c_char,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut workstatus: uint32_t = 0 as uint32_t;
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qset_string(dec, s, &raw mut maxcontext, &raw mut workstatus);
    if workstatus & (MPD_Inexact as uint32_t | MPD_Rounded as uint32_t | MPD_Clamped as uint32_t)
        != 0
    {
        mpd_seterror(dec, MPD_Invalid_operation as uint32_t, status);
    }
    *status = (*status as ::core::ffi::c_uint
        | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint) as uint32_t;
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn word_to_string(
    mut s: *mut ::core::ffi::c_char,
    mut x: mpd_uint_t,
    mut n: ::core::ffi::c_int,
    mut dot: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut current_block_78: u64;
    match n {
        20 => {
            if s == dot {
                let fresh0 = s;
                s = s.offset(1);
                *fresh0 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 = ('0' as i32
                + x.wrapping_div(10000000000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(10000000000000000000 as u64) as mpd_uint_t;
            current_block_78 = 1;
        }
        19 => {
            current_block_78 = 1;
        }
        18 => {
            current_block_78 = 2;
        }
        17 => {
            current_block_78 = 3;
        }
        16 => {
            current_block_78 = 4;
        }
        15 => {
            current_block_78 = 5;
        }
        14 => {
            current_block_78 = 6;
        }
        13 => {
            current_block_78 = 7;
        }
        12 => {
            current_block_78 = 8;
        }
        11 => {
            current_block_78 = 9;
        }
        10 => {
            current_block_78 = 10;
        }
        9 => {
            current_block_78 = 11;
        }
        8 => {
            current_block_78 = 12;
        }
        7 => {
            current_block_78 = 13;
        }
        6 => {
            current_block_78 = 14;
        }
        5 => {
            current_block_78 = 15;
        }
        4 => {
            current_block_78 = 16;
        }
        3 => {
            current_block_78 = 17;
        }
        2 => {
            current_block_78 = 18;
        }
        _ => {
            current_block_78 = 19;
        }
    }
    match current_block_78 {
        1 => {
            if s == dot {
                let fresh2 = s;
                s = s.offset(1);
                *fresh2 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 = ('0' as i32
                + x.wrapping_div(1000000000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(1000000000000000000 as u64) as mpd_uint_t;
            current_block_78 = 2;
        }
        _ => {}
    }
    match current_block_78 {
        2 => {
            if s == dot {
                let fresh4 = s;
                s = s.offset(1);
                *fresh4 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh5 = s;
            s = s.offset(1);
            *fresh5 = ('0' as i32
                + x.wrapping_div(100000000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(100000000000000000 as u64) as mpd_uint_t;
            current_block_78 = 3;
        }
        _ => {}
    }
    match current_block_78 {
        3 => {
            if s == dot {
                let fresh6 = s;
                s = s.offset(1);
                *fresh6 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh7 = s;
            s = s.offset(1);
            *fresh7 = ('0' as i32
                + x.wrapping_div(10000000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(10000000000000000 as u64) as mpd_uint_t;
            current_block_78 = 4;
        }
        _ => {}
    }
    match current_block_78 {
        4 => {
            if s == dot {
                let fresh8 = s;
                s = s.offset(1);
                *fresh8 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh9 = s;
            s = s.offset(1);
            *fresh9 = ('0' as i32
                + x.wrapping_div(1000000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(1000000000000000 as u64) as mpd_uint_t;
            current_block_78 = 5;
        }
        _ => {}
    }
    match current_block_78 {
        5 => {
            if s == dot {
                let fresh10 = s;
                s = s.offset(1);
                *fresh10 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh11 = s;
            s = s.offset(1);
            *fresh11 = ('0' as i32
                + x.wrapping_div(100000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(100000000000000 as u64) as mpd_uint_t;
            current_block_78 = 6;
        }
        _ => {}
    }
    match current_block_78 {
        6 => {
            if s == dot {
                let fresh12 = s;
                s = s.offset(1);
                *fresh12 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh13 = s;
            s = s.offset(1);
            *fresh13 = ('0' as i32
                + x.wrapping_div(10000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(10000000000000 as u64) as mpd_uint_t;
            current_block_78 = 7;
        }
        _ => {}
    }
    match current_block_78 {
        7 => {
            if s == dot {
                let fresh14 = s;
                s = s.offset(1);
                *fresh14 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh15 = s;
            s = s.offset(1);
            *fresh15 = ('0' as i32
                + x.wrapping_div(1000000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(1000000000000 as u64) as mpd_uint_t;
            current_block_78 = 8;
        }
        _ => {}
    }
    match current_block_78 {
        8 => {
            if s == dot {
                let fresh16 = s;
                s = s.offset(1);
                *fresh16 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh17 = s;
            s = s.offset(1);
            *fresh17 = ('0' as i32
                + x.wrapping_div(100000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(100000000000 as u64) as mpd_uint_t;
            current_block_78 = 9;
        }
        _ => {}
    }
    match current_block_78 {
        9 => {
            if s == dot {
                let fresh18 = s;
                s = s.offset(1);
                *fresh18 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh19 = s;
            s = s.offset(1);
            *fresh19 = ('0' as i32
                + x.wrapping_div(10000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u64).wrapping_rem(10000000000 as u64) as mpd_uint_t as mpd_uint_t;
            current_block_78 = 10;
        }
        _ => {}
    }
    match current_block_78 {
        10 => {
            if s == dot {
                let fresh20 = s;
                s = s.offset(1);
                *fresh20 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh21 = s;
            s = s.offset(1);
            *fresh21 = ('0' as i32
                + x.wrapping_div(1000000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = x.wrapping_rem(1000000000 as mpd_uint_t);
            current_block_78 = 11;
        }
        _ => {}
    }
    match current_block_78 {
        11 => {
            if s == dot {
                let fresh22 = s;
                s = s.offset(1);
                *fresh22 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh23 = s;
            s = s.offset(1);
            *fresh23 = ('0' as i32
                + x.wrapping_div(100000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = x.wrapping_rem(100000000 as mpd_uint_t);
            current_block_78 = 12;
        }
        _ => {}
    }
    match current_block_78 {
        12 => {
            if s == dot {
                let fresh24 = s;
                s = s.offset(1);
                *fresh24 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh25 = s;
            s = s.offset(1);
            *fresh25 = ('0' as i32
                + x.wrapping_div(10000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = x.wrapping_rem(10000000 as mpd_uint_t);
            current_block_78 = 13;
        }
        _ => {}
    }
    match current_block_78 {
        13 => {
            if s == dot {
                let fresh26 = s;
                s = s.offset(1);
                *fresh26 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh27 = s;
            s = s.offset(1);
            *fresh27 = ('0' as i32
                + x.wrapping_div(1000000 as mpd_uint_t) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = x.wrapping_rem(1000000 as mpd_uint_t);
            current_block_78 = 14;
        }
        _ => {}
    }
    match current_block_78 {
        14 => {
            if s == dot {
                let fresh28 = s;
                s = s.offset(1);
                *fresh28 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh29 = s;
            s = s.offset(1);
            *fresh29 = ('0' as i32
                + x.wrapping_div(100000 as mpd_uint_t) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = x.wrapping_rem(100000 as mpd_uint_t);
            current_block_78 = 15;
        }
        _ => {}
    }
    match current_block_78 {
        15 => {
            if s == dot {
                let fresh30 = s;
                s = s.offset(1);
                *fresh30 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh31 = s;
            s = s.offset(1);
            *fresh31 = ('0' as i32
                + x.wrapping_div(10000 as mpd_uint_t) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = x.wrapping_rem(10000 as mpd_uint_t);
            current_block_78 = 16;
        }
        _ => {}
    }
    match current_block_78 {
        16 => {
            if s == dot {
                let fresh32 = s;
                s = s.offset(1);
                *fresh32 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh33 = s;
            s = s.offset(1);
            *fresh33 = ('0' as i32
                + x.wrapping_div(1000 as mpd_uint_t) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = x.wrapping_rem(1000 as mpd_uint_t);
            current_block_78 = 17;
        }
        _ => {}
    }
    match current_block_78 {
        17 => {
            if s == dot {
                let fresh34 = s;
                s = s.offset(1);
                *fresh34 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh35 = s;
            s = s.offset(1);
            *fresh35 = ('0' as i32
                + x.wrapping_div(100 as mpd_uint_t) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = x.wrapping_rem(100 as mpd_uint_t);
            current_block_78 = 18;
        }
        _ => {}
    }
    match current_block_78 {
        18 => {
            if s == dot {
                let fresh36 = s;
                s = s.offset(1);
                *fresh36 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh37 = s;
            s = s.offset(1);
            *fresh37 = ('0' as i32
                + x.wrapping_div(10 as mpd_uint_t) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = x.wrapping_rem(10 as mpd_uint_t);
        }
        _ => {}
    }
    if s == dot {
        let fresh38 = s;
        s = s.offset(1);
        *fresh38 = '.' as i32 as ::core::ffi::c_char;
    }
    let fresh39 = s;
    s = s.offset(1);
    *fresh39 = ('0' as i32 + x as ::core::ffi::c_char as ::core::ffi::c_int) as ::core::ffi::c_char;
    *s = '\0' as i32 as ::core::ffi::c_char;
    return s;
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn word_to_string(
    mut s: *mut ::core::ffi::c_char,
    mut x: mpd_uint_t,
    mut n: ::core::ffi::c_int,
    mut dot: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut current_block_38: u64;
    match n {
        10 => {
            if s == dot {
                let fresh0 = s;
                s = s.offset(1);
                *fresh0 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 = ('0' as i32
                + (x as u32).wrapping_div(1000000000 as u32) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(1000000000 as u32) as mpd_uint_t;
            current_block_38 = 1;
        }
        9 => {
            current_block_38 = 1;
        }
        8 => {
            current_block_38 = 2;
        }
        7 => {
            current_block_38 = 3;
        }
        6 => {
            current_block_38 = 4;
        }
        5 => {
            current_block_38 = 5;
        }
        4 => {
            current_block_38 = 6;
        }
        3 => {
            current_block_38 = 7;
        }
        2 => {
            current_block_38 = 8;
        }
        _ => {
            current_block_38 = 9;
        }
    }
    match current_block_38 {
        1 => {
            if s == dot {
                let fresh2 = s;
                s = s.offset(1);
                *fresh2 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 = ('0' as i32
                + (x as u32).wrapping_div(100000000 as u32) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(100000000 as u32) as mpd_uint_t;
            current_block_38 = 2;
        }
        _ => {}
    }
    match current_block_38 {
        2 => {
            if s == dot {
                let fresh4 = s;
                s = s.offset(1);
                *fresh4 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh5 = s;
            s = s.offset(1);
            *fresh5 = ('0' as i32
                + (x as u32).wrapping_div(10000000 as u32) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(10000000 as u32) as mpd_uint_t;
            current_block_38 = 3;
        }
        _ => {}
    }
    match current_block_38 {
        3 => {
            if s == dot {
                let fresh6 = s;
                s = s.offset(1);
                *fresh6 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh7 = s;
            s = s.offset(1);
            *fresh7 = ('0' as i32
                + (x as u32).wrapping_div(1000000 as u32) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(1000000 as u32) as mpd_uint_t;
            current_block_38 = 4;
        }
        _ => {}
    }
    match current_block_38 {
        4 => {
            if s == dot {
                let fresh8 = s;
                s = s.offset(1);
                *fresh8 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh9 = s;
            s = s.offset(1);
            *fresh9 = ('0' as i32
                + (x as u32).wrapping_div(100000 as u32) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(100000 as u32) as mpd_uint_t;
            current_block_38 = 5;
        }
        _ => {}
    }
    match current_block_38 {
        5 => {
            if s == dot {
                let fresh10 = s;
                s = s.offset(1);
                *fresh10 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh11 = s;
            s = s.offset(1);
            *fresh11 = ('0' as i32
                + (x as u32).wrapping_div(10000 as u32) as ::core::ffi::c_char
                    as ::core::ffi::c_int) as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(10000 as u32) as mpd_uint_t;
            current_block_38 = 6;
        }
        _ => {}
    }
    match current_block_38 {
        6 => {
            if s == dot {
                let fresh12 = s;
                s = s.offset(1);
                *fresh12 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh13 = s;
            s = s.offset(1);
            *fresh13 = ('0' as i32
                + (x as u32).wrapping_div(1000 as u32) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(1000 as u32) as mpd_uint_t as mpd_uint_t;
            current_block_38 = 7;
        }
        _ => {}
    }
    match current_block_38 {
        7 => {
            if s == dot {
                let fresh14 = s;
                s = s.offset(1);
                *fresh14 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh15 = s;
            s = s.offset(1);
            *fresh15 = ('0' as i32
                + (x as u32).wrapping_div(100 as u32) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(100 as u32) as mpd_uint_t as mpd_uint_t;
            current_block_38 = 8;
        }
        _ => {}
    }
    match current_block_38 {
        8 => {
            if s == dot {
                let fresh16 = s;
                s = s.offset(1);
                *fresh16 = '.' as i32 as ::core::ffi::c_char;
            }
            let fresh17 = s;
            s = s.offset(1);
            *fresh17 = ('0' as i32
                + (x as u32).wrapping_div(10 as u32) as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            x = (x as u32).wrapping_rem(10 as u32) as mpd_uint_t as mpd_uint_t;
        }
        _ => {}
    }
    if s == dot {
        let fresh18 = s;
        s = s.offset(1);
        *fresh18 = '.' as i32 as ::core::ffi::c_char;
    }
    let fresh19 = s;
    s = s.offset(1);
    *fresh19 = ('0' as i32 + x as ::core::ffi::c_char as ::core::ffi::c_int) as ::core::ffi::c_char;
    *s = '\0' as i32 as ::core::ffi::c_char;
    return s;
}
unsafe extern "C" fn exp_to_string(
    mut s: *mut ::core::ffi::c_char,
    mut x: mpd_ssize_t,
) -> *mut ::core::ffi::c_char {
    let mut sign: ::core::ffi::c_char = '+' as i32 as ::core::ffi::c_char;
    if x < 0 as mpd_ssize_t {
        sign = '-' as i32 as ::core::ffi::c_char;
        x = -x;
    }
    let fresh0 = s;
    s = s.offset(1);
    *fresh0 = sign;
    return word_to_string(
        s,
        x as mpd_uint_t,
        mpd_word_digits(x as mpd_uint_t),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn coeff_to_string(
    mut s: *mut ::core::ffi::c_char,
    mut dec: *const mpd_t,
) -> *mut ::core::ffi::c_char {
    let mut x: mpd_uint_t = 0;
    let mut i: mpd_ssize_t = 0;
    x = mpd_msword(dec);
    s = word_to_string(
        s,
        x,
        mpd_word_digits(x),
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    i = (*dec).len - 2 as mpd_ssize_t;
    while i >= 0 as mpd_ssize_t {
        x = *(*dec).data.offset(i as isize);
        s = word_to_string(
            s,
            x,
            MPD_RDIGITS,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        i -= 1;
    }
    return s;
}
unsafe extern "C" fn coeff_to_string_dot(
    mut s: *mut ::core::ffi::c_char,
    mut dot: *mut ::core::ffi::c_char,
    mut dec: *const mpd_t,
) -> *mut ::core::ffi::c_char {
    let mut x: mpd_uint_t = 0;
    let mut i: mpd_ssize_t = 0;
    x = mpd_msword(dec);
    s = word_to_string(s, x, mpd_word_digits(x), dot);
    i = (*dec).len - 2 as mpd_ssize_t;
    while i >= 0 as mpd_ssize_t {
        x = *(*dec).data.offset(i as isize);
        s = word_to_string(s, x, MPD_RDIGITS, dot);
        i -= 1;
    }
    return s;
}
pub const MPD_FMT_LOWER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MPD_FMT_UPPER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MPD_FMT_TOSCI: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MPD_FMT_TOENG: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MPD_FMT_EXP: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MPD_FMT_FIXED: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MPD_FMT_PERCENT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MPD_FMT_SIGN_SPACE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const MPD_FMT_SIGN_PLUS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MPD_FMT_SIGN_COERCE: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const MPD_DEFAULT_DOTPLACE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_to_string(
    mut result: *mut *mut ::core::ffi::c_char,
    mut dec: *const mpd_t,
    mut flags: ::core::ffi::c_int,
    mut dplace: mpd_ssize_t,
) -> mpd_ssize_t {
    let mut decstring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ldigits: mpd_ssize_t = 0;
    let mut mem: mpd_ssize_t = 0 as mpd_ssize_t;
    let mut k: mpd_ssize_t = 0;
    if mpd_isspecial(dec) != 0 {
        mem = ::core::mem::size_of::<[::core::ffi::c_char; 11]>() as mpd_ssize_t;
        if mpd_isnan(dec) != 0 && (*dec).len > 0 as mpd_ssize_t {
            mem += (*dec).digits;
        }
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as ::core::ffi::c_int) as mpd_ssize_t;
        }
        if mpd_isnegative(dec) != 0 {
            let fresh0 = cp;
            cp = cp.offset(1);
            *fresh0 = '-' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            let fresh1 = cp;
            cp = cp.offset(1);
            *fresh1 = ' ' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            let fresh2 = cp;
            cp = cp.offset(1);
            *fresh2 = '+' as i32 as ::core::ffi::c_char;
        }
        if mpd_isnan(dec) != 0 {
            if mpd_isqnan(dec) != 0 {
                strcpy(cp, b"NaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(3 as isize);
            } else {
                strcpy(cp, b"sNaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(4 as isize);
            }
            if (*dec).len > 0 as mpd_ssize_t {
                cp = coeff_to_string(cp, dec);
            }
        } else if mpd_isinfinite(dec) != 0 {
            strcpy(cp, b"Infinity\0" as *const u8 as *const ::core::ffi::c_char);
            cp = cp.offset(8 as isize);
        } else {
            abort();
        }
    } else {
        if (*dec).len > 0 as mpd_ssize_t {
        } else {
            mpd_c2rust_assert_fail(b"dec->len > 0\0" as *const u8 as *const ::core::ffi::c_char);
        };
        ldigits = (*dec).digits + (*dec).exp;
        if !(flags & MPD_FMT_EXP != 0) {
            if flags & MPD_FMT_FIXED != 0
                || (*dec).exp <= 0 as mpd_ssize_t
                    && ldigits > -(6 as ::core::ffi::c_int) as mpd_ssize_t
            {
                dplace = ldigits;
            } else if flags & MPD_FMT_TOENG != 0 {
                if mpd_iszero(dec) != 0 {
                    dplace = -(1 as ::core::ffi::c_int) as mpd_ssize_t
                        + mod_mpd_ssize_t((*dec).exp + 2 as mpd_ssize_t, 3 as mpd_ssize_t);
                } else {
                    dplace += mod_mpd_ssize_t(ldigits - 1 as mpd_ssize_t, 3 as mpd_ssize_t);
                }
            }
        }
        if dplace <= 0 as mpd_ssize_t {
            mem = -dplace + (*dec).digits + 2 as mpd_ssize_t;
        } else if dplace >= (*dec).digits {
            mem = dplace;
        } else {
            mem = (*dec).digits;
        }
        mem += (MPD_EXPDIGITS + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as mpd_ssize_t;
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as ::core::ffi::c_int) as mpd_ssize_t;
        }
        if mpd_isnegative(dec) != 0 && !(flags & MPD_FMT_SIGN_COERCE != 0 && mpd_iszero(dec) != 0) {
            let fresh3 = cp;
            cp = cp.offset(1);
            *fresh3 = '-' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            let fresh4 = cp;
            cp = cp.offset(1);
            *fresh4 = ' ' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            let fresh5 = cp;
            cp = cp.offset(1);
            *fresh5 = '+' as i32 as ::core::ffi::c_char;
        }
        if dplace <= 0 as mpd_ssize_t {
            let fresh6 = cp;
            cp = cp.offset(1);
            *fresh6 = '0' as i32 as ::core::ffi::c_char;
            let fresh7 = cp;
            cp = cp.offset(1);
            *fresh7 = '.' as i32 as ::core::ffi::c_char;
            k = 0 as mpd_ssize_t;
            while k < -dplace {
                let fresh8 = cp;
                cp = cp.offset(1);
                *fresh8 = '0' as i32 as ::core::ffi::c_char;
                k += 1;
            }
            cp = coeff_to_string(cp, dec);
        } else if dplace >= (*dec).digits {
            cp = coeff_to_string(cp, dec);
            k = 0 as mpd_ssize_t;
            while k < dplace - (*dec).digits {
                let fresh9 = cp;
                cp = cp.offset(1);
                *fresh9 = '0' as i32 as ::core::ffi::c_char;
                k += 1;
            }
        } else {
            cp = coeff_to_string_dot(cp, cp.offset(dplace as isize), dec);
        }
        if ldigits != dplace || flags & MPD_FMT_EXP != 0 {
            let fresh10 = cp;
            cp = cp.offset(1);
            *fresh10 = (if flags & MPD_FMT_UPPER != 0 {
                'E' as i32
            } else {
                'e' as i32
            }) as ::core::ffi::c_char;
            cp = exp_to_string(cp, ldigits - dplace);
        }
    }
    if flags & MPD_FMT_PERCENT != 0 {
        let fresh11 = cp;
        cp = cp.offset(1);
        *fresh11 = '%' as i32 as ::core::ffi::c_char;
    }
    if cp < decstring.offset(mem as isize) {
    } else {
        mpd_c2rust_assert_fail(b"cp < decstring+mem\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if (cp.offset_from(decstring) as i64) < 9223372036854775807 as i64 {
    } else {
        mpd_c2rust_assert_fail(
            b"cp-decstring < MPD_SSIZE_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *cp = '\0' as i32 as ::core::ffi::c_char;
    *result = decstring;
    return cp.offset_from(decstring) as i64 as mpd_ssize_t;
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_to_string(
    mut result: *mut *mut ::core::ffi::c_char,
    mut dec: *const mpd_t,
    mut flags: ::core::ffi::c_int,
    mut dplace: mpd_ssize_t,
) -> mpd_ssize_t {
    let mut decstring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ldigits: mpd_ssize_t = 0;
    let mut mem: mpd_ssize_t = 0 as mpd_ssize_t;
    let mut k: mpd_ssize_t = 0;
    if mpd_isspecial(dec) != 0 {
        mem = ::core::mem::size_of::<[::core::ffi::c_char; 11]>() as mpd_ssize_t;
        if mpd_isnan(dec) != 0 && (*dec).len > 0 as mpd_ssize_t {
            mem += (*dec).digits;
        }
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as mpd_ssize_t);
        }
        if mpd_isnegative(dec) != 0 {
            let fresh0 = cp;
            cp = cp.offset(1);
            *fresh0 = '-' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            let fresh1 = cp;
            cp = cp.offset(1);
            *fresh1 = ' ' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            let fresh2 = cp;
            cp = cp.offset(1);
            *fresh2 = '+' as i32 as ::core::ffi::c_char;
        }
        if mpd_isnan(dec) != 0 {
            if mpd_isqnan(dec) != 0 {
                strcpy(cp, b"NaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(3 as isize);
            } else {
                strcpy(cp, b"sNaN\0" as *const u8 as *const ::core::ffi::c_char);
                cp = cp.offset(4 as isize);
            }
            if (*dec).len > 0 as mpd_ssize_t {
                cp = coeff_to_string(cp, dec);
            }
        } else if mpd_isinfinite(dec) != 0 {
            strcpy(cp, b"Infinity\0" as *const u8 as *const ::core::ffi::c_char);
            cp = cp.offset(8 as isize);
        } else {
            abort();
        }
    } else {
        if (*dec).len > 0 as mpd_ssize_t {
        } else {
            mpd_c2rust_assert_fail(b"dec->len > 0\0" as *const u8 as *const ::core::ffi::c_char);
        };
        ldigits = (*dec).digits + (*dec).exp;
        if !(flags & MPD_FMT_EXP != 0) {
            if flags & MPD_FMT_FIXED != 0
                || (*dec).exp <= 0 as mpd_ssize_t && ldigits > -(6 as mpd_ssize_t)
            {
                dplace = ldigits;
            } else if flags & MPD_FMT_TOENG != 0 {
                if mpd_iszero(dec) != 0 {
                    dplace = -(1 as mpd_ssize_t)
                        + mod_mpd_ssize_t((*dec).exp + 2 as mpd_ssize_t, 3 as mpd_ssize_t);
                } else {
                    dplace += mod_mpd_ssize_t(ldigits - 1 as mpd_ssize_t, 3 as mpd_ssize_t);
                }
            }
        }
        if dplace <= 0 as mpd_ssize_t {
            mem = -dplace + (*dec).digits + 2 as mpd_ssize_t;
        } else if dplace >= (*dec).digits {
            mem = dplace;
        } else {
            mem = (*dec).digits;
        }
        mem = (mem as ::core::ffi::c_int
            + (MPD_EXPDIGITS + 1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int))
            as mpd_ssize_t;
        decstring = mpd_alloc(
            mem as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
        ) as *mut ::core::ffi::c_char;
        cp = decstring;
        if cp.is_null() {
            *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -(1 as mpd_ssize_t);
        }
        if mpd_isnegative(dec) != 0 && !(flags & MPD_FMT_SIGN_COERCE != 0 && mpd_iszero(dec) != 0) {
            let fresh3 = cp;
            cp = cp.offset(1);
            *fresh3 = '-' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_SPACE != 0 {
            let fresh4 = cp;
            cp = cp.offset(1);
            *fresh4 = ' ' as i32 as ::core::ffi::c_char;
        } else if flags & MPD_FMT_SIGN_PLUS != 0 {
            let fresh5 = cp;
            cp = cp.offset(1);
            *fresh5 = '+' as i32 as ::core::ffi::c_char;
        }
        if dplace <= 0 as mpd_ssize_t {
            let fresh6 = cp;
            cp = cp.offset(1);
            *fresh6 = '0' as i32 as ::core::ffi::c_char;
            let fresh7 = cp;
            cp = cp.offset(1);
            *fresh7 = '.' as i32 as ::core::ffi::c_char;
            k = 0 as mpd_ssize_t;
            while k < -dplace {
                let fresh8 = cp;
                cp = cp.offset(1);
                *fresh8 = '0' as i32 as ::core::ffi::c_char;
                k += 1;
            }
            cp = coeff_to_string(cp, dec);
        } else if dplace >= (*dec).digits {
            cp = coeff_to_string(cp, dec);
            k = 0 as mpd_ssize_t;
            while k < dplace - (*dec).digits {
                let fresh9 = cp;
                cp = cp.offset(1);
                *fresh9 = '0' as i32 as ::core::ffi::c_char;
                k += 1;
            }
        } else {
            cp = coeff_to_string_dot(cp, cp.offset(dplace as isize), dec);
        }
        if ldigits != dplace || flags & MPD_FMT_EXP != 0 {
            let fresh10 = cp;
            cp = cp.offset(1);
            *fresh10 = (if flags & MPD_FMT_UPPER != 0 {
                'E' as i32
            } else {
                'e' as i32
            }) as ::core::ffi::c_char;
            cp = exp_to_string(cp, ldigits - dplace);
        }
    }
    if flags & MPD_FMT_PERCENT != 0 {
        let fresh11 = cp;
        cp = cp.offset(1);
        *fresh11 = '%' as i32 as ::core::ffi::c_char;
    }
    if cp < decstring.offset(mem as isize) {
    } else {
        mpd_c2rust_assert_fail(b"cp < decstring+mem\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if (cp.offset_from(decstring) as ::core::ffi::c_int) < 2147483647 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"cp-decstring < MPD_SSIZE_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *cp = '\0' as i32 as ::core::ffi::c_char;
    *result = decstring;
    return cp.offset_from(decstring) as ::core::ffi::c_int as mpd_ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_to_sci(
    mut dec: *const mpd_t,
    mut fmt: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOSCI;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    _mpd_to_string(
        &raw mut res,
        dec,
        flags,
        MPD_DEFAULT_DOTPLACE as mpd_ssize_t,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_to_eng(
    mut dec: *const mpd_t,
    mut fmt: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOENG;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    _mpd_to_string(
        &raw mut res,
        dec,
        flags,
        MPD_DEFAULT_DOTPLACE as mpd_ssize_t,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_to_sci_size(
    mut res: *mut *mut ::core::ffi::c_char,
    mut dec: *const mpd_t,
    mut fmt: ::core::ffi::c_int,
) -> mpd_ssize_t {
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOSCI;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    return _mpd_to_string(res, dec, flags, MPD_DEFAULT_DOTPLACE as mpd_ssize_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_to_eng_size(
    mut res: *mut *mut ::core::ffi::c_char,
    mut dec: *const mpd_t,
    mut fmt: ::core::ffi::c_int,
) -> mpd_ssize_t {
    let mut flags: ::core::ffi::c_int = MPD_FMT_TOENG;
    flags |= if fmt != 0 {
        MPD_FMT_UPPER
    } else {
        MPD_FMT_LOWER
    };
    return _mpd_to_string(res, dec, flags, MPD_DEFAULT_DOTPLACE as mpd_ssize_t);
}
unsafe extern "C" fn _mpd_copy_utf8(
    mut dest: *mut ::core::ffi::c_char,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut cp: *const ::core::ffi::c_uchar = s as *const ::core::ffi::c_uchar;
    let mut lb: ::core::ffi::c_uchar = 0;
    let mut ub: ::core::ffi::c_uchar = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if *cp as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        *dest.offset(0 as isize) = '\0' as i32 as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    } else if *cp as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int {
        *dest.offset(0 as isize) = *cp as ::core::ffi::c_char;
        *dest.offset(1 as isize) = '\0' as i32 as ::core::ffi::c_char;
        return 1 as ::core::ffi::c_int;
    } else {
        if 0xc2 as ::core::ffi::c_int <= *cp as ::core::ffi::c_int
            && *cp as ::core::ffi::c_int <= 0xdf as ::core::ffi::c_int
        {
            lb = 0x80 as ::core::ffi::c_uchar;
            ub = 0xbf as ::core::ffi::c_uchar;
            count = 2 as ::core::ffi::c_int;
            current_block = 1;
        } else if *cp as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int {
            lb = 0xa0 as ::core::ffi::c_uchar;
            ub = 0xbf as ::core::ffi::c_uchar;
            count = 3 as ::core::ffi::c_int;
            current_block = 1;
        } else if *cp as ::core::ffi::c_int <= 0xec as ::core::ffi::c_int {
            lb = 0x80 as ::core::ffi::c_uchar;
            ub = 0xbf as ::core::ffi::c_uchar;
            count = 3 as ::core::ffi::c_int;
            current_block = 1;
        } else if *cp as ::core::ffi::c_int == 0xed as ::core::ffi::c_int {
            lb = 0x80 as ::core::ffi::c_uchar;
            ub = 0x9f as ::core::ffi::c_uchar;
            count = 3 as ::core::ffi::c_int;
            current_block = 1;
        } else if *cp as ::core::ffi::c_int <= 0xef as ::core::ffi::c_int {
            lb = 0x80 as ::core::ffi::c_uchar;
            ub = 0xbf as ::core::ffi::c_uchar;
            count = 3 as ::core::ffi::c_int;
            current_block = 1;
        } else if *cp as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int {
            lb = 0x90 as ::core::ffi::c_uchar;
            ub = 0xbf as ::core::ffi::c_uchar;
            count = 4 as ::core::ffi::c_int;
            current_block = 1;
        } else if *cp as ::core::ffi::c_int <= 0xf3 as ::core::ffi::c_int {
            lb = 0x80 as ::core::ffi::c_uchar;
            ub = 0xbf as ::core::ffi::c_uchar;
            count = 4 as ::core::ffi::c_int;
            current_block = 1;
        } else if *cp as ::core::ffi::c_int == 0xf4 as ::core::ffi::c_int {
            lb = 0x80 as ::core::ffi::c_uchar;
            ub = 0x8f as ::core::ffi::c_uchar;
            count = 4 as ::core::ffi::c_int;
            current_block = 1;
        } else {
            current_block = 2;
        }
        match current_block {
            1 => {
                let fresh0 = cp;
                cp = cp.offset(1);
                *dest.offset(0 as isize) = *fresh0 as ::core::ffi::c_char;
                if !((*cp as ::core::ffi::c_int) < lb as ::core::ffi::c_int
                    || (ub as ::core::ffi::c_int) < *cp as ::core::ffi::c_int)
                {
                    let fresh1 = cp;
                    cp = cp.offset(1);
                    *dest.offset(1 as isize) = *fresh1 as ::core::ffi::c_char;
                    i = 2 as ::core::ffi::c_int;
                    loop {
                        if !(i < count) {
                            current_block = 3;
                            break;
                        }
                        if (*cp as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
                            || (0xbf as ::core::ffi::c_int) < *cp as ::core::ffi::c_int
                        {
                            current_block = 2;
                            break;
                        }
                        let fresh2 = cp;
                        cp = cp.offset(1);
                        *dest.offset(i as isize) = *fresh2 as ::core::ffi::c_char;
                        i += 1;
                    }
                    match current_block {
                        2 => {}
                        _ => {
                            *dest.offset(i as isize) = '\0' as i32 as ::core::ffi::c_char;
                            return count;
                        }
                    }
                }
            }
            _ => {}
        }
        *dest.offset(0 as isize) = '\0' as i32 as ::core::ffi::c_char;
        return -(1 as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_validate_lconv(mut spec: *mut mpd_spec_t) -> ::core::ffi::c_int {
    let mut n: size_t = 0;
    let mut cp: *const ::core::ffi::c_char = (*spec).grouping;
    while *cp as ::core::ffi::c_int != '\0' as i32 {
        let fresh0 = cp;
        cp = cp.offset(1);
        if (*fresh0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    n = strlen((*spec).dot);
    if n == 0 as size_t || n > 4 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if strlen((*spec).sep) > 4 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn mpd_parse_fmt_str(
    mut spec: *mut mpd_spec_t,
    mut fmt: *const ::core::ffi::c_char,
    mut caps: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = fmt as *mut ::core::ffi::c_char;
    let mut have_align: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    (*spec).min_width = 0 as mpd_ssize_t;
    (*spec).prec = -(1 as ::core::ffi::c_int) as mpd_ssize_t;
    (*spec).type_0 = (if caps != 0 { 'G' as i32 } else { 'g' as i32 }) as ::core::ffi::c_char;
    (*spec).align = '>' as i32 as ::core::ffi::c_char;
    (*spec).sign = '-' as i32 as ::core::ffi::c_char;
    (*spec).sign_coerce = 0 as ::core::ffi::c_char;
    (*spec).dot = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).sep = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).grouping = b"\0" as *const u8 as *const ::core::ffi::c_char;
    n = _mpd_copy_utf8(&raw mut (*spec).fill as *mut ::core::ffi::c_char, cp);
    if n < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if *cp as ::core::ffi::c_int != 0
        && (*cp.offset(n as isize) as ::core::ffi::c_int == '<' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '>' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '=' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '^' as i32)
    {
        cp = cp.offset(n as isize);
        let fresh0 = cp;
        cp = cp.offset(1);
        (*spec).align = *fresh0;
        have_align = 1 as ::core::ffi::c_int;
    } else {
        (*spec).fill[0 as usize] = ' ' as i32 as ::core::ffi::c_char;
        (*spec).fill[1 as usize] = '\0' as i32 as ::core::ffi::c_char;
        if *cp as ::core::ffi::c_int == '<' as i32
            || *cp as ::core::ffi::c_int == '>' as i32
            || *cp as ::core::ffi::c_int == '=' as i32
            || *cp as ::core::ffi::c_int == '^' as i32
        {
            let fresh1 = cp;
            cp = cp.offset(1);
            (*spec).align = *fresh1;
            have_align = 1 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == '+' as i32
        || *cp as ::core::ffi::c_int == '-' as i32
        || *cp as ::core::ffi::c_int == ' ' as i32
    {
        let fresh2 = cp;
        cp = cp.offset(1);
        (*spec).sign = *fresh2;
    }
    if *cp as ::core::ffi::c_int == 'z' as i32 {
        (*spec).sign_coerce = 1 as ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '0' as i32 {
        if have_align != 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*spec).align = 'z' as i32 as ::core::ffi::c_char;
        let fresh3 = cp;
        cp = cp.offset(1);
        (*spec).fill[0 as usize] = *fresh3;
        (*spec).fill[1 as usize] = '\0' as i32 as ::core::ffi::c_char;
    }
    if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        if *cp as ::core::ffi::c_int == '0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).min_width =
            mpd_c2rust_strtoll(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == ',' as i32 {
        (*spec).dot = b".\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).sep = b",\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).grouping = b"\x03\x03\0" as *const u8 as *const ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '.' as i32 {
        cp = cp.offset(1);
        if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).prec = mpd_c2rust_strtoll(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == 'E' as i32
        || *cp as ::core::ffi::c_int == 'e' as i32
        || *cp as ::core::ffi::c_int == 'F' as i32
        || *cp as ::core::ffi::c_int == 'f' as i32
        || *cp as ::core::ffi::c_int == 'G' as i32
        || *cp as ::core::ffi::c_int == 'g' as i32
        || *cp as ::core::ffi::c_int == '%' as i32
    {
        let fresh4 = cp;
        cp = cp.offset(1);
        (*spec).type_0 = *fresh4;
    } else if *cp as ::core::ffi::c_int == 'N' as i32 || *cp as ::core::ffi::c_int == 'n' as i32 {
        let mut lc: *mut lconv = ::core::ptr::null_mut::<lconv>();
        if *(*spec).sep != 0 {
            return 0 as ::core::ffi::c_int;
        }
        let fresh5 = cp;
        cp = cp.offset(1);
        (*spec).type_0 = *fresh5;
        (*spec).type_0 = (if (*spec).type_0 as ::core::ffi::c_int == 'N' as i32 {
            'G' as i32
        } else {
            'g' as i32
        }) as ::core::ffi::c_char;
        lc = localeconv();
        (*spec).dot = (*lc).decimal_point;
        (*spec).sep = (*lc).thousands_sep;
        (*spec).grouping = (*lc).grouping;
        if mpd_validate_lconv(spec) < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int != '\0' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn mpd_parse_fmt_str(
    mut spec: *mut mpd_spec_t,
    mut fmt: *const ::core::ffi::c_char,
    mut caps: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = fmt as *mut ::core::ffi::c_char;
    let mut have_align: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    (*spec).min_width = 0 as mpd_ssize_t;
    (*spec).prec = -(1 as ::core::ffi::c_int) as mpd_ssize_t;
    (*spec).type_0 = (if caps != 0 { 'G' as i32 } else { 'g' as i32 }) as ::core::ffi::c_char;
    (*spec).align = '>' as i32 as ::core::ffi::c_char;
    (*spec).sign = '-' as i32 as ::core::ffi::c_char;
    (*spec).sign_coerce = 0 as ::core::ffi::c_char;
    (*spec).dot = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).sep = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*spec).grouping = b"\0" as *const u8 as *const ::core::ffi::c_char;
    n = _mpd_copy_utf8(&raw mut (*spec).fill as *mut ::core::ffi::c_char, cp);
    if n < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if *cp as ::core::ffi::c_int != 0
        && (*cp.offset(n as isize) as ::core::ffi::c_int == '<' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '>' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '=' as i32
            || *cp.offset(n as isize) as ::core::ffi::c_int == '^' as i32)
    {
        cp = cp.offset(n as isize);
        let fresh0 = cp;
        cp = cp.offset(1);
        (*spec).align = *fresh0;
        have_align = 1 as ::core::ffi::c_int;
    } else {
        (*spec).fill[0 as usize] = ' ' as i32 as ::core::ffi::c_char;
        (*spec).fill[1 as usize] = '\0' as i32 as ::core::ffi::c_char;
        if *cp as ::core::ffi::c_int == '<' as i32
            || *cp as ::core::ffi::c_int == '>' as i32
            || *cp as ::core::ffi::c_int == '=' as i32
            || *cp as ::core::ffi::c_int == '^' as i32
        {
            let fresh1 = cp;
            cp = cp.offset(1);
            (*spec).align = *fresh1;
            have_align = 1 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == '+' as i32
        || *cp as ::core::ffi::c_int == '-' as i32
        || *cp as ::core::ffi::c_int == ' ' as i32
    {
        let fresh2 = cp;
        cp = cp.offset(1);
        (*spec).sign = *fresh2;
    }
    if *cp as ::core::ffi::c_int == 'z' as i32 {
        (*spec).sign_coerce = 1 as ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '0' as i32 {
        if have_align != 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*spec).align = 'z' as i32 as ::core::ffi::c_char;
        let fresh3 = cp;
        cp = cp.offset(1);
        (*spec).fill[0 as usize] = *fresh3;
        (*spec).fill[1 as usize] = '\0' as i32 as ::core::ffi::c_char;
    }
    if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        if *cp as ::core::ffi::c_int == '0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).min_width =
            mpd_c2rust_strtol(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == ',' as i32 {
        (*spec).dot = b".\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).sep = b",\0" as *const u8 as *const ::core::ffi::c_char;
        (*spec).grouping = b"\x03\x03\0" as *const u8 as *const ::core::ffi::c_char;
        cp = cp.offset(1);
    }
    if *cp as ::core::ffi::c_int == '.' as i32 {
        cp = cp.offset(1);
        if isdigit(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        *mpd_c2rust_errno() = 0 as ::core::ffi::c_int;
        (*spec).prec = mpd_c2rust_strtol(cp, &raw mut cp, 10 as ::core::ffi::c_int) as mpd_ssize_t;
        if *mpd_c2rust_errno() == ERANGE || *mpd_c2rust_errno() == EINVAL {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int == 'E' as i32
        || *cp as ::core::ffi::c_int == 'e' as i32
        || *cp as ::core::ffi::c_int == 'F' as i32
        || *cp as ::core::ffi::c_int == 'f' as i32
        || *cp as ::core::ffi::c_int == 'G' as i32
        || *cp as ::core::ffi::c_int == 'g' as i32
        || *cp as ::core::ffi::c_int == '%' as i32
    {
        let fresh4 = cp;
        cp = cp.offset(1);
        (*spec).type_0 = *fresh4;
    } else if *cp as ::core::ffi::c_int == 'N' as i32 || *cp as ::core::ffi::c_int == 'n' as i32 {
        let mut lc: *mut lconv = ::core::ptr::null_mut::<lconv>();
        if *(*spec).sep != 0 {
            return 0 as ::core::ffi::c_int;
        }
        let fresh5 = cp;
        cp = cp.offset(1);
        (*spec).type_0 = *fresh5;
        (*spec).type_0 = (if (*spec).type_0 as ::core::ffi::c_int == 'N' as i32 {
            'G' as i32
        } else {
            'g' as i32
        }) as ::core::ffi::c_char;
        lc = localeconv();
        (*spec).dot = (*lc).decimal_point;
        (*spec).sep = (*lc).thousands_sep;
        (*spec).grouping = (*lc).grouping;
        if mpd_validate_lconv(spec) < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *cp as ::core::ffi::c_int != '\0' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_bcopy(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut n: mpd_ssize_t,
) {
    loop {
        n -= 1;
        if !(n >= 0 as mpd_ssize_t) {
            break;
        }
        *dest.offset(n as isize) = *src.offset(n as isize);
    }
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mbstr_copy_char(
    mut dest: *mut mpd_mbstr_t,
    mut src: *const ::core::ffi::c_char,
    mut n: mpd_ssize_t,
) {
    (*dest).nbytes += n;
    (*dest).nchars += (if n > 0 as mpd_ssize_t {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        _mpd_bcopy((*dest).data.offset((*dest).cur as isize), src, n);
    }
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mbstr_copy_char(
    mut dest: *mut mpd_mbstr_t,
    mut src: *const ::core::ffi::c_char,
    mut n: mpd_ssize_t,
) {
    (*dest).nbytes += n;
    (*dest).nchars = ((*dest).nchars as ::core::ffi::c_int
        + if n > 0 as mpd_ssize_t {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as mpd_ssize_t;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        _mpd_bcopy((*dest).data.offset((*dest).cur as isize), src, n);
    }
}
unsafe extern "C" fn _mbstr_copy_ascii(
    mut dest: *mut mpd_mbstr_t,
    mut src: *const ::core::ffi::c_char,
    mut n: mpd_ssize_t,
) {
    (*dest).nbytes += n;
    (*dest).nchars += n;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        _mpd_bcopy((*dest).data.offset((*dest).cur as isize), src, n);
    }
}
unsafe extern "C" fn _mbstr_copy_pad(mut dest: *mut mpd_mbstr_t, mut n: mpd_ssize_t) {
    (*dest).nbytes += n;
    (*dest).nchars += n;
    (*dest).cur -= n;
    if !(*dest).data.is_null() {
        let mut cp: *mut ::core::ffi::c_char = (*dest).data.offset((*dest).cur as isize);
        loop {
            n -= 1;
            if !(n >= 0 as mpd_ssize_t) {
                break;
            }
            *cp.offset(n as isize) = '0' as i32 as ::core::ffi::c_char;
        }
    }
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_add_sep_dot(
    mut dest: *mut mpd_mbstr_t,
    mut sign: *const ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut n_src: mpd_ssize_t,
    mut dot: *const ::core::ffi::c_char,
    mut rest: *const ::core::ffi::c_char,
    mut n_rest: mpd_ssize_t,
    mut spec: *const mpd_spec_t,
) {
    let mut n_sep: mpd_ssize_t = 0;
    let mut n_sign: mpd_ssize_t = 0;
    let mut consume: mpd_ssize_t = 0;
    let mut g: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    n_sign = (if !sign.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    n_sep = strlen((*spec).sep) as mpd_ssize_t;
    (*dest).cur = (*dest).nbytes;
    (*dest).nchars = 0 as mpd_ssize_t;
    (*dest).nbytes = (*dest).nchars;
    _mbstr_copy_ascii(dest, rest, n_rest);
    if !dot.is_null() {
        _mbstr_copy_char(dest, dot, strlen(dot) as mpd_ssize_t);
    }
    g = (*spec).grouping;
    consume = *g as mpd_ssize_t;
    loop {
        if *g as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *g as ::core::ffi::c_int == CHAR_MAX
            || consume > n_src
        {
            consume = n_src;
        }
        n_src -= consume;
        if pad != 0 {
            _mbstr_copy_pad(dest, consume);
        } else {
            _mbstr_copy_ascii(dest, src.offset(n_src as isize), consume);
        }
        if n_src == 0 as mpd_ssize_t {
            if !((*spec).align as ::core::ffi::c_int == 'z' as i32
                && (*dest).nchars + n_sign < (*spec).min_width)
            {
                break;
            }
            n_src = (*spec).min_width - ((*dest).nchars + n_sign);
            consume = *g as mpd_ssize_t - consume;
            pad = 1 as ::core::ffi::c_int;
        } else {
            if n_sep > 0 as mpd_ssize_t {
                if pad != 0 && n_src > 1 as mpd_ssize_t {
                    n_src -= 1 as mpd_ssize_t;
                }
                _mbstr_copy_char(dest, (*spec).sep, n_sep);
            }
            if *g as ::core::ffi::c_int != 0 && *g.offset(1 as isize) as ::core::ffi::c_int != 0 {
                g = g.offset(1);
            }
            consume = *g as mpd_ssize_t;
        }
    }
    if !sign.is_null() {
        _mbstr_copy_ascii(dest, sign, 1 as mpd_ssize_t);
    }
    if !(*dest).data.is_null() {
        *(*dest).data.offset((*dest).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_add_sep_dot(
    mut dest: *mut mpd_mbstr_t,
    mut sign: *const ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut n_src: mpd_ssize_t,
    mut dot: *const ::core::ffi::c_char,
    mut rest: *const ::core::ffi::c_char,
    mut n_rest: mpd_ssize_t,
    mut spec: *const mpd_spec_t,
) {
    let mut n_sep: mpd_ssize_t = 0;
    let mut n_sign: mpd_ssize_t = 0;
    let mut consume: mpd_ssize_t = 0;
    let mut g: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    n_sign = (if !sign.is_null() {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    n_sep = strlen((*spec).sep) as mpd_ssize_t;
    (*dest).cur = (*dest).nbytes;
    (*dest).nchars = 0 as mpd_ssize_t;
    (*dest).nbytes = (*dest).nchars;
    _mbstr_copy_ascii(dest, rest, n_rest);
    if !dot.is_null() {
        _mbstr_copy_char(dest, dot, strlen(dot) as mpd_ssize_t);
    }
    g = (*spec).grouping;
    consume = *g as mpd_ssize_t;
    loop {
        if *g as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *g as ::core::ffi::c_int == CHAR_MAX
            || consume > n_src
        {
            consume = n_src;
        }
        n_src -= consume;
        if pad != 0 {
            _mbstr_copy_pad(dest, consume);
        } else {
            _mbstr_copy_ascii(dest, src.offset(n_src as isize), consume);
        }
        if n_src == 0 as mpd_ssize_t {
            if !((*spec).align as ::core::ffi::c_int == 'z' as i32
                && (*dest).nchars + n_sign < (*spec).min_width)
            {
                break;
            }
            n_src = (*spec).min_width - ((*dest).nchars + n_sign);
            consume = *g as mpd_ssize_t - consume;
            pad = 1 as ::core::ffi::c_int;
        } else {
            if n_sep > 0 as mpd_ssize_t {
                if pad != 0 && n_src > 1 as mpd_ssize_t {
                    n_src = (n_src as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as mpd_ssize_t;
                }
                _mbstr_copy_char(dest, (*spec).sep, n_sep);
            }
            if *g as ::core::ffi::c_int != 0 && *g.offset(1 as isize) as ::core::ffi::c_int != 0 {
                g = g.offset(1);
            }
            consume = *g as mpd_ssize_t;
        }
    }
    if !sign.is_null() {
        _mbstr_copy_ascii(dest, sign, 1 as mpd_ssize_t);
    }
    if !(*dest).data.is_null() {
        *(*dest).data.offset((*dest).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_apply_lconv(
    mut result: *mut mpd_mbstr_t,
    mut spec: *const mpd_spec_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut sign: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut intpart: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut decstring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n_int: mpd_ssize_t = 0;
    let mut n_rest: mpd_ssize_t = 0;
    dp = (*result).data;
    if *dp as ::core::ffi::c_int == '+' as i32
        || *dp as ::core::ffi::c_int == '-' as i32
        || *dp as ::core::ffi::c_int == ' ' as i32
    {
        let fresh0 = dp;
        dp = dp.offset(1);
        sign = fresh0;
    }
    if isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"isdigit((unsigned char)*dp)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let fresh1 = dp;
    dp = dp.offset(1);
    intpart = fresh1;
    while isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        dp = dp.offset(1);
    }
    n_int = dp.offset_from(intpart) as i64 as mpd_ssize_t;
    if *dp as ::core::ffi::c_int == '.' as i32 {
        dp = dp.offset(1);
        dot = (*spec).dot;
    }
    rest = dp;
    n_rest = (*result).nbytes - dp.offset_from((*result).data) as i64 as mpd_ssize_t;
    if dot.is_null()
        && (*(*spec).sep as ::core::ffi::c_int == '\0' as i32
            || *(*spec).grouping as ::core::ffi::c_int == '\0' as i32)
    {
        return 1 as ::core::ffi::c_int;
    }
    decstring = (*result).data;
    (*result).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    (*result).data = mpd_alloc(
        ((*result).nbytes + 1 as mpd_ssize_t) as mpd_size_t,
        1 as mpd_size_t,
    ) as *mut ::core::ffi::c_char;
    if (*result).data.is_null() {
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_apply_lconv(
    mut result: *mut mpd_mbstr_t,
    mut spec: *const mpd_spec_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut sign: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut intpart: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rest: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut decstring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n_int: mpd_ssize_t = 0;
    let mut n_rest: mpd_ssize_t = 0;
    dp = (*result).data;
    if *dp as ::core::ffi::c_int == '+' as i32
        || *dp as ::core::ffi::c_int == '-' as i32
        || *dp as ::core::ffi::c_int == ' ' as i32
    {
        let fresh0 = dp;
        dp = dp.offset(1);
        sign = fresh0;
    }
    if isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"isdigit((unsigned char)*dp)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let fresh1 = dp;
    dp = dp.offset(1);
    intpart = fresh1;
    while isdigit(*dp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        dp = dp.offset(1);
    }
    n_int = dp.offset_from(intpart) as ::core::ffi::c_int as mpd_ssize_t;
    if *dp as ::core::ffi::c_int == '.' as i32 {
        dp = dp.offset(1);
        dot = (*spec).dot;
    }
    rest = dp;
    n_rest = (*result).nbytes - dp.offset_from((*result).data) as ::core::ffi::c_int as mpd_ssize_t;
    if dot.is_null()
        && (*(*spec).sep as ::core::ffi::c_int == '\0' as i32
            || *(*spec).grouping as ::core::ffi::c_int == '\0' as i32)
    {
        return 1 as ::core::ffi::c_int;
    }
    decstring = (*result).data;
    (*result).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    (*result).data = mpd_alloc(
        ((*result).nbytes + 1 as mpd_ssize_t) as mpd_size_t,
        1 as mpd_size_t,
    ) as *mut ::core::ffi::c_char;
    if (*result).data.is_null() {
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    _mpd_add_sep_dot(result, sign, intpart, n_int, dot, rest, n_rest, spec);
    mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn _mpd_add_pad(
    mut result: *mut mpd_mbstr_t,
    mut spec: *const mpd_spec_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if (*result).nchars < (*spec).min_width {
        let mut add_chars: mpd_ssize_t = 0;
        let mut add_bytes: mpd_ssize_t = 0;
        let mut lpad: size_t = 0 as size_t;
        let mut rpad: size_t = 0 as size_t;
        let mut n_fill: size_t = 0;
        let mut len: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut align: ::core::ffi::c_char = (*spec).align;
        let mut err: uint8_t = 0 as uint8_t;
        let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        n_fill = strlen(&raw const (*spec).fill as *const ::core::ffi::c_char);
        add_chars = (*spec).min_width - (*result).nchars;
        add_bytes = add_chars * n_fill as mpd_ssize_t;
        (*result).data = mpd_realloc(
            (*result).data as *mut ::core::ffi::c_void,
            ((*result).nbytes + add_bytes + 1 as mpd_ssize_t) as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
            &raw mut err,
        ) as *mut ::core::ffi::c_char;
        cp = (*result).data;
        if err != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            mpd_free.expect("non-null function pointer")(
                (*result).data as *mut ::core::ffi::c_void,
            );
            return 0 as ::core::ffi::c_int;
        }
        if align as ::core::ffi::c_int == 'z' as i32 {
            align = '=' as i32 as ::core::ffi::c_char;
        }
        if align as ::core::ffi::c_int == '<' as i32 {
            rpad = add_chars as size_t;
        } else if align as ::core::ffi::c_int == '>' as i32
            || align as ::core::ffi::c_int == '=' as i32
        {
            lpad = add_chars as size_t;
        } else {
            lpad = (add_chars / 2 as mpd_ssize_t) as size_t;
            rpad = (add_chars as u64).wrapping_sub(lpad as u64) as size_t;
        }
        len = (*result).nbytes as size_t;
        if align as ::core::ffi::c_int == '=' as i32
            && (*cp as ::core::ffi::c_int == '-' as i32
                || *cp as ::core::ffi::c_int == '+' as i32
                || *cp as ::core::ffi::c_int == ' ' as i32)
        {
            cp = cp.offset(1);
            len = len.wrapping_sub(1);
        }
        memmove(
            cp.offset(n_fill.wrapping_mul(lpad) as isize) as *mut ::core::ffi::c_void,
            cp as *const ::core::ffi::c_void,
            len,
        );
        i = 0 as size_t;
        while i < lpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.offset(i.wrapping_mul(n_fill).wrapping_add(j) as isize) =
                    (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        cp = cp.offset(n_fill.wrapping_mul(lpad).wrapping_add(len) as isize);
        i = 0 as size_t;
        while i < rpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.offset(i.wrapping_mul(n_fill).wrapping_add(j) as isize) =
                    (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*result).nbytes += add_bytes;
        (*result).nchars += add_chars;
        *(*result).data.offset((*result).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn _mpd_add_pad(
    mut result: *mut mpd_mbstr_t,
    mut spec: *const mpd_spec_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if (*result).nchars < (*spec).min_width {
        let mut add_chars: mpd_ssize_t = 0;
        let mut add_bytes: mpd_ssize_t = 0;
        let mut lpad: size_t = 0 as size_t;
        let mut rpad: size_t = 0 as size_t;
        let mut n_fill: size_t = 0;
        let mut len: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut align: ::core::ffi::c_char = (*spec).align;
        let mut err: uint8_t = 0 as uint8_t;
        let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        n_fill = strlen(&raw const (*spec).fill as *const ::core::ffi::c_char);
        add_chars = (*spec).min_width - (*result).nchars;
        add_bytes = add_chars * n_fill as mpd_ssize_t;
        (*result).data = mpd_realloc(
            (*result).data as *mut ::core::ffi::c_void,
            ((*result).nbytes + add_bytes + 1 as mpd_ssize_t) as mpd_size_t,
            ::core::mem::size_of::<::core::ffi::c_char>() as mpd_size_t,
            &raw mut err,
        ) as *mut ::core::ffi::c_char;
        cp = (*result).data;
        if err != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            mpd_free.expect("non-null function pointer")(
                (*result).data as *mut ::core::ffi::c_void,
            );
            return 0 as ::core::ffi::c_int;
        }
        if align as ::core::ffi::c_int == 'z' as i32 {
            align = '=' as i32 as ::core::ffi::c_char;
        }
        if align as ::core::ffi::c_int == '<' as i32 {
            rpad = add_chars as size_t;
        } else if align as ::core::ffi::c_int == '>' as i32
            || align as ::core::ffi::c_int == '=' as i32
        {
            lpad = add_chars as size_t;
        } else {
            lpad = (add_chars / 2 as mpd_ssize_t) as size_t;
            rpad = (add_chars as size_t).wrapping_sub(lpad);
        }
        len = (*result).nbytes as size_t;
        if align as ::core::ffi::c_int == '=' as i32
            && (*cp as ::core::ffi::c_int == '-' as i32
                || *cp as ::core::ffi::c_int == '+' as i32
                || *cp as ::core::ffi::c_int == ' ' as i32)
        {
            cp = cp.offset(1);
            len = len.wrapping_sub(1);
        }
        memmove(
            cp.offset(n_fill.wrapping_mul(lpad) as isize) as *mut ::core::ffi::c_void,
            cp as *const ::core::ffi::c_void,
            len,
        );
        i = 0 as size_t;
        while i < lpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.offset(i.wrapping_mul(n_fill).wrapping_add(j) as isize) =
                    (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        cp = cp.offset(n_fill.wrapping_mul(lpad).wrapping_add(len) as isize);
        i = 0 as size_t;
        while i < rpad {
            j = 0 as size_t;
            while j < n_fill {
                *cp.offset(i.wrapping_mul(n_fill).wrapping_add(j) as isize) =
                    (*spec).fill[j as usize];
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        (*result).nbytes += add_bytes;
        (*result).nchars += add_chars;
        *(*result).data.offset((*result).nbytes as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_round(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut prec: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut exp: mpd_ssize_t = (*a).exp + (*a).digits - prec;
    if prec <= 0 as mpd_ssize_t {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_isspecial(a) != 0 || mpd_iszero(a) != 0 {
        mpd_qcopy(result, a, status);
        return;
    }
    mpd_qrescale_fmt(result, a, exp, ctx, status);
    if (*result).digits > prec {
        mpd_qrescale_fmt(result, result, exp + 1 as mpd_ssize_t, ctx, status);
    }
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn mpd_qformat_spec(
    mut dec: *const mpd_t,
    mut spec: *const mpd_spec_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut current_block: u64;
    let mut dt: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (MPD_STATIC as ::core::ffi::c_int | MPD_STATIC_DATA as ::core::ffi::c_int)
            as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut dt as *mut mpd_uint_t,
    };
    let mut dplace: mpd_ssize_t = MPD_DEFAULT_DOTPLACE as mpd_ssize_t;
    let mut result: mpd_mbstr_t = mpd_mbstr_t {
        nbytes: 0,
        nchars: 0,
        cur: 0,
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut stackspec: mpd_spec_t = mpd_spec_t {
        min_width: 0,
        prec: 0,
        type_0: 0,
        align: 0,
        sign: 0,
        sign_coerce: 0,
        fill: [0; 5],
        dot: ::core::ptr::null::<::core::ffi::c_char>(),
        sep: ::core::ptr::null::<::core::ffi::c_char>(),
        grouping: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut type_0: ::core::ffi::c_char = (*spec).type_0;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*spec).min_width > MPD_MAX_PREC as mpd_ssize_t {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if isupper(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        type_0 =
            tolower(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) as ::core::ffi::c_char;
        flags |= MPD_FMT_UPPER;
    }
    if (*spec).sign_coerce != 0 {
        flags |= MPD_FMT_SIGN_COERCE;
    }
    if (*spec).sign as ::core::ffi::c_int == ' ' as i32 {
        flags |= MPD_FMT_SIGN_SPACE;
    } else if (*spec).sign as ::core::ffi::c_int == '+' as i32 {
        flags |= MPD_FMT_SIGN_PLUS;
    }
    if mpd_isspecial(dec) != 0 {
        if (*spec).align as ::core::ffi::c_int == 'z' as i32 {
            stackspec = *spec;
            stackspec.fill[0 as usize] = ' ' as i32 as ::core::ffi::c_char;
            stackspec.fill[1 as usize] = '\0' as i32 as ::core::ffi::c_char;
            stackspec.align = '>' as i32 as ::core::ffi::c_char;
            spec = &raw mut stackspec;
        }
        if type_0 as ::core::ffi::c_int == '%' as i32 {
            flags |= MPD_FMT_PERCENT;
        }
        current_block = 1;
    } else {
        let mut workstatus: uint32_t = 0 as uint32_t;
        let mut prec: mpd_ssize_t = 0;
        let mut current_block_38: u64;
        match type_0 as ::core::ffi::c_int {
            103 => {
                flags |= MPD_FMT_TOSCI;
                current_block_38 = 2;
            }
            101 => {
                flags |= MPD_FMT_EXP;
                current_block_38 = 2;
            }
            37 => {
                flags |= MPD_FMT_PERCENT;
                if mpd_qcopy(&raw mut tmp, dec, status) == 0 {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                tmp.exp += 2 as mpd_ssize_t;
                dec = &raw mut tmp;
                type_0 = 'f' as i32 as ::core::ffi::c_char;
                current_block_38 = 3;
            }
            102 => {
                current_block_38 = 3;
            }
            _ => {
                abort();
            }
        }
        match current_block_38 {
            3 => {
                flags |= MPD_FMT_FIXED;
            }
            _ => {}
        }
        if (*spec).prec >= 0 as mpd_ssize_t {
            if (*spec).prec > MPD_MAX_PREC as mpd_ssize_t {
                *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                current_block = 4;
            } else {
                match type_0 as ::core::ffi::c_int {
                    103 => {
                        prec = if (*spec).prec == 0 as mpd_ssize_t {
                            1 as mpd_ssize_t
                        } else {
                            (*spec).prec
                        };
                        if (*dec).digits > prec {
                            _mpd_round(&raw mut tmp, dec, prec, ctx, &raw mut workstatus);
                            dec = &raw mut tmp;
                        }
                    }
                    101 => {
                        if mpd_iszero(dec) != 0 {
                            dplace = 1 as mpd_ssize_t - (*spec).prec;
                        } else {
                            _mpd_round(
                                &raw mut tmp,
                                dec,
                                (*spec).prec + 1 as mpd_ssize_t,
                                ctx,
                                &raw mut workstatus,
                            );
                            dec = &raw mut tmp;
                        }
                    }
                    102 => {
                        mpd_qrescale(&raw mut tmp, dec, -(*spec).prec, ctx, &raw mut workstatus);
                        dec = &raw mut tmp;
                    }
                    _ => {}
                }
                current_block = 5;
            }
        } else {
            current_block = 5;
        }
        match current_block {
            4 => {}
            _ => {
                if type_0 as ::core::ffi::c_int == 'f' as i32 {
                    if mpd_iszero(dec) != 0 && (*dec).exp > 0 as mpd_ssize_t {
                        mpd_qrescale(
                            &raw mut tmp,
                            dec,
                            0 as mpd_ssize_t,
                            ctx,
                            &raw mut workstatus,
                        );
                        dec = &raw mut tmp;
                    }
                }
                if workstatus & MPD_Errors as uint32_t != 0 {
                    *status = (*status as ::core::ffi::c_uint
                        | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
                        as uint32_t;
                    current_block = 4;
                } else {
                    current_block = 1;
                }
            }
        }
    }
    match current_block {
        1 => {
            result.nbytes = _mpd_to_string(&raw mut result.data, dec, flags, dplace);
            result.nchars = result.nbytes;
            if result.nbytes < 0 as mpd_ssize_t {
                *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            } else {
                if *(*spec).dot as ::core::ffi::c_int != '\0' as i32 && mpd_isspecial(dec) == 0 {
                    if result.nchars > MPD_MAX_PREC as mpd_ssize_t + 36 as mpd_ssize_t {
                        *status =
                            (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                        mpd_free.expect("non-null function pointer")(
                            result.data as *mut ::core::ffi::c_void,
                        );
                        current_block = 4;
                    } else if _mpd_apply_lconv(&raw mut result, spec, status) == 0 {
                        current_block = 4;
                    } else {
                        current_block = 6;
                    }
                } else {
                    current_block = 6;
                }
                match current_block {
                    4 => {}
                    _ => {
                        if (*spec).min_width != 0 {
                            if _mpd_add_pad(&raw mut result, spec, status) == 0 {
                                current_block = 4;
                            } else {
                                current_block = 7;
                            }
                        } else {
                            current_block = 7;
                        }
                        match current_block {
                            4 => {}
                            _ => {
                                mpd_del(&raw mut tmp);
                                return result.data;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut tmp);
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn mpd_qformat_spec(
    mut dec: *const mpd_t,
    mut spec: *const mpd_spec_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut current_block: u64;
    let mut dt: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (MPD_STATIC as ::core::ffi::c_int | MPD_STATIC_DATA as ::core::ffi::c_int)
            as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut dt as *mut mpd_uint_t,
    };
    let mut dplace: mpd_ssize_t = MPD_DEFAULT_DOTPLACE as mpd_ssize_t;
    let mut result: mpd_mbstr_t = mpd_mbstr_t {
        nbytes: 0,
        nchars: 0,
        cur: 0,
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut stackspec: mpd_spec_t = mpd_spec_t {
        min_width: 0,
        prec: 0,
        type_0: 0,
        align: 0,
        sign: 0,
        sign_coerce: 0,
        fill: [0; 5],
        dot: ::core::ptr::null::<::core::ffi::c_char>(),
        sep: ::core::ptr::null::<::core::ffi::c_char>(),
        grouping: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut type_0: ::core::ffi::c_char = (*spec).type_0;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*spec).min_width > MPD_MAX_PREC as mpd_ssize_t {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if isupper(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        type_0 =
            tolower(type_0 as ::core::ffi::c_uchar as ::core::ffi::c_int) as ::core::ffi::c_char;
        flags |= MPD_FMT_UPPER;
    }
    if (*spec).sign_coerce != 0 {
        flags |= MPD_FMT_SIGN_COERCE;
    }
    if (*spec).sign as ::core::ffi::c_int == ' ' as i32 {
        flags |= MPD_FMT_SIGN_SPACE;
    } else if (*spec).sign as ::core::ffi::c_int == '+' as i32 {
        flags |= MPD_FMT_SIGN_PLUS;
    }
    if mpd_isspecial(dec) != 0 {
        if (*spec).align as ::core::ffi::c_int == 'z' as i32 {
            stackspec = *spec;
            stackspec.fill[0 as usize] = ' ' as i32 as ::core::ffi::c_char;
            stackspec.fill[1 as usize] = '\0' as i32 as ::core::ffi::c_char;
            stackspec.align = '>' as i32 as ::core::ffi::c_char;
            spec = &raw mut stackspec;
        }
        if type_0 as ::core::ffi::c_int == '%' as i32 {
            flags |= MPD_FMT_PERCENT;
        }
        current_block = 1;
    } else {
        let mut workstatus: uint32_t = 0 as uint32_t;
        let mut prec: mpd_ssize_t = 0;
        let mut current_block_38: u64;
        match type_0 as ::core::ffi::c_int {
            103 => {
                flags |= MPD_FMT_TOSCI;
                current_block_38 = 2;
            }
            101 => {
                flags |= MPD_FMT_EXP;
                current_block_38 = 2;
            }
            37 => {
                flags |= MPD_FMT_PERCENT;
                if mpd_qcopy(&raw mut tmp, dec, status) == 0 {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                tmp.exp = (tmp.exp as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as mpd_ssize_t;
                dec = &raw mut tmp;
                type_0 = 'f' as i32 as ::core::ffi::c_char;
                current_block_38 = 3;
            }
            102 => {
                current_block_38 = 3;
            }
            _ => {
                abort();
            }
        }
        match current_block_38 {
            3 => {
                flags |= MPD_FMT_FIXED;
            }
            _ => {}
        }
        if (*spec).prec >= 0 as mpd_ssize_t {
            if (*spec).prec > MPD_MAX_PREC as mpd_ssize_t {
                *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                current_block = 4;
            } else {
                match type_0 as ::core::ffi::c_int {
                    103 => {
                        prec = if (*spec).prec == 0 as mpd_ssize_t {
                            1 as mpd_ssize_t
                        } else {
                            (*spec).prec
                        };
                        if (*dec).digits > prec {
                            _mpd_round(&raw mut tmp, dec, prec, ctx, &raw mut workstatus);
                            dec = &raw mut tmp;
                        }
                    }
                    101 => {
                        if mpd_iszero(dec) != 0 {
                            dplace = 1 as mpd_ssize_t - (*spec).prec;
                        } else {
                            _mpd_round(
                                &raw mut tmp,
                                dec,
                                (*spec).prec + 1 as mpd_ssize_t,
                                ctx,
                                &raw mut workstatus,
                            );
                            dec = &raw mut tmp;
                        }
                    }
                    102 => {
                        mpd_qrescale(&raw mut tmp, dec, -(*spec).prec, ctx, &raw mut workstatus);
                        dec = &raw mut tmp;
                    }
                    _ => {}
                }
                current_block = 5;
            }
        } else {
            current_block = 5;
        }
        match current_block {
            4 => {}
            _ => {
                if type_0 as ::core::ffi::c_int == 'f' as i32 {
                    if mpd_iszero(dec) != 0 && (*dec).exp > 0 as mpd_ssize_t {
                        mpd_qrescale(
                            &raw mut tmp,
                            dec,
                            0 as mpd_ssize_t,
                            ctx,
                            &raw mut workstatus,
                        );
                        dec = &raw mut tmp;
                    }
                }
                if workstatus & MPD_Errors as uint32_t != 0 {
                    *status = (*status as ::core::ffi::c_uint
                        | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
                        as uint32_t;
                    current_block = 4;
                } else {
                    current_block = 1;
                }
            }
        }
    }
    match current_block {
        1 => {
            result.nbytes = _mpd_to_string(&raw mut result.data, dec, flags, dplace);
            result.nchars = result.nbytes;
            if result.nbytes < 0 as mpd_ssize_t {
                *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            } else {
                if *(*spec).dot as ::core::ffi::c_int != '\0' as i32 && mpd_isspecial(dec) == 0 {
                    if result.nchars > MPD_MAX_PREC as mpd_ssize_t + 36 as mpd_ssize_t {
                        *status =
                            (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
                        mpd_free.expect("non-null function pointer")(
                            result.data as *mut ::core::ffi::c_void,
                        );
                        current_block = 4;
                    } else if _mpd_apply_lconv(&raw mut result, spec, status) == 0 {
                        current_block = 4;
                    } else {
                        current_block = 6;
                    }
                } else {
                    current_block = 6;
                }
                match current_block {
                    4 => {}
                    _ => {
                        if (*spec).min_width != 0 {
                            if _mpd_add_pad(&raw mut result, spec, status) == 0 {
                                current_block = 4;
                            } else {
                                current_block = 7;
                            }
                        } else {
                            current_block = 7;
                        }
                        match current_block {
                            4 => {}
                            _ => {
                                mpd_del(&raw mut tmp);
                                return result.data;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut tmp);
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qformat(
    mut dec: *const mpd_t,
    mut fmt: *const ::core::ffi::c_char,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    let mut spec: mpd_spec_t = mpd_spec_t {
        min_width: 0,
        prec: 0,
        type_0: 0,
        align: 0,
        sign: 0,
        sign_coerce: 0,
        fill: [0; 5],
        dot: ::core::ptr::null::<::core::ffi::c_char>(),
        sep: ::core::ptr::null::<::core::ffi::c_char>(),
        grouping: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if mpd_parse_fmt_str(&raw mut spec, fmt, 1 as ::core::ffi::c_int) == 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return mpd_qformat_spec(dec, &raw mut spec, ctx, status);
}
static mut mpd_flag_string: [*const ::core::ffi::c_char; 15] = [
    b"Clamped\0" as *const u8 as *const ::core::ffi::c_char,
    b"Conversion_syntax\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_by_zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_impossible\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_undefined\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fpu_error\0" as *const u8 as *const ::core::ffi::c_char,
    b"Inexact\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid_context\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Malloc_error\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not_implemented\0" as *const u8 as *const ::core::ffi::c_char,
    b"Overflow\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rounded\0" as *const u8 as *const ::core::ffi::c_char,
    b"Subnormal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Underflow\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut mpd_signal_string: [*const ::core::ffi::c_char; 15] = [
    b"Clamped\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Division_by_zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Inexact\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"IEEE_Invalid_operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not_implemented\0" as *const u8 as *const ::core::ffi::c_char,
    b"Overflow\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rounded\0" as *const u8 as *const ::core::ffi::c_char,
    b"Subnormal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Underflow\0" as *const u8 as *const ::core::ffi::c_char,
];
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn mpd_snprint_flags(
    mut dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    mut flags: uint32_t,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if nmemb >= 208 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *dest = '\0' as i32 as ::core::ffi::c_char;
    cp = dest;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s \0" as *const u8 as *const ::core::ffi::c_char,
                mpd_flag_string[j as usize],
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest {
        cp = cp.offset(-1);
        *cp = '\0' as i32 as ::core::ffi::c_char;
    }
    return cp.offset_from(dest) as i64 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn mpd_snprint_flags(
    mut dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    mut flags: uint32_t,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if nmemb >= 208 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    *dest = '\0' as i32 as ::core::ffi::c_char;
    cp = dest;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s \0" as *const u8 as *const ::core::ffi::c_char,
                mpd_flag_string[j as usize],
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest {
        cp = cp.offset(-1);
        *cp = '\0' as i32 as ::core::ffi::c_char;
    }
    return cp.offset_from(dest) as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn mpd_lsnprint_flags(
    mut dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    mut flags: uint32_t,
    mut flag_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if nmemb >= 208 as ::core::ffi::c_int + 18 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if flag_string.is_null() {
        flag_string = &raw mut mpd_flag_string as *mut *const ::core::ffi::c_char
            as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1 as isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1 as isize);
    nmemb -= 1;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                *flag_string.offset(j as isize),
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest.offset(1 as isize) {
        cp = cp.offset(-(2 as isize));
    }
    let fresh0 = cp;
    cp = cp.offset(1);
    *fresh0 = ']' as i32 as ::core::ffi::c_char;
    *cp = '\0' as i32 as ::core::ffi::c_char;
    return cp.offset_from(dest) as i64 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn mpd_lsnprint_flags(
    mut dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    mut flags: uint32_t,
    mut flag_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if nmemb >= 208 as ::core::ffi::c_int + 18 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_FLAG_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if flag_string.is_null() {
        flag_string = &raw mut mpd_flag_string as *mut *const ::core::ffi::c_char
            as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1 as isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1 as isize);
    nmemb -= 1;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        if flags & (1 as uint32_t) << j != 0 {
            n = snprintf(
                cp,
                nmemb as size_t,
                b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                *flag_string.offset(j as isize),
            );
            if n < 0 as ::core::ffi::c_int || n >= nmemb {
                return -(1 as ::core::ffi::c_int);
            }
            cp = cp.offset(n as isize);
            nmemb -= n;
        }
        j += 1;
    }
    if cp != dest.offset(1 as isize) {
        cp = cp.offset(-(2 as isize));
    }
    let fresh0 = cp;
    cp = cp.offset(1);
    *fresh0 = ']' as i32 as ::core::ffi::c_char;
    *cp = '\0' as i32 as ::core::ffi::c_char;
    return cp.offset_from(dest) as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn mpd_lsnprint_signals(
    mut dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    mut flags: uint32_t,
    mut signal_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ieee_invalid_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nmemb >= 121 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_SIGNAL_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if signal_string.is_null() {
        signal_string = &raw mut mpd_signal_string as *mut *const ::core::ffi::c_char
            as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1 as isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1 as isize);
    nmemb -= 1;
    let mut current_block_14: u64;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        let mut f: uint32_t = flags & (1 as uint32_t) << j;
        if f != 0 {
            if f & MPD_IEEE_Invalid_operation as uint32_t != 0 {
                if ieee_invalid_done != 0 {
                    current_block_14 = 1;
                } else {
                    ieee_invalid_done = 1 as ::core::ffi::c_int;
                    current_block_14 = 2;
                }
            } else {
                current_block_14 = 2;
            }
            match current_block_14 {
                1 => {}
                _ => {
                    n = snprintf(
                        cp,
                        nmemb as size_t,
                        b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                        *signal_string.offset(j as isize),
                    );
                    if n < 0 as ::core::ffi::c_int || n >= nmemb {
                        return -(1 as ::core::ffi::c_int);
                    }
                    cp = cp.offset(n as isize);
                    nmemb -= n;
                }
            }
        }
        j += 1;
    }
    if cp != dest.offset(1 as isize) {
        cp = cp.offset(-(2 as isize));
    }
    let fresh0 = cp;
    cp = cp.offset(1);
    *fresh0 = ']' as i32 as ::core::ffi::c_char;
    *cp = '\0' as i32 as ::core::ffi::c_char;
    return cp.offset_from(dest) as i64 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn mpd_lsnprint_signals(
    mut dest: *mut ::core::ffi::c_char,
    mut nmemb: ::core::ffi::c_int,
    mut flags: uint32_t,
    mut signal_string: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ieee_invalid_done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nmemb >= 121 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"nmemb >= MPD_MAX_SIGNAL_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if signal_string.is_null() {
        signal_string = &raw mut mpd_signal_string as *mut *const ::core::ffi::c_char
            as *mut *const ::core::ffi::c_char;
    }
    *dest = '[' as i32 as ::core::ffi::c_char;
    *dest.offset(1 as isize) = '\0' as i32 as ::core::ffi::c_char;
    cp = dest.offset(1 as isize);
    nmemb -= 1;
    let mut current_block_14: u64;
    j = 0 as ::core::ffi::c_int;
    while j < MPD_NUM_FLAGS {
        let mut f: uint32_t = flags & (1 as uint32_t) << j;
        if f != 0 {
            if f & MPD_IEEE_Invalid_operation as uint32_t != 0 {
                if ieee_invalid_done != 0 {
                    current_block_14 = 1;
                } else {
                    ieee_invalid_done = 1 as ::core::ffi::c_int;
                    current_block_14 = 2;
                }
            } else {
                current_block_14 = 2;
            }
            match current_block_14 {
                1 => {}
                _ => {
                    n = snprintf(
                        cp,
                        nmemb as size_t,
                        b"%s, \0" as *const u8 as *const ::core::ffi::c_char,
                        *signal_string.offset(j as isize),
                    );
                    if n < 0 as ::core::ffi::c_int || n >= nmemb {
                        return -(1 as ::core::ffi::c_int);
                    }
                    cp = cp.offset(n as isize);
                    nmemb -= n;
                }
            }
        }
        j += 1;
    }
    if cp != dest.offset(1 as isize) {
        cp = cp.offset(-(2 as isize));
    }
    let fresh0 = cp;
    cp = cp.offset(1);
    *fresh0 = ']' as i32 as ::core::ffi::c_char;
    *cp = '\0' as i32 as ::core::ffi::c_char;
    return cp.offset_from(dest) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_fprint(mut file: *mut FILE, mut dec: *const mpd_t) {
    let mut decstring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    decstring = mpd_to_sci(dec, 1 as ::core::ffi::c_int);
    if !decstring.is_null() {
        fprintf(
            file,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            decstring,
        );
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    } else {
        fputs(
            b"mpd_fprint: output error\n\0" as *const u8 as *const ::core::ffi::c_char,
            file,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_print(mut dec: *const mpd_t) {
    let mut decstring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    decstring = mpd_to_sci(dec, 1 as ::core::ffi::c_int);
    if !decstring.is_null() {
        printf(
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            decstring,
        );
        mpd_free.expect("non-null function pointer")(decstring as *mut ::core::ffi::c_void);
    } else {
        fputs(
            b"mpd_fprint: output error\n\0" as *const u8 as *const ::core::ffi::c_char,
            mpd_c2rust_stderr(),
        );
    };
}
