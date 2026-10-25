# c2rust translation of libmpdec

Phase 1 of the Rust port: translate `libmpdec/*.c` with c2rust and make the
unmodified upstream C tests pass against the translated library through the
C ABI. Every translation is produced by the scripts in this directory, and
every run is appended to [RUNS.md](RUNS.md), so any output can be
regenerated. The result is the crate of the repository root (`Cargo.toml`,
`src/`), written by [install.py](install.py).

## Configuration matrix

Translations ([transpile-all.sh](transpile-all.sh)):

| label                   | where             | MACHINE | config    | use |
|-------------------------|-------------------|---------|-----------|-----|
| `linux-x86_64-uint128`  | container, sysroot `/usr/x86_64-linux-gnu` | uint128 | CONFIG_64 | merged |
| `linux-i686-ansi32`     | container, sysroot `/usr/i686-linux-gnu`   | ansi32  | CONFIG_32 | merged |
| `linux-x86_64-uint128-hooks` | as above, `--test-hooks` | uint128 | CONFIG_64 | merged |
| `linux-i686-ansi32-hooks`    | as above, `--test-hooks` | ansi32  | CONFIG_32 | merged |
| `macos-aarch64-uint128` | macOS host        | uint128 | CONFIG_64 | oracle |
| `macos-x86_64-uint128`  | macOS host, `--target=x86_64-apple-darwin` | uint128 | CONFIG_64 | oracle |
| `macos-aarch64-uint128-hooks` | macOS host, `--test-hooks` | uint128 | CONFIG_64 | oracle |
| `linux-aarch64-uint128` | container, native | uint128 | CONFIG_64 | oracle |
| `linux-armv7-ansi32`    | any host, `--target=armv7-linux-gnueabihf` | ansi32 | CONFIG_32 | oracle |
| `windows-x86_64-uint128` | any host, `--target=x86_64-pc-windows-gnu` | uint128 | CONFIG_64 | oracle |
| `windows-aarch64-uint128` | any host, `--target=aarch64-w64-mingw32` | uint128 | CONFIG_64 | oracle |
| `windows-i686-ansi32`   | any host, `--target=i686-pc-windows-gnu` | ansi32 | CONFIG_32 | oracle |

c2rust 0.22.1 cannot translate for s390x (va_list) or wasm32 (`__externref_t`);
these targets are tested only.

Tests ([test-all.sh](test-all.sh), [test-long.sh](test-long.sh)): macOS
aarch64 and x86_64 (Rosetta 2), Linux aarch64, x86_64 (Rosetta 2, image
[Containerfile.amd64](Containerfile.amd64)), i686, s390x (big-endian) and
armv7 (qemu-user), wasm32-wasip1 (wasi-sdk, wasmtime), Windows x86_64
(MinGW-w64, Wine).

CPython builds libmpdec as `uint128` (or `x64`/`ansi64`) when
`sizeof(size_t) == 8` and as `ansi32` (or `ppro`) when `sizeof(size_t) == 4`
(wasm32, win32, i686, armv7). `x64` and `ppro` differ only by inline
assembly, so `uint128` and `ansi32` cover the observable behaviour.
CONFIG_32 can only be translated for a 32-bit target: `mpdecimal.h` requires
`mpd_size_t == mpd_uint_t`. The merged crate selects the configuration with
`target_pointer_width`.

## Pinned tools

