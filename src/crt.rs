use crate::common::*;
use crate::shim::mpd_c2rust_assert_fail;
use crate::constants::mpd_moduli;
use crate::constants::INV_P1_MOD_P2;
use crate::constants::INV_P1P2_MOD_P3;
use crate::constants::LH_P1P2;
use crate::constants::UH_P1P2;
pub const P3: C2RustUnnamed = 2;
pub const P2: C2RustUnnamed = 1;
pub const P1: C2RustUnnamed = 0;
unsafe extern "C" fn std_setmodulus(mut modnum: ::core::ffi::c_int, mut umod: *mut mpd_uint_t) {
    *umod = *(&raw const mpd_moduli as *const mpd_uint_t).offset(modnum as isize);
}
unsafe extern "C" fn ext_submod(
    mut a: mpd_uint_t,
    mut b: mpd_uint_t,
    mut m: mpd_uint_t,
) -> mpd_uint_t {
    let mut d: mpd_uint_t = 0;
    a = if a >= m { a.wrapping_sub(m) } else { a };
    b = if b >= m { b.wrapping_sub(m) } else { b };
    d = a.wrapping_sub(b);
    d = if a < b { d.wrapping_add(m) } else { d };
    return d;
}
unsafe extern "C" fn dw_reduce(
    mut hi: mpd_uint_t,
    mut lo: mpd_uint_t,
    mut m: mpd_uint_t,
) -> mpd_uint_t {
    let mut r1: mpd_uint_t = 0;
    let mut r2: mpd_uint_t = 0;
    let mut w: mpd_uint_t = 0;
    _mpd_div_word(&raw mut w, &raw mut r1, hi, m);
    _mpd_div_words(&raw mut w, &raw mut r2, r1, lo, m);
    return r2;
}
unsafe extern "C" fn dw_submod(
    mut a: mpd_uint_t,
    mut hi: mpd_uint_t,
    mut lo: mpd_uint_t,
    mut m: mpd_uint_t,
) -> mpd_uint_t {
    let mut d: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    r = dw_reduce(hi, lo, m);
    d = a.wrapping_sub(r);
    d = if a < r { d.wrapping_add(m) } else { d };
    return d;
}
unsafe extern "C" fn _crt_mulP1P2_3(mut w: *mut mpd_uint_t, mut v: mpd_uint_t) {
    let mut hi1: mpd_uint_t = 0;
    let mut hi2: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi1, &raw mut lo, LH_P1P2, v);
    *w.offset(0 as isize) = lo;
    _mpd_mul_words(&raw mut hi2, &raw mut lo, UH_P1P2, v);
    lo = hi1.wrapping_add(lo);
    if lo < hi1 {
        hi2 = hi2.wrapping_add(1);
    }
    *w.offset(1 as isize) = lo;
    *w.offset(2 as isize) = hi2;
}
unsafe extern "C" fn _crt_add3(mut w: *mut mpd_uint_t, mut v: *mut mpd_uint_t) {
    let mut carry: mpd_uint_t = 0;
    *w.offset(0 as isize) = (*w.offset(0 as isize)).wrapping_add(*v.offset(0 as isize));
    carry = (*w.offset(0 as isize) < *v.offset(0 as isize)) as ::core::ffi::c_int as mpd_uint_t;
    *w.offset(1 as isize) = (*w.offset(1 as isize)).wrapping_add(*v.offset(1 as isize));
    if *w.offset(1 as isize) < *v.offset(1 as isize) {
        let ref mut fresh0 = *w.offset(2 as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
    }
    *w.offset(1 as isize) = (*w.offset(1 as isize)).wrapping_add(carry);
    if *w.offset(1 as isize) < carry {
        let ref mut fresh1 = *w.offset(2 as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
    }
    let ref mut fresh2 = *w.offset(2 as isize);
    *fresh2 = (*fresh2).wrapping_add(*v.offset(2 as isize));
}
unsafe extern "C" fn _crt_div3(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut v: mpd_uint_t,
) -> mpd_uint_t {
    let mut r1: mpd_uint_t = *u.offset(2 as isize);
    let mut r2: mpd_uint_t = 0;
    if r1 < v {
        *w.offset(2 as isize) = 0 as mpd_uint_t;
    } else {
        _mpd_div_word(
            w.offset(2 as isize) as *mut mpd_uint_t,
            &raw mut r1,
            *u.offset(2 as isize),
            v,
        );
    }
    _mpd_div_words(
        w.offset(1 as isize) as *mut mpd_uint_t,
        &raw mut r2,
        r1,
        *u.offset(1 as isize),
        v,
    );
    _mpd_div_words(
        w.offset(0 as isize) as *mut mpd_uint_t,
        &raw mut r1,
        r2,
        *u.offset(0 as isize),
        v,
    );
    return r1;
}
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn crt3(
    mut x1: *mut mpd_uint_t,
    mut x2: *mut mpd_uint_t,
    mut x3: *mut mpd_uint_t,
    mut rsize: mpd_size_t,
) {
    let mut p1: mpd_uint_t =
        *(&raw const mpd_moduli as *const mpd_uint_t).offset(P1 as ::core::ffi::c_int as isize);
    let mut umod: mpd_uint_t = 0;
    let mut a1: mpd_uint_t = 0;
    let mut a2: mpd_uint_t = 0;
    let mut a3: mpd_uint_t = 0;
    let mut s: mpd_uint_t = 0;
    let mut z: [mpd_uint_t; 3] = [0; 3];
    let mut t: [mpd_uint_t; 3] = [0; 3];
    let mut carry: [mpd_uint_t; 3] = [0 as mpd_uint_t, 0 as mpd_uint_t, 0 as mpd_uint_t];
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    i = 0 as mpd_size_t;
    while i < rsize {
        a1 = *x1.offset(i as isize);
        a2 = *x2.offset(i as isize);
        a3 = *x3.offset(i as isize);
        std_setmodulus(P2 as ::core::ffi::c_int, &raw mut umod);
        s = ext_submod(a2, a1, umod);
        s = x64_mulmod(s, INV_P1_MOD_P2, umod);
        _mpd_mul_words(&raw mut hi, &raw mut lo, s, p1);
        lo = lo.wrapping_add(a1);
        if lo < a1 {
            hi = hi.wrapping_add(1);
        }
        std_setmodulus(P3 as ::core::ffi::c_int, &raw mut umod);
        s = dw_submod(a3, hi, lo, umod);
        s = x64_mulmod(s, INV_P1P2_MOD_P3, umod);
        z[0 as usize] = lo;
        z[1 as usize] = hi;
        z[2 as usize] = 0 as mpd_uint_t;
        _crt_mulP1P2_3(&raw mut t as *mut mpd_uint_t, s);
        _crt_add3(&raw mut z as *mut mpd_uint_t, &raw mut t as *mut mpd_uint_t);
        _crt_add3(
            &raw mut carry as *mut mpd_uint_t,
            &raw mut z as *mut mpd_uint_t,
        );
        *x1.offset(i as isize) = _crt_div3(
            &raw mut carry as *mut mpd_uint_t,
            &raw mut carry as *mut mpd_uint_t,
            MPD_RADIX as mpd_uint_t,
        );
        i = i.wrapping_add(1);
    }
    if carry[0 as usize] == 0 as mpd_uint_t
        && carry[1 as usize] == 0 as mpd_uint_t
        && carry[2 as usize] == 0 as mpd_uint_t
    {
    } else {
        mpd_c2rust_assert_fail(
            b"carry[0] == 0 && carry[1] == 0 && carry[2] == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn crt3(
    mut x1: *mut mpd_uint_t,
    mut x2: *mut mpd_uint_t,
    mut x3: *mut mpd_uint_t,
    mut rsize: mpd_size_t,
) {
    let mut p1: mpd_uint_t =
        *(&raw const mpd_moduli as *const mpd_uint_t).offset(P1 as ::core::ffi::c_int as isize);
    let mut umod: mpd_uint_t = 0;
    let mut a1: mpd_uint_t = 0;
    let mut a2: mpd_uint_t = 0;
    let mut a3: mpd_uint_t = 0;
    let mut s: mpd_uint_t = 0;
    let mut z: [mpd_uint_t; 3] = [0; 3];
    let mut t: [mpd_uint_t; 3] = [0; 3];
    let mut carry: [mpd_uint_t; 3] = [0 as mpd_uint_t, 0 as mpd_uint_t, 0 as mpd_uint_t];
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    i = 0 as mpd_size_t;
    while i < rsize {
        a1 = *x1.offset(i as isize);
        a2 = *x2.offset(i as isize);
        a3 = *x3.offset(i as isize);
        std_setmodulus(P2 as ::core::ffi::c_int, &raw mut umod);
        s = ext_submod(a2, a1, umod);
        s = std_mulmod(s, INV_P1_MOD_P2, umod);
        _mpd_mul_words(&raw mut hi, &raw mut lo, s, p1);
        lo = lo.wrapping_add(a1);
        if lo < a1 {
            hi = hi.wrapping_add(1);
        }
        std_setmodulus(P3 as ::core::ffi::c_int, &raw mut umod);
        s = dw_submod(a3, hi, lo, umod);
        s = std_mulmod(s, INV_P1P2_MOD_P3, umod);
        z[0 as usize] = lo;
        z[1 as usize] = hi;
        z[2 as usize] = 0 as mpd_uint_t;
        _crt_mulP1P2_3(&raw mut t as *mut mpd_uint_t, s);
        _crt_add3(&raw mut z as *mut mpd_uint_t, &raw mut t as *mut mpd_uint_t);
        _crt_add3(
            &raw mut carry as *mut mpd_uint_t,
            &raw mut z as *mut mpd_uint_t,
        );
        *x1.offset(i as isize) = _crt_div3(
            &raw mut carry as *mut mpd_uint_t,
            &raw mut carry as *mut mpd_uint_t,
            MPD_RADIX as mpd_uint_t,
        );
        i = i.wrapping_add(1);
    }
    if carry[0 as usize] == 0 as mpd_uint_t
        && carry[1 as usize] == 0 as mpd_uint_t
        && carry[2 as usize] == 0 as mpd_uint_t
    {
    } else {
        mpd_c2rust_assert_fail(
            b"carry[0] == 0 && carry[1] == 0 && carry[2] == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
