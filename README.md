# mpdecimal (Rust)

Rust translation of libmpdec from mpdecimal 4.0.1, produced by c2rust. One
source tree builds for all targets; the configuration is selected by
`target_pointer_width`:

- 64-bit targets: CONFIG_64 (`uint128`)
- 32-bit targets: CONFIG_32 (`ansi32`)

The crate exports the C ABI of libmpdec (`staticlib`, `rlib`).

## Layout

- `Cargo.toml`, `src/`: the translated crate, written by
  `c2rust/install.py` and edited by hand since, so `install.py` no longer
  overwrites them.
- `c2rust/`: the translation pipeline, its run log (`c2rust/RUNS.md`) and the
  test scripts. See [c2rust/README.md](c2rust/README.md).
- `libmpdec/`, `configure.ac`: the C sources and header configuration of
  mpdecimal 4.0.1, which are the input of the translation.
- `tests/`: the short tests of mpdecimal 4.0.1, unmodified.
- `testit-2.5.1/`: mpdecimal-testit 2.5.1, the long tests. The feature
  `test-hooks` is needed only for these tests.

## Tests

The upstream C tests are linked with the crate's staticlib:

```sh
CRATE=root c2rust/test-short.sh macos-aarch64-uint128   # one target
CRATE=root c2rust/test-all.sh                           # all targets
```

The crate is frozen on the short tests (`tests/runshort.sh`,
`tests/runshort_alloc.sh`). They pass on macOS aarch64/x86_64, Linux
aarch64/x86_64/i686/s390x/armv7, wasm32-wasip1 and Windows x86_64, with and
without `test-hooks`.

## License

BSD-2-Clause, see [COPYRIGHT.txt](COPYRIGHT.txt).
