extern "C" {
    fn mpd_c2rust_assert_fail(expr: *const ::core::ffi::c_char);
    static mut mpd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    static mpd_moduli: [mpd_uint_t; 0];
    fn _mpd_getkernel(
        n: mpd_uint_t,
        sign: ::core::ffi::c_int,
        modnum: ::core::ffi::c_int,
    ) -> mpd_uint_t;
    fn _mpd_init_fnt_params(
        n: mpd_size_t,
        sign: ::core::ffi::c_int,
        modnum: ::core::ffi::c_int,
    ) -> *mut fnt_params;
    fn fnt_dif2(a: *mut mpd_uint_t, n: mpd_size_t, tparams: *mut fnt_params);
    fn transpose_pow2(
        matrix: *mut mpd_uint_t,
        rows: mpd_size_t,
        cols: mpd_size_t,
    ) -> ::core::ffi::c_int;
}
pub type __uint128_t = u128;
pub type size_t = usize;
pub type uint64_t = u64;
pub type mpd_uint_t = uint64_t;
pub type mpd_size_t = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnt_params {
    pub modnum: ::core::ffi::c_int,
    pub modulus: mpd_uint_t,
    pub kernel: mpd_uint_t,
    pub wtable: [mpd_uint_t; 0],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn ispower2(mut n: mpd_size_t) -> ::core::ffi::c_int {
    return (n != 0 as mpd_size_t && n & n.wrapping_sub(1 as mpd_size_t) == 0 as mpd_size_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn mpd_bsr(mut n: mpd_size_t) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmp: mpd_size_t = 0;
    tmp = n >> 32 as ::core::ffi::c_int;
    if tmp != 0 as mpd_size_t {
        n = tmp;
        pos += 32 as ::core::ffi::c_int;
    }
    tmp = n >> 16 as ::core::ffi::c_int;
    if tmp != 0 as mpd_size_t {
        n = tmp;
        pos += 16 as ::core::ffi::c_int;
    }
    tmp = n >> 8 as ::core::ffi::c_int;
    if tmp != 0 as mpd_size_t {
        n = tmp;
        pos += 8 as ::core::ffi::c_int;
    }
    tmp = n >> 4 as ::core::ffi::c_int;
    if tmp != 0 as mpd_size_t {
        n = tmp;
        pos += 4 as ::core::ffi::c_int;
    }
    tmp = n >> 2 as ::core::ffi::c_int;
    if tmp != 0 as mpd_size_t {
        n = tmp;
        pos += 2 as ::core::ffi::c_int;
    }
    tmp = n >> 1 as ::core::ffi::c_int;
    if tmp != 0 as mpd_size_t {
        n = tmp;
        pos += 1 as ::core::ffi::c_int;
    }
    return pos + n as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
}
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
    if n <= 4294967296 as mpd_size_t {
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
    if n <= 4294967296 as mpd_size_t {
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
