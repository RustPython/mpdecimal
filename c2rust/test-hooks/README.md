# Test hooks of mpdecimal-testit

The long tests of mpdecimal-testit-2.5.1 (`testit-2.5.1/libmpdec/tests`)
need a library built from testit's own `mpdecimal.c` and `mpalloc.c`, which
differ from those of the mpdecimal-2.5.1 release exactly by
[testit-hooks.patch](testit-hooks.patch) (rebased to 4.0.1, where it applies
without changes to its hunks):

- `mpd_qsshiftr`, `_mpd_kmul`, `_mpd_fntmul`, `_mpd_kmul_fnt` (and
  `mpd_set_fenv`, CONFIG_32 without ANSI only) are external instead of
  static;
- fault injection: the globals `func_count`, `func_fail` with
  `mpd_set_func_fail()`, `mpd_set_func()`, and `FAILPOINT()` checks in
  `mpd_qadd_exact`, `mpd_qsub_exact`, `mpd_qmul_exact`, `mpd_sizeinbase`,
  `mpd_qimport_size`;
- Newton division test entry points `mpd_test_newtondiv`,
  `mpd_test_newtondivint`, `mpd_test_newtonrem`, `mpd_test_newtondivmod`;
- `mpalloc.c`: an allocation size check for `MPD_SIZE_MAX < SIZE_MAX`
  (the full_coverage configuration only; false in CONFIG_64 and CONFIG_32).

`prepare.py --test-hooks` applies the patch to a copy of the sources and adds
a `#line` directive after each changed region, so that `__LINE__` and
`__FILE__` (`mpd_err_fatal`) are those of the unpatched 4.0.1 sources and the
translation differs from the one without the patch only by the changes
above.

In the merged crate, the translation with the patch is selected by the
cargo feature `test-hooks` (`merge.py --feature test-hooks`). The feature is
for the long tests only: `FAILPOINT()` updates the unsynchronized globals in
the arithmetic functions.

The patch was checked against the releases with

```sh
diff -u mpdecimal-2.5.1/libmpdec/mpdecimal.c testit-2.5.1/libmpdec/mpdecimal.c
diff -u mpdecimal-2.5.1/libmpdec/mpalloc.c testit-2.5.1/libmpdec/mpalloc.c
```

whose hunks are those of `testit-hooks.patch`.
