#[cfg(target_pointer_width = "64")]
pub type __uint128_t = u128;
pub type size_t = usize;
pub type FILE = crate::shim::FILE;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type int64_t = i64;
pub type uint64_t = u64;
#[cfg(target_pointer_width = "64")]
pub type mpd_uint_t = uint64_t;
#[cfg(target_pointer_width = "32")]
pub type mpd_uint_t = uint32_t;
#[cfg(target_pointer_width = "32")]
pub type mpd_uuint_t = uint64_t;
pub type mpd_size_t = size_t;
#[cfg(target_pointer_width = "64")]
pub type mpd_ssize_t = int64_t;
#[cfg(target_pointer_width = "32")]
pub type mpd_ssize_t = int32_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const UINT64_MAX: u64 = 18446744073709551615 as u64;
#[cfg(target_pointer_width = "64")]
pub const SIZE_MAX: u64 = __SIZE_MAX__;
#[cfg(target_pointer_width = "32")]
pub const SIZE_MAX: ::core::ffi::c_uint = __SIZE_MAX__;
#[cfg(target_pointer_width = "64")]
pub const MPD_SIZE_MAX: u64 = SIZE_MAX;
#[cfg(target_pointer_width = "32")]
pub const MPD_SIZE_MAX: ::core::ffi::c_uint = SIZE_MAX;
#[cfg(target_pointer_width = "64")]
pub const MPD_RADIX: u64 = 10000000000000000000 as u64;
#[cfg(target_pointer_width = "32")]
pub const MPD_RADIX: ::core::ffi::c_uint = 1000000000 as ::core::ffi::c_uint;
#[cfg(target_pointer_width = "64")]
pub const MPD_RDIGITS: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "32")]
pub const MPD_RDIGITS: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MPD_MINALLOC_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn _mpd_mul_words(
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
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn _mpd_mul_words(
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
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn _mpd_div_words(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut hi: mpd_uint_t,
    mut lo: mpd_uint_t,
    mut d: mpd_uint_t,
) {
    let mut hl: __uint128_t = 0;
    hl = ((hi as __uint128_t) << 64 as ::core::ffi::c_int).wrapping_add(lo as __uint128_t);
    *q = hl.wrapping_div(d as __uint128_t) as mpd_uint_t;
    *r = hl.wrapping_sub((*q as __uint128_t).wrapping_mul(d as __uint128_t)) as mpd_uint_t;
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn _mpd_div_words(
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
pub unsafe extern "C" fn _mpd_div_word(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut v: mpd_uint_t,
    mut d: mpd_uint_t,
) {
    *q = v.wrapping_div(d);
    *r = v.wrapping_sub((*q).wrapping_mul(d));
}
pub unsafe extern "C" fn _mpd_isallzero(
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
#[cfg(target_pointer_width = "64")]
pub const __SIZE_MAX__: u64 = 18446744073709551615 as u64;
#[cfg(target_pointer_width = "32")]
pub const __SIZE_MAX__: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MPD_ROUND_GUARD: C2RustUnnamed = 9;
pub const MPD_ROUND_TRUNC: C2RustUnnamed = 8;
pub const MPD_ROUND_05UP: C2RustUnnamed = 7;
pub const MPD_ROUND_HALF_EVEN: C2RustUnnamed = 6;
pub const MPD_ROUND_HALF_DOWN: C2RustUnnamed = 5;
pub const MPD_ROUND_HALF_UP: C2RustUnnamed = 4;
pub const MPD_ROUND_FLOOR: C2RustUnnamed = 3;
pub const MPD_ROUND_CEILING: C2RustUnnamed = 2;
pub const MPD_ROUND_DOWN: C2RustUnnamed = 1;
pub const MPD_ROUND_UP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_context_t {
    pub prec: mpd_ssize_t,
    pub emax: mpd_ssize_t,
    pub emin: mpd_ssize_t,
    pub traps: uint32_t,
    pub status: uint32_t,
    pub newtrap: uint32_t,
    pub round: ::core::ffi::c_int,
    pub clamp: ::core::ffi::c_int,
    pub allcr: ::core::ffi::c_int,
}
#[cfg(target_pointer_width = "64")]
pub const MPD_MAX_PREC: i64 = 999999999999999999 as i64;
#[cfg(target_pointer_width = "32")]
pub const MPD_MAX_PREC: ::core::ffi::c_int = 425000000 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "64")]
pub const MPD_MAX_EMAX: i64 = 999999999999999999 as i64;
#[cfg(target_pointer_width = "32")]
pub const MPD_MAX_EMAX: ::core::ffi::c_int = 425000000 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "64")]
pub const MPD_MIN_EMIN: i64 = -(999999999999999999 as i64);
#[cfg(target_pointer_width = "32")]
pub const MPD_MIN_EMIN: ::core::ffi::c_int = -(425000000 as ::core::ffi::c_int);
pub const MPD_Clamped: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const MPD_Conversion_syntax: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const MPD_Division_by_zero: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const MPD_Division_impossible: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const MPD_Division_undefined: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const MPD_Fpu_error: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const MPD_Invalid_context: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const MPD_Invalid_operation: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const MPD_Malloc_error: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const MPD_Overflow: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const MPD_Underflow: ::core::ffi::c_uint = 0x4000 as ::core::ffi::c_uint;
pub const MPD_IEEE_Invalid_operation: ::core::ffi::c_uint = MPD_Conversion_syntax
    | MPD_Division_impossible
    | MPD_Division_undefined
    | MPD_Fpu_error
    | MPD_Invalid_context
    | MPD_Invalid_operation
    | MPD_Malloc_error;
pub const MPD_Traps: ::core::ffi::c_uint =
    MPD_IEEE_Invalid_operation | MPD_Division_by_zero | MPD_Overflow | MPD_Underflow;
pub const MPD_MINALLOC_MIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub unsafe extern "C" fn ispower2(mut n: mpd_size_t) -> ::core::ffi::c_int {
    return (n != 0 as mpd_size_t && n & n.wrapping_sub(1 as mpd_size_t) == 0 as mpd_size_t)
        as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn std_mulmod(
    mut a: mpd_uint_t,
    mut b: mpd_uint_t,
    mut m: mpd_uint_t,
) -> mpd_uint_t {
    return (a as mpd_uuint_t)
        .wrapping_mul(b as mpd_uuint_t)
        .wrapping_rem(m as mpd_uuint_t) as mpd_uint_t;
}
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn std_mulmod2c(
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
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn std_mulmod2(
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
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn std_powmod(
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
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn x64_mulmod(
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
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn x64_mulmod2c(
    mut a: *mut mpd_uint_t,
    mut b: *mut mpd_uint_t,
    mut w: mpd_uint_t,
    mut m: mpd_uint_t,
) {
    *a = x64_mulmod(*a, w, m);
    *b = x64_mulmod(*b, w, m);
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn x64_mulmod2(
    mut a0: *mut mpd_uint_t,
    mut b0: mpd_uint_t,
    mut a1: *mut mpd_uint_t,
    mut b1: mpd_uint_t,
    mut m: mpd_uint_t,
) {
    *a0 = x64_mulmod(*a0, b0, m);
    *a1 = x64_mulmod(*a1, b1, m);
}
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn x64_powmod(
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnt_params {
    pub modnum: ::core::ffi::c_int,
    pub modulus: mpd_uint_t,
    pub kernel: mpd_uint_t,
    pub wtable: [mpd_uint_t; 0],
}
pub unsafe extern "C" fn addmod(mut a: mpd_uint_t, mut b: mpd_uint_t, mut m: mpd_uint_t) -> mpd_uint_t {
    let mut s: mpd_uint_t = 0;
    s = a.wrapping_add(b);
    s = if s < a { s.wrapping_sub(m) } else { s };
    s = if s >= m { s.wrapping_sub(m) } else { s };
    return s;
}
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_t {
    pub flags: uint8_t,
    pub exp: mpd_ssize_t,
    pub digits: mpd_ssize_t,
    pub len: mpd_ssize_t,
    pub alloc: mpd_ssize_t,
    pub data: *mut mpd_uint_t,
}
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT32_MIN: ::core::ffi::c_int = -INT32_MAX - 1 as ::core::ffi::c_int;
pub const INT64_MAX: i64 = 9223372036854775807 as i64;
pub const INT64_MIN: i64 = -INT64_MAX - 1 as i64;
#[cfg(target_pointer_width = "64")]
pub const MPD_SSIZE_MAX: i64 = INT64_MAX;
#[cfg(target_pointer_width = "32")]
pub const MPD_SSIZE_MAX: ::core::ffi::c_int = INT32_MAX;
#[cfg(target_pointer_width = "64")]
pub const MPD_SSIZE_MIN: i64 = INT64_MIN;
#[cfg(target_pointer_width = "32")]
pub const MPD_SSIZE_MIN: ::core::ffi::c_int = INT32_MIN;
#[cfg(target_pointer_width = "64")]
pub const MPD_EXPDIGITS: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "32")]
pub const MPD_EXPDIGITS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[cfg(target_pointer_width = "64")]
pub const MPD_EXP_INF: i64 = 2000000000000000001 as i64;
#[cfg(target_pointer_width = "32")]
pub const MPD_EXP_INF: ::core::ffi::c_int = 1000000001 as ::core::ffi::c_int;
pub const MPD_Inexact: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const MPD_Rounded: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const MPD_Errors: ::core::ffi::c_uint = MPD_IEEE_Invalid_operation | MPD_Division_by_zero;
pub const MPD_POS: uint8_t = 0 as uint8_t;
pub const MPD_NEG: uint8_t = 1 as uint8_t;
pub const MPD_INF: uint8_t = 2 as uint8_t;
pub const MPD_NAN: uint8_t = 4 as uint8_t;
pub const MPD_SNAN: uint8_t = 8 as uint8_t;
pub const MPD_STATIC: uint8_t = 16 as uint8_t;
pub const MPD_STATIC_DATA: uint8_t = 32 as uint8_t;
pub unsafe extern "C" fn _mpd_idiv_word(
    mut q: *mut mpd_ssize_t,
    mut r: *mut mpd_ssize_t,
    mut v: mpd_ssize_t,
    mut d: mpd_ssize_t,
) {
    *q = v / d;
    *r = v - *q * d;
}
pub type uint16_t = u16;
#[cfg(target_pointer_width = "64")]
pub unsafe extern "C" fn mpd_bsr(mut n: mpd_size_t) -> ::core::ffi::c_int {
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
#[cfg(target_pointer_width = "32")]
pub unsafe extern "C" fn mpd_bsr(mut n: mpd_size_t) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmp: mpd_size_t = 0;
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
