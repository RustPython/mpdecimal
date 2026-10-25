extern "C" {
    fn mpd_addstatus_raise(ctx: *mut mpd_context_t, flags: uint32_t);
    fn mpd_qformat(
        dec: *const mpd_t,
        fmt: *const ::core::ffi::c_char,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    ) -> *mut ::core::ffi::c_char;
    fn mpd_qset_string(
        dec: *mut mpd_t,
        s: *const ::core::ffi::c_char,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmaxcoeff(result: *mut mpd_t, ctx: *const mpd_context_t, status: *mut uint32_t);
    fn mpd_qset_ssize(
        result: *mut mpd_t,
        a: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qset_i32(
        result: *mut mpd_t,
        a: int32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qset_uint(
        result: *mut mpd_t,
        a: mpd_uint_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qset_u32(
        result: *mut mpd_t,
        a: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qset_i64(
        result: *mut mpd_t,
        a: int64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qset_u64(
        result: *mut mpd_t,
        a: uint64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsset_ssize(
        result: *mut mpd_t,
        a: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsset_i32(
        result: *mut mpd_t,
        a: int32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsset_uint(
        result: *mut mpd_t,
        a: mpd_uint_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsset_u32(
        result: *mut mpd_t,
        a: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qget_ssize(dec: *const mpd_t, status: *mut uint32_t) -> mpd_ssize_t;
    fn mpd_qget_uint(dec: *const mpd_t, status: *mut uint32_t) -> mpd_uint_t;
    fn mpd_qabs_uint(dec: *const mpd_t, status: *mut uint32_t) -> mpd_uint_t;
    fn mpd_qget_i32(dec: *const mpd_t, status: *mut uint32_t) -> int32_t;
    fn mpd_qget_u32(dec: *const mpd_t, status: *mut uint32_t) -> uint32_t;
    fn mpd_qget_i64(dec: *const mpd_t, status: *mut uint32_t) -> int64_t;
    fn mpd_qget_u64(dec: *const mpd_t, status: *mut uint32_t) -> uint64_t;
    fn mpd_qcheck_nan(
        nanresult: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qcheck_nans(
        nanresult: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qfinalize(result: *mut mpd_t, ctx: *const mpd_context_t, status: *mut uint32_t);
    fn mpd_qcopy(result: *mut mpd_t, a: *const mpd_t, status: *mut uint32_t) -> ::core::ffi::c_int;
    fn mpd_qcopy_abs(
        result: *mut mpd_t,
        a: *const mpd_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qcopy_negate(
        result: *mut mpd_t,
        a: *const mpd_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qcopy_sign(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qand(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qinvert(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qlogb(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qor(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qscaleb(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qxor(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qrotate(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qshiftl(
        result: *mut mpd_t,
        a: *const mpd_t,
        n: mpd_ssize_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qshiftr(
        result: *mut mpd_t,
        a: *const mpd_t,
        n: mpd_ssize_t,
        status: *mut uint32_t,
    ) -> mpd_uint_t;
    fn mpd_qshift(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qshiftn(
        result: *mut mpd_t,
        a: *const mpd_t,
        n: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qcmp(a: *const mpd_t, b: *const mpd_t, status: *mut uint32_t) -> ::core::ffi::c_int;
    fn mpd_qcompare(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qcompare_signal(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    fn mpd_qround_to_intx(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qround_to_int(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qtrunc(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qfloor(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qceil(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qabs(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmax(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmax_mag(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmin(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmin_mag(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qminus(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qplus(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qnext_minus(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qnext_plus(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qnext_toward(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qquantize(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qrescale(
        result: *mut mpd_t,
        a: *const mpd_t,
        exp: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qreduce(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qadd(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qadd_ssize(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qadd_i32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qadd_uint(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_uint_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qadd_u32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsub(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsub_ssize(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsub_i32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsub_uint(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_uint_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsub_u32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmul(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmul_ssize(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmul_i32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmul_uint(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_uint_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmul_u32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qfma(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        c: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdiv(
        q: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdiv_ssize(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_ssize_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdiv_i32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdiv_uint(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: mpd_uint_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdiv_u32(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdivint(
        q: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qrem(
        r: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qrem_near(
        r: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdivmod(
        q: *mut mpd_t,
        r: *mut mpd_t,
        a: *const mpd_t,
        b: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qpow(
        result: *mut mpd_t,
        base: *const mpd_t,
        exp: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qpowmod(
        result: *mut mpd_t,
        base: *const mpd_t,
        exp: *const mpd_t,
        mod_0: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qexp(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qln(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qlog10(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsqrt(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qinvroot(
        result: *mut mpd_t,
        a: *const mpd_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qadd_i64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qadd_u64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsub_i64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qsub_u64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmul_i64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qmul_u64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdiv_i64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: int64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qdiv_u64(
        result: *mut mpd_t,
        a: *const mpd_t,
        b: uint64_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qimport_u16(
        result: *mut mpd_t,
        srcdata: *const uint16_t,
        srclen: size_t,
        srcsign: uint8_t,
        srcbase: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qimport_u32(
        result: *mut mpd_t,
        srcdata: *const uint32_t,
        srclen: size_t,
        srcsign: uint8_t,
        srcbase: uint32_t,
        ctx: *const mpd_context_t,
        status: *mut uint32_t,
    );
    fn mpd_qexport_u16(
        rdata: *mut *mut uint16_t,
        rlen: size_t,
        base: uint32_t,
        src: *const mpd_t,
        status: *mut uint32_t,
    ) -> size_t;
    fn mpd_qexport_u32(
        rdata: *mut *mut uint32_t,
        rlen: size_t,
        base: uint32_t,
        src: *const mpd_t,
        status: *mut uint32_t,
    ) -> size_t;
}
pub type size_t = usize;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type mpd_uint_t = uint32_t;
pub type mpd_ssize_t = int32_t;
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
#[no_mangle]
pub unsafe extern "C" fn mpd_format(
    mut dec: *const mpd_t,
    mut fmt: *const ::core::ffi::c_char,
    mut ctx: *mut mpd_context_t,
) -> *mut ::core::ffi::c_char {
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut status: uint32_t = 0 as uint32_t;
    ret = mpd_qformat(dec, fmt, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_import_u16(
    mut result: *mut mpd_t,
    mut srcdata: *const uint16_t,
    mut srclen: size_t,
    mut srcsign: uint8_t,
    mut base: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qimport_u16(result, srcdata, srclen, srcsign, base, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_import_u32(
    mut result: *mut mpd_t,
    mut srcdata: *const uint32_t,
    mut srclen: size_t,
    mut srcsign: uint8_t,
    mut base: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qimport_u32(result, srcdata, srclen, srcsign, base, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_export_u16(
    mut rdata: *mut *mut uint16_t,
    mut rlen: size_t,
    mut base: uint32_t,
    mut src: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> size_t {
    let mut n: size_t = 0;
    let mut status: uint32_t = 0 as uint32_t;
    n = mpd_qexport_u16(rdata, rlen, base, src, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_export_u32(
    mut rdata: *mut *mut uint32_t,
    mut rlen: size_t,
    mut base: uint32_t,
    mut src: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> size_t {
    let mut n: size_t = 0;
    let mut status: uint32_t = 0 as uint32_t;
    n = mpd_qexport_u32(rdata, rlen, base, src, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_finalize(mut result: *mut mpd_t, mut ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qfinalize(result, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_check_nan(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcheck_nan(result, a, ctx, &raw mut status) != 0 {
        mpd_addstatus_raise(ctx, status);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_check_nans(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcheck_nans(result, a, b, ctx, &raw mut status) != 0 {
        mpd_addstatus_raise(ctx, status);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_string(
    mut result: *mut mpd_t,
    mut s: *const ::core::ffi::c_char,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_string(result, s, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_maxcoeff(mut result: *mut mpd_t, mut ctx: *mut mpd_context_t) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmaxcoeff(result, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sset_ssize(
    mut result: *mut mpd_t,
    mut a: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_ssize(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sset_i32(
    mut result: *mut mpd_t,
    mut a: int32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_i32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sset_uint(
    mut result: *mut mpd_t,
    mut a: mpd_uint_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_uint(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sset_u32(
    mut result: *mut mpd_t,
    mut a: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsset_u32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_ssize(
    mut result: *mut mpd_t,
    mut a: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_ssize(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_i32(
    mut result: *mut mpd_t,
    mut a: int32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_i32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_i64(
    mut result: *mut mpd_t,
    mut a: int64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_i64(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_uint(
    mut result: *mut mpd_t,
    mut a: mpd_uint_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_uint(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_u32(
    mut result: *mut mpd_t,
    mut a: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_u32(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_set_u64(
    mut result: *mut mpd_t,
    mut a: uint64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qset_u64(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_get_ssize(
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> mpd_ssize_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut ret: mpd_ssize_t = 0;
    ret = mpd_qget_ssize(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_get_i32(mut a: *const mpd_t, mut ctx: *mut mpd_context_t) -> int32_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut ret: int32_t = 0;
    ret = mpd_qget_i32(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_get_i64(mut a: *const mpd_t, mut ctx: *mut mpd_context_t) -> int64_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut ret: int64_t = 0;
    ret = mpd_qget_i64(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_get_uint(
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> mpd_uint_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut ret: mpd_uint_t = 0;
    ret = mpd_qget_uint(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_abs_uint(
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> mpd_uint_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut ret: mpd_uint_t = 0;
    ret = mpd_qabs_uint(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_get_u32(mut a: *const mpd_t, mut ctx: *mut mpd_context_t) -> uint32_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut ret: uint32_t = 0;
    ret = mpd_qget_u32(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_get_u64(mut a: *const mpd_t, mut ctx: *mut mpd_context_t) -> uint64_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut ret: uint64_t = 0;
    ret = mpd_qget_u64(a, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_and(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qand(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_copy(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy(result, a, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_canonical(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    mpd_copy(result, a, ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_copy_abs(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy_abs(result, a, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_copy_negate(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy_negate(result, a, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_copy_sign(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    if mpd_qcopy_sign(result, a, b, &raw mut status) == 0 {
        mpd_addstatus_raise(ctx, status);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpd_invert(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qinvert(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_logb(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qlogb(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_or(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qor(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_rotate(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrotate(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_scaleb(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qscaleb(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_shiftl(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut n: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qshiftl(result, a, n, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_shiftr(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut n: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) -> mpd_uint_t {
    let mut status: uint32_t = 0 as uint32_t;
    let mut rnd: mpd_uint_t = 0;
    rnd = mpd_qshiftr(result, a, n, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return rnd;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_shiftn(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut n: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qshiftn(result, a, n, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_shift(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qshift(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_xor(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qxor(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_abs(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qabs(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_cmp(
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    let mut c: ::core::ffi::c_int = 0;
    c = mpd_qcmp(a, b, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_compare(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    let mut c: ::core::ffi::c_int = 0;
    c = mpd_qcompare(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_compare_signal(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) -> ::core::ffi::c_int {
    let mut status: uint32_t = 0 as uint32_t;
    let mut c: ::core::ffi::c_int = 0;
    c = mpd_qcompare_signal(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpd_add(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sub(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_add_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_add_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_add_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_add_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_add_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_add_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qadd_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sub_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sub_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sub_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sub_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sub_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sub_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsub_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_div(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv(q, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_div_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_div_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_div_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_div_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_div_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_div_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdiv_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_divmod(
    mut q: *mut mpd_t,
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdivmod(q, r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_divint(
    mut q: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qdivint(q, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_exp(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qexp(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_fma(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut c: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qfma(result, a, b, c, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_ln(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qln(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_log10(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qlog10(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_max(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmax(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_max_mag(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmax_mag(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_min(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmin(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_min_mag(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmin_mag(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_minus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qminus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_mul(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_mul_ssize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_ssize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_mul_i32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_i32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_mul_i64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: int64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_i64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_mul_uint(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: mpd_uint_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_uint(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_mul_u32(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint32_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_u32(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_mul_u64(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: uint64_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qmul_u64(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_next_minus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qnext_minus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_next_plus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qnext_plus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_next_toward(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qnext_toward(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_plus(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qplus(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_pow(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qpow(result, base, exp, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_powmod(
    mut result: *mut mpd_t,
    mut base: *const mpd_t,
    mut exp: *const mpd_t,
    mut mod_0: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qpowmod(result, base, exp, mod_0, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_quantize(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qquantize(result, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_rescale(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut exp: mpd_ssize_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrescale(result, a, exp, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_reduce(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qreduce(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_rem(
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrem(r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_rem_near(
    mut r: *mut mpd_t,
    mut a: *const mpd_t,
    mut b: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qrem_near(r, a, b, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_round_to_intx(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qround_to_intx(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_round_to_int(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qround_to_int(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_trunc(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qtrunc(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_floor(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qfloor(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_ceil(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qceil(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_sqrt(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qsqrt(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
#[no_mangle]
pub unsafe extern "C" fn mpd_invroot(
    mut result: *mut mpd_t,
    mut a: *const mpd_t,
    mut ctx: *mut mpd_context_t,
) {
    let mut status: uint32_t = 0 as uint32_t;
    mpd_qinvroot(result, a, ctx, &raw mut status);
    mpd_addstatus_raise(ctx, status);
}
