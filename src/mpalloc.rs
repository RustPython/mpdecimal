use crate::common::*;
use crate::shim::mpd_c2rust_assert_fail;
use crate::context::mpd_addstatus_raise;
use crate::mpdecimal::mpd_set_qnan;
use crate::mpdecimal::mpd_set_positive;
use crate::mpdecimal::mpd_set_dynamic_data;
extern "C" {
    fn malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn free(ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        s: *mut ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" fn add_size_t_overflow(
    mut a: mpd_size_t,
    mut b: mpd_size_t,
    mut overflow: *mut mpd_size_t,
) -> mpd_size_t {
    let mut ret: mpd_size_t = 0;
    *overflow = 0 as mpd_size_t;
    ret = a.wrapping_add(b);
    if ret < a {
        *overflow = 1 as mpd_size_t;
    }
    return ret;
}
unsafe extern "C" fn mul_size_t_overflow(
    mut a: mpd_size_t,
    mut b: mpd_size_t,
    mut overflow: *mut mpd_size_t,
) -> mpd_size_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi, &raw mut lo, a as mpd_uint_t, b as mpd_uint_t);
    *overflow = hi as mpd_size_t;
    return lo as mpd_size_t;
}
#[cfg_attr(feature = "capi", no_mangle)]
pub static mut MPD_MINALLOC: mpd_ssize_t = MPD_MINALLOC_MIN as mpd_ssize_t;
#[cfg_attr(feature = "capi", no_mangle)]
pub static mut mpd_mallocfunc: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void> =
    unsafe { Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void) };
#[cfg_attr(feature = "capi", no_mangle)]
pub static mut mpd_reallocfunc: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
> = unsafe {
    Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    )
};
#[cfg_attr(feature = "capi", no_mangle)]
pub static mut mpd_callocfunc: Option<
    unsafe extern "C" fn(size_t, size_t) -> *mut ::core::ffi::c_void,
