extern "C" {
    fn mpd_c2rust_assert_fail(expr: *const ::core::ffi::c_char);
    static mut mpd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    fn _mpd_init_fnt_params(
        n: mpd_size_t,
        sign: ::core::ffi::c_int,
        modnum: ::core::ffi::c_int,
    ) -> *mut fnt_params;
    fn fnt_dif2(a: *mut mpd_uint_t, n: mpd_size_t, tparams: *mut fnt_params);
}
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
#[no_mangle]
pub unsafe extern "C" fn std_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n as u64 <= (3 as u64).wrapping_mul(4294967296 as u64) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    tparams = _mpd_init_fnt_params(n, -(1 as ::core::ffi::c_int), modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    fnt_dif2(a as *mut mpd_uint_t, n, tparams);
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn std_inv_fnt(
    mut a: *mut mpd_uint_t,
    mut n: mpd_size_t,
    mut modnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tparams: *mut fnt_params = ::core::ptr::null_mut::<fnt_params>();
    if ispower2(n) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(n)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n >= 4 as mpd_size_t {
    } else {
        mpd_c2rust_assert_fail(b"n >= 4\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if n as u64 <= (3 as u64).wrapping_mul(4294967296 as u64) {
    } else {
        mpd_c2rust_assert_fail(
            b"n <= 3*MPD_MAXTRANSFORM_2N\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    tparams = _mpd_init_fnt_params(n, 1 as ::core::ffi::c_int, modnum);
    if tparams.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    fnt_dif2(a as *mut mpd_uint_t, n, tparams);
    mpd_free.expect("non-null function pointer")(tparams as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
