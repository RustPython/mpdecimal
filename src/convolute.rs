use crate::common::*;
use crate::constants::mpd_moduli;
use crate::fnt::std_fnt;
use crate::fnt::std_inv_fnt;
use crate::fourstep::four_step_fnt;
use crate::fourstep::inv_four_step_fnt;
use crate::sixstep::six_step_fnt;
use crate::sixstep::inv_six_step_fnt;
pub const SIX_STEP_THRESHOLD: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn std_setmodulus(mut modnum: ::core::ffi::c_int, mut umod: *mut mpd_uint_t) {
    *umod = *(&raw const mpd_moduli as *const mpd_uint_t).offset(modnum as isize);
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn fnt_convolute(
    mut c1: *mut mpd_uint_t,
    mut c2: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut inv_fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut n_inv: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    std_setmodulus(modnum, &raw mut umod);
    n_inv = x64_powmod(n as mpd_uint_t, umod.wrapping_sub(2 as mpd_uint_t), umod);
    if ispower2(n) != 0 {
        if n > SIX_STEP_THRESHOLD as mpd_size_t {
            fnt = Some(
                six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                inv_six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        } else {
            fnt = Some(
                std_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                std_inv_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        }
    } else {
        fnt = Some(
            four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        inv_fnt = Some(
            inv_four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    }
    if fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if fnt.expect("non-null function pointer")(c2, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(1 as mpd_size_t) {
        let mut x0: mpd_uint_t = *c1.offset(i as isize);
        let mut y0: mpd_uint_t = *c2.offset(i as isize);
        let mut x1: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        let mut y1: mpd_uint_t = *c2.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        x64_mulmod2(&raw mut x0, y0, &raw mut x1, y1, umod);
        *c1.offset(i as isize) = x0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1;
        i = i.wrapping_add(2 as mpd_size_t);
    }
    if inv_fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(3 as mpd_size_t) {
        let mut x0_0: mpd_uint_t = *c1.offset(i as isize);
        let mut x1_0: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        let mut x2: mpd_uint_t = *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize);
        let mut x3: mpd_uint_t = *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize);
        x64_mulmod2c(&raw mut x0_0, &raw mut x1_0, n_inv, umod);
        x64_mulmod2c(&raw mut x2, &raw mut x3, n_inv, umod);
        *c1.offset(i as isize) = x0_0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1_0;
        *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize) = x2;
        *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize) = x3;
        i = i.wrapping_add(4 as mpd_size_t);
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn fnt_convolute(
    mut c1: *mut mpd_uint_t,
    mut c2: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut inv_fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut n_inv: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    std_setmodulus(modnum, &raw mut umod);
    n_inv = std_powmod(n as mpd_uint_t, umod.wrapping_sub(2 as mpd_uint_t), umod);
    if ispower2(n) != 0 {
        if n > SIX_STEP_THRESHOLD as mpd_size_t {
            fnt = Some(
                six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                inv_six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        } else {
            fnt = Some(
                std_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                std_inv_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        }
    } else {
        fnt = Some(
            four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        inv_fnt = Some(
            inv_four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    }
    if fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if fnt.expect("non-null function pointer")(c2, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(1 as mpd_size_t) {
        let mut x0: mpd_uint_t = *c1.offset(i as isize);
        let mut y0: mpd_uint_t = *c2.offset(i as isize);
        let mut x1: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        let mut y1: mpd_uint_t = *c2.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        std_mulmod2(&raw mut x0, y0, &raw mut x1, y1, umod);
        *c1.offset(i as isize) = x0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1;
        i = i.wrapping_add(2 as mpd_size_t);
    }
    if inv_fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(3 as mpd_size_t) {
        let mut x0_0: mpd_uint_t = *c1.offset(i as isize);
        let mut x1_0: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        let mut x2: mpd_uint_t = *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize);
        let mut x3: mpd_uint_t = *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize);
        std_mulmod2c(&raw mut x0_0, &raw mut x1_0, n_inv, umod);
        std_mulmod2c(&raw mut x2, &raw mut x3, n_inv, umod);
        *c1.offset(i as isize) = x0_0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1_0;
        *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize) = x2;
        *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize) = x3;
        i = i.wrapping_add(4 as mpd_size_t);
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
#[no_mangle]
pub unsafe extern "C" fn fnt_autoconvolute(
    mut c1: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut inv_fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut n_inv: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    std_setmodulus(modnum, &raw mut umod);
    n_inv = x64_powmod(n as mpd_uint_t, umod.wrapping_sub(2 as mpd_uint_t), umod);
    if ispower2(n) != 0 {
        if n > SIX_STEP_THRESHOLD as mpd_size_t {
            fnt = Some(
                six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                inv_six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        } else {
            fnt = Some(
                std_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                std_inv_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        }
    } else {
        fnt = Some(
            four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        inv_fnt = Some(
            inv_four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    }
    if fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(1 as mpd_size_t) {
        let mut x0: mpd_uint_t = *c1.offset(i as isize);
        let mut x1: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        x64_mulmod2(&raw mut x0, x0, &raw mut x1, x1, umod);
        *c1.offset(i as isize) = x0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1;
        i = i.wrapping_add(2 as mpd_size_t);
    }
    if inv_fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(3 as mpd_size_t) {
        let mut x0_0: mpd_uint_t = *c1.offset(i as isize);
        let mut x1_0: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        let mut x2: mpd_uint_t = *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize);
        let mut x3: mpd_uint_t = *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize);
        x64_mulmod2c(&raw mut x0_0, &raw mut x1_0, n_inv, umod);
        x64_mulmod2c(&raw mut x2, &raw mut x3, n_inv, umod);
        *c1.offset(i as isize) = x0_0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1_0;
        *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize) = x2;
        *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize) = x3;
        i = i.wrapping_add(4 as mpd_size_t);
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn fnt_autoconvolute(
    mut c1: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut inv_fnt: Option<
        unsafe extern "C" fn(*mut mpd_uint_t, mpd_size_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let mut n_inv: mpd_uint_t = 0;
    let mut umod: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    std_setmodulus(modnum, &raw mut umod);
    n_inv = std_powmod(n as mpd_uint_t, umod.wrapping_sub(2 as mpd_uint_t), umod);
    if ispower2(n) != 0 {
        if n > SIX_STEP_THRESHOLD as mpd_size_t {
            fnt = Some(
                six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                inv_six_step_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        } else {
            fnt = Some(
                std_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
            inv_fnt = Some(
                std_inv_fnt
                    as unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut mpd_uint_t,
                        mpd_size_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        }
    } else {
        fnt = Some(
            four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        inv_fnt = Some(
            inv_four_step_fnt
                as unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut mpd_uint_t,
                    mpd_size_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    }
    if fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(1 as mpd_size_t) {
        let mut x0: mpd_uint_t = *c1.offset(i as isize);
        let mut x1: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        std_mulmod2(&raw mut x0, x0, &raw mut x1, x1, umod);
        *c1.offset(i as isize) = x0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1;
        i = i.wrapping_add(2 as mpd_size_t);
    }
    if inv_fnt.expect("non-null function pointer")(c1, n, modnum) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as mpd_size_t;
    while i < n.wrapping_sub(3 as mpd_size_t) {
        let mut x0_0: mpd_uint_t = *c1.offset(i as isize);
        let mut x1_0: mpd_uint_t = *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize);
        let mut x2: mpd_uint_t = *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize);
        let mut x3: mpd_uint_t = *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize);
        std_mulmod2c(&raw mut x0_0, &raw mut x1_0, n_inv, umod);
        std_mulmod2c(&raw mut x2, &raw mut x3, n_inv, umod);
        *c1.offset(i as isize) = x0_0;
        *c1.offset(i.wrapping_add(1 as mpd_size_t) as isize) = x1_0;
        *c1.offset(i.wrapping_add(2 as mpd_size_t) as isize) = x2;
        *c1.offset(i.wrapping_add(3 as mpd_size_t) as isize) = x3;
        i = i.wrapping_add(4 as mpd_size_t);
    }
    return 1 as ::core::ffi::c_int;
}
