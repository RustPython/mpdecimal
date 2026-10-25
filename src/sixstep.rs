use crate::common::*;
use crate::shim::mpd_c2rust_assert_fail;
use crate::mpalloc::mpd_free;
use crate::constants::mpd_moduli;
use crate::numbertheory::_mpd_getkernel;
use crate::numbertheory::_mpd_init_fnt_params;
use crate::difradix2::fnt_dif2;
use crate::transpose::transpose_pow2;
unsafe extern "C" fn std_setmodulus(mut modnum: ::core::ffi::c_int, mut umod: *mut mpd_uint_t) {
    *umod = *(&raw const mpd_moduli as *const mpd_uint_t).offset(modnum as isize);
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn six_step_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    let mut log2n: mpd_size_t = 0;
    let mut C: mpd_size_t = 0;
    let mut R: mpd_size_t = 0;
    let mut kernel: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut x: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut k: mpd_size_t = 0;
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 16 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 16\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n as u64 <= 4294967296 as u64 {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    log2n = mpd_bsr(n) as mpd_size_t;
    C = (1 as mpd_size_t) << log2n.wrapping_div(2 as mpd_size_t);
    R = (1 as mpd_size_t) << log2n.wrapping_sub(log2n.wrapping_div(2 as mpd_size_t));
    if transpose_pow2(a, R, C) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    tparams = _mpd_init_fnt_params(R, -(1 as ::core::ffi::c_int), modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, R, tparams);
        x = x.offset(R as isize);
    }
    if transpose_pow2(a, C, R) == 0 {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    std_setmodulus(modnum, &raw mut umod);
    kernel = _mpd_getkernel(n as mpd_uint_t, -(1 as ::core::ffi::c_int), modnum);
    i = 1 as mpd_size_t;
    while i < R {
        w0 = 1 as mpd_uint_t;
        w1 = x64_powmod(kernel, i as mpd_uint_t, umod);
        wstep = x64_mulmod(w1, w1, umod);
        k = 0 as mpd_size_t;
        while k < C {
            let mut x0: mpd_uint_t = *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize);
            let mut x1: mpd_uint_t = *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            );
            x64_mulmod2(&raw mut x0, w0, &raw mut x1, w1, umod);
            x64_mulmod2c(&raw mut w0, &raw mut w1, wstep, umod);
            *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize) = x0;
            *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            ) = x1;
            k = k.wrapping_add(2 as mpd_size_t);
        }
        i = i.wrapping_add(1);
    }
    if C != R {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        tparams = _mpd_init_fnt_params(C, -(1 as ::core::ffi::c_int), modnum);
        if tparams.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, C, tparams);
        x = x.offset(C as isize);
    }
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn six_step_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    let mut log2n: mpd_size_t = 0;
    let mut C: mpd_size_t = 0;
    let mut R: mpd_size_t = 0;
    let mut kernel: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut x: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut k: mpd_size_t = 0;
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 16 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 16\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n <= 33554432 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    log2n = mpd_bsr(n) as mpd_size_t;
    C = (1 as mpd_size_t) << log2n.wrapping_div(2 as mpd_size_t);
    R = (1 as mpd_size_t) << log2n.wrapping_sub(log2n.wrapping_div(2 as mpd_size_t));
    if transpose_pow2(a, R, C) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    tparams = _mpd_init_fnt_params(R, -(1 as ::core::ffi::c_int), modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, R, tparams);
        x = x.offset(R as isize);
    }
    if transpose_pow2(a, C, R) == 0 {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    std_setmodulus(modnum, &raw mut umod);
    kernel = _mpd_getkernel(n as mpd_uint_t, -(1 as ::core::ffi::c_int), modnum);
    i = 1 as mpd_size_t;
    while i < R {
        w0 = 1 as mpd_uint_t;
        w1 = std_powmod(kernel, i as mpd_uint_t, umod);
        wstep = std_mulmod(w1, w1, umod);
        k = 0 as mpd_size_t;
        while k < C {
            let mut x0: mpd_uint_t = *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize);
            let mut x1: mpd_uint_t = *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            );
            std_mulmod2(&raw mut x0, w0, &raw mut x1, w1, umod);
            std_mulmod2c(&raw mut w0, &raw mut w1, wstep, umod);
            *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize) = x0;
            *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            ) = x1;
            k = k.wrapping_add(2 as mpd_size_t);
        }
        i = i.wrapping_add(1);
    }
    if C != R {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        tparams = _mpd_init_fnt_params(C, -(1 as ::core::ffi::c_int), modnum);
        if tparams.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, C, tparams);
        x = x.offset(C as isize);
    }
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn inv_six_step_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    let mut log2n: mpd_size_t = 0;
    let mut C: mpd_size_t = 0;
    let mut R: mpd_size_t = 0;
    let mut kernel: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut x: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut k: mpd_size_t = 0;
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 16 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 16\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n as u64 <= 4294967296 as u64 {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    log2n = mpd_bsr(n) as mpd_size_t;
    C = (1 as mpd_size_t) << log2n.wrapping_div(2 as mpd_size_t);
    R = (1 as mpd_size_t) << log2n.wrapping_sub(log2n.wrapping_div(2 as mpd_size_t));
    tparams = _mpd_init_fnt_params(C, 1 as ::core::ffi::c_int, modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, C, tparams);
        x = x.offset(C as isize);
    }
    std_setmodulus(modnum, &raw mut umod);
    kernel = _mpd_getkernel(n as mpd_uint_t, 1 as ::core::ffi::c_int, modnum);
    i = 1 as mpd_size_t;
    while i < R {
        w0 = 1 as mpd_uint_t;
        w1 = x64_powmod(kernel, i as mpd_uint_t, umod);
        wstep = x64_mulmod(w1, w1, umod);
        k = 0 as mpd_size_t;
        while k < C {
            let mut x0: mpd_uint_t = *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize);
            let mut x1: mpd_uint_t = *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            );
            x64_mulmod2(&raw mut x0, w0, &raw mut x1, w1, umod);
            x64_mulmod2c(&raw mut w0, &raw mut w1, wstep, umod);
            *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize) = x0;
            *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            ) = x1;
            k = k.wrapping_add(2 as mpd_size_t);
        }
        i = i.wrapping_add(1);
    }
    if transpose_pow2(a, R, C) == 0 {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    if R != C {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        tparams = _mpd_init_fnt_params(R, 1 as ::core::ffi::c_int, modnum);
        if tparams.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, R, tparams);
        x = x.offset(R as isize);
    }
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    if transpose_pow2(a, C, R) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn inv_six_step_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    let mut log2n: mpd_size_t = 0;
    let mut C: mpd_size_t = 0;
    let mut R: mpd_size_t = 0;
    let mut kernel: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut x: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut k: mpd_size_t = 0;
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 16 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 16\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n <= 33554432 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    log2n = mpd_bsr(n) as mpd_size_t;
    C = (1 as mpd_size_t) << log2n.wrapping_div(2 as mpd_size_t);
    R = (1 as mpd_size_t) << log2n.wrapping_sub(log2n.wrapping_div(2 as mpd_size_t));
    tparams = _mpd_init_fnt_params(C, 1 as ::core::ffi::c_int, modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, C, tparams);
        x = x.offset(C as isize);
    }
    std_setmodulus(modnum, &raw mut umod);
    kernel = _mpd_getkernel(n as mpd_uint_t, 1 as ::core::ffi::c_int, modnum);
    i = 1 as mpd_size_t;
    while i < R {
        w0 = 1 as mpd_uint_t;
        w1 = std_powmod(kernel, i as mpd_uint_t, umod);
        wstep = std_mulmod(w1, w1, umod);
        k = 0 as mpd_size_t;
        while k < C {
            let mut x0: mpd_uint_t = *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize);
            let mut x1: mpd_uint_t = *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            );
            std_mulmod2(&raw mut x0, w0, &raw mut x1, w1, umod);
            std_mulmod2c(&raw mut w0, &raw mut w1, wstep, umod);
            *a.offset(i.wrapping_mul(C).wrapping_add(k) as isize) = x0;
            *a.offset(
                i.wrapping_mul(C)
                    .wrapping_add(k)
                    .wrapping_add(1 as mpd_size_t) as isize,
            ) = x1;
            k = k.wrapping_add(2 as mpd_size_t);
        }
        i = i.wrapping_add(1);
    }
    if transpose_pow2(a, R, C) == 0 {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    if R != C {
        mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
        tparams = _mpd_init_fnt_params(R, 1 as ::core::ffi::c_int, modnum);
        if tparams.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
    x = a;
    while x < a.offset(n as isize) {
        fnt_dif2(x as *mut mpd_uint_t, R, tparams);
        x = x.offset(R as isize);
    }
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    if transpose_pow2(a, C, R) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
