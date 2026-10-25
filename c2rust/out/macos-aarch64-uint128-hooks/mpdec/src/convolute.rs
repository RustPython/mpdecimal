extern "C" {
    static mpd_moduli: [mpd_uint_t; 0];
    fn std_fnt(a: *mut mpd_uint_t, n: mpd_size_t, modnum: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn std_inv_fnt(
        a: *mut mpd_uint_t,
        n: mpd_size_t,
        modnum: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn four_step_fnt(
        a: *mut mpd_uint_t,
        n: mpd_size_t,
        modnum: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inv_four_step_fnt(
        a: *mut mpd_uint_t,
        n: mpd_size_t,
        modnum: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn six_step_fnt(
        a: *mut mpd_uint_t,
        n: mpd_size_t,
        modnum: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn inv_six_step_fnt(
        a: *mut mpd_uint_t,
        n: mpd_size_t,
        modnum: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type __uint128_t = u128;
pub type uint64_t = u64;
pub type size_t = usize;
pub type mpd_uint_t = uint64_t;
pub type mpd_size_t = size_t;
unsafe extern "C" fn ispower2(mut n: mpd_size_t) -> ::core::ffi::c_int {
    return (n != 0 as mpd_size_t && n & n.wrapping_sub(1 as mpd_size_t) == 0 as mpd_size_t)
        as ::core::ffi::c_int;
}
pub const SIX_STEP_THRESHOLD: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn std_setmodulus(mut modnum: ::core::ffi::c_int, mut umod: *mut mpd_uint_t) {
    *umod = *(&raw const mpd_moduli as *const mpd_uint_t).offset(modnum as isize);
}
unsafe extern "C" fn _mpd_mul_words(
    mut hi: *mut mpd_uint_t,
    mut lo: *mut mpd_uint_t,
    mut a: mpd_uint_t,
    mut b: mpd_uint_t,
) {
    let mut hl: __uint128_t = 0;
    hl = (a as __uint128_t).wrapping_mul(b as __uint128_t);
    *hi = (hl >> 64 as ::core::ffi::c_int) as mpd_uint_t;
    *lo = hl as mpd_uint_t;
}
unsafe extern "C" fn x64_mulmod(
    mut a: mpd_uint_t,
    mut b: mpd_uint_t,
    mut m: mpd_uint_t,
) -> mpd_uint_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut x: mpd_uint_t = 0;
    let mut y: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi, &raw mut lo, a, b);
    if m & (1 as mpd_uint_t) << 32 as ::core::ffi::c_int != 0 {
        y = hi;
        x = y;
        hi >>= 32 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 32 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        y = hi;
        x = y;
        hi >>= 32 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 32 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        return if hi != 0 || lo >= m {
            lo.wrapping_sub(m)
        } else {
            lo
        };
    } else if m & (1 as mpd_uint_t) << 34 as ::core::ffi::c_int != 0 {
        y = hi;
        x = y;
        hi >>= 30 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 34 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        y = hi;
        x = y;
        hi >>= 30 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 34 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        y = hi;
        x = y;
        hi >>= 30 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 34 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        return if hi != 0 || lo >= m {
            lo.wrapping_sub(m)
        } else {
            lo
        };
    } else {
        y = hi;
        x = y;
        hi >>= 24 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 40 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        y = hi;
        x = y;
        hi >>= 24 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 40 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        y = hi;
        x = y;
        hi >>= 24 as ::core::ffi::c_int;
        x = lo.wrapping_sub(x);
        if x > lo {
            hi = hi.wrapping_sub(1);
        }
        y <<= 40 as ::core::ffi::c_int;
        lo = y.wrapping_add(x);
        if lo < y {
            hi = hi.wrapping_add(1);
        }
        return if hi != 0 || lo >= m {
            lo.wrapping_sub(m)
        } else {
            lo
        };
    };
}
unsafe extern "C" fn x64_mulmod2c(
    mut a: *mut mpd_uint_t,
    mut b: *mut mpd_uint_t,
    mut w: mpd_uint_t,
    mut m: mpd_uint_t,
) {
    *a = x64_mulmod(*a, w, m);
    *b = x64_mulmod(*b, w, m);
}
unsafe extern "C" fn x64_mulmod2(
    mut a0: *mut mpd_uint_t,
    mut b0: mpd_uint_t,
    mut a1: *mut mpd_uint_t,
    mut b1: mpd_uint_t,
    mut m: mpd_uint_t,
) {
    *a0 = x64_mulmod(*a0, b0, m);
    *a1 = x64_mulmod(*a1, b1, m);
}
unsafe extern "C" fn x64_powmod(
    mut base: mpd_uint_t,
    mut exp: mpd_uint_t,
    mut umod: mpd_uint_t,
) -> mpd_uint_t {
    let mut r: mpd_uint_t = 1 as mpd_uint_t;
    while exp > 0 as mpd_uint_t {
        if exp & 1 as mpd_uint_t != 0 {
            r = x64_mulmod(r, base, umod);
        }
        base = x64_mulmod(base, base, umod);
        exp >>= 1 as ::core::ffi::c_int;
    }
    return r;
}
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