> = unsafe { Some(calloc as unsafe extern "C" fn(size_t, size_t) -> *mut ::core::ffi::c_void) };
#[cfg_attr(feature = "capi", no_mangle)]
pub static mut mpd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> =
    unsafe { Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()) };
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_callocfunc_em(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut req: size_t = 0;
    let mut overflow: mpd_size_t = 0;
    req = mul_size_t_overflow(nmemb, size, &raw mut overflow) as size_t;
    if overflow != 0 {
        return NULL;
    }
    ptr = mpd_mallocfunc.expect("non-null function pointer")(req);
    if ptr.is_null() {
        return NULL;
    }
    memset(ptr, 0 as ::core::ffi::c_int, req);
    return ptr;
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_alloc(
    mut nmemb: mpd_size_t,
    mut size: mpd_size_t,
) -> *mut ::core::ffi::c_void {
    let mut req: mpd_size_t = 0;
    let mut overflow: mpd_size_t = 0;
    req = mul_size_t_overflow(nmemb, size, &raw mut overflow);
    if overflow != 0 {
        return NULL;
    }
    return mpd_mallocfunc.expect("non-null function pointer")(req as size_t);
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_calloc(
    mut nmemb: mpd_size_t,
    mut size: mpd_size_t,
) -> *mut ::core::ffi::c_void {
    let mut overflow: mpd_size_t = 0;
    mul_size_t_overflow(nmemb, size, &raw mut overflow);
    if overflow != 0 {
        return NULL;
    }
    return mpd_callocfunc.expect("non-null function pointer")(nmemb as size_t, size as size_t);
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut nmemb: mpd_size_t,
    mut size: mpd_size_t,
    mut err: *mut uint8_t,
) -> *mut ::core::ffi::c_void {
    let mut new: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut req: mpd_size_t = 0;
    let mut overflow: mpd_size_t = 0;
    req = mul_size_t_overflow(nmemb, size, &raw mut overflow);
    if overflow != 0 {
        *err = 1 as uint8_t;
        return ptr;
    }
    new = mpd_reallocfunc.expect("non-null function pointer")(ptr, req as size_t);
    if new.is_null() {
        *err = 1 as uint8_t;
        return ptr;
    }
    return new;
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_sh_alloc(
    mut struct_size: mpd_size_t,
    mut nmemb: mpd_size_t,
    mut size: mpd_size_t,
) -> *mut ::core::ffi::c_void {
    let mut req: mpd_size_t = 0;
    let mut overflow: mpd_size_t = 0;
    req = mul_size_t_overflow(nmemb, size, &raw mut overflow);
    if overflow != 0 {
        return NULL;
    }
    req = add_size_t_overflow(req, struct_size, &raw mut overflow);
    if overflow != 0 {
        return NULL;
    }
    return mpd_mallocfunc.expect("non-null function pointer")(req as size_t);
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_qnew_size(mut nwords: mpd_ssize_t) -> *mut mpd_t {
    let mut result: *mut mpd_t = ::core::ptr::null_mut::<mpd_t>();
    nwords = if nwords < MPD_MINALLOC {
        MPD_MINALLOC
    } else {
        nwords
    };
    result = mpd_alloc(
        1 as mpd_size_t,
        ::core::mem::size_of::<mpd_t>() as mpd_size_t,
    ) as *mut mpd_t;
    if result.is_null() {
        return ::core::ptr::null_mut::<mpd_t>();
    }
    (*result).data = mpd_alloc(
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
    ) as *mut mpd_uint_t;
    if (*result).data.is_null() {
        mpd_free.expect("non-null function pointer")(result as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<mpd_t>();
    }
    (*result).flags = 0 as uint8_t;
    (*result).exp = 0 as mpd_ssize_t;
    (*result).digits = 0 as mpd_ssize_t;
    (*result).len = 0 as mpd_ssize_t;
    (*result).alloc = nwords;
    return result;
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_qnew() -> *mut mpd_t {
    return mpd_qnew_size(MPD_MINALLOC);
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_new(mut ctx: *mut mpd_context_t) -> *mut mpd_t {
    let mut result: *mut mpd_t = ::core::ptr::null_mut::<mpd_t>();
    result = mpd_qnew();
    if result.is_null() {
        mpd_addstatus_raise(ctx, MPD_Malloc_error as uint32_t);
    }
    return result;
}
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_switch_to_dyn(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut p: *mut mpd_uint_t = (*result).data;
    if nwords >= (*result).alloc {
    } else {
        mpd_c2rust_assert_fail(
            b"nwords >= result->alloc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    (*result).data = mpd_alloc(
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
    ) as *mut mpd_uint_t;
    if (*result).data.is_null() {
        (*result).data = p;
        mpd_set_qnan(result);
        mpd_set_positive(result);
        (*result).len = 0 as mpd_ssize_t;
        (*result).digits = (*result).len;
        (*result).exp = (*result).digits;
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        (*result).data as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ((*result).alloc as u64).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as u64)
            as size_t,
    );
    (*result).alloc = nwords;
    mpd_set_dynamic_data(result);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_switch_to_dyn(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut p: *mut mpd_uint_t = (*result).data;
    if nwords >= (*result).alloc {
    } else {
        mpd_c2rust_assert_fail(
            b"nwords >= result->alloc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    (*result).data = mpd_alloc(
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
    ) as *mut mpd_uint_t;
    if (*result).data.is_null() {
        (*result).data = p;
        mpd_set_qnan(result);
        mpd_set_positive(result);
        (*result).len = 0 as mpd_ssize_t;
        (*result).digits = (*result).len;
        (*result).exp = (*result).digits;
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        (*result).data as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        ((*result).alloc as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
    );
    (*result).alloc = nwords;
    mpd_set_dynamic_data(result);
    return 1 as ::core::ffi::c_int;
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_switch_to_dyn_zero(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut p: *mut mpd_uint_t = (*result).data;
    (*result).data = mpd_calloc(
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
    ) as *mut mpd_uint_t;
    if (*result).data.is_null() {
        (*result).data = p;
        mpd_set_qnan(result);
        mpd_set_positive(result);
        (*result).len = 0 as mpd_ssize_t;
        (*result).digits = (*result).len;
        (*result).exp = (*result).digits;
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    (*result).alloc = nwords;
    mpd_set_dynamic_data(result);
    return 1 as ::core::ffi::c_int;
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_realloc_dyn(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
    mut status: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut err: uint8_t = 0 as uint8_t;
    (*result).data = mpd_realloc(
        (*result).data as *mut ::core::ffi::c_void,
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
        &raw mut err,
    ) as *mut mpd_uint_t;
    if err == 0 {
        (*result).alloc = nwords;
    } else if nwords > (*result).alloc {
        mpd_set_qnan(result);
        mpd_set_positive(result);
        (*result).len = 0 as mpd_ssize_t;
        (*result).digits = (*result).len;
        (*result).exp = (*result).digits;
        *status = (*status as ::core::ffi::c_uint | MPD_Malloc_error) as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "64")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_switch_to_dyn_cxx(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
) -> ::core::ffi::c_int {
    if nwords >= (*result).alloc {
    } else {
        mpd_c2rust_assert_fail(
            b"nwords >= result->alloc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let mut data: *mut mpd_uint_t = mpd_alloc(
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
    ) as *mut mpd_uint_t;
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        data as *mut ::core::ffi::c_void,
        (*result).data as *const ::core::ffi::c_void,
        ((*result).alloc as u64).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as u64)
            as size_t,
    );
    (*result).data = data;
    (*result).alloc = nwords;
    mpd_set_dynamic_data(result);
    return 1 as ::core::ffi::c_int;
}
#[cfg(target_pointer_width = "32")]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_switch_to_dyn_cxx(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
) -> ::core::ffi::c_int {
    if nwords >= (*result).alloc {
    } else {
        mpd_c2rust_assert_fail(
            b"nwords >= result->alloc\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let mut data: *mut mpd_uint_t = mpd_alloc(
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
    ) as *mut mpd_uint_t;
    if data.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memcpy(
        data as *mut ::core::ffi::c_void,
        (*result).data as *const ::core::ffi::c_void,
        ((*result).alloc as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
    );
    (*result).data = data;
    (*result).alloc = nwords;
    mpd_set_dynamic_data(result);
    return 1 as ::core::ffi::c_int;
}
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn mpd_realloc_dyn_cxx(
    mut result: *mut mpd_t,
    mut nwords: mpd_ssize_t,
) -> ::core::ffi::c_int {
    let mut err: uint8_t = 0 as uint8_t;
    let mut p: *mut mpd_uint_t = mpd_realloc(
        (*result).data as *mut ::core::ffi::c_void,
        nwords as mpd_size_t,
        ::core::mem::size_of::<mpd_uint_t>() as mpd_size_t,
        &raw mut err,
    ) as *mut mpd_uint_t;
    if err == 0 {
        (*result).data = p;
        (*result).alloc = nwords;
    } else if nwords > (*result).alloc {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
