extern "C" {
    fn mpd_c2rust_assert_fail(expr: *const ::core::ffi::c_char);
    fn mpd_sh_alloc(
        struct_size: mpd_size_t,
        nmemb: mpd_size_t,
        size: mpd_size_t,
    ) -> *mut ::core::ffi::c_void;
    static mpd_moduli: [mpd_uint_t; 0];
    static mpd_roots: [mpd_uint_t; 0];
}
pub type size_t = usize;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type mpd_uint_t = uint32_t;
pub type mpd_uuint_t = uint64_t;
pub type mpd_size_t = size_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const P3: C2RustUnnamed = 2;
pub const P2: C2RustUnnamed = 1;
pub const P1: C2RustUnnamed = 0;
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
unsafe extern "C" fn std_setmodulus(mut modnum: ::core::ffi::c_int, mut umod: *mut mpd_uint_t) {
    *umod = *(&raw const mpd_moduli as *const mpd_uint_t).offset(modnum as isize);
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
#[no_mangle]
pub unsafe extern "C" fn _mpd_getkernel(
    mut n: mpd_uint_t,
    mut sign: ::core::ffi::c_int,
    mut modnum: ::core::ffi::c_int,
) -> mpd_uint_t {
    let mut umod: mpd_uint_t = 0;
    let mut p: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    let mut xi: mpd_uint_t = 0;
    std_setmodulus(modnum, &raw mut umod);
    r = *(&raw const mpd_roots as *const mpd_uint_t).offset(modnum as isize);
    p = umod;
    xi = p.wrapping_sub(1 as mpd_uint_t).wrapping_div(n);
    if sign == -(1 as ::core::ffi::c_int) {
        return std_powmod(r, p.wrapping_sub(1 as mpd_uint_t).wrapping_sub(xi), umod);
    } else {
        return std_powmod(r, xi, umod);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_init_fnt_params(
    mut n: mpd_size_t,
    mut sign: ::core::ffi::c_int,
    mut modnum: ::core::ffi::c_int,
) -> *mut fnt_params {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    let mut umod: mpd_uint_t = 0;
    let mut kernel: mpd_uint_t = 0;
    let mut w: mpd_uint_t = 0;
    let mut i: mpd_uint_t = 0;
    let mut nhalf: mpd_size_t = 0;
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if sign == -(1 as ::core::ffi::c_int) || sign == 1 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"sign == -1 || sign == 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if P1 as ::core::ffi::c_int <= modnum && modnum <= P3 as ::core::ffi::c_int {
    } else {
        mpd_c2rust_assert_fail(
            b"P1 <= modnum && modnum <= P3\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    nhalf = n.wrapping_div(2 as mpd_size_t);
    tparams = mpd_sh_alloc(
        ::core::mem::size_of::<fnt_params>() as mpd_size_t,
        nhalf,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
    ) as *mut fnt_params;
    if tparams.is_null() {
        return ::core::ptr::null_mut::<fnt_params>();
    }
    std_setmodulus(modnum, &raw mut umod);
    kernel = _mpd_getkernel(n as mpd_uint_t, sign, modnum);
    (*tparams).modnum = modnum;
    (*tparams).modulus = umod;
    (*tparams).kernel = kernel;
    w = 1 as mpd_uint_t;
    i = 0 as mpd_uint_t;
    while i < nhalf as mpd_uint_t {
        *(&raw mut (*tparams).wtable as *mut mpd_uint_t).offset(i as isize) = w;
        w = std_mulmod(w, kernel, umod);
        i = i.wrapping_add(1);
    }
    return tparams;
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_init_w3table(
    mut w3table: *mut mpd_uint_t,
    mut sign: ::core::ffi::c_int,
    mut modnum: ::core::ffi::c_int,
) {
    let mut umod: mpd_uint_t = 0;
    let mut kernel: mpd_uint_t = 0;
    std_setmodulus(modnum, &raw mut umod);
    kernel = _mpd_getkernel(3 as mpd_uint_t, sign, modnum);
    *w3table.offset(0 as isize) = 1 as mpd_uint_t;
    *w3table.offset(1 as isize) = kernel;
    *w3table.offset(2 as isize) = std_powmod(kernel, 2 as mpd_uint_t, umod);
}
