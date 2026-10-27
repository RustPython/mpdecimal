use crate::common::*;
use crate::shim::mpd_c2rust_assert_fail;
use crate::mpalloc::mpd_free;
use crate::numbertheory::_mpd_init_fnt_params;
use crate::difradix2::fnt_dif2;
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn std_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n as u64 <= (3 as u64).wrapping_mul(4294967296 as u64) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    tparams = _mpd_init_fnt_params(n, -(1 as ::core::ffi::c_int), modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    fnt_dif2(a as *mut mpd_uint_t, n, tparams);
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn std_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n <= (3 as mpd_size_t).wrapping_mul(33554432 as mpd_size_t) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    tparams = _mpd_init_fnt_params(n, -(1 as ::core::ffi::c_int), modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    fnt_dif2(a as *mut mpd_uint_t, n, tparams);
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn std_inv_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n as u64 <= (3 as u64).wrapping_mul(4294967296 as u64) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    tparams = _mpd_init_fnt_params(n, 1 as ::core::ffi::c_int, modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    fnt_dif2(a as *mut mpd_uint_t, n, tparams);
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn std_inv_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n <= (3 as mpd_size_t).wrapping_mul(33554432 as mpd_size_t) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    tparams = _mpd_init_fnt_params(n, 1 as ::core::ffi::c_int, modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    fnt_dif2(a as *mut mpd_uint_t, n, tparams);
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
