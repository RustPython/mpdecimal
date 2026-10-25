use crate::common::*;
use crate::shim::mpd_c2rust_assert_fail;
use crate::constants::mpd_moduli;
#[cfg(target_pointer_width = "64")]
unsafe extern "C" fn mpd_bsf(mut n: mpd_size_t) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = 0;
    pos = 63 as ::core::ffi::c_int;
    if n as u64 & 0xffffffff as u64 != 0 {
        pos -= 32 as ::core::ffi::c_int;
    } else {
        n >>= 32 as ::core::ffi::c_int;
    }
    if n as u64 & 0xffff as u64 != 0 {
        pos -= 16 as ::core::ffi::c_int;
    } else {
        n >>= 16 as ::core::ffi::c_int;
    }
    if n as u64 & 0xff as u64 != 0 {
        pos -= 8 as ::core::ffi::c_int;
    } else {
        n >>= 8 as ::core::ffi::c_int;
    }
    if n as u64 & 0xf as u64 != 0 {
        pos -= 4 as ::core::ffi::c_int;
    } else {
        n >>= 4 as ::core::ffi::c_int;
    }
    if n as u64 & 0x3 as u64 != 0 {
        pos -= 2 as ::core::ffi::c_int;
    } else {
        n >>= 2 as ::core::ffi::c_int;
    }
    if n as u64 & 0x1 as u64 != 0 {
        pos -= 1 as ::core::ffi::c_int;
    }
    return pos;
}
#[cfg(target_pointer_width = "32")]
unsafe extern "C" fn mpd_bsf(mut n: mpd_size_t) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = 0;
    pos = 31 as ::core::ffi::c_int;
    if n as u32 & 0xffff as u32 != 0 {
        pos -= 16 as ::core::ffi::c_int;
    } else {
        n >>= 16 as ::core::ffi::c_int;
    }
    if n as u32 & 0xff as u32 != 0 {
        pos -= 8 as ::core::ffi::c_int;
    } else {
        n >>= 8 as ::core::ffi::c_int;
    }
    if n as u32 & 0xf as u32 != 0 {
        pos -= 4 as ::core::ffi::c_int;
    } else {
        n >>= 4 as ::core::ffi::c_int;
    }
    if n as u32 & 0x3 as u32 != 0 {
        pos -= 2 as ::core::ffi::c_int;
    } else {
        n >>= 2 as ::core::ffi::c_int;
    }
    if n as u32 & 0x1 as u32 != 0 {
        pos -= 1 as ::core::ffi::c_int;
    }
    return pos;
}
unsafe extern "C" fn std_setmodulus(mut modnum: ::core::ffi::c_int, mut umod: *mut mpd_uint_t) {
    *umod = *(&raw const mpd_moduli as *const mpd_uint_t).offset(modnum as isize);
}
unsafe extern "C" fn submod(mut a: mpd_uint_t, mut b: mpd_uint_t, mut m: mpd_uint_t) -> mpd_uint_t {
    let mut d: mpd_uint_t = 0;
    d = a.wrapping_sub(b);
    d = if a < b { d.wrapping_add(m) } else { d };
    return d;
}
unsafe extern "C" fn bitreverse_permute(mut a: *mut mpd_uint_t, mut n: mpd_size_t) {
    let mut x: mpd_size_t = 0 as mpd_size_t;
    let mut r: mpd_size_t = 0 as mpd_size_t;
    let mut t: mpd_uint_t = 0;
    loop {
        if r > x {
            t = *a.offset(x as isize);
            *a.offset(x as isize) = *a.offset(r as isize);
            *a.offset(r as isize) = t;
        }
        x = x.wrapping_add(1 as mpd_size_t);
        r ^= n.wrapping_sub(n >> mpd_bsf(x) + 1 as ::core::ffi::c_int);
        if !(x < n) {
            break;
        }
    }
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn fnt_dif2(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut tparams: *mut fnt_params,
) {
    let mut wtable: *mut mpd_uint_t = &raw mut (*tparams).wtable as *mut mpd_uint_t;
    let mut umod: mpd_uint_t = 0;
    let mut u0: mpd_uint_t = 0;
    let mut u1: mpd_uint_t = 0;
    let mut v0: mpd_uint_t = 0;
    let mut v1: mpd_uint_t = 0;
    let mut w: mpd_uint_t = 0;
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut m: mpd_size_t = 0;
    let mut mhalf: mpd_size_t = 0;
    let mut j: mpd_size_t = 0;
    let mut r: mpd_size_t = 0;
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    std_setmodulus((*tparams).modnum, &raw mut umod);
    mhalf = n.wrapping_div(2 as mpd_size_t);
    j = 0 as mpd_size_t;
    while j < mhalf {
        w0 = *wtable.offset(j as isize);
        w1 = *wtable.offset(j.wrapping_add(1 as mpd_size_t) as isize);
        u0 = *a.offset(j as isize);
        v0 = *a.offset(j.wrapping_add(mhalf) as isize);
        u1 = *a.offset(j.wrapping_add(1 as mpd_size_t) as isize);
        v1 = *a.offset(j.wrapping_add(1 as mpd_size_t).wrapping_add(mhalf) as isize);
        *a.offset(j as isize) = addmod(u0, v0, umod);
        v0 = submod(u0, v0, umod);
        *a.offset(j.wrapping_add(1 as mpd_size_t) as isize) = addmod(u1, v1, umod);
        v1 = submod(u1, v1, umod);
        x64_mulmod2(&raw mut v0, w0, &raw mut v1, w1, umod);
        *a.offset(j.wrapping_add(mhalf) as isize) = v0;
        *a.offset(j.wrapping_add(1 as mpd_size_t).wrapping_add(mhalf) as isize) = v1;
        j = j.wrapping_add(2 as mpd_size_t);
    }
    wstep = 2 as mpd_uint_t;
    m = n.wrapping_div(2 as mpd_size_t);
    while m >= 2 as mpd_size_t {
        mhalf = m.wrapping_div(2 as mpd_size_t);
        r = 0 as mpd_size_t;
        while r < n {
            u0 = *a.offset(r as isize);
            v0 = *a.offset(r.wrapping_add(mhalf) as isize);
            u1 = *a.offset(m.wrapping_add(r) as isize);
            v1 = *a.offset(m.wrapping_add(r).wrapping_add(mhalf) as isize);
            *a.offset(r as isize) = addmod(u0, v0, umod);
            v0 = submod(u0, v0, umod);
            *a.offset(m.wrapping_add(r) as isize) = addmod(u1, v1, umod);
            v1 = submod(u1, v1, umod);
            *a.offset(r.wrapping_add(mhalf) as isize) = v0;
            *a.offset(m.wrapping_add(r).wrapping_add(mhalf) as isize) = v1;
            r = r.wrapping_add((2 as mpd_size_t).wrapping_mul(m));
        }
        j = 1 as mpd_size_t;
        while j < mhalf {
            w = *wtable.offset((j as mpd_uint_t).wrapping_mul(wstep) as isize);
            r = 0 as mpd_size_t;
            while r < n {
                u0 = *a.offset(r.wrapping_add(j) as isize);
                v0 = *a.offset(r.wrapping_add(j).wrapping_add(mhalf) as isize);
                u1 = *a.offset(m.wrapping_add(r).wrapping_add(j) as isize);
                v1 = *a.offset(m.wrapping_add(r).wrapping_add(j).wrapping_add(mhalf) as isize);
                *a.offset(r.wrapping_add(j) as isize) = addmod(u0, v0, umod);
                v0 = submod(u0, v0, umod);
                *a.offset(m.wrapping_add(r).wrapping_add(j) as isize) = addmod(u1, v1, umod);
                v1 = submod(u1, v1, umod);
                x64_mulmod2c(&raw mut v0, &raw mut v1, w, umod);
                *a.offset(r.wrapping_add(j).wrapping_add(mhalf) as isize) = v0;
                *a.offset(m.wrapping_add(r).wrapping_add(j).wrapping_add(mhalf) as isize) = v1;
                r = r.wrapping_add((2 as mpd_size_t).wrapping_mul(m));
            }
            j = j.wrapping_add(1);
        }
        m >>= 1 as ::core::ffi::c_int;
        wstep <<= 1 as ::core::ffi::c_int;
    }
    bitreverse_permute(a, n);
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn fnt_dif2(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut tparams: *mut fnt_params,
) {
    let mut wtable: *mut mpd_uint_t = &raw mut (*tparams).wtable as *mut mpd_uint_t;
    let mut umod: mpd_uint_t = 0;
    let mut u0: mpd_uint_t = 0;
    let mut u1: mpd_uint_t = 0;
    let mut v0: mpd_uint_t = 0;
    let mut v1: mpd_uint_t = 0;
    let mut w: mpd_uint_t = 0;
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut m: mpd_size_t = 0;
    let mut mhalf: mpd_size_t = 0;
    let mut j: mpd_size_t = 0;
    let mut r: mpd_size_t = 0;
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    std_setmodulus((*tparams).modnum, &raw mut umod);
    mhalf = n.wrapping_div(2 as mpd_size_t);
    j = 0 as mpd_size_t;
    while j < mhalf {
        w0 = *wtable.offset(j as isize);
        w1 = *wtable.offset(j.wrapping_add(1 as mpd_size_t) as isize);
        u0 = *a.offset(j as isize);
        v0 = *a.offset(j.wrapping_add(mhalf) as isize);
        u1 = *a.offset(j.wrapping_add(1 as mpd_size_t) as isize);
        v1 = *a.offset(j.wrapping_add(1 as mpd_size_t).wrapping_add(mhalf) as isize);
        *a.offset(j as isize) = addmod(u0, v0, umod);
        v0 = submod(u0, v0, umod);
        *a.offset(j.wrapping_add(1 as mpd_size_t) as isize) = addmod(u1, v1, umod);
        v1 = submod(u1, v1, umod);
        std_mulmod2(&raw mut v0, w0, &raw mut v1, w1, umod);
        *a.offset(j.wrapping_add(mhalf) as isize) = v0;
        *a.offset(j.wrapping_add(1 as mpd_size_t).wrapping_add(mhalf) as isize) = v1;
        j = j.wrapping_add(2 as mpd_size_t);
    }
    wstep = 2 as mpd_uint_t;
    m = n.wrapping_div(2 as mpd_size_t);
    while m >= 2 as mpd_size_t {
        mhalf = m.wrapping_div(2 as mpd_size_t);
        r = 0 as mpd_size_t;
        while r < n {
            u0 = *a.offset(r as isize);
            v0 = *a.offset(r.wrapping_add(mhalf) as isize);
            u1 = *a.offset(m.wrapping_add(r) as isize);
            v1 = *a.offset(m.wrapping_add(r).wrapping_add(mhalf) as isize);
            *a.offset(r as isize) = addmod(u0, v0, umod);
            v0 = submod(u0, v0, umod);
            *a.offset(m.wrapping_add(r) as isize) = addmod(u1, v1, umod);
            v1 = submod(u1, v1, umod);
            *a.offset(r.wrapping_add(mhalf) as isize) = v0;
            *a.offset(m.wrapping_add(r).wrapping_add(mhalf) as isize) = v1;
            r = r.wrapping_add((2 as mpd_size_t).wrapping_mul(m));
        }
        j = 1 as mpd_size_t;
        while j < mhalf {
            w = *wtable.offset(j.wrapping_mul(wstep as mpd_size_t) as isize);
            r = 0 as mpd_size_t;
            while r < n {
                u0 = *a.offset(r.wrapping_add(j) as isize);
                v0 = *a.offset(r.wrapping_add(j).wrapping_add(mhalf) as isize);
                u1 = *a.offset(m.wrapping_add(r).wrapping_add(j) as isize);
                v1 = *a.offset(m.wrapping_add(r).wrapping_add(j).wrapping_add(mhalf) as isize);
                *a.offset(r.wrapping_add(j) as isize) = addmod(u0, v0, umod);
                v0 = submod(u0, v0, umod);
                *a.offset(m.wrapping_add(r).wrapping_add(j) as isize) = addmod(u1, v1, umod);
                v1 = submod(u1, v1, umod);
                std_mulmod2c(&raw mut v0, &raw mut v1, w, umod);
                *a.offset(r.wrapping_add(j).wrapping_add(mhalf) as isize) = v0;
                *a.offset(m.wrapping_add(r).wrapping_add(j).wrapping_add(mhalf) as isize) = v1;
                r = r.wrapping_add((2 as mpd_size_t).wrapping_mul(m));
            }
            j = j.wrapping_add(1);
        }
        m >>= 1 as ::core::ffi::c_int;
        wstep <<= 1 as ::core::ffi::c_int;
    }
    bitreverse_permute(a, n);
}
