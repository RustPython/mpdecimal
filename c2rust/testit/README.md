# Adaptations for running testit 2.5.1 against 4.0.1

Used by [../test-long.sh](../test-long.sh) on a copy of
`testit-2.5.1/libmpdec/tests`; the upstream files are not modified.

- [runtest-version.patch](runtest-version.patch): `runtest.c` exits unless
  the library version is 2.5.1. The patch makes it check for 4.0.1.
- [pyshim/distutils](pyshim/distutils): `formathelper.py` (used by
  `genlocale.py`, `genrandlocale.py`, `genrandformat.py`) imports
  `distutils.spawn.find_executable`, and distutils was removed in Python
  3.12. The stand-in provides `shutil.which` under that name and is put on
  `PYTHONPATH`.
