extern "C" {
    fn mpd_c2rust_assert_fail(expr: *const ::core::ffi::c_char);
    fn mpd_c2rust_stderr() -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputc(c: ::core::ffi::c_int, stream: *mut FILE) -> ::core::ffi::c_int;
    fn abort() -> !;
    static mut mpd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    fn mpd_alloc(nmemb: mpd_size_t, size: mpd_size_t) -> *mut ::core::ffi::c_void;
    fn mpd_uint_zero(dest: *mut mpd_uint_t, len: mpd_size_t);
    static mpd_pow10: [mpd_uint_t; 0];
}
pub type size_t = usize;
pub type FILE = crate::src::shim::FILE;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type mpd_uint_t = uint32_t;
pub type mpd_uuint_t = uint64_t;
pub type mpd_size_t = size_t;
pub type mpd_ssize_t = int32_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIZE_MAX: ::core::ffi::c_uint = __SIZE_MAX__;
pub const MPD_SIZE_MAX: ::core::ffi::c_uint = SIZE_MAX;
pub const MPD_RADIX: ::core::ffi::c_uint = 1000000000 as ::core::ffi::c_uint;
pub const MPD_RDIGITS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MPD_MINALLOC_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn _mpd_mul_words(
    mut hi: *mut mpd_uint_t,
    mut lo: *mut mpd_uint_t,
    mut a: mpd_uint_t,
    mut b: mpd_uint_t,
) {
    let mut hl: mpd_uuint_t = 0;
    hl = (a as mpd_uuint_t).wrapping_mul(b as mpd_uuint_t);
    *hi = (hl >> 32 as ::core::ffi::c_int) as mpd_uint_t;
    *lo = hl as mpd_uint_t;
}
unsafe extern "C" fn _mpd_div_words(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut hi: mpd_uint_t,
    mut lo: mpd_uint_t,
    mut d: mpd_uint_t,
) {
    let mut hl: mpd_uuint_t = 0;
    hl = ((hi as mpd_uuint_t) << 32 as ::core::ffi::c_int).wrapping_add(lo as mpd_uuint_t);
    *q = hl.wrapping_div(d as mpd_uuint_t) as mpd_uint_t;
    *r = hl.wrapping_sub((*q as mpd_uuint_t).wrapping_mul(d as mpd_uuint_t)) as mpd_uint_t;
}
unsafe extern "C" fn _mpd_divmod_pow10(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut v: mpd_uint_t,
    mut exp: mpd_uint_t,
) {
    if exp <= 9 as mpd_uint_t {
    } else {
        mpd_c2rust_assert_fail(b"exp <= 9\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if exp <= 4 as mpd_uint_t {
        match exp {
            0 => {
                *q = v;
                *r = 0 as mpd_uint_t;
            }
            1 => {
                *q = (v as u32).wrapping_div(10 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(10 as u32)) as mpd_uint_t;
            }
            2 => {
                *q = (v as u32).wrapping_div(100 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(100 as u32)) as mpd_uint_t;
            }
            3 => {
                *q = (v as u32).wrapping_div(1000 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(1000 as u32)) as mpd_uint_t;
            }
            4 => {
                *q = (v as u32).wrapping_div(10000 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(10000 as u32)) as mpd_uint_t;
            }
            _ => {}
        }
    } else {
        match exp {
            5 => {
                *q = (v as u32).wrapping_div(100000 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(100000 as u32)) as mpd_uint_t;
            }
            6 => {
                *q = (v as u32).wrapping_div(1000000 as u32) as mpd_uint_t;
                *r =
                    (v as u32).wrapping_sub((*q as u32).wrapping_mul(1000000 as u32)) as mpd_uint_t;
            }
            7 => {
                *q = (v as u32).wrapping_div(10000000 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(10000000 as u32))
                    as mpd_uint_t;
            }
            8 => {
                *q = (v as u32).wrapping_div(100000000 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(100000000 as u32))
                    as mpd_uint_t;
            }
            9 => {
                *q = (v as u32).wrapping_div(1000000000 as u32) as mpd_uint_t;
                *r = (v as u32).wrapping_sub((*q as u32).wrapping_mul(1000000000 as u32))
                    as mpd_uint_t;
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn _mpd_div_word(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut v: mpd_uint_t,
    mut d: mpd_uint_t,
) {
    *q = v.wrapping_div(d);
    *r = v.wrapping_sub((*q).wrapping_mul(d));
}
unsafe extern "C" fn sub_size_t(mut a: mpd_size_t, mut b: mpd_size_t) -> mpd_size_t {
    if b > a {
        fprintf(
            mpd_c2rust_stderr(),
            b"%s:%d: error: \0" as *const u8 as *const ::core::ffi::c_char,
            b"libmpdec\\typearith.h\0" as *const u8 as *const ::core::ffi::c_char,
            601 as ::core::ffi::c_int,
        );
        fprintf(
            mpd_c2rust_stderr(),
            b"sub_size_t(): overflow: check the context\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fputc('\n' as i32, mpd_c2rust_stderr());
        abort();
    }
    return a.wrapping_sub(b);
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_baseadd(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut v: *const mpd_uint_t,
    mut m: mpd_size_t,
    mut n: mpd_size_t,
) -> mpd_uint_t {
    let mut s: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n > 0 as mpd_size_t && m >= n {
    } else {
        mpd_c2rust_assert_fail(b"n > 0 && m >= n\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = 0 as mpd_size_t;
    while i < n {
        s = (*u.offset(i as isize)).wrapping_add((*v.offset(i as isize)).wrapping_add(carry));
        carry = ((s < *u.offset(i as isize)) as ::core::ffi::c_int
            | (s >= MPD_RADIX as mpd_uint_t) as ::core::ffi::c_int) as mpd_uint_t;
        *w.offset(i as isize) = if carry != 0 {
            s.wrapping_sub(MPD_RADIX as mpd_uint_t)
        } else {
            s
        };
        i = i.wrapping_add(1);
    }
    while carry != 0 && i < m {
        s = (*u.offset(i as isize)).wrapping_add(carry);
        carry = (s == MPD_RADIX as mpd_uint_t) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if carry != 0 { 0 as mpd_uint_t } else { s };
        i = i.wrapping_add(1);
    }
    while i < m {
        *w.offset(i as isize) = *u.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_baseaddto(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut n: mpd_size_t,
) {
    let mut s: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n == 0 as mpd_size_t {
        return;
    }
    i = 0 as mpd_size_t;
    while i < n {
        s = (*w.offset(i as isize)).wrapping_add((*u.offset(i as isize)).wrapping_add(carry));
        carry = ((s < *w.offset(i as isize)) as ::core::ffi::c_int
            | (s >= MPD_RADIX as mpd_uint_t) as ::core::ffi::c_int) as mpd_uint_t;
        *w.offset(i as isize) = if carry != 0 {
            s.wrapping_sub(MPD_RADIX as mpd_uint_t)
        } else {
            s
        };
        i = i.wrapping_add(1);
    }
    while carry != 0 {
        s = (*w.offset(i as isize)).wrapping_add(carry);
        carry = (s == MPD_RADIX as mpd_uint_t) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if carry != 0 { 0 as mpd_uint_t } else { s };
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_shortadd(
    mut w: *mut mpd_uint_t,
    mut m: mpd_size_t,
    mut v: mpd_uint_t,
) -> mpd_uint_t {
    let mut s: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    if m > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"m > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    s = (*w.offset(0 as isize)).wrapping_add(v);
    carry = ((s < v) as ::core::ffi::c_int | (s >= MPD_RADIX as mpd_uint_t) as ::core::ffi::c_int)
        as mpd_uint_t;
    *w.offset(0 as isize) = if carry != 0 {
        s.wrapping_sub(MPD_RADIX as mpd_uint_t)
    } else {
        s
    };
    i = 1 as mpd_size_t;
    while carry != 0 && i < m {
        s = (*w.offset(i as isize)).wrapping_add(carry);
        carry = (s == MPD_RADIX as mpd_uint_t) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if carry != 0 { 0 as mpd_uint_t } else { s };
        i = i.wrapping_add(1);
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_baseincr(mut u: *mut mpd_uint_t, mut n: mpd_size_t) -> mpd_uint_t {
    let mut s: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 1 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = 0 as mpd_size_t;
    while carry != 0 && i < n {
        s = (*u.offset(i as isize)).wrapping_add(carry);
        carry = (s == MPD_RADIX as mpd_uint_t) as ::core::ffi::c_int as mpd_uint_t;
        *u.offset(i as isize) = if carry != 0 { 0 as mpd_uint_t } else { s };
        i = i.wrapping_add(1);
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_basesub(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut v: *const mpd_uint_t,
    mut m: mpd_size_t,
    mut n: mpd_size_t,
) {
    let mut d: mpd_uint_t = 0;
    let mut borrow: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if m > 0 as mpd_size_t && n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"m > 0 && n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = 0 as mpd_size_t;
    while i < n {
        d = (*u.offset(i as isize)).wrapping_sub((*v.offset(i as isize)).wrapping_add(borrow));
        borrow = (*u.offset(i as isize) < d) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if borrow != 0 {
            d.wrapping_add(MPD_RADIX as mpd_uint_t)
        } else {
            d
        };
        i = i.wrapping_add(1);
    }
    while borrow != 0 && i < m {
        d = (*u.offset(i as isize)).wrapping_sub(borrow);
        borrow = (*u.offset(i as isize) == 0 as mpd_uint_t) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if borrow != 0 {
            (MPD_RADIX as mpd_uint_t).wrapping_sub(1 as mpd_uint_t)
        } else {
            d
        };
        i = i.wrapping_add(1);
    }
    while i < m {
        *w.offset(i as isize) = *u.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_basesubfrom(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut n: mpd_size_t,
) {
    let mut d: mpd_uint_t = 0;
    let mut borrow: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n == 0 as mpd_size_t {
        return;
    }
    i = 0 as mpd_size_t;
    while i < n {
        d = (*w.offset(i as isize)).wrapping_sub((*u.offset(i as isize)).wrapping_add(borrow));
        borrow = (*w.offset(i as isize) < d) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if borrow != 0 {
            d.wrapping_add(MPD_RADIX as mpd_uint_t)
        } else {
            d
        };
        i = i.wrapping_add(1);
    }
    while borrow != 0 {
        d = (*w.offset(i as isize)).wrapping_sub(borrow);
        borrow = (*w.offset(i as isize) == 0 as mpd_uint_t) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if borrow != 0 {
            (MPD_RADIX as mpd_uint_t).wrapping_sub(1 as mpd_uint_t)
        } else {
            d
        };
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_shortmul(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut n: mpd_size_t,
    mut v: mpd_uint_t,
) {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = 0 as mpd_size_t;
    while i < n {
        _mpd_mul_words(&raw mut hi, &raw mut lo, *u.offset(i as isize), v);
        lo = carry.wrapping_add(lo);
        if lo < carry {
            hi = hi.wrapping_add(1);
        }
        _mpd_div_words_r(
            &raw mut carry,
            w.offset(i as isize) as *mut mpd_uint_t,
            hi,
            lo,
        );
        i = i.wrapping_add(1);
    }
    *w.offset(i as isize) = carry;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_basemul(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut v: *const mpd_uint_t,
    mut m: mpd_size_t,
    mut n: mpd_size_t,
) {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut j: mpd_size_t = 0;
    if m > 0 as mpd_size_t && n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"m > 0 && n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    j = 0 as mpd_size_t;
    while j < n {
        carry = 0 as mpd_uint_t;
        i = 0 as mpd_size_t;
        while i < m {
            _mpd_mul_words(
                &raw mut hi,
                &raw mut lo,
                *u.offset(i as isize),
                *v.offset(j as isize),
            );
            lo = (*w.offset(i.wrapping_add(j) as isize)).wrapping_add(lo);
            if lo < *w.offset(i.wrapping_add(j) as isize) {
                hi = hi.wrapping_add(1);
            }
            lo = carry.wrapping_add(lo);
            if lo < carry {
                hi = hi.wrapping_add(1);
            }
            _mpd_div_words_r(
                &raw mut carry,
                w.offset(i.wrapping_add(j) as isize) as *mut mpd_uint_t,
                hi,
                lo,
            );
            i = i.wrapping_add(1);
        }
        *w.offset(j.wrapping_add(m) as isize) = carry;
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_shortdiv(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut n: mpd_size_t,
    mut v: mpd_uint_t,
) -> mpd_uint_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut rem: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = n.wrapping_sub(1 as mpd_size_t);
    while i != MPD_SIZE_MAX as mpd_size_t {
        _mpd_mul_words(&raw mut hi, &raw mut lo, rem, MPD_RADIX as mpd_uint_t);
        lo = (*u.offset(i as isize)).wrapping_add(lo);
        if lo < *u.offset(i as isize) {
            hi = hi.wrapping_add(1);
        }
        _mpd_div_words(
            w.offset(i as isize) as *mut mpd_uint_t,
            &raw mut rem,
            hi,
            lo,
            v,
        );
        i = i.wrapping_sub(1);
    }
    return rem;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_basedivmod(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut uconst: *const mpd_uint_t,
    mut vconst: *const mpd_uint_t,
    mut nplusm: mpd_size_t,
    mut n: mpd_size_t,
) -> ::core::ffi::c_int {
    let mut ustatic: [mpd_uint_t; 64] = [0; 64];
    let mut vstatic: [mpd_uint_t; 64] = [0; 64];
    let mut u: *mut mpd_uint_t = &raw mut ustatic as *mut mpd_uint_t;
    let mut v: *mut mpd_uint_t = &raw mut vstatic as *mut mpd_uint_t;
    let mut d: mpd_uint_t = 0;
    let mut qhat: mpd_uint_t = 0;
    let mut rhat: mpd_uint_t = 0;
    let mut w2: [mpd_uint_t; 2] = [0; 2];
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut x: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut j: mpd_size_t = 0;
    let mut m: mpd_size_t = 0;
    let mut retval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if n > 1 as mpd_size_t && nplusm >= n {
    } else {
        mpd_c2rust_assert_fail(
            b"n > 1 && nplusm >= n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    m = sub_size_t(nplusm, n);
    d = (MPD_RADIX as mpd_uint_t).wrapping_div(
        (*vconst.offset(n.wrapping_sub(1 as mpd_size_t) as isize)).wrapping_add(1 as mpd_uint_t),
    );
    if nplusm >= MPD_MINALLOC_MAX as mpd_size_t {
        u = mpd_alloc(
            nplusm.wrapping_add(1 as mpd_size_t),
            ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
        ) as *mut mpd_uint_t;
        if u.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if n >= MPD_MINALLOC_MAX as mpd_size_t {
        v = mpd_alloc(
            n.wrapping_add(1 as mpd_size_t),
            ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
        ) as *mut mpd_uint_t;
        if v.is_null() {
            mpd_free.expect("non-null function pointer")(u as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    _mpd_shortmul(u, uconst, nplusm, d);
    _mpd_shortmul(v, vconst, n, d);
    j = m;
    while j != MPD_SIZE_MAX as mpd_size_t {
        rhat = _mpd_shortdiv(
            &raw mut w2 as *mut mpd_uint_t,
            u.offset(j as isize)
                .offset(n as isize)
                .offset(-(1 as isize)),
            2 as mpd_size_t,
            *v.offset(n.wrapping_sub(1 as mpd_size_t) as isize),
        );
        qhat = w2[1 as usize]
            .wrapping_mul(MPD_RADIX as mpd_uint_t)
            .wrapping_add(w2[0 as usize]);
        loop {
            if qhat < MPD_RADIX as mpd_uint_t {
                _mpd_singlemul(
                    &raw mut w2 as *mut mpd_uint_t,
                    qhat,
                    *v.offset(n.wrapping_sub(2 as mpd_size_t) as isize),
                );
                if w2[1 as usize] <= rhat {
                    if w2[1 as usize] != rhat
                        || w2[0 as usize]
                            <= *u.offset(j.wrapping_add(n).wrapping_sub(2 as mpd_size_t) as isize)
                    {
                        break;
                    }
                }
            }
            qhat = qhat.wrapping_sub(1 as mpd_uint_t);
            rhat = rhat.wrapping_add(*v.offset(n.wrapping_sub(1 as mpd_size_t) as isize));
            if rhat < *v.offset(n.wrapping_sub(1 as mpd_size_t) as isize)
                || rhat >= MPD_RADIX as mpd_uint_t
            {
                break;
            }
        }
        carry = 0 as mpd_uint_t;
        i = 0 as mpd_size_t;
        while i <= n {
            _mpd_mul_words(&raw mut hi, &raw mut lo, qhat, *v.offset(i as isize));
            lo = carry.wrapping_add(lo);
            if lo < carry {
                hi = hi.wrapping_add(1);
            }
            _mpd_div_words_r(&raw mut hi, &raw mut lo, hi, lo);
            x = (*u.offset(i.wrapping_add(j) as isize)).wrapping_sub(lo);
            carry = (*u.offset(i.wrapping_add(j) as isize) < x) as ::core::ffi::c_int as mpd_uint_t;
            *u.offset(i.wrapping_add(j) as isize) = if carry != 0 {
                x.wrapping_add(MPD_RADIX as mpd_uint_t)
            } else {
                x
            };
            carry = carry.wrapping_add(hi);
            i = i.wrapping_add(1);
        }
        *q.offset(j as isize) = qhat;
        if carry != 0 {
            let ref mut fresh0 = *q.offset(j as isize);
            *fresh0 = (*fresh0).wrapping_sub(1 as mpd_uint_t);
            _mpd_baseadd(
                u.offset(j as isize),
                u.offset(j as isize),
                v,
                n.wrapping_add(1 as mpd_size_t),
                n,
            );
        }
        j = j.wrapping_sub(1);
    }
    if !r.is_null() {
        _mpd_shortdiv(r, u, n, d);
        retval = 0 as ::core::ffi::c_int;
    } else {
        retval = (_mpd_isallzero(u, n as mpd_ssize_t) == 0) as ::core::ffi::c_int;
    }
    if u != &raw mut ustatic as *mut mpd_uint_t {
        mpd_free.expect("non-null function pointer")(u as *mut ::core::ffi::c_void);
    }
    if v != &raw mut vstatic as *mut mpd_uint_t {
        mpd_free.expect("non-null function pointer")(v as *mut ::core::ffi::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_baseshiftl(
    mut dest: *mut mpd_uint_t,
    mut src: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut m: mpd_size_t,
    mut shift: mpd_size_t,
) {
    let mut l: mpd_uint_t = 0;
    let mut lprev: mpd_uint_t = 0;
    let mut h: mpd_uint_t = 0;
    let mut q: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    let mut ph: mpd_uint_t = 0;
    if m > 0 as mpd_size_t && n >= m {
    } else {
        mpd_c2rust_assert_fail(b"m > 0 && n >= m\0" as *const u8 as *const ::core::ffi::c_char);
    };
    _mpd_div_word(
        &raw mut q,
        &raw mut r,
        shift as mpd_uint_t,
        MPD_RDIGITS as mpd_uint_t,
    );
    if r != 0 as mpd_uint_t {
        ph = *(&raw const mpd_pow10 as *const mpd_uint_t).offset(r as isize);
        m = m.wrapping_sub(1);
        n = n.wrapping_sub(1);
        let fresh0 = m;
        m = m.wrapping_sub(1);
        _mpd_divmod_pow10(
            &raw mut h,
            &raw mut lprev,
            *src.offset(fresh0 as isize),
            (MPD_RDIGITS as mpd_uint_t).wrapping_sub(r),
        );
        if h != 0 as mpd_uint_t {
            let fresh1 = n;
            n = n.wrapping_sub(1);
            *dest.offset(fresh1 as isize) = h;
        }
        while m != MPD_SIZE_MAX as mpd_size_t {
            _mpd_divmod_pow10(
                &raw mut h,
                &raw mut l,
                *src.offset(m as isize),
                (MPD_RDIGITS as mpd_uint_t).wrapping_sub(r),
            );
            *dest.offset(n as isize) = ph.wrapping_mul(lprev).wrapping_add(h);
            lprev = l;
            m = m.wrapping_sub(1);
            n = n.wrapping_sub(1);
        }
        *dest.offset(q as isize) = ph.wrapping_mul(lprev);
    } else {
        loop {
            m = m.wrapping_sub(1);
            if !(m != MPD_SIZE_MAX as mpd_size_t) {
                break;
            }
            *dest.offset(m.wrapping_add(q as mpd_size_t) as isize) = *src.offset(m as isize);
        }
    }
    mpd_uint_zero(dest, q as mpd_size_t);
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_baseshiftr(
    mut dest: *mut mpd_uint_t,
    mut src: *mut mpd_uint_t,
    mut slen: mpd_size_t,
    mut shift: mpd_size_t,
) -> mpd_uint_t {
    let mut l: mpd_uint_t = 0;
    let mut h: mpd_uint_t = 0;
    let mut hprev: mpd_uint_t = 0;
    let mut rnd: mpd_uint_t = 0;
    let mut rest: mpd_uint_t = 0;
    let mut q: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    let mut j: mpd_size_t = 0;
    let mut ph: mpd_uint_t = 0;
    if slen > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"slen > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    _mpd_div_word(
        &raw mut q,
        &raw mut r,
        shift as mpd_uint_t,
        MPD_RDIGITS as mpd_uint_t,
    );
    rest = 0 as mpd_uint_t;
    rnd = rest;
    if r != 0 as mpd_uint_t {
        ph = *(&raw const mpd_pow10 as *const mpd_uint_t)
            .offset((MPD_RDIGITS as mpd_uint_t).wrapping_sub(r) as isize);
        _mpd_divmod_pow10(&raw mut hprev, &raw mut rest, *src.offset(q as isize), r);
        _mpd_divmod_pow10(
            &raw mut rnd,
            &raw mut rest,
            rest,
            r.wrapping_sub(1 as mpd_uint_t),
        );
        if rest == 0 as mpd_uint_t && q > 0 as mpd_uint_t {
            rest = (_mpd_isallzero(src, q as mpd_ssize_t) == 0) as ::core::ffi::c_int as mpd_uint_t;
        }
        j = 0 as mpd_size_t;
        i = q.wrapping_add(1 as mpd_uint_t) as mpd_size_t;
        while i < slen {
            _mpd_divmod_pow10(&raw mut h, &raw mut l, *src.offset(i as isize), r);
            *dest.offset(j as isize) = ph.wrapping_mul(l).wrapping_add(hprev);
            hprev = h;
            i = i.wrapping_add(1);
            j = j.wrapping_add(1);
        }
        if hprev != 0 as mpd_uint_t {
            *dest.offset(j as isize) = hprev;
        }
    } else {
        if q > 0 as mpd_uint_t {
            _mpd_divmod_pow10(
                &raw mut rnd,
                &raw mut rest,
                *src.offset(q.wrapping_sub(1 as mpd_uint_t) as isize),
                (MPD_RDIGITS - 1 as ::core::ffi::c_int) as mpd_uint_t,
            );
            if rest == 0 as mpd_uint_t {
                rest = (_mpd_isallzero(src, q.wrapping_sub(1 as mpd_uint_t) as mpd_ssize_t) == 0)
                    as ::core::ffi::c_int as mpd_uint_t;
            }
        }
        j = 0 as mpd_size_t;
        while j < slen.wrapping_sub(q as mpd_size_t) {
            *dest.offset(j as isize) = *src.offset(q.wrapping_add(j as mpd_uint_t) as isize);
            j = j.wrapping_add(1);
        }
    }
    return if rnd == 0 as mpd_uint_t || rnd == 5 as mpd_uint_t {
        rnd.wrapping_add((rest != 0) as ::core::ffi::c_int as mpd_uint_t)
    } else {
        rnd
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_shortadd_b(
    mut w: *mut mpd_uint_t,
    mut m: mpd_size_t,
    mut v: mpd_uint_t,
    mut b: mpd_uint_t,
) -> mpd_uint_t {
    let mut s: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0;
    let mut i: mpd_size_t = 0;
    if m > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"m > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    s = (*w.offset(0 as isize)).wrapping_add(v);
    carry = ((s < v) as ::core::ffi::c_int | (s >= b) as ::core::ffi::c_int) as mpd_uint_t;
    *w.offset(0 as isize) = if carry != 0 { s.wrapping_sub(b) } else { s };
    i = 1 as mpd_size_t;
    while carry != 0 && i < m {
        s = (*w.offset(i as isize)).wrapping_add(carry);
        carry = (s == b) as ::core::ffi::c_int as mpd_uint_t;
        *w.offset(i as isize) = if carry != 0 { 0 as mpd_uint_t } else { s };
        i = i.wrapping_add(1);
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_shortmul_c(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut n: mpd_size_t,
    mut v: mpd_uint_t,
) -> mpd_uint_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = 0 as mpd_size_t;
    while i < n {
        _mpd_mul_words(&raw mut hi, &raw mut lo, *u.offset(i as isize), v);
        lo = carry.wrapping_add(lo);
        if lo < carry {
            hi = hi.wrapping_add(1);
        }
        _mpd_div_words_r(
            &raw mut carry,
            w.offset(i as isize) as *mut mpd_uint_t,
            hi,
            lo,
        );
        i = i.wrapping_add(1);
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_shortmul_b(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut n: mpd_size_t,
    mut v: mpd_uint_t,
    mut b: mpd_uint_t,
) -> mpd_uint_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut carry: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = 0 as mpd_size_t;
    while i < n {
        _mpd_mul_words(&raw mut hi, &raw mut lo, *u.offset(i as isize), v);
        lo = carry.wrapping_add(lo);
        if lo < carry {
            hi = hi.wrapping_add(1);
        }
        _mpd_div_words(
            &raw mut carry,
            w.offset(i as isize) as *mut mpd_uint_t,
            hi,
            lo,
            b,
        );
        i = i.wrapping_add(1);
    }
    return carry;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_shortdiv_b(
    mut w: *mut mpd_uint_t,
    mut u: *const mpd_uint_t,
    mut n: mpd_size_t,
    mut v: mpd_uint_t,
    mut b: mpd_uint_t,
) -> mpd_uint_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut rem: mpd_uint_t = 0 as mpd_uint_t;
    let mut i: mpd_size_t = 0;
    if n > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    i = n.wrapping_sub(1 as mpd_size_t);
    while i != MPD_SIZE_MAX as mpd_size_t {
        _mpd_mul_words(&raw mut hi, &raw mut lo, rem, b);
        lo = (*u.offset(i as isize)).wrapping_add(lo);
        if lo < *u.offset(i as isize) {
            hi = hi.wrapping_add(1);
        }
        _mpd_div_words(
            w.offset(i as isize) as *mut mpd_uint_t,
            &raw mut rem,
            hi,
            lo,
            v,
        );
        i = i.wrapping_sub(1);
    }
    return rem;
}
unsafe extern "C" fn _mpd_div_words_r(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut hi: mpd_uint_t,
    mut lo: mpd_uint_t,
) {
    _mpd_div_words(q, r, hi, lo, MPD_RADIX as mpd_uint_t);
}
unsafe extern "C" fn _mpd_singlemul(mut w: *mut mpd_uint_t, mut u: mpd_uint_t, mut v: mpd_uint_t) {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi, &raw mut lo, u, v);
    _mpd_div_words_r(
        w.offset(1 as isize) as *mut mpd_uint_t,
        w.offset(0 as isize) as *mut mpd_uint_t,
        hi,
        lo,
    );
}
unsafe extern "C" fn _mpd_isallzero(
    mut data: *const mpd_uint_t,
    mut len: mpd_ssize_t,
) -> ::core::ffi::c_int {
    loop {
        len -= 1;
        if !(len >= 0 as mpd_ssize_t) {
            break;
        }
        if *data.offset(len as isize) != 0 as mpd_uint_t {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
pub const __SIZE_MAX__: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
