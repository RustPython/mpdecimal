extern "C" {
    fn mpd_c2rust_assert_fail(expr: *const ::core::ffi::c_char);
    static mpd_moduli: [mpd_uint_t; 0];
    fn _mpd_getkernel(
        n: mpd_uint_t,
        sign: ::core::ffi::c_int,
        modnum: ::core::ffi::c_int,
    ) -> mpd_uint_t;
    fn _mpd_init_w3table(
        w3table: *mut mpd_uint_t,
        sign: ::core::ffi::c_int,
        modnum: ::core::ffi::c_int,
    );
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
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
pub type mpd_uint_t = uint32_t;
pub type mpd_uuint_t = uint64_t;
pub type mpd_size_t = size_t;
unsafe extern "C" fn std_setmodulus(mut modnum: ::core::ffi::c_int, mut umod: *mut mpd_uint_t) {
    *umod = *(&raw const mpd_moduli as *const mpd_uint_t).offset(modnum as isize);
}
unsafe extern "C" fn addmod(mut a: mpd_uint_t, mut b: mpd_uint_t, mut m: mpd_uint_t) -> mpd_uint_t {
    let mut s: mpd_uint_t = 0;
    s = a.wrapping_add(b);
    s = if s < a { s.wrapping_sub(m) } else { s };
    s = if s >= m { s.wrapping_sub(m) } else { s };
    return s;
}
unsafe extern "C" fn std_mulmod(
    mut a: mpd_uint_t,
    mut b: mpd_uint_t,
    mut m: mpd_uint_t,
) -> mpd_uint_t {
    return (a as mpd_uuint_t)
        .wrapping_mul(b as mpd_uuint_t)
        .wrapping_rem(m as mpd_uuint_t) as mpd_uint_t;
}
unsafe extern "C" fn std_mulmod2c(
    mut a: *mut mpd_uint_t,
    mut b: *mut mpd_uint_t,
    mut w: mpd_uint_t,
    mut m: mpd_uint_t,
) {
    *a = (*a as mpd_uuint_t)
        .wrapping_mul(w as mpd_uuint_t)
        .wrapping_rem(m as mpd_uuint_t) as mpd_uint_t;
    *b = (*b as mpd_uuint_t)
        .wrapping_mul(w as mpd_uuint_t)
        .wrapping_rem(m as mpd_uuint_t) as mpd_uint_t;
}
unsafe extern "C" fn std_mulmod2(
    mut a0: *mut mpd_uint_t,
    mut b0: mpd_uint_t,
    mut a1: *mut mpd_uint_t,
    mut b1: mpd_uint_t,
    mut m: mpd_uint_t,
) {
    *a0 = (*a0 as mpd_uuint_t)
        .wrapping_mul(b0 as mpd_uuint_t)
        .wrapping_rem(m as mpd_uuint_t) as mpd_uint_t;
    *a1 = (*a1 as mpd_uuint_t)
        .wrapping_mul(b1 as mpd_uuint_t)
        .wrapping_rem(m as mpd_uuint_t) as mpd_uint_t;
}
unsafe extern "C" fn std_powmod(
    mut base: mpd_uint_t,
    mut exp: mpd_uint_t,
    mut umod: mpd_uint_t,
) -> mpd_uint_t {
    let mut r: mpd_uint_t = 1 as mpd_uint_t;
    while exp > 0 as mpd_uint_t {
        if exp & 1 as mpd_uint_t != 0 {
            r = std_mulmod(r, base, umod);
        }
        base = std_mulmod(base, base, umod);
        exp >>= 1 as ::core::ffi::c_int;
    }
    return r;
}
unsafe extern "C" fn std_size3_ntt(
    mut x1: *mut mpd_uint_t,
    mut x2: *mut mpd_uint_t,
    mut x3: *mut mpd_uint_t,
    mut w3table: *mut mpd_uint_t,
    mut umod: mpd_uint_t,
) {
    let mut r1: mpd_uint_t = 0;
    let mut r2: mpd_uint_t = 0;
    let mut w: mpd_uint_t = 0;
    let mut s: mpd_uint_t = 0;
    let mut tmp: mpd_uint_t = 0;
    s = *x1;
    s = addmod(s, *x2, umod);
    s = addmod(s, *x3, umod);
    r1 = s;
    s = *x1;
    w = *w3table.offset(1 as isize);
    tmp = std_mulmod(*x2, w, umod);
    s = addmod(s, tmp, umod);
    w = *w3table.offset(2 as isize);
    tmp = std_mulmod(*x3, w, umod);
    s = addmod(s, tmp, umod);
    r2 = s;
    s = *x1;
    w = *w3table.offset(2 as isize);
    tmp = std_mulmod(*x2, w, umod);
    s = addmod(s, tmp, umod);
    w = *w3table.offset(1 as isize);
    tmp = std_mulmod(*x3, w, umod);
    s = addmod(s, tmp, umod);
    *x3 = s;
    *x2 = r2;
    *x1 = r1;
}
#[no_mangle]
pub unsafe extern "C" fn four_step_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut R: mpd_size_t = 3 as mpd_size_t;
    let mut C: mpd_size_t = n.wrapping_div(3 as mpd_size_t);
    let mut w3table: [mpd_uint_t; 3] = [0; 3];
    let mut kernel: mpd_uint_t = 0;
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut s: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut p0: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut p1: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut p2: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut umod: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut k: mpd_size_t = 0;
    if n >= 48 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 48\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n <= (3 as mpd_size_t).wrapping_mul(33554432 as mpd_size_t) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    std_setmodulus(modnum, &raw mut umod);
    _mpd_init_w3table(
        &raw mut w3table as *mut mpd_uint_t,
        -(1 as ::core::ffi::c_int),
        modnum,
    );
    p0 = a;
    p1 = p0.offset(C as isize);
    p2 = p0.offset((2 as mpd_size_t).wrapping_mul(C) as isize);
    while p0 < a.offset(C as isize) {
        std_size3_ntt(p0, p1, p2, &raw mut w3table as *mut mpd_uint_t, umod);
        p0 = p0.offset(1);
        p1 = p1.offset(1);
        p2 = p2.offset(1);
    }
    kernel = _mpd_getkernel(n as mpd_uint_t, -(1 as ::core::ffi::c_int), modnum);
    i = 1 as mpd_size_t;
    while i < R {
        w0 = 1 as mpd_uint_t;
        w1 = std_powmod(kernel, i as mpd_uint_t, umod);
        wstep = std_mulmod(w1, w1, umod);
        k = 0 as mpd_size_t;
        while k < C.wrapping_sub(1 as mpd_size_t) {
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
    s = a;
    while s < a.offset(n as isize) {
        if six_step_fnt(s, C, modnum) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        s = s.offset(C as isize);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inv_four_step_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut R: mpd_size_t = 3 as mpd_size_t;
    let mut C: mpd_size_t = n.wrapping_div(3 as mpd_size_t);
    let mut w3table: [mpd_uint_t; 3] = [0; 3];
    let mut kernel: mpd_uint_t = 0;
    let mut w0: mpd_uint_t = 0;
    let mut w1: mpd_uint_t = 0;
    let mut wstep: mpd_uint_t = 0;
    let mut s: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut p0: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut p1: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut p2: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut umod: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut k: mpd_size_t = 0;
    if n >= 48 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 48\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n <= (3 as mpd_size_t).wrapping_mul(33554432 as mpd_size_t) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    s = a;
    while s < a.offset(n as isize) {
        if inv_six_step_fnt(s, C, modnum) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        s = s.offset(C as isize);
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
    _mpd_init_w3table(
        &raw mut w3table as *mut mpd_uint_t,
        1 as ::core::ffi::c_int,
        modnum,
    );
    p0 = a;
    p1 = p0.offset(C as isize);
    p2 = p0.offset((2 as mpd_size_t).wrapping_mul(C) as isize);
    while p0 < a.offset(C as isize) {
        std_size3_ntt(p0, p1, p2, &raw mut w3table as *mut mpd_uint_t, umod);
        p0 = p0.offset(1);
        p1 = p1.offset(1);
        p2 = p2.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
