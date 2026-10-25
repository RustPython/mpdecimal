//! Platform-dependent part of the neutral libc headers (`../neutral-libc`).
//!
//! This file is not translated: `normalize.py` copies it into every
//! translated crate as `src/shim.rs`. It defines the `mpd_c2rust_*` symbols
//! that the neutral headers redirect to.

use core::cell::Cell;
use core::ffi::{c_char, c_int, c_long, c_longlong, CStr};

const EINVAL: c_int = 22; // must match ../neutral-libc/errno.h
const ERANGE: c_int = 34;

/// Opaque FILE of the C library, only handled through pointers.
#[repr(C)]
pub struct FILE {
    _opaque: [u8; 0],
}

std::thread_local! {
    static ERRNO: Cell<c_int> = const { Cell::new(0) };
}

#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_errno() -> *mut c_int {
    ERRNO.with(|e| e.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_assert_fail(expr: *const c_char) -> ! {
    let expr = CStr::from_ptr(expr).to_string_lossy();
    eprintln!("libmpdec: assertion failed: {expr}");
    std::process::abort()
}

#[cfg(not(target_os = "wasi"))]
extern "C" {
    fn raise(sig: c_int) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_raise(sig: c_int) -> c_int {
    #[cfg(not(target_os = "wasi"))]
    return raise(sig);
    // No signals: the default action of SIGFPE is to terminate the process.
    #[cfg(target_os = "wasi")]
    {
        let _ = sig;
        std::process::abort()
    }
}

#[cfg(any(
    target_vendor = "apple",
    target_os = "freebsd",
    target_os = "dragonfly"
))]
#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_stderr() -> *mut FILE {
    extern "C" {
        static mut __stderrp: *mut FILE;
    }
    __stderrp
}

#[cfg(windows)]
#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_stderr() -> *mut FILE {
    extern "C" {
        fn __acrt_iob_func(index: u32) -> *mut FILE;
    }
    __acrt_iob_func(2)
}

#[cfg(not(any(
    target_vendor = "apple",
    target_os = "freebsd",
    target_os = "dragonfly",
    windows
)))]
#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_stderr() -> *mut FILE {
    extern "C" {
        static mut stderr: *mut FILE;
    }
    stderr
}

/// strtol()/strtoll() of the C standard (7.22.1.4) in the "C" locale, with
/// errors reported through the private errno. If no conversion can be
/// performed, errno is set to EINVAL (POSIX permits this).
unsafe fn strto(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
    min: i128,
    max: i128,
) -> i128 {
    let at = |i: usize| *nptr.add(i) as u8;
    let digit = |c: u8| -> Option<u32> { (c as char).to_digit(36) };
    let set_end = |i: usize| {
        if !endptr.is_null() {
            *endptr = nptr.add(i) as *mut c_char;
        }
    };

    if base < 0 || base == 1 || base > 36 {
        set_end(0);
        *mpd_c2rust_errno() = EINVAL;
        return 0;
    }
    let mut base = base as u32;

    let mut i = 0;
    while matches!(at(i), b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
        i += 1;
    }
    let neg = at(i) == b'-';
    if matches!(at(i), b'+' | b'-') {
        i += 1;
    }
    let hex_prefix = at(i) == b'0'
        && matches!(at(i + 1), b'x' | b'X')
        && digit(at(i + 2)).is_some_and(|d| d < 16);
    if (base == 0 || base == 16) && hex_prefix {
        base = 16;
        i += 2;
    } else if base == 0 {
        base = if at(i) == b'0' { 8 } else { 10 };
    }

    let start = i;
    let mut acc: i128 = 0;
    let mut overflow = false;
    while let Some(d) = digit(at(i)).filter(|&d| d < base) {
        if !overflow {
            acc = acc * base as i128 + d as i128;
            overflow = if neg { -acc < min } else { acc > max };
        }
        i += 1;
    }
    if i == start {
        set_end(0);
        *mpd_c2rust_errno() = EINVAL;
        return 0;
    }
    set_end(i);
    if overflow {
        *mpd_c2rust_errno() = ERANGE;
        return if neg { min } else { max };
    }
    if neg {
        -acc
    } else {
        acc
    }
}

#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_strtol(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> c_long {
    strto(nptr, endptr, base, c_long::MIN as i128, c_long::MAX as i128) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn mpd_c2rust_strtoll(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> c_longlong {
    strto(
        nptr,
        endptr,
        base,
        c_longlong::MIN as i128,
        c_longlong::MAX as i128,
    ) as c_longlong
}