- c2rust 0.22.1 with LLVM 21 (0.22.1 does not build with LLVM 22;
  immunant/c2rust#1671 is unreleased)
  - macOS: `brew install llvm@21 cmake` and
    `LLVM_CONFIG_PATH=$(brew --prefix llvm@21)/bin/llvm-config cargo install --locked c2rust@0.22.1`
  - Linux: [Containerfile](Containerfile), built with Apple `container`:
    `container build -t mpdecimal-c2rust -f c2rust/Containerfile c2rust`
    (also has the cross compilers, qemu-user, wasi-sdk 25 and wasmtime
    26.0.0 for the tests)
  - x86_64 test image:
    `container build --arch amd64 -t mpdecimal-amd64 -f c2rust/Containerfile.amd64 c2rust`
- Rust 1.99.0 with rustfmt: builds the translated crates (after
  normalization they need no unstable features) and formats them

## Usage

```sh
c2rust/transpile-all.sh [LABEL...]   # translate + normalize -> c2rust/out/LABEL/mpdec
                                     # (all labels: also oracles, merge,
                                     # dedupe, install -> ./Cargo.toml, ./src)
c2rust/test-all.sh [LABEL...]        # tests/runshort.sh + runshort_alloc.sh
CRATE=root c2rust/test-all.sh        # the same for the crate of the root
CRATE=root FEATURES=test-hooks c2rust/test-all.sh
c2rust/test-long.sh LABEL [TEST...]  # testit long tests, root crate with
                                     # the feature test-hooks
c2rust/test-long-all.sh [LABEL...]   # long tests for the test matrix
```

`transpile.sh [--test-hooks] LABEL MACHINE [CLANG_ARG...]` and
`test-short.sh LABEL` run a single configuration; `in-container.sh CMD...`
runs a command in a Linux image with the repository mounted at the same
absolute path.

## Pipeline

1. **Translation input** ([prepare.py](prepare.py), below).
2. **Translation** with c2rust; the raw output is kept in
   `c2rust/work/LABEL/raw`.
3. **Normalization** ([normalize.py](normalize.py)) of the crate in place:
   relooper labels and `freshN` temporaries are renumbered per item,
   `c_long`/`c_ulong` become fixed-width types of the translation target
   (except in C ABI declarations such as `long mpd_radix(void)`), identity
   casts that this leaves are removed, `CHAR_MAX` follows `c_char`, the
   hand-written [shim](shim/shim.rs) is added and the modules are formatted
   with rustfmt.
4. **Oracles** ([check-oracles.sh](check-oracles.sh)): every translation of
   a configuration is identical to the one that is merged, except as
   recorded in [expected/](expected): unsigned `char` (Linux aarch64, armv7),
   and on Windows backslashes in `__FILE__` and, with CONFIG_64, no
   conversions between `size_t` and `uint64_t`, which are the same C type
   with LLP64.
5. **Merge** ([merge.py](merge.py)): CONFIG_64 and CONFIG_32 into one crate,
   once without and once with the test hooks: equal items are emitted once,
   the others once per configuration under
   `#[cfg(target_pointer_width = "64")]` / `"32"`; then the two results
   with `#[cfg(not(feature = "test-hooks"))]` / `#[cfg(feature =
   "test-hooks")]` (`c2rust/work/merged/mpdec`).
6. **Deduplication** ([dedupe.py](dedupe.py)) of the merged crate: types,
   structs, constants and header inline functions that several modules
   define are moved to `src/common.rs`, and extern declarations of functions
   and statics that the crate defines become `use` declarations, so that
   rustc checks the calls between modules.
7. **Installation** ([install.py](install.py)) as the crate of the
   repository root (package `mpdecimal`, feature `test-hooks`). It stops if
   a file of the root crate was changed since the last installation.
8. **Tests** of the root crate on every target of the matrix.

## Translation input

[prepare.py](prepare.py) derives the input from the upstream build files:
`mpdecimal.h` from `mpdecimal.h.in` with the header block from
`configure.ac`, and `compile_commands.json` from `OBJS` in
`libmpdec/Makefile.in`. Flags are the library flags of `configure.ac`
without warnings, `-O2` and `-DNDEBUG` (assertions are kept), plus:

- `-Ic2rust/neutral-libc`: the library is translated against
  [neutral libc headers](neutral-libc/README.md) that declare only standard
  interfaces, with the exact-width types spelled the same on every
  platform. What cannot be declared that way (stderr, raise,
  assertion failure, and strtol/strtoll with a private errno) is redirected
  to `mpd_c2rust_*` functions of the hand-written
  [shim/shim.rs](shim/shim.rs). This also avoids the macOS ctype and
  aarch64 glibc math translation failures.
- `-Dinline=`: c2rust drops `#[no_mangle]` from C99 `inline` external
  definitions (`mpd_isnan`, `mpd_msword`, ...).
- `-fmacro-prefix-map=<repository>/=`: `__FILE__` (in `mpd_err_fatal`) is
  relative to the repository.
- `include-c2rust/mpdecimal.h`: in the CONFIG_32 block the suffixes `UL`/`L`
  become `U`/none, matching `mpd_uint_t`/`mpd_ssize_t` (`uint32_t`/`int32_t`).
  The values of all expressions are unchanged on the 32-bit targets that
  CONFIG_32 requires. The C tests are compiled with the unmodified header.
- `--test-hooks`: the sources with the test hooks of testit
  ([test-hooks/README.md](test-hooks/README.md)).

Before normalization, the translations of one MACHINE differ between
targets only in

- relooper block labels (`current_block = <number>`) and `freshN`
  temporaries, which are numbered differently;
- `long`: the type of pointer differences (`ptrdiff_t` of the target) and
  of `long mpd_radix(void)`;
- the signedness of `char`: `CHAR_MAX`, and the `#if CHAR_MAX == SCHAR_MAX`
  block of `mpd_validate_lconv`, which is a real platform difference;
- the path separator of `__FILE__` on Windows.

Options that were tried and are not used:

- `--ddebug-labels`: labels become strings such as `"c_769"`, which still
  differ between targets.
- `--translate-const-macros experimental`: output identical to the default.

## Freeze

The root crate is frozen on the short tests (`tests/runshort.sh`,
`tests/runshort_alloc.sh`): `CRATE=root c2rust/test-all.sh` and
`CRATE=root FEATURES=test-hooks c2rust/test-all.sh` pass on all nine targets
of the test matrix. The long tests below are not part of the freeze; they
were run in part to check that the runner works.

## Long tests

[test-long.sh](test-long.sh) builds the programs of
`testit-2.5.1/libmpdec/tests` against the 4.0.1 headers and the root crate
with the feature `test-hooks` (optimized, with overflow checks and debug
assertions) and runs them as `runalltests.sh` does. See
[testit/README.md](testit/README.md) for the two adaptations it applies to a
copy of the tests. The full run takes hours natively (`additional` with
`--all` repeats the tests for every `MPD_MINALLOC`); `TAG` allows running
groups of tests at the same time. [test-long-all.sh](test-long-all.sh) runs
all tests on the native targets and wasm32-wasip1 (CONFIG_32 at near native
speed), and a reduced set on the emulated targets.

Target conditions of the test programs (not of the crate):

- wasm32-wasip1: wasm-ld places a 64 KiB stack below the static data, which
  the unoptimized crate overflows into the statics; the test programs are
  linked with an 8 MiB stack placed first. `clock()` needs
  `-D_WASI_EMULATED_PROCESS_CLOCKS`. `cov.c` cannot be built (signal
  handlers, `tmpfile`). The shim's `raise` aborts on WASI (SIGFPE's default
  action), as WASI has no signals without the emulation library.
- Windows: MinGW-w64 and Wine 64-bit, linking the native libraries that
  rustc reports for the staticlib.
