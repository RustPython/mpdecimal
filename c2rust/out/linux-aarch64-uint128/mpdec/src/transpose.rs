extern "C" {
    fn mpd_c2rust_assert_fail(expr: *const ::core::ffi::c_char);
    fn mpd_c2rust_stderr() -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputc(c: ::core::ffi::c_int, stream: *mut FILE) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut mpd_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    fn mpd_calloc(nmemb: mpd_size_t, size: mpd_size_t) -> *mut ::core::ffi::c_void;
    static mpd_bits: [mpd_size_t; 0];
}
pub type __uint128_t = u128;
pub type size_t = usize;
pub type FILE = crate::src::shim::FILE;
pub type uint64_t = u64;
pub type mpd_uint_t = uint64_t;
pub type mpd_size_t = size_t;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const BACKWARD_CYCLE: C2RustUnnamed = 1;
pub const FORWARD_CYCLE: C2RustUnnamed = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn ispower2(mut n: mpd_size_t) -> ::core::ffi::c_int {
    return (n != 0 as mpd_size_t && n & n.wrapping_sub(1 as mpd_size_t) == 0 as mpd_size_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn pointerswap(mut a: *mut *mut mpd_uint_t, mut b: *mut *mut mpd_uint_t) {
    let mut tmp: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    tmp = *b;
    *b = *a;
    *a = tmp;
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
unsafe extern "C" fn _mpd_div_words(
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
unsafe extern "C" fn mulmod_size_t(
    mut a: mpd_size_t,
    mut b: mpd_size_t,
    mut m: mpd_size_t,
) -> mpd_size_t {
    let mut hi: mpd_uint_t = 0;
    let mut lo: mpd_uint_t = 0;
    let mut q: mpd_uint_t = 0;
    let mut r: mpd_uint_t = 0;
    _mpd_mul_words(&raw mut hi, &raw mut lo, a as mpd_uint_t, b as mpd_uint_t);
    _mpd_div_words(&raw mut q, &raw mut r, hi, lo, m as mpd_uint_t);
    return r as mpd_size_t;
}
pub const BUFSIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const SIDE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn std_trans(
    mut dest: *mut mpd_uint_t,
    mut src: *mut mpd_uint_t,
    mut rows: mpd_size_t,
    mut cols: mpd_size_t,
) {
    let mut idest: mpd_size_t = 0;
    let mut isrc: mpd_size_t = 0;
    let mut r: mpd_size_t = 0;
    let mut c: mpd_size_t = 0;
    r = 0 as mpd_size_t;
    while r < rows {
        isrc = r.wrapping_mul(cols);
        idest = r;
        c = 0 as mpd_size_t;
        while c < cols {
            *dest.offset(idest as isize) = *src.offset(isrc as isize);
            isrc = isrc.wrapping_add(1 as mpd_size_t);
            idest = idest.wrapping_add(rows);
            c = c.wrapping_add(1);
        }
        r = r.wrapping_add(1);
    }
}
unsafe extern "C" fn swap_halfrows_pow2(
    mut matrix: *mut mpd_uint_t,
    mut rows: mpd_size_t,
    mut cols: mpd_size_t,
    mut dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf1: [mpd_uint_t; 4096] = [0; 4096];
    let mut buf2: [mpd_uint_t; 4096] = [0; 4096];
    let mut readbuf: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut writebuf: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut hp: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut done: *mut mpd_size_t = ::core::ptr::null_mut::<mpd_size_t>();
    let mut dbits: mpd_size_t = 0;
    let mut b: mpd_size_t = BUFSIZE as mpd_size_t;
    let mut stride: mpd_size_t = 0;
    let mut hn: mpd_size_t = 0;
    let mut hmax: mpd_size_t = 0;
    let mut m: mpd_size_t = 0;
    let mut r: mpd_size_t = 0 as mpd_size_t;
    let mut offset: mpd_size_t = 0;
    let mut next: mpd_size_t = 0;
    if cols == mul_size_t(2 as mpd_size_t, rows) {
    } else {
        mpd_c2rust_assert_fail(
            b"cols == mul_size_t(2, rows)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if dir == FORWARD_CYCLE as ::core::ffi::c_int {
        r = rows;
    } else if dir == BACKWARD_CYCLE as ::core::ffi::c_int {
        r = 2 as mpd_size_t;
    } else {
        abort();
    }
    m = cols.wrapping_sub(1 as mpd_size_t);
    hmax = rows;
    dbits = (8 as usize).wrapping_mul(::core::mem::size_of::<mpd_size_t>() as usize) as mpd_size_t;
    done = mpd_calloc(
        hmax.wrapping_div(::core::mem::size_of::<mpd_size_t>() as mpd_size_t)
            .wrapping_add(1 as mpd_size_t),
        ::core::mem::size_of::<mpd_size_t>() as mpd_size_t,
    ) as *mut mpd_size_t;
    if done.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    hn = 1 as mpd_size_t;
    while hn <= hmax {
        if !(*done.offset(hn.wrapping_div(dbits) as isize)
            & *(&raw const mpd_bits as *const mpd_size_t).offset(hn.wrapping_rem(dbits) as isize)
            != 0)
        {
            readbuf = &raw mut buf1 as *mut mpd_uint_t;
            writebuf = &raw mut buf2 as *mut mpd_uint_t;
            offset = 0 as mpd_size_t;
            while offset < cols.wrapping_div(2 as mpd_size_t) {
                stride = if offset.wrapping_add(b) < cols.wrapping_div(2 as mpd_size_t) {
                    b
                } else {
                    cols.wrapping_div(2 as mpd_size_t).wrapping_sub(offset)
                };
                hp = matrix.offset(hn.wrapping_mul(cols).wrapping_div(2 as mpd_size_t) as isize);
                memcpy(
                    readbuf as *mut ::core::ffi::c_void,
                    hp.offset(offset as isize) as *const ::core::ffi::c_void,
                    (stride as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                );
                pointerswap(&raw mut readbuf, &raw mut writebuf);
                next = mulmod_size_t(hn, r, m);
                hp = matrix.offset(next.wrapping_mul(cols).wrapping_div(2 as mpd_size_t) as isize);
                while next != hn {
                    memcpy(
                        readbuf as *mut ::core::ffi::c_void,
                        hp.offset(offset as isize) as *const ::core::ffi::c_void,
                        (stride as size_t)
                            .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    memcpy(
                        hp.offset(offset as isize) as *mut ::core::ffi::c_void,
                        writebuf as *const ::core::ffi::c_void,
                        (stride as size_t)
                            .wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    pointerswap(&raw mut readbuf, &raw mut writebuf);
                    *done.offset(next.wrapping_div(dbits) as isize) |= *(&raw const mpd_bits
                        as *const mpd_size_t)
                        .offset(next.wrapping_rem(dbits) as isize);
                    next = mulmod_size_t(next, r, m);
                    hp = matrix
                        .offset(next.wrapping_mul(cols).wrapping_div(2 as mpd_size_t) as isize);
                }
                memcpy(
                    hp.offset(offset as isize) as *mut ::core::ffi::c_void,
                    writebuf as *const ::core::ffi::c_void,
                    (stride as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                );
                *done.offset(hn.wrapping_div(dbits) as isize) |= *(&raw const mpd_bits
                    as *const mpd_size_t)
                    .offset(hn.wrapping_rem(dbits) as isize);
                offset = offset.wrapping_add(b);
            }
        }
        hn = hn.wrapping_add(2 as mpd_size_t);
    }
    mpd_free.expect("non-null function pointer")(done as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn squaretrans(mut buf: *mut mpd_uint_t, mut cols: mpd_size_t) {
    let mut tmp: mpd_uint_t = 0;
    let mut idest: mpd_size_t = 0;
    let mut isrc: mpd_size_t = 0;
    let mut r: mpd_size_t = 0;
    let mut c: mpd_size_t = 0;
    r = 0 as mpd_size_t;
    while r < cols {
        c = r.wrapping_add(1 as mpd_size_t);
        isrc = r.wrapping_mul(cols).wrapping_add(c);
        idest = c.wrapping_mul(cols).wrapping_add(r);
        c = r.wrapping_add(1 as mpd_size_t);
        while c < cols {
            tmp = *buf.offset(isrc as isize);
            *buf.offset(isrc as isize) = *buf.offset(idest as isize);
            *buf.offset(idest as isize) = tmp;
            isrc = isrc.wrapping_add(1 as mpd_size_t);
            idest = idest.wrapping_add(cols);
            c = c.wrapping_add(1);
        }
        r = r.wrapping_add(1);
    }
}
unsafe extern "C" fn squaretrans_pow2(mut matrix: *mut mpd_uint_t, mut size: mpd_size_t) {
    let mut buf1: [mpd_uint_t; 16384] = [0; 16384];
    let mut buf2: [mpd_uint_t; 16384] = [0; 16384];
    let mut to: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut from: *mut mpd_uint_t = ::core::ptr::null_mut::<mpd_uint_t>();
    let mut b: mpd_size_t = size;
    let mut r: mpd_size_t = 0;
    let mut c: mpd_size_t = 0;
    let mut i: mpd_size_t = 0;
    while b > SIDE as mpd_size_t {
        b >>= 1 as ::core::ffi::c_int;
    }
    r = 0 as mpd_size_t;
    while r < size {
        c = r;
        while c < size {
            from = matrix
                .offset(r.wrapping_mul(size) as isize)
                .offset(c as isize);
            to = &raw mut buf1 as *mut mpd_uint_t;
            i = 0 as mpd_size_t;
            while i < b {
                memcpy(
                    to as *mut ::core::ffi::c_void,
                    from as *const ::core::ffi::c_void,
                    (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                );
                from = from.offset(size as isize);
                to = to.offset(b as isize);
                i = i.wrapping_add(1);
            }
            squaretrans(&raw mut buf1 as *mut mpd_uint_t, b);
            if r == c {
                to = matrix
                    .offset(r.wrapping_mul(size) as isize)
                    .offset(c as isize);
                from = &raw mut buf1 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.offset(b as isize);
                    to = to.offset(size as isize);
                    i = i.wrapping_add(1);
                }
            } else {
                from = matrix
                    .offset(c.wrapping_mul(size) as isize)
                    .offset(r as isize);
                to = &raw mut buf2 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.offset(size as isize);
                    to = to.offset(b as isize);
                    i = i.wrapping_add(1);
                }
                squaretrans(&raw mut buf2 as *mut mpd_uint_t, b);
                to = matrix
                    .offset(c.wrapping_mul(size) as isize)
                    .offset(r as isize);
                from = &raw mut buf1 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.offset(b as isize);
                    to = to.offset(size as isize);
                    i = i.wrapping_add(1);
                }
                to = matrix
                    .offset(r.wrapping_mul(size) as isize)
                    .offset(c as isize);
                from = &raw mut buf2 as *mut mpd_uint_t;
                i = 0 as mpd_size_t;
                while i < b {
                    memcpy(
                        to as *mut ::core::ffi::c_void,
                        from as *const ::core::ffi::c_void,
                        (b as size_t).wrapping_mul(::core::mem::size_of::<mpd_uint_t>() as size_t),
                    );
                    from = from.offset(b as isize);
                    to = to.offset(size as isize);
                    i = i.wrapping_add(1);
                }
            }
            c = c.wrapping_add(b);
        }
        r = r.wrapping_add(b);
    }
}
#[no_mangle]
pub unsafe extern "C" fn transpose_pow2(
    mut matrix: *mut mpd_uint_t,
    mut rows: mpd_size_t,
    mut cols: mpd_size_t,
) -> ::core::ffi::c_int {
    let mut size: mpd_size_t = mul_size_t(rows, cols);
    if ispower2(rows) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(rows)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if ispower2(cols) != 0 {
    } else {
        mpd_c2rust_assert_fail(b"ispower2(cols)\0" as *const u8 as *const ::core::ffi::c_char);
    };
    if cols == rows {
        squaretrans_pow2(matrix, rows);
    } else if cols == mul_size_t(2 as mpd_size_t, rows) {
        if swap_halfrows_pow2(matrix, rows, cols, FORWARD_CYCLE as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        squaretrans_pow2(matrix, rows);
        squaretrans_pow2(
            matrix.offset(size.wrapping_div(2 as mpd_size_t) as isize),
            rows,
        );
    } else if rows == mul_size_t(2 as mpd_size_t, cols) {
        squaretrans_pow2(matrix, cols);
        squaretrans_pow2(
            matrix.offset(size.wrapping_div(2 as mpd_size_t) as isize),
            cols,
        );
        if swap_halfrows_pow2(matrix, cols, rows, BACKWARD_CYCLE as ::core::ffi::c_int) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        abort();
    }
    return 1 as ::core::ffi::c_int;
}
