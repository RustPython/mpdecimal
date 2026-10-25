pub type uint32_t = u32;
pub type size_t = usize;
pub type mpd_uint_t = uint32_t;
pub type mpd_size_t = size_t;
#[no_mangle]
pub static mut mpd_moduli: [mpd_uint_t; 3] = [
    2113929217 as u32 as mpd_uint_t,
    2013265921 as u32 as mpd_uint_t,
    1811939329 as u32 as mpd_uint_t,
];
#[no_mangle]
pub static mut mpd_roots: [mpd_uint_t; 3] = [
    5 as u32 as mpd_uint_t,
    31 as u32 as mpd_uint_t,
    13 as u32 as mpd_uint_t,
];
#[no_mangle]
pub static mut mpd_invmoduli: [[uint32_t; 3]; 3] = [
    [
        4293885170 as ::core::ffi::c_uint,
        2181570688 as ::core::ffi::c_uint,
        16352 as ::core::ffi::c_uint,
    ],
    [
        1698898177 as ::core::ffi::c_uint,
        2290649223 as ::core::ffi::c_uint,
        16352 as ::core::ffi::c_uint,
    ],
    [
        2716021846 as ::core::ffi::c_uint,
        2545165803 as ::core::ffi::c_uint,
        16352 as ::core::ffi::c_uint,
    ],
];
#[no_mangle]
pub static mut MPD_TWO63: ::core::ffi::c_float = 9223372036854775808.0f32;
#[no_mangle]
pub static mut INV_P1_MOD_P2: mpd_uint_t = 2013265901 as mpd_uint_t;
#[no_mangle]
pub static mut INV_P1P2_MOD_P3: mpd_uint_t = 54 as mpd_uint_t;
#[no_mangle]
pub static mut LH_P1P2: mpd_uint_t = 4127195137 as mpd_uint_t;
#[no_mangle]
pub static mut UH_P1P2: mpd_uint_t = 990904320 as mpd_uint_t;
#[no_mangle]
pub static mut mpd_bits: [mpd_size_t; 32] = [
    1 as mpd_size_t,
    2 as mpd_size_t,
    4 as mpd_size_t,
    8 as mpd_size_t,
    16 as mpd_size_t,
    32 as mpd_size_t,
    64 as mpd_size_t,
    128 as mpd_size_t,
    256 as mpd_size_t,
    512 as mpd_size_t,
    1024 as mpd_size_t,
    2048 as mpd_size_t,
    4096 as mpd_size_t,
    8192 as mpd_size_t,
    16384 as mpd_size_t,
    32768 as mpd_size_t,
    65536 as mpd_size_t,
    131072 as mpd_size_t,
    262144 as mpd_size_t,
    524288 as mpd_size_t,
    1048576 as mpd_size_t,
    2097152 as mpd_size_t,
    4194304 as mpd_size_t,
    8388608 as mpd_size_t,
    16777216 as mpd_size_t,
    33554432 as mpd_size_t,
    67108864 as mpd_size_t,
    134217728 as mpd_size_t,
    268435456 as mpd_size_t,
    536870912 as mpd_size_t,
    1073741824 as mpd_size_t,
    2147483648 as u32 as mpd_size_t,
];
#[no_mangle]
pub static mut mpd_pow10: [mpd_uint_t; 10] = [
    1 as mpd_uint_t,
    10 as mpd_uint_t,
    100 as mpd_uint_t,
    1000 as mpd_uint_t,
    10000 as mpd_uint_t,
    100000 as mpd_uint_t,
    1000000 as mpd_uint_t,
    10000000 as mpd_uint_t,
    100000000 as mpd_uint_t,
    1000000000 as mpd_uint_t,
];
#[no_mangle]
pub static mut mpd_round_string: [*const ::core::ffi::c_char; 9] = [
    b"ROUND_UP\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_DOWN\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_CEILING\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_FLOOR\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_HALF_UP\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_HALF_DOWN\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_HALF_EVEN\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_05UP\0" as *const u8 as *const ::core::ffi::c_char,
    b"ROUND_TRUNC\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub static mut mpd_clamp_string: [*const ::core::ffi::c_char; 2] = [
    b"CLAMP_DEFAULT\0" as *const u8 as *const ::core::ffi::c_char,
    b"CLAMP_IEEE_754\0" as *const u8 as *const ::core::ffi::c_char,
];
