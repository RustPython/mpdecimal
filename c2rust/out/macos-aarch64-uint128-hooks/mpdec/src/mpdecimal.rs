extern "C" {
    fn mpd_c2rust_assert_fail(expr: *const ::core::ffi::c_char);
    fn ceil(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log10(x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn mpd_c2rust_stderr() -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputc(c: ::core::ffi::c_int, stream: *mut FILE) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut MPD_MINALLOC: mpd_ssize_t;
    fn mpd_maxcontext(ctx: *mut mpd_context_t);
    fn mpd_addstatus_raise(ctx: *mut mpd_context_t, flags: uint32_t);
    static mut mpd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    fn mpd_alloc(nmemb: mpd_size_t, size: mpd_size_t) -> *mut ::core::ffi::c_void;
    fn mpd_calloc(nmemb: mpd_size_t, size: mpd_size_t) -> *mut ::core::ffi::c_void;
    fn mpd_realloc(
        ptr: *mut ::core::ffi::c_void,
        nmemb: mpd_size_t,
        size: mpd_size_t,
        err: *mut uint8_t,
    ) -> *mut ::core::ffi::c_void;
    fn mpd_qnew() -> *mut mpd_t;
    fn mpd_qnew_size(nwords: mpd_ssize_t) -> *mut mpd_t;
    fn _mpd_baseadd(
        w: *mut mpd_uint_t,
        u: *const mpd_uint_t,
        v: *const mpd_uint_t,
        m: mpd_size_t,
        n: mpd_size_t,
    ) -> mpd_uint_t;
    fn _mpd_baseaddto(w: *mut mpd_uint_t, u: *const mpd_uint_t, n: mpd_size_t);
    fn _mpd_shortadd(w: *mut mpd_uint_t, m: mpd_size_t, v: mpd_uint_t) -> mpd_uint_t;
    fn _mpd_shortadd_b(
        w: *mut mpd_uint_t,
        m: mpd_size_t,
        v: mpd_uint_t,
        b: mpd_uint_t,
    ) -> mpd_uint_t;
    fn _mpd_baseincr(u: *mut mpd_uint_t, n: mpd_size_t) -> mpd_uint_t;
    fn _mpd_basesub(
        w: *mut mpd_uint_t,
        u: *const mpd_uint_t,
        v: *const mpd_uint_t,
        m: mpd_size_t,
        n: mpd_size_t,
    );
    fn _mpd_basesubfrom(w: *mut mpd_uint_t, u: *const mpd_uint_t, n: mpd_size_t);
    fn _mpd_basemul(
        w: *mut mpd_uint_t,
        u: *const mpd_uint_t,
        v: *const mpd_uint_t,
        m: mpd_size_t,
        n: mpd_size_t,
    );
    fn _mpd_shortmul(w: *mut mpd_uint_t, u: *const mpd_uint_t, n: mpd_size_t, v: mpd_uint_t);
    fn _mpd_shortmul_c(
        w: *mut mpd_uint_t,
        u: *const mpd_uint_t,
        n: mpd_size_t,
        v: mpd_uint_t,
    ) -> mpd_uint_t;
    fn _mpd_shortmul_b(
        w: *mut mpd_uint_t,
        u: *const mpd_uint_t,
        n: mpd_size_t,
        v: mpd_uint_t,
        b: mpd_uint_t,
    ) -> mpd_uint_t;
    fn _mpd_shortdiv(
        w: *mut mpd_uint_t,
        u: *const mpd_uint_t,
        n: mpd_size_t,
        v: mpd_uint_t,
    ) -> mpd_uint_t;
    fn _mpd_shortdiv_b(
        w: *mut mpd_uint_t,
        u: *const mpd_uint_t,
        n: mpd_size_t,
        v: mpd_uint_t,
        b: mpd_uint_t,
    ) -> mpd_uint_t;
    fn _mpd_basedivmod(
        q: *mut mpd_uint_t,
        r: *mut mpd_uint_t,
        uconst: *const mpd_uint_t,
        vconst: *const mpd_uint_t,
        nplusm: mpd_size_t,
        n: mpd_size_t,
    ) -> ::core::ffi::c_int;
    fn _mpd_baseshiftl(
        dest: *mut mpd_uint_t,
        src: *mut mpd_uint_t,
        n: mpd_size_t,
        m: mpd_size_t,
        shift: mpd_size_t,
    );
    fn _mpd_baseshiftr(
        dest: *mut mpd_uint_t,
        src: *mut mpd_uint_t,
        slen: mpd_size_t,
        shift: mpd_size_t,
    ) -> mpd_uint_t;
    static mprime_rdx: mpd_uint_t;
    static mpd_bits: [mpd_size_t; 0];
    static mpd_pow10: [mpd_uint_t; 0];
    fn fnt_convolute(
        c1: *mut mpd_uint_t,
        c2: *mut mpd_uint_t,
        n: mpd_size_t,
        modnum: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fnt_autoconvolute(
        c1: *mut mpd_uint_t,
        n: mpd_size_t,
        modnum: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn crt3(x1: *mut mpd_uint_t, x2: *mut mpd_uint_t, x3: *mut mpd_uint_t, rsize: mpd_size_t);
    fn mpd_switch_to_dyn(
        result: *mut mpd_t,
        nwords: mpd_ssize_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_switch_to_dyn_zero(
        result: *mut mpd_t,
        nwords: mpd_ssize_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_realloc_dyn(
        result: *mut mpd_t,
        nwords: mpd_ssize_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_switch_to_dyn_cxx(result: *mut mpd_t, nwords: mpd_ssize_t) -> ::core::ffi::c_int;
    fn mpd_realloc_dyn_cxx(result: *mut mpd_t, nwords: mpd_ssize_t) -> ::core::ffi::c_int;
}
pub type __uint128_t = u128;
pub type size_t = usize;
pub type FILE = crate::src::shim::FILE;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type mpd_uint_t = uint64_t;
pub type mpd_size_t = size_t;
pub type mpd_ssize_t = int64_t;
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
pub type mpd_triple_class = ::core::ffi::c_uint;
pub const MPD_TRIPLE_ERROR: mpd_triple_class = 4;
pub const MPD_TRIPLE_SNAN: mpd_triple_class = 3;
pub const MPD_TRIPLE_QNAN: mpd_triple_class = 2;
pub const MPD_TRIPLE_INF: mpd_triple_class = 1;
pub const MPD_TRIPLE_NORMAL: mpd_triple_class = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpd_uint128_triple_t {
    pub tag: mpd_triple_class,
    pub sign: uint8_t,
    pub hi: uint64_t,
    pub lo: uint64_t,
    pub exp: int64_t,
}
pub const TO_INT_EXACT: C2RustUnnamed_3 = 0;
pub const TO_INT_SILENT: C2RustUnnamed_3 = 1;
pub const TO_INT_TRUNC: C2RustUnnamed_3 = 2;
pub const P3: C2RustUnnamed_0 = 2;
pub const P2: C2RustUnnamed_0 = 1;
pub const P1: C2RustUnnamed_0 = 0;
pub const SET_IDEAL_EXP: C2RustUnnamed_1 = 1;
pub const DO_FINALIZE: C2RustUnnamed_2 = 1;
pub const NO_IDEAL_EXP: C2RustUnnamed_1 = 0;
pub const SKIP_FINALIZE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT32_MIN: ::core::ffi::c_int = -INT32_MAX - 1 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const INT64_MAX: i64 = 9223372036854775807 as i64;
pub const INT64_MIN: i64 = -INT64_MAX - 1 as i64;
pub const UINT64_MAX: u64 = 18446744073709551615 as u64;
pub const SIZE_MAX: u64 = __SIZE_MAX__;
pub const MPD_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"4.0.1\0") };
pub const MPD_UINT_MAX: u64 = UINT64_MAX;
pub const MPD_SIZE_MAX: u64 = SIZE_MAX;
pub const MPD_SSIZE_MAX: i64 = INT64_MAX;
pub const MPD_SSIZE_MIN: i64 = INT64_MIN;
pub const MPD_RADIX: u64 = 10000000000000000000 as u64;
pub const MPD_RDIGITS: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const MPD_EXPDIGITS: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const MPD_MAXTRANSFORM_2N: u64 = 4294967296 as u64;
pub const MPD_MAX_PREC: i64 = 999999999999999999 as i64;
pub const MPD_MAX_EMAX: i64 = 999999999999999999 as i64;
pub const MPD_MIN_EMIN: i64 = -(999999999999999999 as i64);
pub const MPD_MIN_ETINY: i64 = MPD_MIN_EMIN - (MPD_MAX_PREC - 1 as i64);
pub const MPD_EXP_INF: i64 = 2000000000000000001 as i64;
pub const MPD_EXP_CLAMP: i64 = -(4000000000000000001 as i64);
pub const MPD_Clamped: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const MPD_Conversion_syntax: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const MPD_Division_by_zero: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const MPD_Division_impossible: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const MPD_Division_undefined: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const MPD_Fpu_error: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const MPD_Inexact: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const MPD_Invalid_context: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const MPD_Invalid_operation: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const MPD_Malloc_error: ::core::ffi::c_uint = 0x200 as ::core::ffi::c_uint;
pub const MPD_Overflow: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const MPD_Rounded: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const MPD_Subnormal: ::core::ffi::c_uint = 0x2000 as ::core::ffi::c_uint;
pub const MPD_Underflow: ::core::ffi::c_uint = 0x4000 as ::core::ffi::c_uint;
pub const MPD_IEEE_Invalid_operation: ::core::ffi::c_uint = MPD_Conversion_syntax
    | MPD_Division_impossible
    | MPD_Division_undefined
    | MPD_Fpu_error
    | MPD_Invalid_context
    | MPD_Invalid_operation
    | MPD_Malloc_error;
pub const MPD_Errors: ::core::ffi::c_uint = MPD_IEEE_Invalid_operation | MPD_Division_by_zero;
pub const MPD_Traps: ::core::ffi::c_uint =
    MPD_IEEE_Invalid_operation | MPD_Division_by_zero | MPD_Overflow | MPD_Underflow;
pub const MPD_MINALLOC_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MPD_POS: uint8_t = 0 as uint8_t;
pub const MPD_NEG: uint8_t = 1 as uint8_t;
pub const MPD_INF: uint8_t = 2 as uint8_t;
pub const MPD_NAN: uint8_t = 4 as uint8_t;
pub const MPD_SNAN: uint8_t = 8 as uint8_t;
pub const MPD_SPECIAL: ::core::ffi::c_int =
    MPD_INF as ::core::ffi::c_int | MPD_NAN as ::core::ffi::c_int | MPD_SNAN as ::core::ffi::c_int;
pub const MPD_STATIC: uint8_t = 16 as uint8_t;
pub const MPD_STATIC_DATA: uint8_t = 32 as uint8_t;
pub const MPD_SHARED_DATA: uint8_t = 64 as uint8_t;
pub const MPD_CONST_DATA: uint8_t = 128 as uint8_t;
pub const MPD_DATAFLAGS: ::core::ffi::c_int = MPD_STATIC_DATA as ::core::ffi::c_int
    | MPD_SHARED_DATA as ::core::ffi::c_int
    | MPD_CONST_DATA as ::core::ffi::c_int;
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
unsafe extern "C" fn _mpd_divmod_pow10(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut v: mpd_uint_t,
    mut exp: mpd_uint_t,
) {
    if exp <= 19 as mpd_uint_t {
    } else {
        mpd_c2rust_assert_fail(b"exp <= 19\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if exp <= 9 as mpd_uint_t {
        if exp <= 4 as mpd_uint_t {
            match exp {
                0 => {
                    *q = v;
                    *r = 0 as mpd_uint_t;
                }
                1 => {
                    *q = v.wrapping_div(10 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(10 as mpd_uint_t));
                }
                2 => {
                    *q = v.wrapping_div(100 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(100 as mpd_uint_t));
                }
                3 => {
                    *q = v.wrapping_div(1000 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(1000 as mpd_uint_t));
                }
                4 => {
                    *q = v.wrapping_div(10000 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(10000 as mpd_uint_t));
                }
                _ => {}
            }
        } else {
            match exp {
                5 => {
                    *q = v.wrapping_div(100000 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(100000 as mpd_uint_t));
                }
                6 => {
                    *q = v.wrapping_div(1000000 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(1000000 as mpd_uint_t));
                }
                7 => {
                    *q = v.wrapping_div(10000000 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(10000000 as mpd_uint_t));
                }
                8 => {
                    *q = v.wrapping_div(100000000 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(100000000 as mpd_uint_t));
                }
                9 => {
                    *q = v.wrapping_div(1000000000 as mpd_uint_t);
                    *r = v.wrapping_sub((*q).wrapping_mul(1000000000 as mpd_uint_t));
                }
                _ => {}
            }
        }
    } else if exp <= 14 as mpd_uint_t {
        match exp {
            10 => {
                *q = v.wrapping_div(10000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(10000000000 as mpd_uint_t));
            }
            11 => {
                *q = v.wrapping_div(100000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(100000000000 as mpd_uint_t));
            }
            12 => {
                *q = v.wrapping_div(1000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(1000000000000 as mpd_uint_t));
            }
            13 => {
                *q = v.wrapping_div(10000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(10000000000000 as mpd_uint_t));
            }
            14 => {
                *q = v.wrapping_div(100000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(100000000000000 as mpd_uint_t));
            }
            _ => {}
        }
    } else {
        match exp {
            15 => {
                *q = v.wrapping_div(1000000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(1000000000000000 as mpd_uint_t));
            }
            16 => {
                *q = v.wrapping_div(10000000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(10000000000000000 as mpd_uint_t));
            }
            17 => {
                *q = v.wrapping_div(100000000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(100000000000000000 as mpd_uint_t));
            }
            18 => {
                *q = v.wrapping_div(1000000000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(1000000000000000000 as mpd_uint_t));
            }
            19 => {
                *q = v.wrapping_div(10000000000000000000 as mpd_uint_t);
                *r = v.wrapping_sub((*q).wrapping_mul(10000000000000000000 as mpd_uint_t));
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
unsafe extern "C" fn _mpd_idiv_word(
    mut q: *mut mpd_ssize_t,
    mut r: *mut mpd_ssize_t,
    mut v: mpd_ssize_t,
    mut d: mpd_ssize_t,
) {
    *q = v / d;
    *r = v - *q * d;
}
unsafe extern "C" fn add_size_t(mut a: mpd_size_t, mut b: mpd_size_t) -> mpd_size_t {
    if a > (MPD_SIZE_MAX as mpd_size_t).wrapping_sub(b) {
        fprintf(
            mpd_c2rust_stderr(),
            b"%s:%d: error: \0" as *const u8 as *const ::core::ffi::c_char,
            b"libmpdec/typearith.h\0" as *const u8 as *const ::core::ffi::c_char,
            592 as ::core::ffi::c_int,
        );
        fprintf(
            mpd_c2rust_stderr(),
            b"add_size_t(): overflow: check the context\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fputc('\n' as i32, mpd_c2rust_stderr());
        abort();
    }
    return a.wrapping_add(b);
}
unsafe extern "C" fn mul_size_t(mut a: mpd_size_t, mut b: mpd_size_t) -> mpd_size_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi, &raw mut lo, a as mpd_uint_t, b as mpd_uint_t);
    if hi != 0 {
        fprintf(
            mpd_c2rust_stderr(),
            b"%s:%d: error: \0" as *const u8 as *const ::core::ffi::c_char,
            b"libmpdec/typearith.h\0" as *const u8 as *const ::core::ffi::c_char,
            617 as ::core::ffi::c_int,
        );
        fprintf(
            mpd_c2rust_stderr(),
            b"mul_size_t(): overflow: check the context\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        fputc('\n' as i32, mpd_c2rust_stderr());
        abort();
    }
    return lo as mpd_size_t;
}
unsafe extern "C" fn _mpd_div_words_r(
    mut q: *mut mpd_uint_t,
    mut r: *mut mpd_uint_t,
    mut hi: mpd_uint_t,
    mut lo: mpd_uint_t,
) {
    let mut n_adj: mpd_uint_t = 0;
    let mut h: mpd_uint_t = 0;
    let mut l: mpd_uint_t = 0;
    let mut t: mpd_uint_t = 0;
    let mut n1_neg: mpd_uint_t = 0;
    n1_neg = (if lo & (1 as mpd_uint_t) << 63 as ::core::ffi::c_int != 0 {
        MPD_UINT_MAX
    } else {
        0 as u64
    }) as mpd_uint_t;
    n_adj = lo.wrapping_add(n1_neg & MPD_RADIX as mpd_uint_t);
    _mpd_mul_words(&raw mut h, &raw mut l, mprime_rdx, hi.wrapping_sub(n1_neg));
    l = l.wrapping_add(n_adj);
    if l < n_adj {
        h = h.wrapping_add(1);
    }
    t = h.wrapping_add(hi);
    t = (MPD_UINT_MAX as mpd_uint_t).wrapping_sub(t);
    _mpd_mul_words(&raw mut h, &raw mut l, t, MPD_RADIX as mpd_uint_t);
    l = l.wrapping_add(lo);
    if l < lo {
        h = h.wrapping_add(1);
    }
    h = h.wrapping_add(hi);
    h = (h as u64).wrapping_sub(MPD_RADIX) as mpd_uint_t;
    *q = h.wrapping_sub(t);
    *r = l.wrapping_add(MPD_RADIX as mpd_uint_t & h);
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
unsafe extern "C" fn _mpd_mul_2_le2(
    mut w: *mut mpd_uint_t,
    mut u: *mut mpd_uint_t,
    mut v: *mut mpd_uint_t,
    mut m: mpd_ssize_t,
) {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    _mpd_mul_words(
        &raw mut hi,
        &raw mut lo,
        *u.offset(0 as isize),
        *v.offset(0 as isize),
    );
    _mpd_div_words_r(
        w.offset(1 as isize) as *mut mpd_uint_t,
        w.offset(0 as isize) as *mut mpd_uint_t,
        hi,
        lo,
    );
    _mpd_mul_words(
        &raw mut hi,
        &raw mut lo,
        *u.offset(1 as isize),
        *v.offset(0 as isize),
    );
    lo = (*w.offset(1 as isize)).wrapping_add(lo);
    if lo < *w.offset(1 as isize) {
        hi = hi.wrapping_add(1);
    }
    _mpd_div_words_r(
        w.offset(2 as isize) as *mut mpd_uint_t,
        w.offset(1 as isize) as *mut mpd_uint_t,
        hi,
        lo,
    );
    if m == 1 as mpd_ssize_t {
        return;
    }
    _mpd_mul_words(
        &raw mut hi,
        &raw mut lo,
        *u.offset(0 as isize),
        *v.offset(1 as isize),
    );
    lo = (*w.offset(1 as isize)).wrapping_add(lo);
    if lo < *w.offset(1 as isize) {
        hi = hi.wrapping_add(1);
    }
    _mpd_div_words_r(
        w.offset(3 as isize) as *mut mpd_uint_t,
        w.offset(1 as isize) as *mut mpd_uint_t,
        hi,
        lo,
    );
    _mpd_mul_words(
        &raw mut hi,
        &raw mut lo,
        *u.offset(1 as isize),
        *v.offset(1 as isize),
    );
    lo = (*w.offset(2 as isize)).wrapping_add(lo);
    if lo < *w.offset(2 as isize) {
        hi = hi.wrapping_add(1);
    }
    lo = (*w.offset(3 as isize)).wrapping_add(lo);
    if lo < *w.offset(3 as isize) {
        hi = hi.wrapping_add(1);
    }
    _mpd_div_words_r(
        w.offset(3 as isize) as *mut mpd_uint_t,
        w.offset(2 as isize) as *mut mpd_uint_t,
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
unsafe extern "C" fn _mpd_isallnine(
    mut data: *const mpd_uint_t,
    mut len: mpd_ssize_t,
) -> ::core::ffi::c_int {
    loop {
        len -= 1;
        if !(len >= 0 as mpd_ssize_t) {
            break;
        }
        if *data.offset(len as isize) != (MPD_RADIX as mpd_uint_t).wrapping_sub(1 as mpd_uint_t) {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
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
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const MPD_NEWTONDIV_CUTOFF: i64 = 1024 as i64;
static mut data_one: [mpd_uint_t; 1] = [1 as mpd_uint_t];
static mut data_zero: [mpd_uint_t; 1] = [0 as mpd_uint_t];
static mut one: mpd_t = unsafe {
    mpd_t {
        flags: (MPD_STATIC as ::core::ffi::c_int | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw const data_one as *mut mpd_uint_t,
    }
};
static mut minus_one: mpd_t = unsafe {
    mpd_t {
        flags: (MPD_NEG as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw const data_one as *mut mpd_uint_t,
    }
};
static mut zero: mpd_t = unsafe {
    mpd_t {
        flags: (MPD_STATIC as ::core::ffi::c_int | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw const data_zero as *mut mpd_uint_t,
    }
};
#[no_mangle]
pub unsafe extern "C" fn mpd_set_func() {
    func_count = 0 as ::core::ffi::c_int;
    func_fail = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut func_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut func_fail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn mpd_version() -> *const ::core::ffi::c_char {
    return MPD_VERSION.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_func_fail(mut n: ::core::ffi::c_int) {
    func_count = 0 as ::core::ffi::c_int;
    func_fail = n;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_word_digits(mut word: mpd_uint_t) -> ::core::ffi::c_int {
    if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(9 as isize) {
        if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(4 as isize) {
            if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(2 as isize) {
                return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(1 as isize) {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                };
            }
            return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(3 as isize) {
                3 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            };
        }
        if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(6 as isize) {
            return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(5 as isize) {
                5 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
            };
        }
        if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(8 as isize) {
            return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(7 as isize) {
                7 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            };
        }
        return 9 as ::core::ffi::c_int;
    }
    if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(14 as isize) {
        if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(11 as isize) {
            return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(10 as isize) {
                10 as ::core::ffi::c_int
            } else {
                11 as ::core::ffi::c_int
            };
        }
        if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(13 as isize) {
            return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(12 as isize) {
                12 as ::core::ffi::c_int
            } else {
                13 as ::core::ffi::c_int
            };
        }
        return 14 as ::core::ffi::c_int;
    }
    if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(18 as isize) {
        if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(16 as isize) {
            return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(15 as isize) {
                15 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            };
        }
        return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(17 as isize) {
            17 as ::core::ffi::c_int
        } else {
            18 as ::core::ffi::c_int
        };
    }
    return if word < *(&raw const mpd_pow10 as *const mpd_uint_t).offset(19 as isize) {
        19 as ::core::ffi::c_int
    } else {
        20 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_adjexp(mut dec: *const mpd_t) -> mpd_ssize_t {
    return (*dec).exp + (*dec).digits - 1 as mpd_ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_etiny(mut ctx: *const mpd_context_t) -> mpd_ssize_t {
    return (*ctx).emin - ((*ctx).prec - 1 as mpd_ssize_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_etop(mut ctx: *const mpd_context_t) -> mpd_ssize_t {
    return (*ctx).emax - ((*ctx).prec - 1 as mpd_ssize_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_msword(mut dec: *const mpd_t) -> mpd_uint_t {
    if (*dec).len > 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(b"dec->len > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    return *(*dec).data.offset(((*dec).len - 1 as mpd_ssize_t) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_msd(mut word: mpd_uint_t) -> mpd_uint_t {
    let mut n: ::core::ffi::c_int = 0;
    n = mpd_word_digits(word);
    return word.wrapping_div(
        *(&raw const mpd_pow10 as *const mpd_uint_t).offset((n - 1 as ::core::ffi::c_int) as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpd_lsd(mut word: mpd_uint_t) -> mpd_uint_t {
    return word.wrapping_rem(10 as mpd_uint_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_digits_to_size(mut digits: mpd_ssize_t) -> mpd_ssize_t {
    let mut q: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    _mpd_idiv_word(&raw mut q, &raw mut r, digits, MPD_RDIGITS as mpd_ssize_t);
    return if r == 0 as mpd_ssize_t {
        q
    } else {
        q + 1 as mpd_ssize_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_exp_digits(mut exp: mpd_ssize_t) -> ::core::ffi::c_int {
    exp = if exp < 0 as mpd_ssize_t { -exp } else { exp };
    return mpd_word_digits(exp as mpd_uint_t);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_iscanonical(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isfinite(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return ((*dec).flags as ::core::ffi::c_int & MPD_SPECIAL == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isinfinite(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_INF as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isnan(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int
        & (MPD_NAN as ::core::ffi::c_int | MPD_SNAN as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isnegative(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_NEG as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_ispositive(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return ((*dec).flags as ::core::ffi::c_int & MPD_NEG as ::core::ffi::c_int == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isqnan(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_NAN as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_issigned(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_NEG as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_issnan(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_SNAN as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isspecial(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_SPECIAL;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_iszero(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (mpd_isspecial(dec) == 0 && mpd_msword(dec) == 0 as mpd_uint_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_iszerocoeff(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (mpd_msword(dec) == 0 as mpd_uint_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isnormal(
    mut dec: *const mpd_t,
    mut ctx: *const mpd_context_t,
) -> ::core::ffi::c_int {
    if mpd_isspecial(dec) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if mpd_iszerocoeff(dec) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return (mpd_adjexp(dec) >= (*ctx).emin) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_issubnormal(
    mut dec: *const mpd_t,
    mut ctx: *const mpd_context_t,
) -> ::core::ffi::c_int {
    if mpd_isspecial(dec) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if mpd_iszerocoeff(dec) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return (mpd_adjexp(dec) < (*ctx).emin) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isoddword(mut word: mpd_uint_t) -> ::core::ffi::c_int {
    return (word & 1 as mpd_uint_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isoddcoeff(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return mpd_isoddword(*(*dec).data.offset(0 as isize));
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sign(mut dec: *const mpd_t) -> uint8_t {
    return ((*dec).flags as ::core::ffi::c_int & MPD_NEG as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_arith_sign(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int * mpd_isnegative(dec);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_radix() -> ::core::ffi::c_long {
    return 10 as ::core::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isdynamic(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return ((*dec).flags as ::core::ffi::c_int & MPD_STATIC as ::core::ffi::c_int == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isstatic(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_STATIC as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isdynamic_data(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return ((*dec).flags as ::core::ffi::c_int & MPD_DATAFLAGS == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isstatic_data(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_STATIC_DATA as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isshared_data(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_SHARED_DATA as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isconst_data(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (*dec).flags as ::core::ffi::c_int & MPD_CONST_DATA as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_uint_zero(mut dest: *mut mpd_uint_t, mut len: mpd_size_t) {
    let mut i: mpd_size_t = 0;
    i = 0 as mpd_size_t;
    while i < len {
        *dest.offset(i as isize) = 0 as mpd_uint_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_del(mut dec: *mut mpd_t) {
    if mpd_isdynamic_data(dec) != 0 {
        mpd_free.expect("non-null function pointer")((*dec).data as *mut ::core::ffi::c_void);
    }
    if mpd_isdynamic(dec) != 0 {
        mpd_free.expect("non-null function pointer")(dec as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qresize(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if mpd_isconst_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isconst_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if mpd_isshared_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isshared_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if MPD_MINALLOC <= (*result).alloc {
    } else {
        mpd_c2rust_assert_fail(
            b"MPD_MINALLOC <= result->alloc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    nwords = if nwords <= MPD_MINALLOC {
        MPD_MINALLOC
    } else {
        nwords
    };
    if nwords == (*result).alloc {
        return 1 as ::core::ffi::c_int;
    }
    if mpd_isstatic_data(result) != 0 {
        if nwords > (*result).alloc {
            return mpd_switch_to_dyn(result, nwords, status);
        }
        return 1 as ::core::ffi::c_int;
    }
    return mpd_realloc_dyn(result, nwords, status);
}
unsafe extern "C" fn mpd_qresize_cxx(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
) -> ::core::ffi::c_int {
    if mpd_isconst_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isconst_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if mpd_isshared_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isshared_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if MPD_MINALLOC <= (*result).alloc {
    } else {
        mpd_c2rust_assert_fail(
            b"MPD_MINALLOC <= result->alloc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    nwords = if nwords <= MPD_MINALLOC {
        MPD_MINALLOC
    } else {
        nwords
    };
    if nwords == (*result).alloc {
        return 1 as ::core::ffi::c_int;
    }
    if mpd_isstatic_data(result) != 0 {
        if nwords > (*result).alloc {
            return mpd_switch_to_dyn_cxx(result, nwords);
        }
        return 1 as ::core::ffi::c_int;
    }
    return mpd_realloc_dyn_cxx(result, nwords);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qresize_zero(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if mpd_isconst_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isconst_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if mpd_isshared_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isshared_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if MPD_MINALLOC <= (*result).alloc {
    } else {
        mpd_c2rust_assert_fail(
            b"MPD_MINALLOC <= result->alloc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    nwords = if nwords <= MPD_MINALLOC {
        MPD_MINALLOC
    } else {
        nwords
    };
    if nwords != (*result).alloc {
        if mpd_isstatic_data(result) != 0 {
            if nwords > (*result).alloc {
                return mpd_switch_to_dyn_zero(result, nwords, status);
            }
        } else if mpd_realloc_dyn(result, nwords, status) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    mpd_uint_zero((*result).data, nwords as mpd_size_t);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_minalloc(mut result: *mut mpd_t) {
    if mpd_isconst_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isconst_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if mpd_isshared_data(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isshared_data(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if mpd_isstatic_data(result) == 0 && (*result).alloc > MPD_MINALLOC {
        let mut err: uint8_t = 0 as uint8_t;
        (*result).data = mpd_realloc(
            (*result).data as *mut ::core::ffi::c_void,
            MPD_MINALLOC as mpd_size_t,
            ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
            &raw mut err,
        ) as *mut mpd_uint_t;
        if err == 0 {
            (*result).alloc = MPD_MINALLOC;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_resize(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qresize(result, nwords, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_resize_zero(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qresize_zero(result, nwords, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_setdigits(mut result: *mut mpd_t) {
    let mut wdigits: mpd_ssize_t = mpd_word_digits(mpd_msword(result)) as mpd_ssize_t;
    (*result).digits = wdigits + ((*result).len - 1 as mpd_ssize_t) * MPD_RDIGITS as mpd_ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_sign(mut result: *mut mpd_t, mut sign: uint8_t) {
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int & !(MPD_NEG as ::core::ffi::c_int)) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | sign as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_signcpy(mut result: *mut mpd_t, mut a: *const mpd_t) {
    let mut sign: uint8_t =
        ((*a).flags as ::core::ffi::c_int & MPD_NEG as ::core::ffi::c_int) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int & !(MPD_NEG as ::core::ffi::c_int)) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | sign as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_infinity(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int & !MPD_SPECIAL) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_INF as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_qnan(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int & !MPD_SPECIAL) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_NAN as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_snan(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int & !MPD_SPECIAL) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_SNAN as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_negative(mut result: *mut mpd_t) {
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_NEG as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_positive(mut result: *mut mpd_t) {
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int & !(MPD_NEG as ::core::ffi::c_int)) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_dynamic(mut result: *mut mpd_t) {
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int & !(MPD_STATIC as ::core::ffi::c_int)) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_static(mut result: *mut mpd_t) {
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_STATIC as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_dynamic_data(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int & !MPD_DATAFLAGS) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_static_data(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int & !MPD_DATAFLAGS) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_shared_data(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int & !MPD_DATAFLAGS) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_SHARED_DATA as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_const_data(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int & !MPD_DATAFLAGS) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_clear_flags(mut result: *mut mpd_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int
        & (MPD_STATIC as ::core::ffi::c_int | MPD_DATAFLAGS)) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_flags(mut result: *mut mpd_t, mut flags: uint8_t) {
    (*result).flags = ((*result).flags as ::core::ffi::c_int
        & (MPD_STATIC as ::core::ffi::c_int | MPD_DATAFLAGS)) as uint8_t;
    (*result).flags =
        ((*result).flags as ::core::ffi::c_int | flags as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_copy_flags(mut result: *mut mpd_t, mut a: *const mpd_t) {
    let mut aflags: uint8_t = (*a).flags;
    (*result).flags = ((*result).flags as ::core::ffi::c_int
        & (MPD_STATIC as ::core::ffi::c_int | MPD_DATAFLAGS)) as uint8_t;
    (*result).flags = ((*result).flags as ::core::ffi::c_int
        | aflags as ::core::ffi::c_int & !(MPD_STATIC as ::core::ffi::c_int | MPD_DATAFLAGS))
        as uint8_t;
}
unsafe extern "C" fn mpd_workcontext(
    mut workctx: *mut mpd_context_t,
    mut ctx: *const mpd_context_t,
) {
    (*workctx).prec = (*ctx).prec;
    (*workctx).emax = (*ctx).emax;
    (*workctx).emin = (*ctx).emin;
    (*workctx).round = (*ctx).round;
    (*workctx).traps = 0 as uint32_t;
    (*workctx).status = 0 as uint32_t;
    (*workctx).newtrap = 0 as uint32_t;
    (*workctx).clamp = (*ctx).clamp;
    (*workctx).allcr = (*ctx).allcr;
}
unsafe extern "C" fn _mpd_negate(mut dec: *mut mpd_t) {
    (*dec).flags = ((*dec).flags as ::core::ffi::c_int ^ MPD_NEG as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_zerocoeff(mut result: *mut mpd_t) {
    mpd_minalloc(result);
    (*result).digits = 1 as mpd_ssize_t;
    (*result).len = 1 as mpd_ssize_t;
    *(*result).data.offset(0 as isize) = 0 as mpd_uint_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmaxcoeff(
    mut result: *mut mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut len: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    _mpd_idiv_word(
        &raw mut len,
        &raw mut r,
        (*ctx).prec,
        MPD_RDIGITS as mpd_ssize_t,
    );
    len = if r == 0 as mpd_ssize_t {
        len
    } else {
        len + 1 as mpd_ssize_t
    };
    if mpd_qresize(result, len, status) == 0 {
        return;
    }
    (*result).len = len;
    (*result).digits = (*ctx).prec;
    len -= 1;
    if r > 0 as mpd_ssize_t {
        let fresh0 = len;
        len = len - 1;
        *(*result).data.offset(fresh0 as isize) = (*(&raw const mpd_pow10 as *const mpd_uint_t)
            .offset(r as isize))
        .wrapping_sub(1 as mpd_uint_t);
    }
    while len >= 0 as mpd_ssize_t {
        *(*result).data.offset(len as isize) = MPD_RADIX.wrapping_sub(1 as u64) as mpd_uint_t;
        len -= 1;
    }
}
unsafe extern "C" fn _mpd_cap(mut result: *mut mpd_t, mut ctx: *const mpd_context_t) {
    let mut dummy: uint32_t = 0;
    let mut len: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    if (*result).len > 0 as mpd_ssize_t && (*result).digits > (*ctx).prec {
        _mpd_idiv_word(
            &raw mut len,
            &raw mut r,
            (*ctx).prec,
            MPD_RDIGITS as mpd_ssize_t,
        );
        len = if r == 0 as mpd_ssize_t {
            len
        } else {
            len + 1 as mpd_ssize_t
        };
        if r != 0 as mpd_ssize_t {
            let ref mut fresh0 = *(*result).data.offset((len - 1 as mpd_ssize_t) as isize);
            *fresh0 = (*fresh0)
                .wrapping_rem(*(&raw const mpd_pow10 as *const mpd_uint_t).offset(r as isize));
        }
        len = _mpd_real_size((*result).data, len);
        mpd_qresize(result, len, &raw mut dummy);
        (*result).len = len;
        mpd_setdigits(result);
    }
    if mpd_iszero(result) != 0 {
        _settriple(result, mpd_sign(result), 0 as mpd_uint_t, (*result).exp);
    }
}
unsafe extern "C" fn _mpd_fix_nan(mut result: *mut mpd_t, mut ctx: *const mpd_context_t) {
    let mut dummy: uint32_t = 0;
    let mut prec: mpd_ssize_t = 0;
    let mut len: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    prec = (*ctx).prec - (*ctx).clamp as mpd_ssize_t;
    if (*result).len > 0 as mpd_ssize_t && (*result).digits > prec {
        if prec == 0 as mpd_ssize_t {
            mpd_minalloc(result);
            (*result).digits = 0 as mpd_ssize_t;
            (*result).len = (*result).digits;
        } else {
            _mpd_idiv_word(&raw mut len, &raw mut r, prec, MPD_RDIGITS as mpd_ssize_t);
            len = if r == 0 as mpd_ssize_t {
                len
            } else {
                len + 1 as mpd_ssize_t
            };
            if r != 0 as mpd_ssize_t {
                let ref mut fresh0 = *(*result).data.offset((len - 1 as mpd_ssize_t) as isize);
                *fresh0 = (*fresh0)
                    .wrapping_rem(*(&raw const mpd_pow10 as *const mpd_uint_t).offset(r as isize));
            }
            len = _mpd_real_size((*result).data, len);
            mpd_qresize(result, len, &raw mut dummy);
            (*result).len = len;
            mpd_setdigits(result);
            if mpd_iszerocoeff(result) != 0 {
                (*result).digits = 0 as mpd_ssize_t;
                (*result).len = (*result).digits;
            }
        }
    }
}
unsafe extern "C" fn _mpd_get_msdigits(
    mut hi: *mut mpd_uint_t,
    mut lo: *mut mpd_uint_t,
    mut dec: *const mpd_t,
    mut n: ::core::ffi::c_uint,
) {
    let mut r: mpd_uint_t = 0;
    let mut tmp: mpd_uint_t = 0;
    if (0 as ::core::ffi::c_uint) < n
        && n <= (19 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
    } else {
        mpd_c2rust_assert_fail(
            b"0 < n && n <= MPD_RDIGITS+1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    _mpd_div_word(
        &raw mut tmp,
        &raw mut r,
        (*dec).digits as mpd_uint_t,
        MPD_RDIGITS as mpd_uint_t,
    );
    r = if r == 0 as mpd_uint_t {
        MPD_RDIGITS as mpd_uint_t
    } else {
        r
    };
    *hi = 0 as mpd_uint_t;
    *lo = *(*dec).data.offset(((*dec).len - 1 as mpd_ssize_t) as isize);
    if n as mpd_uint_t <= r {
        *lo = (*lo).wrapping_div(
            *(&raw const mpd_pow10 as *const mpd_uint_t)
                .offset(r.wrapping_sub(n as mpd_uint_t) as isize),
        );
    } else if (*dec).len > 1 as mpd_ssize_t {
        _mpd_mul_words(
            hi,
            lo,
            *lo,
            *(&raw const mpd_pow10 as *const mpd_uint_t)
                .offset((n as mpd_uint_t).wrapping_sub(r) as isize),
        );
        tmp = (*(*dec).data.offset(((*dec).len - 2 as mpd_ssize_t) as isize)).wrapping_div(
            *(&raw const mpd_pow10 as *const mpd_uint_t).offset(
                (MPD_RDIGITS as mpd_uint_t).wrapping_sub((n as mpd_uint_t).wrapping_sub(r))
                    as isize,
            ),
        );
        *lo = (*lo).wrapping_add(tmp);
        if *lo < tmp {
            *hi = (*hi).wrapping_add(1);
        }
    }
}
unsafe extern "C" fn _mpd_real_size(
    mut data: *mut mpd_uint_t,
    mut size: mpd_ssize_t,
) -> mpd_ssize_t {
    while size > 1 as mpd_ssize_t
        && *data.offset((size - 1 as mpd_ssize_t) as isize) == 0 as mpd_uint_t
    {
        size -= 1;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_trail_zeros(mut dec: *const mpd_t) -> mpd_ssize_t {
    let mut word: mpd_uint_t = 0;
    let mut i: mpd_ssize_t = 0;
    let mut tz: mpd_ssize_t = 0 as mpd_ssize_t;
    i = 0 as mpd_ssize_t;
    while i < (*dec).len {
        if *(*dec).data.offset(i as isize) != 0 as mpd_uint_t {
            word = *(*dec).data.offset(i as isize);
            tz = i * MPD_RDIGITS as mpd_ssize_t;
            while word.wrapping_rem(10 as mpd_uint_t) == 0 as mpd_uint_t {
                word = word.wrapping_div(10 as mpd_uint_t);
                tz += 1;
            }
            break;
        } else {
            i += 1;
        }
    }
    return tz;
}
unsafe extern "C" fn _mpd_isint(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    let mut tz: mpd_ssize_t = 0;
    if mpd_iszerocoeff(dec) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    tz = mpd_trail_zeros(dec);
    return ((*dec).exp + tz >= 0 as mpd_ssize_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isinteger(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    if mpd_isspecial(dec) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return _mpd_isint(dec);
}
unsafe extern "C" fn mpd_word_ispow10(mut word: mpd_uint_t) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    n = mpd_word_digits(word);
    if word
        == *(&raw const mpd_pow10 as *const mpd_uint_t)
            .offset((n - 1 as ::core::ffi::c_int) as isize)
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mpd_coeff_ispow10(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    if mpd_word_ispow10(mpd_msword(dec)) != 0 {
        if _mpd_isallzero((*dec).data, (*dec).len - 1 as mpd_ssize_t) != 0 {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mpd_word_isallnine(mut word: mpd_uint_t) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    n = mpd_word_digits(word);
    if word
        == (*(&raw const mpd_pow10 as *const mpd_uint_t).offset(n as isize))
            .wrapping_sub(1 as mpd_uint_t)
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mpd_coeff_isallnine(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    if mpd_word_isallnine(mpd_msword(dec)) != 0 {
        if _mpd_isallnine((*dec).data, (*dec).len - 1 as mpd_ssize_t) != 0 {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_isodd(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    let mut q: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    if mpd_isinteger(dec) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"mpd_isinteger(dec)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszerocoeff(dec) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*dec).exp < 0 as mpd_ssize_t {
        _mpd_div_word(
            &raw mut q,
            &raw mut r,
            -(*dec).exp as mpd_uint_t,
            MPD_RDIGITS as mpd_uint_t,
        );
        q = (*(*dec).data.offset(q as isize))
            .wrapping_div(*(&raw const mpd_pow10 as *const mpd_uint_t).offset(r as isize));
        return mpd_isoddword(q);
    }
    return ((*dec).exp == 0 as mpd_ssize_t && mpd_isoddword(*(*dec).data.offset(0 as isize)) != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_iseven(mut dec: *const mpd_t) -> ::core::ffi::c_int {
    return (mpd_isodd(dec) == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn _ssettriple(
    mut result: *mut mpd_t,
    mut sign: uint8_t,
    mut a: mpd_uint_t,
    mut exp: mpd_ssize_t,
) {
    mpd_set_flags(result, sign);
    (*result).exp = exp;
    _mpd_div_word(
        (*result).data.offset(1 as isize) as *mut mpd_uint_t,
        (*result).data.offset(0 as isize) as *mut mpd_uint_t,
        a,
        MPD_RADIX as mpd_uint_t,
    );
    (*result).len = (if *(*result).data.offset(1 as isize) == 0 as mpd_uint_t {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    mpd_setdigits(result);
}
unsafe extern "C" fn _settriple(
    mut result: *mut mpd_t,
    mut sign: uint8_t,
    mut a: mpd_uint_t,
    mut exp: mpd_ssize_t,
) {
    mpd_minalloc(result);
    mpd_set_flags(result, sign);
    (*result).exp = exp;
    _mpd_div_word(
        (*result).data.offset(1 as isize) as *mut mpd_uint_t,
        (*result).data.offset(0 as isize) as *mut mpd_uint_t,
        a,
        MPD_RADIX as mpd_uint_t,
    );
    (*result).len = (if *(*result).data.offset(1 as isize) == 0 as mpd_uint_t {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    mpd_setdigits(result);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_setspecial(
    mut result: *mut mpd_t,
    mut sign: uint8_t,
    mut type_0: uint8_t,
) {
    mpd_minalloc(result);
    (*result).flags = ((*result).flags as ::core::ffi::c_int
        & !(MPD_NEG as ::core::ffi::c_int | MPD_SPECIAL)) as uint8_t;
    (*result).flags = ((*result).flags as ::core::ffi::c_int
        | (sign as ::core::ffi::c_int | type_0 as ::core::ffi::c_int))
        as uint8_t;
    (*result).len = 0 as mpd_ssize_t;
    (*result).digits = (*result).len;
    (*result).exp = (*result).digits;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_seterror(
    mut result: *mut mpd_t,
    mut flags: uint32_t,
    mut status: *mut uint32_t,
) {
    mpd_minalloc(result);
    mpd_set_qnan(result);
    mpd_set_positive(result);
    (*result).len = 0 as mpd_ssize_t;
    (*result).digits = (*result).len;
    (*result).exp = (*result).digits;
    *status |= flags;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsset_ssize(
    mut result: *mut mpd_t,
    mut a: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut u: mpd_uint_t = 0;
    let mut sign: uint8_t = MPD_POS;
    if a < 0 as mpd_ssize_t {
        if a == MPD_SSIZE_MIN as mpd_ssize_t {
            u = (MPD_SSIZE_MAX as mpd_uint_t)
                .wrapping_add(-(MPD_SSIZE_MIN + MPD_SSIZE_MAX) as mpd_uint_t);
        } else {
            u = -a as mpd_uint_t;
        }
        sign = MPD_NEG;
    } else {
        u = a as mpd_uint_t;
    }
    _ssettriple(result, sign, u, 0 as mpd_ssize_t);
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsset_uint(
    mut result: *mut mpd_t,
    mut a: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _ssettriple(result, MPD_POS, a, 0 as mpd_ssize_t);
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsset_i32(
    mut result: *mut mpd_t,
    mut a: int32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsset_ssize(result, a as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsset_u32(
    mut result: *mut mpd_t,
    mut a: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsset_uint(result, a as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsset_i64(
    mut result: *mut mpd_t,
    mut a: int64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsset_ssize(result, a as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsset_u64(
    mut result: *mut mpd_t,
    mut a: uint64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsset_uint(result, a as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_ssize(
    mut result: *mut mpd_t,
    mut a: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_minalloc(result);
    mpd_qsset_ssize(result, a, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_uint(
    mut result: *mut mpd_t,
    mut a: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _settriple(result, MPD_POS, a, 0 as mpd_ssize_t);
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_i32(
    mut result: *mut mpd_t,
    mut a: int32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qset_ssize(result, a as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_u32(
    mut result: *mut mpd_t,
    mut a: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qset_uint(result, a as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_i64(
    mut result: *mut mpd_t,
    mut a: int64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qset_ssize(result, a as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_i64_exact(
    mut result: *mut mpd_t,
    mut a: int64_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut workstatus: uint32_t = 0 as uint32_t;
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qset_ssize(
        result,
        a as mpd_ssize_t,
        &raw mut maxcontext,
        &raw mut workstatus,
    );
    if workstatus & (MPD_Inexact as uint32_t | MPD_Rounded as uint32_t | MPD_Clamped as uint32_t)
        != 0
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
    }
    *status = (*status as ::core::ffi::c_uint
        | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_u64(
    mut result: *mut mpd_t,
    mut a: uint64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qset_uint(result, a as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qset_u64_exact(
    mut result: *mut mpd_t,
    mut a: uint64_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut workstatus: uint32_t = 0 as uint32_t;
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qset_uint(
        result,
        a as mpd_uint_t,
        &raw mut maxcontext,
        &raw mut workstatus,
    );
    if workstatus & (MPD_Inexact as uint32_t | MPD_Rounded as uint32_t | MPD_Clamped as uint32_t)
        != 0
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
    }
    *status = (*status as ::core::ffi::c_uint
        | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint) as uint32_t;
}
unsafe extern "C" fn _mpd_qget_uint(
    mut use_sign: ::core::ffi::c_int,
    mut a: *const mpd_t,
    mut status: *mut uint32_t,
) -> mpd_uint_t {
    let mut tmp: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    let mut tmp_data: [mpd_uint_t; 2] = [0; 2];
    let mut lo: mpd_uint_t = 0;
    let mut hi: mpd_uint_t = 0;
    if mpd_isspecial(a) != 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return MPD_UINT_MAX as mpd_uint_t;
    }
    if mpd_iszero(a) != 0 {
        return 0 as mpd_uint_t;
    }
    if use_sign != 0 && mpd_isnegative(a) != 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return MPD_UINT_MAX as mpd_uint_t;
    }
    if (*a).digits + (*a).exp > (MPD_RDIGITS + 1 as ::core::ffi::c_int) as mpd_ssize_t {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return MPD_UINT_MAX as mpd_uint_t;
    }
    if (*a).exp < 0 as mpd_ssize_t {
        if _mpd_isint(a) == 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return MPD_UINT_MAX as mpd_uint_t;
        }
        tmp.data = &raw mut tmp_data as *mut mpd_uint_t;
        tmp.flags =
            (MPD_STATIC as ::core::ffi::c_int | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t;
        tmp.alloc = 2 as mpd_ssize_t;
        mpd_qsshiftr(&raw mut tmp, a, -(*a).exp);
        tmp.exp = 0 as mpd_ssize_t;
        a = &raw mut tmp;
    }
    _mpd_get_msdigits(
        &raw mut hi,
        &raw mut lo,
        a,
        (MPD_RDIGITS + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    if hi != 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return MPD_UINT_MAX as mpd_uint_t;
    }
    if (*a).exp > 0 as mpd_ssize_t {
        _mpd_mul_words(
            &raw mut hi,
            &raw mut lo,
            lo,
            *(&raw const mpd_pow10 as *const mpd_uint_t).offset((*a).exp as isize),
        );
        if hi != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return MPD_UINT_MAX as mpd_uint_t;
        }
    }
    return lo;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qget_uint(
    mut a: *const mpd_t,
    mut status: *mut uint32_t,
) -> mpd_uint_t {
    return _mpd_qget_uint(1 as ::core::ffi::c_int, a, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qabs_uint(
    mut a: *const mpd_t,
    mut status: *mut uint32_t,
) -> mpd_uint_t {
    return _mpd_qget_uint(0 as ::core::ffi::c_int, a, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qget_ssize(
    mut a: *const mpd_t,
    mut status: *mut uint32_t,
) -> mpd_ssize_t {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut u: mpd_uint_t = 0;
    let mut isneg: ::core::ffi::c_int = 0;
    u = mpd_qabs_uint(a, &raw mut workstatus);
    if workstatus & MPD_Invalid_operation as uint32_t != 0 {
        *status |= workstatus;
        return MPD_SSIZE_MAX as mpd_ssize_t;
    }
    isneg = mpd_isnegative(a);
    if u <= MPD_SSIZE_MAX as mpd_uint_t {
        return if isneg != 0 {
            -(u as mpd_ssize_t)
        } else {
            u as mpd_ssize_t
        };
    } else if isneg != 0
        && u.wrapping_add((MPD_SSIZE_MIN + MPD_SSIZE_MAX) as mpd_uint_t)
            == MPD_SSIZE_MAX as mpd_uint_t
    {
        return MPD_SSIZE_MIN as mpd_ssize_t;
    }
    *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
    return MPD_SSIZE_MAX as mpd_ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qget_u64(mut a: *const mpd_t, mut status: *mut uint32_t) -> uint64_t {
    return mpd_qget_uint(a, status) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qget_i64(mut a: *const mpd_t, mut status: *mut uint32_t) -> int64_t {
    return mpd_qget_ssize(a, status) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qget_u32(mut a: *const mpd_t, mut status: *mut uint32_t) -> uint32_t {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut x: uint64_t = mpd_qget_uint(a, &raw mut workstatus) as uint64_t;
    if workstatus & MPD_Invalid_operation as uint32_t != 0 {
        *status |= workstatus;
        return UINT32_MAX as uint32_t;
    }
    if x > UINT32_MAX as uint64_t {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return UINT32_MAX as uint32_t;
    }
    return x as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qget_i32(mut a: *const mpd_t, mut status: *mut uint32_t) -> int32_t {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut x: int64_t = mpd_qget_ssize(a, &raw mut workstatus) as int64_t;
    if workstatus & MPD_Invalid_operation as uint32_t != 0 {
        *status |= workstatus;
        return INT32_MAX as int32_t;
    }
    if x < INT32_MIN as int64_t || x > INT32_MAX as int64_t {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return INT32_MAX as int32_t;
    }
    return x as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcheck_nan(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if mpd_isnan(a) != 0 {
        *status = (*status as ::core::ffi::c_uint
            | if mpd_issnan(a) != 0 {
                MPD_Invalid_operation
            } else {
                0 as ::core::ffi::c_uint
            }) as uint32_t;
        mpd_qcopy(result, a, status);
        mpd_set_qnan(result);
        _mpd_fix_nan(result, ctx);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcheck_nans(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if ((*a).flags as ::core::ffi::c_int | (*b).flags as ::core::ffi::c_int)
        & (MPD_NAN as ::core::ffi::c_int | MPD_SNAN as ::core::ffi::c_int)
        != 0
    {
        let mut choice: *const mpd_t = b;
        if mpd_issnan(a) != 0 {
            choice = a;
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        } else if mpd_issnan(b) != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        } else if mpd_isqnan(a) != 0 {
            choice = a;
        }
        mpd_qcopy(result, choice, status);
        mpd_set_qnan(result);
        _mpd_fix_nan(result, ctx);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mpd_qcheck_3nans(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut c: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if ((*a).flags as ::core::ffi::c_int
        | (*b).flags as ::core::ffi::c_int
        | (*c).flags as ::core::ffi::c_int)
        & (MPD_NAN as ::core::ffi::c_int | MPD_SNAN as ::core::ffi::c_int)
        != 0
    {
        let mut choice: *const mpd_t = c;
        if mpd_issnan(a) != 0 {
            choice = a;
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        } else if mpd_issnan(b) != 0 {
            choice = b;
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        } else if mpd_issnan(c) != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        } else if mpd_isqnan(a) != 0 {
            choice = a;
        } else if mpd_isqnan(b) != 0 {
            choice = b;
        }
        mpd_qcopy(result, choice, status);
        mpd_set_qnan(result);
        _mpd_fix_nan(result, ctx);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_rnd_incr(
    mut dec: *const mpd_t,
    mut rnd: mpd_uint_t,
    mut ctx: *const mpd_context_t,
) -> ::core::ffi::c_int {
    let mut ld: ::core::ffi::c_int = 0;
    match (*ctx).round {
        1 | 8 => return 0 as ::core::ffi::c_int,
        4 => return (rnd >= 5 as mpd_uint_t) as ::core::ffi::c_int,
        6 => {
            return (rnd > 5 as mpd_uint_t || rnd == 5 as mpd_uint_t && mpd_isoddcoeff(dec) != 0)
                as ::core::ffi::c_int;
        }
        2 => {
            return !(rnd == 0 as mpd_uint_t || mpd_isnegative(dec) != 0) as ::core::ffi::c_int;
        }
        3 => {
            return !(rnd == 0 as mpd_uint_t || mpd_ispositive(dec) != 0) as ::core::ffi::c_int;
        }
        5 => return (rnd > 5 as mpd_uint_t) as ::core::ffi::c_int,
        0 => return !(rnd == 0 as mpd_uint_t) as ::core::ffi::c_int,
        7 => {
            ld = mpd_lsd(*(*dec).data.offset(0 as isize)) as ::core::ffi::c_int;
            return (!(rnd == 0 as mpd_uint_t)
                && (ld == 0 as ::core::ffi::c_int || ld == 5 as ::core::ffi::c_int))
                as ::core::ffi::c_int;
        }
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn _mpd_apply_round(
    mut dec: *mut mpd_t,
    mut rnd: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if _mpd_rnd_incr(dec, rnd, ctx) != 0 {
        let mut carry: mpd_uint_t = _mpd_baseincr((*dec).data, (*dec).len as mpd_size_t);
        if carry != 0 {
            *(*dec).data.offset(((*dec).len - 1 as mpd_ssize_t) as isize) = *(&raw const mpd_pow10
                as *const mpd_uint_t)
                .offset((MPD_RDIGITS - 1 as ::core::ffi::c_int) as isize);
            (*dec).exp += 1 as mpd_ssize_t;
            _mpd_check_exp(dec, ctx, status);
            return;
        }
        mpd_setdigits(dec);
        if (*dec).digits > (*ctx).prec {
            mpd_qshiftr_inplace(dec, 1 as mpd_ssize_t);
            (*dec).exp += 1 as mpd_ssize_t;
            (*dec).digits = (*ctx).prec;
            _mpd_check_exp(dec, ctx, status);
        }
    }
}
unsafe extern "C" fn _mpd_apply_round_excess(
    mut dec: *mut mpd_t,
    mut rnd: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if _mpd_rnd_incr(dec, rnd, ctx) != 0 {
        let mut carry: mpd_uint_t = _mpd_baseincr((*dec).data, (*dec).len as mpd_size_t);
        if carry != 0 {
            if mpd_qresize(dec, (*dec).len + 1 as mpd_ssize_t, status) == 0 {
                return;
            }
            *(*dec).data.offset((*dec).len as isize) = 1 as mpd_uint_t;
            (*dec).len += 1 as mpd_ssize_t;
        }
        mpd_setdigits(dec);
    }
}
unsafe extern "C" fn _mpd_apply_round_fit(
    mut dec: *mut mpd_t,
    mut rnd: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if _mpd_rnd_incr(dec, rnd, ctx) != 0 {
        let mut carry: mpd_uint_t = _mpd_baseincr((*dec).data, (*dec).len as mpd_size_t);
        if carry != 0 {
            if mpd_qresize(dec, (*dec).len + 1 as mpd_ssize_t, status) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            *(*dec).data.offset((*dec).len as isize) = 1 as mpd_uint_t;
            (*dec).len += 1 as mpd_ssize_t;
        }
        mpd_setdigits(dec);
        if (*dec).digits > (*ctx).prec {
            mpd_seterror(dec, MPD_Invalid_operation as uint32_t, status);
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_check_exp(
    mut dec: *mut mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut adjexp: mpd_ssize_t = 0;
    let mut etiny: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut rnd: ::core::ffi::c_int = 0;
    adjexp = mpd_adjexp(dec);
    if adjexp > (*ctx).emax {
        if mpd_iszerocoeff(dec) != 0 {
            (*dec).exp = (*ctx).emax;
            if (*ctx).clamp != 0 {
                (*dec).exp -= (*ctx).prec - 1 as mpd_ssize_t;
            }
            mpd_zerocoeff(dec);
            *status = (*status as ::core::ffi::c_uint | MPD_Clamped) as uint32_t;
            return;
        }
        match (*ctx).round {
            4 | 6 | 5 | 0 | 8 => {
                mpd_setspecial(dec, mpd_sign(dec), MPD_INF);
            }
            1 | 7 => {
                mpd_qmaxcoeff(dec, ctx, status);
                (*dec).exp = (*ctx).emax - (*ctx).prec + 1 as mpd_ssize_t;
            }
            2 => {
                if mpd_isnegative(dec) != 0 {
                    mpd_qmaxcoeff(dec, ctx, status);
                    (*dec).exp = (*ctx).emax - (*ctx).prec + 1 as mpd_ssize_t;
                } else {
                    mpd_setspecial(dec, MPD_POS, MPD_INF);
                }
            }
            3 => {
                if mpd_ispositive(dec) != 0 {
                    mpd_qmaxcoeff(dec, ctx, status);
                    (*dec).exp = (*ctx).emax - (*ctx).prec + 1 as mpd_ssize_t;
                } else {
                    mpd_setspecial(dec, MPD_NEG, MPD_INF);
                }
            }
            _ => {
                abort();
            }
        }
        *status = (*status as ::core::ffi::c_uint | (MPD_Overflow | MPD_Inexact | MPD_Rounded))
            as uint32_t;
    } else if (*ctx).clamp != 0 && (*dec).exp > mpd_etop(ctx) {
        shift = (*dec).exp - mpd_etop(ctx);
        if mpd_qshiftl(dec, dec, shift, status) == 0 {
            return;
        }
        (*dec).exp -= shift;
        *status = (*status as ::core::ffi::c_uint | MPD_Clamped) as uint32_t;
        if mpd_iszerocoeff(dec) == 0 && adjexp < (*ctx).emin {
            *status = (*status as ::core::ffi::c_uint | MPD_Subnormal) as uint32_t;
        }
    } else if adjexp < (*ctx).emin {
        etiny = mpd_etiny(ctx);
        if mpd_iszerocoeff(dec) != 0 {
            if (*dec).exp < etiny {
                (*dec).exp = etiny;
                mpd_zerocoeff(dec);
                *status = (*status as ::core::ffi::c_uint | MPD_Clamped) as uint32_t;
            }
            return;
        }
        *status = (*status as ::core::ffi::c_uint | MPD_Subnormal) as uint32_t;
        if (*dec).exp < etiny {
            shift = etiny - (*dec).exp;
            rnd = mpd_qshiftr_inplace(dec, shift) as ::core::ffi::c_int;
            (*dec).exp = etiny;
            _mpd_apply_round_excess(dec, rnd as mpd_uint_t, ctx, status);
            *status = (*status as ::core::ffi::c_uint | MPD_Rounded) as uint32_t;
            if rnd != 0 {
                *status =
                    (*status as ::core::ffi::c_uint | (MPD_Inexact | MPD_Underflow)) as uint32_t;
                if mpd_iszerocoeff(dec) != 0 {
                    mpd_zerocoeff(dec);
                    *status = (*status as ::core::ffi::c_uint | MPD_Clamped) as uint32_t;
                }
            }
        }
    }
}
unsafe extern "C" fn mpd_check_underflow(
    mut dec: *mut mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_adjexp(dec) < (*ctx).emin && mpd_iszero(dec) == 0 && (*dec).exp < mpd_etiny(ctx) {
        *status = (*status as ::core::ffi::c_uint | MPD_Underflow) as uint32_t;
    }
}
unsafe extern "C" fn _mpd_check_round(
    mut dec: *mut mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut rnd: mpd_uint_t = 0;
    let mut shift: mpd_ssize_t = 0;
    if mpd_isspecial(dec) != 0 {
        return;
    }
    if (*dec).digits > (*ctx).prec {
        shift = (*dec).digits - (*ctx).prec;
        rnd = mpd_qshiftr_inplace(dec, shift);
        (*dec).exp += shift;
        _mpd_apply_round(dec, rnd, ctx, status);
        *status = (*status as ::core::ffi::c_uint | MPD_Rounded) as uint32_t;
        if rnd != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Inexact) as uint32_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qfinalize(
    mut result: *mut mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(result) != 0 {
        if mpd_isnan(result) != 0 {
            _mpd_fix_nan(result, ctx);
        }
        return;
    }
    _mpd_check_exp(result, ctx, status);
    _mpd_check_round(result, ctx, status);
}
unsafe extern "C" fn _mpd_copy_shared(mut dest: *mut mpd_t, mut src: *const mpd_t) {
    (*dest).flags = (*src).flags;
    (*dest).exp = (*src).exp;
    (*dest).digits = (*src).digits;
    (*dest).len = (*src).len;
    (*dest).alloc = (*src).alloc;
    (*dest).data = (*src).data;
    mpd_set_shared_data(dest);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcopy(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if result == a as *mut mpd_t {
        return 1 as ::core::ffi::c_int;
    }
    if mpd_qresize(result, (*a).len, status) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    mpd_copy_flags(result, a);
    (*result).exp = (*a).exp;
    (*result).digits = (*a).digits;
    (*result).len = (*a).len;
    memcpy(
        (*result).data as *mut ::core::ffi::c_void,
        (*a).data as *const ::core::ffi::c_void,
        ((*a).len as u64).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as u64) as size_t,
    );
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcopy_cxx(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
) -> ::core::ffi::c_int {
    if result == a as *mut mpd_t {
        return 1 as ::core::ffi::c_int;
    }
    if mpd_qresize_cxx(result, (*a).len) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    mpd_copy_flags(result, a);
    (*result).exp = (*a).exp;
    (*result).digits = (*a).digits;
    (*result).len = (*a).len;
    memcpy(
        (*result).data as *mut ::core::ffi::c_void,
        (*a).data as *const ::core::ffi::c_void,
        ((*a).len as u64).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as u64) as size_t,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn mpd_qcopy_static(mut result: *mut mpd_t, mut a: *const mpd_t) {
    if result == a as *mut mpd_t {
        return;
    }
    memcpy(
        (*result).data as *mut ::core::ffi::c_void,
        (*a).data as *const ::core::ffi::c_void,
        ((*a).len as u64).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as u64) as size_t,
    );
    mpd_copy_flags(result, a);
    (*result).exp = (*a).exp;
    (*result).digits = (*a).digits;
    (*result).len = (*a).len;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qncopy(mut a: *const mpd_t) -> *mut mpd_t {
    let mut result: *mut mpd_t = ::core::ptr::null_mut::<mpd_t>();
    result = mpd_qnew_size((*a).len);
    if result.is_null() {
        return ::core::ptr::null_mut::<mpd_t>();
    }
    memcpy(
        (*result).data as *mut ::core::ffi::c_void,
        (*a).data as *const ::core::ffi::c_void,
        ((*a).len as u64).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as u64) as size_t,
    );
    mpd_copy_flags(result, a);
    (*result).exp = (*a).exp;
    (*result).digits = (*a).digits;
    (*result).len = (*a).len;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcopy_abs(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if mpd_qcopy(result, a, status) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    mpd_set_positive(result);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcopy_negate(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if mpd_qcopy(result, a, status) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    _mpd_negate(result);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcopy_sign(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut sign_b: uint8_t = mpd_sign(b);
    if mpd_qcopy(result, a, status) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    mpd_set_sign(result, sign_b);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_basecmp(
    mut big: *mut mpd_uint_t,
    mut small: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut m: mpd_size_t,
    mut shift: mpd_size_t,
) -> ::core::ffi::c_int {
    let mut l: mpd_uint_t = 0;
    let mut lprev: mpd_uint_t = 0;
    let mut h: mpd_uint_t = 0;
    let mut q: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    let mut ph: mpd_uint_t = 0;
    let mut x: mpd_uint_t = 0;
    if m > 0 as mpd_size_t && n >= m && shift > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(
            b"m > 0 && n >= m && shift > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
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
            *small.offset(fresh0 as isize),
            (MPD_RDIGITS as mpd_uint_t).wrapping_sub(r),
        );
        if h != 0 as mpd_uint_t {
            if *big.offset(n as isize) != h {
                if *big.offset(n as isize) < h {
                    return -(1 as ::core::ffi::c_int);
                }
                return 1 as ::core::ffi::c_int;
            }
            n = n.wrapping_sub(1);
        }
        while m != MPD_SIZE_MAX as mpd_size_t {
            _mpd_divmod_pow10(
                &raw mut h,
                &raw mut l,
                *small.offset(m as isize),
                (MPD_RDIGITS as mpd_uint_t).wrapping_sub(r),
            );
            x = ph.wrapping_mul(lprev).wrapping_add(h);
            if *big.offset(n as isize) != x {
                if *big.offset(n as isize) < x {
                    return -(1 as ::core::ffi::c_int);
                }
                return 1 as ::core::ffi::c_int;
            }
            lprev = l;
            m = m.wrapping_sub(1);
            n = n.wrapping_sub(1);
        }
        x = ph.wrapping_mul(lprev);
        if *big.offset(q as isize) != x {
            if *big.offset(q as isize) < x {
                return -(1 as ::core::ffi::c_int);
            }
            return 1 as ::core::ffi::c_int;
        }
    } else {
        loop {
            m = m.wrapping_sub(1);
            if !(m != MPD_SIZE_MAX as mpd_size_t) {
                break;
            }
            if *big.offset((m as mpd_uint_t).wrapping_add(q) as isize) != *small.offset(m as isize)
            {
                if *big.offset((m as mpd_uint_t).wrapping_add(q) as isize)
                    < *small.offset(m as isize)
                {
                    return -(1 as ::core::ffi::c_int);
                }
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    return (_mpd_isallzero(big, q as mpd_ssize_t) == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_cmp_same_adjexp(
    mut a: *const mpd_t,
    mut b: *const mpd_t,
) -> ::core::ffi::c_int {
    let mut shift: mpd_ssize_t = 0;
    let mut i: mpd_ssize_t = 0;
    if (*a).exp != (*b).exp {
        shift = (*a).exp - (*b).exp;
        if shift > 0 as mpd_ssize_t {
            return -(1 as ::core::ffi::c_int)
                * _mpd_basecmp(
                    (*b).data,
                    (*a).data,
                    (*b).len as mpd_size_t,
                    (*a).len as mpd_size_t,
                    shift as mpd_size_t,
                );
        } else {
            return _mpd_basecmp(
                (*a).data,
                (*b).data,
                (*a).len as mpd_size_t,
                (*b).len as mpd_size_t,
                -shift as mpd_size_t,
            );
        }
    }
    i = (*a).len - 1 as mpd_ssize_t;
    while i >= 0 as mpd_ssize_t {
        if *(*a).data.offset(i as isize) != *(*b).data.offset(i as isize) {
            if *(*a).data.offset(i as isize) < *(*b).data.offset(i as isize) {
                return -(1 as ::core::ffi::c_int);
            }
            return 1 as ::core::ffi::c_int;
        }
        i -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_cmp(mut a: *const mpd_t, mut b: *const mpd_t) -> ::core::ffi::c_int {
    let mut adjexp_a: mpd_ssize_t = 0;
    let mut adjexp_b: mpd_ssize_t = 0;
    if a == b {
        return 0 as ::core::ffi::c_int;
    }
    if mpd_isinfinite(a) != 0 {
        if mpd_isinfinite(b) != 0 {
            return mpd_isnegative(b) - mpd_isnegative(a);
        }
        return mpd_arith_sign(a);
    }
    if mpd_isinfinite(b) != 0 {
        return -mpd_arith_sign(b);
    }
    if mpd_iszerocoeff(a) != 0 {
        if mpd_iszerocoeff(b) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return -mpd_arith_sign(b);
    }
    if mpd_iszerocoeff(b) != 0 {
        return mpd_arith_sign(a);
    }
    if mpd_sign(a) as ::core::ffi::c_int != mpd_sign(b) as ::core::ffi::c_int {
        return mpd_sign(b) as ::core::ffi::c_int - mpd_sign(a) as ::core::ffi::c_int;
    }
    adjexp_a = mpd_adjexp(a);
    adjexp_b = mpd_adjexp(b);
    if adjexp_a != adjexp_b {
        if adjexp_a < adjexp_b {
            return -(1 as ::core::ffi::c_int) * mpd_arith_sign(a);
        }
        return mpd_arith_sign(a);
    }
    return _mpd_cmp_same_adjexp(a, b) * mpd_arith_sign(a);
}
unsafe extern "C" fn _mpd_cmp_abs(mut a: *const mpd_t, mut b: *const mpd_t) -> ::core::ffi::c_int {
    let mut adjexp_a: mpd_ssize_t = 0;
    let mut adjexp_b: mpd_ssize_t = 0;
    if a == b {
        return 0 as ::core::ffi::c_int;
    }
    if mpd_isinfinite(a) != 0 {
        if mpd_isinfinite(b) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
    if mpd_isinfinite(b) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if mpd_iszerocoeff(a) != 0 {
        if mpd_iszerocoeff(b) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return -(1 as ::core::ffi::c_int);
    }
    if mpd_iszerocoeff(b) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    adjexp_a = mpd_adjexp(a);
    adjexp_b = mpd_adjexp(b);
    if adjexp_a != adjexp_b {
        if adjexp_a < adjexp_b {
            return -(1 as ::core::ffi::c_int);
        }
        return 1 as ::core::ffi::c_int;
    }
    return _mpd_cmp_same_adjexp(a, b);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcmp(
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_isnan(a) != 0 || mpd_isnan(b) != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return INT_MAX;
        }
    }
    return _mpd_cmp(a, b);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcompare(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return INT_MAX;
        }
    }
    c = _mpd_cmp(a, b);
    _settriple(
        result,
        (c < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t,
        (c != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as mpd_uint_t,
        0 as mpd_ssize_t,
    );
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qcompare_signal(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return INT_MAX;
        }
    }
    c = _mpd_cmp(a, b);
    _settriple(
        result,
        (c < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t,
        (c != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as mpd_uint_t,
        0 as mpd_ssize_t,
    );
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_cmp_total(
    mut a: *const mpd_t,
    mut b: *const mpd_t,
) -> ::core::ffi::c_int {
    let mut aa: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    let mut bb: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    let mut nan_a: ::core::ffi::c_int = 0;
    let mut nan_b: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if mpd_sign(a) as ::core::ffi::c_int != mpd_sign(b) as ::core::ffi::c_int {
        return mpd_sign(b) as ::core::ffi::c_int - mpd_sign(a) as ::core::ffi::c_int;
    }
    if mpd_isnan(a) != 0 {
        c = 1 as ::core::ffi::c_int;
        if mpd_isnan(b) != 0 {
            nan_a = if mpd_isqnan(a) != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            nan_b = if mpd_isqnan(b) != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            if nan_b == nan_a {
                if (*a).len > 0 as mpd_ssize_t && (*b).len > 0 as mpd_ssize_t {
                    _mpd_copy_shared(&raw mut aa, a);
                    _mpd_copy_shared(&raw mut bb, b);
                    bb.exp = 0 as mpd_ssize_t;
                    aa.exp = bb.exp;
                    c = _mpd_cmp_abs(&raw mut aa, &raw mut bb);
                } else {
                    c = ((*a).len > 0 as mpd_ssize_t) as ::core::ffi::c_int
                        - ((*b).len > 0 as mpd_ssize_t) as ::core::ffi::c_int;
                }
            } else {
                c = nan_a - nan_b;
            }
        }
    } else if mpd_isnan(b) != 0 {
        c = -(1 as ::core::ffi::c_int);
    } else {
        c = _mpd_cmp_abs(a, b);
        if c == 0 as ::core::ffi::c_int && (*a).exp != (*b).exp {
            c = if (*a).exp < (*b).exp {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
    }
    return c * mpd_arith_sign(a);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_compare_total(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    c = mpd_cmp_total(a, b);
    _settriple(
        result,
        (c < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t,
        (c != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as mpd_uint_t,
        0 as mpd_ssize_t,
    );
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_cmp_total_mag(
    mut a: *const mpd_t,
    mut b: *const mpd_t,
) -> ::core::ffi::c_int {
    let mut aa: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    let mut bb: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    _mpd_copy_shared(&raw mut aa, a);
    _mpd_copy_shared(&raw mut bb, b);
    mpd_set_positive(&raw mut aa);
    mpd_set_positive(&raw mut bb);
    return mpd_cmp_total(&raw mut aa, &raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_compare_total_mag(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    c = mpd_cmp_total_mag(a, b);
    _settriple(
        result,
        (c < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t,
        (c != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as mpd_uint_t,
        0 as mpd_ssize_t,
    );
    return c;
}
unsafe extern "C" fn _mpd_cmp_numequal(
    mut a: *const mpd_t,
    mut b: *const mpd_t,
) -> ::core::ffi::c_int {
    let mut sign_a: ::core::ffi::c_int = 0;
    let mut sign_b: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    sign_a = mpd_sign(a) as ::core::ffi::c_int;
    sign_b = mpd_sign(b) as ::core::ffi::c_int;
    if sign_a != sign_b {
        c = sign_b - sign_a;
    } else {
        c = if (*a).exp < (*b).exp {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
        c *= mpd_arith_sign(a);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qshiftl(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut n: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut size: mpd_ssize_t = 0;
    if mpd_isspecial(a) == 0 {
    } else {
        mpd_c2rust_assert_fail(b"!mpd_isspecial(a)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszerocoeff(a) != 0 || n == 0 as mpd_ssize_t {
        return mpd_qcopy(result, a, status);
    }
    size = mpd_digits_to_size((*a).digits + n);
    if mpd_qresize(result, size, status) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    _mpd_baseshiftl(
        (*result).data,
        (*a).data,
        size as mpd_size_t,
        (*a).len as mpd_size_t,
        n as mpd_size_t,
    );
    mpd_copy_flags(result, a);
    (*result).exp = (*a).exp;
    (*result).digits = (*a).digits + n;
    (*result).len = size;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_get_rnd(
    mut data: *const mpd_uint_t,
    mut len: mpd_ssize_t,
    mut use_msd: ::core::ffi::c_int,
) -> mpd_uint_t {
    let mut rnd: mpd_uint_t = 0 as mpd_uint_t;
    let mut rest: mpd_uint_t = 0 as mpd_uint_t;
    let mut word: mpd_uint_t = 0;
    word = *data.offset((len - 1 as mpd_ssize_t) as isize);
    if use_msd != 0 {
        _mpd_divmod_pow10(
            &raw mut rnd,
            &raw mut rest,
            word,
            (mpd_word_digits(word) - 1 as ::core::ffi::c_int) as mpd_uint_t,
        );
        if len > 1 as mpd_ssize_t && rest == 0 as mpd_uint_t {
            rest = (_mpd_isallzero(data, len - 1 as mpd_ssize_t) == 0) as ::core::ffi::c_int
                as mpd_uint_t;
        }
    } else {
        rest = (_mpd_isallzero(data, len) == 0) as ::core::ffi::c_int as mpd_uint_t;
    }
    return if rnd == 0 as mpd_uint_t || rnd == 5 as mpd_uint_t {
        rnd.wrapping_add((rest != 0) as ::core::ffi::c_int as mpd_uint_t)
    } else {
        rnd
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsshiftr(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut n: mpd_ssize_t,
) -> mpd_uint_t {
    let mut rnd: mpd_uint_t = 0;
    let mut size: mpd_ssize_t = 0;
    if mpd_isspecial(a) == 0 {
    } else {
        mpd_c2rust_assert_fail(b"!mpd_isspecial(a)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszerocoeff(a) != 0 || n == 0 as mpd_ssize_t {
        mpd_qcopy_static(result, a);
        return 0 as mpd_uint_t;
    }
    if n >= (*a).digits {
        rnd = _mpd_get_rnd(
            (*a).data,
            (*a).len,
            (n == (*a).digits) as ::core::ffi::c_int,
        );
        mpd_zerocoeff(result);
    } else {
        (*result).digits = (*a).digits - n;
        size = mpd_digits_to_size((*result).digits);
        rnd = _mpd_baseshiftr(
            (*result).data,
            (*a).data,
            (*a).len as mpd_size_t,
            n as mpd_size_t,
        );
        (*result).len = size;
    }
    mpd_copy_flags(result, a);
    (*result).exp = (*a).exp;
    return rnd;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qshiftr_inplace(
    mut result: *mut mpd_t,
    mut n: mpd_ssize_t,
) -> mpd_uint_t {
    let mut dummy: uint32_t = 0;
    let mut rnd: mpd_uint_t = 0;
    let mut size: mpd_ssize_t = 0;
    if mpd_isspecial(result) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isspecial(result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if n >= 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszerocoeff(result) != 0 || n == 0 as mpd_ssize_t {
        return 0 as mpd_uint_t;
    }
    if n >= (*result).digits {
        rnd = _mpd_get_rnd(
            (*result).data,
            (*result).len,
            (n == (*result).digits) as ::core::ffi::c_int,
        );
        mpd_zerocoeff(result);
    } else {
        rnd = _mpd_baseshiftr(
            (*result).data,
            (*result).data,
            (*result).len as mpd_size_t,
            n as mpd_size_t,
        );
        (*result).digits -= n;
        size = mpd_digits_to_size((*result).digits);
        mpd_qresize(result, size, &raw mut dummy);
        (*result).len = size;
    }
    return rnd;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qshiftr(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut n: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> mpd_uint_t {
    let mut rnd: mpd_uint_t = 0;
    let mut size: mpd_ssize_t = 0;
    if mpd_isspecial(a) == 0 {
    } else {
        mpd_c2rust_assert_fail(b"!mpd_isspecial(a)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszerocoeff(a) != 0 || n == 0 as mpd_ssize_t {
        if mpd_qcopy(result, a, status) == 0 {
            return MPD_UINT_MAX as mpd_uint_t;
        }
        return 0 as mpd_uint_t;
    }
    if n >= (*a).digits {
        rnd = _mpd_get_rnd(
            (*a).data,
            (*a).len,
            (n == (*a).digits) as ::core::ffi::c_int,
        );
        mpd_zerocoeff(result);
    } else {
        (*result).digits = (*a).digits - n;
        size = mpd_digits_to_size((*result).digits);
        if result == a as *mut mpd_t {
            rnd = _mpd_baseshiftr(
                (*result).data,
                (*a).data,
                (*a).len as mpd_size_t,
                n as mpd_size_t,
            );
            mpd_qresize(result, size, status);
        } else {
            if mpd_qresize(result, size, status) == 0 {
                return MPD_UINT_MAX as mpd_uint_t;
            }
            rnd = _mpd_baseshiftr(
                (*result).data,
                (*a).data,
                (*a).len as mpd_size_t,
                n as mpd_size_t,
            );
        }
        (*result).len = size;
    }
    mpd_copy_flags(result, a);
    (*result).exp = (*a).exp;
    return rnd;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qand(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut big: *const mpd_t = a;
    let mut small: *const mpd_t = b;
    let mut x: mpd_uint_t = 0;
    let mut y: mpd_uint_t = 0;
    let mut z: mpd_uint_t = 0;
    let mut xbit: mpd_uint_t = 0;
    let mut ybit: mpd_uint_t = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut mswdigits: ::core::ffi::c_int = 0;
    let mut i: mpd_ssize_t = 0;
    if mpd_isspecial(a) != 0
        || mpd_isspecial(b) != 0
        || mpd_isnegative(a) != 0
        || mpd_isnegative(b) != 0
        || (*a).exp != 0 as mpd_ssize_t
        || (*b).exp != 0 as mpd_ssize_t
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if (*b).digits > (*a).digits {
        big = b;
        small = a;
    }
    if mpd_qresize(result, (*big).len, status) == 0 {
        return;
    }
    i = 0 as mpd_ssize_t;
    's_48: loop {
        if !(i < (*small).len - 1 as mpd_ssize_t) {
            current_block = 1;
            break;
        }
        x = *(*small).data.offset(i as isize);
        y = *(*big).data.offset(i as isize);
        z = 0 as mpd_uint_t;
        k = 0 as ::core::ffi::c_int;
        while k < MPD_RDIGITS {
            xbit = x.wrapping_rem(10 as mpd_uint_t);
            x = x.wrapping_div(10 as mpd_uint_t);
            ybit = y.wrapping_rem(10 as mpd_uint_t);
            y = y.wrapping_div(10 as mpd_uint_t);
            if xbit > 1 as mpd_uint_t || ybit > 1 as mpd_uint_t {
                current_block = 2;
                break 's_48;
            }
            z = z.wrapping_add(if xbit & ybit != 0 {
                *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize)
            } else {
                0 as mpd_uint_t
            });
            k += 1;
        }
        *(*result).data.offset(i as isize) = z;
        i += 1;
    }
    match current_block {
        1 => {
            x = *(*small).data.offset(i as isize);
            y = *(*big).data.offset(i as isize);
            z = 0 as mpd_uint_t;
            mswdigits = mpd_word_digits(x);
            k = 0 as ::core::ffi::c_int;
            loop {
                if !(k < mswdigits) {
                    current_block = 3;
                    break;
                }
                xbit = x.wrapping_rem(10 as mpd_uint_t);
                x = x.wrapping_div(10 as mpd_uint_t);
                ybit = y.wrapping_rem(10 as mpd_uint_t);
                y = y.wrapping_div(10 as mpd_uint_t);
                if xbit > 1 as mpd_uint_t || ybit > 1 as mpd_uint_t {
                    current_block = 2;
                    break;
                }
                z = z.wrapping_add(if xbit & ybit != 0 {
                    *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize)
                } else {
                    0 as mpd_uint_t
                });
                k += 1;
            }
            match current_block {
                2 => {}
                _ => {
                    let fresh0 = i;
                    i = i + 1;
                    *(*result).data.offset(fresh0 as isize) = z;
                    loop {
                        if !(k < MPD_RDIGITS) {
                            current_block = 4;
                            break;
                        }
                        ybit = y.wrapping_rem(10 as mpd_uint_t);
                        y = y.wrapping_div(10 as mpd_uint_t);
                        if ybit > 1 as mpd_uint_t {
                            current_block = 2;
                            break;
                        }
                        k += 1;
                    }
                    match current_block {
                        2 => {}
                        _ => {
                            's_181: loop {
                                if !(i < (*big).len) {
                                    current_block = 5;
                                    break;
                                }
                                y = *(*big).data.offset(i as isize);
                                k = 0 as ::core::ffi::c_int;
                                while k < MPD_RDIGITS {
                                    ybit = y.wrapping_rem(10 as mpd_uint_t);
                                    y = y.wrapping_div(10 as mpd_uint_t);
                                    if ybit > 1 as mpd_uint_t {
                                        current_block = 2;
                                        break 's_181;
                                    }
                                    k += 1;
                                }
                                i += 1;
                            }
                            match current_block {
                                2 => {}
                                _ => {
                                    mpd_clear_flags(result);
                                    (*result).exp = 0 as mpd_ssize_t;
                                    (*result).len = _mpd_real_size((*result).data, (*small).len);
                                    mpd_qresize(result, (*result).len, status);
                                    mpd_setdigits(result);
                                    _mpd_cap(result, ctx);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_class(
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
) -> *const ::core::ffi::c_char {
    if mpd_isnan(a) != 0 {
        if mpd_isqnan(a) != 0 {
            return b"NaN\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            return b"sNaN\0" as *const u8 as *const ::core::ffi::c_char;
        }
    } else if mpd_ispositive(a) != 0 {
        if mpd_isinfinite(a) != 0 {
            return b"+Infinity\0" as *const u8 as *const ::core::ffi::c_char;
        } else if mpd_iszero(a) != 0 {
            return b"+Zero\0" as *const u8 as *const ::core::ffi::c_char;
        } else if mpd_isnormal(a, ctx) != 0 {
            return b"+Normal\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            return b"+Subnormal\0" as *const u8 as *const ::core::ffi::c_char;
        }
    } else if mpd_isinfinite(a) != 0 {
        return b"-Infinity\0" as *const u8 as *const ::core::ffi::c_char;
    } else if mpd_iszero(a) != 0 {
        return b"-Zero\0" as *const u8 as *const ::core::ffi::c_char;
    } else if mpd_isnormal(a, ctx) != 0 {
        return b"-Normal\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        return b"-Subnormal\0" as *const u8 as *const ::core::ffi::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qinvert(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut x: mpd_uint_t = 0;
    let mut z: mpd_uint_t = 0;
    let mut xbit: mpd_uint_t = 0;
    let mut i: mpd_ssize_t = 0;
    let mut digits: mpd_ssize_t = 0;
    let mut len: mpd_ssize_t = 0;
    let mut q: mpd_ssize_t = 0;
    let mut r: mpd_ssize_t = 0;
    let mut k: ::core::ffi::c_int = 0;
    if mpd_isspecial(a) != 0 || mpd_isnegative(a) != 0 || (*a).exp != 0 as mpd_ssize_t {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    digits = if (*a).digits < (*ctx).prec {
        (*ctx).prec
    } else {
        (*a).digits
    };
    _mpd_idiv_word(&raw mut q, &raw mut r, digits, MPD_RDIGITS as mpd_ssize_t);
    len = if r == 0 as mpd_ssize_t {
        q
    } else {
        q + 1 as mpd_ssize_t
    };
    if mpd_qresize(result, len, status) == 0 {
        return;
    }
    i = 0 as mpd_ssize_t;
    's_42: loop {
        if !(i < len) {
            current_block = 1;
            break;
        }
        x = if i < (*a).len {
            *(*a).data.offset(i as isize)
        } else {
            0 as mpd_uint_t
        };
        z = 0 as mpd_uint_t;
        k = 0 as ::core::ffi::c_int;
        while k < MPD_RDIGITS {
            xbit = x.wrapping_rem(10 as mpd_uint_t);
            x = x.wrapping_div(10 as mpd_uint_t);
            if xbit > 1 as mpd_uint_t {
                current_block = 2;
                break 's_42;
            }
            z = z.wrapping_add(if xbit == 0 {
                *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize)
            } else {
                0 as mpd_uint_t
            });
            k += 1;
        }
        *(*result).data.offset(i as isize) = z;
        i += 1;
    }
    match current_block {
        2 => {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        _ => {
            mpd_clear_flags(result);
            (*result).exp = 0 as mpd_ssize_t;
            (*result).len = _mpd_real_size((*result).data, len);
            mpd_qresize(result, (*result).len, status);
            mpd_setdigits(result);
            _mpd_cap(result, ctx);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qlogb(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        mpd_setspecial(result, MPD_POS, MPD_INF);
    } else if mpd_iszerocoeff(a) != 0 {
        mpd_setspecial(result, MPD_NEG, MPD_INF);
        *status = (*status as ::core::ffi::c_uint | MPD_Division_by_zero) as uint32_t;
    } else {
        mpd_qset_ssize(result, mpd_adjexp(a), ctx, status);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qor(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut big: *const mpd_t = a;
    let mut small: *const mpd_t = b;
    let mut x: mpd_uint_t = 0;
    let mut y: mpd_uint_t = 0;
    let mut z: mpd_uint_t = 0;
    let mut xbit: mpd_uint_t = 0;
    let mut ybit: mpd_uint_t = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut mswdigits: ::core::ffi::c_int = 0;
    let mut i: mpd_ssize_t = 0;
    if mpd_isspecial(a) != 0
        || mpd_isspecial(b) != 0
        || mpd_isnegative(a) != 0
        || mpd_isnegative(b) != 0
        || (*a).exp != 0 as mpd_ssize_t
        || (*b).exp != 0 as mpd_ssize_t
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if (*b).digits > (*a).digits {
        big = b;
        small = a;
    }
    if mpd_qresize(result, (*big).len, status) == 0 {
        return;
    }
    i = 0 as mpd_ssize_t;
    's_48: loop {
        if !(i < (*small).len - 1 as mpd_ssize_t) {
            current_block = 1;
            break;
        }
        x = *(*small).data.offset(i as isize);
        y = *(*big).data.offset(i as isize);
        z = 0 as mpd_uint_t;
        k = 0 as ::core::ffi::c_int;
        while k < MPD_RDIGITS {
            xbit = x.wrapping_rem(10 as mpd_uint_t);
            x = x.wrapping_div(10 as mpd_uint_t);
            ybit = y.wrapping_rem(10 as mpd_uint_t);
            y = y.wrapping_div(10 as mpd_uint_t);
            if xbit > 1 as mpd_uint_t || ybit > 1 as mpd_uint_t {
                current_block = 2;
                break 's_48;
            }
            z = z.wrapping_add(if xbit | ybit != 0 {
                *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize)
            } else {
                0 as mpd_uint_t
            });
            k += 1;
        }
        *(*result).data.offset(i as isize) = z;
        i += 1;
    }
    match current_block {
        1 => {
            x = *(*small).data.offset(i as isize);
            y = *(*big).data.offset(i as isize);
            z = 0 as mpd_uint_t;
            mswdigits = mpd_word_digits(x);
            k = 0 as ::core::ffi::c_int;
            loop {
                if !(k < mswdigits) {
                    current_block = 3;
                    break;
                }
                xbit = x.wrapping_rem(10 as mpd_uint_t);
                x = x.wrapping_div(10 as mpd_uint_t);
                ybit = y.wrapping_rem(10 as mpd_uint_t);
                y = y.wrapping_div(10 as mpd_uint_t);
                if xbit > 1 as mpd_uint_t || ybit > 1 as mpd_uint_t {
                    current_block = 2;
                    break;
                }
                z = z.wrapping_add(if xbit | ybit != 0 {
                    *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize)
                } else {
                    0 as mpd_uint_t
                });
                k += 1;
            }
            match current_block {
                2 => {}
                _ => {
                    loop {
                        if !(k < MPD_RDIGITS) {
                            current_block = 4;
                            break;
                        }
                        ybit = y.wrapping_rem(10 as mpd_uint_t);
                        y = y.wrapping_div(10 as mpd_uint_t);
                        if ybit > 1 as mpd_uint_t {
                            current_block = 2;
                            break;
                        }
                        z = z.wrapping_add(ybit.wrapping_mul(
                            *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize),
                        ));
                        k += 1;
                    }
                    match current_block {
                        2 => {}
                        _ => {
                            let fresh0 = i;
                            i = i + 1;
                            *(*result).data.offset(fresh0 as isize) = z;
                            's_184: loop {
                                if !(i < (*big).len) {
                                    current_block = 5;
                                    break;
                                }
                                y = *(*big).data.offset(i as isize);
                                k = 0 as ::core::ffi::c_int;
                                while k < MPD_RDIGITS {
                                    ybit = y.wrapping_rem(10 as mpd_uint_t);
                                    y = y.wrapping_div(10 as mpd_uint_t);
                                    if ybit > 1 as mpd_uint_t {
                                        current_block = 2;
                                        break 's_184;
                                    }
                                    k += 1;
                                }
                                *(*result).data.offset(i as isize) =
                                    *(*big).data.offset(i as isize);
                                i += 1;
                            }
                            match current_block {
                                2 => {}
                                _ => {
                                    mpd_clear_flags(result);
                                    (*result).exp = 0 as mpd_ssize_t;
                                    (*result).len = _mpd_real_size((*result).data, (*big).len);
                                    mpd_qresize(result, (*result).len, status);
                                    mpd_setdigits(result);
                                    _mpd_cap(result, ctx);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qrotate(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut tmp_data: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tmp_data as *mut mpd_uint_t,
    };
    let mut big_data: [mpd_uint_t; 64] = [0; 64];
    let mut big: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut big_data as *mut mpd_uint_t,
    };
    let mut small_data: [mpd_uint_t; 64] = [0; 64];
    let mut small: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut small_data as *mut mpd_uint_t,
    };
    let mut n: mpd_ssize_t = 0;
    let mut lshift: mpd_ssize_t = 0;
    let mut rshift: mpd_ssize_t = 0;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return;
        }
    }
    if (*b).exp != 0 as mpd_ssize_t || mpd_isinfinite(b) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    n = mpd_qget_ssize(b, &raw mut workstatus);
    if workstatus & MPD_Invalid_operation as uint32_t != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if n > (*ctx).prec || n < -(*ctx).prec {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_isinfinite(a) != 0 {
        mpd_qcopy(result, a, status);
        return;
    }
    if n >= 0 as mpd_ssize_t {
        lshift = n;
        rshift = (*ctx).prec - n;
    } else {
        lshift = (*ctx).prec + n;
        rshift = -n;
    }
    if (*a).digits > (*ctx).prec {
        if mpd_qcopy(&raw mut tmp, a, status) == 0 {
            mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
            current_block = 1;
        } else {
            _mpd_cap(&raw mut tmp, ctx);
            a = &raw mut tmp;
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            if mpd_qshiftl(&raw mut big, a, lshift, status) == 0 {
                mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
            } else {
                _mpd_cap(&raw mut big, ctx);
                if mpd_qshiftr(&raw mut small, a, rshift, status) == MPD_UINT_MAX as mpd_uint_t {
                    mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
                } else {
                    _mpd_qadd(result, &raw mut big, &raw mut small, ctx, status);
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut tmp);
    mpd_del(&raw mut big);
    mpd_del(&raw mut small);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qscaleb(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut n: mpd_uint_t = 0;
    let mut maxjump: mpd_uint_t = 0;
    let mut exp: int64_t = 0;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return;
        }
    }
    if (*b).exp != 0 as mpd_ssize_t || mpd_isinfinite(b) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    n = mpd_qabs_uint(b, &raw mut workstatus);
    maxjump = (2 as mpd_uint_t).wrapping_mul(((*ctx).emax + (*ctx).prec) as mpd_uint_t);
    if n > maxjump || workstatus & MPD_Invalid_operation as uint32_t != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_isinfinite(a) != 0 {
        mpd_qcopy(result, a, status);
        return;
    }
    exp = ((*a).exp + n as mpd_ssize_t * mpd_arith_sign(b) as mpd_ssize_t) as int64_t;
    exp = if exp > MPD_EXP_INF as int64_t {
        MPD_EXP_INF as int64_t
    } else {
        exp
    };
    exp = if exp < MPD_EXP_CLAMP as int64_t {
        MPD_EXP_CLAMP as int64_t
    } else {
        exp
    };
    mpd_qcopy(result, a, status);
    (*result).exp = exp;
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qshiftn(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut n: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        mpd_qcopy(result, a, status);
        return;
    }
    if n >= 0 as mpd_ssize_t && n <= (*ctx).prec {
        mpd_qshiftl(result, a, n, status);
        _mpd_cap(result, ctx);
    } else if n < 0 as mpd_ssize_t && n >= -(*ctx).prec {
        if mpd_qcopy(result, a, status) == 0 {
            return;
        }
        _mpd_cap(result, ctx);
        mpd_qshiftr_inplace(result, -n);
    } else {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qshift(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut n: mpd_ssize_t = 0;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return;
        }
    }
    if (*b).exp != 0 as mpd_ssize_t || mpd_isinfinite(b) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    n = mpd_qget_ssize(b, &raw mut workstatus);
    if workstatus & MPD_Invalid_operation as uint32_t != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if n > (*ctx).prec || n < -(*ctx).prec {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_isinfinite(a) != 0 {
        mpd_qcopy(result, a, status);
        return;
    }
    if n >= 0 as mpd_ssize_t {
        mpd_qshiftl(result, a, n, status);
        _mpd_cap(result, ctx);
    } else {
        if mpd_qcopy(result, a, status) == 0 {
            return;
        }
        _mpd_cap(result, ctx);
        mpd_qshiftr_inplace(result, -n);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qxor(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut big: *const mpd_t = a;
    let mut small: *const mpd_t = b;
    let mut x: mpd_uint_t = 0;
    let mut y: mpd_uint_t = 0;
    let mut z: mpd_uint_t = 0;
    let mut xbit: mpd_uint_t = 0;
    let mut ybit: mpd_uint_t = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut mswdigits: ::core::ffi::c_int = 0;
    let mut i: mpd_ssize_t = 0;
    if mpd_isspecial(a) != 0
        || mpd_isspecial(b) != 0
        || mpd_isnegative(a) != 0
        || mpd_isnegative(b) != 0
        || (*a).exp != 0 as mpd_ssize_t
        || (*b).exp != 0 as mpd_ssize_t
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if (*b).digits > (*a).digits {
        big = b;
        small = a;
    }
    if mpd_qresize(result, (*big).len, status) == 0 {
        return;
    }
    i = 0 as mpd_ssize_t;
    's_48: loop {
        if !(i < (*small).len - 1 as mpd_ssize_t) {
            current_block = 1;
            break;
        }
        x = *(*small).data.offset(i as isize);
        y = *(*big).data.offset(i as isize);
        z = 0 as mpd_uint_t;
        k = 0 as ::core::ffi::c_int;
        while k < MPD_RDIGITS {
            xbit = x.wrapping_rem(10 as mpd_uint_t);
            x = x.wrapping_div(10 as mpd_uint_t);
            ybit = y.wrapping_rem(10 as mpd_uint_t);
            y = y.wrapping_div(10 as mpd_uint_t);
            if xbit > 1 as mpd_uint_t || ybit > 1 as mpd_uint_t {
                current_block = 2;
                break 's_48;
            }
            z = z.wrapping_add(if xbit ^ ybit != 0 {
                *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize)
            } else {
                0 as mpd_uint_t
            });
            k += 1;
        }
        *(*result).data.offset(i as isize) = z;
        i += 1;
    }
    match current_block {
        1 => {
            x = *(*small).data.offset(i as isize);
            y = *(*big).data.offset(i as isize);
            z = 0 as mpd_uint_t;
            mswdigits = mpd_word_digits(x);
            k = 0 as ::core::ffi::c_int;
            loop {
                if !(k < mswdigits) {
                    current_block = 3;
                    break;
                }
                xbit = x.wrapping_rem(10 as mpd_uint_t);
                x = x.wrapping_div(10 as mpd_uint_t);
                ybit = y.wrapping_rem(10 as mpd_uint_t);
                y = y.wrapping_div(10 as mpd_uint_t);
                if xbit > 1 as mpd_uint_t || ybit > 1 as mpd_uint_t {
                    current_block = 2;
                    break;
                }
                z = z.wrapping_add(if xbit ^ ybit != 0 {
                    *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize)
                } else {
                    0 as mpd_uint_t
                });
                k += 1;
            }
            match current_block {
                2 => {}
                _ => {
                    loop {
                        if !(k < MPD_RDIGITS) {
                            current_block = 4;
                            break;
                        }
                        ybit = y.wrapping_rem(10 as mpd_uint_t);
                        y = y.wrapping_div(10 as mpd_uint_t);
                        if ybit > 1 as mpd_uint_t {
                            current_block = 2;
                            break;
                        }
                        z = z.wrapping_add(ybit.wrapping_mul(
                            *(&raw const mpd_pow10 as *const mpd_uint_t).offset(k as isize),
                        ));
                        k += 1;
                    }
                    match current_block {
                        2 => {}
                        _ => {
                            let fresh0 = i;
                            i = i + 1;
                            *(*result).data.offset(fresh0 as isize) = z;
                            's_184: loop {
                                if !(i < (*big).len) {
                                    current_block = 5;
                                    break;
                                }
                                y = *(*big).data.offset(i as isize);
                                k = 0 as ::core::ffi::c_int;
                                while k < MPD_RDIGITS {
                                    ybit = y.wrapping_rem(10 as mpd_uint_t);
                                    y = y.wrapping_div(10 as mpd_uint_t);
                                    if ybit > 1 as mpd_uint_t {
                                        current_block = 2;
                                        break 's_184;
                                    }
                                    k += 1;
                                }
                                *(*result).data.offset(i as isize) =
                                    *(*big).data.offset(i as isize);
                                i += 1;
                            }
                            match current_block {
                                2 => {}
                                _ => {
                                    mpd_clear_flags(result);
                                    (*result).exp = 0 as mpd_ssize_t;
                                    (*result).len = _mpd_real_size((*result).data, (*big).len);
                                    mpd_qresize(result, (*result).len, status);
                                    mpd_setdigits(result);
                                    _mpd_cap(result, ctx);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qabs(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
    }
    if mpd_isnegative(a) != 0 {
        mpd_qminus(result, a, ctx, status);
    } else {
        mpd_qplus(result, a, ctx, status);
    };
}
unsafe extern "C" fn _mpd_ptrswap(mut a: *mut *const mpd_t, mut b: *mut *const mpd_t) {
    let mut t: *const mpd_t = *a;
    *a = *b;
    *b = t;
}
unsafe extern "C" fn _mpd_qaddsub_inf(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut sign_b: uint8_t,
    mut status: *mut uint32_t,
) {
    if mpd_isinfinite(a) != 0 {
        if mpd_sign(a) as ::core::ffi::c_int != sign_b as ::core::ffi::c_int
            && mpd_isinfinite(b) != 0
        {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        } else {
            mpd_setspecial(result, mpd_sign(a), MPD_INF);
        }
        return;
    }
    if mpd_isinfinite(b) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"mpd_isinfinite(b)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    mpd_setspecial(result, sign_b, MPD_INF);
}
unsafe extern "C" fn _mpd_qaddsub(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut sign_b: uint8_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut big: *const mpd_t = ::core::ptr::null::<mpd_t>();
    let mut small: *const mpd_t = ::core::ptr::null::<mpd_t>();
    let mut big_aligned_data: [mpd_uint_t; 64] = [0; 64];
    let mut big_aligned: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut big_aligned_data as *mut mpd_uint_t,
    };
    let mut tiny_data: [mpd_uint_t; 1] = [1 as mpd_uint_t];
    let mut tiny: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut tiny_data as *mut mpd_uint_t,
    };
    let mut carry: mpd_uint_t = 0;
    let mut newsize: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut exp: mpd_ssize_t = 0;
    let mut i: mpd_ssize_t = 0;
    let mut swap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    big = a;
    small = b;
    if (*big).exp != (*small).exp {
        if (*small).exp > (*big).exp {
            _mpd_ptrswap(&raw mut big, &raw mut small);
            swap += 1;
        }
        if mpd_iszerocoeff(big) == 0 {
            exp = (*big).exp - 1 as mpd_ssize_t;
            exp += if (*big).digits > (*ctx).prec {
                0 as mpd_ssize_t
            } else {
                (*big).digits - (*ctx).prec - 1 as mpd_ssize_t
            };
            if mpd_adjexp(small) < exp {
                mpd_copy_flags(&raw mut tiny, small);
                tiny.exp = exp;
                tiny.digits = 1 as mpd_ssize_t;
                tiny.len = 1 as mpd_ssize_t;
                *tiny.data.offset(0 as isize) = (if mpd_iszerocoeff(small) != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as mpd_uint_t;
                small = &raw mut tiny;
            }
            shift = (*big).exp - (*small).exp;
            if mpd_qshiftl(&raw mut big_aligned, big, shift, status) == 0 {
                mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
                current_block = 1;
            } else {
                big = &raw mut big_aligned;
                current_block = 2;
            }
        } else {
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            (*result).exp = (*small).exp;
            if (*big).len < (*small).len {
                _mpd_ptrswap(&raw mut big, &raw mut small);
                swap += 1;
            }
            newsize = (*big).len;
            if !(mpd_qresize(result, newsize, status) == 0) {
                if mpd_sign(a) as ::core::ffi::c_int == sign_b as ::core::ffi::c_int {
                    carry = _mpd_baseadd(
                        (*result).data,
                        (*big).data,
                        (*small).data,
                        (*big).len as mpd_size_t,
                        (*small).len as mpd_size_t,
                    );
                    if carry != 0 {
                        newsize = (*big).len + 1 as mpd_ssize_t;
                        if mpd_qresize(result, newsize, status) == 0 {
                            current_block = 1;
                        } else {
                            *(*result).data.offset((newsize - 1 as mpd_ssize_t) as isize) = carry;
                            current_block = 3;
                        }
                    } else {
                        current_block = 3;
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            (*result).len = newsize;
                            mpd_set_flags(result, sign_b);
                            current_block = 4;
                        }
                    }
                } else {
                    if (*big).len == (*small).len {
                        i = (*big).len - 1 as mpd_ssize_t;
                        while i >= 0 as mpd_ssize_t {
                            if *(*big).data.offset(i as isize) != *(*small).data.offset(i as isize)
                            {
                                if *(*big).data.offset(i as isize)
                                    < *(*small).data.offset(i as isize)
                                {
                                    _mpd_ptrswap(&raw mut big, &raw mut small);
                                    swap += 1;
                                }
                                break;
                            } else {
                                i -= 1;
                            }
                        }
                    }
                    _mpd_basesub(
                        (*result).data,
                        (*big).data,
                        (*small).data,
                        (*big).len as mpd_size_t,
                        (*small).len as mpd_size_t,
                    );
                    newsize = _mpd_real_size((*result).data, (*big).len);
                    mpd_qresize(result, newsize, status);
                    (*result).len = newsize;
                    sign_b = (if swap & 1 as ::core::ffi::c_int != 0 {
                        sign_b as ::core::ffi::c_int
                    } else {
                        mpd_sign(a) as ::core::ffi::c_int
                    }) as uint8_t;
                    mpd_set_flags(result, sign_b);
                    if mpd_iszerocoeff(result) != 0 {
                        mpd_set_positive(result);
                        if (*ctx).round == MPD_ROUND_FLOOR as ::core::ffi::c_int {
                            mpd_set_negative(result);
                        }
                    }
                    current_block = 4;
                }
                match current_block {
                    1 => {}
                    _ => {
                        mpd_setdigits(result);
                    }
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut big_aligned);
}
unsafe extern "C" fn _mpd_qadd(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _mpd_qaddsub(result, a, b, mpd_sign(b), ctx, status);
}
unsafe extern "C" fn _mpd_qsub(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _mpd_qaddsub(
        result,
        a,
        b,
        (mpd_sign(b) == 0) as ::core::ffi::c_int as uint8_t,
        ctx,
        status,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qadd(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return;
        }
        _mpd_qaddsub_inf(result, a, b, mpd_sign(b), status);
        return;
    }
    _mpd_qaddsub(result, a, b, mpd_sign(b), ctx, status);
    mpd_qfinalize(result, ctx, status);
}
unsafe extern "C" fn _mpd_qadd_exact(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    mpd_qadd(result, a, b, ctx, &raw mut workstatus);
    *status |= workstatus;
    func_count += 1;
    if func_count == func_fail {
        workstatus = (workstatus as ::core::ffi::c_uint | MPD_Inexact) as uint32_t;
    }
    if workstatus & (MPD_Inexact as uint32_t | MPD_Rounded as uint32_t | MPD_Clamped as uint32_t)
        != 0
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsub(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return;
        }
        _mpd_qaddsub_inf(
            result,
            a,
            b,
            (mpd_sign(b) == 0) as ::core::ffi::c_int as uint8_t,
            status,
        );
        return;
    }
    _mpd_qaddsub(
        result,
        a,
        b,
        (mpd_sign(b) == 0) as ::core::ffi::c_int as uint8_t,
        ctx,
        status,
    );
    mpd_qfinalize(result, ctx, status);
}
unsafe extern "C" fn _mpd_qsub_exact(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    mpd_qsub(result, a, b, ctx, &raw mut workstatus);
    *status |= workstatus;
    func_count += 1;
    if func_count == func_fail {
        workstatus = (workstatus as ::core::ffi::c_uint | MPD_Inexact) as uint32_t;
    }
    if workstatus & (MPD_Inexact as uint32_t | MPD_Rounded as uint32_t | MPD_Clamped as uint32_t)
        != 0
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qadd_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_ssize(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qadd(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qadd_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_uint(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qadd(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsub_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_ssize(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qsub(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsub_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_uint(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qsub(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qadd_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qadd_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qadd_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qadd_uint(result, a, b as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qadd_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qadd_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qadd_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qadd_uint(result, a, b as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsub_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsub_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsub_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsub_uint(result, a, b as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsub_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsub_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsub_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qsub_uint(result, a, b as mpd_uint_t, ctx, status);
}
unsafe extern "C" fn _mpd_qdiv_inf(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isinfinite(a) != 0 {
        if mpd_isinfinite(b) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        mpd_setspecial(
            result,
            (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t,
            MPD_INF,
        );
        return;
    }
    if mpd_isinfinite(b) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"mpd_isinfinite(b)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    _settriple(
        result,
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t,
        0 as mpd_uint_t,
        mpd_etiny(ctx),
    );
    *status = (*status as ::core::ffi::c_uint | MPD_Clamped) as uint32_t;
}
unsafe extern "C" fn _mpd_qdiv(
    mut action: ::core::ffi::c_int,
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut aligned_data: [mpd_uint_t; 64] = [0; 64];
    let mut aligned: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut aligned_data as *mut mpd_uint_t,
    };
    let mut ld: mpd_uint_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut exp: mpd_ssize_t = 0;
    let mut tz: mpd_ssize_t = 0;
    let mut newsize: mpd_ssize_t = 0;
    let mut ideal_exp: mpd_ssize_t = 0;
    let mut rem: mpd_uint_t = 0;
    let mut sign_a: uint8_t = mpd_sign(a);
    let mut sign_b: uint8_t = mpd_sign(b);
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(q, a, b, ctx, status) != 0 {
            return;
        }
        _mpd_qdiv_inf(q, a, b, ctx, status);
        return;
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_seterror(q, MPD_Division_undefined as uint32_t, status);
        } else {
            mpd_setspecial(
                q,
                (sign_a as ::core::ffi::c_int ^ sign_b as ::core::ffi::c_int) as uint8_t,
                MPD_INF,
            );
            *status = (*status as ::core::ffi::c_uint | MPD_Division_by_zero) as uint32_t;
        }
        return;
    }
    if mpd_iszerocoeff(a) != 0 {
        exp = (*a).exp - (*b).exp;
        _settriple(
            q,
            (sign_a as ::core::ffi::c_int ^ sign_b as ::core::ffi::c_int) as uint8_t,
            0 as mpd_uint_t,
            exp,
        );
        mpd_qfinalize(q, ctx, status);
        return;
    }
    shift = (*b).digits - (*a).digits + (*ctx).prec + 1 as mpd_ssize_t;
    ideal_exp = (*a).exp - (*b).exp;
    exp = ideal_exp - shift;
    if shift > 0 as mpd_ssize_t {
        if mpd_qshiftl(&raw mut aligned, a, shift, status) == 0 {
            mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
            current_block = 1;
        } else {
            a = &raw mut aligned;
            current_block = 2;
        }
    } else if shift < 0 as mpd_ssize_t {
        shift = -shift;
        if mpd_qshiftl(&raw mut aligned, b, shift, status) == 0 {
            mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
            current_block = 1;
        } else {
            b = &raw mut aligned;
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            newsize = (*a).len - (*b).len + 1 as mpd_ssize_t;
            if q != b as *mut mpd_t && q != a as *mut mpd_t
                || q == b as *mut mpd_t && newsize > (*b).len
            {
                if mpd_qresize(q, newsize, status) == 0 {
                    mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
                    current_block = 1;
                } else {
                    current_block = 3;
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    if (*b).len == 1 as mpd_ssize_t {
                        rem = _mpd_shortdiv(
                            (*q).data,
                            (*a).data,
                            (*a).len as mpd_size_t,
                            *(*b).data.offset(0 as isize),
                        );
                        current_block = 4;
                    } else if (*b).len <= MPD_NEWTONDIV_CUTOFF as mpd_ssize_t {
                        let mut ret: ::core::ffi::c_int = _mpd_basedivmod(
                            (*q).data,
                            ::core::ptr::null_mut::<mpd_uint_t>(),
                            (*a).data,
                            (*b).data,
                            (*a).len as mpd_size_t,
                            (*b).len as mpd_size_t,
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
                            current_block = 1;
                        } else {
                            rem = ret as mpd_uint_t;
                            current_block = 4;
                        }
                    } else {
                        let mut r_data: [mpd_uint_t; 64] = [0; 64];
                        let mut r: mpd_t = mpd_t {
                            flags: (0 as ::core::ffi::c_int
                                | MPD_STATIC as ::core::ffi::c_int
                                | MPD_STATIC_DATA as ::core::ffi::c_int)
                                as uint8_t,
                            exp: 0 as mpd_ssize_t,
                            digits: 0 as mpd_ssize_t,
                            len: 0 as mpd_ssize_t,
                            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
                            data: &raw mut r_data as *mut mpd_uint_t,
                        };
                        _mpd_base_ndivmod(q, &raw mut r, a, b, status);
                        if mpd_isspecial(q) != 0 || mpd_isspecial(&raw mut r) != 0 {
                            mpd_setspecial(q, MPD_POS, MPD_NAN);
                            mpd_del(&raw mut r);
                            current_block = 1;
                        } else {
                            rem = (mpd_iszerocoeff(&raw mut r) == 0) as ::core::ffi::c_int
                                as mpd_uint_t;
                            mpd_del(&raw mut r);
                            newsize = (*q).len;
                            current_block = 4;
                        }
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            newsize = _mpd_real_size((*q).data, newsize);
                            mpd_qresize(q, newsize, status);
                            mpd_set_flags(
                                q,
                                (sign_a as ::core::ffi::c_int ^ sign_b as ::core::ffi::c_int)
                                    as uint8_t,
                            );
                            (*q).len = newsize;
                            mpd_setdigits(q);
                            shift = ideal_exp - exp;
                            if rem != 0 {
                                ld = mpd_lsd(*(*q).data.offset(0 as isize));
                                if ld == 0 as mpd_uint_t || ld == 5 as mpd_uint_t {
                                    let ref mut fresh0 = *(*q).data.offset(0 as isize);
                                    *fresh0 = (*fresh0).wrapping_add(1 as mpd_uint_t);
                                }
                            } else if action == SET_IDEAL_EXP as ::core::ffi::c_int
                                && shift > 0 as mpd_ssize_t
                            {
                                tz = mpd_trail_zeros(q);
                                shift = if tz > shift { shift } else { tz };
                                mpd_qshiftr_inplace(q, shift);
                                exp += shift;
                            }
                            (*q).exp = exp;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut aligned);
    mpd_qfinalize(q, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdiv(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut aa_data: [mpd_uint_t; 64] = [0; 64];
    let mut aa: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut aa_data as *mut mpd_uint_t,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    let mut xstatus: uint32_t = 0 as uint32_t;
    if q == a as *mut mpd_t {
        if mpd_qcopy(&raw mut aa, a, status) == 0 {
            mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
            current_block = 1;
        } else {
            a = &raw mut aa;
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            if q == b as *mut mpd_t {
                if mpd_qcopy(&raw mut bb, b, status) == 0 {
                    mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
                    current_block = 1;
                } else {
                    b = &raw mut bb;
                    current_block = 3;
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    _mpd_qdiv(
                        SET_IDEAL_EXP as ::core::ffi::c_int,
                        q,
                        a,
                        b,
                        ctx,
                        &raw mut xstatus,
                    );
                    if xstatus
                        & (MPD_Malloc_error as uint32_t | MPD_Division_impossible as uint32_t)
                        != 0
                    {
                        let mut ystatus: uint32_t = 0 as uint32_t;
                        let mut workctx: mpd_context_t = mpd_context_t {
                            prec: 0,
                            emax: 0,
                            emin: 0,
                            traps: 0,
                            status: 0,
                            newtrap: 0,
                            round: 0,
                            clamp: 0,
                            allcr: 0,
                        };
                        mpd_workcontext(&raw mut workctx, ctx);
                        workctx.prec = (*a).digits + (*b).digits * 4 as mpd_ssize_t;
                        if workctx.prec >= (*ctx).prec {
                            *status = (*status as ::core::ffi::c_uint
                                | (xstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
                                as uint32_t;
                        } else {
                            _mpd_qdiv(
                                SET_IDEAL_EXP as ::core::ffi::c_int,
                                q,
                                a,
                                b,
                                &raw mut workctx,
                                &raw mut ystatus,
                            );
                            if ystatus != 0 as uint32_t {
                                ystatus = *status | (ystatus | xstatus) & MPD_Errors as uint32_t;
                                mpd_seterror(q, ystatus, status);
                            }
                        }
                    } else {
                        *status |= xstatus;
                    }
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut aa);
    mpd_del(&raw mut bb);
}
unsafe extern "C" fn _mpd_qdivmod(
    mut q: *mut mpd_t,
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut aligned_data: [mpd_uint_t; 64] = [0; 64];
    let mut aligned: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut aligned_data as *mut mpd_uint_t,
    };
    let mut qsize: mpd_ssize_t = 0;
    let mut rsize: mpd_ssize_t = 0;
    let mut ideal_exp: mpd_ssize_t = 0;
    let mut expdiff: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut sign_a: uint8_t = mpd_sign(a);
    let mut sign_ab: uint8_t =
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t;
    ideal_exp = if (*a).exp > (*b).exp {
        (*b).exp
    } else {
        (*a).exp
    };
    if mpd_iszerocoeff(a) != 0 {
        if mpd_qcopy(r, a, status) == 0 {
            current_block = 1;
        } else {
            (*r).exp = ideal_exp;
            _settriple(q, sign_ab, 0 as mpd_uint_t, 0 as mpd_ssize_t);
            return;
        }
    } else {
        expdiff = mpd_adjexp(a) - mpd_adjexp(b);
        if expdiff < 0 as mpd_ssize_t {
            if (*a).exp > (*b).exp {
                shift = (*a).exp - (*b).exp;
                if mpd_qshiftl(r, a, shift, status) == 0 {
                    current_block = 1;
                } else {
                    (*r).exp = ideal_exp;
                    current_block = 2;
                }
            } else if mpd_qcopy(r, a, status) == 0 {
                current_block = 1;
            } else {
                current_block = 2;
            }
            match current_block {
                1 => {}
                _ => {
                    _settriple(q, sign_ab, 0 as mpd_uint_t, 0 as mpd_ssize_t);
                    return;
                }
            }
        } else if expdiff > (*ctx).prec {
            *status = (*status as ::core::ffi::c_uint | MPD_Division_impossible) as uint32_t;
            current_block = 1;
        } else {
            if (*a).exp != (*b).exp {
                shift = (*a).exp - (*b).exp;
                if shift > 0 as mpd_ssize_t {
                    if mpd_qshiftl(&raw mut aligned, a, shift, status) == 0 {
                        current_block = 1;
                    } else {
                        a = &raw mut aligned;
                        current_block = 3;
                    }
                } else {
                    shift = -shift;
                    if mpd_qshiftl(&raw mut aligned, b, shift, status) == 0 {
                        current_block = 1;
                    } else {
                        b = &raw mut aligned;
                        current_block = 3;
                    }
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    qsize = (*a).len - (*b).len + 1 as mpd_ssize_t;
                    if !(q == a as *mut mpd_t && qsize < (*a).len)
                        && !(q == b as *mut mpd_t && qsize < (*b).len)
                    {
                        if mpd_qresize(q, qsize, status) == 0 {
                            current_block = 1;
                        } else {
                            current_block = 4;
                        }
                    } else {
                        current_block = 4;
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            rsize = (*b).len;
                            if !(r == a as *mut mpd_t && rsize < (*a).len) {
                                if mpd_qresize(r, rsize, status) == 0 {
                                    current_block = 1;
                                } else {
                                    current_block = 5;
                                }
                            } else {
                                current_block = 5;
                            }
                            match current_block {
                                1 => {}
                                _ => {
                                    if (*b).len == 1 as mpd_ssize_t {
                                        if *(*b).data.offset(0 as isize) != 0 as mpd_uint_t {
                                        } else {
                                            mpd_c2rust_assert_fail(
                                                b"b->data[0] != 0\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        };
                                        if (*a).len == 1 as mpd_ssize_t {
                                            _mpd_div_word(
                                                (*q).data.offset(0 as isize) as *mut mpd_uint_t,
                                                (*r).data.offset(0 as isize) as *mut mpd_uint_t,
                                                *(*a).data.offset(0 as isize),
                                                *(*b).data.offset(0 as isize),
                                            );
                                        } else {
                                            *(*r).data.offset(0 as isize) = _mpd_shortdiv(
                                                (*q).data,
                                                (*a).data,
                                                (*a).len as mpd_size_t,
                                                *(*b).data.offset(0 as isize),
                                            );
                                        }
                                        current_block = 6;
                                    } else if (*b).len <= MPD_NEWTONDIV_CUTOFF as mpd_ssize_t {
                                        let mut ret: ::core::ffi::c_int = 0;
                                        ret = _mpd_basedivmod(
                                            (*q).data,
                                            (*r).data,
                                            (*a).data,
                                            (*b).data,
                                            (*a).len as mpd_size_t,
                                            (*b).len as mpd_size_t,
                                        );
                                        if ret == -(1 as ::core::ffi::c_int) {
                                            *status = (*status as ::core::ffi::c_uint
                                                | MPD_Malloc_error)
                                                as uint32_t;
                                            current_block = 1;
                                        } else {
                                            current_block = 6;
                                        }
                                    } else {
                                        _mpd_base_ndivmod(q, r, a, b, status);
                                        if mpd_isspecial(q) != 0 || mpd_isspecial(r) != 0 {
                                            current_block = 1;
                                        } else {
                                            qsize = (*q).len;
                                            rsize = (*r).len;
                                            current_block = 6;
                                        }
                                    }
                                    match current_block {
                                        1 => {}
                                        _ => {
                                            qsize = _mpd_real_size((*q).data, qsize);
                                            mpd_qresize(q, qsize, status);
                                            (*q).len = qsize;
                                            mpd_setdigits(q);
                                            mpd_set_flags(q, sign_ab);
                                            (*q).exp = 0 as mpd_ssize_t;
                                            if (*q).digits > (*ctx).prec {
                                                *status = (*status as ::core::ffi::c_uint
                                                    | MPD_Division_impossible)
                                                    as uint32_t;
                                                current_block = 1;
                                            } else {
                                                rsize = _mpd_real_size((*r).data, rsize);
                                                mpd_qresize(r, rsize, status);
                                                (*r).len = rsize;
                                                mpd_setdigits(r);
                                                mpd_set_flags(r, sign_a);
                                                (*r).exp = ideal_exp;
                                                current_block = 7;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        1 => {
            mpd_setspecial(q, MPD_POS, MPD_NAN);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
        }
        _ => {}
    }
    mpd_del(&raw mut aligned);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdivmod(
    mut q: *mut mpd_t,
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut sign: uint8_t =
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(q, a, b, ctx, status) != 0 {
            mpd_qcopy(r, q, status);
            return;
        }
        if mpd_isinfinite(a) != 0 {
            if mpd_isinfinite(b) != 0 {
                mpd_setspecial(q, MPD_POS, MPD_NAN);
            } else {
                mpd_setspecial(q, sign, MPD_INF);
            }
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return;
        }
        if mpd_isinfinite(b) != 0 {
            if mpd_qcopy(r, a, status) == 0 {
                mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
                return;
            }
            mpd_qfinalize(r, ctx, status);
            _settriple(q, sign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
            return;
        }
        abort();
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_setspecial(q, MPD_POS, MPD_NAN);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status = (*status as ::core::ffi::c_uint | MPD_Division_undefined) as uint32_t;
        } else {
            mpd_setspecial(q, sign, MPD_INF);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status = (*status as ::core::ffi::c_uint
                | (MPD_Division_by_zero | MPD_Invalid_operation)) as uint32_t;
        }
        return;
    }
    _mpd_qdivmod(q, r, a, b, ctx, status);
    mpd_qfinalize(q, ctx, status);
    mpd_qfinalize(r, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdivint(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut r_data: [mpd_uint_t; 64] = [0; 64];
    let mut r: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut r_data as *mut mpd_uint_t,
    };
    let mut sign: uint8_t =
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(q, a, b, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 && mpd_isinfinite(b) != 0 {
            mpd_seterror(q, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        if mpd_isinfinite(a) != 0 {
            mpd_setspecial(q, sign, MPD_INF);
            return;
        }
        if mpd_isinfinite(b) != 0 {
            _settriple(q, sign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
            return;
        }
        abort();
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_seterror(q, MPD_Division_undefined as uint32_t, status);
        } else {
            mpd_setspecial(q, sign, MPD_INF);
            *status = (*status as ::core::ffi::c_uint | MPD_Division_by_zero) as uint32_t;
        }
        return;
    }
    _mpd_qdivmod(q, &raw mut r, a, b, ctx, status);
    mpd_del(&raw mut r);
    mpd_qfinalize(q, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdiv_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_ssize(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qdiv(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdiv_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_uint(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qdiv(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdiv_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qdiv_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdiv_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qdiv_uint(result, a, b as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdiv_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qdiv_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qdiv_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qdiv_uint(result, a, b as mpd_uint_t, ctx, status);
}
unsafe extern "C" fn _mpd_zeropad(
    mut result: *mut mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(result) == 0 && mpd_iszero(result) == 0 && (*result).digits < (*ctx).prec {
        let mut shift: mpd_ssize_t = (*ctx).prec - (*result).digits;
        mpd_qshiftl(result, result, shift, status);
        (*result).exp -= shift;
    }
}
unsafe extern "C" fn _mpd_qexp_check_one(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut lim_data: [mpd_uint_t; 1] = [9 as mpd_uint_t];
    let mut lim: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: -((*ctx).prec + 1 as mpd_ssize_t),
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut lim_data as *mut mpd_uint_t,
    };
    let mut aa: mpd_t = mpd_t {
        flags: ((*a).flags as ::core::ffi::c_int & !MPD_DATAFLAGS
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_SHARED_DATA as ::core::ffi::c_int) as uint8_t,
        exp: (*a).exp,
        digits: (*a).digits,
        len: (*a).len,
        alloc: (*a).alloc,
        data: (*a).data,
    };
    mpd_set_positive(&raw mut aa);
    if _mpd_cmp(&raw mut aa, &raw mut lim) <= 0 as ::core::ffi::c_int {
        _settriple(result, 0 as uint8_t, 1 as mpd_uint_t, 0 as mpd_ssize_t);
        *status = (*status as ::core::ffi::c_uint | (MPD_Rounded | MPD_Inexact)) as uint32_t;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_get_exp_iterations(
    mut r: *const mpd_t,
    mut p: mpd_ssize_t,
) -> mpd_ssize_t {
    let mut log10pbyr: mpd_ssize_t = 0;
    let mut n: mpd_ssize_t = 0;
    if p >= 10 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(b"p >= 10\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszero(r) == 0 {
    } else {
        mpd_c2rust_assert_fail(b"!mpd_iszero(r)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if -p < mpd_adjexp(r) && mpd_adjexp(r) <= -(1 as ::core::ffi::c_int) as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(
            b"-p < mpd_adjexp(r) && mpd_adjexp(r) <= -1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if p > ((1 as u64) << 52 as ::core::ffi::c_int) as mpd_ssize_t {
        return MPD_SSIZE_MAX as mpd_ssize_t;
    }
    log10pbyr = (mpd_word_digits(p as mpd_uint_t) - 1 as ::core::ffi::c_int) as mpd_ssize_t
        - (mpd_adjexp(r) + 1 as mpd_ssize_t);
    n = ceil(
        (1.43503f64 * p as ::core::ffi::c_double - 1.182f64) / log10pbyr as ::core::ffi::c_double,
    ) as mpd_ssize_t;
    return if n >= 3 as mpd_ssize_t {
        n
    } else {
        3 as mpd_ssize_t
    };
}
unsafe extern "C" fn _mpd_qexp(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut tmp_data: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tmp_data as *mut mpd_uint_t,
    };
    let mut sum_data: [mpd_uint_t; 64] = [0; 64];
    let mut sum: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut sum_data as *mut mpd_uint_t,
    };
    let mut word_data: [mpd_uint_t; 1] = [1 as mpd_uint_t];
    let mut word: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut word_data as *mut mpd_uint_t,
    };
    let mut j: mpd_ssize_t = 0;
    let mut n: mpd_ssize_t = 0;
    let mut t: mpd_ssize_t = 0;
    if mpd_isspecial(a) == 0 {
    } else {
        mpd_c2rust_assert_fail(b"!mpd_isspecial(a)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszerocoeff(a) != 0 {
        _settriple(result, MPD_POS, 1 as mpd_uint_t, 0 as mpd_ssize_t);
        return 1 as ::core::ffi::c_int;
    }
    t = (*a).digits + (*a).exp;
    t = if t > 0 as mpd_ssize_t {
        t
    } else {
        0 as mpd_ssize_t
    };
    if t > MPD_EXP_MAX_T as mpd_ssize_t {
        if mpd_ispositive(a) != 0 {
            mpd_setspecial(result, MPD_POS, MPD_INF);
            *status = (*status as ::core::ffi::c_uint | (MPD_Overflow | MPD_Inexact | MPD_Rounded))
                as uint32_t;
        } else {
            _settriple(result, MPD_POS, 0 as mpd_uint_t, mpd_etiny(ctx));
            *status = (*status as ::core::ffi::c_uint
                | (MPD_Inexact | MPD_Rounded | MPD_Subnormal | MPD_Underflow | MPD_Clamped))
                as uint32_t;
        }
        return 1 as ::core::ffi::c_int;
    }
    if _mpd_qexp_check_one(result, a, ctx, status) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    mpd_maxcontext(&raw mut workctx);
    workctx.prec = (*ctx).prec + t + 2 as mpd_ssize_t;
    workctx.prec = if workctx.prec < 10 as mpd_ssize_t {
        10 as mpd_ssize_t
    } else {
        workctx.prec
    };
    workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    if mpd_qcopy(result, a, status) == 0 {
        return 1 as ::core::ffi::c_int;
    }
    (*result).exp -= t;
    n = _mpd_get_exp_iterations(result, workctx.prec);
    if n == MPD_SSIZE_MAX as mpd_ssize_t {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return 1 as ::core::ffi::c_int;
    }
    _settriple(&raw mut sum, MPD_POS, 1 as mpd_uint_t, 0 as mpd_ssize_t);
    j = n - 1 as mpd_ssize_t;
    while j >= 1 as mpd_ssize_t {
        *word.data.offset(0 as isize) = j as mpd_uint_t;
        mpd_setdigits(&raw mut word);
        mpd_qdiv(
            &raw mut tmp,
            result,
            &raw mut word,
            &raw mut workctx,
            &raw mut workctx.status,
        );
        mpd_qfma(
            &raw mut sum,
            &raw mut sum,
            &raw mut tmp,
            &raw const one,
            &raw mut workctx,
            &raw mut workctx.status,
        );
        j -= 1;
    }
    _mpd_qpow_uint(
        result,
        &raw mut sum,
        *(&raw const mpd_pow10 as *const mpd_uint_t).offset(t as isize),
        MPD_POS,
        &raw mut workctx,
        status,
    );
    mpd_del(&raw mut tmp);
    mpd_del(&raw mut sum);
    *status = (*status as ::core::ffi::c_uint
        | (workctx.status & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
        as uint32_t;
    *status = (*status as ::core::ffi::c_uint | (MPD_Inexact | MPD_Rounded)) as uint32_t;
    return 0 as ::core::ffi::c_int;
}
pub const MPD_EXP_MAX_T: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn mpd_qexp(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        if mpd_isnegative(a) != 0 {
            _settriple(result, MPD_POS, 0 as mpd_uint_t, 0 as mpd_ssize_t);
        } else {
            mpd_setspecial(result, MPD_POS, MPD_INF);
        }
        return;
    }
    if mpd_iszerocoeff(a) != 0 {
        _settriple(result, MPD_POS, 1 as mpd_uint_t, 0 as mpd_ssize_t);
        return;
    }
    mpd_workcontext(&raw mut workctx, ctx);
    workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    if (*ctx).allcr != 0 {
        let mut hi_data: [mpd_uint_t; 64] = [0; 64];
        let mut hi: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut hi_data as *mut mpd_uint_t,
        };
        let mut lo_data: [mpd_uint_t; 64] = [0; 64];
        let mut lo: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut lo_data as *mut mpd_uint_t,
        };
        let mut ulp_data: [mpd_uint_t; 64] = [0; 64];
        let mut ulp: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut ulp_data as *mut mpd_uint_t,
        };
        let mut aa_data: [mpd_uint_t; 64] = [0; 64];
        let mut aa: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut aa_data as *mut mpd_uint_t,
        };
        let mut loop_protect: uint32_t = 0 as uint32_t;
        let mut prec: mpd_ssize_t = 0;
        let mut ulpexp: mpd_ssize_t = 0;
        if result == a as *mut mpd_t {
            if mpd_qcopy(&raw mut aa, a, status) == 0 {
                mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
                return;
            }
            a = &raw mut aa;
        }
        workctx.clamp = 0 as ::core::ffi::c_int;
        prec = (*ctx).prec + 3 as mpd_ssize_t;
        loop {
            let mut status_res: uint32_t = 0 as uint32_t;
            let mut status_hi: uint32_t = 0 as uint32_t;
            let mut status_lo: uint32_t = 0 as uint32_t;
            let mut shortcut: ::core::ffi::c_int = 0;
            let mut bounds_eq: ::core::ffi::c_int = 0;
            let mut subnormal_eq: ::core::ffi::c_int = 0;
            workctx.prec = prec;
            shortcut = _mpd_qexp(result, a, &raw mut workctx, &raw mut status_res);
            if mpd_isnan(result) != 0 {
                mpd_seterror(result, status_res, status);
                break;
            } else if shortcut != 0 || mpd_isinfinite(result) != 0 || mpd_iszero(result) != 0 {
                *status |= status_res;
                workctx.prec = (*ctx).prec;
                workctx.clamp = (*ctx).clamp;
                _mpd_zeropad(result, &raw mut workctx, status);
                mpd_qfinalize(result, &raw mut workctx, status);
                break;
            } else {
                ulpexp = (*result).exp + (*result).digits - workctx.prec;
                if status_res & MPD_Underflow as uint32_t != 0 {
                    ulpexp = (*result).exp;
                }
                _ssettriple(&raw mut ulp, MPD_POS, 1 as mpd_uint_t, ulpexp);
                workctx.prec = (*ctx).prec;
                mpd_qadd(
                    &raw mut hi,
                    result,
                    &raw mut ulp,
                    &raw mut workctx,
                    &raw mut status_hi,
                );
                mpd_qsub(
                    &raw mut lo,
                    result,
                    &raw mut ulp,
                    &raw mut workctx,
                    &raw mut status_lo,
                );
                if mpd_isnan(&raw mut hi) != 0 || mpd_isnan(&raw mut lo) != 0 {
                    mpd_seterror(result, status_hi | status_lo, status);
                    break;
                } else {
                    subnormal_eq = (status_hi & MPD_Subnormal as uint32_t
                        == status_lo & MPD_Subnormal as uint32_t)
                        as ::core::ffi::c_int;
                    bounds_eq = (mpd_qcmp(&raw mut hi, &raw mut lo, status)
                        == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if bounds_eq != 0 && {
                        loop_protect = loop_protect.wrapping_add(1);
                        loop_protect > 5 as uint32_t
                    } {
                        subnormal_eq = 1 as ::core::ffi::c_int;
                    }
                    if subnormal_eq != 0 && bounds_eq != 0 {
                        *status |= status_lo;
                        workctx.clamp = (*ctx).clamp;
                        _mpd_zeropad(result, &raw mut workctx, status);
                        mpd_qfinalize(result, &raw mut workctx, status);
                        break;
                    } else {
                        if subnormal_eq != 0 {
                            prec += MPD_RDIGITS as mpd_ssize_t;
                        } else {
                            prec *= 2 as mpd_ssize_t;
                        }
                        if !(prec > MPD_MAX_PREC as mpd_ssize_t) {
                            continue;
                        }
                        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
                        break;
                    }
                }
            }
        }
        mpd_del(&raw mut hi);
        mpd_del(&raw mut lo);
        mpd_del(&raw mut ulp);
        mpd_del(&raw mut aa);
    } else {
        _mpd_qexp(result, a, &raw mut workctx, status);
        _mpd_zeropad(result, &raw mut workctx, status);
        mpd_check_underflow(result, &raw mut workctx, status);
        mpd_qfinalize(result, &raw mut workctx, status);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qfma(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut c: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut cc: *mut mpd_t = ::core::ptr::null_mut::<mpd_t>();
    if result == c as *mut mpd_t {
        cc = mpd_qncopy(c);
        if cc.is_null() {
            mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
            return;
        }
        c = cc;
    }
    _mpd_qmul(result, a, b, ctx, &raw mut workstatus);
    if workstatus & MPD_Invalid_operation as uint32_t == 0 {
        mpd_qadd(result, result, c, ctx, &raw mut workstatus);
    }
    if !cc.is_null() {
        mpd_del(cc);
    }
    *status |= workstatus;
}
unsafe extern "C" fn ln_schedule_prec(
    mut klist: *mut mpd_ssize_t,
    mut maxprec: mpd_ssize_t,
    mut initprec: mpd_ssize_t,
) -> ::core::ffi::c_int {
    let mut k: mpd_ssize_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    if maxprec >= 2 as mpd_ssize_t && initprec >= 2 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(
            b"maxprec >= 2 && initprec >= 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if maxprec <= initprec {
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    k = maxprec;
    loop {
        k = (k + 2 as mpd_ssize_t) / 2 as mpd_ssize_t;
        let fresh0 = i;
        i = i + 1;
        *klist.offset(fresh0 as isize) = k;
        if !(k > initprec) {
            break;
        }
    }
    return i - 1 as ::core::ffi::c_int;
}
static mut mpd_ln10_data: [mpd_uint_t; 64] = [
    6983716328982174407 as u64,
    9089704281976336583 as u64,
    1515961135648465461 as u64,
    4416816335727555703 as u64,
    2900988039194170265 as u64,
    2307925037472986509 as u64,
    107598438319191292 as u64,
    3466624107184669231 as u64,
    4450099781311469159 as u64,
    9807828059751193854 as u64,
    7713456862091670584 as u64,
    1492198849978748873 as u64,
    6528728696511086257 as u64,
    2385392051446341972 as u64,
    8692180205189339507 as u64,
    6518769751037497088 as u64,
    2375253577097505395 as u64,
    9095610299291824318 as u64,
    982748238504564801 as u64,
    5438635917781170543 as u64,
    7547331541421808427 as u64,
    752371033310119785 as u64,
    3171643095059950878 as u64,
    9785265383207606726 as u64,
    2932258279850258550 as u64,
    5497347726624257094 as u64,
    2976979522110718264 as u64,
    9221477656763693866 as u64,
    1979650047149510504 as u64,
    6674183485704422507 as u64,
    9702766860595249671 as u64,
    9278096762712757753 as u64,
    9314848524948644871 as u64,
    6826928280848118428 as u64,
    754403708474699401 as u64,
    230105703089634572 as u64,
    1929203337658714166 as u64,
    7589402567763113569 as u64,
    4208241314695689016 as u64,
    2922455440575892572 as u64,
    9356734206705811364 as u64,
    2684916746550586856 as u64,
    644507064800027750 as u64,
    9476834636167921018 as u64,
    5659121373450747856 as u64,
    2835522011480466371 as u64,
    6470806855677432162 as u64,
    7141748003688084012 as u64,
    9619404400222105101 as u64,
    5504893431493939147 as u64,
    6674744042432743651 as u64,
    2287698219886746543 as u64,
    7773262884616336622 as u64,
    1985283935053089653 as u64,
    4680843799894826233 as u64,
    8168948290720832555 as u64,
    8067566662873690987 as u64,
    6248633409525465082 as u64,
    9829834196778404228 as u64,
    3524802359972050895 as u64,
    3327900967572609677 as u64,
    110148862877297603 as u64,
    179914546843642076 as u64,
    2302585092994045684 as u64,
];
static mut _mpd_ln10: mpd_t = unsafe {
    mpd_t {
        flags: (MPD_STATIC as ::core::ffi::c_int | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: -(MPD_MINALLOC_MAX * MPD_RDIGITS - 1 as ::core::ffi::c_int) as mpd_ssize_t,
        digits: (MPD_MINALLOC_MAX * MPD_RDIGITS) as mpd_ssize_t,
        len: MPD_MINALLOC_MAX as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw const mpd_ln10_data as *const mpd_uint_t as *mut mpd_uint_t,
    }
};
#[no_mangle]
pub unsafe extern "C" fn mpd_qln10(
    mut result: *mut mpd_t,
    mut prec: mpd_ssize_t,
    mut status: *mut uint32_t,
) {
    let mut varcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut tmp_data: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tmp_data as *mut mpd_uint_t,
    };
    let mut static10_data: [mpd_uint_t; 1] = [10 as mpd_uint_t];
    let mut static10: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 2 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut static10_data as *mut mpd_uint_t,
    };
    let mut klist: [mpd_ssize_t; 64] = [0; 64];
    let mut rnd: mpd_uint_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    if prec >= 1 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(b"prec >= 1\0" as *const u8 as *const ::core::ffi::c_char);
    };
    shift = (MPD_MINALLOC_MAX * MPD_RDIGITS) as mpd_ssize_t - prec;
    shift = if shift < 0 as mpd_ssize_t {
        0 as mpd_ssize_t
    } else {
        shift
    };
    rnd = mpd_qshiftr(result, &raw const _mpd_ln10, shift, status);
    if rnd == MPD_UINT_MAX as mpd_uint_t {
        mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
        return;
    }
    (*result).exp = -((*result).digits - 1 as mpd_ssize_t);
    mpd_maxcontext(&raw mut maxcontext);
    if prec < (MPD_MINALLOC_MAX * MPD_RDIGITS) as mpd_ssize_t {
        maxcontext.prec = prec;
        _mpd_apply_round_excess(result, rnd, &raw mut maxcontext, status);
        *status = (*status as ::core::ffi::c_uint | (MPD_Inexact | MPD_Rounded)) as uint32_t;
        return;
    }
    mpd_maxcontext(&raw mut varcontext);
    varcontext.round = MPD_ROUND_TRUNC as ::core::ffi::c_int;
    i = ln_schedule_prec(
        &raw mut klist as *mut mpd_ssize_t,
        prec + 2 as mpd_ssize_t,
        -(*result).exp,
    );
    while i >= 0 as ::core::ffi::c_int {
        varcontext.prec = 2 as mpd_ssize_t * klist[i as usize] + 3 as mpd_ssize_t;
        (*result).flags =
            ((*result).flags as ::core::ffi::c_int ^ MPD_NEG as ::core::ffi::c_int) as uint8_t;
        _mpd_qexp(&raw mut tmp, result, &raw mut varcontext, status);
        (*result).flags =
            ((*result).flags as ::core::ffi::c_int ^ MPD_NEG as ::core::ffi::c_int) as uint8_t;
        mpd_qmul(
            &raw mut tmp,
            &raw mut static10,
            &raw mut tmp,
            &raw mut varcontext,
            status,
        );
        mpd_qsub(
            &raw mut tmp,
            &raw mut tmp,
            &raw const one,
            &raw mut maxcontext,
            status,
        );
        mpd_qadd(result, result, &raw mut tmp, &raw mut maxcontext, status);
        if mpd_isspecial(result) != 0 {
            break;
        }
        i -= 1;
    }
    mpd_del(&raw mut tmp);
    maxcontext.prec = prec;
    mpd_qfinalize(result, &raw mut maxcontext, status);
}
static mut lnapprox: [uint16_t; 900] = [
    0 as uint16_t,
    10 as uint16_t,
    20 as uint16_t,
    30 as uint16_t,
    39 as uint16_t,
    49 as uint16_t,
    58 as uint16_t,
    68 as uint16_t,
    77 as uint16_t,
    86 as uint16_t,
    95 as uint16_t,
    104 as uint16_t,
    113 as uint16_t,
    122 as uint16_t,
    131 as uint16_t,
    140 as uint16_t,
    148 as uint16_t,
    157 as uint16_t,
    166 as uint16_t,
    174 as uint16_t,
    182 as uint16_t,
    191 as uint16_t,
    199 as uint16_t,
    207 as uint16_t,
    215 as uint16_t,
    223 as uint16_t,
    231 as uint16_t,
    239 as uint16_t,
    247 as uint16_t,
    255 as uint16_t,
    262 as uint16_t,
    270 as uint16_t,
    278 as uint16_t,
    285 as uint16_t,
    293 as uint16_t,
    300 as uint16_t,
    308 as uint16_t,
    315 as uint16_t,
    322 as uint16_t,
    329 as uint16_t,
    336 as uint16_t,
    344 as uint16_t,
    351 as uint16_t,
    358 as uint16_t,
    365 as uint16_t,
    372 as uint16_t,
    378 as uint16_t,
    385 as uint16_t,
    392 as uint16_t,
    399 as uint16_t,
    406 as uint16_t,
    412 as uint16_t,
    419 as uint16_t,
    425 as uint16_t,
    432 as uint16_t,
    438 as uint16_t,
    445 as uint16_t,
    451 as uint16_t,
    457 as uint16_t,
    464 as uint16_t,
    470 as uint16_t,
    476 as uint16_t,
    482 as uint16_t,
    489 as uint16_t,
    495 as uint16_t,
    501 as uint16_t,
    507 as uint16_t,
    513 as uint16_t,
    519 as uint16_t,
    525 as uint16_t,
    531 as uint16_t,
    536 as uint16_t,
    542 as uint16_t,
    548 as uint16_t,
    554 as uint16_t,
    560 as uint16_t,
    565 as uint16_t,
    571 as uint16_t,
    577 as uint16_t,
    582 as uint16_t,
    588 as uint16_t,
    593 as uint16_t,
    599 as uint16_t,
    604 as uint16_t,
    610 as uint16_t,
    615 as uint16_t,
    621 as uint16_t,
    626 as uint16_t,
    631 as uint16_t,
    637 as uint16_t,
    642 as uint16_t,
    647 as uint16_t,
    652 as uint16_t,
    658 as uint16_t,
    663 as uint16_t,
    668 as uint16_t,
    673 as uint16_t,
    678 as uint16_t,
    683 as uint16_t,
    688 as uint16_t,
    693 as uint16_t,
    698 as uint16_t,
    703 as uint16_t,
    708 as uint16_t,
    713 as uint16_t,
    718 as uint16_t,
    723 as uint16_t,
    728 as uint16_t,
    732 as uint16_t,
    737 as uint16_t,
    742 as uint16_t,
    747 as uint16_t,
    751 as uint16_t,
    756 as uint16_t,
    761 as uint16_t,
    766 as uint16_t,
    770 as uint16_t,
    775 as uint16_t,
    779 as uint16_t,
    784 as uint16_t,
    788 as uint16_t,
    793 as uint16_t,
    798 as uint16_t,
    802 as uint16_t,
    806 as uint16_t,
    811 as uint16_t,
    815 as uint16_t,
    820 as uint16_t,
    824 as uint16_t,
    829 as uint16_t,
    833 as uint16_t,
    837 as uint16_t,
    842 as uint16_t,
    846 as uint16_t,
    850 as uint16_t,
    854 as uint16_t,
    859 as uint16_t,
    863 as uint16_t,
    867 as uint16_t,
    871 as uint16_t,
    876 as uint16_t,
    880 as uint16_t,
    884 as uint16_t,
    888 as uint16_t,
    892 as uint16_t,
    896 as uint16_t,
    900 as uint16_t,
    904 as uint16_t,
    908 as uint16_t,
    912 as uint16_t,
    916 as uint16_t,
    920 as uint16_t,
    924 as uint16_t,
    928 as uint16_t,
    932 as uint16_t,
    936 as uint16_t,
    940 as uint16_t,
    944 as uint16_t,
    948 as uint16_t,
    952 as uint16_t,
    956 as uint16_t,
    959 as uint16_t,
    963 as uint16_t,
    967 as uint16_t,
    971 as uint16_t,
    975 as uint16_t,
    978 as uint16_t,
    982 as uint16_t,
    986 as uint16_t,
    990 as uint16_t,
    993 as uint16_t,
    997 as uint16_t,
    1001 as uint16_t,
    1004 as uint16_t,
    1008 as uint16_t,
    1012 as uint16_t,
    1015 as uint16_t,
    1019 as uint16_t,
    1022 as uint16_t,
    1026 as uint16_t,
    1030 as uint16_t,
    1033 as uint16_t,
    1037 as uint16_t,
    1040 as uint16_t,
    1044 as uint16_t,
    1047 as uint16_t,
    1051 as uint16_t,
    1054 as uint16_t,
    1058 as uint16_t,
    1061 as uint16_t,
    1065 as uint16_t,
    1068 as uint16_t,
    1072 as uint16_t,
    1075 as uint16_t,
    1078 as uint16_t,
    1082 as uint16_t,
    1085 as uint16_t,
    1089 as uint16_t,
    1092 as uint16_t,
    1095 as uint16_t,
    1099 as uint16_t,
    1102 as uint16_t,
    1105 as uint16_t,
    1109 as uint16_t,
    1112 as uint16_t,
    1115 as uint16_t,
    1118 as uint16_t,
    1122 as uint16_t,
    1125 as uint16_t,
    1128 as uint16_t,
    1131 as uint16_t,
    1135 as uint16_t,
    1138 as uint16_t,
    1141 as uint16_t,
    1144 as uint16_t,
    1147 as uint16_t,
    1151 as uint16_t,
    1154 as uint16_t,
    1157 as uint16_t,
    1160 as uint16_t,
    1163 as uint16_t,
    1166 as uint16_t,
    1169 as uint16_t,
    1172 as uint16_t,
    1176 as uint16_t,
    1179 as uint16_t,
    1182 as uint16_t,
    1185 as uint16_t,
    1188 as uint16_t,
    1191 as uint16_t,
    1194 as uint16_t,
    1197 as uint16_t,
    1200 as uint16_t,
    1203 as uint16_t,
    1206 as uint16_t,
    1209 as uint16_t,
    1212 as uint16_t,
    1215 as uint16_t,
    1218 as uint16_t,
    1221 as uint16_t,
    1224 as uint16_t,
    1227 as uint16_t,
    1230 as uint16_t,
    1233 as uint16_t,
    1235 as uint16_t,
    1238 as uint16_t,
    1241 as uint16_t,
    1244 as uint16_t,
    1247 as uint16_t,
    1250 as uint16_t,
    1253 as uint16_t,
    1256 as uint16_t,
    1258 as uint16_t,
    1261 as uint16_t,
    1264 as uint16_t,
    1267 as uint16_t,
    1270 as uint16_t,
    1273 as uint16_t,
    1275 as uint16_t,
    1278 as uint16_t,
    1281 as uint16_t,
    1284 as uint16_t,
    1286 as uint16_t,
    1289 as uint16_t,
    1292 as uint16_t,
    1295 as uint16_t,
    1297 as uint16_t,
    1300 as uint16_t,
    1303 as uint16_t,
    1306 as uint16_t,
    1308 as uint16_t,
    1311 as uint16_t,
    1314 as uint16_t,
    1316 as uint16_t,
    1319 as uint16_t,
    1322 as uint16_t,
    1324 as uint16_t,
    1327 as uint16_t,
    1330 as uint16_t,
    1332 as uint16_t,
    1335 as uint16_t,
    1338 as uint16_t,
    1340 as uint16_t,
    1343 as uint16_t,
    1345 as uint16_t,
    1348 as uint16_t,
    1351 as uint16_t,
    1353 as uint16_t,
    1356 as uint16_t,
    1358 as uint16_t,
    1361 as uint16_t,
    1364 as uint16_t,
    1366 as uint16_t,
    1369 as uint16_t,
    1371 as uint16_t,
    1374 as uint16_t,
    1376 as uint16_t,
    1379 as uint16_t,
    1381 as uint16_t,
    1384 as uint16_t,
    1386 as uint16_t,
    1389 as uint16_t,
    1391 as uint16_t,
    1394 as uint16_t,
    1396 as uint16_t,
    1399 as uint16_t,
    1401 as uint16_t,
    1404 as uint16_t,
    1406 as uint16_t,
    1409 as uint16_t,
    1411 as uint16_t,
    1413 as uint16_t,
    1416 as uint16_t,
    1418 as uint16_t,
    1421 as uint16_t,
    1423 as uint16_t,
    1426 as uint16_t,
    1428 as uint16_t,
    1430 as uint16_t,
    1433 as uint16_t,
    1435 as uint16_t,
    1437 as uint16_t,
    1440 as uint16_t,
    1442 as uint16_t,
    1445 as uint16_t,
    1447 as uint16_t,
    1449 as uint16_t,
    1452 as uint16_t,
    1454 as uint16_t,
    1456 as uint16_t,
    1459 as uint16_t,
    1461 as uint16_t,
    1463 as uint16_t,
    1466 as uint16_t,
    1468 as uint16_t,
    1470 as uint16_t,
    1472 as uint16_t,
    1475 as uint16_t,
    1477 as uint16_t,
    1479 as uint16_t,
    1482 as uint16_t,
    1484 as uint16_t,
    1486 as uint16_t,
    1488 as uint16_t,
    1491 as uint16_t,
    1493 as uint16_t,
    1495 as uint16_t,
    1497 as uint16_t,
    1500 as uint16_t,
    1502 as uint16_t,
    1504 as uint16_t,
    1506 as uint16_t,
    1509 as uint16_t,
    1511 as uint16_t,
    1513 as uint16_t,
    1515 as uint16_t,
    1517 as uint16_t,
    1520 as uint16_t,
    1522 as uint16_t,
    1524 as uint16_t,
    1526 as uint16_t,
    1528 as uint16_t,
    1530 as uint16_t,
    1533 as uint16_t,
    1535 as uint16_t,
    1537 as uint16_t,
    1539 as uint16_t,
    1541 as uint16_t,
    1543 as uint16_t,
    1545 as uint16_t,
    1548 as uint16_t,
    1550 as uint16_t,
    1552 as uint16_t,
    1554 as uint16_t,
    1556 as uint16_t,
    1558 as uint16_t,
    1560 as uint16_t,
    1562 as uint16_t,
    1564 as uint16_t,
    1567 as uint16_t,
    1569 as uint16_t,
    1571 as uint16_t,
    1573 as uint16_t,
    1575 as uint16_t,
    1577 as uint16_t,
    1579 as uint16_t,
    1581 as uint16_t,
    1583 as uint16_t,
    1585 as uint16_t,
    1587 as uint16_t,
    1589 as uint16_t,
    1591 as uint16_t,
    1593 as uint16_t,
    1595 as uint16_t,
    1597 as uint16_t,
    1599 as uint16_t,
    1601 as uint16_t,
    1603 as uint16_t,
    1605 as uint16_t,
    1607 as uint16_t,
    1609 as uint16_t,
    691 as uint16_t,
    689 as uint16_t,
    687 as uint16_t,
    685 as uint16_t,
    683 as uint16_t,
    681 as uint16_t,
    679 as uint16_t,
    677 as uint16_t,
    675 as uint16_t,
    673 as uint16_t,
    671 as uint16_t,
    669 as uint16_t,
    668 as uint16_t,
    666 as uint16_t,
    664 as uint16_t,
    662 as uint16_t,
    660 as uint16_t,
    658 as uint16_t,
    656 as uint16_t,
    654 as uint16_t,
    652 as uint16_t,
    650 as uint16_t,
    648 as uint16_t,
    646 as uint16_t,
    644 as uint16_t,
    642 as uint16_t,
    641 as uint16_t,
    639 as uint16_t,
    637 as uint16_t,
    635 as uint16_t,
    633 as uint16_t,
    631 as uint16_t,
    629 as uint16_t,
    627 as uint16_t,
    626 as uint16_t,
    624 as uint16_t,
    622 as uint16_t,
    620 as uint16_t,
    618 as uint16_t,
    616 as uint16_t,
    614 as uint16_t,
    612 as uint16_t,
    611 as uint16_t,
    609 as uint16_t,
    607 as uint16_t,
    605 as uint16_t,
    603 as uint16_t,
    602 as uint16_t,
    600 as uint16_t,
    598 as uint16_t,
    596 as uint16_t,
    594 as uint16_t,
    592 as uint16_t,
    591 as uint16_t,
    589 as uint16_t,
    587 as uint16_t,
    585 as uint16_t,
    583 as uint16_t,
    582 as uint16_t,
    580 as uint16_t,
    578 as uint16_t,
    576 as uint16_t,
    574 as uint16_t,
    573 as uint16_t,
    571 as uint16_t,
    569 as uint16_t,
    567 as uint16_t,
    566 as uint16_t,
    564 as uint16_t,
    562 as uint16_t,
    560 as uint16_t,
    559 as uint16_t,
    557 as uint16_t,
    555 as uint16_t,
    553 as uint16_t,
    552 as uint16_t,
    550 as uint16_t,
    548 as uint16_t,
    546 as uint16_t,
    545 as uint16_t,
    543 as uint16_t,
    541 as uint16_t,
    540 as uint16_t,
    538 as uint16_t,
    536 as uint16_t,
    534 as uint16_t,
    533 as uint16_t,
    531 as uint16_t,
    529 as uint16_t,
    528 as uint16_t,
    526 as uint16_t,
    524 as uint16_t,
    523 as uint16_t,
    521 as uint16_t,
    519 as uint16_t,
    518 as uint16_t,
    516 as uint16_t,
    514 as uint16_t,
    512 as uint16_t,
    511 as uint16_t,
    509 as uint16_t,
    508 as uint16_t,
    506 as uint16_t,
    504 as uint16_t,
    502 as uint16_t,
    501 as uint16_t,
    499 as uint16_t,
    498 as uint16_t,
    496 as uint16_t,
    494 as uint16_t,
    493 as uint16_t,
    491 as uint16_t,
    489 as uint16_t,
    488 as uint16_t,
    486 as uint16_t,
    484 as uint16_t,
    483 as uint16_t,
    481 as uint16_t,
    480 as uint16_t,
    478 as uint16_t,
    476 as uint16_t,
    475 as uint16_t,
    473 as uint16_t,
    472 as uint16_t,
    470 as uint16_t,
    468 as uint16_t,
    467 as uint16_t,
    465 as uint16_t,
    464 as uint16_t,
    462 as uint16_t,
    460 as uint16_t,
    459 as uint16_t,
    457 as uint16_t,
    456 as uint16_t,
    454 as uint16_t,
    453 as uint16_t,
    451 as uint16_t,
    449 as uint16_t,
    448 as uint16_t,
    446 as uint16_t,
    445 as uint16_t,
    443 as uint16_t,
    442 as uint16_t,
    440 as uint16_t,
    438 as uint16_t,
    437 as uint16_t,
    435 as uint16_t,
    434 as uint16_t,
    432 as uint16_t,
    431 as uint16_t,
    429 as uint16_t,
    428 as uint16_t,
    426 as uint16_t,
    425 as uint16_t,
    423 as uint16_t,
    422 as uint16_t,
    420 as uint16_t,
    419 as uint16_t,
    417 as uint16_t,
    416 as uint16_t,
    414 as uint16_t,
    412 as uint16_t,
    411 as uint16_t,
    410 as uint16_t,
    408 as uint16_t,
    406 as uint16_t,
    405 as uint16_t,
    404 as uint16_t,
    402 as uint16_t,
    400 as uint16_t,
    399 as uint16_t,
    398 as uint16_t,
    396 as uint16_t,
    394 as uint16_t,
    393 as uint16_t,
    392 as uint16_t,
    390 as uint16_t,
    389 as uint16_t,
    387 as uint16_t,
    386 as uint16_t,
    384 as uint16_t,
    383 as uint16_t,
    381 as uint16_t,
    380 as uint16_t,
    378 as uint16_t,
    377 as uint16_t,
    375 as uint16_t,
    374 as uint16_t,
    372 as uint16_t,
    371 as uint16_t,
    370 as uint16_t,
    368 as uint16_t,
    367 as uint16_t,
    365 as uint16_t,
    364 as uint16_t,
    362 as uint16_t,
    361 as uint16_t,
    360 as uint16_t,
    358 as uint16_t,
    357 as uint16_t,
    355 as uint16_t,
    354 as uint16_t,
    352 as uint16_t,
    351 as uint16_t,
    350 as uint16_t,
    348 as uint16_t,
    347 as uint16_t,
    345 as uint16_t,
    344 as uint16_t,
    342 as uint16_t,
    341 as uint16_t,
    340 as uint16_t,
    338 as uint16_t,
    337 as uint16_t,
    336 as uint16_t,
    334 as uint16_t,
    333 as uint16_t,
    331 as uint16_t,
    330 as uint16_t,
    328 as uint16_t,
    327 as uint16_t,
    326 as uint16_t,
    324 as uint16_t,
    323 as uint16_t,
    322 as uint16_t,
    320 as uint16_t,
    319 as uint16_t,
    318 as uint16_t,
    316 as uint16_t,
    315 as uint16_t,
    313 as uint16_t,
    312 as uint16_t,
    311 as uint16_t,
    309 as uint16_t,
    308 as uint16_t,
    306 as uint16_t,
    305 as uint16_t,
    304 as uint16_t,
    302 as uint16_t,
    301 as uint16_t,
    300 as uint16_t,
    298 as uint16_t,
    297 as uint16_t,
    296 as uint16_t,
    294 as uint16_t,
    293 as uint16_t,
    292 as uint16_t,
    290 as uint16_t,
    289 as uint16_t,
    288 as uint16_t,
    286 as uint16_t,
    285 as uint16_t,
    284 as uint16_t,
    282 as uint16_t,
    281 as uint16_t,
    280 as uint16_t,
    278 as uint16_t,
    277 as uint16_t,
    276 as uint16_t,
    274 as uint16_t,
    273 as uint16_t,
    272 as uint16_t,
    270 as uint16_t,
    269 as uint16_t,
    268 as uint16_t,
    267 as uint16_t,
    265 as uint16_t,
    264 as uint16_t,
    263 as uint16_t,
    261 as uint16_t,
    260 as uint16_t,
    259 as uint16_t,
    258 as uint16_t,
    256 as uint16_t,
    255 as uint16_t,
    254 as uint16_t,
    252 as uint16_t,
    251 as uint16_t,
    250 as uint16_t,
    248 as uint16_t,
    247 as uint16_t,
    246 as uint16_t,
    245 as uint16_t,
    243 as uint16_t,
    242 as uint16_t,
    241 as uint16_t,
    240 as uint16_t,
    238 as uint16_t,
    237 as uint16_t,
    236 as uint16_t,
    234 as uint16_t,
    233 as uint16_t,
    232 as uint16_t,
    231 as uint16_t,
    229 as uint16_t,
    228 as uint16_t,
    227 as uint16_t,
    226 as uint16_t,
    224 as uint16_t,
    223 as uint16_t,
    222 as uint16_t,
    221 as uint16_t,
    219 as uint16_t,
    218 as uint16_t,
    217 as uint16_t,
    216 as uint16_t,
    214 as uint16_t,
    213 as uint16_t,
    212 as uint16_t,
    211 as uint16_t,
    210 as uint16_t,
    208 as uint16_t,
    207 as uint16_t,
    206 as uint16_t,
    205 as uint16_t,
    203 as uint16_t,
    202 as uint16_t,
    201 as uint16_t,
    200 as uint16_t,
    198 as uint16_t,
    197 as uint16_t,
    196 as uint16_t,
    195 as uint16_t,
    194 as uint16_t,
    192 as uint16_t,
    191 as uint16_t,
    190 as uint16_t,
    189 as uint16_t,
    188 as uint16_t,
    186 as uint16_t,
    185 as uint16_t,
    184 as uint16_t,
    183 as uint16_t,
    182 as uint16_t,
    180 as uint16_t,
    179 as uint16_t,
    178 as uint16_t,
    177 as uint16_t,
    176 as uint16_t,
    174 as uint16_t,
    173 as uint16_t,
    172 as uint16_t,
    171 as uint16_t,
    170 as uint16_t,
    168 as uint16_t,
    167 as uint16_t,
    166 as uint16_t,
    165 as uint16_t,
    164 as uint16_t,
    162 as uint16_t,
    161 as uint16_t,
    160 as uint16_t,
    159 as uint16_t,
    158 as uint16_t,
    157 as uint16_t,
    156 as uint16_t,
    154 as uint16_t,
    153 as uint16_t,
    152 as uint16_t,
    151 as uint16_t,
    150 as uint16_t,
    148 as uint16_t,
    147 as uint16_t,
    146 as uint16_t,
    145 as uint16_t,
    144 as uint16_t,
    143 as uint16_t,
    142 as uint16_t,
    140 as uint16_t,
    139 as uint16_t,
    138 as uint16_t,
    137 as uint16_t,
    136 as uint16_t,
    135 as uint16_t,
    134 as uint16_t,
    132 as uint16_t,
    131 as uint16_t,
    130 as uint16_t,
    129 as uint16_t,
    128 as uint16_t,
    127 as uint16_t,
    126 as uint16_t,
    124 as uint16_t,
    123 as uint16_t,
    122 as uint16_t,
    121 as uint16_t,
    120 as uint16_t,
    119 as uint16_t,
    118 as uint16_t,
    116 as uint16_t,
    115 as uint16_t,
    114 as uint16_t,
    113 as uint16_t,
    112 as uint16_t,
    111 as uint16_t,
    110 as uint16_t,
    109 as uint16_t,
    108 as uint16_t,
    106 as uint16_t,
    105 as uint16_t,
    104 as uint16_t,
    103 as uint16_t,
    102 as uint16_t,
    101 as uint16_t,
    100 as uint16_t,
    99 as uint16_t,
    98 as uint16_t,
    97 as uint16_t,
    95 as uint16_t,
    94 as uint16_t,
    93 as uint16_t,
    92 as uint16_t,
    91 as uint16_t,
    90 as uint16_t,
    89 as uint16_t,
    88 as uint16_t,
    87 as uint16_t,
    86 as uint16_t,
    84 as uint16_t,
    83 as uint16_t,
    82 as uint16_t,
    81 as uint16_t,
    80 as uint16_t,
    79 as uint16_t,
    78 as uint16_t,
    77 as uint16_t,
    76 as uint16_t,
    75 as uint16_t,
    74 as uint16_t,
    73 as uint16_t,
    72 as uint16_t,
    70 as uint16_t,
    69 as uint16_t,
    68 as uint16_t,
    67 as uint16_t,
    66 as uint16_t,
    65 as uint16_t,
    64 as uint16_t,
    63 as uint16_t,
    62 as uint16_t,
    61 as uint16_t,
    60 as uint16_t,
    59 as uint16_t,
    58 as uint16_t,
    57 as uint16_t,
    56 as uint16_t,
    54 as uint16_t,
    53 as uint16_t,
    52 as uint16_t,
    51 as uint16_t,
    50 as uint16_t,
    49 as uint16_t,
    48 as uint16_t,
    47 as uint16_t,
    46 as uint16_t,
    45 as uint16_t,
    44 as uint16_t,
    43 as uint16_t,
    42 as uint16_t,
    41 as uint16_t,
    40 as uint16_t,
    39 as uint16_t,
    38 as uint16_t,
    37 as uint16_t,
    36 as uint16_t,
    35 as uint16_t,
    34 as uint16_t,
    33 as uint16_t,
    31 as uint16_t,
    30 as uint16_t,
    29 as uint16_t,
    28 as uint16_t,
    27 as uint16_t,
    26 as uint16_t,
    25 as uint16_t,
    24 as uint16_t,
    23 as uint16_t,
    22 as uint16_t,
    21 as uint16_t,
    20 as uint16_t,
    19 as uint16_t,
    18 as uint16_t,
    17 as uint16_t,
    16 as uint16_t,
    15 as uint16_t,
    14 as uint16_t,
    13 as uint16_t,
    12 as uint16_t,
    11 as uint16_t,
    10 as uint16_t,
    9 as uint16_t,
    8 as uint16_t,
    7 as uint16_t,
    6 as uint16_t,
    5 as uint16_t,
    4 as uint16_t,
    3 as uint16_t,
    2 as uint16_t,
    1 as uint16_t,
];
unsafe extern "C" fn _mpd_qln(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut varcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut z: *mut mpd_t = result;
    let mut v_data: [mpd_uint_t; 64] = [0; 64];
    let mut v: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut v_data as *mut mpd_uint_t,
    };
    let mut vtmp_data: [mpd_uint_t; 64] = [0; 64];
    let mut vtmp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut vtmp_data as *mut mpd_uint_t,
    };
    let mut tmp_data: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tmp_data as *mut mpd_uint_t,
    };
    let mut klist: [mpd_ssize_t; 64] = [0; 64];
    let mut maxprec: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut t: mpd_ssize_t = 0;
    let mut a_digits: mpd_ssize_t = 0;
    let mut a_exp: mpd_ssize_t = 0;
    let mut dummy: mpd_uint_t = 0;
    let mut x: mpd_uint_t = 0;
    let mut ret: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if mpd_isspecial(a) == 0 && mpd_iszerocoeff(a) == 0 {
    } else {
        mpd_c2rust_assert_fail(
            b"!mpd_isspecial(a) && !mpd_iszerocoeff(a)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if mpd_qcopy(&raw mut v, a, status) == 0 {
        mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
    } else {
        _mpd_get_msdigits(
            &raw mut dummy,
            &raw mut x,
            &raw mut v,
            3 as ::core::ffi::c_uint,
        );
        if x < 10 as mpd_uint_t {
            x = x.wrapping_mul(10 as mpd_uint_t);
        }
        if x < 100 as mpd_uint_t {
            x = x.wrapping_mul(10 as mpd_uint_t);
        }
        x = x.wrapping_sub(100 as mpd_uint_t);
        a_digits = (*a).digits;
        a_exp = (*a).exp;
        mpd_minalloc(z);
        mpd_clear_flags(z);
        *(*z).data.offset(0 as isize) = lnapprox[x as usize] as mpd_uint_t;
        (*z).len = 1 as mpd_ssize_t;
        (*z).exp = -(3 as ::core::ffi::c_int) as mpd_ssize_t;
        mpd_setdigits(z);
        if x <= 400 as mpd_uint_t {
            v.exp = -(a_digits - 1 as mpd_ssize_t);
            t = a_exp + a_digits - 1 as mpd_ssize_t;
        } else {
            v.exp = -a_digits;
            t = a_exp + a_digits;
            mpd_set_negative(z);
        }
        mpd_maxcontext(&raw mut maxcontext);
        mpd_maxcontext(&raw mut varcontext);
        varcontext.round = MPD_ROUND_TRUNC as ::core::ffi::c_int;
        maxprec = (*ctx).prec + 2 as mpd_ssize_t;
        if t == 0 as mpd_ssize_t && (x <= 15 as mpd_uint_t || x >= 800 as mpd_uint_t) {
            let mut cmp: ::core::ffi::c_int = _mpd_cmp(&raw mut v, &raw const one);
            _mpd_qsub(
                &raw mut tmp,
                &raw mut v,
                &raw const one,
                &raw mut maxcontext,
                &raw mut maxcontext.status,
            );
            if maxcontext.status & MPD_Errors as uint32_t != 0 {
                mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
                current_block = 1;
            } else {
                if cmp < 0 as ::core::ffi::c_int {
                    tmp.exp += 1 as mpd_ssize_t;
                }
                if mpd_adjexp(&raw mut tmp) < mpd_etiny(ctx) {
                    _settriple(
                        result,
                        (cmp < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as uint8_t,
                        1 as mpd_uint_t,
                        mpd_etiny(ctx) - 1 as mpd_ssize_t,
                    );
                    current_block = 1;
                } else {
                    tmp.exp -= 1 as mpd_ssize_t;
                    if mpd_adjexp(&raw mut tmp) < 0 as mpd_ssize_t {
                        maxprec = maxprec - mpd_adjexp(&raw mut tmp);
                    }
                    current_block = 2;
                }
            }
        } else {
            current_block = 2;
        }
        match current_block {
            1 => {}
            _ => {
                i = ln_schedule_prec(
                    &raw mut klist as *mut mpd_ssize_t,
                    maxprec,
                    2 as mpd_ssize_t,
                );
                while i >= 0 as ::core::ffi::c_int {
                    varcontext.prec = 2 as mpd_ssize_t * klist[i as usize] + 3 as mpd_ssize_t;
                    (*z).flags = ((*z).flags as ::core::ffi::c_int ^ MPD_NEG as ::core::ffi::c_int)
                        as uint8_t;
                    _mpd_qexp(&raw mut tmp, z, &raw mut varcontext, status);
                    (*z).flags = ((*z).flags as ::core::ffi::c_int ^ MPD_NEG as ::core::ffi::c_int)
                        as uint8_t;
                    if v.digits > varcontext.prec {
                        shift = v.digits - varcontext.prec;
                        mpd_qshiftr(&raw mut vtmp, &raw mut v, shift, status);
                        vtmp.exp += shift;
                        mpd_qmul(
                            &raw mut tmp,
                            &raw mut vtmp,
                            &raw mut tmp,
                            &raw mut varcontext,
                            status,
                        );
                    } else {
                        mpd_qmul(
                            &raw mut tmp,
                            &raw mut v,
                            &raw mut tmp,
                            &raw mut varcontext,
                            status,
                        );
                    }
                    mpd_qsub(
                        &raw mut tmp,
                        &raw mut tmp,
                        &raw const one,
                        &raw mut maxcontext,
                        status,
                    );
                    mpd_qadd(z, z, &raw mut tmp, &raw mut maxcontext, status);
                    if mpd_isspecial(z) != 0 {
                        break;
                    }
                    i -= 1;
                }
                mpd_qln10(&raw mut v, maxprec + 1 as mpd_ssize_t, status);
                mpd_qmul_ssize(&raw mut tmp, &raw mut v, t, &raw mut maxcontext, status);
                mpd_qadd(result, &raw mut tmp, z, &raw mut maxcontext, status);
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    *status = (*status as ::core::ffi::c_uint | (MPD_Inexact | MPD_Rounded)) as uint32_t;
    mpd_del(&raw mut v);
    mpd_del(&raw mut vtmp);
    mpd_del(&raw mut tmp);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qln(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut adjexp: mpd_ssize_t = 0;
    let mut t: mpd_ssize_t = 0;
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        if mpd_isnegative(a) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        mpd_setspecial(result, MPD_POS, MPD_INF);
        return;
    }
    if mpd_iszerocoeff(a) != 0 {
        mpd_setspecial(result, MPD_NEG, MPD_INF);
        return;
    }
    if mpd_isnegative(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if _mpd_cmp(a, &raw const one) == 0 as ::core::ffi::c_int {
        _settriple(result, MPD_POS, 0 as mpd_uint_t, 0 as mpd_ssize_t);
        return;
    }
    adjexp = mpd_adjexp(a);
    t = if adjexp < 0 as mpd_ssize_t {
        -adjexp - 1 as mpd_ssize_t
    } else {
        adjexp
    };
    t *= 2 as mpd_ssize_t;
    if (mpd_exp_digits(t) - 1 as ::core::ffi::c_int) as mpd_ssize_t > (*ctx).emax {
        *status = (*status as ::core::ffi::c_uint | (MPD_Overflow | MPD_Inexact | MPD_Rounded))
            as uint32_t;
        mpd_setspecial(
            result,
            (adjexp < 0 as mpd_ssize_t) as ::core::ffi::c_int as uint8_t,
            MPD_INF,
        );
        return;
    }
    mpd_workcontext(&raw mut workctx, ctx);
    workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    if (*ctx).allcr != 0 {
        let mut hi_data: [mpd_uint_t; 64] = [0; 64];
        let mut hi: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut hi_data as *mut mpd_uint_t,
        };
        let mut lo_data: [mpd_uint_t; 64] = [0; 64];
        let mut lo: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut lo_data as *mut mpd_uint_t,
        };
        let mut ulp_data: [mpd_uint_t; 64] = [0; 64];
        let mut ulp: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut ulp_data as *mut mpd_uint_t,
        };
        let mut aa_data: [mpd_uint_t; 64] = [0; 64];
        let mut aa: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut aa_data as *mut mpd_uint_t,
        };
        let mut loop_protect: uint32_t = 0 as uint32_t;
        let mut prec: mpd_ssize_t = 0;
        if result == a as *mut mpd_t {
            if mpd_qcopy(&raw mut aa, a, status) == 0 {
                mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
                return;
            }
            a = &raw mut aa;
        }
        workctx.clamp = 0 as ::core::ffi::c_int;
        prec = (*ctx).prec + 3 as mpd_ssize_t;
        loop {
            let mut status_res: uint32_t = 0 as uint32_t;
            let mut status_hi: uint32_t = 0 as uint32_t;
            let mut status_lo: uint32_t = 0 as uint32_t;
            let mut shortcut: ::core::ffi::c_int = 0;
            let mut bounds_eq: ::core::ffi::c_int = 0;
            let mut subnormal_eq: ::core::ffi::c_int = 0;
            workctx.prec = prec;
            shortcut = _mpd_qln(result, a, &raw mut workctx, &raw mut status_res);
            if mpd_isnan(result) != 0 {
                mpd_seterror(result, status_res, status);
                break;
            } else if shortcut != 0 || mpd_isinfinite(result) != 0 || mpd_iszero(result) != 0 {
                *status |= status_res;
                workctx.prec = (*ctx).prec;
                workctx.clamp = (*ctx).clamp;
                mpd_qfinalize(result, &raw mut workctx, status);
                break;
            } else {
                _ssettriple(
                    &raw mut ulp,
                    MPD_POS,
                    1 as mpd_uint_t,
                    (*result).exp + (*result).digits - workctx.prec,
                );
                workctx.prec = (*ctx).prec;
                mpd_qadd(
                    &raw mut hi,
                    result,
                    &raw mut ulp,
                    &raw mut workctx,
                    &raw mut status_hi,
                );
                mpd_qsub(
                    &raw mut lo,
                    result,
                    &raw mut ulp,
                    &raw mut workctx,
                    &raw mut status_lo,
                );
                if mpd_isnan(&raw mut hi) != 0 || mpd_isnan(&raw mut lo) != 0 {
                    mpd_seterror(result, status_hi | status_lo, status);
                    break;
                } else {
                    subnormal_eq = (status_hi & MPD_Subnormal as uint32_t
                        == status_lo & MPD_Subnormal as uint32_t)
                        as ::core::ffi::c_int;
                    bounds_eq = (mpd_qcmp(&raw mut hi, &raw mut lo, status)
                        == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if bounds_eq != 0 && {
                        loop_protect = loop_protect.wrapping_add(1);
                        loop_protect > 5 as uint32_t
                    } {
                        subnormal_eq = 1 as ::core::ffi::c_int;
                    }
                    if subnormal_eq != 0 && bounds_eq != 0 {
                        *status |= status_lo;
                        workctx.clamp = (*ctx).clamp;
                        mpd_qfinalize(result, &raw mut workctx, status);
                        break;
                    } else {
                        if subnormal_eq != 0 {
                            prec += MPD_RDIGITS as mpd_ssize_t;
                        } else {
                            prec *= 2 as mpd_ssize_t;
                        }
                        if !(prec > MPD_MAX_PREC as mpd_ssize_t) {
                            continue;
                        }
                        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
                        break;
                    }
                }
            }
        }
        mpd_del(&raw mut hi);
        mpd_del(&raw mut lo);
        mpd_del(&raw mut ulp);
        mpd_del(&raw mut aa);
    } else {
        _mpd_qln(result, a, &raw mut workctx, status);
        mpd_check_underflow(result, &raw mut workctx, status);
        mpd_qfinalize(result, &raw mut workctx, status);
    };
}
unsafe extern "C" fn _mpd_qlog10(
    mut action: ::core::ffi::c_int,
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut ln10_data: [mpd_uint_t; 64] = [0; 64];
    let mut ln10: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut ln10_data as *mut mpd_uint_t,
    };
    let mut ret: ::core::ffi::c_int = 0;
    mpd_maxcontext(&raw mut workctx);
    workctx.prec = (*ctx).prec + 3 as mpd_ssize_t;
    ret = _mpd_qln(result, a, &raw mut workctx, status);
    mpd_qln10(&raw mut ln10, workctx.prec, status);
    if action == DO_FINALIZE as ::core::ffi::c_int {
        mpd_workcontext(&raw mut workctx, ctx);
        workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    }
    _mpd_qdiv(
        NO_IDEAL_EXP as ::core::ffi::c_int,
        result,
        result,
        &raw mut ln10,
        &raw mut workctx,
        status,
    );
    mpd_del(&raw mut ln10);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qlog10(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut adjexp: mpd_ssize_t = 0;
    let mut t: mpd_ssize_t = 0;
    mpd_workcontext(&raw mut workctx, ctx);
    workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        if mpd_isnegative(a) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        mpd_setspecial(result, MPD_POS, MPD_INF);
        return;
    }
    if mpd_iszerocoeff(a) != 0 {
        mpd_setspecial(result, MPD_NEG, MPD_INF);
        return;
    }
    if mpd_isnegative(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_coeff_ispow10(a) != 0 {
        let mut sign: uint8_t = 0 as uint8_t;
        adjexp = mpd_adjexp(a);
        if adjexp < 0 as mpd_ssize_t {
            sign = 1 as uint8_t;
            adjexp = -adjexp;
        }
        _settriple(result, sign, adjexp as mpd_uint_t, 0 as mpd_ssize_t);
        mpd_qfinalize(result, &raw mut workctx, status);
        return;
    }
    adjexp = mpd_adjexp(a);
    t = if adjexp < 0 as mpd_ssize_t {
        -adjexp - 1 as mpd_ssize_t
    } else {
        adjexp
    };
    if (mpd_exp_digits(t) - 1 as ::core::ffi::c_int) as mpd_ssize_t > (*ctx).emax {
        *status = (*status as ::core::ffi::c_uint | (MPD_Overflow | MPD_Inexact | MPD_Rounded))
            as uint32_t;
        mpd_setspecial(
            result,
            (adjexp < 0 as mpd_ssize_t) as ::core::ffi::c_int as uint8_t,
            MPD_INF,
        );
        return;
    }
    if (*ctx).allcr != 0 {
        let mut hi_data: [mpd_uint_t; 64] = [0; 64];
        let mut hi: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut hi_data as *mut mpd_uint_t,
        };
        let mut lo_data: [mpd_uint_t; 64] = [0; 64];
        let mut lo: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut lo_data as *mut mpd_uint_t,
        };
        let mut ulp_data: [mpd_uint_t; 64] = [0; 64];
        let mut ulp: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut ulp_data as *mut mpd_uint_t,
        };
        let mut aa_data: [mpd_uint_t; 64] = [0; 64];
        let mut aa: mpd_t = mpd_t {
            flags: (0 as ::core::ffi::c_int
                | MPD_STATIC as ::core::ffi::c_int
                | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 0 as mpd_ssize_t,
            len: 0 as mpd_ssize_t,
            alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
            data: &raw mut aa_data as *mut mpd_uint_t,
        };
        let mut loop_protect: uint32_t = 0 as uint32_t;
        let mut prec: mpd_ssize_t = 0;
        if result == a as *mut mpd_t {
            if mpd_qcopy(&raw mut aa, a, status) == 0 {
                mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
                return;
            }
            a = &raw mut aa;
        }
        workctx.clamp = 0 as ::core::ffi::c_int;
        prec = (*ctx).prec + 3 as mpd_ssize_t;
        loop {
            let mut status_res: uint32_t = 0 as uint32_t;
            let mut status_hi: uint32_t = 0 as uint32_t;
            let mut status_lo: uint32_t = 0 as uint32_t;
            let mut shortcut: ::core::ffi::c_int = 0;
            let mut bounds_eq: ::core::ffi::c_int = 0;
            let mut subnormal_eq: ::core::ffi::c_int = 0;
            workctx.prec = prec;
            shortcut = _mpd_qlog10(
                SKIP_FINALIZE as ::core::ffi::c_int,
                result,
                a,
                &raw mut workctx,
                &raw mut status_res,
            );
            if mpd_isnan(result) != 0 {
                mpd_seterror(result, status_res, status);
                break;
            } else if shortcut != 0 || mpd_isinfinite(result) != 0 || mpd_iszero(result) != 0 {
                *status |= status_res;
                workctx.prec = (*ctx).prec;
                workctx.clamp = (*ctx).clamp;
                mpd_qfinalize(result, &raw mut workctx, status);
                break;
            } else {
                _ssettriple(
                    &raw mut ulp,
                    MPD_POS,
                    1 as mpd_uint_t,
                    (*result).exp + (*result).digits - workctx.prec,
                );
                workctx.prec = (*ctx).prec;
                mpd_qadd(
                    &raw mut hi,
                    result,
                    &raw mut ulp,
                    &raw mut workctx,
                    &raw mut status_hi,
                );
                mpd_qsub(
                    &raw mut lo,
                    result,
                    &raw mut ulp,
                    &raw mut workctx,
                    &raw mut status_lo,
                );
                if mpd_isnan(&raw mut hi) != 0 || mpd_isnan(&raw mut lo) != 0 {
                    mpd_seterror(result, status_hi | status_lo, status);
                    break;
                } else {
                    subnormal_eq = (status_hi & MPD_Subnormal as uint32_t
                        == status_lo & MPD_Subnormal as uint32_t)
                        as ::core::ffi::c_int;
                    bounds_eq = (mpd_qcmp(&raw mut hi, &raw mut lo, status)
                        == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if bounds_eq != 0 && {
                        loop_protect = loop_protect.wrapping_add(1);
                        loop_protect > 5 as uint32_t
                    } {
                        subnormal_eq = 1 as ::core::ffi::c_int;
                    }
                    if subnormal_eq != 0 && bounds_eq != 0 {
                        *status |= status_lo;
                        workctx.clamp = (*ctx).clamp;
                        mpd_qfinalize(result, &raw mut workctx, status);
                        break;
                    } else {
                        if subnormal_eq != 0 {
                            prec += MPD_RDIGITS as mpd_ssize_t;
                        } else {
                            prec *= 2 as mpd_ssize_t;
                        }
                        if !(prec > MPD_MAX_PREC as mpd_ssize_t) {
                            continue;
                        }
                        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
                        break;
                    }
                }
            }
        }
        mpd_del(&raw mut hi);
        mpd_del(&raw mut lo);
        mpd_del(&raw mut ulp);
        mpd_del(&raw mut aa);
    } else {
        _mpd_qlog10(
            DO_FINALIZE as ::core::ffi::c_int,
            result,
            a,
            &raw mut workctx,
            status,
        );
        mpd_check_underflow(result, &raw mut workctx, status);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmax(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut c: ::core::ffi::c_int = 0;
    if mpd_isqnan(a) != 0 && mpd_isnan(b) == 0 {
        mpd_qcopy(result, b, status);
    } else if mpd_isqnan(b) != 0 && mpd_isnan(a) == 0 {
        mpd_qcopy(result, a, status);
    } else if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
        return;
    } else {
        c = _mpd_cmp(a, b);
        if c == 0 as ::core::ffi::c_int {
            c = _mpd_cmp_numequal(a, b);
        }
        if c < 0 as ::core::ffi::c_int {
            mpd_qcopy(result, b, status);
        } else {
            mpd_qcopy(result, a, status);
        }
    }
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmax_mag(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut c: ::core::ffi::c_int = 0;
    if mpd_isqnan(a) != 0 && mpd_isnan(b) == 0 {
        mpd_qcopy(result, b, status);
    } else if mpd_isqnan(b) != 0 && mpd_isnan(a) == 0 {
        mpd_qcopy(result, a, status);
    } else if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
        return;
    } else {
        c = _mpd_cmp_abs(a, b);
        if c == 0 as ::core::ffi::c_int {
            c = _mpd_cmp_numequal(a, b);
        }
        if c < 0 as ::core::ffi::c_int {
            mpd_qcopy(result, b, status);
        } else {
            mpd_qcopy(result, a, status);
        }
    }
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmin(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut c: ::core::ffi::c_int = 0;
    if mpd_isqnan(a) != 0 && mpd_isnan(b) == 0 {
        mpd_qcopy(result, b, status);
    } else if mpd_isqnan(b) != 0 && mpd_isnan(a) == 0 {
        mpd_qcopy(result, a, status);
    } else if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
        return;
    } else {
        c = _mpd_cmp(a, b);
        if c == 0 as ::core::ffi::c_int {
            c = _mpd_cmp_numequal(a, b);
        }
        if c < 0 as ::core::ffi::c_int {
            mpd_qcopy(result, a, status);
        } else {
            mpd_qcopy(result, b, status);
        }
    }
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmin_mag(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut c: ::core::ffi::c_int = 0;
    if mpd_isqnan(a) != 0 && mpd_isnan(b) == 0 {
        mpd_qcopy(result, b, status);
    } else if mpd_isqnan(b) != 0 && mpd_isnan(a) == 0 {
        mpd_qcopy(result, a, status);
    } else if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
        return;
    } else {
        c = _mpd_cmp_abs(a, b);
        if c == 0 as ::core::ffi::c_int {
            c = _mpd_cmp_numequal(a, b);
        }
        if c < 0 as ::core::ffi::c_int {
            mpd_qcopy(result, a, status);
        } else {
            mpd_qcopy(result, b, status);
        }
    }
    mpd_qfinalize(result, ctx, status);
}
unsafe extern "C" fn _kmul_resultsize(mut la: mpd_size_t, mut lb: mpd_size_t) -> mpd_size_t {
    let mut n: mpd_size_t = 0;
    let mut m: mpd_size_t = 0;
    n = add_size_t(la, lb);
    n = add_size_t(n, 1 as mpd_size_t);
    m = la
        .wrapping_add(1 as mpd_size_t)
        .wrapping_div(2 as mpd_size_t)
        .wrapping_add(1 as mpd_size_t);
    m = mul_size_t(m, 3 as mpd_size_t);
    return if m > n { m } else { n };
}
unsafe extern "C" fn _kmul_worksize(mut n: mpd_size_t, mut lim: mpd_size_t) -> mpd_size_t {
    let mut m: mpd_size_t = 0;
    if n <= lim {
        return 0 as mpd_size_t;
    }
    m = n
        .wrapping_add(1 as mpd_size_t)
        .wrapping_div(2 as mpd_size_t)
        .wrapping_add(1 as mpd_size_t);
    return add_size_t(mul_size_t(m, 2 as mpd_size_t), _kmul_worksize(m, lim));
}
pub const MPD_KARATSUBA_BASECASE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
unsafe extern "C" fn _karatsuba_rec(
    mut c: *mut mpd_uint_t,
    mut a: *const mpd_uint_t,
    mut b: *const mpd_uint_t,
    mut w: *mut mpd_uint_t,
    mut la: mpd_size_t,
    mut lb: mpd_size_t,
) {
    let mut m: mpd_size_t = 0;
    let mut lt: mpd_size_t = 0;
    if la >= lb && lb > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"la >= lb && lb > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if la <= 16 as mpd_size_t || !w.is_null() {
    } else {
        mpd_c2rust_assert_fail(
            b"la <= MPD_KARATSUBA_BASECASE || w != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if la <= MPD_KARATSUBA_BASECASE as mpd_size_t {
        _mpd_basemul(c, a, b, la, lb);
        return;
    }
    m = la
        .wrapping_add(1 as mpd_size_t)
        .wrapping_div(2 as mpd_size_t);
    if lb <= m {
        if lb > la.wrapping_sub(m) {
            lt = lb.wrapping_add(lb).wrapping_add(1 as mpd_size_t);
            mpd_uint_zero(w, lt);
            _karatsuba_rec(
                w,
                b,
                a.offset(m as isize),
                w.offset(lt as isize),
                lb,
                la.wrapping_sub(m),
            );
        } else {
            lt = la
                .wrapping_sub(m)
                .wrapping_add(la.wrapping_sub(m))
                .wrapping_add(1 as mpd_size_t);
            mpd_uint_zero(w, lt);
            _karatsuba_rec(
                w,
                a.offset(m as isize),
                b,
                w.offset(lt as isize),
                la.wrapping_sub(m),
                lb,
            );
        }
        _mpd_baseaddto(c.offset(m as isize), w, la.wrapping_sub(m).wrapping_add(lb));
        lt = m.wrapping_add(m).wrapping_add(1 as mpd_size_t);
        mpd_uint_zero(w, lt);
        _karatsuba_rec(w, a, b, w.offset(lt as isize), m, lb);
        _mpd_baseaddto(c, w, m.wrapping_add(lb));
        return;
    }
    memcpy(
        w as *mut ::core::ffi::c_void,
        a as *const ::core::ffi::c_void,
        (m as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
    );
    *w.offset(m as isize) = 0 as mpd_uint_t;
    _mpd_baseaddto(w, a.offset(m as isize), la.wrapping_sub(m));
    memcpy(
        w.offset(m.wrapping_add(1 as mpd_size_t) as isize) as *mut ::core::ffi::c_void,
        b as *const ::core::ffi::c_void,
        (m as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
    );
    *w.offset(m.wrapping_add(1 as mpd_size_t).wrapping_add(m) as isize) = 0 as mpd_uint_t;
    _mpd_baseaddto(
        w.offset(m.wrapping_add(1 as mpd_size_t) as isize),
        b.offset(m as isize),
        lb.wrapping_sub(m),
    );
    _karatsuba_rec(
        c.offset(m as isize),
        w,
        w.offset(m.wrapping_add(1 as mpd_size_t) as isize),
        w.offset((2 as mpd_size_t).wrapping_mul(m.wrapping_add(1 as mpd_size_t)) as isize),
        m.wrapping_add(1 as mpd_size_t),
        m.wrapping_add(1 as mpd_size_t),
    );
    lt = la
        .wrapping_sub(m)
        .wrapping_add(la.wrapping_sub(m))
        .wrapping_add(1 as mpd_size_t);
    mpd_uint_zero(w, lt);
    _karatsuba_rec(
        w,
        a.offset(m as isize),
        b.offset(m as isize),
        w.offset(lt as isize),
        la.wrapping_sub(m),
        lb.wrapping_sub(m),
    );
    _mpd_baseaddto(
        c.offset((2 as mpd_size_t).wrapping_mul(m) as isize),
        w,
        la.wrapping_sub(m).wrapping_add(lb.wrapping_sub(m)),
    );
    _mpd_basesubfrom(
        c.offset(m as isize),
        w,
        la.wrapping_sub(m).wrapping_add(lb.wrapping_sub(m)),
    );
    lt = m.wrapping_add(m).wrapping_add(1 as mpd_size_t);
    mpd_uint_zero(w, lt);
    _karatsuba_rec(w, a, b, w.offset(lt as isize), m, m);
    _mpd_baseaddto(c, w, m.wrapping_add(m));
    _mpd_basesubfrom(c.offset(m as isize), w, m.wrapping_add(m));
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_kmul(
    mut u: *const mpd_uint_t,
    mut v: *const mpd_uint_t,
    mut ulen: mpd_size_t,
    mut vlen: mpd_size_t,
    mut rsize: *mut mpd_size_t,
) -> *mut mpd_uint_t {
    let mut result: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut w: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut m: mpd_size_t = 0;
    if ulen >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"ulen >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if ulen >= vlen {
    } else {
        mpd_c2rust_assert_fail(b"ulen >= vlen\0" as *const u8 as *const ::core::ffi::c_char);
    };
    *rsize = _kmul_resultsize(ulen, vlen);
    result =
        mpd_calloc(*rsize, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t) as *mut mpd_uint_t;
    if result.is_null() {
        return ::core::ptr::null_mut::<mpd_uint_t>();
    }
    m = _kmul_worksize(ulen, MPD_KARATSUBA_BASECASE as mpd_size_t);
    if m != 0 && {
        w = mpd_calloc(m, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t) as *mut mpd_uint_t;
        w.is_null()
    } {
        mpd_free.expect("non-null function pointer")(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<mpd_uint_t>();
    }
    _karatsuba_rec(result, u, v, w, ulen, vlen);
    if !w.is_null() {
        mpd_free.expect("non-null function pointer")(w as *mut ::core::ffi::c_void);
    }
    return result;
}
unsafe extern "C" fn _mpd_get_transform_len(mut rsize: mpd_size_t) -> mpd_size_t {
    let mut log2rsize: mpd_size_t = 0;
    let mut x: mpd_size_t = 0;
    let mut step: mpd_size_t = 0;
    if rsize >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"rsize >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    log2rsize = mpd_bsr(rsize) as mpd_size_t;
    if rsize <= 1024 as mpd_size_t {
        x = (1 as mpd_size_t) << log2rsize;
        return if rsize == x {
            x
        } else {
            x << 1 as ::core::ffi::c_int
        };
    } else if rsize as u64 <= MPD_MAXTRANSFORM_2N {
        x = (1 as mpd_size_t) << log2rsize;
        if rsize == x {
            return x;
        }
        step = x >> 1 as ::core::ffi::c_int;
        x = x.wrapping_add(step);
        return if rsize <= x { x } else { x.wrapping_add(step) };
    } else if rsize as u64
        <= MPD_MAXTRANSFORM_2N.wrapping_add(MPD_MAXTRANSFORM_2N.wrapping_div(2 as u64))
    {
        return MPD_MAXTRANSFORM_2N.wrapping_add(MPD_MAXTRANSFORM_2N.wrapping_div(2 as u64))
            as mpd_size_t;
    } else if rsize as u64 <= (3 as u64).wrapping_mul(MPD_MAXTRANSFORM_2N) {
        return (3 as u64).wrapping_mul(MPD_MAXTRANSFORM_2N) as mpd_size_t;
    } else {
        return MPD_SIZE_MAX as mpd_size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_fntmul(
    mut u: *const mpd_uint_t,
    mut v: *const mpd_uint_t,
    mut ulen: mpd_size_t,
    mut vlen: mpd_size_t,
    mut rsize: *mut mpd_size_t,
) -> *mut mpd_uint_t {
    let mut current_block: u64;
    let mut c1: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut c2: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut c3: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut vtmp: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut n: mpd_size_t = 0;
    *rsize = add_size_t(ulen, vlen);
    n = _mpd_get_transform_len(*rsize);
    if n == MPD_SIZE_MAX as mpd_size_t {
        current_block = 1;
    } else {
        c1 = mpd_calloc(n, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t) as *mut mpd_uint_t;
        if c1.is_null() {
            current_block = 1;
        } else {
            c2 = mpd_calloc(n, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t)
                as *mut mpd_uint_t;
            if c2.is_null() {
                current_block = 1;
            } else {
                c3 = mpd_calloc(n, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t)
                    as *mut mpd_uint_t;
                if c3.is_null() {
                    current_block = 1;
                } else {
                    memcpy(
                        c1 as *mut ::core::ffi::c_void,
                        u as *const ::core::ffi::c_void,
                        (ulen as size_t)
                            .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    memcpy(
                        c2 as *mut ::core::ffi::c_void,
                        u as *const ::core::ffi::c_void,
                        (ulen as size_t)
                            .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    memcpy(
                        c3 as *mut ::core::ffi::c_void,
                        u as *const ::core::ffi::c_void,
                        (ulen as size_t)
                            .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    if u == v {
                        if fnt_autoconvolute(c1, n, P1 as ::core::ffi::c_int) == 0
                            || fnt_autoconvolute(c2, n, P2 as ::core::ffi::c_int) == 0
                            || fnt_autoconvolute(c3, n, P3 as ::core::ffi::c_int) == 0
                        {
                            current_block = 1;
                        } else {
                            current_block = 2;
                        }
                    } else {
                        vtmp = mpd_calloc(n, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t)
                            as *mut mpd_uint_t;
                        if vtmp.is_null() {
                            current_block = 1;
                        } else {
                            memcpy(
                                vtmp as *mut ::core::ffi::c_void,
                                v as *const ::core::ffi::c_void,
                                (vlen as size_t)
                                    .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                            );
                            if fnt_convolute(c1, vtmp, n, P1 as ::core::ffi::c_int) == 0 {
                                mpd_free.expect("non-null function pointer")(
                                    vtmp as *mut ::core::ffi::c_void,
                                );
                                current_block = 1;
                            } else {
                                memcpy(
                                    vtmp as *mut ::core::ffi::c_void,
                                    v as *const ::core::ffi::c_void,
                                    (vlen as size_t).wrapping_mul(
                                        ::core::mem::size_of::<mpd_uint_t>() as size_t,
                                    ),
                                );
                                mpd_uint_zero(vtmp.offset(vlen as isize), n.wrapping_sub(vlen));
                                if fnt_convolute(c2, vtmp, n, P2 as ::core::ffi::c_int) == 0 {
                                    mpd_free.expect("non-null function pointer")(
                                        vtmp as *mut ::core::ffi::c_void,
                                    );
                                    current_block = 1;
                                } else {
                                    memcpy(
                                        vtmp as *mut ::core::ffi::c_void,
                                        v as *const ::core::ffi::c_void,
                                        (vlen as size_t).wrapping_mul(::core::mem::size_of::<
                                            mpd_uint_t,
                                        >(
                                        )
                                            as size_t),
                                    );
                                    mpd_uint_zero(vtmp.offset(vlen as isize), n.wrapping_sub(vlen));
                                    if fnt_convolute(c3, vtmp, n, P3 as ::core::ffi::c_int) == 0 {
                                        mpd_free.expect("non-null function pointer")(
                                            vtmp as *mut ::core::ffi::c_void,
                                        );
                                        current_block = 1;
                                    } else {
                                        mpd_free.expect("non-null function pointer")(
                                            vtmp as *mut ::core::ffi::c_void,
                                        );
                                        current_block = 2;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            crt3(c1, c2, c3, *rsize);
                            current_block = 3;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        1 => {
            if !c1.is_null() {
                mpd_free.expect("non-null function pointer")(c1 as *mut ::core::ffi::c_void);
            }
            c1 = ::core::ptr::null_mut::<mpd_uint_t>();
        }
        _ => {}
    }
    if !c2.is_null() {
        mpd_free.expect("non-null function pointer")(c2 as *mut ::core::ffi::c_void);
    }
    if !c3.is_null() {
        mpd_free.expect("non-null function pointer")(c3 as *mut ::core::ffi::c_void);
    }
    return c1;
}
unsafe extern "C" fn _karatsuba_rec_fnt(
    mut c: *mut mpd_uint_t,
    mut a: *const mpd_uint_t,
    mut b: *const mpd_uint_t,
    mut w: *mut mpd_uint_t,
    mut la: mpd_size_t,
    mut lb: mpd_size_t,
) -> ::core::ffi::c_int {
    let mut m: mpd_size_t = 0;
    let mut lt: mpd_size_t = 0;
    if la >= lb && lb > 0 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"la >= lb && lb > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if la as u64 <= (3 as u64).wrapping_mul((4294967296 as u64).wrapping_div(2 as u64))
        || !w.is_null()
    {
    } else {
        mpd_c2rust_assert_fail(
            b"la <= 3*(MPD_MAXTRANSFORM_2N/2) || w != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if la as u64 <= (3 as u64).wrapping_mul(MPD_MAXTRANSFORM_2N.wrapping_div(2 as u64)) {
        if lb <= 192 as mpd_size_t {
            _mpd_basemul(c, b, a, lb, la);
        } else {
            let mut result: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
            let mut dummy: mpd_size_t = 0;
            result = _mpd_fntmul(a, b, la, lb, &raw mut dummy);
            if result.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            memcpy(
                c as *mut ::core::ffi::c_void,
                result as *const ::core::ffi::c_void,
                (la as size_t)
                    .wrapping_add(lb as size_t)
                    .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
            );
            mpd_free.expect("non-null function pointer")(result as *mut ::core::ffi::c_void);
        }
        return 1 as ::core::ffi::c_int;
    }
    m = la
        .wrapping_add(1 as mpd_size_t)
        .wrapping_div(2 as mpd_size_t);
    if lb <= m {
        if lb > la.wrapping_sub(m) {
            lt = lb.wrapping_add(lb).wrapping_add(1 as mpd_size_t);
            mpd_uint_zero(w, lt);
            if _karatsuba_rec_fnt(
                w,
                b,
                a.offset(m as isize),
                w.offset(lt as isize),
                lb,
                la.wrapping_sub(m),
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        } else {
            lt = la
                .wrapping_sub(m)
                .wrapping_add(la.wrapping_sub(m))
                .wrapping_add(1 as mpd_size_t);
            mpd_uint_zero(w, lt);
            if _karatsuba_rec_fnt(
                w,
                a.offset(m as isize),
                b,
                w.offset(lt as isize),
                la.wrapping_sub(m),
                lb,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        _mpd_baseaddto(c.offset(m as isize), w, la.wrapping_sub(m).wrapping_add(lb));
        lt = m.wrapping_add(m).wrapping_add(1 as mpd_size_t);
        mpd_uint_zero(w, lt);
        if _karatsuba_rec_fnt(w, a, b, w.offset(lt as isize), m, lb) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        _mpd_baseaddto(c, w, m.wrapping_add(lb));
        return 1 as ::core::ffi::c_int;
    }
    memcpy(
        w as *mut ::core::ffi::c_void,
        a as *const ::core::ffi::c_void,
        (m as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
    );
    *w.offset(m as isize) = 0 as mpd_uint_t;
    _mpd_baseaddto(w, a.offset(m as isize), la.wrapping_sub(m));
    memcpy(
        w.offset(m.wrapping_add(1 as mpd_size_t) as isize) as *mut ::core::ffi::c_void,
        b as *const ::core::ffi::c_void,
        (m as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
    );
    *w.offset(m.wrapping_add(1 as mpd_size_t).wrapping_add(m) as isize) = 0 as mpd_uint_t;
    _mpd_baseaddto(
        w.offset(m.wrapping_add(1 as mpd_size_t) as isize),
        b.offset(m as isize),
        lb.wrapping_sub(m),
    );
    if _karatsuba_rec_fnt(
        c.offset(m as isize),
        w,
        w.offset(m.wrapping_add(1 as mpd_size_t) as isize),
        w.offset((2 as mpd_size_t).wrapping_mul(m.wrapping_add(1 as mpd_size_t)) as isize),
        m.wrapping_add(1 as mpd_size_t),
        m.wrapping_add(1 as mpd_size_t),
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    lt = la
        .wrapping_sub(m)
        .wrapping_add(la.wrapping_sub(m))
        .wrapping_add(1 as mpd_size_t);
    mpd_uint_zero(w, lt);
    if _karatsuba_rec_fnt(
        w,
        a.offset(m as isize),
        b.offset(m as isize),
        w.offset(lt as isize),
        la.wrapping_sub(m),
        lb.wrapping_sub(m),
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    _mpd_baseaddto(
        c.offset((2 as mpd_size_t).wrapping_mul(m) as isize),
        w,
        la.wrapping_sub(m).wrapping_add(lb.wrapping_sub(m)),
    );
    _mpd_basesubfrom(
        c.offset(m as isize),
        w,
        la.wrapping_sub(m).wrapping_add(lb.wrapping_sub(m)),
    );
    lt = m.wrapping_add(m).wrapping_add(1 as mpd_size_t);
    mpd_uint_zero(w, lt);
    if _karatsuba_rec_fnt(w, a, b, w.offset(lt as isize), m, m) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    _mpd_baseaddto(c, w, m.wrapping_add(m));
    _mpd_basesubfrom(c.offset(m as isize), w, m.wrapping_add(m));
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_kmul_fnt(
    mut u: *const mpd_uint_t,
    mut v: *const mpd_uint_t,
    mut ulen: mpd_size_t,
    mut vlen: mpd_size_t,
    mut rsize: *mut mpd_size_t,
) -> *mut mpd_uint_t {
    let mut result: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut w: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut m: mpd_size_t = 0;
    if ulen >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"ulen >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if ulen >= vlen {
    } else {
        mpd_c2rust_assert_fail(b"ulen >= vlen\0" as *const u8 as *const ::core::ffi::c_char);
    };
    *rsize = _kmul_resultsize(ulen, vlen);
    result =
        mpd_calloc(*rsize, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t) as *mut mpd_uint_t;
    if result.is_null() {
        return ::core::ptr::null_mut::<mpd_uint_t>();
    }
    m = _kmul_worksize(
        ulen,
        (3 as u64).wrapping_mul(MPD_MAXTRANSFORM_2N.wrapping_div(2 as u64)) as mpd_size_t,
    );
    if m != 0 && {
        w = mpd_calloc(m, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t) as *mut mpd_uint_t;
        w.is_null()
    } {
        mpd_free.expect("non-null function pointer")(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<mpd_uint_t>();
    }
    if _karatsuba_rec_fnt(result, u, v, w, ulen, vlen) == 0 {
        mpd_free.expect("non-null function pointer")(result as *mut ::core::ffi::c_void);
        result = ::core::ptr::null_mut::<mpd_uint_t>();
    }
    if !w.is_null() {
        mpd_free.expect("non-null function pointer")(w as *mut ::core::ffi::c_void);
    }
    return result;
}
unsafe extern "C" fn _mpd_qmul_inf(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut status: *mut uint32_t,
) {
    if mpd_isinfinite(a) != 0 {
        if mpd_iszero(b) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        } else {
            mpd_setspecial(
                result,
                (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t,
                MPD_INF,
            );
        }
        return;
    }
    if mpd_isinfinite(b) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"mpd_isinfinite(b)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszero(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
    } else {
        mpd_setspecial(
            result,
            (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t,
            MPD_INF,
        );
    };
}
unsafe extern "C" fn _mpd_qmul(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut big: *const mpd_t = a;
    let mut small: *const mpd_t = b;
    let mut rdata: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut rbuf: [mpd_uint_t; 64] = [0; 64];
    let mut rsize: mpd_size_t = 0;
    let mut i: mpd_size_t = 0;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return;
        }
        _mpd_qmul_inf(result, a, b, status);
        return;
    }
    if (*small).len > (*big).len {
        _mpd_ptrswap(&raw mut big, &raw mut small);
    }
    rsize = ((*big).len + (*small).len) as mpd_size_t;
    if (*big).len == 1 as mpd_ssize_t {
        _mpd_singlemul(
            (*result).data as *mut mpd_uint_t,
            *(*big).data.offset(0 as isize),
            *(*small).data.offset(0 as isize),
        );
    } else if rsize <= MPD_MINALLOC_MAX as mpd_size_t {
        if (*big).len == 2 as mpd_ssize_t {
            _mpd_mul_2_le2(
                &raw mut rbuf as *mut mpd_uint_t,
                (*big).data as *mut mpd_uint_t,
                (*small).data as *mut mpd_uint_t,
                (*small).len,
            );
        } else {
            mpd_uint_zero(&raw mut rbuf as *mut mpd_uint_t, rsize);
            if (*small).len == 1 as mpd_ssize_t {
                _mpd_shortmul(
                    &raw mut rbuf as *mut mpd_uint_t,
                    (*big).data,
                    (*big).len as mpd_size_t,
                    *(*small).data.offset(0 as isize),
                );
            } else {
                _mpd_basemul(
                    &raw mut rbuf as *mut mpd_uint_t,
                    (*small).data,
                    (*big).data,
                    (*small).len as mpd_size_t,
                    (*big).len as mpd_size_t,
                );
            }
        }
        if mpd_qresize(result, rsize as mpd_ssize_t, status) == 0 {
            return;
        }
        i = 0 as mpd_size_t;
        while i < rsize {
            *(*result).data.offset(i as isize) = rbuf[i as usize];
            i = i.wrapping_add(1);
        }
    } else {
        if (*small).len <= 256 as mpd_ssize_t {
            rdata = mpd_calloc(rsize, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t)
                as *mut mpd_uint_t;
            if !rdata.is_null() {
                if (*small).len == 1 as mpd_ssize_t {
                    _mpd_shortmul(
                        rdata,
                        (*big).data,
                        (*big).len as mpd_size_t,
                        *(*small).data.offset(0 as isize),
                    );
                } else {
                    _mpd_basemul(
                        rdata,
                        (*small).data,
                        (*big).data,
                        (*small).len as mpd_size_t,
                        (*big).len as mpd_size_t,
                    );
                }
            }
        } else if rsize <= 1024 as mpd_size_t {
            rdata = _mpd_kmul(
                (*big).data,
                (*small).data,
                (*big).len as mpd_size_t,
                (*small).len as mpd_size_t,
                &raw mut rsize,
            );
        } else if rsize as u64 <= (3 as u64).wrapping_mul(MPD_MAXTRANSFORM_2N) {
            rdata = _mpd_fntmul(
                (*big).data,
                (*small).data,
                (*big).len as mpd_size_t,
                (*small).len as mpd_size_t,
                &raw mut rsize,
            );
        } else {
            rdata = _mpd_kmul_fnt(
                (*big).data,
                (*small).data,
                (*big).len as mpd_size_t,
                (*small).len as mpd_size_t,
                &raw mut rsize,
            );
        }
        if rdata.is_null() {
            mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
            return;
        }
        if mpd_isdynamic_data(result) != 0 {
            mpd_free.expect("non-null function pointer")(
                (*result).data as *mut ::core::ffi::c_void,
            );
        }
        (*result).data = rdata;
        (*result).alloc = rsize as mpd_ssize_t;
        mpd_set_dynamic_data(result);
    }
    mpd_set_flags(
        result,
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t,
    );
    (*result).exp = (*big).exp + (*small).exp;
    (*result).len = _mpd_real_size((*result).data, rsize as mpd_ssize_t);
    mpd_qresize(result, (*result).len, status);
    mpd_setdigits(result);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmul(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _mpd_qmul(result, a, b, ctx, status);
    mpd_qfinalize(result, ctx, status);
}
unsafe extern "C" fn _mpd_qmul_exact(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    mpd_qmul(result, a, b, ctx, &raw mut workstatus);
    *status |= workstatus;
    func_count += 1;
    if func_count == func_fail {
        workstatus = (workstatus as ::core::ffi::c_uint | MPD_Inexact) as uint32_t;
    }
    if workstatus & (MPD_Inexact as uint32_t | MPD_Rounded as uint32_t | MPD_Clamped as uint32_t)
        != 0
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmul_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_ssize(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qmul(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmul_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut bb_data: [mpd_uint_t; 64] = [0; 64];
    let mut bb: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut bb_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qsset_uint(&raw mut bb, b, &raw mut maxcontext, status);
    mpd_qmul(result, a, &raw mut bb, ctx, status);
    mpd_del(&raw mut bb);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmul_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qmul_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmul_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qmul_uint(result, a, b as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmul_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qmul_ssize(result, a, b as mpd_ssize_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qmul_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    mpd_qmul_uint(result, a, b as mpd_uint_t, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qminus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
    }
    if mpd_iszero(a) != 0 && (*ctx).round != MPD_ROUND_FLOOR as ::core::ffi::c_int {
        mpd_qcopy_abs(result, a, status);
    } else {
        mpd_qcopy_negate(result, a, status);
    }
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qplus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
    }
    if mpd_iszero(a) != 0 && (*ctx).round != MPD_ROUND_FLOOR as ::core::ffi::c_int {
        mpd_qcopy_abs(result, a, status);
    } else {
        mpd_qcopy(result, a, status);
    }
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qnext_minus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut tiny_data: [mpd_uint_t; 1] = [1 as mpd_uint_t];
    let mut tiny: mpd_t = mpd_t {
        flags: (0 as uint8_t as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: mpd_etiny(ctx) - 1 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut tiny_data as *mut mpd_uint_t,
    };
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 {
        } else {
            mpd_c2rust_assert_fail(
                b"mpd_isinfinite(a)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        };
        if mpd_isnegative(a) != 0 {
            mpd_qcopy(result, a, status);
            return;
        } else {
            mpd_clear_flags(result);
            mpd_qmaxcoeff(result, ctx, status);
            if mpd_isnan(result) != 0 {
                return;
            }
            (*result).exp = mpd_etop(ctx);
            return;
        }
    }
    mpd_workcontext(&raw mut workctx, ctx);
    workctx.round = MPD_ROUND_FLOOR as ::core::ffi::c_int;
    if mpd_qcopy(result, a, status) == 0 {
        return;
    }
    mpd_qfinalize(result, &raw mut workctx, &raw mut workctx.status);
    if workctx.status & (MPD_Inexact as uint32_t | MPD_Errors as uint32_t) != 0 {
        *status = (*status as ::core::ffi::c_uint
            | (workctx.status & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
            as uint32_t;
        return;
    }
    workctx.status = 0 as uint32_t;
    mpd_qsub(
        result,
        a,
        &raw mut tiny,
        &raw mut workctx,
        &raw mut workctx.status,
    );
    *status = (*status as ::core::ffi::c_uint
        | (workctx.status & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qnext_plus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut tiny_data: [mpd_uint_t; 1] = [1 as mpd_uint_t];
    let mut tiny: mpd_t = mpd_t {
        flags: (0 as uint8_t as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: mpd_etiny(ctx) - 1 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut tiny_data as *mut mpd_uint_t,
    };
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 {
        } else {
            mpd_c2rust_assert_fail(
                b"mpd_isinfinite(a)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        };
        if mpd_ispositive(a) != 0 {
            mpd_qcopy(result, a, status);
        } else {
            mpd_clear_flags(result);
            mpd_qmaxcoeff(result, ctx, status);
            if mpd_isnan(result) != 0 {
                return;
            }
            mpd_set_flags(result, MPD_NEG);
            (*result).exp = mpd_etop(ctx);
        }
        return;
    }
    mpd_workcontext(&raw mut workctx, ctx);
    workctx.round = MPD_ROUND_CEILING as ::core::ffi::c_int;
    if mpd_qcopy(result, a, status) == 0 {
        return;
    }
    mpd_qfinalize(result, &raw mut workctx, &raw mut workctx.status);
    if workctx.status & (MPD_Inexact as uint32_t | MPD_Errors as uint32_t) != 0 {
        *status = (*status as ::core::ffi::c_uint
            | (workctx.status & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
            as uint32_t;
        return;
    }
    workctx.status = 0 as uint32_t;
    mpd_qadd(
        result,
        a,
        &raw mut tiny,
        &raw mut workctx,
        &raw mut workctx.status,
    );
    *status = (*status as ::core::ffi::c_uint
        | (workctx.status & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qnext_toward(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut c: ::core::ffi::c_int = 0;
    if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
        return;
    }
    c = _mpd_cmp(a, b);
    if c == 0 as ::core::ffi::c_int {
        mpd_qcopy_sign(result, a, b, status);
        return;
    }
    if c < 0 as ::core::ffi::c_int {
        mpd_qnext_plus(result, a, ctx, status);
    } else {
        mpd_qnext_minus(result, a, ctx, status);
    }
    if mpd_isinfinite(result) != 0 {
        *status = (*status as ::core::ffi::c_uint | (MPD_Overflow | MPD_Rounded | MPD_Inexact))
            as uint32_t;
    } else if mpd_adjexp(result) < (*ctx).emin {
        *status = (*status as ::core::ffi::c_uint
            | (MPD_Underflow | MPD_Subnormal | MPD_Rounded | MPD_Inexact))
            as uint32_t;
        if mpd_iszero(result) != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Clamped) as uint32_t;
        }
    }
}
unsafe extern "C" fn _mpd_qpow_uint(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: mpd_uint_t,
    mut resultsign: uint8_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut n: mpd_uint_t = 0;
    if exp == 0 as mpd_uint_t {
        _settriple(result, resultsign, 1 as mpd_uint_t, 0 as mpd_ssize_t);
        return;
    }
    if mpd_qcopy(result, base, status) == 0 {
        return;
    }
    n = *(&raw const mpd_bits as *const mpd_size_t).offset(mpd_bsr(exp as mpd_size_t) as isize)
        as mpd_uint_t;
    loop {
        n >>= 1 as ::core::ffi::c_int;
        if !(n != 0) {
            break;
        }
        mpd_qmul(result, result, result, ctx, &raw mut workstatus);
        if exp & n != 0 {
            mpd_qmul(result, result, base, ctx, &raw mut workstatus);
        }
        if mpd_isspecial(result) != 0
            || mpd_iszerocoeff(result) != 0 && workstatus & MPD_Clamped as uint32_t != 0
        {
            break;
        }
    }
    *status |= workstatus;
    mpd_set_sign(result, resultsign);
}
unsafe extern "C" fn _mpd_qpow_mpd(
    mut result: *mut mpd_t,
    mut tbase: *mut mpd_t,
    mut texp: *mut mpd_t,
    mut resultsign: uint8_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut maxctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut two_data: [mpd_uint_t; 1] = [2 as mpd_uint_t];
    let mut two: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut two_data as *mut mpd_uint_t,
    };
    mpd_maxcontext(&raw mut maxctx);
    mpd_qcopy(result, &raw const one, status);
    while mpd_iszero(texp) == 0 {
        if mpd_isodd(texp) != 0 {
            mpd_qmul(result, result, tbase, ctx, &raw mut workstatus);
            *status |= workstatus;
            if mpd_isspecial(result) != 0
                || mpd_iszerocoeff(result) != 0 && workstatus & MPD_Clamped as uint32_t != 0
            {
                break;
            }
        }
        mpd_qmul(tbase, tbase, tbase, ctx, &raw mut workstatus);
        mpd_qdivint(
            texp,
            texp,
            &raw mut two,
            &raw mut maxctx,
            &raw mut workstatus,
        );
        if mpd_isnan(tbase) != 0 || mpd_isnan(texp) != 0 {
            mpd_seterror(result, workstatus & MPD_Errors as uint32_t, status);
            return;
        }
    }
    mpd_set_sign(result, resultsign);
}
unsafe extern "C" fn _mpd_qpow_int(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: *const mpd_t,
    mut resultsign: uint8_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut tbase_data: [mpd_uint_t; 64] = [0; 64];
    let mut tbase: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tbase_data as *mut mpd_uint_t,
    };
    let mut texp_data: [mpd_uint_t; 64] = [0; 64];
    let mut texp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut texp_data as *mut mpd_uint_t,
    };
    let mut n: mpd_uint_t = 0;
    mpd_workcontext(&raw mut workctx, ctx);
    workctx.prec += (*exp).digits + (*exp).exp + 2 as mpd_ssize_t;
    workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    workctx.clamp = 0 as ::core::ffi::c_int;
    if mpd_isnegative(exp) != 0 {
        let mut workstatus: uint32_t = 0 as uint32_t;
        workctx.prec += 1 as mpd_ssize_t;
        mpd_qdiv(
            &raw mut tbase,
            &raw const one,
            base,
            &raw mut workctx,
            &raw mut workstatus,
        );
        *status |= workstatus;
        if workstatus & MPD_Errors as uint32_t != 0 {
            mpd_setspecial(result, MPD_POS, MPD_NAN);
            current_block = 1;
        } else {
            current_block = 2;
        }
    } else if mpd_qcopy(&raw mut tbase, base, status) == 0 {
        mpd_setspecial(result, MPD_POS, MPD_NAN);
        current_block = 1;
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            n = mpd_qabs_uint(exp, &raw mut workctx.status);
            if workctx.status & MPD_Invalid_operation as uint32_t != 0 {
                if mpd_qcopy(&raw mut texp, exp, status) == 0 {
                    mpd_setspecial(result, MPD_POS, MPD_NAN);
                    current_block = 1;
                } else {
                    _mpd_qpow_mpd(
                        result,
                        &raw mut tbase,
                        &raw mut texp,
                        resultsign,
                        &raw mut workctx,
                        status,
                    );
                    current_block = 3;
                }
            } else {
                _mpd_qpow_uint(
                    result,
                    &raw mut tbase,
                    n,
                    resultsign,
                    &raw mut workctx,
                    status,
                );
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    if mpd_isinfinite(result) != 0 {
                        _settriple(
                            result,
                            resultsign,
                            1 as mpd_uint_t,
                            MPD_EXP_INF as mpd_ssize_t,
                        );
                    }
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut tbase);
    mpd_del(&raw mut texp);
    mpd_qfinalize(result, ctx, status);
}
unsafe extern "C" fn _qcheck_pow_one_inf(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut resultsign: uint8_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut shift: mpd_ssize_t = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    cmp = _mpd_cmp(base, &raw const one);
    if cmp == 0 as ::core::ffi::c_int {
        shift = (*ctx).prec - 1 as mpd_ssize_t;
        mpd_qshiftl(result, &raw const one, shift, status);
        (*result).exp = -shift;
        mpd_set_flags(result, resultsign);
        *status = (*status as ::core::ffi::c_uint | (MPD_Inexact | MPD_Rounded)) as uint32_t;
    }
    return cmp;
}
unsafe extern "C" fn _qcheck_pow_one(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: *const mpd_t,
    mut resultsign: uint8_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut shift: mpd_ssize_t = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    cmp = _mpd_cmp_abs(base, &raw const one);
    if cmp == 0 as ::core::ffi::c_int {
        if _mpd_isint(exp) != 0 {
            if mpd_isnegative(exp) != 0 {
                _settriple(result, resultsign, 1 as mpd_uint_t, 0 as mpd_ssize_t);
                return 0 as ::core::ffi::c_int;
            }
            mpd_qmul_ssize(result, exp, -(*base).exp, ctx, &raw mut workstatus);
            if workstatus & MPD_Errors as uint32_t != 0 {
                *status = (*status as ::core::ffi::c_uint
                    | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
                    as uint32_t;
                return 0 as ::core::ffi::c_int;
            }
            shift = mpd_qget_ssize(result, &raw mut workstatus);
            if shift > (*ctx).prec - 1 as mpd_ssize_t {
                shift = (*ctx).prec - 1 as mpd_ssize_t;
                *status = (*status as ::core::ffi::c_uint | MPD_Rounded) as uint32_t;
            }
        } else if mpd_ispositive(base) != 0 {
            shift = (*ctx).prec - 1 as mpd_ssize_t;
            *status = (*status as ::core::ffi::c_uint | (MPD_Inexact | MPD_Rounded)) as uint32_t;
        } else {
            return -(2 as ::core::ffi::c_int);
        }
        if mpd_qshiftl(result, &raw const one, shift, status) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*result).exp = -shift;
        mpd_set_flags(result, resultsign);
    }
    return cmp;
}
unsafe extern "C" fn _lower_bound_zeta(
    mut x: *const mpd_t,
    mut status: *mut uint32_t,
) -> mpd_ssize_t {
    let mut maxctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut scratch_data: [mpd_uint_t; 64] = [0; 64];
    let mut scratch: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut scratch_data as *mut mpd_uint_t,
    };
    let mut t: mpd_ssize_t = 0;
    let mut u: mpd_ssize_t = 0;
    t = mpd_adjexp(x);
    if t > 0 as mpd_ssize_t {
        return (mpd_exp_digits(t) - 1 as ::core::ffi::c_int) as mpd_ssize_t;
    } else if t < -(1 as ::core::ffi::c_int) as mpd_ssize_t {
        return (mpd_exp_digits(t + 1 as mpd_ssize_t) - 1 as ::core::ffi::c_int) as mpd_ssize_t;
    } else {
        mpd_maxcontext(&raw mut maxctx);
        mpd_qsub(&raw mut scratch, x, &raw const one, &raw mut maxctx, status);
        if mpd_isspecial(&raw mut scratch) != 0 {
            mpd_del(&raw mut scratch);
            return MPD_SSIZE_MAX as mpd_ssize_t;
        }
        u = mpd_adjexp(&raw mut scratch);
        mpd_del(&raw mut scratch);
        return if t == 0 as mpd_ssize_t {
            u - 2 as mpd_ssize_t
        } else {
            u - 1 as mpd_ssize_t
        };
    };
}
unsafe extern "C" fn _qcheck_pow_bounds(
    mut result: *mut mpd_t,
    mut x: *const mpd_t,
    mut y: *const mpd_t,
    mut resultsign: uint8_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut abs_x: mpd_t = mpd_t {
        flags: ((*x).flags as ::core::ffi::c_int & !MPD_DATAFLAGS
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_SHARED_DATA as ::core::ffi::c_int) as uint8_t,
        exp: (*x).exp,
        digits: (*x).digits,
        len: (*x).len,
        alloc: (*x).alloc,
        data: (*x).data,
    };
    let mut ub_omega: mpd_ssize_t = 0;
    let mut lb_zeta: mpd_ssize_t = 0;
    let mut lb_theta: mpd_ssize_t = 0;
    let mut sign: uint8_t = 0;
    mpd_set_positive(&raw mut abs_x);
    lb_theta = mpd_adjexp(y);
    lb_zeta = _lower_bound_zeta(&raw mut abs_x, status);
    if lb_zeta == MPD_SSIZE_MAX as mpd_ssize_t {
        mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
        return 1 as ::core::ffi::c_int;
    }
    sign = ((mpd_adjexp(&raw mut abs_x) < 0 as mpd_ssize_t) as ::core::ffi::c_int
        ^ mpd_sign(y) as ::core::ffi::c_int) as uint8_t;
    if sign as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ub_omega = mpd_exp_digits((*ctx).emax) as mpd_ssize_t;
        if ub_omega < lb_zeta + lb_theta {
            _settriple(
                result,
                resultsign,
                1 as mpd_uint_t,
                MPD_EXP_INF as mpd_ssize_t,
            );
            mpd_qfinalize(result, ctx, status);
            return 1 as ::core::ffi::c_int;
        }
    } else {
        ub_omega = mpd_exp_digits(mpd_etiny(ctx)) as mpd_ssize_t;
        if ub_omega < lb_zeta + lb_theta {
            _settriple(
                result,
                resultsign,
                1 as mpd_uint_t,
                mpd_etiny(ctx) - 1 as mpd_ssize_t,
            );
            mpd_qfinalize(result, ctx, status);
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_qpow_real(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut texp_data: [mpd_uint_t; 64] = [0; 64];
    let mut texp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut texp_data as *mut mpd_uint_t,
    };
    if mpd_qcopy(&raw mut texp, exp, status) == 0 {
        mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
        return;
    }
    mpd_maxcontext(&raw mut workctx);
    workctx.prec = if (*base).digits > (*ctx).prec {
        (*base).digits
    } else {
        (*ctx).prec
    };
    workctx.prec += (4 as ::core::ffi::c_int + MPD_EXPDIGITS) as mpd_ssize_t;
    workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    workctx.allcr = (*ctx).allcr;
    mpd_qln(result, base, &raw mut workctx, &raw mut workctx.status);
    mpd_qmul(
        result,
        result,
        &raw mut texp,
        &raw mut workctx,
        &raw mut workctx.status,
    );
    mpd_qexp(result, result, &raw mut workctx, status);
    mpd_del(&raw mut texp);
    *status = (*status as ::core::ffi::c_uint
        | (workctx.status & MPD_Errors as uint32_t) as ::core::ffi::c_uint)
        as uint32_t;
    *status = (*status as ::core::ffi::c_uint | (MPD_Inexact | MPD_Rounded)) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qpow(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut resultsign: uint8_t = 0 as uint8_t;
    let mut intexp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cmp: ::core::ffi::c_int = 0;
    if mpd_isspecial(base) != 0 || mpd_isspecial(exp) != 0 {
        if mpd_qcheck_nans(result, base, exp, ctx, status) != 0 {
            return;
        }
    }
    if mpd_isinteger(exp) != 0 {
        intexp = 1 as ::core::ffi::c_int;
        resultsign =
            (mpd_isnegative(base) != 0 && mpd_isodd(exp) != 0) as ::core::ffi::c_int as uint8_t;
    }
    if mpd_iszero(base) != 0 {
        if mpd_iszero(exp) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        } else if mpd_isnegative(exp) != 0 {
            mpd_setspecial(result, resultsign, MPD_INF);
        } else {
            _settriple(result, resultsign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
        }
        return;
    }
    if mpd_isnegative(base) != 0 {
        if intexp == 0 || mpd_isinfinite(exp) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
    }
    if mpd_isinfinite(exp) != 0 {
        cmp = _qcheck_pow_one_inf(result, base, resultsign, ctx, status);
        if cmp == 0 as ::core::ffi::c_int {
            return;
        } else {
            cmp *= mpd_arith_sign(exp);
            if cmp < 0 as ::core::ffi::c_int {
                _settriple(result, resultsign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
            } else {
                mpd_setspecial(result, resultsign, MPD_INF);
            }
        }
        return;
    }
    if mpd_isinfinite(base) != 0 {
        if mpd_iszero(exp) != 0 {
            _settriple(result, resultsign, 1 as mpd_uint_t, 0 as mpd_ssize_t);
        } else if mpd_isnegative(exp) != 0 {
            _settriple(result, resultsign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
        } else {
            mpd_setspecial(result, resultsign, MPD_INF);
        }
        return;
    }
    if mpd_iszero(exp) != 0 {
        _settriple(result, resultsign, 1 as mpd_uint_t, 0 as mpd_ssize_t);
        return;
    }
    if _qcheck_pow_one(result, base, exp, resultsign, ctx, status) == 0 as ::core::ffi::c_int {
        return;
    }
    if _qcheck_pow_bounds(result, base, exp, resultsign, ctx, status) != 0 {
        return;
    }
    if intexp != 0 {
        _mpd_qpow_int(result, base, exp, resultsign, ctx, status);
    } else {
        _mpd_qpow_real(result, base, exp, ctx, status);
        if mpd_isspecial(result) == 0 && _mpd_cmp(result, &raw const one) == 0 as ::core::ffi::c_int
        {
            let mut shift: mpd_ssize_t = (*ctx).prec - 1 as mpd_ssize_t;
            mpd_qshiftl(result, &raw const one, shift, status);
            (*result).exp = -shift;
        }
        if mpd_isinfinite(result) != 0 {
            _settriple(result, MPD_POS, 1 as mpd_uint_t, MPD_EXP_INF as mpd_ssize_t);
        }
        mpd_qfinalize(result, ctx, status);
    };
}
unsafe extern "C" fn _mpd_qpowmod_uint(
    mut result: *mut mpd_t,
    mut base: *mut mpd_t,
    mut exp: mpd_uint_t,
    mut mod_0: *const mpd_t,
    mut status: *mut uint32_t,
) {
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qcopy(result, &raw const one, status);
    while exp > 0 as mpd_uint_t {
        if exp & 1 as mpd_uint_t != 0 {
            _mpd_qmul_exact(result, result, base, &raw mut maxcontext, status);
            mpd_qrem(result, result, mod_0, &raw mut maxcontext, status);
        }
        _mpd_qmul_exact(base, base, base, &raw mut maxcontext, status);
        mpd_qrem(base, base, mod_0, &raw mut maxcontext, status);
        exp >>= 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qpowmod(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: *const mpd_t,
    mut mod_0: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut tbase_data: [mpd_uint_t; 64] = [0; 64];
    let mut tbase: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tbase_data as *mut mpd_uint_t,
    };
    let mut texp_data: [mpd_uint_t; 64] = [0; 64];
    let mut texp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut texp_data as *mut mpd_uint_t,
    };
    let mut tmod_data: [mpd_uint_t; 64] = [0; 64];
    let mut tmod: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tmod_data as *mut mpd_uint_t,
    };
    let mut tmp_data: [mpd_uint_t; 64] = [0; 64];
    let mut tmp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tmp_data as *mut mpd_uint_t,
    };
    let mut two_data: [mpd_uint_t; 1] = [2 as mpd_uint_t];
    let mut two: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut two_data as *mut mpd_uint_t,
    };
    let mut tbase_exp: mpd_ssize_t = 0;
    let mut texp_exp: mpd_ssize_t = 0;
    let mut i: mpd_ssize_t = 0;
    let mut t: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    let mut r: mpd_uint_t = 0;
    let mut sign: uint8_t = 0;
    if mpd_isspecial(base) != 0 || mpd_isspecial(exp) != 0 || mpd_isspecial(mod_0) != 0 {
        if mpd_qcheck_3nans(result, base, exp, mod_0, ctx, status) != 0 {
            return;
        }
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if _mpd_isint(base) == 0 || _mpd_isint(exp) == 0 || _mpd_isint(mod_0) == 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_iszerocoeff(mod_0) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if (*mod_0).digits + (*mod_0).exp > (*ctx).prec {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    sign = (mpd_isnegative(base) != 0 && mpd_isodd(exp) != 0) as ::core::ffi::c_int as uint8_t;
    if mpd_iszerocoeff(exp) != 0 {
        if mpd_iszerocoeff(base) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        r = (if _mpd_cmp_abs(mod_0, &raw const one) == 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as mpd_uint_t;
        _settriple(result, sign, r, 0 as mpd_ssize_t);
        return;
    }
    if mpd_isnegative(exp) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_iszerocoeff(base) != 0 {
        _settriple(result, sign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
        return;
    }
    mpd_maxcontext(&raw mut maxcontext);
    mpd_qrescale(
        &raw mut tmod,
        mod_0,
        0 as mpd_ssize_t,
        &raw mut maxcontext,
        &raw mut maxcontext.status,
    );
    if maxcontext.status & MPD_Errors as uint32_t != 0 {
        mpd_seterror(result, maxcontext.status & MPD_Errors as uint32_t, status);
    } else {
        maxcontext.status = 0 as uint32_t;
        mpd_set_positive(&raw mut tmod);
        mpd_qround_to_int(&raw mut tbase, base, &raw mut maxcontext, status);
        mpd_set_positive(&raw mut tbase);
        tbase_exp = tbase.exp;
        tbase.exp = 0 as mpd_ssize_t;
        mpd_qround_to_int(&raw mut texp, exp, &raw mut maxcontext, status);
        texp_exp = texp.exp;
        texp.exp = 0 as mpd_ssize_t;
        mpd_qrem(
            &raw mut tbase,
            &raw mut tbase,
            &raw mut tmod,
            &raw mut maxcontext,
            status,
        );
        mpd_qshiftl(result, &raw const one, tbase_exp, status);
        mpd_qrem(result, result, &raw mut tmod, &raw mut maxcontext, status);
        _mpd_qmul_exact(
            &raw mut tbase,
            &raw mut tbase,
            result,
            &raw mut maxcontext,
            status,
        );
        mpd_qrem(
            &raw mut tbase,
            &raw mut tbase,
            &raw mut tmod,
            &raw mut maxcontext,
            status,
        );
        if mpd_isspecial(&raw mut tbase) != 0
            || mpd_isspecial(&raw mut texp) != 0
            || mpd_isspecial(&raw mut tmod) != 0
        {
            current_block = 1;
        } else {
            i = 0 as mpd_ssize_t;
            while i < texp_exp {
                _mpd_qpowmod_uint(
                    &raw mut tmp,
                    &raw mut tbase,
                    10 as mpd_uint_t,
                    &raw mut tmod,
                    status,
                );
                t = tmp;
                tmp = tbase;
                tbase = t;
                i += 1;
            }
            if mpd_isspecial(&raw mut tbase) != 0 {
                current_block = 1;
            } else {
                mpd_qcopy(result, &raw const one, status);
                while mpd_isfinite(&raw mut texp) != 0 && mpd_iszero(&raw mut texp) == 0 {
                    if mpd_isodd(&raw mut texp) != 0 {
                        _mpd_qmul_exact(
                            result,
                            result,
                            &raw mut tbase,
                            &raw mut maxcontext,
                            status,
                        );
                        mpd_qrem(result, result, &raw mut tmod, &raw mut maxcontext, status);
                    }
                    _mpd_qmul_exact(
                        &raw mut tbase,
                        &raw mut tbase,
                        &raw mut tbase,
                        &raw mut maxcontext,
                        status,
                    );
                    mpd_qrem(
                        &raw mut tbase,
                        &raw mut tbase,
                        &raw mut tmod,
                        &raw mut maxcontext,
                        status,
                    );
                    mpd_qdivint(
                        &raw mut texp,
                        &raw mut texp,
                        &raw mut two,
                        &raw mut maxcontext,
                        status,
                    );
                }
                if mpd_isspecial(&raw mut texp) != 0
                    || mpd_isspecial(&raw mut tbase) != 0
                    || mpd_isspecial(&raw mut tmod) != 0
                    || mpd_isspecial(result) != 0
                {
                    current_block = 1;
                } else {
                    mpd_set_sign(result, sign);
                    current_block = 2;
                }
            }
        }
        match current_block {
            2 => {}
            _ => {
                mpd_setspecial(result, MPD_POS, MPD_NAN);
            }
        }
    }
    mpd_del(&raw mut tbase);
    mpd_del(&raw mut texp);
    mpd_del(&raw mut tmod);
    mpd_del(&raw mut tmp);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qquantize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut b_exp: mpd_ssize_t = (*b).exp;
    let mut expdiff: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut rnd: mpd_uint_t = 0;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(result, a, b, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 && mpd_isinfinite(b) != 0 {
            mpd_qcopy(result, a, status);
            return;
        }
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if (*b).exp > (*ctx).emax || (*b).exp < mpd_etiny(ctx) {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_iszero(a) != 0 {
        _settriple(result, mpd_sign(a), 0 as mpd_uint_t, (*b).exp);
        mpd_qfinalize(result, ctx, status);
        return;
    }
    expdiff = (*a).exp - (*b).exp;
    if (*a).digits + expdiff > (*ctx).prec {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if expdiff >= 0 as mpd_ssize_t {
        shift = expdiff;
        if mpd_qshiftl(result, a, shift, status) == 0 {
            return;
        }
        (*result).exp = b_exp;
    } else {
        shift = -expdiff;
        rnd = mpd_qshiftr(result, a, shift, status);
        if rnd == MPD_UINT_MAX as mpd_uint_t {
            return;
        }
        (*result).exp = b_exp;
        if _mpd_apply_round_fit(result, rnd, ctx, status) == 0 {
            return;
        }
        workstatus = (workstatus as ::core::ffi::c_uint | MPD_Rounded) as uint32_t;
        if rnd != 0 {
            workstatus = (workstatus as ::core::ffi::c_uint | MPD_Inexact) as uint32_t;
        }
    }
    if mpd_adjexp(result) > (*ctx).emax || mpd_adjexp(result) < mpd_etiny(ctx) {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    *status |= workstatus;
    mpd_qfinalize(result, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qreduce(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut shift: mpd_ssize_t = 0;
    let mut maxexp: mpd_ssize_t = 0;
    let mut maxshift: mpd_ssize_t = 0;
    let mut sign_a: uint8_t = mpd_sign(a);
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        mpd_qcopy(result, a, status);
        return;
    }
    if mpd_qcopy(result, a, status) == 0 {
        return;
    }
    mpd_qfinalize(result, ctx, status);
    if mpd_isspecial(result) != 0 {
        return;
    }
    if mpd_iszero(result) != 0 {
        _settriple(result, sign_a, 0 as mpd_uint_t, 0 as mpd_ssize_t);
        return;
    }
    shift = mpd_trail_zeros(result);
    maxexp = if (*ctx).clamp != 0 {
        mpd_etop(ctx)
    } else {
        (*ctx).emax
    };
    maxshift = maxexp - (*result).exp;
    shift = if shift > maxshift { maxshift } else { shift };
    mpd_qshiftr_inplace(result, shift);
    (*result).exp += shift;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qrem(
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut q_data: [mpd_uint_t; 64] = [0; 64];
    let mut q: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut q_data as *mut mpd_uint_t,
    };
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(r, a, b, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 {
            mpd_seterror(r, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        if mpd_isinfinite(b) != 0 {
            mpd_qcopy(r, a, status);
            mpd_qfinalize(r, ctx, status);
            return;
        }
        abort();
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_seterror(r, MPD_Division_undefined as uint32_t, status);
        } else {
            mpd_seterror(r, MPD_Invalid_operation as uint32_t, status);
        }
        return;
    }
    _mpd_qdivmod(&raw mut q, r, a, b, ctx, status);
    mpd_del(&raw mut q);
    mpd_qfinalize(r, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qrem_near(
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut btmp_data: [mpd_uint_t; 64] = [0; 64];
    let mut btmp: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut btmp_data as *mut mpd_uint_t,
    };
    let mut q_data: [mpd_uint_t; 64] = [0; 64];
    let mut q: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut q_data as *mut mpd_uint_t,
    };
    let mut expdiff: mpd_ssize_t = 0;
    let mut qdigits: mpd_ssize_t = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    let mut isodd: ::core::ffi::c_int = 0;
    let mut allnine: ::core::ffi::c_int = 0;
    if !r.is_null() {
    } else {
        mpd_c2rust_assert_fail(b"r != NULL\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(r, a, b, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 {
            mpd_seterror(r, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        if mpd_isinfinite(b) != 0 {
            mpd_qcopy(r, a, status);
            mpd_qfinalize(r, ctx, status);
            return;
        }
        abort();
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_seterror(r, MPD_Division_undefined as uint32_t, status);
        } else {
            mpd_seterror(r, MPD_Invalid_operation as uint32_t, status);
        }
        return;
    }
    if r == b as *mut mpd_t {
        if mpd_qcopy(&raw mut btmp, b, status) == 0 {
            mpd_seterror(r, MPD_Malloc_error as uint32_t, status);
            return;
        }
        b = &raw mut btmp;
    }
    _mpd_qdivmod(&raw mut q, r, a, b, ctx, status);
    if !(mpd_isnan(&raw mut q) != 0 || mpd_isnan(r) != 0) {
        if !(mpd_iszerocoeff(r) != 0) {
            expdiff = mpd_adjexp(b) - mpd_adjexp(r);
            if -(1 as ::core::ffi::c_int) as mpd_ssize_t <= expdiff && expdiff <= 1 as mpd_ssize_t {
                allnine = mpd_coeff_isallnine(&raw mut q);
                qdigits = q.digits;
                isodd = mpd_isodd(&raw mut q);
                mpd_maxcontext(&raw mut workctx);
                if mpd_sign(a) as ::core::ffi::c_int == mpd_sign(b) as ::core::ffi::c_int {
                    _mpd_qsub(&raw mut q, r, b, &raw mut workctx, &raw mut workctx.status);
                } else {
                    _mpd_qadd(&raw mut q, r, b, &raw mut workctx, &raw mut workctx.status);
                }
                if workctx.status & MPD_Errors as uint32_t != 0 {
                    mpd_seterror(r, workctx.status & MPD_Errors as uint32_t, status);
                } else {
                    cmp = _mpd_cmp_abs(&raw mut q, r);
                    if cmp < 0 as ::core::ffi::c_int || cmp == 0 as ::core::ffi::c_int && isodd != 0
                    {
                        if allnine != 0 && qdigits == (*ctx).prec {
                            mpd_seterror(r, MPD_Division_impossible as uint32_t, status);
                        } else {
                            mpd_qcopy(r, &raw mut q, status);
                        }
                    }
                }
            }
        }
    }
    mpd_del(&raw mut btmp);
    mpd_del(&raw mut q);
    mpd_qfinalize(r, ctx, status);
}
unsafe extern "C" fn _mpd_qrescale(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut exp: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut expdiff: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut rnd: mpd_uint_t = 0;
    if mpd_isspecial(a) != 0 {
        mpd_qcopy(result, a, status);
        return;
    }
    if mpd_iszero(a) != 0 {
        _settriple(result, mpd_sign(a), 0 as mpd_uint_t, exp);
        return;
    }
    expdiff = (*a).exp - exp;
    if expdiff >= 0 as mpd_ssize_t {
        shift = expdiff;
        if (*a).digits + shift > MPD_MAX_PREC as mpd_ssize_t + 1 as mpd_ssize_t {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        if mpd_qshiftl(result, a, shift, status) == 0 {
            return;
        }
        (*result).exp = exp;
    } else {
        shift = -expdiff;
        rnd = mpd_qshiftr(result, a, shift, status);
        if rnd == MPD_UINT_MAX as mpd_uint_t {
            return;
        }
        (*result).exp = exp;
        _mpd_apply_round_excess(result, rnd, ctx, status);
        *status = (*status as ::core::ffi::c_uint | MPD_Rounded) as uint32_t;
        if rnd != 0 {
            *status = (*status as ::core::ffi::c_uint | MPD_Inexact) as uint32_t;
        }
    }
    if mpd_issubnormal(result, ctx) != 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Subnormal) as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qrescale(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut exp: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if exp > MPD_MAX_EMAX as mpd_ssize_t + 1 as mpd_ssize_t || exp < MPD_MIN_ETINY as mpd_ssize_t {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    _mpd_qrescale(result, a, exp, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qrescale_fmt(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut exp: mpd_ssize_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if exp > MPD_MAX_EMAX as mpd_ssize_t + 1 as mpd_ssize_t
        || exp < MPD_MIN_ETINY as mpd_ssize_t - MPD_MAX_PREC as mpd_ssize_t
    {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    _mpd_qrescale(result, a, exp, ctx, status);
}
unsafe extern "C" fn _mpd_qround_to_integral(
    mut action: ::core::ffi::c_int,
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut rnd: mpd_uint_t = 0;
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        mpd_qcopy(result, a, status);
        return;
    }
    if (*a).exp >= 0 as mpd_ssize_t {
        mpd_qcopy(result, a, status);
        return;
    }
    if mpd_iszerocoeff(a) != 0 {
        _settriple(result, mpd_sign(a), 0 as mpd_uint_t, 0 as mpd_ssize_t);
        return;
    }
    rnd = mpd_qshiftr(result, a, -(*a).exp, status);
    if rnd == MPD_UINT_MAX as mpd_uint_t {
        return;
    }
    (*result).exp = 0 as mpd_ssize_t;
    if action == TO_INT_EXACT as ::core::ffi::c_int || action == TO_INT_SILENT as ::core::ffi::c_int
    {
        _mpd_apply_round_excess(result, rnd, ctx, status);
        if action == TO_INT_EXACT as ::core::ffi::c_int {
            *status = (*status as ::core::ffi::c_uint | MPD_Rounded) as uint32_t;
            if rnd != 0 {
                *status = (*status as ::core::ffi::c_uint | MPD_Inexact) as uint32_t;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qround_to_intx(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _mpd_qround_to_integral(TO_INT_EXACT as ::core::ffi::c_int, result, a, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qround_to_int(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _mpd_qround_to_integral(TO_INT_SILENT as ::core::ffi::c_int, result, a, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qtrunc(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    if mpd_isspecial(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    _mpd_qround_to_integral(TO_INT_TRUNC as ::core::ffi::c_int, result, a, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qfloor(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    mpd_workcontext(&raw mut workctx, ctx);
    if mpd_isspecial(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    workctx.round = MPD_ROUND_FLOOR as ::core::ffi::c_int;
    _mpd_qround_to_integral(
        TO_INT_SILENT as ::core::ffi::c_int,
        result,
        a,
        &raw mut workctx,
        status,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qceil(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    mpd_workcontext(&raw mut workctx, ctx);
    if mpd_isspecial(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    workctx.round = MPD_ROUND_CEILING as ::core::ffi::c_int;
    _mpd_qround_to_integral(
        TO_INT_SILENT as ::core::ffi::c_int,
        result,
        a,
        &raw mut workctx,
        status,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpd_same_quantum(
    mut a: *const mpd_t,
    mut b: *const mpd_t,
) -> ::core::ffi::c_int {
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        return (mpd_isnan(a) != 0 && mpd_isnan(b) != 0
            || mpd_isinfinite(a) != 0 && mpd_isinfinite(b) != 0)
            as ::core::ffi::c_int;
    }
    return ((*a).exp == (*b).exp) as ::core::ffi::c_int;
}
unsafe extern "C" fn recpr_schedule_prec(
    mut klist: *mut mpd_ssize_t,
    mut maxprec: mpd_ssize_t,
    mut initprec: mpd_ssize_t,
) -> ::core::ffi::c_int {
    let mut k: mpd_ssize_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    if maxprec > 0 as mpd_ssize_t && initprec > 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(
            b"maxprec > 0 && initprec > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if maxprec <= initprec {
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    k = maxprec;
    loop {
        k = (k + 1 as mpd_ssize_t) / 2 as mpd_ssize_t;
        let fresh0 = i;
        i = i + 1;
        *klist.offset(fresh0 as isize) = k;
        if !(k > initprec) {
            break;
        }
    }
    return i - 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _mpd_qreciprocal_approx(
    mut z: *mut mpd_t,
    mut v: *const mpd_t,
    mut status: *mut uint32_t,
) {
    let mut p10data: [mpd_uint_t; 2] = [
        0 as mpd_uint_t,
        *(&raw const mpd_pow10 as *const mpd_uint_t)
            .offset((MPD_RDIGITS - 2 as ::core::ffi::c_int) as isize),
    ];
    let mut dummy: mpd_uint_t = 0;
    let mut word: mpd_uint_t = 0;
    let mut n: ::core::ffi::c_int = 0;
    if (*v).exp == -(*v).digits {
    } else {
        mpd_c2rust_assert_fail(
            b"v->exp == -v->digits\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    _mpd_get_msdigits(
        &raw mut dummy,
        &raw mut word,
        v,
        MPD_RDIGITS as ::core::ffi::c_uint,
    );
    n = mpd_word_digits(word);
    word = word.wrapping_mul(
        *(&raw const mpd_pow10 as *const mpd_uint_t).offset((MPD_RDIGITS - n) as isize),
    );
    mpd_qresize(z, 2 as mpd_ssize_t, status);
    _mpd_shortdiv(
        (*z).data,
        &raw mut p10data as *mut mpd_uint_t,
        2 as mpd_size_t,
        word,
    );
    mpd_clear_flags(z);
    (*z).exp = -(MPD_RDIGITS - 2 as ::core::ffi::c_int) as mpd_ssize_t;
    (*z).len = (if *(*z).data.offset(1 as isize) == 0 as mpd_uint_t {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as mpd_ssize_t;
    mpd_setdigits(z);
}
unsafe extern "C" fn _mpd_qreciprocal(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut varcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut z: *mut mpd_t = result;
    let mut v: *mut mpd_t = ::core::ptr::null_mut::<mpd_t>();
    let mut vtmp: mpd_t = mpd_t {
        flags: ((*a).flags as ::core::ffi::c_int & !MPD_DATAFLAGS
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_SHARED_DATA as ::core::ffi::c_int) as uint8_t,
        exp: (*a).exp,
        digits: (*a).digits,
        len: (*a).len,
        alloc: (*a).alloc,
        data: (*a).data,
    };
    let mut s_data: [mpd_uint_t; 64] = [0; 64];
    let mut s: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut s_data as *mut mpd_uint_t,
    };
    let mut t_data: [mpd_uint_t; 64] = [0; 64];
    let mut t: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut t_data as *mut mpd_uint_t,
    };
    let mut two_data: [mpd_uint_t; 1] = [2 as mpd_uint_t];
    let mut two: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut two_data as *mut mpd_uint_t,
    };
    let mut klist: [mpd_ssize_t; 64] = [0; 64];
    let mut adj: mpd_ssize_t = 0;
    let mut maxprec: mpd_ssize_t = 0;
    let mut initprec: mpd_ssize_t = 0;
    let mut sign: uint8_t = mpd_sign(a);
    let mut i: ::core::ffi::c_int = 0;
    if result != a as *mut mpd_t {
    } else {
        mpd_c2rust_assert_fail(b"result != a\0" as *const u8 as *const ::core::ffi::c_char);
    };
    v = &raw mut vtmp;
    mpd_clear_flags(v);
    adj = (*v).digits + (*v).exp;
    (*v).exp = -(*v).digits;
    _mpd_qreciprocal_approx(z, v, status);
    mpd_maxcontext(&raw mut varcontext);
    mpd_maxcontext(&raw mut maxcontext);
    maxcontext.round = MPD_ROUND_TRUNC as ::core::ffi::c_int;
    varcontext.round = maxcontext.round;
    maxcontext.emax = (MPD_MAX_EMAX + 100 as i64) as mpd_ssize_t;
    varcontext.emax = maxcontext.emax;
    maxcontext.emin = (MPD_MIN_EMIN - 100 as i64) as mpd_ssize_t;
    varcontext.emin = maxcontext.emin;
    maxcontext.prec = (MPD_MAX_PREC + 100 as i64) as mpd_ssize_t;
    maxprec = (*ctx).prec;
    maxprec += 2 as mpd_ssize_t;
    initprec = (MPD_RDIGITS - 3 as ::core::ffi::c_int) as mpd_ssize_t;
    i = recpr_schedule_prec(&raw mut klist as *mut mpd_ssize_t, maxprec, initprec);
    while i >= 0 as ::core::ffi::c_int {
        _mpd_qmul_exact(&raw mut s, z, z, &raw mut maxcontext, status);
        varcontext.prec = 2 as mpd_ssize_t * klist[i as usize] + 5 as mpd_ssize_t;
        if (*v).digits > varcontext.prec {
            mpd_qshiftr(&raw mut t, v, (*v).digits - varcontext.prec, status);
            t.exp = -varcontext.prec;
            mpd_qmul(
                &raw mut t,
                &raw mut t,
                &raw mut s,
                &raw mut varcontext,
                status,
            );
        } else {
            mpd_qmul(&raw mut t, v, &raw mut s, &raw mut varcontext, status);
        }
        _mpd_qmul_exact(&raw mut s, z, &raw mut two, &raw mut maxcontext, status);
        _mpd_qsub_exact(z, &raw mut s, &raw mut t, &raw mut maxcontext, status);
        i -= 1;
    }
    if mpd_isspecial(z) == 0 {
        (*z).exp -= adj;
        mpd_set_flags(z, sign);
    }
    mpd_del(&raw mut s);
    mpd_del(&raw mut t);
    mpd_qfinalize(z, ctx, status);
}
unsafe extern "C" fn _mpd_base_ndivmod(
    mut q: *mut mpd_t,
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut qq: *mut mpd_t = q;
    let mut rr: *mut mpd_t = r;
    let mut aa: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    let mut bb: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    let mut k: ::core::ffi::c_int = 0;
    _mpd_copy_shared(&raw mut aa, a);
    _mpd_copy_shared(&raw mut bb, b);
    mpd_set_positive(&raw mut aa);
    mpd_set_positive(&raw mut bb);
    aa.exp = 0 as mpd_ssize_t;
    bb.exp = 0 as mpd_ssize_t;
    if q == a as *mut mpd_t || q == b as *mut mpd_t {
        qq = mpd_qnew();
        if qq.is_null() {
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
            current_block = 1;
        } else {
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            if r == a as *mut mpd_t || r == b as *mut mpd_t {
                rr = mpd_qnew();
                if rr.is_null() {
                    *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
                    current_block = 1;
                } else {
                    current_block = 3;
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    mpd_maxcontext(&raw mut workctx);
                    workctx.prec = (*a).digits - (*b).digits + 1 as mpd_ssize_t + 3 as mpd_ssize_t;
                    if (*a).digits > MPD_MAX_PREC as mpd_ssize_t
                        || workctx.prec > MPD_MAX_PREC as mpd_ssize_t
                    {
                        *status =
                            (*status as ::core::ffi::c_uint | MPD_Division_impossible) as uint32_t;
                    } else {
                        _mpd_qreciprocal(
                            rr,
                            &raw mut bb,
                            &raw mut workctx,
                            &raw mut workctx.status,
                        );
                        _mpd_qmul(
                            qq,
                            &raw mut aa,
                            rr,
                            &raw mut workctx,
                            &raw mut workctx.status,
                        );
                        mpd_qtrunc(qq, qq, &raw mut workctx, &raw mut workctx.status);
                        workctx.prec = aa.digits + 3 as mpd_ssize_t;
                        workctx.emax = (MPD_MAX_EMAX + 3 as i64) as mpd_ssize_t;
                        workctx.emin = (MPD_MIN_EMIN - 3 as i64) as mpd_ssize_t;
                        _mpd_qmul(
                            rr,
                            &raw mut bb,
                            qq,
                            &raw mut workctx,
                            &raw mut workctx.status,
                        );
                        _mpd_qsub_exact(
                            rr,
                            &raw mut aa,
                            rr,
                            &raw mut workctx,
                            &raw mut workctx.status,
                        );
                        k = 0 as ::core::ffi::c_int;
                        loop {
                            if mpd_isspecial(qq) != 0 || mpd_isspecial(rr) != 0 {
                                *status = (*status as ::core::ffi::c_uint
                                    | (workctx.status & MPD_Errors as uint32_t)
                                        as ::core::ffi::c_uint)
                                    as uint32_t;
                                current_block = 1;
                                break;
                            } else if k > 2 as ::core::ffi::c_int {
                                fprintf(
                                    mpd_c2rust_stderr(),
                                    b"%s:%d: warning: \0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"libmpdec/mpdecimal.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    7764 as ::core::ffi::c_int,
                                );
                                fprintf(
                                    mpd_c2rust_stderr(),
                                    b"libmpdec: internal error in _mpd_base_ndivmod: please report\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                                fputc('\n' as i32, mpd_c2rust_stderr());
                                *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation)
                                    as uint32_t;
                                current_block = 1;
                                break;
                            } else {
                                if _mpd_cmp(&raw const zero, rr) == 1 as ::core::ffi::c_int {
                                    _mpd_qadd_exact(
                                        rr,
                                        rr,
                                        &raw mut bb,
                                        &raw mut workctx,
                                        &raw mut workctx.status,
                                    );
                                    _mpd_qadd_exact(
                                        qq,
                                        qq,
                                        &raw const minus_one,
                                        &raw mut workctx,
                                        &raw mut workctx.status,
                                    );
                                } else {
                                    if _mpd_cmp(rr, &raw mut bb) == -(1 as ::core::ffi::c_int) {
                                        current_block = 4;
                                        break;
                                    }
                                    _mpd_qsub_exact(
                                        rr,
                                        rr,
                                        &raw mut bb,
                                        &raw mut workctx,
                                        &raw mut workctx.status,
                                    );
                                    _mpd_qadd_exact(
                                        qq,
                                        qq,
                                        &raw const one,
                                        &raw mut workctx,
                                        &raw mut workctx.status,
                                    );
                                }
                                k += 1;
                            }
                        }
                        match current_block {
                            1 => {}
                            _ => {
                                if qq != q {
                                    if mpd_qcopy(q, qq, status) == 0 {
                                        current_block = 1;
                                    } else {
                                        mpd_del(qq);
                                        current_block = 5;
                                    }
                                } else {
                                    current_block = 5;
                                }
                                match current_block {
                                    1 => {}
                                    _ => {
                                        if rr != r {
                                            if mpd_qcopy(r, rr, status) == 0 {
                                                current_block = 1;
                                            } else {
                                                mpd_del(rr);
                                                current_block = 6;
                                            }
                                        } else {
                                            current_block = 6;
                                        }
                                        match current_block {
                                            1 => {}
                                            _ => {
                                                *status = (*status as ::core::ffi::c_uint
                                                    | (workctx.status & MPD_Errors as uint32_t)
                                                        as ::core::ffi::c_uint)
                                                    as uint32_t;
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !qq.is_null() && qq != q {
        mpd_del(qq);
    }
    if !rr.is_null() && rr != r {
        mpd_del(rr);
    }
    mpd_setspecial(q, MPD_POS, MPD_NAN);
    mpd_setspecial(r, MPD_POS, MPD_NAN);
}
unsafe extern "C" fn invroot_schedule_prec(
    mut klist: *mut mpd_ssize_t,
    mut maxprec: mpd_ssize_t,
    mut initprec: mpd_ssize_t,
) -> ::core::ffi::c_int {
    let mut k: mpd_ssize_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    if maxprec >= 3 as mpd_ssize_t && initprec >= 3 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(
            b"maxprec >= 3 && initprec >= 3\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if maxprec <= initprec {
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    k = maxprec;
    loop {
        k = (k + 3 as mpd_ssize_t) / 2 as mpd_ssize_t;
        let fresh0 = i;
        i = i + 1;
        *klist.offset(fresh0 as isize) = k;
        if !(k > initprec) {
            break;
        }
    }
    return i - 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn _invroot_init_approx(mut z: *mut mpd_t, mut vhat: mpd_uint_t) {
    let mut lo: mpd_uint_t = 1000 as mpd_uint_t;
    let mut hi: mpd_uint_t = 10000 as mpd_uint_t;
    let mut a: mpd_uint_t = 0;
    let mut sq: mpd_uint_t = 0;
    if lo.wrapping_mul(lo) <= vhat
        && vhat
            < hi.wrapping_add(1 as mpd_uint_t)
                .wrapping_mul(hi.wrapping_add(1 as mpd_uint_t))
    {
    } else {
        mpd_c2rust_assert_fail(
            b"lo*lo <= vhat && vhat < (hi+1)*(hi+1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    loop {
        a = lo.wrapping_add(hi).wrapping_div(2 as mpd_uint_t);
        sq = a.wrapping_mul(a);
        if vhat >= sq {
            if vhat
                < sq.wrapping_add((2 as mpd_uint_t).wrapping_mul(a))
                    .wrapping_add(1 as mpd_uint_t)
            {
                break;
            }
            lo = a.wrapping_add(1 as mpd_uint_t);
        } else {
            hi = a.wrapping_sub(1 as mpd_uint_t);
        }
    }
    mpd_minalloc(z);
    mpd_clear_flags(z);
    *(*z).data.offset(0 as isize) = (1000000000 as mpd_uint_t).wrapping_div(a);
    (*z).len = 1 as mpd_ssize_t;
    (*z).exp = -(6 as ::core::ffi::c_int) as mpd_ssize_t;
    mpd_setdigits(z);
}
unsafe extern "C" fn _mpd_qinvroot(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workstatus: uint32_t = 0 as uint32_t;
    let mut varcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut z: *mut mpd_t = result;
    let mut v: *mut mpd_t = ::core::ptr::null_mut::<mpd_t>();
    let mut vtmp: mpd_t = mpd_t {
        flags: ((*a).flags as ::core::ffi::c_int & !MPD_DATAFLAGS
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_SHARED_DATA as ::core::ffi::c_int) as uint8_t,
        exp: (*a).exp,
        digits: (*a).digits,
        len: (*a).len,
        alloc: (*a).alloc,
        data: (*a).data,
    };
    let mut s_data: [mpd_uint_t; 64] = [0; 64];
    let mut s: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut s_data as *mut mpd_uint_t,
    };
    let mut t_data: [mpd_uint_t; 64] = [0; 64];
    let mut t: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut t_data as *mut mpd_uint_t,
    };
    let mut one_half_data: [mpd_uint_t; 1] = [5 as mpd_uint_t];
    let mut one_half: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: -(1 as ::core::ffi::c_int) as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut one_half_data as *mut mpd_uint_t,
    };
    let mut three_data: [mpd_uint_t; 1] = [3 as mpd_uint_t];
    let mut three: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut three_data as *mut mpd_uint_t,
    };
    let mut klist: [mpd_ssize_t; 64] = [0; 64];
    let mut ideal_exp: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut adj: mpd_ssize_t = 0;
    let mut tz: mpd_ssize_t = 0;
    let mut maxprec: mpd_ssize_t = 0;
    let mut fracdigits: mpd_ssize_t = 0;
    let mut vhat: mpd_uint_t = 0;
    let mut dummy: mpd_uint_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    ideal_exp = -((*a).exp - ((*a).exp & 1 as mpd_ssize_t)) / 2 as mpd_ssize_t;
    v = &raw mut vtmp;
    if result == a as *mut mpd_t {
        v = mpd_qncopy(a);
        if v.is_null() {
            mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
            return;
        }
    }
    if (*v).digits + (*v).exp & 1 as mpd_ssize_t != 0 {
        fracdigits = (*v).digits - 1 as mpd_ssize_t;
        (*v).exp = -fracdigits;
        n = if (*v).digits > 7 as mpd_ssize_t {
            7 as ::core::ffi::c_int
        } else {
            (*v).digits as ::core::ffi::c_int
        };
        _mpd_get_msdigits(&raw mut dummy, &raw mut vhat, v, n as ::core::ffi::c_uint);
        if n < 7 as ::core::ffi::c_int {
            vhat = vhat.wrapping_mul(
                *(&raw const mpd_pow10 as *const mpd_uint_t)
                    .offset((7 as ::core::ffi::c_int - n) as isize),
            );
        }
    } else {
        fracdigits = (*v).digits - 2 as mpd_ssize_t;
        (*v).exp = -fracdigits;
        n = if (*v).digits > 8 as mpd_ssize_t {
            8 as ::core::ffi::c_int
        } else {
            (*v).digits as ::core::ffi::c_int
        };
        _mpd_get_msdigits(&raw mut dummy, &raw mut vhat, v, n as ::core::ffi::c_uint);
        if n < 8 as ::core::ffi::c_int {
            vhat = vhat.wrapping_mul(
                *(&raw const mpd_pow10 as *const mpd_uint_t)
                    .offset((8 as ::core::ffi::c_int - n) as isize),
            );
        }
    }
    adj = ((*a).exp - (*v).exp) / 2 as mpd_ssize_t;
    _invroot_init_approx(z, vhat);
    mpd_maxcontext(&raw mut maxcontext);
    mpd_maxcontext(&raw mut varcontext);
    varcontext.round = MPD_ROUND_TRUNC as ::core::ffi::c_int;
    maxprec = (*ctx).prec + 1 as mpd_ssize_t;
    i = invroot_schedule_prec(
        &raw mut klist as *mut mpd_ssize_t,
        maxprec,
        3 as mpd_ssize_t,
    );
    while i >= 0 as ::core::ffi::c_int {
        varcontext.prec = 2 as mpd_ssize_t * klist[i as usize] + 2 as mpd_ssize_t;
        mpd_qmul(&raw mut s, z, z, &raw mut maxcontext, &raw mut workstatus);
        if (*v).digits > varcontext.prec {
            shift = (*v).digits - varcontext.prec;
            mpd_qshiftr(&raw mut t, v, shift, &raw mut workstatus);
            t.exp += shift;
            mpd_qmul(
                &raw mut t,
                &raw mut t,
                &raw mut s,
                &raw mut varcontext,
                &raw mut workstatus,
            );
        } else {
            mpd_qmul(
                &raw mut t,
                v,
                &raw mut s,
                &raw mut varcontext,
                &raw mut workstatus,
            );
        }
        mpd_qsub(
            &raw mut t,
            &raw mut three,
            &raw mut t,
            &raw mut maxcontext,
            &raw mut workstatus,
        );
        mpd_qmul(z, z, &raw mut t, &raw mut varcontext, &raw mut workstatus);
        mpd_qmul(
            z,
            z,
            &raw mut one_half,
            &raw mut maxcontext,
            &raw mut workstatus,
        );
        i -= 1;
    }
    (*z).exp -= adj;
    tz = mpd_trail_zeros(result);
    shift = ideal_exp - (*result).exp;
    shift = if tz > shift { shift } else { tz };
    if shift > 0 as mpd_ssize_t {
        mpd_qshiftr_inplace(result, shift);
        (*result).exp += shift;
    }
    mpd_del(&raw mut s);
    mpd_del(&raw mut t);
    if v != &raw mut vtmp {
        mpd_del(v);
    }
    *status = (*status as ::core::ffi::c_uint
        | (workstatus & MPD_Errors as uint32_t) as ::core::ffi::c_uint) as uint32_t;
    *status = (*status as ::core::ffi::c_uint | (MPD_Rounded | MPD_Inexact)) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qinvroot(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut workctx: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        if mpd_isnegative(a) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        _settriple(result, MPD_POS, 0 as mpd_uint_t, mpd_etiny(ctx));
        *status = (*status as ::core::ffi::c_uint | MPD_Clamped) as uint32_t;
        return;
    }
    if mpd_iszero(a) != 0 {
        mpd_setspecial(result, mpd_sign(a), MPD_INF);
        *status = (*status as ::core::ffi::c_uint | MPD_Division_by_zero) as uint32_t;
        return;
    }
    if mpd_isnegative(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    mpd_workcontext(&raw mut workctx, ctx);
    workctx.prec += 2 as mpd_ssize_t;
    workctx.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    _mpd_qinvroot(result, a, &raw mut workctx, status);
    mpd_qfinalize(result, ctx, status);
}
unsafe extern "C" fn _mpd_qsqrt(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut maxcontext: mpd_context_t = mpd_context_t {
        prec: 0,
        emax: 0,
        emin: 0,
        traps: 0,
        status: 0,
        newtrap: 0,
        round: 0,
        clamp: 0,
        allcr: 0,
    };
    let mut c_data: [mpd_uint_t; 64] = [0; 64];
    let mut c: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut c_data as *mut mpd_uint_t,
    };
    let mut q_data: [mpd_uint_t; 64] = [0; 64];
    let mut q: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut q_data as *mut mpd_uint_t,
    };
    let mut r_data: [mpd_uint_t; 64] = [0; 64];
    let mut r: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut r_data as *mut mpd_uint_t,
    };
    let mut two_data: [mpd_uint_t; 1] = [2 as mpd_uint_t];
    let mut two: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_CONST_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 1 as mpd_ssize_t,
        len: 1 as mpd_ssize_t,
        alloc: 1 as mpd_ssize_t,
        data: &raw mut two_data as *mut mpd_uint_t,
    };
    let mut prec: mpd_ssize_t = 0;
    let mut ideal_exp: mpd_ssize_t = 0;
    let mut l: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut exact: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ideal_exp = ((*a).exp - ((*a).exp & 1 as mpd_ssize_t)) / 2 as mpd_ssize_t;
    if mpd_isspecial(a) != 0 {
        if mpd_qcheck_nan(result, a, ctx, status) != 0 {
            return;
        }
        if mpd_isnegative(a) != 0 {
            mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        mpd_setspecial(result, MPD_POS, MPD_INF);
        return;
    }
    if mpd_iszero(a) != 0 {
        _settriple(result, mpd_sign(a), 0 as mpd_uint_t, ideal_exp);
        mpd_qfinalize(result, ctx, status);
        return;
    }
    if mpd_isnegative(a) != 0 {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    mpd_maxcontext(&raw mut maxcontext);
    prec = (*ctx).prec + 1 as mpd_ssize_t;
    if mpd_qcopy(&raw mut c, a, status) == 0 {
        current_block = 1;
    } else {
        c.exp = 0 as mpd_ssize_t;
        if (*a).exp & 1 as mpd_ssize_t != 0 {
            if mpd_qshiftl(&raw mut c, &raw mut c, 1 as mpd_ssize_t, status) == 0 {
                current_block = 1;
            } else {
                l = ((*a).digits >> 1 as ::core::ffi::c_int) + 1 as mpd_ssize_t;
                current_block = 2;
            }
        } else {
            l = (*a).digits + 1 as mpd_ssize_t >> 1 as ::core::ffi::c_int;
            current_block = 2;
        }
        match current_block {
            1 => {}
            _ => {
                shift = prec - l;
                if shift >= 0 as mpd_ssize_t {
                    if mpd_qshiftl(&raw mut c, &raw mut c, 2 as mpd_ssize_t * shift, status) == 0 {
                        current_block = 1;
                    } else {
                        exact = 1 as ::core::ffi::c_int;
                        current_block = 3;
                    }
                } else {
                    exact = (mpd_qshiftr_inplace(
                        &raw mut c,
                        -(2 as ::core::ffi::c_int) as mpd_ssize_t * shift,
                    ) == 0) as ::core::ffi::c_int;
                    current_block = 3;
                }
                match current_block {
                    1 => {}
                    _ => {
                        ideal_exp -= shift;
                        if mpd_qshiftl(result, &raw const one, prec, status) == 0 {
                            current_block = 1;
                        } else {
                            loop {
                                _mpd_qdivmod(
                                    &raw mut q,
                                    &raw mut r,
                                    &raw mut c,
                                    result,
                                    &raw mut maxcontext,
                                    &raw mut maxcontext.status,
                                );
                                if mpd_isspecial(result) != 0 || mpd_isspecial(&raw mut q) != 0 {
                                    mpd_seterror(
                                        result,
                                        maxcontext.status & MPD_Errors as uint32_t,
                                        status,
                                    );
                                    current_block = 4;
                                    break;
                                } else {
                                    if _mpd_cmp(result, &raw mut q) <= 0 as ::core::ffi::c_int {
                                        current_block = 5;
                                        break;
                                    }
                                    _mpd_qadd_exact(
                                        result,
                                        result,
                                        &raw mut q,
                                        &raw mut maxcontext,
                                        &raw mut maxcontext.status,
                                    );
                                    if mpd_isspecial(result) != 0 {
                                        mpd_seterror(
                                            result,
                                            maxcontext.status & MPD_Errors as uint32_t,
                                            status,
                                        );
                                        current_block = 4;
                                        break;
                                    } else {
                                        _mpd_qdivmod(
                                            result,
                                            &raw mut r,
                                            result,
                                            &raw mut two,
                                            &raw mut maxcontext,
                                            &raw mut maxcontext.status,
                                        );
                                    }
                                }
                            }
                            match current_block {
                                4 => {}
                                _ => {
                                    if exact != 0 {
                                        _mpd_qmul_exact(
                                            &raw mut r,
                                            result,
                                            result,
                                            &raw mut maxcontext,
                                            &raw mut maxcontext.status,
                                        );
                                        if mpd_isspecial(&raw mut r) != 0 {
                                            mpd_seterror(
                                                result,
                                                maxcontext.status & MPD_Errors as uint32_t,
                                                status,
                                            );
                                            current_block = 4;
                                        } else {
                                            exact = (_mpd_cmp(&raw mut r, &raw mut c)
                                                == 0 as ::core::ffi::c_int)
                                                as ::core::ffi::c_int;
                                            current_block = 6;
                                        }
                                    } else {
                                        current_block = 6;
                                    }
                                    match current_block {
                                        4 => {}
                                        _ => {
                                            if exact != 0 {
                                                if shift >= 0 as mpd_ssize_t {
                                                    mpd_qshiftr_inplace(result, shift);
                                                    current_block = 7;
                                                } else if mpd_qshiftl(
                                                    result, result, -shift, status,
                                                ) == 0
                                                {
                                                    current_block = 1;
                                                } else {
                                                    current_block = 7;
                                                }
                                                match current_block {
                                                    1 => {}
                                                    _ => {
                                                        ideal_exp += shift;
                                                        current_block = 8;
                                                    }
                                                }
                                            } else {
                                                let mut lsd: ::core::ffi::c_int =
                                                    mpd_lsd(*(*result).data.offset(0 as isize))
                                                        as ::core::ffi::c_int;
                                                if lsd == 0 as ::core::ffi::c_int
                                                    || lsd == 5 as ::core::ffi::c_int
                                                {
                                                    let ref mut fresh0 =
                                                        *(*result).data.offset(0 as isize);
                                                    *fresh0 =
                                                        (*fresh0).wrapping_add(1 as mpd_uint_t);
                                                }
                                                current_block = 8;
                                            }
                                            match current_block {
                                                1 => {}
                                                _ => {
                                                    (*result).exp = ideal_exp;
                                                    current_block = 4;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        1 => {
            mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
        }
        _ => {}
    }
    mpd_del(&raw mut c);
    mpd_del(&raw mut q);
    mpd_del(&raw mut r);
    mpd_workcontext(&raw mut maxcontext, ctx);
    maxcontext.round = MPD_ROUND_HALF_EVEN as ::core::ffi::c_int;
    mpd_qfinalize(result, &raw mut maxcontext, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qsqrt(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut aa_data: [mpd_uint_t; 64] = [0; 64];
    let mut aa: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut aa_data as *mut mpd_uint_t,
    };
    let mut xstatus: uint32_t = 0 as uint32_t;
    if result == a as *mut mpd_t {
        if mpd_qcopy(&raw mut aa, a, status) == 0 {
            mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
            current_block = 1;
        } else {
            a = &raw mut aa;
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            _mpd_qsqrt(result, a, ctx, &raw mut xstatus);
            if xstatus & (MPD_Malloc_error as uint32_t | MPD_Division_impossible as uint32_t) != 0 {
                let mut ystatus: uint32_t = 0 as uint32_t;
                let mut workctx: mpd_context_t = mpd_context_t {
                    prec: 0,
                    emax: 0,
                    emin: 0,
                    traps: 0,
                    status: 0,
                    newtrap: 0,
                    round: 0,
                    clamp: 0,
                    allcr: 0,
                };
                mpd_workcontext(&raw mut workctx, ctx);
                workctx.prec = (*a).digits;
                if workctx.prec >= (*ctx).prec {
                    *status = (*status as ::core::ffi::c_uint
                        | (xstatus | MPD_Errors as uint32_t) as ::core::ffi::c_uint)
                        as uint32_t;
                } else {
                    _mpd_qsqrt(result, a, &raw mut workctx, &raw mut ystatus);
                    if ystatus != 0 as uint32_t {
                        ystatus = *status | (xstatus | ystatus) & MPD_Errors as uint32_t;
                        mpd_seterror(result, ystatus, status);
                    }
                }
            } else {
                *status |= xstatus;
            }
        }
        _ => {}
    }
    mpd_del(&raw mut aa);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sizeinbase(mut a: *const mpd_t, mut base: uint32_t) -> size_t {
    let mut x: ::core::ffi::c_double = 0.;
    let mut digits: size_t = 0;
    let mut upper_bound: ::core::ffi::c_double = 0.;
    if mpd_isinteger(a) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"mpd_isinteger(a)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if base >= 2 as uint32_t {
    } else {
        mpd_c2rust_assert_fail(b"base >= 2\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_iszero(a) != 0 {
        return 1 as size_t;
    }
    digits = ((*a).digits + (*a).exp) as size_t;
    func_count += 1;
    if func_count == func_fail {
        return SIZE_MAX as size_t;
    }
    if digits as u64 > 2711437152599294 as u64 {
        return SIZE_MAX as size_t;
    }
    upper_bound =
        ((1 as u64) << 53 as ::core::ffi::c_int).wrapping_sub(1 as u64) as ::core::ffi::c_double;
    x = digits as ::core::ffi::c_double / log10(base as ::core::ffi::c_double);
    return if x > upper_bound {
        SIZE_MAX as size_t
    } else {
        (x as size_t).wrapping_add(1 as size_t)
    };
}
unsafe extern "C" fn _mpd_importsize(mut srclen: size_t, mut base: uint32_t) -> mpd_ssize_t {
    let mut x: ::core::ffi::c_double = 0.;
    let mut upper_bound: ::core::ffi::c_double = 0.;
    if srclen > 0 as size_t {
    } else {
        mpd_c2rust_assert_fail(b"srclen > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if base >= 2 as uint32_t {
    } else {
        mpd_c2rust_assert_fail(b"base >= 2\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if srclen as u64 > (1 as u64) << 53 as ::core::ffi::c_int {
        return MPD_SSIZE_MAX as mpd_ssize_t;
    }
    if (1 as u64) << 53 as ::core::ffi::c_int <= 105263157894736842 as u64 {
    } else {
        mpd_c2rust_assert_fail(
            b"(1ULL<<53) <= MPD_MAXIMPORT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    upper_bound =
        ((1 as u64) << 53 as ::core::ffi::c_int).wrapping_sub(1 as u64) as ::core::ffi::c_double;
    x = srclen as ::core::ffi::c_double
        * (log10(base as ::core::ffi::c_double) / MPD_RDIGITS as ::core::ffi::c_double);
    func_count += 1;
    if func_count == func_fail {
        return 1 as mpd_ssize_t;
    } else {
        return if x > upper_bound {
            MPD_SSIZE_MAX as mpd_ssize_t
        } else {
            x as mpd_ssize_t + 1 as mpd_ssize_t
        };
    };
}
unsafe extern "C" fn mpd_resize_u16(mut w: *mut *mut uint16_t, mut nmemb: size_t) -> uint8_t {
    let mut err: uint8_t = 0 as uint8_t;
    *w = mpd_realloc(
        *w as *mut ::core::ffi::c_void,
        nmemb as mpd_size_t,
        ::core::mem::size_of::<uint16_t>() as mpd_size_t,
        &raw mut err,
    ) as *mut uint16_t;
    return (err == 0) as ::core::ffi::c_int as uint8_t;
}
unsafe extern "C" fn mpd_resize_u32(mut w: *mut *mut uint32_t, mut nmemb: size_t) -> uint8_t {
    let mut err: uint8_t = 0 as uint8_t;
    *w = mpd_realloc(
        *w as *mut ::core::ffi::c_void,
        nmemb as mpd_size_t,
        ::core::mem::size_of::<uint32_t>() as mpd_size_t,
        &raw mut err,
    ) as *mut uint32_t;
    return (err == 0) as ::core::ffi::c_int as uint8_t;
}
unsafe extern "C" fn _baseconv_to_u16(
    mut w: *mut *mut uint16_t,
    mut wlen: size_t,
    mut wbase: mpd_uint_t,
    mut u: *mut mpd_uint_t,
    mut ulen: mpd_ssize_t,
) -> size_t {
    let mut n: size_t = 0 as size_t;
    if wlen > 0 as size_t && ulen > 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(
            b"wlen > 0 && ulen > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if wbase <= ((1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int) as mpd_uint_t {
    } else {
        mpd_c2rust_assert_fail(b"wbase <= (1U<<16)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    loop {
        if n >= wlen {
            if mpd_resize_u16(w, n.wrapping_add(1 as size_t)) == 0 {
                return SIZE_MAX as size_t;
            }
            wlen = n.wrapping_add(1 as size_t);
        }
        let fresh0 = n;
        n = n.wrapping_add(1);
        *(*w).offset(fresh0 as isize) = _mpd_shortdiv(u, u, ulen as mpd_size_t, wbase) as uint16_t;
        ulen = _mpd_real_size(u, ulen);
        if !(*u.offset((ulen - 1 as mpd_ssize_t) as isize) != 0 as mpd_uint_t) {
            break;
        }
    }
    return n;
}
unsafe extern "C" fn _coeff_from_u16(
    mut w: *mut mpd_t,
    mut wlen: mpd_ssize_t,
    mut u: *const mpd_uint_t,
    mut ulen: size_t,
    mut ubase: uint32_t,
    mut status: *mut uint32_t,
) -> size_t {
    let mut n: mpd_ssize_t = 0 as mpd_ssize_t;
    let mut carry: mpd_uint_t = 0;
    if wlen > 0 as mpd_ssize_t && ulen > 0 as size_t {
    } else {
        mpd_c2rust_assert_fail(
            b"wlen > 0 && ulen > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if ubase <= (1 as uint32_t) << 16 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(b"ubase <= (1U<<16)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    ulen = ulen.wrapping_sub(1);
    let fresh0 = n;
    n = n + 1;
    *(*w).data.offset(fresh0 as isize) = *u.offset(ulen as isize);
    loop {
        ulen = ulen.wrapping_sub(1);
        if !(ulen != SIZE_MAX as size_t) {
            break;
        }
        carry = _mpd_shortmul_c((*w).data, (*w).data, n as mpd_size_t, ubase as mpd_uint_t);
        if carry != 0 {
            if n >= wlen {
                if mpd_qresize(w, n + 1 as mpd_ssize_t, status) == 0 {
                    return SIZE_MAX as size_t;
                }
                wlen = n + 1 as mpd_ssize_t;
            }
            let fresh1 = n;
            n = n + 1;
            *(*w).data.offset(fresh1 as isize) = carry;
        }
        carry = _mpd_shortadd((*w).data, n as mpd_size_t, *u.offset(ulen as isize));
        if carry != 0 {
            if n >= wlen {
                if mpd_qresize(w, n + 1 as mpd_ssize_t, status) == 0 {
                    return SIZE_MAX as size_t;
                }
                wlen = n + 1 as mpd_ssize_t;
            }
            let fresh2 = n;
            n = n + 1;
            *(*w).data.offset(fresh2 as isize) = carry;
        }
    }
    return n as size_t;
}
unsafe extern "C" fn _baseconv_to_smaller(
    mut w: *mut *mut uint32_t,
    mut wlen: size_t,
    mut wbase: uint32_t,
    mut u: *mut mpd_uint_t,
    mut ulen: mpd_ssize_t,
    mut ubase: mpd_uint_t,
) -> size_t {
    let mut n: size_t = 0 as size_t;
    if wlen > 0 as size_t && ulen > 0 as mpd_ssize_t {
    } else {
        mpd_c2rust_assert_fail(
            b"wlen > 0 && ulen > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if (wbase as mpd_uint_t) < ubase {
    } else {
        mpd_c2rust_assert_fail(b"wbase < ubase\0" as *const u8 as *const ::core::ffi::c_char);
    };
    loop {
        if n >= wlen {
            if mpd_resize_u32(w, n.wrapping_add(1 as size_t)) == 0 {
                return SIZE_MAX as size_t;
            }
            wlen = n.wrapping_add(1 as size_t);
        }
        let fresh0 = n;
        n = n.wrapping_add(1);
        *(*w).offset(fresh0 as isize) =
            _mpd_shortdiv_b(u, u, ulen as mpd_size_t, wbase as mpd_uint_t, ubase) as uint32_t;
        ulen = _mpd_real_size(u, ulen);
        if !(*u.offset((ulen - 1 as mpd_ssize_t) as isize) != 0 as mpd_uint_t) {
            break;
        }
    }
    return n;
}
unsafe extern "C" fn _coeff_from_smaller_base(
    mut w: *mut mpd_t,
    mut wlen: mpd_ssize_t,
    mut wbase: mpd_uint_t,
    mut u: *const uint32_t,
    mut ulen: size_t,
    mut ubase: mpd_uint_t,
    mut status: *mut uint32_t,
) -> size_t {
    let mut n: mpd_ssize_t = 0 as mpd_ssize_t;
    let mut carry: mpd_uint_t = 0;
    if wlen > 0 as mpd_ssize_t && ulen > 0 as size_t {
    } else {
        mpd_c2rust_assert_fail(
            b"wlen > 0 && ulen > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if wbase > ubase {
    } else {
        mpd_c2rust_assert_fail(b"wbase > ubase\0" as *const u8 as *const ::core::ffi::c_char);
    };
    ulen = ulen.wrapping_sub(1);
    let fresh0 = n;
    n = n + 1;
    *(*w).data.offset(fresh0 as isize) = *u.offset(ulen as isize) as mpd_uint_t;
    loop {
        ulen = ulen.wrapping_sub(1);
        if !(ulen != SIZE_MAX as size_t) {
            break;
        }
        carry = _mpd_shortmul_b((*w).data, (*w).data, n as mpd_size_t, ubase, wbase);
        if carry != 0 {
            if n >= wlen {
                if mpd_qresize(w, n + 1 as mpd_ssize_t, status) == 0 {
                    return SIZE_MAX as size_t;
                }
                wlen = n + 1 as mpd_ssize_t;
            }
            let fresh1 = n;
            n = n + 1;
            *(*w).data.offset(fresh1 as isize) = carry;
        }
        carry = _mpd_shortadd_b(
            (*w).data,
            n as mpd_size_t,
            *u.offset(ulen as isize) as mpd_uint_t,
            wbase,
        );
        if carry != 0 {
            if n >= wlen {
                if mpd_qresize(w, n + 1 as mpd_ssize_t, status) == 0 {
                    return SIZE_MAX as size_t;
                }
                wlen = n + 1 as mpd_ssize_t;
            }
            let fresh2 = n;
            n = n + 1;
            *(*w).data.offset(fresh2 as isize) = carry;
        }
    }
    return n as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qexport_u16(
    mut rdata: *mut *mut uint16_t,
    mut rlen: size_t,
    mut rbase: uint32_t,
    mut src: *const mpd_t,
    mut status: *mut uint32_t,
) -> size_t {
    let mut current_block: u64;
    let mut tsrc_data: [mpd_uint_t; 64] = [0; 64];
    let mut tsrc: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tsrc_data as *mut mpd_uint_t,
    };
    let mut alloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: size_t = 0;
    if rbase <= (1 as uint32_t) << 16 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(b"rbase <= (1U<<16)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_isspecial(src) != 0 || _mpd_isint(src) == 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return SIZE_MAX as size_t;
    }
    if (*rdata).is_null() {
        rlen = mpd_sizeinbase(src, rbase);
        if rlen == SIZE_MAX as size_t {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return SIZE_MAX as size_t;
        }
        *rdata = mpd_alloc(
            rlen as mpd_size_t,
            ::core::mem::size_of::<uint16_t>() as mpd_size_t,
        ) as *mut uint16_t;
        if (*rdata).is_null() {
            current_block = 1;
        } else {
            alloc = 1 as ::core::ffi::c_int;
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            if mpd_iszero(src) != 0 {
                **rdata = 0 as uint16_t;
                return 1 as size_t;
            }
            if (*src).exp >= 0 as mpd_ssize_t {
                if mpd_qshiftl(&raw mut tsrc, src, (*src).exp, status) == 0 {
                    current_block = 1;
                } else {
                    current_block = 3;
                }
            } else if mpd_qshiftr(&raw mut tsrc, src, -(*src).exp, status)
                == MPD_UINT_MAX as mpd_uint_t
            {
                current_block = 1;
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    n = _baseconv_to_u16(rdata, rlen, rbase as mpd_uint_t, tsrc.data, tsrc.len);
                    if n == SIZE_MAX as size_t {
                        current_block = 1;
                    } else {
                        current_block = 4;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        1 => {
            if alloc != 0 {
                mpd_free.expect("non-null function pointer")(*rdata as *mut ::core::ffi::c_void);
                *rdata = ::core::ptr::null_mut::<uint16_t>();
            }
            n = SIZE_MAX as size_t;
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        }
        _ => {}
    }
    mpd_del(&raw mut tsrc);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qexport_u32(
    mut rdata: *mut *mut uint32_t,
    mut rlen: size_t,
    mut rbase: uint32_t,
    mut src: *const mpd_t,
    mut status: *mut uint32_t,
) -> size_t {
    let mut current_block: u64;
    let mut tsrc_data: [mpd_uint_t; 64] = [0; 64];
    let mut tsrc: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut tsrc_data as *mut mpd_uint_t,
    };
    let mut alloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: size_t = 0;
    if mpd_isspecial(src) != 0 || _mpd_isint(src) == 0 {
        *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
        return SIZE_MAX as size_t;
    }
    if (*rdata).is_null() {
        rlen = mpd_sizeinbase(src, rbase);
        if rlen == SIZE_MAX as size_t {
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return SIZE_MAX as size_t;
        }
        *rdata = mpd_alloc(
            rlen as mpd_size_t,
            ::core::mem::size_of::<uint32_t>() as mpd_size_t,
        ) as *mut uint32_t;
        if (*rdata).is_null() {
            current_block = 1;
        } else {
            alloc = 1 as ::core::ffi::c_int;
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            if mpd_iszero(src) != 0 {
                **rdata = 0 as uint32_t;
                return 1 as size_t;
            }
            if (*src).exp >= 0 as mpd_ssize_t {
                if mpd_qshiftl(&raw mut tsrc, src, (*src).exp, status) == 0 {
                    current_block = 1;
                } else {
                    current_block = 3;
                }
            } else if mpd_qshiftr(&raw mut tsrc, src, -(*src).exp, status)
                == MPD_UINT_MAX as mpd_uint_t
            {
                current_block = 1;
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    n = _baseconv_to_smaller(
                        rdata,
                        rlen,
                        rbase,
                        tsrc.data,
                        tsrc.len,
                        MPD_RADIX as mpd_uint_t,
                    );
                    if n == SIZE_MAX as size_t {
                        current_block = 1;
                    } else {
                        current_block = 4;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        1 => {
            if alloc != 0 {
                mpd_free.expect("non-null function pointer")(*rdata as *mut ::core::ffi::c_void);
                *rdata = ::core::ptr::null_mut::<uint32_t>();
            }
            n = SIZE_MAX as size_t;
            *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        }
        _ => {}
    }
    mpd_del(&raw mut tsrc);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qimport_u16(
    mut result: *mut mpd_t,
    mut srcdata: *const uint16_t,
    mut srclen: size_t,
    mut srcsign: uint8_t,
    mut srcbase: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut usrc: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut rlen: mpd_ssize_t = 0;
    let mut n: size_t = 0;
    if srclen > 0 as size_t {
    } else {
        mpd_c2rust_assert_fail(b"srclen > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if srcbase <= (1 as uint32_t) << 16 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(b"srcbase <= (1U<<16)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    rlen = _mpd_importsize(srclen, srcbase);
    if rlen == MPD_SSIZE_MAX as mpd_ssize_t {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    usrc = mpd_alloc(srclen, ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t) as *mut mpd_uint_t;
    if usrc.is_null() {
        mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
        return;
    }
    n = 0 as size_t;
    while n < srclen {
        *usrc.offset(n as isize) = *srcdata.offset(n as isize) as mpd_uint_t;
        n = n.wrapping_add(1);
    }
    if !(mpd_qresize(result, rlen, status) == 0) {
        n = _coeff_from_u16(result, rlen, usrc, srclen, srcbase, status);
        if !(n == SIZE_MAX as size_t) {
            mpd_set_flags(result, srcsign);
            (*result).exp = 0 as mpd_ssize_t;
            (*result).len = n as mpd_ssize_t;
            mpd_setdigits(result);
            mpd_qresize(result, (*result).len, status);
            mpd_qfinalize(result, ctx, status);
        }
    }
    mpd_free.expect("non-null function pointer")(usrc as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qimport_u32(
    mut result: *mut mpd_t,
    mut srcdata: *const uint32_t,
    mut srclen: size_t,
    mut srcsign: uint8_t,
    mut srcbase: uint32_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut rlen: mpd_ssize_t = 0;
    let mut n: size_t = 0;
    if srclen > 0 as size_t {
    } else {
        mpd_c2rust_assert_fail(b"srclen > 0\0" as *const u8 as *const ::core::ffi::c_char);
    };
    rlen = _mpd_importsize(srclen, srcbase);
    if rlen == MPD_SSIZE_MAX as mpd_ssize_t {
        mpd_seterror(result, MPD_Invalid_operation as uint32_t, status);
        return;
    }
    if mpd_qresize(result, rlen, status) == 0 {
        return;
    }
    n = _coeff_from_smaller_base(
        result,
        rlen,
        MPD_RADIX as mpd_uint_t,
        srcdata,
        srclen,
        srcbase as mpd_uint_t,
        status,
    );
    if n == SIZE_MAX as size_t {
        return;
    }
    mpd_set_flags(result, srcsign);
    (*result).exp = 0 as mpd_ssize_t;
    (*result).len = n as mpd_ssize_t;
    mpd_setdigits(result);
    mpd_qresize(result, (*result).len, status);
    mpd_qfinalize(result, ctx, status);
}
unsafe extern "C" fn _set_coeff(
    mut data: *mut uint64_t,
    mut hi: uint64_t,
    mut lo: uint64_t,
) -> mpd_ssize_t {
    let mut d: __uint128_t =
        ((hi as __uint128_t) << 64 as ::core::ffi::c_int).wrapping_add(lo as __uint128_t);
    let mut q: __uint128_t = 0;
    let mut r: __uint128_t = 0;
    q = d.wrapping_div(MPD_RADIX as __uint128_t);
    r = d.wrapping_rem(MPD_RADIX as __uint128_t);
    *data.offset(0 as isize) = r as uint64_t;
    d = q;
    q = d.wrapping_div(MPD_RADIX as __uint128_t);
    r = d.wrapping_rem(MPD_RADIX as __uint128_t);
    *data.offset(1 as isize) = r as uint64_t;
    d = q;
    q = d.wrapping_div(MPD_RADIX as __uint128_t);
    r = d.wrapping_rem(MPD_RADIX as __uint128_t);
    *data.offset(2 as isize) = r as uint64_t;
    if q != 0 as __uint128_t {
        abort();
    }
    return (if *data.offset(2 as isize) != 0 as uint64_t {
        3 as ::core::ffi::c_int
    } else if *data.offset(1 as isize) != 0 as uint64_t {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as mpd_ssize_t;
}
unsafe extern "C" fn _set_uint128_coeff_exp(
    mut result: *mut mpd_t,
    mut hi: uint64_t,
    mut lo: uint64_t,
    mut exp: mpd_ssize_t,
) -> ::core::ffi::c_int {
    let mut data: [mpd_uint_t; 5] = [0 as mpd_uint_t, 0, 0, 0, 0];
    let mut status: uint32_t = 0 as uint32_t;
    let mut len: mpd_ssize_t = 0;
    len = _set_coeff(&raw mut data as *mut uint64_t, hi, lo);
    if mpd_qresize(result, len, &raw mut status) == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: mpd_ssize_t = 0 as mpd_ssize_t;
    while i < len {
        *(*result).data.offset(i as isize) = data[i as usize];
        i += 1;
    }
    (*result).exp = exp;
    (*result).len = len;
    mpd_setdigits(result);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_from_uint128_triple(
    mut result: *mut mpd_t,
    mut triple: *const mpd_uint128_triple_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    static mut maxcontext: mpd_context_t = mpd_context_t {
        prec: MPD_MAX_PREC as mpd_ssize_t,
        emax: MPD_MAX_EMAX as mpd_ssize_t,
        emin: MPD_MIN_EMIN as mpd_ssize_t,
        traps: MPD_Traps as uint32_t,
        status: 0 as uint32_t,
        newtrap: 0 as uint32_t,
        round: MPD_ROUND_HALF_EVEN as ::core::ffi::c_int,
        clamp: 0 as ::core::ffi::c_int,
        allcr: 1 as ::core::ffi::c_int,
    };
    let tag: mpd_triple_class = (*triple).tag;
    let sign: uint8_t = (*triple).sign;
    let hi: uint64_t = (*triple).hi;
    let lo: uint64_t = (*triple).lo;
    let mut exp: mpd_ssize_t = 0;
    exp = (*triple).exp;
    match tag as ::core::ffi::c_uint {
        2 | 3 => {
            if sign as ::core::ffi::c_int > 1 as ::core::ffi::c_int || exp != 0 as mpd_ssize_t {
                current_block = 1;
            } else {
                let flags: uint8_t = (if tag as ::core::ffi::c_uint
                    == MPD_TRIPLE_QNAN as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    MPD_NAN as ::core::ffi::c_int
                } else {
                    MPD_SNAN as ::core::ffi::c_int
                }) as uint8_t;
                mpd_setspecial(result, sign, flags);
                if hi == 0 as uint64_t && lo == 0 as uint64_t {
                    return 0 as ::core::ffi::c_int;
                }
                if _set_uint128_coeff_exp(result, hi, lo, exp) < 0 as ::core::ffi::c_int {
                    current_block = 2;
                } else {
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        1 => {
            if sign as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                || hi != 0 as uint64_t
                || lo != 0 as uint64_t
                || exp != 0 as mpd_ssize_t
            {
                current_block = 1;
            } else {
                mpd_setspecial(result, sign, MPD_INF);
                return 0 as ::core::ffi::c_int;
            }
        }
        0 => {
            if sign as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                current_block = 1;
            } else {
                let flags_0: uint8_t = (if sign as ::core::ffi::c_int != 0 {
                    MPD_NEG as ::core::ffi::c_int
                } else {
                    MPD_POS as ::core::ffi::c_int
                }) as uint8_t;
                mpd_set_flags(result, flags_0);
                if exp > MPD_EXP_INF as mpd_ssize_t {
                    exp = MPD_EXP_INF as mpd_ssize_t;
                }
                if exp == MPD_SSIZE_MIN as mpd_ssize_t {
                    exp = (MPD_SSIZE_MIN + 1 as i64) as mpd_ssize_t;
                }
                if _set_uint128_coeff_exp(result, hi, lo, exp) < 0 as ::core::ffi::c_int {
                    current_block = 2;
                } else {
                    let mut workstatus: uint32_t = 0 as uint32_t;
                    mpd_qfinalize(result, &raw const maxcontext, &raw mut workstatus);
                    if workstatus
                        & (MPD_Inexact as uint32_t
                            | MPD_Rounded as uint32_t
                            | MPD_Clamped as uint32_t)
                        != 0
                    {
                        current_block = 1;
                    } else {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        _ => {
            current_block = 1;
        }
    }
    match current_block {
        2 => {
            mpd_seterror(result, MPD_Malloc_error as uint32_t, status);
            return -(1 as ::core::ffi::c_int);
        }
        _ => {
            mpd_seterror(result, MPD_Conversion_syntax as uint32_t, status);
            return -(1 as ::core::ffi::c_int);
        }
    };
}
unsafe extern "C" fn _get_coeff(mut hi: *mut uint64_t, mut lo: *mut uint64_t, mut a: *const mpd_t) {
    let mut u128: __uint128_t = 0 as __uint128_t;
    let mut current_block_3: u64;
    match (*a).len {
        3 => {
            u128 = *(*a).data.offset(2 as isize) as __uint128_t;
            current_block_3 = 1;
        }
        2 => {
            current_block_3 = 1;
        }
        1 => {
            current_block_3 = 2;
        }
        _ => {
            abort();
        }
    }
    match current_block_3 {
        1 => {
            u128 = u128
                .wrapping_mul(MPD_RADIX as __uint128_t)
                .wrapping_add(*(*a).data.offset(1 as isize) as __uint128_t);
        }
        _ => {}
    }
    u128 = u128
        .wrapping_mul(MPD_RADIX as __uint128_t)
        .wrapping_add(*(*a).data.offset(0 as isize) as __uint128_t);
    *hi = (u128 >> 64 as ::core::ffi::c_int) as uint64_t;
    *lo = u128 as uint64_t;
}
unsafe extern "C" fn _coeff_as_uint128(
    mut hi: *mut uint64_t,
    mut lo: *mut uint64_t,
    mut a: *const mpd_t,
) -> mpd_triple_class {
    static mut uint128_max_data: [mpd_uint_t; 3] = [
        3374607431768211455 as u64,
        4028236692093846346 as u64,
        3 as u64,
    ];
    static mut uint128_max: mpd_t = unsafe {
        mpd_t {
            flags: (MPD_STATIC as ::core::ffi::c_int | MPD_CONST_DATA as ::core::ffi::c_int)
                as uint8_t,
            exp: 0 as mpd_ssize_t,
            digits: 39 as mpd_ssize_t,
            len: 3 as mpd_ssize_t,
            alloc: 3 as mpd_ssize_t,
            data: &raw const uint128_max_data as *mut mpd_uint_t,
        }
    };
    let mut ret: mpd_triple_class = MPD_TRIPLE_NORMAL;
    let mut status: uint32_t = 0 as uint32_t;
    let mut coeff: mpd_t = mpd_t {
        flags: 0,
        exp: 0,
        digits: 0,
        len: 0,
        alloc: 0,
        data: ::core::ptr::null_mut::<mpd_uint_t>(),
    };
    *lo = 0 as u64 as uint64_t;
    *hi = *lo;
    if mpd_isspecial(a) != 0 {
        if mpd_isinfinite(a) != 0 {
            return MPD_TRIPLE_INF;
        }
        ret = (if mpd_isqnan(a) != 0 {
            MPD_TRIPLE_QNAN as ::core::ffi::c_int
        } else {
            MPD_TRIPLE_SNAN as ::core::ffi::c_int
        }) as mpd_triple_class;
        if (*a).len == 0 as mpd_ssize_t {
            return ret;
        }
    } else if mpd_iszero(a) != 0 {
        return ret;
    }
    _mpd_copy_shared(&raw mut coeff, a);
    mpd_set_flags(&raw mut coeff, 0 as uint8_t);
    coeff.exp = 0 as mpd_ssize_t;
    if mpd_qcmp(&raw mut coeff, &raw const uint128_max, &raw mut status) > 0 as ::core::ffi::c_int {
        return MPD_TRIPLE_ERROR;
    }
    _get_coeff(hi, lo, &raw mut coeff);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_as_uint128_triple(mut a: *const mpd_t) -> mpd_uint128_triple_t {
    let mut triple: mpd_uint128_triple_t = mpd_uint128_triple_t {
        tag: MPD_TRIPLE_ERROR,
        sign: 0 as uint8_t,
        hi: 0 as uint64_t,
        lo: 0 as uint64_t,
        exp: 0 as int64_t,
    };
    triple.tag = _coeff_as_uint128(&raw mut triple.hi, &raw mut triple.lo, a);
    if triple.tag as ::core::ffi::c_uint
        == MPD_TRIPLE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return triple;
    }
    triple.sign = (mpd_isnegative(a) != 0) as ::core::ffi::c_int as uint8_t;
    if triple.tag as ::core::ffi::c_uint
        == MPD_TRIPLE_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        triple.exp = (*a).exp as int64_t;
    }
    return triple;
}
unsafe extern "C" fn mpd_qtest_newtondiv(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    _mpd_qtest_newtondiv(SET_IDEAL_EXP as ::core::ffi::c_int, q, a, b, ctx, status);
}
unsafe extern "C" fn _mpd_qtest_newtondiv(
    mut action: ::core::ffi::c_int,
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut aligned_data: [mpd_uint_t; 64] = [0; 64];
    let mut aligned: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut aligned_data as *mut mpd_uint_t,
    };
    let mut ld: mpd_uint_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut exp: mpd_ssize_t = 0;
    let mut tz: mpd_ssize_t = 0;
    let mut newsize: mpd_ssize_t = 0;
    let mut ideal_exp: mpd_ssize_t = 0;
    let mut rem: mpd_uint_t = 0;
    let mut sign_a: uint8_t = mpd_sign(a);
    let mut sign_b: uint8_t = mpd_sign(b);
    if !q.is_null() {
    } else {
        mpd_c2rust_assert_fail(b"q != NULL\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(q, a, b, ctx, status) != 0 {
            return;
        }
        _mpd_qdiv_inf(q, a, b, ctx, status);
        return;
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_seterror(q, MPD_Division_undefined as uint32_t, status);
        } else {
            mpd_setspecial(
                q,
                (sign_a as ::core::ffi::c_int ^ sign_b as ::core::ffi::c_int) as uint8_t,
                MPD_INF,
            );
            *status = (*status as ::core::ffi::c_uint | MPD_Division_by_zero) as uint32_t;
        }
        return;
    }
    if mpd_iszerocoeff(a) != 0 {
        exp = (*a).exp - (*b).exp;
        _settriple(
            q,
            (sign_a as ::core::ffi::c_int ^ sign_b as ::core::ffi::c_int) as uint8_t,
            0 as mpd_uint_t,
            exp,
        );
        mpd_qfinalize(q, ctx, status);
        return;
    }
    shift = (*b).digits - (*a).digits + (*ctx).prec + 1 as mpd_ssize_t;
    ideal_exp = (*a).exp - (*b).exp;
    exp = ideal_exp - shift;
    if shift > 0 as mpd_ssize_t {
        if mpd_qshiftl(&raw mut aligned, a, shift, status) == 0 {
            mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
            current_block = 1;
        } else {
            a = &raw mut aligned;
            current_block = 2;
        }
    } else if shift < 0 as mpd_ssize_t {
        shift = -shift;
        if mpd_qshiftl(&raw mut aligned, b, shift, status) == 0 {
            mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
            current_block = 1;
        } else {
            b = &raw mut aligned;
            current_block = 2;
        }
    } else {
        current_block = 2;
    }
    match current_block {
        2 => {
            newsize = (*a).len - (*b).len + 1 as mpd_ssize_t;
            if q != b as *mut mpd_t && q != a as *mut mpd_t
                || q == b as *mut mpd_t && newsize > (*b).len
            {
                if mpd_qresize(q, newsize, status) == 0 {
                    mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
                    current_block = 1;
                } else {
                    current_block = 3;
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    let mut r_data: [mpd_uint_t; 64] = [0; 64];
                    let mut r: mpd_t = mpd_t {
                        flags: (0 as ::core::ffi::c_int
                            | MPD_STATIC as ::core::ffi::c_int
                            | MPD_STATIC_DATA as ::core::ffi::c_int)
                            as uint8_t,
                        exp: 0 as mpd_ssize_t,
                        digits: 0 as mpd_ssize_t,
                        len: 0 as mpd_ssize_t,
                        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
                        data: &raw mut r_data as *mut mpd_uint_t,
                    };
                    _mpd_base_ndivmod(q, &raw mut r, a, b, status);
                    if mpd_isspecial(q) != 0 || mpd_isspecial(&raw mut r) != 0 {
                        mpd_setspecial(q, MPD_POS, MPD_NAN);
                        mpd_del(&raw mut r);
                    } else {
                        rem =
                            (mpd_iszerocoeff(&raw mut r) == 0) as ::core::ffi::c_int as mpd_uint_t;
                        mpd_del(&raw mut r);
                        newsize = (*q).len;
                        newsize = _mpd_real_size((*q).data, newsize);
                        mpd_qresize(q, newsize, status);
                        mpd_set_flags(
                            q,
                            (sign_a as ::core::ffi::c_int ^ sign_b as ::core::ffi::c_int)
                                as uint8_t,
                        );
                        (*q).len = newsize;
                        mpd_setdigits(q);
                        shift = ideal_exp - exp;
                        if rem != 0 {
                            ld = mpd_lsd(*(*q).data.offset(0 as isize));
                            if ld == 0 as mpd_uint_t || ld == 5 as mpd_uint_t {
                                let ref mut fresh0 = *(*q).data.offset(0 as isize);
                                *fresh0 = (*fresh0).wrapping_add(1 as mpd_uint_t);
                            }
                        } else if action == SET_IDEAL_EXP as ::core::ffi::c_int
                            && shift > 0 as mpd_ssize_t
                        {
                            tz = mpd_trail_zeros(q);
                            shift = if tz > shift { shift } else { tz };
                            mpd_qshiftr_inplace(q, shift);
                            exp += shift;
                        }
                        (*q).exp = exp;
                    }
                }
            }
        }
        _ => {}
    }
    mpd_del(&raw mut aligned);
    mpd_qfinalize(q, ctx, status);
}
unsafe extern "C" fn _mpd_qtest_ndivmod(
    mut q: *mut mpd_t,
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut current_block: u64;
    let mut aligned_data: [mpd_uint_t; 64] = [0; 64];
    let mut aligned: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut aligned_data as *mut mpd_uint_t,
    };
    let mut qsize: mpd_ssize_t = 0;
    let mut rsize: mpd_ssize_t = 0;
    let mut ideal_exp: mpd_ssize_t = 0;
    let mut expdiff: mpd_ssize_t = 0;
    let mut shift: mpd_ssize_t = 0;
    let mut sign_a: uint8_t = mpd_sign(a);
    let mut sign_ab: uint8_t =
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t;
    ideal_exp = if (*a).exp > (*b).exp {
        (*b).exp
    } else {
        (*a).exp
    };
    if mpd_iszerocoeff(a) != 0 {
        if mpd_qcopy(r, a, status) == 0 {
            current_block = 1;
        } else {
            (*r).exp = ideal_exp;
            _settriple(q, sign_ab, 0 as mpd_uint_t, 0 as mpd_ssize_t);
            return;
        }
    } else {
        expdiff = mpd_adjexp(a) - mpd_adjexp(b);
        if expdiff < 0 as mpd_ssize_t {
            if (*a).exp > (*b).exp {
                shift = (*a).exp - (*b).exp;
                if mpd_qshiftl(r, a, shift, status) == 0 {
                    current_block = 1;
                } else {
                    (*r).exp = ideal_exp;
                    current_block = 2;
                }
            } else if mpd_qcopy(r, a, status) == 0 {
                current_block = 1;
            } else {
                current_block = 2;
            }
            match current_block {
                1 => {}
                _ => {
                    _settriple(q, sign_ab, 0 as mpd_uint_t, 0 as mpd_ssize_t);
                    return;
                }
            }
        } else if expdiff > (*ctx).prec {
            *status = (*status as ::core::ffi::c_uint | MPD_Division_impossible) as uint32_t;
            current_block = 1;
        } else {
            if (*a).exp != (*b).exp {
                shift = (*a).exp - (*b).exp;
                if shift > 0 as mpd_ssize_t {
                    if mpd_qshiftl(&raw mut aligned, a, shift, status) == 0 {
                        current_block = 1;
                    } else {
                        a = &raw mut aligned;
                        current_block = 3;
                    }
                } else {
                    shift = -shift;
                    if mpd_qshiftl(&raw mut aligned, b, shift, status) == 0 {
                        current_block = 1;
                    } else {
                        b = &raw mut aligned;
                        current_block = 3;
                    }
                }
            } else {
                current_block = 3;
            }
            match current_block {
                1 => {}
                _ => {
                    qsize = (*a).len - (*b).len + 1 as mpd_ssize_t;
                    if !(q == a as *mut mpd_t && qsize < (*a).len)
                        && !(q == b as *mut mpd_t && qsize < (*b).len)
                    {
                        if mpd_qresize(q, qsize, status) == 0 {
                            current_block = 1;
                        } else {
                            current_block = 4;
                        }
                    } else {
                        current_block = 4;
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            rsize = (*b).len;
                            if !(r == a as *mut mpd_t && rsize < (*a).len) {
                                if mpd_qresize(r, rsize, status) == 0 {
                                    current_block = 1;
                                } else {
                                    current_block = 5;
                                }
                            } else {
                                current_block = 5;
                            }
                            match current_block {
                                1 => {}
                                _ => {
                                    _mpd_base_ndivmod(q, r, a, b, status);
                                    if mpd_isspecial(q) != 0 || mpd_isspecial(r) != 0 {
                                        current_block = 1;
                                    } else {
                                        qsize = (*q).len;
                                        rsize = (*r).len;
                                        qsize = _mpd_real_size((*q).data, qsize);
                                        mpd_qresize(q, qsize, status);
                                        (*q).len = qsize;
                                        mpd_setdigits(q);
                                        mpd_set_flags(q, sign_ab);
                                        (*q).exp = 0 as mpd_ssize_t;
                                        if (*q).digits > (*ctx).prec {
                                            *status = (*status as ::core::ffi::c_uint
                                                | MPD_Division_impossible)
                                                as uint32_t;
                                            current_block = 1;
                                        } else {
                                            rsize = _mpd_real_size((*r).data, rsize);
                                            mpd_qresize(r, rsize, status);
                                            (*r).len = rsize;
                                            mpd_setdigits(r);
                                            mpd_set_flags(r, sign_a);
                                            (*r).exp = ideal_exp;
                                            current_block = 6;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        1 => {
            mpd_setspecial(q, MPD_POS, MPD_NAN);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
        }
        _ => {}
    }
    mpd_del(&raw mut aligned);
}
unsafe extern "C" fn mpd_qtest_newtondivint(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut r_data: [mpd_uint_t; 64] = [0; 64];
    let mut r: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut r_data as *mut mpd_uint_t,
    };
    let mut sign: uint8_t =
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(q, a, b, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 && mpd_isinfinite(b) != 0 {
            mpd_seterror(q, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        if mpd_isinfinite(a) != 0 {
            mpd_setspecial(q, sign, MPD_INF);
            return;
        }
        if mpd_isinfinite(b) != 0 {
            _settriple(q, sign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
            return;
        }
        abort();
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_seterror(q, MPD_Division_undefined as uint32_t, status);
        } else {
            mpd_setspecial(q, sign, MPD_INF);
            *status = (*status as ::core::ffi::c_uint | MPD_Division_by_zero) as uint32_t;
        }
        return;
    }
    _mpd_qtest_ndivmod(q, &raw mut r, a, b, ctx, status);
    mpd_del(&raw mut r);
    mpd_qfinalize(q, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_qtest_newtonrem(
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut q_data: [mpd_uint_t; 64] = [0; 64];
    let mut q: mpd_t = mpd_t {
        flags: (0 as ::core::ffi::c_int
            | MPD_STATIC as ::core::ffi::c_int
            | MPD_STATIC_DATA as ::core::ffi::c_int) as uint8_t,
        exp: 0 as mpd_ssize_t,
        digits: 0 as mpd_ssize_t,
        len: 0 as mpd_ssize_t,
        alloc: MPD_MINALLOC_MAX as mpd_ssize_t,
        data: &raw mut q_data as *mut mpd_uint_t,
    };
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(r, a, b, ctx, status) != 0 {
            return;
        }
        if mpd_isinfinite(a) != 0 {
            mpd_seterror(r, MPD_Invalid_operation as uint32_t, status);
            return;
        }
        if mpd_isinfinite(b) != 0 {
            mpd_qcopy(r, a, status);
            mpd_qfinalize(r, ctx, status);
            return;
        }
        abort();
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_seterror(r, MPD_Division_undefined as uint32_t, status);
        } else {
            mpd_seterror(r, MPD_Invalid_operation as uint32_t, status);
        }
        return;
    }
    _mpd_qtest_ndivmod(&raw mut q, r, a, b, ctx, status);
    mpd_del(&raw mut q);
    mpd_qfinalize(r, ctx, status);
}
unsafe extern "C" fn mpd_qtest_newtondivmod(
    mut q: *mut mpd_t,
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *const mpd_context_t,
    mut status: *mut uint32_t,
) {
    let mut sign: uint8_t =
        (mpd_sign(a) as ::core::ffi::c_int ^ mpd_sign(b) as ::core::ffi::c_int) as uint8_t;
    if mpd_isspecial(a) != 0 || mpd_isspecial(b) != 0 {
        if mpd_qcheck_nans(q, a, b, ctx, status) != 0 {
            mpd_qcopy(r, q, status);
            return;
        }
        if mpd_isinfinite(a) != 0 {
            if mpd_isinfinite(b) != 0 {
                mpd_setspecial(q, MPD_POS, MPD_NAN);
            } else {
                mpd_setspecial(q, sign, MPD_INF);
            }
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status = (*status as ::core::ffi::c_uint | MPD_Invalid_operation) as uint32_t;
            return;
        }
        if mpd_isinfinite(b) != 0 {
            if mpd_qcopy(r, a, status) == 0 {
                mpd_seterror(q, MPD_Malloc_error as uint32_t, status);
                return;
            }
            mpd_qfinalize(r, ctx, status);
            _settriple(q, sign, 0 as mpd_uint_t, 0 as mpd_ssize_t);
            return;
        }
        abort();
    }
    if mpd_iszerocoeff(b) != 0 {
        if mpd_iszerocoeff(a) != 0 {
            mpd_setspecial(q, MPD_POS, MPD_NAN);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status = (*status as ::core::ffi::c_uint | MPD_Division_undefined) as uint32_t;
        } else {
            mpd_setspecial(q, sign, MPD_INF);
            mpd_setspecial(r, MPD_POS, MPD_NAN);
            *status = (*status as ::core::ffi::c_uint
                | (MPD_Division_by_zero | MPD_Invalid_operation)) as uint32_t;
        }
        return;
    }
    _mpd_qtest_ndivmod(q, r, a, b, ctx, status);
    mpd_qfinalize(q, ctx, status);
    mpd_qfinalize(r, ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_test_newtondiv(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qtest_newtondiv(q, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_test_newtondivint(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qtest_newtondivint(q, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_test_newtonrem(
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qtest_newtonrem(r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_test_newtondivmod(
    mut q: *mut mpd_t,
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qtest_newtondivmod(q, r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
pub const __SIZE_MAX__: u64 = 18446744073709551615 as u64;
