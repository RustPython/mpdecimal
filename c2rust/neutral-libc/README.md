# Neutral libc headers for translation

These headers replace the system headers of the same name while libmpdec is
translated (never when it is compiled as C). They declare only standard
interfaces, so that the translation does not contain the implementation
details of a particular libc, which differ between platforms:

| header     | system header content that is avoided                        |
|------------|--------------------------------------------------------------|
| assert.h   | `__assert_rtn` (macOS), `__assert_fail` (glibc), file/line    |
| ctype.h    | inline definitions (macOS), `__ctype_b_loc` tables (glibc)    |
| errno.h    | `__error()` (macOS), `__errno_location()` (glibc)             |
| locale.h   | libc-specific `struct lconv` layout (a prefix is declared)    |
| math.h     | vector math declarations (aarch64 glibc >= 2.39)              |
| stdio.h    | `FILE` layout, `__stderrp` (macOS) / `stderr` (glibc)          |
| stdint.h   | `uint64_t` as `unsigned long` (glibc LP64) or `unsigned long long` |
| inttypes.h | format macros matching the system `stdint.h`                 |
| limits.h   | libc-specific spelling (`__INT_MAX__`, `UCHAR_MAX` chains)    |
| stddef.h   | `size_t` via `__darwin_size_t` (macOS), `__DARWIN_NULL`      |
| stdlib.h   | libc-internal typedefs and declarations                      |
| string.h   | same as stdlib.h                                             |
| signal.h   | libc-specific declarations; targets without signals          |

Where a standard interface cannot be declared without libc details, the
header redirects to an `mpd_c2rust_*` function of the hand-written
[../shim/shim.rs](../shim/shim.rs):

- `stderr`: the object differs between C libraries (`__stderrp`, `stderr`,
  `__acrt_iob_func(2)`);
- `raise`: not available on targets without signals (WASI);
- `assert`: the failure handler differs between C libraries;
- `errno`: libmpdec reads errno only after `strtol`/`strtoll`, so these two
  are implemented by the shim with a private errno, and `EINVAL`/`ERANGE`
  are private values.

The exact-width types are spelled the same on every platform (64-bit types
as `long long`). `size_t` and `ptrdiff_t` are the compiler's types
(`__SIZE_TYPE__`, `__PTRDIFF_TYPE__`), which c2rust translates as
`usize`/`isize`. Where `size_t` and `uint64_t` are different C types (LP64),
the translation converts between them; with LLP64 (Windows) they are the
same type and it does not (`../expected/windows-*.diff`).

All platform-dependent code is therefore confined to `shim.rs`.
